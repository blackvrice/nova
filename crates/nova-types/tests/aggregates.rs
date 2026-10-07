use nova_types::{
    struct_layout, ConstValue, StructField, StructId, StructLayout, StructRegistry, StructShape,
    Type,
};
#[test]
fn malformed_child_layout_is_rejected_without_arithmetic_panics() {
    for (align, depth, size) in [
        (0, 1, 0),
        (3, 1, 0),
        (8, usize::MAX, 0),
        (8, 1, usize::MAX),
        (8, 1, 3),
    ] {
        let mut shapes = StructRegistry::new();
        shapes.insert(
            StructId(1),
            StructShape {
                name: "bad".into(),
                fields: vec![],
                layout: Some(StructLayout {
                    size,
                    align,
                    depth,
                    occurrences: 0,
                    offsets: vec![],
                }),
            },
        );
        assert_eq!(
            struct_layout(
                &[StructField {
                    name: "f".into(),
                    ty: Type::Struct(StructId(1)),
                    mutable: false
                }],
                &shapes
            ),
            Err(0)
        );
    }
}
#[test]
fn deeply_malformed_const_payload_drop_uses_no_recursive_host_stack() {
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(|| {
            let mut value = ConstValue::Unit;
            for _ in 0..30_000 {
                value = ConstValue::Struct(StructId(1), vec![value]);
            }
            drop(value);
        })
        .unwrap()
        .join()
        .unwrap();
}
