use crate::*;
use nova_hir::{HirKind, Module as HirModule};
use nova_resolve::{DefinitionKind, Resolution, Resolved};
use nova_typecheck::{Checked, ConstEvaluation};
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
        entry: sources[hir.root().0],
        structs: checked.structs.clone(),
        enums: checked.enums.clone(),
        enums_original: checked.enums.clone(),
        sums: checked.sums.clone(),
        sums_original: checked.sums.clone(),
        sum_origins: checked
            .sum_origins
            .iter()
            .map(|(&id, at)| (id, sources[at.0]))
            .collect(),
        variant_provenance: checked
            .variants
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| v.map(|v| (i, v)))
            .collect(),
        binder_provenance: BTreeMap::new(),
        match_provenance: BTreeMap::new(),
        try_certificates: BTreeMap::new(),
        try_controls: BTreeMap::new(),
        structs_original: checked.structs.clone(),
        tuple_ids: checked.tuple_ids.clone(),
        callee_provenance: vec![],
        projection_provenance: checked
            .projections
            .iter()
            .enumerate()
            .filter_map(|(i, &f)| f.map(|f| (i, f)))
            .collect(),
        constructor_provenance: checked
            .calls
            .iter()
            .enumerate()
            .filter_map(|(i, d)| {
                d.filter(|d| matches!(resolved.definitions[d.0].kind, DefinitionKind::Struct(_)))
                    .map(|d| (i, StructId(d.0)))
            })
            .collect(),
        mutation_provenance: BTreeMap::new(),
        mutable_definitions: resolved
            .definitions
            .iter()
            .enumerate()
            .filter(|(_, d)| d.mutable)
            .map(|(i, _)| i)
            .collect(),
        entry_main: None,
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
    result.callee_provenance = result.callees.clone();
    for (index, node) in hir.nodes().iter().enumerate() {
        if matches!(node.kind, HirKind::Match { .. }) {
            let mut patterns = vec![];
            for &arm in &node.children[1..] {
                let pattern = hir.nodes()[arm.0].children[0];
                let info = checked.patterns[pattern.0].ok_or(LoweringError::InvalidAnalysis)?;
                patterns.push(info);
                if let nova_typecheck::MatchPattern::Variant(v) = info {
                    for (at, &binder) in hir.nodes()[pattern.0].children.iter().enumerate() {
                        if matches!(hir.nodes()[binder.0].kind, HirKind::Binder { .. }) {
                            result.binder_provenance.insert(binder.0, (v, at));
                        }
                    }
                }
            }
            if !checked.exhaustive_matches.contains(&index) {
                return Err(LoweringError::InvalidAnalysis);
            }
            result.match_provenance.insert(
                index,
                (ty(checked.type_table[node.children[0].0])?, patterns),
            );
        }
        if node.kind == HirKind::Tuple {
            result.constructor_provenance.insert(
                index,
                ty(checked.type_table[index])?
                    .aggregate()
                    .ok_or(LoweringError::InvalidAnalysis)?,
            );
        }
        if node.kind == HirKind::Assignment {
            let mut root = node.children[0];
            let mut path = vec![];
            while matches!(
                hir.nodes()[root.0].kind,
                HirKind::Projection { .. } | HirKind::TupleProjection { .. }
            ) {
                path.push(checked.projections[root.0].ok_or(LoweringError::InvalidAnalysis)?);
                root = hir.nodes()[root.0].children[0];
            }
            if !path.is_empty() {
                path.reverse();
                let Some(Resolution::Definition(def)) = resolved.references[root.0] else {
                    return Err(LoweringError::InvalidAnalysis);
                };
                result.mutation_provenance.insert(index, (def, path));
            }
        }
    }
    for callee in &result.callees {
        let DefinitionKind::Function(id) = resolved.definitions[callee.definition.0].kind else {
            continue;
        };
        if callee.name == "main" && result.sources[id.0].span.file() == result.entry.span.file() {
            result.entry_main = Some((callee.clone(), result.sources[id.0]));
        }
    }
    // HIR IDs are module-wide and function trees are disjoint. Allocate the
    // expression result table once, rather than once per function.
    let mut values = vec![None; hir.nodes().len()];
    for id in hir.items() {
        if matches!(
            hir.nodes()[id.0].kind,
            HirKind::Binding { constant: true, .. }
                | HirKind::Import { .. }
                | HirKind::Struct { .. }
                | HirKind::Enum { .. }
        ) {
            continue;
        }
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
            match_falls: Default::default(),
            tries: vec![],
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
        if !builder.tries.is_empty() {
            result.try_controls.insert(
                callee.0,
                TryControlCertificate {
                    source: builder.body.source,
                    entry: builder.body.entry,
                    terminators: builder
                        .body
                        .blocks
                        .iter()
                        .map(|b| b.terminator.clone())
                        .collect(),
                },
            );
            result.try_certificates.extend(
                builder
                    .tries
                    .into_iter()
                    .map(|c| ((c.callee.0, c.source.hir.0), c)),
            );
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
    Convert(HirId),
    FinishExpression(HirId),
    Try(HirId),
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
    Match(HirId),
    ArmEntry {
        arm: HirId,
        block: BlockId,
        snapshot: Place,
        join: BlockId,
    },
    ArmDone {
        arm: HirId,
        join: BlockId,
    },
    MatchDone {
        id: HirId,
        join: BlockId,
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
    match_falls: std::collections::BTreeSet<usize>,
    tries: Vec<TryCertificate>,
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
    fn temporary(&mut self, ty: Type, id: HirId) -> Place {
        let local = LocalId(self.body.locals.len());
        self.body.locals.push(Local {
            ty,
            definition: None,
            source: self.sources[id.0],
        });
        Place(local)
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
                        HirKind::Match { .. } => {
                            work.push(Work::Match(id));
                            work.push(Work::Expression(node.children[0]));
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
                Work::Match(id) => {
                    let node = &self.hir.nodes()[id.0];
                    let expression = node.children[0];
                    let snapshot = Place(self.local(expression, None)?);
                    self.assign(snapshot, Rvalue::Use(self.value(expression)?), expression)?;
                    let join = self.block();
                    let mut arms = vec![];
                    let mut entries = vec![];
                    for &arm in &node.children[1..] {
                        let pattern = self.hir.nodes()[arm.0].children[0];
                        let block = self.block();
                        arms.push((
                            self.checked.patterns[pattern.0]
                                .ok_or(LoweringError::InvalidAnalysis)?,
                            block,
                        ));
                        entries.push(Work::ArmEntry {
                            arm,
                            block,
                            snapshot,
                            join,
                        });
                    }
                    self.end(
                        TerminatorKind::Match {
                            scrutinee: Operand::Place(snapshot),
                            arms,
                        },
                        id,
                    )?;
                    work.push(Work::MatchDone { id, join });
                    work.extend(entries.into_iter().rev());
                }
                Work::ArmEntry {
                    arm,
                    block,
                    snapshot,
                    join,
                } => {
                    self.current = Some(block);
                    let pattern = self.hir.nodes()[arm.0].children[0];
                    if let Some(nova_typecheck::MatchPattern::Variant(v)) =
                        self.checked.patterns[pattern.0]
                    {
                        for (at, &binder) in self.hir.nodes()[pattern.0].children.iter().enumerate()
                        {
                            if let Some(def) = self.resolved.declaration_ids[binder.0] {
                                let local = self.local(binder, Some(def))?;
                                self.declarations.insert(def.0, local);
                                self.assign(
                                    Place(local),
                                    Rvalue::EnumPayload(Operand::Place(snapshot), v, at),
                                    binder,
                                )?;
                            }
                        }
                    }
                    work.push(Work::ArmDone { arm, join });
                    work.push(Work::Statement(self.hir.nodes()[arm.0].children[1]));
                }
                Work::ArmDone { arm, join } => {
                    if self.current.is_some() {
                        self.match_falls.insert(join.0);
                        self.end(TerminatorKind::Goto(join), arm)?;
                    }
                }
                Work::MatchDone { id, join } => {
                    self.current = Some(join);
                    if !self.match_falls.remove(&join.0) {
                        self.end(TerminatorKind::Unreachable, id)?;
                    }
                }
                Work::Convert(id) => {
                    if let Some(dest) = self.checked.coercions[id.0] {
                        let ty = self
                            .checked
                            .types
                            .get(dest)
                            .ok_or(LoweringError::InvalidAnalysis)?;
                        let local = LocalId(self.body.locals.len());
                        self.body.locals.push(Local {
                            ty,
                            definition: None,
                            source: self.sources[id.0],
                        });
                        let destination = Place(local);
                        let value = self.value(id)?;
                        self.assign(
                            destination,
                            if ty.float().is_some() {
                                Rvalue::NumericConvert(value, ty)
                            } else {
                                Rvalue::Widen(value, ty)
                            },
                            id,
                        )?;
                        self.values[id.0] = Some(Operand::Place(destination));
                    }
                }
                Work::Expression(id) => {
                    work.push(Work::Convert(id));
                    let node = &self.hir.nodes()[id.0];
                    if let Some(value) = self.checked.float_literals[id.0] {
                        self.values[id.0] = Some(Operand::Constant(Constant::Float(value)));
                        continue;
                    }
                    // Direct -2147483648 has a checked signed value on the prefix,
                    // while its magnitude child deliberately has no Int32 value.
                    if let Some(value) = self.checked.integer_literals[id.0] {
                        self.values[id.0] = Some(Operand::Constant(Constant::from_integer(value)));
                        continue;
                    }
                    match &node.kind {
                        HirKind::Character(value) => {
                            self.values[id.0] = Some(Operand::Constant(Constant::Char(*value)))
                        }
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
                            if matches!(
                                self.resolved.definitions[def.0].kind,
                                DefinitionKind::GlobalConst(_)
                            ) {
                                let ConstEvaluation::Value { value, .. } =
                                    &self.checked.const_values[def.0]
                                else {
                                    return Err(LoweringError::InvalidAnalysis);
                                };
                                self.values[id.0] =
                                    Some(Operand::Constant(Constant::from_const(value)));
                                continue;
                            }
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
                        HirKind::VariantPath { .. } | HirKind::None => {
                            let v = self.checked.variants[id.0]
                                .ok_or(LoweringError::InvalidAnalysis)?;
                            self.values[id.0] = Some(Operand::Constant(Constant::Enum(v, vec![])));
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
                        HirKind::Try { .. } => {
                            work.push(Work::Try(id));
                            work.push(Work::Expression(node.children[0]));
                        }
                        HirKind::Cast { .. } => {
                            work.push(Work::FinishExpression(id));
                            work.push(Work::Expression(node.children[0]));
                        }
                        HirKind::Group
                        | HirKind::Interpolation
                        | HirKind::Tuple
                        | HirKind::TupleProjection { .. }
                        | HirKind::Projection { .. }
                        | HirKind::Prefix(_)
                        | HirKind::Binary(_)
                        | HirKind::InterpolatedString => {
                            work.push(Work::FinishExpression(id));
                            work.extend(node.children.iter().rev().copied().map(Work::Expression));
                        }
                        _ => return Err(LoweringError::InvalidAnalysis),
                    }
                }
                Work::Try(id) => {
                    let node = &self.hir.nodes()[id.0];
                    let HirKind::Try { keyword } = node.kind else {
                        return Err(LoweringError::InvalidAnalysis);
                    };
                    let from = self
                        .checked
                        .types
                        .get(self.checked.type_table[node.children[0].0])
                        .ok_or(LoweringError::InvalidAnalysis)?;
                    let def = self.resolved.declaration_ids[self.body.source.hir.0]
                        .ok_or(LoweringError::InvalidAnalysis)?;
                    let to = self
                        .checked
                        .types
                        .get(
                            self.checked.signatures[def.0]
                                .as_ref()
                                .ok_or(LoweringError::InvalidAnalysis)?
                                .return_type,
                        )
                        .ok_or(LoweringError::InvalidAnalysis)?;
                    let (Type::Enum(source_result), Type::Enum(destination_result)) = (from, to)
                    else {
                        return Err(LoweringError::InvalidAnalysis);
                    };
                    let source_key = &self.checked.sums[&source_result];
                    let success_type = source_key.arguments[0];
                    let error_type = source_key.arguments[1];
                    let snapshot = self.temporary(from, id);
                    self.assign(snapshot, Rvalue::Use(self.value(node.children[0])?), id)?;
                    let dispatch = self.current.ok_or(LoweringError::InvalidAnalysis)?;
                    let snapshot_statement = self.body.blocks[dispatch.0]
                        .statements
                        .last()
                        .unwrap()
                        .clone();
                    let success = self.block();
                    let error = self.block();
                    self.end(
                        TerminatorKind::Try {
                            snapshot,
                            success,
                            error,
                        },
                        id,
                    )?;
                    let variant = |index| nova_types::VariantId {
                        enumeration: source_result,
                        index,
                    };
                    self.current = Some(error);
                    let error_value = self.temporary(error_type, id);
                    self.assign(
                        error_value,
                        Rvalue::EnumPayload(Operand::Place(snapshot), variant(1), 0),
                        id,
                    )?;
                    let result = self.temporary(to, id);
                    self.assign(
                        result,
                        Rvalue::Enum(
                            nova_types::VariantId {
                                enumeration: destination_result,
                                index: 1,
                            },
                            vec![Operand::Place(error_value)],
                        ),
                        id,
                    )?;
                    self.end(TerminatorKind::Return(Operand::Place(result)), id)?;
                    let error_body = self.body.blocks[error.0].clone();
                    self.current = Some(success);
                    let value = self.temporary(success_type, id);
                    self.assign(
                        value,
                        Rvalue::EnumPayload(Operand::Place(snapshot), variant(0), 0),
                        id,
                    )?;
                    let success_read = self.body.blocks[success.0].statements[0].clone();
                    self.values[id.0] = Some(Operand::Place(value));
                    self.tries.push(TryCertificate {
                        callee: self.body.callee,
                        source: self.sources[id.0],
                        keyword,
                        source_result,
                        destination_result,
                        dispatch,
                        snapshot: snapshot_statement,
                        success,
                        success_read,
                        error,
                        error_body,
                    });
                }
                Work::FinishExpression(id) => {
                    let node = &self.hir.nodes()[id.0];
                    let value = match node.kind {
                        HirKind::Group | HirKind::Interpolation => {
                            self.values[id.0] = Some(self.value(node.children[0])?);
                            continue;
                        }
                        HirKind::Cast { .. } => Rvalue::CheckedCast(
                            self.value(node.children[0])?,
                            self.checked
                                .types
                                .get(self.checked.type_table[id.0])
                                .ok_or(LoweringError::InvalidAnalysis)?,
                        ),
                        HirKind::Projection { .. } | HirKind::TupleProjection { .. } => {
                            Rvalue::Project(
                                self.value(node.children[0])?,
                                self.checked.projections[id.0]
                                    .ok_or(LoweringError::InvalidAnalysis)?,
                            )
                        }
                        HirKind::Tuple => Rvalue::Aggregate(
                            self.checked
                                .types
                                .get(self.checked.type_table[id.0])
                                .and_then(Type::aggregate)
                                .ok_or(LoweringError::InvalidAnalysis)?,
                            node.children
                                .iter()
                                .map(|&c| self.value(c))
                                .collect::<Result<_, _>>()?,
                        ),
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
                        HirKind::Call if self.checked.variants[id.0].is_some() => Rvalue::Enum(
                            self.checked.variants[id.0].expect("variant"),
                            node.children[1..]
                                .iter()
                                .map(|&c| self.value(c))
                                .collect::<Result<_, _>>()?,
                        ),
                        HirKind::Call => {
                            let definition =
                                self.checked.calls[id.0].ok_or(LoweringError::InvalidAnalysis)?;
                            if matches!(
                                self.resolved.definitions[definition.0].kind,
                                DefinitionKind::Struct(_)
                            ) {
                                let value = Rvalue::Aggregate(
                                    StructId(definition.0),
                                    node.children[1..]
                                        .iter()
                                        .map(|&c| self.value(c))
                                        .collect::<Result<_, _>>()?,
                                );
                                let dest = Place(self.local(id, None)?);
                                self.assign(dest, value, id)?;
                                self.values[id.0] = Some(Operand::Place(dest));
                                continue;
                            }
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
                        Operand::Constant(Constant::from_const(value))
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
                    let mut root = children[0];
                    let mut path = vec![];
                    while matches!(
                        self.hir.nodes()[root.0].kind,
                        HirKind::Projection { .. } | HirKind::TupleProjection { .. }
                    ) {
                        path.push(
                            self.checked.projections[root.0]
                                .ok_or(LoweringError::InvalidAnalysis)?,
                        );
                        root = self.hir.nodes()[root.0].children[0];
                    }
                    path.reverse();
                    let Some(Resolution::Definition(def)) = self.resolved.references[root.0] else {
                        return Err(LoweringError::InvalidAnalysis);
                    };
                    let local = *self
                        .declarations
                        .get(&def.0)
                        .ok_or(LoweringError::InvalidAnalysis)?;
                    let rhs = self.value(children[1])?;
                    self.assign(
                        Place(local),
                        if path.is_empty() {
                            Rvalue::Use(rhs)
                        } else {
                            Rvalue::Update(Operand::Place(Place(local)), path, rhs)
                        },
                        id,
                    )?;
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
