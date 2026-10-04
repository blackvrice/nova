use crate::*;
use nova_hir::{HirKind, Module as HirModule};
use nova_resolve::{DefinitionKind, Resolution, Resolved};
use nova_typecheck::{Checked, ConstEvaluation};
use nova_types::ConstValue;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoweringError {
    FrontendErrors,
    InvalidAnalysis,
    InvalidMir(Vec<ValidationError>),
}
impl fmt::Display for LoweringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MIR lowering rejected input: {self:?}")
    }
}
impl std::error::Error for LoweringError {}

/// `upstream_has_errors` MUST include Lexer and Parser diagnostics. Recovery HIR
/// alone cannot prove that the source had no inserted punctuation.
/// Public, mutable side tables are verified before they can become successful MIR.
pub fn lower(
    hir: &HirModule,
    resolved: &Resolved,
    checked: &Checked,
    upstream_has_errors: bool,
) -> Result<Module, LoweringError> {
    if upstream_has_errors
        || hir.has_errors()
        || checked.has_errors()
        || !resolved.diagnostics.is_empty()
    {
        return Err(LoweringError::FrontendErrors);
    }
    // Recompute to establish exact module/table correspondence, not merely lengths.
    // This also protects all indexing below from malformed external side tables.
    if nova_resolve::resolve(hir) != *resolved
        || nova_typecheck::check(hir, resolved).map_err(|_| LoweringError::InvalidAnalysis)?
            != *checked
    {
        return Err(LoweringError::InvalidAnalysis);
    }
    let sources = hir
        .nodes()
        .iter()
        .enumerate()
        .map(|(index, n)| SourceInfo {
            hir: HirId(index),
            span: n.span,
            origin: n.origin,
        })
        .collect::<Vec<_>>();
    let ty = |id| checked.types.get(id).ok_or(LoweringError::InvalidAnalysis);
    let mut result = Module {
        callees: vec![],
        bodies: vec![],
        sources,
    };
    let mut callees = BTreeMap::new();
    for (index, definition) in resolved.definitions.iter().enumerate() {
        if !matches!(
            definition.kind,
            DefinitionKind::BuiltinPrint | DefinitionKind::Function(_)
        ) {
            continue;
        }
        let signature = checked.signatures[index]
            .as_ref()
            .ok_or(LoweringError::InvalidAnalysis)?;
        callees.insert(index, CalleeId(result.callees.len()));
        result.callees.push(Callee {
            definition: DefId(index),
            name: definition.name.clone(),
            parameters: signature
                .parameters
                .iter()
                .copied()
                .map(ty)
                .collect::<Result<_, _>>()?,
            return_type: ty(signature.return_type)?,
            builtin_print: definition.kind == DefinitionKind::BuiltinPrint,
        });
    }
    // HIR IDs are module-wide and function trees are disjoint. Allocate the
    // expression result table once, rather than once per function.
    let mut values = vec![None; hir.nodes().len()];
    for &id in &hir.nodes()[hir.root().0].children {
        let HirKind::Function { parameters, .. } = hir.nodes()[id.0].kind else {
            return Err(LoweringError::InvalidAnalysis);
        };
        let definition = resolved.declaration_ids[id.0].ok_or(LoweringError::InvalidAnalysis)?;
        let callee = *callees
            .get(&definition.0)
            .ok_or(LoweringError::InvalidAnalysis)?;
        let body = Body {
            callee,
            source: result.sources[id.0],
            parameters: vec![],
            locals: vec![],
            blocks: vec![],
            entry: BlockId(0),
        };
        let mut builder = Builder {
            hir,
            resolved,
            checked,
            sources: &result.sources,
            callees: &callees,
            body,
            declarations: BTreeMap::new(),
            values: &mut values,
            current: None,
            loops: vec![],
        };
        builder.current = Some(builder.block());
        for &parameter in &hir.nodes()[id.0].children[..parameters] {
            let definition =
                resolved.declaration_ids[parameter.0].ok_or(LoweringError::InvalidAnalysis)?;
            let local = builder.local(parameter, Some(definition))?;
            builder.declarations.insert(definition.0, local);
            builder.body.parameters.push(local);
        }
        builder.walk(
            *hir.nodes()[id.0]
                .children
                .last()
                .ok_or(LoweringError::InvalidAnalysis)?,
        )?;
        if builder.current.is_some() {
            if result.callees[callee.0].return_type != Type::Unit {
                return Err(LoweringError::InvalidAnalysis);
            }
            builder.end(
                TerminatorKind::Return(Operand::Constant(Constant::Unit)),
                id,
            )?;
        }
        result.bodies.push(builder.body);
    }
    let errors = validate(&result);
    if errors.is_empty() {
        Ok(result)
    } else {
        Err(LoweringError::InvalidMir(errors))
    }
}

