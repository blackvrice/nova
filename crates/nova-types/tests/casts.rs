use nova_types::*;
fn ty(s: &str) -> Type {
    match s {
        "int8" => Type::Int8,
        "int16" => Type::Int16,
        "int32" => Type::Int32,
        "int64" => Type::Int64,
        "uint8" => Type::UInt8,
        "uint16" => Type::UInt16,
        "uint32" => Type::UInt32,
        "uint64" => Type::UInt64,
        "float32" => Type::Float32,
        "float64" => Type::Float64,
        _ => unreachable!(),
    }
}
#[test]
fn p10_exact_rational_oracle_all_numeric_pairs() {
    for row in include_str!("../../../tools/tests/fixtures/casts.tsv").lines() {
        let r: Vec<_> = row.split('\t').collect();
        let s = ty(r[0]);
        let d = ty(r[1]);
        let value = if let Some(kind) = s.integer() {
            ConstValue::from_integer(IntegerValue::new(kind, r[2].parse().unwrap()).unwrap())
        } else {
            ConstValue::Float(
                FloatValue::from_bits(s.float().unwrap(), u64::from_str_radix(r[2], 16).unwrap())
                    .unwrap(),
            )
        };
        let result = value.checked_cast(d);
        if r[3] == "error" {
            assert_eq!(result, Err(CastError::OutOfRange), "{r:?}");
        } else {
            let result = result.unwrap();
            assert_eq!(result.ty(), d, "{r:?}");
            if let ConstValue::Float(f) = result {
                assert_eq!(f.bits(), u64::from_str_radix(r[3], 16).unwrap(), "{r:?}");
            } else {
                assert_eq!(
                    result.integer().unwrap().value(),
                    r[3].parse::<i128>().unwrap(),
                    "{r:?}"
                );
            }
        }
    }
}
#[test]
fn p10_numeric_only_and_identity_contract() {
    for value in [
        ConstValue::Bool(true),
        ConstValue::Char('a'),
        ConstValue::String("1".into()),
        ConstValue::Unit,
    ] {
        assert_eq!(
            value.checked_cast(Type::Int32),
            Err(CastError::InvalidTypes)
        );
    }
    for dest in [
        Type::Bool,
        Type::Char,
        Type::String,
        Type::Unit,
        Type::Function,
        Type::Error,
    ] {
        assert_eq!(
            ConstValue::Int32(1).checked_cast(dest),
            Err(CastError::InvalidTypes)
        );
    }
}
#[cfg(target_arch = "x86_64")]
#[test]
fn p10_explicit_conversion_controls_and_restores_mxcsr() {
    unsafe fn read() -> u32 {
        let mut v = 0u32;
        unsafe {
            std::arch::asm!("stmxcsr [{p}]",p=in(reg)&mut v,options(nostack));
        }
        v
    }
    unsafe fn set(v: u32) {
        unsafe {
            std::arch::asm!("ldmxcsr [{p}]",p=in(reg)&v,options(nostack));
        }
    }
    struct Restore(u32);
    impl Drop for Restore {
        fn drop(&mut self) {
            unsafe { set(self.0) }
        }
    }
    let _restore = Restore(unsafe { read() });
    for bad in [0x1f80u32, 0xffc0, 0x7fc0, 0x9fc1] {
        unsafe { set(bad) };
        let v = IntegerValue::new(IntKind::I64, (1i128 << 62) + (1i128 << 38) + 1).unwrap();
        assert_eq!(
            FloatValue::cast_integer(v, FloatKind::F32).unwrap().bits(),
            0x5e800001
        );
        assert_eq!(unsafe { read() }, bad);
        let x = FloatValue::from_bits(FloatKind::F64, 0x36a0000000000000).unwrap();
        assert_eq!(x.cast_float(FloatKind::F32).unwrap().bits(), 1);
        assert_eq!(unsafe { read() }, bad);
        let x = FloatValue::from_bits(FloatKind::F64, 0x7fefffffffffffff).unwrap();
        assert_eq!(x.cast_float(FloatKind::F32), Err(CastError::OutOfRange));
        assert_eq!(unsafe { read() }, bad);
    }
}
