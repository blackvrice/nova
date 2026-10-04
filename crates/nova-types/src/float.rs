//! P09 binary values and controlled host evaluation. No LLVM dependency.
use crate::{IntKind, IntegerValue, Type};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FloatKind {
    F32,
    F64,
}
impl FloatKind {
    pub const fn ty(self) -> Type {
        match self {
            Self::F32 => Type::Float32,
            Self::F64 => Type::Float64,
        }
    }
    pub const fn bits(self) -> u32 {
        match self {
            Self::F32 => 32,
            Self::F64 => 64,
        }
    }
    pub const fn precision(self) -> u32 {
        match self {
            Self::F32 => 24,
            Self::F64 => 53,
        }
    }
    pub const fn accepts_integer(self, source: IntKind) -> bool {
        source.bits() - if source.signed() { 1 } else { 0 } <= self.precision()
    }
    pub const fn nan_bits(self) -> u64 {
        match self {
            Self::F32 => 0x7fc00000,
            Self::F64 => 0x7ff8000000000000,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FloatOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FloatComparison {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FloatError {
    InvalidDecimal,
    LiteralOverflow,
    UnsupportedHost,
}

/// Metadata equality is bit equality, not source numeric equality.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FloatValue {
    kind: FloatKind,
    bits: u64,
}
impl FloatValue {
    pub fn from_bits(kind: FloatKind, bits: u64) -> Option<Self> {
        if kind == FloatKind::F32 && bits > u32::MAX as u64 {
            return None;
        }
        let nan = match kind {
            FloatKind::F32 => bits & 0x7f800000 == 0x7f800000 && bits & 0x007fffff != 0,
            FloatKind::F64 => {
                bits & 0x7ff0000000000000 == 0x7ff0000000000000 && bits & 0x000fffffffffffff != 0
            }
        };
        Some(Self {
            kind,
            bits: if nan { kind.nan_bits() } else { bits },
        })
    }
    pub fn from_f32(value: f32) -> Self {
        Self::from_bits(FloatKind::F32, value.to_bits() as u64).expect("binary32 bits")
    }
    pub fn from_f64(value: f64) -> Self {
        Self::from_bits(FloatKind::F64, value.to_bits()).expect("binary64 bits")
    }
    pub const fn kind(self) -> FloatKind {
        self.kind
    }
    pub const fn bits(self) -> u64 {
        self.bits
    }
    pub fn is_nan(self) -> bool {
        self.bits == self.kind.nan_bits()
    }
    pub fn parse_decimal(kind: FloatKind, text: &str) -> Result<Self, FloatError> {
        let _environment = Environment::enter().ok_or(FloatError::UnsupportedHost)?;
        let text = text.replace('_', "");
        let result = match kind {
            FloatKind::F32 => Self::from_f32(
                text.parse::<f32>()
                    .map_err(|_| FloatError::InvalidDecimal)?,
            ),
            FloatKind::F64 => Self::from_f64(
                text.parse::<f64>()
                    .map_err(|_| FloatError::InvalidDecimal)?,
            ),
        };
        let finite = match kind {
            FloatKind::F32 => f32::from_bits(result.bits as u32).is_finite(),
            FloatKind::F64 => f64::from_bits(result.bits).is_finite(),
        };
        if finite {
            Ok(result)
        } else {
            Err(FloatError::LiteralOverflow)
        }
    }
    pub fn from_integer(value: IntegerValue, dest: FloatKind) -> Option<Self> {
        if !dest.accepts_integer(value.kind()) {
            return None;
        }
        let _environment = Environment::enter()?;
        let value = std::hint::black_box(value.value());
        Some(std::hint::black_box(match dest {
            FloatKind::F32 => Self::from_f32(value as f32),
            FloatKind::F64 => Self::from_f64(value as f64),
        }))
    }
    pub fn widen(self, dest: FloatKind) -> Option<Self> {
        if self.kind == dest {
            return Some(self);
        }
        if self.kind != FloatKind::F32 || dest != FloatKind::F64 {
            return None;
        }
        let _environment = Environment::enter()?;
        let value = std::hint::black_box(f32::from_bits(self.bits as u32));
        Some(std::hint::black_box(Self::from_f64(value as f64)))
    }
    pub fn negated(self) -> Self {
        Self::from_bits(self.kind, self.bits ^ (1u64 << (self.kind.bits() - 1)))
            .expect("same binary width")
    }
    pub fn arithmetic(self, op: FloatOp, other: Self) -> Option<Self> {
        if self.kind != other.kind {
            return None;
        }
        let _environment = Environment::enter()?;
        macro_rules! evaluate {
            ($ty:ty, $from:ident) => {{
                let a = std::hint::black_box(<$ty>::from_bits(self.bits as _));
                let b = std::hint::black_box(<$ty>::from_bits(other.bits as _));
                Self::$from(match op {
                    FloatOp::Add => a + b,
                    FloatOp::Subtract => a - b,
                    FloatOp::Multiply => a * b,
                    FloatOp::Divide => a / b,
                })
            }};
        }
        Some(std::hint::black_box(match self.kind {
            FloatKind::F32 => evaluate!(f32, from_f32),
            FloatKind::F64 => evaluate!(f64, from_f64),
        }))
    }
    pub fn compare(self, op: FloatComparison, other: Self) -> Option<bool> {
        if self.kind != other.kind {
            return None;
        }
        let _environment = Environment::enter()?;
        macro_rules! compare {
            ($ty:ty) => {{
                let a = std::hint::black_box(<$ty>::from_bits(self.bits as _));
                let b = std::hint::black_box(<$ty>::from_bits(other.bits as _));
                match op {
                    FloatComparison::Equal => a == b,
                    FloatComparison::NotEqual => a != b,
                    FloatComparison::Less => a < b,
                    FloatComparison::LessEqual => a <= b,
                    FloatComparison::Greater => a > b,
                    FloatComparison::GreaterEqual => a >= b,
                }
            }};
        }
        Some(std::hint::black_box(match self.kind {
            FloatKind::F32 => compare!(f32),
            FloatKind::F64 => compare!(f64),
        }))
    }
}

pub fn float_host_supported() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        true
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}
// Each operation establishes RN, gradual underflow and masked exceptions,
// then restores the caller's thread-local control/status exactly.
struct Environment {
    previous: u32,
}
impl Environment {
    fn enter() -> Option<Self> {
        if !float_host_supported() {
            return None;
        }
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            let mut previous = 0u32;
            unsafe {
                std::arch::asm!("stmxcsr [{pointer}]", pointer = in(reg) &mut previous, options(nostack));
            }
            let current = (previous & !0xe07f) | 0x1f80;
            unsafe {
                std::arch::asm!("ldmxcsr [{pointer}]", pointer = in(reg) &current, options(nostack));
            }
            Some(Self { previous })
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        {
            None
        }
    }
}
impl Drop for Environment {
    fn drop(&mut self) {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        unsafe {
            std::arch::asm!("ldmxcsr [{pointer}]", pointer = in(reg) &self.previous, options(nostack));
        }
    }
}
