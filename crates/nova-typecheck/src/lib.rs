//! P02 single-file semantic checking. Successful checking is not native execution.
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, Severity};
use nova_hir::{HirId, HirKind, Module};
use nova_resolve::{DefId, DefinitionKind, Resolution, Resolved};
use nova_source::Span;
use nova_syntax::Symbol;
use nova_types::{Type, TypeId, TypeInterner};
use std::fmt::{self, Write};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature {
    pub parameters: Vec<TypeId>,
    pub return_type: TypeId,
    pub parameter_spans: Vec<Option<Span>>,
    pub return_span: Option<Span>,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Checked {
    pub types: TypeInterner,
    pub type_table: Vec<TypeId>,
    pub definition_types: Vec<TypeId>,
    pub signatures: Vec<Option<Signature>>,
    pub calls: Vec<Option<DefId>>,
    pub integer_values: Vec<Option<i32>>,
    pub always_returns: Vec<bool>,
    pub diagnostics: Vec<Diagnostic>,
    upstream_errors: bool,
}
impl Checked {
    pub fn has_errors(&self) -> bool {
        self.upstream_errors
            || !self.diagnostics.is_empty()
            || self
                .type_table
                .iter()
                .chain(&self.definition_types)
                .any(|id| matches!(self.types.get(*id), None | Some(Type::Error)))
    }
    pub fn dump(&self) -> String {
        let mut output = String::new();
        for (index, &ty) in self.type_table.iter().enumerate() {
            let _ = writeln!(
                output,
                "{index} {:?} value {:?} call {:?} returns {}",
                self.types.get(ty),
                self.integer_values[index],
                self.calls[index],
                self.always_returns[index]
            );
        }
        output
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckError {
    InvalidResolution,
}
impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("resolution tables do not match the HIR module")
    }
}
impl std::error::Error for CheckError {}

struct Checker<'a> {
    module: &'a Module,
    resolved: &'a Resolved,
    result: Checked,
}
#[derive(Clone, Copy)]
struct Context {
    expected: Option<TypeId>,
    expected_span: Option<Span>,
    return_type: TypeId,
    return_span: Option<Span>,
    direct_callee: bool,
}
enum Work {
    Enter(HirId, Context),
    Exit(HirId, Context),
}

pub fn check(module: &Module, resolved: &Resolved) -> Result<Checked, CheckError> {
    let size = module.nodes().len();
    if resolved.references.len() != size
        || resolved.declaration_ids.len() != size
        || resolved.node_scopes.len() != size
        || resolved.references.iter().any(
            |r| matches!(r, Some(Resolution::Definition(id)) if id.0 >= resolved.definitions.len()),
        )
        || resolved
            .declaration_ids
            .iter()
            .flatten()
            .any(|id| id.0 >= resolved.definitions.len())
    {
        return Err(CheckError::InvalidResolution);
    }
    for (index, definition) in resolved.definitions.iter().enumerate() {
        let id = match definition.kind {
            DefinitionKind::BuiltinPrint => continue,
            DefinitionKind::Function(id)
            | DefinitionKind::Parameter(id)
            | DefinitionKind::Local(id) => id,
        };
        let Some(node) = module.node(id) else {
            return Err(CheckError::InvalidResolution);
        };
        let name = match (definition.kind, &node.kind) {
            (DefinitionKind::Function(_), HirKind::Function { name, .. })
            | (DefinitionKind::Parameter(_), HirKind::Parameter { name, .. })
            | (DefinitionKind::Local(_), HirKind::Binding { name, .. }) => name,
            _ => return Err(CheckError::InvalidResolution),
        };
        if module.symbol(*name) != Some(definition.name.as_str())
            || resolved.declaration_ids[id.0] != Some(DefId(index))
        {
            return Err(CheckError::InvalidResolution);
        }
    }
    for (index, reference) in resolved.references.iter().enumerate() {
        if let Some(reference) = reference {
            let HirKind::Name(name) = module.nodes()[index].kind else {
                return Err(CheckError::InvalidResolution);
            };
            if let Resolution::Definition(def) = reference {
                if module.symbol(name) != Some(resolved.definitions[def.0].name.as_str()) {
                    return Err(CheckError::InvalidResolution);
                }
            }
        }
    }
    let mut types = TypeInterner::default();
    let error = types.intern(Type::Error);
    for ty in [
        Type::Unit,
        Type::Int32,
        Type::Bool,
        Type::String,
        Type::Function,
    ] {
        types.intern(ty);
    }
    let result = Checked {
        types,
        type_table: vec![error; size],
        definition_types: vec![error; resolved.definitions.len()],
        signatures: vec![None; resolved.definitions.len()],
        calls: vec![None; size],
        integer_values: vec![None; size],
        always_returns: vec![false; size],
        diagnostics: resolved.diagnostics.clone(),
        upstream_errors: module.has_errors(),
    };
    let mut checker = Checker {
        module,
        resolved,
        result,
    };
    checker.collect_signatures();
    let items = module.nodes()[module.root().0].children.clone();
    for item in items {
        let node = &module.nodes()[item.0];
        let HirKind::Function { parameters, .. } = node.kind else {
            continue;
        };
        let return_id = node.children[parameters];
        let return_type = checker.result.type_table[return_id.0];
        let body = *node.children.last().expect("lowered function body");
        let context = Context {
            expected: None,
            expected_span: None,
            return_type,
            return_span: Some(module.nodes()[return_id.0].span),
            direct_callee: false,
        };
        checker.walk(body, context);
        if !checker.result.upstream_errors
            && checker.ty(return_type) != Type::Unit
            && checker.ty(return_type) != Type::Error
            && !checker.result.always_returns[body.0]
        {
            checker.report(
                3003,
                module.nodes()[body.0].span,
                "non-Unit function can fall through without returning",
                context.return_span,
            );
        }
        checker.set(item, Type::Function);
    }
    checker.set(module.root(), Type::Unit);
    Ok(checker.result)
}

