//! P07 fixed-width semantics, independent of host and backend representation.
use crate::Type;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum IntKind {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
}
impl IntKind {
    pub const ALL: [Self; 8] = [
        Self::I8,
        Self::U8,
        Self::I16,
        Self::U16,
        Self::I32,
        Self::U32,
        Self::I64,
        Self::U64,
    ];
    pub const fn bits(self) -> u32 {
        match self {
            Self::I8 | Self::U8 => 8,
            Self::I16 | Self::U16 => 16,
            Self::I32 | Self::U32 => 32,
            Self::I64 | Self::U64 => 64,
        }
    }
    pub const fn signed(self) -> bool {
        matches!(self, Self::I8 | Self::I16 | Self::I32 | Self::I64)
    }
    pub const fn min(self) -> i128 {
        if self.signed() {
            -(1i128 << (self.bits() - 1))
        } else {
            0
        }
    }
    pub const fn max(self) -> i128 {
        (1i128 << (self.bits() - if self.signed() { 1 } else { 0 })) - 1
    }
    pub const fn ty(self) -> Type {
        match self {
            Self::I8 => Type::Int8,
            Self::U8 => Type::UInt8,
            Self::I16 => Type::Int16,
            Self::U16 => Type::UInt16,
            Self::I32 => Type::Int32,
            Self::U32 => Type::UInt32,
            Self::I64 => Type::Int64,
            Self::U64 => Type::UInt64,
        }
    }
    pub const fn widens_to(self, dest: Self) -> bool {
        self.min() >= dest.min() && self.max() <= dest.max()
    }
    pub fn common(self, other: Self) -> Option<Self> {
        // Smallest width first; unsigned wins ties because its range is smaller.
        [
            Self::U8,
            Self::I8,
            Self::U16,
            Self::I16,
            Self::U32,
            Self::I32,
            Self::U64,
            Self::I64,
        ]
        .into_iter()
        .find(|k| self.widens_to(*k) && other.widens_to(*k))
    }
}

/// Private fields guarantee a value is inside its declared fixed-width range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegerValue {
    kind: IntKind,
    value: i128,
}
impl IntegerValue {
    pub const fn new(kind: IntKind, value: i128) -> Option<Self> {
        if value >= kind.min() && value <= kind.max() {
            Some(Self { kind, value })
        } else {
            None
        }
    }
    pub const fn kind(self) -> IntKind {
        self.kind
    }
    pub const fn value(self) -> i128 {
        self.value
    }
    pub fn widen(self, dest: IntKind) -> Option<Self> {
        if self.kind.widens_to(dest) {
            Self::new(dest, self.value)
        } else {
            None
        }
    }
    pub fn negated(self) -> Option<Self> {
        if self.kind.signed() {
            Self::new(self.kind, -self.value)
        } else {
            None
        }
    }
    pub fn arithmetic(self, op: IntegerOp, rhs: Self) -> Option<Self> {
        if self.kind != rhs.kind {
            return None;
        }
        if matches!(op, IntegerOp::Divide | IntegerOp::Remainder)
            && (rhs.value == 0
                || (self.kind.signed() && self.value == self.kind.min() && rhs.value == -1))
        {
            return None;
        }
        let value = match op {
            IntegerOp::Add => self.value.checked_add(rhs.value),
            IntegerOp::Subtract => self.value.checked_sub(rhs.value),
            IntegerOp::Multiply => self.value.checked_mul(rhs.value),
            IntegerOp::Divide => self.value.checked_div(rhs.value),
            IntegerOp::Remainder => self.value.checked_rem(rhs.value),
        }?;
        Self::new(self.kind, value)
    }
}
#[derive(Clone, Copy)]
pub enum IntegerOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}
