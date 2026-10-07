//! P02 single-file semantic checking. Successful checking is not native execution.
mod aggregates;
mod arguments;
mod defaults;
pub use defaults::{DefaultArgument, ParameterDefault};
mod enums;
mod sums;
pub use arguments::NamedCall;
pub use enums::MatchPattern;
mod const_eval;
mod global_consts;
pub use const_eval::{ConstEvaluation, CONST_NODE_LIMIT};
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, Severity};
use nova_hir::{HirId, HirKind, Module};
use nova_resolve::{DefId, DefinitionKind, Resolution, Resolved};
use nova_source::Span;
use nova_syntax::Symbol;
use nova_types::{
    float_host_supported, FloatKind, FloatValue, IntKind, IntegerValue, Type, TypeId, TypeInterner,
};
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
    pub structs: nova_types::StructRegistry,
    pub enums: nova_types::EnumRegistry,
    pub sums: nova_types::SumRegistry,
    pub sum_origins: std::collections::BTreeMap<nova_types::EnumId, HirId>,
    pub variants: Vec<Option<nova_types::VariantId>>,
    pub patterns: Vec<Option<MatchPattern>>,
    pub exhaustive_matches: std::collections::BTreeSet<usize>,
    pub tuple_ids: std::collections::BTreeSet<nova_types::StructId>,
    pub tuple_origins: std::collections::BTreeMap<nova_types::StructId, HirId>,
    pub field_sources: std::collections::BTreeMap<nova_types::StructId, Vec<HirId>>,
    pub projections: Vec<Option<nova_types::FieldId>>,
    pub type_table: Vec<TypeId>,
    pub definition_types: Vec<TypeId>,
    pub signatures: Vec<Option<Signature>>,
    pub calls: Vec<Option<DefId>>,
    /// P17 source argument order and its bijection to declaration parameter order.
    pub named_calls: Vec<Option<NamedCall>>,
    /// P18 declaration defaults indexed by parameter HIR ID.
    pub defaults: Vec<Option<ParameterDefault>>,
    pub integer_values: Vec<Option<i32>>,
    /// P07 literal payloads; legacy Int32 payloads above remain available.
    pub integer_literals: Vec<Option<IntegerValue>>,
    pub float_literals: Vec<Option<FloatValue>>,
    /// Destination at each expression use; type_table retains its source type.
    pub coercions: Vec<Option<TypeId>>,
    pub always_returns: Vec<bool>,
    /// P05/P06 evaluation state, indexed by resolved DefId.
    pub const_values: Vec<ConstEvaluation>,
    pub diagnostics: Vec<Diagnostic>,
    upstream_errors: bool,
}
impl Checked {
    pub fn has_errors(&self) -> bool {
        self.upstream_errors
            || !self.diagnostics.is_empty()
            || self.const_values.iter().any(|v| {
                matches!(
                    v,
                    ConstEvaluation::Pending
                        | ConstEvaluation::Invalid
                        | ConstEvaluation::Failed { .. }
                )
            })
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
        for (index, value) in self.const_values.iter().enumerate() {
            if *value != ConstEvaluation::NotConstant {
                let _ = writeln!(output, "const def {index} {value:?}");
            }
        }
        for (index, value) in self.integer_literals.iter().enumerate() {
            if let Some(value) = value.filter(|v| v.kind() != IntKind::I32) {
                let _ = writeln!(output, "integer {index} {value:?}");
            }
        }
        for (index, dest) in self.coercions.iter().enumerate() {
            if let Some(value) = self.float_literals[index] {
                let _ = writeln!(output, "float {index} {value:?}");
            }
            if let Some(dest) = dest {
                let _ = writeln!(
                    output,
                    "widen {index} {:?} -> {:?}",
                    self.types.get(self.type_table[index]),
                    self.types.get(*dest)
                );
            }
        }
        for (index, default) in self.defaults.iter().enumerate() {
            if let Some(default) = default {
                let _ = writeln!(output, "default {index} {default:?}");
            }
        }
        for (index, call) in self.named_calls.iter().enumerate() {
            if let Some(call) = call.as_ref().filter(|c| !c.defaults.is_empty()) {
                let _ = writeln!(output, "default call {index} {call:?}");
            }
        }
        output
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckError {
    InvalidResolution,
    UnsupportedFloatHost,
}
impl fmt::Display for CheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidResolution => "resolution tables do not match the HIR module",
            Self::UnsupportedFloatHost => {
                "float evaluation requires an x86_64 host with controlled IEEE environment"
            }
        })
    }
}
impl std::error::Error for CheckError {}

