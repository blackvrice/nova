use nova_types::*;

fn kind(width: &str) -> FloatKind {
    if width == "32" {
        FloatKind::F32
    } else {
        FloatKind::F64
    }
}
fn bits(text: &str) -> u64 {
    u64::from_str_radix(text, 16).unwrap()
}

#[test]
fn rational_oracle_literal_rounding_avoids_double_rounding() {
    for row in include_str!("../../../tools/tests/fixtures/float-literals.tsv").lines() {
        let row: Vec<_> = row.split('\t').collect();
        let value = FloatValue::parse_decimal(kind(row[0]), row[1]).unwrap();
        assert_eq!(value.bits(), bits(row[2]), "{row:?}");
    }
    assert_eq!(
        FloatValue::parse_decimal(FloatKind::F32, "3.4028236e38"),
        Err(FloatError::LiteralOverflow)
    );
    assert_eq!(
        FloatValue::parse_decimal(FloatKind::F64, "1.8e308"),
        Err(FloatError::LiteralOverflow)
    );
    assert_eq!(
        FloatValue::parse_decimal(FloatKind::F32, "1e-1000")
            .unwrap()
            .bits(),
        0
    );
    assert_eq!(
        FloatValue::parse_decimal(FloatKind::F64, "1_000.25")
            .unwrap()
            .bits(),
        1000.25f64.to_bits()
    );
}

#[test]
fn rational_oracle_arithmetic_matches_ieee_rounding() {
    for row in include_str!("../../../tools/tests/fixtures/float-operations.tsv").lines() {
        let row: Vec<_> = row.split('\t').collect();
        let kind = kind(row[0]);
        let a = FloatValue::from_bits(kind, bits(row[1])).unwrap();
        let b = FloatValue::from_bits(kind, bits(row[3])).unwrap();
        let op = match row[2] {
            "+" => FloatOp::Add,
            "-" => FloatOp::Subtract,
            "*" => FloatOp::Multiply,
            "/" => FloatOp::Divide,
            _ => unreachable!(),
        };
        assert_eq!(a.arithmetic(op, b).unwrap().bits(), bits(row[4]), "{row:?}");
    }
}

#[test]
fn canonical_nan_signed_zero_and_source_comparisons() {
    for kind in [FloatKind::F32, FloatKind::F64] {
        let zero = FloatValue::from_bits(kind, 0).unwrap();
        let negative = zero.negated();
        assert_ne!(zero, negative);
        assert_eq!(zero.compare(FloatComparison::Equal, negative), Some(true));
        let nan = zero.arithmetic(FloatOp::Divide, zero).unwrap();
        assert_eq!(nan.bits(), kind.nan_bits());
        assert_eq!(nan.negated(), nan);
        assert_eq!(
            FloatValue::from_bits(kind, kind.nan_bits() | 1).unwrap(),
            nan
        );
        for op in [
            FloatComparison::Equal,
            FloatComparison::Less,
            FloatComparison::LessEqual,
            FloatComparison::Greater,
            FloatComparison::GreaterEqual,
        ] {
            assert_eq!(nan.compare(op, nan), Some(false));
        }
        assert_eq!(nan.compare(FloatComparison::NotEqual, nan), Some(true));
    }
    assert!(FloatValue::from_bits(FloatKind::F32, 1 << 32).is_none());
    assert_eq!(
        FloatValue::from_bits(FloatKind::F32, 0xffc12345)
            .unwrap()
            .widen(FloatKind::F64)
            .unwrap()
            .bits(),
        FloatKind::F64.nan_bits()
    );
}

#[test]
fn all_whole_type_conversions_and_common_types() {
    for source in [
        IntKind::I8,
        IntKind::U8,
        IntKind::I16,
        IntKind::U16,
        IntKind::I32,
        IntKind::U32,
        IntKind::I64,
        IntKind::U64,
    ] {
        for dest in [FloatKind::F32, FloatKind::F64] {
            let allowed = source.bits() - u32::from(source.signed()) <= dest.precision();
            assert_eq!(source.ty().widens_to(dest.ty()), allowed);
            for value in [source.min(), 0, source.max()] {
                assert_eq!(
                    FloatValue::from_integer(IntegerValue::new(source, value).unwrap(), dest)
                        .is_some(),
                    allowed
                );
            }
            let common = if FloatKind::F32.accepts_integer(source) && dest == FloatKind::F32 {
                Some(Type::Float32)
            } else if FloatKind::F64.accepts_integer(source) {
                Some(Type::Float64)
            } else {
                None
            };
            assert_eq!(source.ty().common_numeric(dest.ty()), common);
            assert_eq!(dest.ty().common_numeric(source.ty()), common);
        }
    }
    assert_eq!(Type::Int64.common_numeric(Type::UInt64), None);
    assert!(!Type::Float64.widens_to(Type::Float32));
    assert!(!Type::Float32.widens_to(Type::Int64));
}

#[test]
#[cfg(target_arch = "x86_64")]
fn host_rounding_ftz_daz_are_controlled_and_restored_on_all_paths() {
    unsafe fn read() -> u32 {
        let mut v = 0;
        unsafe {
            std::arch::asm!("stmxcsr [{p}]",p=in(reg)&mut v,options(nostack));
        }
        v
    }
    unsafe fn write(v: u32) {
        unsafe {
            std::arch::asm!("ldmxcsr [{p}]",p=in(reg)&v,options(nostack));
        }
    }
    struct Restore(u32);
    impl Drop for Restore {
        fn drop(&mut self) {
            unsafe {
                write(self.0);
            }
        }
    }
    let _restore = Restore(unsafe { read() });
    let bad = 0xffc0; // Toward zero, FTZ/DAZ, exceptions masked.
    unsafe {
        write(bad);
    }
    let one = FloatValue::from_bits(FloatKind::F32, 0x3f800000).unwrap();
    let ulp = FloatValue::from_bits(FloatKind::F32, 0x34000000).unwrap();
    assert_eq!(
        one.arithmetic(FloatOp::Add, ulp).unwrap().bits(),
        0x3f800001
    );
    assert_eq!(unsafe { read() }, bad);
    let sub = FloatValue::from_bits(FloatKind::F32, 1).unwrap();
    assert_eq!(sub.arithmetic(FloatOp::Multiply, one).unwrap(), sub);
    assert_eq!(unsafe { read() }, bad);
    assert_eq!(
        FloatValue::parse_decimal(FloatKind::F32, "1.00000011920928955078125")
            .unwrap()
            .bits(),
        0x3f800001
    );
    assert_eq!(unsafe { read() }, bad);
    assert_eq!(
        FloatValue::parse_decimal(FloatKind::F32, "bad"),
        Err(FloatError::InvalidDecimal)
    );
    assert_eq!(unsafe { read() }, bad);
    assert_eq!(
        FloatValue::parse_decimal(FloatKind::F32, "1e999"),
        Err(FloatError::LiteralOverflow)
    );
    assert_eq!(unsafe { read() }, bad);
    assert_eq!(
        sub.widen(FloatKind::F64).unwrap().bits(),
        0x36a0000000000000
    );
    assert_eq!(unsafe { read() }, bad);
}
