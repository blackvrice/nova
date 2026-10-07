//! P02 Stage A semantic types. No LLVM or target host type dependency.
mod aggregate;
pub use aggregate::{
    layout as struct_layout, FieldId, StructField, StructId, StructLayout, StructRegistry,
    StructShape,
};
mod enumeration;
pub use enumeration::{enum_layout, EnumId, EnumRegistry, EnumShape, EnumVariant, VariantId};
mod float;
mod integer;
pub use float::{
    float_host_supported, FloatComparison, FloatError, FloatKind, FloatOp, FloatValue,
};
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
    Float32,
    Float64,
    Bool,
    Char,
    String,
    Function,
    Struct(StructId),
    Tuple(StructId),
    Enum(EnumId),
}
impl Type {
    pub const fn aggregate(self) -> Option<StructId> {
        match self {
            Self::Struct(id) | Self::Tuple(id) => Some(id),
            Self::Enum(id) => Some(StructId(id.0)),
            _ => None,
        }
    }
    pub const fn float(self) -> Option<FloatKind> {
        match self {
            Self::Float32 => Some(FloatKind::F32),
            Self::Float64 => Some(FloatKind::F64),
            _ => None,
        }
    }
    pub const fn numeric(self) -> bool {
        self.integer().is_some() || self.float().is_some()
    }
    pub fn widens_to(self, dest: Self) -> bool {
        if let (Some(source), Some(dest)) = (self.integer(), dest.integer()) {
            return source.widens_to(dest);
        }
        if let Some(dest) = dest.float() {
            return self == dest.ty()
                || self == Type::Float32 && dest == FloatKind::F64
                || self
                    .integer()
                    .is_some_and(|source| dest.accepts_integer(source));
        }
        false
    }
    pub fn common_numeric(self, other: Self) -> Option<Self> {
        if let (Some(a), Some(b)) = (self.integer(), other.integer()) {
            return a.common(b).map(IntKind::ty);
        }
        if self.float().is_some() || other.float().is_some() {
            return [Self::Float32, Self::Float64]
                .into_iter()
                .find(|&dest| self.widens_to(dest) && other.widens_to(dest));
        }
        None
    }
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
    Float(FloatValue),
    Bool(bool),
    Char(char),
    String(String),
    Unit,
    Struct(StructId, Vec<ConstValue>),
    Tuple(StructId, Vec<ConstValue>),
    Enum(VariantId, Vec<ConstValue>),
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CastError {
    InvalidTypes,
    OutOfRange,
    UnsupportedHost,
}
impl ConstValue {
    /// P10 explicit numeric conversion; never wrapping or saturating.
    pub fn checked_cast(&self, dest: Type) -> Result<Self, CastError> {
        if !self.ty().numeric() || !dest.numeric() {
            return Err(CastError::InvalidTypes);
        }
        if let Some(value) = self.integer() {
            if let Some(kind) = dest.integer() {
                return IntegerValue::new(kind, value.value())
                    .map(Self::from_integer)
                    .ok_or(CastError::OutOfRange);
            }
            return FloatValue::cast_integer(value, dest.float().expect("numeric target"))
                .map(Self::Float);
        }
        let Self::Float(value) = self else {
            unreachable!("numeric value")
        };
        if let Some(kind) = dest.integer() {
            value.truncated_integer(kind).map(Self::from_integer)
        } else {
            value
                .cast_float(dest.float().expect("numeric target"))
                .map(Self::Float)
        }
    }

    pub const fn ty(&self) -> Type {
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
    pub fn widen(&self, dest: Type) -> Option<Self> {
        if let Some(dest) = dest.float() {
            return match self {
                Self::Float(value) => value.widen(dest).map(Self::Float),
                _ => self
                    .integer()
                    .and_then(|value| FloatValue::from_integer(value, dest))
                    .map(Self::Float),
            };
        }
        self.integer()
            .and_then(|value| dest.integer().and_then(|dest| value.widen(dest)))
            .map(Self::from_integer)
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

impl Drop for ConstValue {
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