struct Checker<'a> {
    parameter_indices:
        std::collections::HashMap<usize, std::collections::HashMap<nova_hir::SymbolId, usize>>,
    argument_mappings: Vec<Option<arguments::ArgumentMapping>>,
    default_functions: std::collections::HashSet<usize>,
    sum_shapes: std::collections::HashMap<nova_types::SumKey, nova_types::EnumId>,
    sum_sources: std::collections::HashMap<nova_types::SumKey, HirId>,
    attempted_variants: std::collections::BTreeSet<usize>,
    type_syntax_seen: std::collections::BTreeSet<usize>,
    tuple_shapes: std::collections::HashMap<Vec<Type>, nova_types::StructId>,
    tuple_shape_sources: std::collections::HashMap<Vec<Type>, HirId>,
    module: &'a Module,
    resolved: &'a Resolved,
    result: Checked,
    flow: Vec<Flow>,
    literal_only: Vec<bool>,
    float_literal_only: Vec<bool>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
enum Flow {
    Fallthrough,
    Return,
    Jump,
}
#[derive(Clone, Copy)]
struct Context {
    expected: Option<TypeId>,
    expected_span: Option<Span>,
    return_type: TypeId,
    return_span: Option<Span>,
    direct_callee: bool,
    loop_depth: usize,
    const_declaration: Option<HirId>,
}
enum Work {
    Enter(HirId, Context),
    Exit(HirId, Context),
    AssignmentValue(HirId, Context),
    MatchArms(HirId, Context),
    Peer {
        literal: HirId,
        typed: HirId,
        context: Context,
        floating: bool,
    },
}