enum Work {
    Statement(HirId),
    Expression(HirId),
    FinishExpression(HirId),
    Binding(HirId),
    Assignment(HirId),
    Return(HirId),
    While {
        id: HirId,
        condition: BlockId,
        body: BlockId,
        exit: BlockId,
    },
    AfterLoop {
        id: HirId,
        condition: BlockId,
        exit: BlockId,
    },
    If(HirId),
    AfterThen {
        id: HirId,
        otherwise: BlockId,
        join: BlockId,
    },
    AfterElse {
        id: HirId,
        join: BlockId,
        then_falls: bool,
    },
    Logical(HirId),
    AfterRhs {
        id: HirId,
        destination: Place,
        join: BlockId,
    },
}
struct Builder<'a> {
    hir: &'a HirModule,
    resolved: &'a Resolved,
    checked: &'a Checked,
    sources: &'a [SourceInfo],
    callees: &'a BTreeMap<usize, CalleeId>,
    body: Body,
    declarations: BTreeMap<usize, LocalId>,
    values: &'a mut [Option<Operand>],
    current: Option<BlockId>,
    /// Active lexical loops: continue condition and break exit.
    loops: Vec<(BlockId, BlockId)>,
}
impl Builder<'_> {
    fn block(&mut self) -> BlockId {
        let id = BlockId(self.body.blocks.len());
        self.body.blocks.push(BasicBlockData {
            statements: vec![],
            terminator: None,
        });
        id
    }
    fn local(&mut self, id: HirId, definition: Option<DefId>) -> Result<LocalId, LoweringError> {
        // A binding statement has Unit type; its declared local has the
        // initializer/annotation type in the definition table.
        let type_id = match definition {
            Some(def) => self.checked.definition_types[def.0],
            None => self.checked.type_table[id.0],
        };
        let ty = self
            .checked
            .types
            .get(type_id)
            .ok_or(LoweringError::InvalidAnalysis)?;
        let local = LocalId(self.body.locals.len());
        self.body.locals.push(Local {
            ty,
            definition,
            source: self.sources[id.0],
        });
        Ok(local)
    }
    fn end(&mut self, kind: TerminatorKind, id: HirId) -> Result<(), LoweringError> {
        let block = self.current.take().ok_or(LoweringError::InvalidAnalysis)?;
        self.body.blocks[block.0].terminator = Some(Terminator {
            kind,
            source: self.sources[id.0],
        });
        Ok(())
    }
    fn assign(&mut self, place: Place, value: Rvalue, id: HirId) -> Result<(), LoweringError> {
        let block = self.current.ok_or(LoweringError::InvalidAnalysis)?;
        self.body.blocks[block.0].statements.push(Statement {
            kind: StatementKind::Assign(place, value),
            source: self.sources[id.0],
        });
        Ok(())
    }
    fn value(&self, id: HirId) -> Result<Operand, LoweringError> {
        self.values[id.0]
            .clone()
            .ok_or(LoweringError::InvalidAnalysis)
    }
    fn walk(&mut self, root: HirId) -> Result<(), LoweringError> {
        let mut work = vec![Work::Statement(root)];
        while let Some(task) = work.pop() {
            match task {
                Work::Statement(id) => {
                    if self.current.is_none() {
                        continue;
                    }
                    let node = &self.hir.nodes()[id.0];
                    match node.kind {
                        HirKind::Block => {
                            work.extend(node.children.iter().rev().copied().map(Work::Statement))
                        }
                        HirKind::Binding { constant, .. } => {
                            work.push(Work::Binding(id));
                            if !constant {
                                work.push(Work::Expression(
                                    *node.children.last().ok_or(LoweringError::InvalidAnalysis)?,
                                ));
                            }
                        }
                        HirKind::Return => {
                            work.push(Work::Return(id));
                            if let Some(&value) = node.children.first() {
                                work.push(Work::Expression(value));
                            }
                        }
                        HirKind::If => {
                            work.push(Work::If(id));
                            work.push(Work::Expression(node.children[0]));
                        }
                        HirKind::Assignment => {
                            work.push(Work::Assignment(id));
                            work.push(Work::Expression(node.children[1]));
                        }
                        HirKind::While => {
                            let condition = self.block();
                            let body = self.block();
                            let exit = self.block();
                            self.end(TerminatorKind::Goto(condition), id)?;
                            self.current = Some(condition);
                            work.push(Work::While {
                                id,
                                condition,
                                body,
                                exit,
                            });
                            work.push(Work::Expression(node.children[0]));
                        }
                        HirKind::Break | HirKind::Continue => {
                            let &(condition, exit) =
                                self.loops.last().ok_or(LoweringError::InvalidAnalysis)?;
                            self.end(
                                TerminatorKind::Goto(if node.kind == HirKind::Break {
                                    exit
                                } else {
                                    condition
                                }),
                                id,
                            )?;
                        }
                        HirKind::ExpressionStatement => {
                            work.push(Work::Expression(node.children[0]))
                        }
                        _ => return Err(LoweringError::InvalidAnalysis),
                    }
                }
                Work::Expression(id) => {
                    let node = &self.hir.nodes()[id.0];
                    // Direct -2147483648 has a checked signed value on the prefix,
                    // while its magnitude child deliberately has no Int32 value.
                    if let Some(value) = self.checked.integer_values[id.0] {
                        self.values[id.0] = Some(Operand::Constant(Constant::Int32(value)));
                        continue;
                    }
                    match &node.kind {
                        HirKind::String(text) => {
                            self.values[id.0] =
                                Some(Operand::Constant(Constant::String(text.clone())))
                        }
                        HirKind::Boolean(value) => {
                            self.values[id.0] = Some(Operand::Constant(Constant::Bool(*value)))
                        }
                        HirKind::Unit => {
                            self.values[id.0] = Some(Operand::Constant(Constant::Unit))
                        }
                        HirKind::Name(_) => {
                            let Some(Resolution::Definition(def)) = self.resolved.references[id.0]
                            else {
                                return Err(LoweringError::InvalidAnalysis);
                            };
                            let local = *self
                                .declarations
                                .get(&def.0)
                                .ok_or(LoweringError::InvalidAnalysis)?;
                            self.values[id.0] = Some(Operand::Place(Place(local)));
                        }
                        HirKind::Binary(Symbol::AndAnd | Symbol::OrOr) => {
                            work.push(Work::Logical(id));
                            work.push(Work::Expression(node.children[0]));
                        }
                        HirKind::Call => {
                            // Callee identity is statically resolved; it is not a function value.
                            work.push(Work::FinishExpression(id));
                            work.extend(
                                node.children[1..]
                                    .iter()
                                    .rev()
                                    .copied()
                                    .map(Work::Expression),
                            );
                        }
                        HirKind::Group
                        | HirKind::Interpolation
                        | HirKind::Prefix(_)
                        | HirKind::Binary(_)
                        | HirKind::InterpolatedString => {
                            work.push(Work::FinishExpression(id));
                            work.extend(node.children.iter().rev().copied().map(Work::Expression));
                        }
                        _ => return Err(LoweringError::InvalidAnalysis),
                    }
                }
                Work::FinishExpression(id) => {
                    let node = &self.hir.nodes()[id.0];
                    let value = match node.kind {
                        HirKind::Group | HirKind::Interpolation => {
                            self.values[id.0] = Some(self.value(node.children[0])?);
                            continue;
                        }
                        HirKind::Prefix(op) => Rvalue::Unary(op, self.value(node.children[0])?),
                        HirKind::Binary(op) => Rvalue::Binary(
                            op,
                            self.value(node.children[0])?,
                            self.value(node.children[1])?,
                        ),
                        HirKind::InterpolatedString => Rvalue::Interpolate(
                            node.children
                                .iter()
                                .map(|&c| self.value(c))
                                .collect::<Result<_, _>>()?,
                        ),
                        HirKind::Call => {
                            let definition =
                                self.checked.calls[id.0].ok_or(LoweringError::InvalidAnalysis)?;
                            let callee = *self
                                .callees
                                .get(&definition.0)
                                .ok_or(LoweringError::InvalidAnalysis)?;
                            let arguments = node.children[1..]
                                .iter()
                                .map(|&c| self.value(c))
                                .collect::<Result<_, _>>()?;
                            let destination = Place(self.local(id, None)?);
                            let target = self.block();
                            self.end(
                                TerminatorKind::Call {
                                    callee,
                                    arguments,
                                    destination,
                                    target,
                                },
                                id,
                            )?;
                            self.current = Some(target);
                            self.values[id.0] = Some(Operand::Place(destination));
                            continue;
                        }
                        _ => return Err(LoweringError::InvalidAnalysis),
                    };
                    let destination = Place(self.local(id, None)?);
                    self.assign(destination, value, id)?;
                    self.values[id.0] = Some(Operand::Place(destination));
                }
                Work::Binding(id) => {
                    let definition = self.resolved.declaration_ids[id.0]
                        .ok_or(LoweringError::InvalidAnalysis)?;
                    let local = self.local(id, Some(definition))?;
                    let initializer = *self.hir.nodes()[id.0]
                        .children
                        .last()
                        .ok_or(LoweringError::InvalidAnalysis)?;
                    let value = if let ConstEvaluation::Value { value, .. } =
                        &self.checked.const_values[definition.0]
                    {
                        Operand::Constant(match value {
                            ConstValue::Int32(value) => Constant::Int32(*value),
                            ConstValue::Bool(value) => Constant::Bool(*value),
                            ConstValue::String(value) => Constant::String(value.clone()),
                            ConstValue::Unit => Constant::Unit,
                        })
                    } else {
                        self.value(initializer)?
                    };
                    self.assign(Place(local), Rvalue::Use(value), id)?;
                    self.declarations.insert(definition.0, local);
                }
                Work::Return(id) => {
                    let value = match self.hir.nodes()[id.0].children.first() {
                        Some(&child) => self.value(child)?,
                        None => Operand::Constant(Constant::Unit),
                    };
                    self.end(TerminatorKind::Return(value), id)?;
                }
                Work::If(id) => {
                    // Both branches preserve the enclosing lexical loop stack.
                    let node = &self.hir.nodes()[id.0];
                    let then_block = self.block();
                    let otherwise = self.block();
                    let join = self.block();
                    self.end(
                        TerminatorKind::Branch {
                            condition: self.value(node.children[0])?,
                            then_block,
                            else_block: otherwise,
                        },
                        id,
                    )?;
                    self.current = Some(then_block);
                    work.push(Work::AfterThen {
                        id,
                        otherwise,
                        join,
                    });
                    work.push(Work::Statement(node.children[1]));
                }
                Work::Assignment(id) => {
                    let children = &self.hir.nodes()[id.0].children;
                    let Some(Resolution::Definition(def)) = self.resolved.references[children[0].0]
                    else {
                        return Err(LoweringError::InvalidAnalysis);
                    };
                    let local = *self
                        .declarations
                        .get(&def.0)
                        .ok_or(LoweringError::InvalidAnalysis)?;
                    self.assign(Place(local), Rvalue::Use(self.value(children[1])?), id)?;
                }
                Work::While {
                    id,
                    condition,
                    body,
                    exit,
                } => {
                    let children = &self.hir.nodes()[id.0].children;
                    self.end(
                        TerminatorKind::Branch {
                            condition: self.value(children[0])?,
                            then_block: body,
                            else_block: exit,
                        },
                        id,
                    )?;
                    self.loops.push((condition, exit));
                    self.current = Some(body);
                    work.push(Work::AfterLoop {
                        id,
                        condition,
                        exit,
                    });
                    work.push(Work::Statement(children[1]));
                }
                Work::AfterLoop {
                    id,
                    condition,
                    exit,
                } => {
                    if self.current.is_some() {
                        self.end(TerminatorKind::Goto(condition), id)?;
                    }
                    if self.loops.pop() != Some((condition, exit)) {
                        return Err(LoweringError::InvalidAnalysis);
                    }
                    self.current = Some(exit);
                }
                Work::AfterThen {
                    id,
                    otherwise,
                    join,
                } => {
                    let then_falls = self.current.is_some();
                    if then_falls {
                        self.end(TerminatorKind::Goto(join), id)?;
                    }
                    self.current = Some(otherwise);
                    work.push(Work::AfterElse {
                        id,
                        join,
                        then_falls,
                    });
                    if let Some(&child) = self.hir.nodes()[id.0].children.get(2) {
                        work.push(Work::Statement(child));
                    }
                }
                Work::AfterElse {
                    id,
                    join,
                    then_falls,
                } => {
                    let else_falls = self.current.is_some();
                    if else_falls {
                        self.end(TerminatorKind::Goto(join), id)?;
                    }
                    self.current = Some(join);
                    if !then_falls && !else_falls {
                        self.end(TerminatorKind::Unreachable, id)?;
                    }
                }
                Work::Logical(id) => {
                    let node = &self.hir.nodes()[id.0];
                    let is_and = node.kind == HirKind::Binary(Symbol::AndAnd);
                    let destination = Place(self.local(id, None)?);
                    let rhs = self.block();
                    let short = self.block();
                    let join = self.block();
                    self.end(
                        TerminatorKind::Branch {
                            condition: self.value(node.children[0])?,
                            then_block: if is_and { rhs } else { short },
                            else_block: if is_and { short } else { rhs },
                        },
                        id,
                    )?;
                    self.current = Some(short);
                    self.assign(
                        destination,
                        Rvalue::Use(Operand::Constant(Constant::Bool(!is_and))),
                        id,
                    )?;
                    self.end(TerminatorKind::Goto(join), id)?;
                    self.current = Some(rhs);
                    work.push(Work::AfterRhs {
                        id,
                        destination,
                        join,
                    });
                    work.push(Work::Expression(node.children[1]));
                }
                Work::AfterRhs {
                    id,
                    destination,
                    join,
                } => {
                    let rhs = self.hir.nodes()[id.0].children[1];
                    self.assign(destination, Rvalue::Use(self.value(rhs)?), id)?;
                    self.end(TerminatorKind::Goto(join), id)?;
                    self.current = Some(join);
                    self.values[id.0] = Some(Operand::Place(destination));
                }
            }
        }
        Ok(())
    }
}
