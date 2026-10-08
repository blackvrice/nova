//! Stage A non-SSA MIR. Arithmetic and formatting remain abstract Nova operations.
//! This IR is not permission to select a runtime overflow, print, or ABI policy.
pub use nova_typecheck::MatchPattern;
mod lower;
mod validate;

pub use lower::{lower, LoweringError};
use nova_hir::{HirId, SourceOrigin};
use nova_resolve::DefId;
use nova_source::Span;
use nova_syntax::Symbol;
use nova_types::{
    ConstValue, FieldId, FloatValue, IntKind, IntegerValue, StructId, StructRegistry, Type,
};
use std::fmt::Write;
pub use validate::{validate, ValidationError, Violation};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BlockId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LocalId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalleeId(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceInfo {
    pub hir: HirId,
    pub span: Span,
    pub origin: SourceOrigin,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Callee {
    pub definition: DefId,
    pub name: String,
    pub parameters: Vec<Type>,
    pub return_type: Type,
    pub builtin_print: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Local {
    pub ty: Type,
    /// None denotes a compiler temporary, not a source declaration.
    pub definition: Option<DefId>,
    pub source: SourceInfo,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Place(pub LocalId);
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Int32(i32),
    Integer(IntegerValue),
    Float(FloatValue),
    Bool(bool),
    Char(char),
    String(String),
    Unit,
    Struct(StructId, Vec<Constant>),
    Tuple(StructId, Vec<Constant>),
    Enum(nova_types::VariantId, Vec<Constant>),
}
impl Constant {
    pub fn from_const(value: &ConstValue) -> Self {
        enum Work<'a> {
            Value(&'a ConstValue),
            Aggregate(StructId, usize, bool),
            Enumeration(nova_types::VariantId, usize),
        }
        let mut work = vec![Work::Value(value)];
        let mut values = vec![];
        while let Some(task) = work.pop() {
            match task {
                Work::Enumeration(id, count) => {
                    let fields = values.split_off(values.len() - count);
                    values.push(Self::Enum(id, fields));
                }
                Work::Aggregate(id, count, tuple) => {
                    let fields = values.split_off(values.len() - count);
                    values.push(if tuple {
                        Self::Tuple(id, fields)
                    } else {
                        Self::Struct(id, fields)
                    });
                }
                Work::Value(value) => match value {
                    ConstValue::Int32(v) => values.push(Self::Int32(*v)),
                    ConstValue::Integer(v) => values.push(Self::from_integer(*v)),
                    ConstValue::Float(v) => values.push(Self::Float(*v)),
                    ConstValue::Bool(v) => values.push(Self::Bool(*v)),
                    ConstValue::Char(v) => values.push(Self::Char(*v)),
                    ConstValue::String(v) => values.push(Self::String(v.clone())),
                    ConstValue::Unit => values.push(Self::Unit),
                    ConstValue::Enum(id, fields) => {
                        work.push(Work::Enumeration(*id, fields.len()));
                        work.extend(fields.iter().rev().map(Work::Value));
                    }
                    ConstValue::Struct(id, fields) | ConstValue::Tuple(id, fields) => {
                        work.push(Work::Aggregate(
                            *id,
                            fields.len(),
                            matches!(value, ConstValue::Tuple(..)),
                        ));
                        work.extend(fields.iter().rev().map(Work::Value));
                    }
                },
            }
        }
        values.pop().expect("one converted constant")
    }
    pub fn from_integer(value: IntegerValue) -> Self {
        if value.kind() == IntKind::I32 {
            Self::Int32(value.value() as i32)
        } else {
            Self::Integer(value)
        }
    }
    pub fn ty(&self) -> Type {
        match self {
            Self::Int32(_) => Type::Int32,
            Self::Integer(value) => value.kind().ty(),
            Self::Float(value) => value.kind().ty(),
            Self::Bool(_) => Type::Bool,
            Self::Char(_) => Type::Char,
            Self::String(_) => Type::String,
            Self::Unit => Type::Unit,
            Self::Struct(id, _) => Type::Struct(*id),
            Self::Tuple(id, _) => Type::Tuple(*id),
            Self::Enum(id, _) => Type::Enum(id.enumeration),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operand {
    Place(Place),
    Constant(Constant),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Rvalue {
    Enum(nova_types::VariantId, Vec<Operand>),
    EnumPayload(Operand, nova_types::VariantId, usize),
    Use(Operand),
    Aggregate(StructId, Vec<Operand>),
    Project(Operand, FieldId),
    /// Whole-root reconstruction through a checked mutable field path.
    Update(Operand, Vec<FieldId>, Operand),
    /// P07 whole-range lossless sign/zero extension, never narrowing.
    Widen(Operand, Type),
    /// P09 whole-type lossless integer-to-float or float widening.
    NumericConvert(Operand, Type),
    /// P10 explicit checked conversion, distinct from lossless coercion.
    CheckedCast(Operand, Type),
    Unary(Symbol, Operand),
    /// Never contains && or ||; those are CFG branches.
    Binary(Symbol, Operand, Operand),
    /// Ordered, already evaluated components. Runtime formatting is deferred.
    Interpolate(Vec<Operand>),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StatementKind {
    Assign(Place, Rvalue),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub source: SourceInfo,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminatorKind {
    /// Intrinsic Result tag dispatch; payload reads occur only on their active edge.
    Try {
        snapshot: Place,
        success: BlockId,
        error: BlockId,
    },
    Match {
        scrutinee: Operand,
        arms: Vec<(nova_typecheck::MatchPattern, BlockId)>,
    },
    Goto(BlockId),
    Branch {
        condition: Operand,
        then_block: BlockId,
        else_block: BlockId,
    },
    Call {
        callee: CalleeId,
        arguments: Vec<Operand>,
        destination: Place,
        target: BlockId,
    },
    Return(Operand),
    /// Structural orphan join after two returning branches; never a panic operation.
    Unreachable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Terminator {
    pub kind: TerminatorKind,
    pub source: SourceInfo,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicBlockData {
    pub statements: Vec<Statement>,
    pub terminator: Option<Terminator>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Body {
    pub callee: CalleeId,
    pub source: SourceInfo,
    pub parameters: Vec<LocalId>,
    pub locals: Vec<Local>,
    pub blocks: Vec<BasicBlockData>,
    pub entry: BlockId,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct TryCertificate {
    callee: CalleeId,
    source: SourceInfo,
    keyword: Span,
    source_result: nova_types::EnumId,
    destination_result: nova_types::EnumId,
    dispatch: BlockId,
    snapshot: Statement,
    success: BlockId,
    success_read: Statement,
    error: BlockId,
    error_body: BasicBlockData,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct TryControlCertificate {
    source: SourceInfo,
    entry: BlockId,
    terminators: Vec<Option<Terminator>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct NamedCallCertificate {
    mapping: nova_typecheck::NamedCall,
    source: SourceInfo,
    snapshots: Vec<(BlockId, usize, Statement)>,
    defaults: Vec<(nova_typecheck::ParameterDefault, BlockId, usize, Statement)>,
    block: BlockId,
    call: Terminator,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Module {
    entry: SourceInfo,
    pub structs: StructRegistry,
    pub enums: nova_types::EnumRegistry,
    enums_original: nova_types::EnumRegistry,
    pub sums: nova_types::SumRegistry,
    sums_original: nova_types::SumRegistry,
    sum_origins: std::collections::BTreeMap<nova_types::EnumId, SourceInfo>,
    variant_provenance: std::collections::BTreeMap<usize, nova_types::VariantId>,
    binder_provenance: std::collections::BTreeMap<usize, (nova_types::VariantId, usize)>,
    try_certificates: std::collections::BTreeMap<(usize, usize), TryCertificate>,
    try_controls: std::collections::BTreeMap<usize, TryControlCertificate>,
    named_calls: std::collections::BTreeMap<(usize, usize), NamedCallCertificate>,
    default_sources: std::collections::BTreeMap<(usize, usize), SourceInfo>,
    /// P17 freezes evaluation CFG and writes until a transform can preserve the proof.
    named_bodies: std::collections::BTreeMap<usize, Body>,
    /// P19 retains a private full-body proof of bound evaluation and loop CFG.
    loop_bodies: std::collections::BTreeMap<usize, Body>,
    /// P20 freezes predicate snapshots, tag direction, effects and enclosing CFG.
    exists_bodies: std::collections::BTreeMap<usize, Body>,
    match_provenance: std::collections::BTreeMap<usize, (Type, Vec<nova_typecheck::MatchPattern>)>,
    structs_original: StructRegistry,
    tuple_ids: std::collections::BTreeSet<StructId>,
    callee_provenance: Vec<Callee>,
    projection_provenance: std::collections::BTreeMap<usize, FieldId>,
    constructor_provenance: std::collections::BTreeMap<usize, StructId>,
    mutation_provenance: std::collections::BTreeMap<usize, (DefId, Vec<FieldId>)>,
    mutable_definitions: std::collections::BTreeSet<usize>,
    method_owners: std::collections::BTreeMap<usize, StructId>,
    method_bodies: std::collections::BTreeMap<usize, Body>,
    entry_main: Option<(Callee, SourceInfo)>,
    pub callees: Vec<Callee>,
    pub bodies: Vec<Body>,
    /// An immutable provenance table retained for independent MIR validation.
    pub sources: Vec<SourceInfo>,
}
impl Module {
    pub fn aggregate_type(&self, id: StructId) -> Type {
        if self.enums_original.contains_key(&nova_types::EnumId(id.0)) {
            Type::Enum(nova_types::EnumId(id.0))
        } else if self.tuple_ids.contains(&id) {
            Type::Tuple(id)
        } else {
            Type::Struct(id)
        }
    }
    pub fn entry_definition(&self) -> Option<DefId> {
        self.entry_main
            .as_ref()
            .map(|(callee, _)| callee.definition)
    }
    pub fn entry_source(&self) -> SourceInfo {
        self.entry
    }
    /// Deterministic, versioned debug representation, not a serialized ABI.
    pub fn dump(&self) -> String {
        let mut text = String::from("nova-mir v1 (abstract-runtime)\n");
        for (id, callee) in self.callees.iter().enumerate() {
            let _ = writeln!(text, "callee {id} {callee:?}");
        }
        for body in &self.bodies {
            let _ = writeln!(
                text,
                "body {} entry {} {:?}",
                body.callee.0, body.entry.0, body.source
            );
            for (id, local) in body.locals.iter().enumerate() {
                let _ = writeln!(text, "  local {id} {local:?}");
            }
            let _ = writeln!(text, "  parameters {:?}", body.parameters);
            for (id, block) in body.blocks.iter().enumerate() {
                let _ = writeln!(text, "  bb{id}:");
                for statement in &block.statements {
                    let _ = writeln!(text, "    {:?} @ {:?}", statement.kind, statement.source);
                }
                let _ = writeln!(text, "    {:?}", block.terminator);
            }
        }
        text
    }
}

impl Drop for Constant {
    fn drop(&mut self) {
        let mut pending = match self {
            Self::Struct(_, fields) | Self::Tuple(_, fields) | Self::Enum(_, fields) => {
                std::mem::take(fields)
            }
            _ => return,
        };
        while let Some(mut value) = pending.pop() {
            if let Self::Struct(_, fields) | Self::Tuple(_, fields) | Self::Enum(_, fields) =
                &mut value
            {
                pending.append(fields);
            }
        }
    }
}
