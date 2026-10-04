use nova_types::{ConstValue, Type, TypeInterner};

#[test]
fn all_scalar_values_have_distinct_char_type_and_exact_utf8() {
    let mut types = TypeInterner::default();
    let char_type = types.intern(Type::Char);
    for other in [Type::Int32, Type::Bool, Type::String, Type::Unit] {
        let other = types.intern(other);
        assert!(!types.compatible(char_type, other));
    }
    assert_eq!(Type::Char.integer(), None);
    for value in 0..=0x110000u32 {
        let allowed = value <= 0x10ffff && !(0xd800..=0xdfff).contains(&value);
        let scalar = char::from_u32(value);
        assert_eq!(scalar.is_some(), allowed);
        if let Some(scalar) = scalar {
            assert_eq!(ConstValue::Char(scalar).ty(), Type::Char);
            assert_eq!(ConstValue::Char(scalar).integer(), None);
            let expected = match value {
                0..=0x7f => vec![value as u8],
                0x80..=0x7ff => vec![(0xc0 | (value >> 6)) as u8, (0x80 | (value & 63)) as u8],
                0x800..=0xffff => vec![
                    (0xe0 | (value >> 12)) as u8,
                    (0x80 | ((value >> 6) & 63)) as u8,
                    (0x80 | (value & 63)) as u8,
                ],
                _ => vec![
                    (0xf0 | (value >> 18)) as u8,
                    (0x80 | ((value >> 12) & 63)) as u8,
                    (0x80 | ((value >> 6) & 63)) as u8,
                    (0x80 | (value & 63)) as u8,
                ],
            };
            assert_eq!(scalar.encode_utf8(&mut [0; 4]).as_bytes(), expected);
        }
    }
    assert_eq!(char::from_u32(u32::MAX), None);
}
