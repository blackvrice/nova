use nova_types::{IntKind, IntegerOp, IntegerValue};

#[test]
fn all_ranges_conversions_and_common_types_match_independent_oracle() {
    let ranges = [
        (-128, 127),
        (0, 255),
        (-32768, 32767),
        (0, 65535),
        (-2147483648, 2147483647),
        (0, 4294967295),
        (-9223372036854775808, 9223372036854775807),
        (0, 18446744073709551615),
    ];
    for (i, &source) in IntKind::ALL.iter().enumerate() {
        assert_eq!((source.min(), source.max()), ranges[i]);
        for (j, &dest) in IntKind::ALL.iter().enumerate() {
            let expected = ranges[i].0 >= ranges[j].0 && ranges[i].1 <= ranges[j].1;
            assert_eq!(source.widens_to(dest), expected, "{source:?} -> {dest:?}");
            let candidate = [1, 0, 3, 2, 5, 4, 7, 6]
                .into_iter()
                .find(|&k| {
                    ranges[k].0 <= ranges[i].0.min(ranges[j].0)
                        && ranges[k].1 >= ranges[i].1.max(ranges[j].1)
                })
                .map(|k| IntKind::ALL[k]);
            assert_eq!(source.common(dest), candidate);
        }
    }
}

#[test]
fn checked_values_preserve_all_width_boundaries_and_division_contract() {
    for kind in IntKind::ALL {
        let value = |n| IntegerValue::new(kind, n).unwrap();
        assert!(IntegerValue::new(kind, kind.min() - 1).is_none());
        assert!(IntegerValue::new(kind, kind.max() + 1).is_none());
        assert!(value(kind.max())
            .arithmetic(IntegerOp::Add, value(1))
            .is_none());
        assert!(value(kind.max())
            .arithmetic(IntegerOp::Multiply, value(2))
            .is_none());
        assert!(value(kind.min())
            .arithmetic(IntegerOp::Subtract, value(1))
            .is_none());
        assert!(value(1).arithmetic(IntegerOp::Divide, value(0)).is_none());
        assert!(value(1)
            .arithmetic(IntegerOp::Remainder, value(0))
            .is_none());
        assert_eq!(
            value(7).arithmetic(IntegerOp::Divide, value(3)),
            Some(value(2))
        );
        assert_eq!(
            value(7).arithmetic(IntegerOp::Remainder, value(3)),
            Some(value(1))
        );
        if kind.signed() {
            assert!(value(kind.min()).negated().is_none());
            for op in [IntegerOp::Divide, IntegerOp::Remainder] {
                assert!(value(kind.min()).arithmetic(op, value(-1)).is_none());
            }
            assert_eq!(
                value(-7).arithmetic(IntegerOp::Divide, value(3)),
                Some(value(-2))
            );
            assert_eq!(
                value(-7).arithmetic(IntegerOp::Remainder, value(3)),
                Some(value(-1))
            );
        } else {
            assert!(value(0).negated().is_none());
        }
    }
}
