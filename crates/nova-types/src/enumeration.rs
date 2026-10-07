//! P14 finite nominal tagged values, independent of LLVM.
use crate::{struct_layout, StructField, StructLayout, StructRegistry};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EnumId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VariantId {
    pub enumeration: EnumId,
    pub index: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Vec<StructField>,
    pub layout: Option<StructLayout>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumShape {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}
pub type EnumRegistry = BTreeMap<EnumId, EnumShape>;

/// Returns the flattened payload edge causing invalid layout or a limit.
pub fn enum_layout(shape: &mut EnumShape, types: &StructRegistry) -> Result<StructLayout, usize> {
    if shape.variants.is_empty() || shape.variants.len() > 1024 {
        return Err(0);
    }
    let mut align = 1usize;
    let mut size = 0usize;
    let mut depth = 1usize;
    let mut occurrences = 0usize;
    let mut edge = 0;
    for variant in &mut shape.variants {
        let layout = struct_layout(&variant.fields, types).map_err(|at| edge + at)?;
        occurrences = occurrences.checked_add(layout.occurrences).ok_or(edge)?;
        if occurrences > 65_536 {
            return Err(edge);
        }
        align = align.max(layout.align);
        size = size.max(layout.size);
        depth = depth.max(layout.depth);
        variant.layout = Some(layout);
        edge += variant.fields.len();
    }
    let offset = (4usize.checked_add(align - 1).ok_or(edge)? / align) * align;
    align = align.max(4);
    size = offset
        .checked_add(size)
        .and_then(|s| s.checked_add(align - 1))
        .ok_or(edge)?
        / align
        * align;
    if size > 1_048_576 {
        return Err(edge.saturating_sub(1));
    }
    Ok(StructLayout {
        size,
        align,
        depth,
        occurrences,
        offsets: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StructId, StructShape, Type};
    #[test]
    fn p14_layout_tag_padding_and_payload_size_limit_are_checked() {
        let sid = StructId(1);
        let mut types = StructRegistry::new();
        types.insert(
            sid,
            StructShape {
                name: "cached".into(),
                fields: vec![],
                layout: Some(StructLayout {
                    size: 1_048_568,
                    align: 8,
                    depth: 1,
                    occurrences: 0,
                    offsets: vec![],
                }),
            },
        );
        let mut shape = EnumShape {
            name: "E".into(),
            variants: vec![EnumVariant {
                name: "A".into(),
                fields: vec![StructField {
                    name: "0".into(),
                    ty: Type::Struct(sid),
                    mutable: false,
                }],
                layout: None,
            }],
        };
        let layout = enum_layout(&mut shape, &types).unwrap();
        assert_eq!(
            (layout.size, layout.align, layout.depth, layout.occurrences),
            (1_048_576, 8, 2, 1)
        );
        types.get_mut(&sid).unwrap().layout.as_mut().unwrap().size = 1_048_576;
        assert_eq!(enum_layout(&mut shape, &types), Err(0));
        types.get_mut(&sid).unwrap().layout.as_mut().unwrap().align = 0;
        assert_eq!(enum_layout(&mut shape, &types), Err(0));
        shape.variants.clear();
        assert_eq!(enum_layout(&mut shape, &types), Err(0));
    }
}