impl Checker<'_> {
    fn ty(&self, id: TypeId) -> Type {
        self.result
            .types
            .get(id)
            .expect("type table contains interned IDs")
    }
    fn set(&mut self, id: HirId, ty: Type) {
        self.result.type_table[id.0] = self.result.types.intern(ty);
    }
    fn report(&mut self, code: u16, span: Span, message: &str, secondary: Option<Span>) {
        self.result.diagnostics.push(Diagnostic {
            code: DiagnosticCode::new(code).expect("approved semantic code"),
            severity: Severity::Error,
            message: message.into(),
            primary: Label {
                span,
                message: message.into(),
            },
            secondary: secondary
                .into_iter()
                .map(|span| Label {
                    span,
                    message: "expected type or declaration here".into(),
                })
                .collect(),
            notes: vec![],
            suggestions: vec![],
        });
    }
    fn type_syntax(&mut self, id: HirId) -> TypeId {
        let node = &self.module.nodes()[id.0];
        let ty = match node.kind {
            HirKind::UnitType => Type::Unit,
            HirKind::TypeName(name) => {
                match self.module.symbol(name).expect("type symbol exists") {
                    "int32" => Type::Int32,
                    "bool" => Type::Bool,
                    "string" => Type::String,
                    "byte" | "char" | "int8" | "int16" | "int64" | "uint8" | "uint16"
                    | "uint32" | "uint64" | "float32" | "float64" | "never" => {
                        self.report(
                            1102,
                            node.span,
                            "this Primitive type is outside Stage A",
                            None,
                        );
                        Type::Error
                    }
                    _ => {
                        self.report(2001, node.span, "undefined type name", None);
                        Type::Error
                    }
                }
            }
            HirKind::Error => Type::Error,
            _ => Type::Error,
        };
        self.set(id, ty);
        self.result.type_table[id.0]
    }
    fn collect_signatures(&mut self) {
        for (index, definition) in self.resolved.definitions.iter().enumerate() {
            match definition.kind {
                DefinitionKind::BuiltinPrint => {
                    self.result.definition_types[index] = self.result.types.intern(Type::Function);
                    self.result.signatures[index] = Some(Signature {
                        parameters: vec![self.result.types.intern(Type::String)],
                        return_type: self.result.types.intern(Type::Unit),
                        parameter_spans: vec![None],
                        return_span: None,
                    });
                }
                DefinitionKind::Function(id) => {
                    let node = &self.module.nodes()[id.0];
                    let HirKind::Function { parameters, .. } = node.kind else {
                        unreachable!("resolution validated above")
                    };
                    let mut parameter_types = vec![];
                    let mut spans = vec![];
                    for &parameter in &node.children[..parameters] {
                        if self.module.nodes()[parameter.0].kind == HirKind::Error {
                            parameter_types.push(self.result.types.intern(Type::Error));
                            spans.push(None);
                            continue;
                        }
                        let ty_id = self.module.nodes()[parameter.0].children[0];
                        parameter_types.push(self.type_syntax(ty_id));
                        spans.push(Some(self.module.nodes()[ty_id.0].span));
                    }
                    let return_id = node.children[parameters];
                    let return_type = self.type_syntax(return_id);
                    self.result.definition_types[index] = self.result.types.intern(Type::Function);
                    self.result.signatures[index] = Some(Signature {
                        parameters: parameter_types,
                        return_type,
                        parameter_spans: spans,
                        return_span: Some(self.module.nodes()[return_id.0].span),
                    });
                }
                DefinitionKind::Parameter(id) => {
                    let ty_id = self.module.nodes()[id.0].children[0];
                    // Function signatures have already visited parameter syntax.
                    let ty = self.result.type_table[ty_id.0];
                    self.result.definition_types[index] = ty;
                    self.result.type_table[id.0] = ty;
                }
                DefinitionKind::Local(_) => {}
            }
        }
    }
    fn mismatch(&mut self, id: HirId, expected: TypeId, secondary: Option<Span>) -> bool {
        let actual = self.result.type_table[id.0];
        if !self.result.types.compatible(actual, expected) {
            self.report(
                2101,
                self.module.nodes()[id.0].span,
                &format!(
                    "expected {:?}, found {:?}",
                    self.ty(expected),
                    self.ty(actual)
                ),
                secondary,
            );
            true
        } else {
            false
        }
    }
    fn walk(&mut self, root: HirId, context: Context) {
        let mut pending = vec![Work::Enter(root, context)];
        while let Some(work) = pending.pop() {
            let (id, context, exit) = match work {
                Work::Enter(id, cx) => (id, cx, false),
                Work::Exit(id, cx) => (id, cx, true),
            };
            if exit {
                self.finish(id, context);
                self.apply_expected(id, context);
                continue;
            }
            let node = &self.module.nodes()[id.0];
            pending.push(Work::Exit(id, context));
            match node.kind {
                HirKind::Binding { has_type, .. } => {
                    let expected = if has_type {
                        Some(self.type_syntax(node.children[0]))
                    } else {
                        None
                    };
                    pending.push(Work::Enter(
                        *node.children.last().expect("binding initializer"),
                        Context {
                            expected,
                            expected_span: if has_type {
                                Some(self.module.nodes()[node.children[0].0].span)
                            } else {
                                None
                            },
                            direct_callee: false,
                            ..context
                        },
                    ));
                }
                HirKind::Return => {
                    if let Some(&value) = node.children.first() {
                        pending.push(Work::Enter(
                            value,
                            Context {
                                expected: Some(context.return_type),
                                expected_span: context.return_span,
                                direct_callee: false,
                                ..context
                            },
                        ));
                    }
                }
                HirKind::Call => {
                    let def = self.callee_definition(node.children[0]);
                    for (index, &child) in node.children.iter().enumerate().rev() {
                        let expected = if index == 0 {
                            None
                        } else {
                            def.and_then(|d| self.result.signatures[d.0].as_ref())
                                .and_then(|s| s.parameters.get(index - 1))
                                .copied()
                        };
                        pending.push(Work::Enter(
                            child,
                            Context {
                                expected,
                                expected_span: if index == 0 {
                                    None
                                } else {
                                    def.and_then(|d| self.result.signatures[d.0].as_ref())
                                        .and_then(|s| s.parameter_spans.get(index - 1))
                                        .copied()
                                        .flatten()
                                },
                                direct_callee: index == 0,
                                ..context
                            },
                        ));
                    }
                }
                HirKind::Prefix(Symbol::Minus)
                    if matches!(
                        self.module.nodes()[node.children[0].0].kind,
                        HirKind::Integer(_)
                    ) =>
                {
                    // Check signed magnitude as one literal so Int32::MIN is valid.
                    let child = node.children[0];
                    let HirKind::Integer(ref spelling) = self.module.nodes()[child.0].kind else {
                        unreachable!()
                    };
                    if let Some(value) = integer(spelling, true) {
                        self.set(child, Type::Int32);
                        self.set(id, Type::Int32);
                        self.result.integer_values[id.0] = Some(value);
                    } else {
                        self.report(
                            2102,
                            node.span,
                            "integer literal is outside Int32 range",
                            None,
                        );
                        self.set(child, Type::Error);
                        self.set(id, Type::Error);
                    }
                    pending.pop();
                    self.apply_expected(id, context);
                }
                HirKind::Group => {
                    pending.push(Work::Enter(node.children[0], context));
                }
                _ => {
                    for &child in node.children.iter().rev() {
                        pending.push(Work::Enter(
                            child,
                            Context {
                                expected: None,
                                expected_span: None,
                                direct_callee: false,
                                ..context
                            },
                        ));
                    }
                }
            }
        }
    }
    fn callee_definition(&self, mut id: HirId) -> Option<DefId> {
        while self.module.nodes()[id.0].kind == HirKind::Group {
            id = self.module.nodes()[id.0].children[0];
        }
        if let Some(Resolution::Definition(def)) = self.resolved.references[id.0] {
            if matches!(
                self.resolved.definitions[def.0].kind,
                DefinitionKind::Function(_) | DefinitionKind::BuiltinPrint
            ) {
                return Some(def);
            }
        }
        None
    }
    fn apply_expected(&mut self, id: HirId, context: Context) {
        if let Some(expected) = context.expected {
            if self.mismatch(id, expected, context.expected_span) {
                self.set(id, Type::Error);
            }
        }
    }
    fn finish(&mut self, id: HirId, context: Context) {
        let node = &self.module.nodes()[id.0];
        match &node.kind {
            HirKind::Error => self.set(id, Type::Error),
            HirKind::Integer(spelling) => {
                if let Some(value) = integer(spelling, false) {
                    self.result.integer_values[id.0] = Some(value);
                    self.set(id, Type::Int32);
                } else {
                    self.report(
                        2102,
                        node.span,
                        "integer literal is outside Int32 range",
                        None,
                    );
                    self.set(id, Type::Error);
                }
            }
            HirKind::String(_) => self.set(id, Type::String),
            HirKind::Boolean(_) => self.set(id, Type::Bool),
            HirKind::Unit => self.set(id, Type::Unit),
            HirKind::Name(_) => {
                if let Some(Resolution::Definition(def)) = self.resolved.references[id.0] {
                    let ty = self.result.definition_types[def.0];
                    if self.ty(ty) == Type::Function && !context.direct_callee {
                        self.report(
                            1102,
                            node.span,
                            "function values are outside Stage A; use a direct call",
                            None,
                        );
                    } else {
                        self.result.type_table[id.0] = ty;
                    }
                }
            }
            HirKind::Group => {
                self.result.type_table[id.0] = self.result.type_table[node.children[0].0]
            }
            HirKind::Binding { has_type, .. } => {
                let value = *node.children.last().expect("initializer");
                let inferred = self.result.type_table[value.0];
                let declared = if *has_type {
                    self.result.type_table[node.children[0].0]
                } else {
                    inferred
                };
                let mismatch = self.mismatch(
                    value,
                    declared,
                    if *has_type {
                        Some(self.module.nodes()[node.children[0].0].span)
                    } else {
                        None
                    },
                );
                let ty = if mismatch || self.ty(inferred) == Type::Error {
                    self.result.types.intern(Type::Error)
                } else {
                    declared
                };
                if let Some(def) = self.resolved.declaration_ids[id.0] {
                    self.result.definition_types[def.0] = ty;
                }
                self.set(id, Type::Unit);
            }
            HirKind::Return => {
                if let Some(&value) = node.children.first() {
                    self.mismatch(value, context.return_type, context.return_span);
                } else if !matches!(self.ty(context.return_type), Type::Unit | Type::Error) {
                    self.report(
                        3002,
                        node.span,
                        "bare return requires a Unit return type",
                        context.return_span,
                    );
                }
                self.result.always_returns[id.0] = true;
                self.set(id, Type::Unit);
            }
            HirKind::If => {
                let condition = node.children[0];
                if !matches!(
                    self.ty(self.result.type_table[condition.0]),
                    Type::Bool | Type::Error
                ) {
                    self.report(
                        3001,
                        self.module.nodes()[condition.0].span,
                        "if condition must be Bool",
                        None,
                    );
                }
                self.result.always_returns[id.0] = node.children.len() == 3
                    && self.result.always_returns[node.children[1].0]
                    && self.result.always_returns[node.children[2].0];
                self.set(id, Type::Unit);
            }
            HirKind::Block => {
                self.result.always_returns[id.0] = node
                    .children
                    .iter()
                    .any(|c| self.result.always_returns[c.0]);
                self.set(id, Type::Unit);
            }
            HirKind::ExpressionStatement => self.set(id, Type::Unit),
            HirKind::Prefix(op) => {
                let operand = node.children[0];
                let wanted = if *op == Symbol::Bang {
                    Type::Bool
                } else {
                    Type::Int32
                };
                let expected = self.result.types.intern(wanted);
                let wrong = self.mismatch(operand, expected, None);
                self.set(
                    id,
                    if wrong || self.ty(self.result.type_table[operand.0]) == Type::Error {
                        Type::Error
                    } else {
                        wanted
                    },
                );
            }
            HirKind::Binary(op) => {
                let left = self.ty(self.result.type_table[node.children[0].0]);
                let right = self.ty(self.result.type_table[node.children[1].0]);
                if left == Type::Error || right == Type::Error {
                    self.set(id, Type::Error);
                    return;
                }
                let comparison = matches!(
                    op,
                    Symbol::EqualEqual
                        | Symbol::BangEqual
                        | Symbol::Less
                        | Symbol::LessEqual
                        | Symbol::Greater
                        | Symbol::GreaterEqual
                );
                let logical = matches!(op, Symbol::AndAnd | Symbol::OrOr);
                let valid = if logical {
                    left == Type::Bool && right == Type::Bool
                } else if matches!(op, Symbol::EqualEqual | Symbol::BangEqual) {
                    left == right && matches!(left, Type::Int32 | Type::Bool)
                } else {
                    left == Type::Int32 && right == Type::Int32
                };
                if !valid {
                    self.report(
                        2101,
                        node.span,
                        "operator operand types are incompatible",
                        None,
                    );
                    self.set(id, Type::Error);
                } else {
                    self.set(
                        id,
                        if comparison || logical {
                            Type::Bool
                        } else {
                            Type::Int32
                        },
                    );
                }
            }
            HirKind::Call => {
                let callee = node.children[0];
                if let Some(def) = self.callee_definition(callee) {
                    let signature = self.result.signatures[def.0]
                        .clone()
                        .expect("function signature collected");
                    self.result.calls[id.0] = Some(def);
                    let mut wrong = false;
                    if node.children.len() - 1 != signature.parameters.len() {
                        self.report(
                            2201,
                            node.span,
                            "function argument count does not match",
                            self.resolved.definitions[def.0].span,
                        );
                        wrong = true;
                    }
                    for (index, (&argument, &expected)) in node.children[1..]
                        .iter()
                        .zip(&signature.parameters)
                        .enumerate()
                    {
                        wrong |=
                            self.mismatch(argument, expected, signature.parameter_spans[index]);
                        wrong |= self.ty(self.result.type_table[argument.0]) == Type::Error;
                    }
                    if wrong {
                        self.set(id, Type::Error);
                    } else {
                        self.result.type_table[id.0] = signature.return_type;
                    }
                } else {
                    if self.ty(self.result.type_table[callee.0]) != Type::Error {
                        self.report(
                            2101,
                            self.module.nodes()[callee.0].span,
                            "expression is not callable",
                            None,
                        );
                    }
                    self.set(id, Type::Error);
                }
            }
            HirKind::Interpolation => {
                let value = node.children[0];
                let actual = self.ty(self.result.type_table[value.0]);
                if !matches!(
                    actual,
                    Type::Int32 | Type::Bool | Type::String | Type::Error
                ) {
                    self.report(
                        2101,
                        self.module.nodes()[value.0].span,
                        "interpolation requires Int32, Bool or String",
                        None,
                    );
                    self.set(id, Type::Error);
                } else {
                    self.set(
                        id,
                        if actual == Type::Error {
                            Type::Error
                        } else {
                            Type::String
                        },
                    );
                }
            }
            HirKind::InterpolatedString => {
                let error = node
                    .children
                    .iter()
                    .any(|c| self.ty(self.result.type_table[c.0]) == Type::Error);
                self.set(id, if error { Type::Error } else { Type::String });
            }
            HirKind::TypeName(_) | HirKind::UnitType => {
                self.type_syntax(id);
            }
            _ => self.set(id, Type::Unit),
        }
    }
}

/// No fixed-width accumulation until each digit is checked against the allowed
/// signed magnitude, so arbitrarily long source literals cannot overflow Rust.
fn integer(spelling: &str, negative: bool) -> Option<i32> {
    let (radix, digits) = if let Some(s) = spelling.strip_prefix("0x") {
        (16, s)
    } else if let Some(s) = spelling.strip_prefix("0b") {
        (2, s)
    } else if let Some(s) = spelling.strip_prefix("0o") {
        (8, s)
    } else {
        (10, spelling)
    };
    let limit = if negative {
        2_147_483_648u64
    } else {
        2_147_483_647
    };
    let mut value = 0u64;
    let mut previous_digit = false;
    let mut seen = false;
    for character in digits.chars() {
        if character == '_' {
            if !previous_digit {
                return None;
            }
            previous_digit = false;
            continue;
        }
        let digit = character.to_digit(radix)? as u64;
        value = value.checked_mul(radix as u64)?.checked_add(digit)?;
        if value > limit {
            return None;
        }
        previous_digit = true;
        seen = true;
    }
    if !seen || !previous_digit {
        return None;
    }
    let signed = if negative {
        -(value as i64)
    } else {
        value as i64
    };
    i32::try_from(signed).ok()
}
