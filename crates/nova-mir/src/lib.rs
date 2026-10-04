//! Stage A non-SSA MIR. Arithmetic and formatting remain abstract Nova operations.
//! This IR is not permission to select a runtime overflow, print, or ABI policy.
mod lower;
mod validate;

pub use lower::{lower, LoweringError};
use nova_hir::{HirId, SourceOrigin};
use nova_resolve::DefId;
use nova_source::Span;
use nova_syntax::Symbol;
use nova_types::{IntKind, IntegerValue, Type};
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
    Bool(bool),
    Char(char),
    String(String),
    Unit,
}
impl Constant {
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
            Self::Bool(_) => Type::Bool,
            Self::Char(_) => Type::Char,
            Self::String(_) => Type::String,
            Self::Unit => Type::Unit,
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
    Use(Operand),
    /// P07 whole-range lossless sign/zero extension, never narrowing.
    Widen(Operand, Type),
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
pub struct Module {
    pub callees: Vec<Callee>,
    pub bodies: Vec<Body>,
    /// An immutable provenance table retained for independent MIR validation.
    pub sources: Vec<SourceInfo>,
}
impl Module {
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
