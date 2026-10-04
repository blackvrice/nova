//! P02 Stage A semantic types. No LLVM or target host type dependency.
mod integer;
pub use integer::{IntKind, IntegerOp, IntegerValue};
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Type {
    Error,
    Unit,
    Int32,
    Int8,
    Int16,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Bool,
    Char,
    String,
    Function,
}
impl Type {
    pub const fn integer(self) -> Option<IntKind> {
        match self {
            Self::Int8 => Some(IntKind::I8),
            Self::Int16 => Some(IntKind::I16),
            Self::Int32 => Some(IntKind::I32),
            Self::Int64 => Some(IntKind::I64),
            Self::UInt8 => Some(IntKind::U8),
            Self::UInt16 => Some(IntKind::U16),
            Self::UInt32 => Some(IntKind::U32),
            Self::UInt64 => Some(IntKind::U64),
            _ => None,
        }
    }
}

/// P05 target-independent fixed-width values; no runtime or LLVM representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstValue {
    Int32(i32),
    Integer(IntegerValue),
    Bool(bool),
    Char(char),
    String(String),
    Unit,
}
impl ConstValue {
    pub const fn ty(&self) -> Type {
        match self {
            Self::Int32(_) => Type::Int32,
            Self::Integer(value) => value.kind().ty(),
            Self::Bool(_) => Type::Bool,
            Self::Char(_) => Type::Char,
            Self::String(_) => Type::String,
            Self::Unit => Type::Unit,
        }
    }
    pub fn from_integer(value: IntegerValue) -> Self {
        if value.kind() == IntKind::I32 {
            Self::Int32(value.value() as i32)
        } else {
            Self::Integer(value)
        }
    }
    pub fn integer(&self) -> Option<IntegerValue> {
        match self {
            Self::Int32(value) => IntegerValue::new(IntKind::I32, *value as i128),
            Self::Integer(value) => Some(*value),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(usize);

#[derive(Debug, Default, Eq, PartialEq)]
pub struct TypeInterner {
    types: Vec<Type>,
}
impl TypeInterner {
    pub fn intern(&mut self, ty: Type) -> TypeId {
        if let Some(index) = self.types.iter().position(|existing| *existing == ty) {
            return TypeId(index);
        }
        let id = TypeId(self.types.len());
        self.types.push(ty);
        id
    }
    pub fn get(&self, id: TypeId) -> Option<Type> {
        self.types.get(id.0).copied()
    }
    /// ErrorType suppresses errors originating in an earlier check.
    pub fn compatible(&self, actual: TypeId, expected: TypeId) -> bool {
        match (self.get(actual), self.get(expected)) {
            (Some(Type::Error), Some(_)) | (Some(_), Some(Type::Error)) => true,
            (Some(left), Some(right)) => left == right,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable_interning_and_error_compatibility() {
        let mut types = TypeInterner::default();
        let int = types.intern(Type::Int32);
        let string = types.intern(Type::String);
        let error = types.intern(Type::Error);
        for _ in 0..100 {
            assert_eq!(types.intern(Type::Int32), int);
        }
        assert!(!types.compatible(int, string));
        assert!(types.compatible(error, string));
        assert!(types.compatible(int, error));
        assert_eq!(types.get(TypeId(100)), None);
        assert!(!types.compatible(TypeId(100), error));
    }
}