pub fn check(module: &Module, resolved: &Resolved) -> Result<Checked, CheckError> {
    if !float_host_supported() && module.nodes().iter().any(|node| matches!(node.kind, HirKind::Float(_)) || matches!(node.kind, HirKind::TypeName(name) if matches!(module.symbol(name), Some("float32" | "float64")))) {
        return Err(CheckError::UnsupportedFloatHost);
    }
    // Names, scopes and declaration kinds are public tables; verify their exact provenance.
    if nova_resolve::resolve(module) != *resolved {
        return Err(CheckError::InvalidResolution);
    }
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
            DefinitionKind::BuiltinPrint => {
                if definition.mutable || definition.constant {
                    return Err(CheckError::InvalidResolution);
                }
                continue;
            }
            DefinitionKind::Function(id)
            | DefinitionKind::Enum(id)
            | DefinitionKind::Struct(id)
            | DefinitionKind::GlobalConst(id)
            | DefinitionKind::Parameter(id)
            | DefinitionKind::Local(id) => id,
        };
        let Some(node) = module.node(id) else {
            return Err(CheckError::InvalidResolution);
        };
        let name = match (definition.kind, &node.kind) {
            (DefinitionKind::Enum(_), HirKind::Enum { name, .. })
            | (DefinitionKind::Local(_), HirKind::Binder { name, .. })
            | (DefinitionKind::Struct(_), HirKind::Struct { name, .. })
            | (DefinitionKind::Function(_), HirKind::Function { name, .. })
            | (DefinitionKind::Parameter(_), HirKind::Parameter { name, .. })
            | (DefinitionKind::Local(_), HirKind::Binding { name, .. }) => name,
            (
                DefinitionKind::GlobalConst(_),
                HirKind::Binding {
                    name,
                    constant: true,
                    mutable: false,
                    ..
                },
            ) => name,
            _ => return Err(CheckError::InvalidResolution),
        };
        if module.symbol(*name) != Some(definition.name.as_str())
            || resolved.declaration_ids[id.0] != Some(DefId(index))
            || definition.mutable != matches!(node.kind, HirKind::Binding { mutable: true, .. })
            || definition.constant != matches!(node.kind, HirKind::Binding { constant: true, .. })
        {
            return Err(CheckError::InvalidResolution);
        }
    }
    for (index, reference) in resolved.references.iter().enumerate() {
        // An alias spelling differs from its original definition. Exact
        // resolver recomputation above already verifies the target identity.
        if reference.is_some() && !matches!(module.nodes()[index].kind, HirKind::Name(_)) {
            return Err(CheckError::InvalidResolution);
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
        structs: Default::default(),
        enums: Default::default(),
        sums: Default::default(),
        sum_origins: Default::default(),
        variants: vec![None; size],
        patterns: vec![None; size],
        exhaustive_matches: Default::default(),
        tuple_ids: Default::default(),
        tuple_origins: Default::default(),
        field_sources: Default::default(),
        projections: vec![None; size],
        type_table: vec![error; size],
        definition_types: vec![error; resolved.definitions.len()],
        signatures: vec![None; resolved.definitions.len()],
        calls: vec![None; size],
        named_calls: vec![None; size],
        defaults: vec![None; size],
        integer_values: vec![None; size],
        integer_literals: vec![None; size],
        float_literals: vec![None; size],
        coercions: vec![None; size],
        always_returns: vec![false; size],
        const_values: resolved
            .definitions
            .iter()
            .map(|d| {
                if d.constant {
                    ConstEvaluation::Pending
                } else {
                    ConstEvaluation::NotConstant
                }
            })
            .collect(),
        diagnostics: resolved.diagnostics.clone(),
        upstream_errors: module.has_errors(),
    };
    let float_literal_only =
        module
            .nodes()
            .iter()
            .fold(Vec::with_capacity(size), |mut flags, node| {
                let allowed = matches!(
                    node.kind,
                    HirKind::Float(_)
                        | HirKind::Group
                        | HirKind::Prefix(Symbol::Plus | Symbol::Minus)
                        | HirKind::Binary(
                            Symbol::Plus | Symbol::Minus | Symbol::Star | Symbol::Slash
                        )
                );
                flags.push(allowed && node.children.iter().all(|child| flags[child.0]));
                flags
            });
    let mut checker = Checker {
        parameter_indices: Default::default(),
        argument_mappings: (0..size).map(|_| None).collect(),
        default_functions: Default::default(),
        sum_shapes: Default::default(),
        sum_sources: Default::default(),
        attempted_variants: Default::default(),
        type_syntax_seen: Default::default(),
        tuple_shapes: Default::default(),
        tuple_shape_sources: Default::default(),
        module,
        resolved,
        result,
        flow: vec![Flow::Fallthrough; size],
        float_literal_only,
        literal_only: module
            .nodes()
            .iter()
            .fold(Vec::with_capacity(size), |mut flags, node| {
                let allowed = matches!(
                    node.kind,
                    HirKind::Integer(_)
                        | HirKind::Group
                        | HirKind::Prefix(Symbol::Plus | Symbol::Minus)
                        | HirKind::Binary(
                            Symbol::Plus
                                | Symbol::Minus
                                | Symbol::Star
                                | Symbol::Slash
                                | Symbol::Percent
                        )
                );
                flags.push(allowed && node.children.iter().all(|child| flags[child.0]));
                flags
            }),
    };
    checker.collect_structs();
    checker.collect_signatures();
    checker.collect_parameter_indices();
    checker.check_globals();
    checker.check_defaults();
    let items = module.items().collect::<Vec<_>>();
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
            loop_depth: 0,
            const_declaration: None,
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
    for (index, node) in module.nodes().iter().enumerate() {
        if matches!(
            node.kind,
            HirKind::Module
                | HirKind::Visible { .. }
                | HirKind::Import { .. }
                | HirKind::ImportSegment(_)
        ) {
            checker.set(HirId(index), Type::Unit);
        }
    }
    checker.check_tuple_limit();
    checker.check_sum_limit();
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
        let mut work = vec![(id, false)];
        while let Some((at, exit)) = work.pop() {
            if self.type_syntax_seen.contains(&at.0) {
                continue;
            }
            let node = &self.module.nodes()[at.0];
            if matches!(
                node.kind,
                HirKind::TupleType | HirKind::NullableType | HirKind::GenericType { .. }
            ) && !exit
            {
                work.push((at, true));
                work.extend(node.children.iter().rev().map(|&c| (c, false)));
                continue;
            }
            if node.kind == HirKind::TupleType {
                let fields = node
                    .children
                    .iter()
                    .map(|c| self.ty(self.result.type_table[c.0]))
                    .collect();
                let ty = self.intern_tuple(at, fields, false);
                self.set(at, ty);
            } else if matches!(
                node.kind,
                HirKind::NullableType | HirKind::GenericType { .. }
            ) {
                let ty = self.sum_type_syntax(at);
                self.set(at, ty);
            } else {
                self.type_syntax_atom(at);
            }
            self.type_syntax_seen.insert(at.0);
        }
        self.result.type_table[id.0]
    }
    fn type_syntax_atom(&mut self, id: HirId) -> TypeId {
        let node = &self.module.nodes()[id.0];
        let ty = match node.kind {
            HirKind::UnitType => Type::Unit,
            HirKind::TypeName(name) => {
                match self.module.symbol(name).expect("type symbol exists") {
                    "int32" => Type::Int32,
                    "bool" => Type::Bool,
                    "char" => Type::Char,
                    "string" => Type::String,
                    "int8" => Type::Int8,
                    "int16" => Type::Int16,
                    "int64" => Type::Int64,
                    "uint8" => Type::UInt8,
                    "uint16" => Type::UInt16,
                    "uint32" => Type::UInt32,
                    "uint64" => Type::UInt64,
                    "float32" => Type::Float32,
                    "float64" => Type::Float64,
                    "never" => {
                        self.report(
                            1102,
                            node.span,
                            "this Primitive type is outside the approved subset",
                            None,
                        );
                        Type::Error
                    }
                    "Option" | "Result" if self.builtin_family(id, name).is_some() => {
                        self.report(
                            2101,
                            node.span,
                            "builtin family requires type arguments",
                            None,
                        );
                        Type::Error
                    }
                    _ => self.struct_type_name(id, name),
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
                DefinitionKind::Struct(id) => {
                    let sid = nova_types::StructId(index);
                    let shape = &self.result.structs[&sid];
                    let fields = shape.fields.clone();
                    self.result.definition_types[index] = self.result.types.intern(Type::Function);
                    self.result.signatures[index] = Some(Signature {
                        parameters: fields
                            .iter()
                            .map(|f| self.result.types.intern(f.ty))
                            .collect(),
                        return_type: self.result.types.intern(Type::Struct(sid)),
                        parameter_spans: self.result.field_sources[&sid]
                            .iter()
                            .map(|f| Some(self.module.nodes()[f.0].span))
                            .collect(),
                        return_span: Some(self.module.nodes()[id.0].span),
                    });
                }
                DefinitionKind::Enum(_) => {
                    self.result.definition_types[index] = self.result.types.intern(Type::Unit);
                }
                DefinitionKind::Parameter(id) => {
                    let ty_id = self.module.nodes()[id.0].children[0];
                    // Function signatures have already visited parameter syntax.
                    let ty = self.result.type_table[ty_id.0];
                    self.result.definition_types[index] = ty;
                    self.result.type_table[id.0] = ty;
                }
                DefinitionKind::Local(_) | DefinitionKind::GlobalConst(_) => {}
            }
        }
    }
    fn mismatch(&mut self, id: HirId, expected: TypeId, secondary: Option<Span>) -> bool {
        let actual = self.result.type_table[id.0];
        if self.result.types.compatible(actual, expected) {
            return false;
        }
        if self.ty(actual).widens_to(self.ty(expected)) {
            self.result.coercions[id.0] = Some(expected);
            return false;
        }
        {
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
        }
    }
    fn literal(&mut self, id: HirId, spelling: &str, negative: bool, context: Context) {
        if context
            .expected
            .is_some_and(|ty| self.ty(ty) == Type::Error)
        {
            self.set(id, Type::Error);
            return;
        }
        let kind = context
            .expected
            .and_then(|ty| self.ty(ty).integer())
            .unwrap_or(IntKind::I32);
        if let Some(value) = integer(spelling, negative, kind) {
            self.set(id, kind.ty());
            self.result.integer_literals[id.0] = Some(value);
            if kind == IntKind::I32 {
                self.result.integer_values[id.0] = Some(value.value() as i32);
            }
        } else {
            self.report(
                2102,
                self.module.nodes()[id.0].span,
                "integer literal is outside its expected/default range",
                context.expected_span,
            );
            self.set(id, Type::Error);
        }
    }
    fn walk(&mut self, root: HirId, context: Context) {
        let mut pending = vec![Work::Enter(root, context)];
        while let Some(work) = pending.pop() {
            let (id, context, exit) = match work {
                Work::Peer {
                    literal,
                    typed,
                    context,
                    floating,
                } => {
                    let peer = self.result.type_table[typed.0];
                    let expected = if (if floating {
                        self.ty(peer).float().is_some()
                    } else {
                        self.ty(peer).integer().is_some()
                    }) || self.ty(peer) == Type::Error
                    {
                        Some(peer)
                    } else {
                        None
                    };
                    pending.push(Work::Enter(
                        literal,
                        Context {
                            expected,
                            expected_span: Some(self.module.nodes()[typed.0].span),
                            ..context
                        },
                    ));
                    continue;
                }
                Work::MatchArms(id, cx) => {
                    self.prepare_match(id);
                    for &arm in self.module.nodes()[id.0].children[1..].iter().rev() {
                        if self.module.nodes()[arm.0].kind == HirKind::Arm {
                            pending.push(Work::Enter(
                                self.module.nodes()[arm.0].children[1],
                                Context {
                                    expected: None,
                                    expected_span: None,
                                    direct_callee: false,
                                    ..cx
                                },
                            ));
                        }
                    }
                    continue;
                }
                Work::AssignmentValue(id, cx) => {
                    let n = &self.module.nodes()[id.0];
                    let expected = if self.assignment_target(n.children[0]) {
                        self.result.type_table[n.children[0].0]
                    } else {
                        self.result.types.intern(Type::Error)
                    };
                    pending.push(Work::Enter(
                        n.children[1],
                        Context {
                            expected: Some(expected),
                            expected_span: Some(self.module.nodes()[n.children[0].0].span),
                            direct_callee: false,
                            ..cx
                        },
                    ));
                    continue;
                }
                Work::Enter(id, cx) => (id, cx, false),
                Work::Exit(id, cx) => (id, cx, true),
            };
            if exit {
                self.finish(id, context);
                self.result.always_returns[id.0] = self.flow[id.0] == Flow::Return;
                self.apply_expected(id, context);
                continue;
            }
            let node = &self.module.nodes()[id.0];
            pending.push(Work::Exit(id, context));
            if node.kind == HirKind::Call {
                self.prepare_named_call(id);
            }
            match node.kind {
                HirKind::Match { .. } => {
                    pending.push(Work::MatchArms(id, context));
                    pending.push(Work::Enter(
                        node.children[0],
                        Context {
                            expected: None,
                            expected_span: None,
                            direct_callee: false,
                            ..context
                        },
                    ));
                }
                HirKind::Assignment => {
                    let target = node.children[0];
                    pending.push(Work::AssignmentValue(id, context));
                    pending.push(Work::Enter(
                        target,
                        Context {
                            expected: None,
                            expected_span: None,
                            direct_callee: true,
                            ..context
                        },
                    ));
                }
                HirKind::While => {
                    pending.push(Work::Enter(
                        node.children[1],
                        Context {
                            expected: None,
                            expected_span: None,
                            direct_callee: false,
                            loop_depth: context.loop_depth + 1,
                            ..context
                        },
                    ));
                    pending.push(Work::Enter(
                        node.children[0],
                        Context {
                            expected: None,
                            expected_span: None,
                            direct_callee: false,
                            ..context
                        },
                    ));
                }
                HirKind::Binding {
                    has_type, constant, ..
                } => {
                    let expected = if has_type {
                        Some(self.type_syntax(node.children[0]))
                    } else {
                        None
                    };
                    pending.push(Work::Enter(
                        *node.children.last().expect("binding initializer"),
                        Context {
                            expected,
                            const_declaration: if constant {
                                Some(id)
                            } else {
                                context.const_declaration
                            },
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
                HirKind::Call
                    if matches!(
                        self.module.nodes()[node.children[0].0].kind,
                        HirKind::VariantPath { .. }
                    ) =>
                {
                    let head = node.children[0];
                    let builtin = self.sum_head(head).is_some();
                    let variant = if builtin {
                        self.prepare_sum(head, context.expected)
                    } else {
                        self.resolve_variant(head)
                    };
                    self.result.variants[id.0] = variant;
                    if self.has_named(id) && (builtin || variant.is_some()) {
                        self.reject_named_constructor(id);
                    }
                    for (at, &child) in node.children.iter().enumerate().rev() {
                        let expected = variant
                            .and_then(|v| self.result.enums.get(&v.enumeration))
                            .and_then(|s| {
                                s.variants[variant.expect("variant").index]
                                    .fields
                                    .get(at.saturating_sub(1))
                            })
                            .map(|f| self.result.types.intern(f.ty));
                        if at == 0 && builtin {
                            self.set(child, Type::Unit);
                            continue;
                        }
                        pending.push(Work::Enter(
                            child,
                            Context {
                                expected: if at == 0 || self.argument_mappings[id.0].is_some() {
                                    None
                                } else {
                                    expected
                                },
                                expected_span: None,
                                direct_callee: at == 0,
                                ..context
                            },
                        ));
                    }
                }
                HirKind::Call => {
                    let def = self.callee_definition(node.children[0]);
                    for (index, &child) in node.children.iter().enumerate().rev() {
                        let parameter = if index == 0 {
                            None
                        } else {
                            self.argument_mappings[id.0]
                                .as_ref()
                                .map_or(Some(index - 1), |m| m.parameters[index - 1])
                        };
                        let expected = if index == 0 {
                            None
                        } else {
                            def.and_then(|d| self.result.signatures[d.0].as_ref())
                                .and_then(|s| parameter.and_then(|p| s.parameters.get(p)))
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
                                        .and_then(|s| {
                                            parameter.and_then(|p| s.parameter_spans.get(p))
                                        })
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
                    let child = node.children[0];
                    let HirKind::Integer(ref spelling) = self.module.nodes()[child.0].kind else {
                        unreachable!()
                    };
                    self.literal(id, spelling, true, context);
                    let ty = self.result.type_table[id.0];
                    self.result.type_table[child.0] = ty;
                    pending.pop();
                    self.apply_expected(id, context);
                }
                HirKind::Binary(op) if !matches!(op, Symbol::AndAnd | Symbol::OrOr) => {
                    let left = node.children[0];
                    let right = node.children[1];
                    let clean = Context {
                        expected: None,
                        expected_span: None,
                        direct_callee: false,
                        ..context
                    };
                    let arithmetic = matches!(
                        op,
                        Symbol::Plus
                            | Symbol::Minus
                            | Symbol::Star
                            | Symbol::Slash
                            | Symbol::Percent
                    );
                    let expected = context.expected.filter(|ty| {
                        arithmetic
                            && (self.ty(*ty).integer().is_some()
                                || self.ty(*ty) == Type::Error
                                || (op != Symbol::Percent
                                    && self.ty(*ty).float().is_some()
                                    && (self.float_literal_only[left.0]
                                        || self.float_literal_only[right.0])))
                    });
                    if let Some(expected) = expected {
                        for child in [right, left] {
                            let cx = if if self.ty(expected).float().is_some() {
                                self.float_literal_only[child.0]
                            } else {
                                self.literal_only[child.0]
                            } {
                                Context {
                                    expected: Some(expected),
                                    ..context
                                }
                            } else {
                                clean
                            };
                            pending.push(Work::Enter(child, cx));
                        }
                    } else if self.literal_only[left.0] != self.literal_only[right.0]
                        || self.float_literal_only[left.0] != self.float_literal_only[right.0]
                    {
                        let floating = self.literal_only[left.0] == self.literal_only[right.0];
                        let flags = if floating {
                            &self.float_literal_only
                        } else {
                            &self.literal_only
                        };
                        let (literal, typed) = if flags[left.0] {
                            (left, right)
                        } else {
                            (right, left)
                        };
                        pending.push(Work::Peer {
                            literal,
                            typed,
                            context: clean,
                            floating,
                        });
                        pending.push(Work::Enter(typed, clean));
                    } else {
                        pending.push(Work::Enter(right, clean));
                        pending.push(Work::Enter(left, clean));
                    }
                }
                HirKind::Prefix(Symbol::Plus | Symbol::Minus) => {
                    let child = node.children[0];
                    let cx = if self.literal_only[child.0] || self.float_literal_only[child.0] {
                        context
                    } else {
                        Context {
                            expected: None,
                            expected_span: None,
                            ..context
                        }
                    };
                    pending.push(Work::Enter(child, cx));
                }
                HirKind::Try { .. } => {
                    pending.push(Work::Enter(
                        node.children[0],
                        Context {
                            expected: None,
                            expected_span: None,
                            direct_callee: false,
                            ..context
                        },
                    ));
                }
                HirKind::Cast { .. } => {
                    pending.push(Work::Enter(
                        node.children[1],
                        Context {
                            expected: None,
                            expected_span: None,
                            direct_callee: false,
                            ..context
                        },
                    ));
                    // Allow inspection of a bare function's type, solely so the
                    // cast itself reports N2101 rather than first-class N1102.
                    pending.push(Work::Enter(
                        node.children[0],
                        Context {
                            expected: None,
                            expected_span: None,
                            direct_callee: true,
                            ..context
                        },
                    ));
                }
                HirKind::Tuple => {
                    let expected = context.expected.map(|t| self.ty(t));
                    let fields = expected.and_then(|ty| {
                        if let Type::Tuple(sid) = ty {
                            Some(self.result.structs[&sid].fields.clone())
                        } else {
                            None
                        }
                    });
                    for (index, &child) in node.children.iter().enumerate().rev() {
                        let expected = fields
                            .as_ref()
                            .filter(|f| f.len() == node.children.len())
                            .map(|f| self.result.types.intern(f[index].ty));
                        pending.push(Work::Enter(
                            child,
                            Context {
                                expected,
                                direct_callee: false,
                                ..context
                            },
                        ));
                    }
                }
                HirKind::Group | HirKind::NamedArgument { .. } => {
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
                DefinitionKind::Function(_)
                    | DefinitionKind::BuiltinPrint
                    | DefinitionKind::Struct(_)
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
            HirKind::Match { .. } => {
                let flows = node.children[1..]
                    .iter()
                    .filter(|a| self.module.nodes()[a.0].kind == HirKind::Arm)
                    .map(|a| self.flow[self.module.nodes()[a.0].children[1].0])
                    .collect::<Vec<_>>();
                if self.result.exhaustive_matches.contains(&id.0) && !flows.is_empty() {
                    self.flow[id.0] = if flows.iter().all(|f| *f == Flow::Return) {
                        Flow::Return
                    } else if flows.iter().all(|f| *f != Flow::Fallthrough) {
                        Flow::Jump
                    } else {
                        Flow::Fallthrough
                    };
                }
                self.set(id, Type::Unit);
            }
            HirKind::Try { keyword } => {
                let operand = node.children[0];
                let source = self.ty(self.result.type_table[operand.0]);
                if let Some(declaration) = context.const_declaration {
                    self.report(
                        3201,
                        *keyword,
                        "try is not permitted in a const initializer",
                        Some(self.module.nodes()[declaration.0].span),
                    );
                    self.set(id, Type::Error);
                    return;
                }
                if source == Type::Error {
                    self.set(id, Type::Error);
                    return;
                }
                let source_key = match source {
                    Type::Enum(e) => self
                        .result
                        .sums
                        .get(&e)
                        .filter(|k| k.family == nova_types::SumFamily::Result)
                        .cloned(),
                    _ => None,
                };
                let destination = self.ty(context.return_type);
                let destination_key = match destination {
                    Type::Enum(e) => self
                        .result
                        .sums
                        .get(&e)
                        .filter(|k| k.family == nova_types::SumFamily::Result)
                        .cloned(),
                    _ => None,
                };
                let mut invalid = false;
                if source_key.is_none() {
                    self.report(
                        2101,
                        self.module.nodes()[operand.0].span,
                        "try operand must be an intrinsic Result",
                        None,
                    );
                    invalid = true;
                }
                if destination_key.is_none() && destination != Type::Error {
                    self.report(
                        3002,
                        *keyword,
                        "try requires an intrinsic Result function return type",
                        context.return_span,
                    );
                    invalid = true;
                }
                if let (Some(from), Some(to)) = (&source_key, &destination_key) {
                    if from.arguments[1] != to.arguments[1] {
                        self.report(
                            2101,
                            *keyword,
                            "try Error types must be exactly identical",
                            context.return_span,
                        );
                        invalid = true;
                    }
                }
                self.set(
                    id,
                    if invalid || destination == Type::Error {
                        Type::Error
                    } else {
                        source_key.map_or(Type::Error, |k| k.arguments[0])
                    },
                );
            }
            HirKind::None => self.finish_sum_value(id, context.expected),
            HirKind::VariantPath { .. }
                if self.sum_head(id).is_some() && !context.direct_callee =>
            {
                self.finish_sum_value(id, context.expected)
            }
            HirKind::VariantPath { .. } => self.finish_variant(id, context.direct_callee),
            HirKind::Call if self.argument_mappings[id.0].is_some() => self.finish_named_call(id),
            HirKind::Call if self.sum_head(node.children[0]).is_some() => {
                self.finish_sum_call(id, context.expected)
            }
            HirKind::Call if self.result.variants[id.0].is_some() => self.finish_variant_call(id),
            HirKind::Error => self.set(id, Type::Error),
            HirKind::Integer(spelling) => self.literal(id, spelling, false, context),
            HirKind::Float(spelling) => {
                if context
                    .expected
                    .is_some_and(|ty| self.ty(ty) == Type::Error)
                {
                    self.set(id, Type::Error);
                    return;
                }
                let kind = context
                    .expected
                    .and_then(|ty| self.ty(ty).float())
                    .unwrap_or(FloatKind::F32);
                match FloatValue::parse_decimal(kind, spelling) {
                    Ok(value) => {
                        self.set(id, kind.ty());
                        self.result.float_literals[id.0] = Some(value);
                    }
                    Err(_) => {
                        self.report(
                            2102,
                            node.span,
                            "float literal is outside its expected/default finite range",
                            context.expected_span,
                        );
                        self.set(id, Type::Error);
                    }
                }
            }
            HirKind::Character(_) => self.set(id, Type::Char),
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
            HirKind::Group | HirKind::NamedArgument { .. } => {
                self.result.type_table[id.0] = self.result.coercions[node.children[0].0]
                    .unwrap_or(self.result.type_table[node.children[0].0])
            }
            HirKind::Binding {
                has_type, constant, ..
            } => {
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
                    if *constant {
                        if self.ty(ty) == Type::Error {
                            self.result.const_values[def.0] = ConstEvaluation::Invalid;
                        } else {
                            match const_eval::evaluate(
                                self.module,
                                self.resolved,
                                &self.result,
                                value,
                            ) {
                                Ok((value, nodes)) => {
                                    self.result.const_values[def.0] =
                                        ConstEvaluation::Value { value, nodes };
                                }
                                Err(error) => {
                                    self.report(
                                        error.code,
                                        self.module.nodes()[error.node.0].span,
                                        error.message,
                                        self.resolved.definitions[def.0].span,
                                    );
                                    if error.code == 3202 {
                                        self.result
                                            .diagnostics
                                            .last_mut()
                                            .expect("reported const budget")
                                            .notes
                                            .push(format!(
                                                "const initializer node limit: {CONST_NODE_LIMIT}"
                                            ));
                                    }
                                    self.result.const_values[def.0] = ConstEvaluation::Failed {
                                        code: error.code,
                                        nodes: error.nodes,
                                    };
                                    self.result.definition_types[def.0] =
                                        self.result.types.intern(Type::Error);
                                }
                            }
                        }
                    }
                }
                self.set(id, Type::Unit);
            }
            HirKind::Return => {
                self.flow[id.0] = Flow::Return;
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
                self.set(id, Type::Unit);
            }
            HirKind::Assignment => self.set(id, Type::Unit),
            HirKind::Tuple => {
                if let Some(expected) = context.expected {
                    if let Type::Tuple(sid) = self.ty(expected) {
                        let fields = self.result.structs[&sid].fields.clone();
                        if fields.len() == node.children.len() {
                            let mut invalid = false;
                            for (&child, field) in node.children.iter().zip(fields) {
                                let expected = self.result.types.intern(field.ty);
                                invalid |= self.mismatch(child, expected, None);
                            }
                            if invalid {
                                self.set(id, Type::Error);
                                return;
                            }
                        }
                    }
                }
                let fields = node
                    .children
                    .iter()
                    .map(|c| {
                        self.ty(self.result.coercions[c.0].unwrap_or(self.result.type_table[c.0]))
                    })
                    .collect();
                let ty = self.intern_tuple(id, fields, true);
                self.set(id, ty);
            }
            HirKind::TupleProjection { index, index_span } => {
                self.tuple_projection(id, *index, *index_span)
            }
            HirKind::Projection { name, name_span } => self.projection(id, *name, *name_span),
            HirKind::Break | HirKind::Continue => {
                if context.loop_depth == 0 {
                    self.report(
                        3002,
                        node.span,
                        "jump requires an enclosing while loop",
                        None,
                    );
                }
                self.flow[id.0] = Flow::Jump;
                self.set(id, Type::Unit);
            }
            HirKind::If | HirKind::While => {
                let condition = node.children[0];
                if !matches!(
                    self.ty(self.result.type_table[condition.0]),
                    Type::Bool | Type::Error
                ) {
                    self.report(
                        3001,
                        self.module.nodes()[condition.0].span,
                        "condition must be Bool",
                        None,
                    );
                }
                if node.kind == HirKind::If && node.children.len() == 3 {
                    let then_flow = self.flow[node.children[1].0];
                    let else_flow = self.flow[node.children[2].0];
                    self.flow[id.0] = if then_flow == Flow::Return && else_flow == Flow::Return {
                        Flow::Return
                    } else if then_flow != Flow::Fallthrough && else_flow != Flow::Fallthrough {
                        Flow::Jump
                    } else {
                        Flow::Fallthrough
                    };
                }
                self.set(id, Type::Unit);
            }
            HirKind::Block => {
                self.flow[id.0] = node
                    .children
                    .iter()
                    .map(|c| self.flow[c.0])
                    .find(|flow| *flow != Flow::Fallthrough)
                    .unwrap_or(Flow::Fallthrough);
                self.set(id, Type::Unit);
            }
            HirKind::ExpressionStatement => self.set(id, Type::Unit),
            HirKind::Prefix(op) => {
                let operand = node.children[0];
                let ty = self
                    .ty(self.result.coercions[operand.0]
                        .unwrap_or(self.result.type_table[operand.0]));
                let valid = if *op == Symbol::Bang {
                    ty == Type::Bool
                } else {
                    ty.float().is_some()
                        || ty
                            .integer()
                            .is_some_and(|kind| *op == Symbol::Plus || kind.signed())
                };
                if !valid && ty != Type::Error {
                    self.report(2101, node.span, "invalid unary operand type", None);
                }
                self.set(id, if valid { ty } else { Type::Error });
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
                let char_valid = left == Type::Char && right == Type::Char && comparison;
                let bool_valid = left == Type::Bool
                    && right == Type::Bool
                    && (logical || matches!(op, Symbol::EqualEqual | Symbol::BangEqual));
                let common = if logical {
                    None
                } else {
                    left.common_numeric(right)
                        .filter(|ty| ty.float().is_none() || *op != Symbol::Percent)
                };
                if let Some(common) = common {
                    let expected = self.result.types.intern(common);
                    self.mismatch(node.children[0], expected, None);
                    self.mismatch(node.children[1], expected, None);
                    self.set(id, if comparison { Type::Bool } else { common });
                } else if bool_valid || char_valid {
                    self.set(id, Type::Bool);
                } else {
                    self.report(
                        2101,
                        node.span,
                        "operator operand types have no supported common type",
                        None,
                    );
                    self.set(id, Type::Error);
                }
            }
            HirKind::Call => {
                let callee = node.children[0];
                if let Some(def) = self.callee_definition(callee) {
                    let signature = self.result.signatures[def.0]
                        .clone()
                        .expect("function signature collected");
                    self.result.calls[id.0] = Some(def);
                    let mut wrong = !self.constructor_visible(def, node.span);
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
                            if matches!(self.module.nodes()[callee.0].kind,HirKind::Name(name) if self.type_definition(callee,name).is_some()) { node.span } else { self.module.nodes()[callee.0].span },
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
                if !actual.numeric()
                    && !matches!(actual, Type::Bool | Type::Char | Type::String | Type::Error)
                {
                    self.report(
                        2101,
                        self.module.nodes()[value.0].span,
                        "interpolation requires numeric, Bool, Char or String",
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
            HirKind::Cast { .. } => {
                let source = self.ty(self.result.type_table[node.children[0].0]);
                let dest = self.ty(self.result.type_table[node.children[1].0]);
                if source == Type::Error || dest == Type::Error {
                    self.set(id, Type::Error);
                } else if source.numeric() && dest.numeric() {
                    self.set(id, dest);
                } else {
                    self.report(
                        2101,
                        node.span,
                        "as requires numeric source and target types",
                        Some(self.module.nodes()[node.children[1].0].span),
                    );
                    self.set(id, Type::Error);
                }
            }
            HirKind::TypeName(_)
            | HirKind::UnitType
            | HirKind::TupleType
            | HirKind::GenericType { .. }
            | HirKind::NullableType => {
                self.type_syntax(id);
            }
            _ => self.set(id, Type::Unit),
        }
    }
}

/// No fixed-width accumulation until each digit is checked against the allowed
/// signed magnitude, so arbitrarily long source literals cannot overflow Rust.
fn integer(spelling: &str, negative: bool, kind: IntKind) -> Option<IntegerValue> {
    if negative && !kind.signed() {
        return None;
    }
    let (radix, digits) = if let Some(s) = spelling.strip_prefix("0x") {
        (16, s)
    } else if let Some(s) = spelling.strip_prefix("0b") {
        (2, s)
    } else if let Some(s) = spelling.strip_prefix("0o") {
        (8, s)
    } else {
        (10, spelling)
    };
    let limit = if negative { -kind.min() } else { kind.max() } as u64;
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
        -(value as i128)
    } else {
        value as i128
    };
    IntegerValue::new(kind, signed)
}
