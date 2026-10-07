//! P12 nominal aggregate identities and checked target-independent x64 layout.
use crate::Type;
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct StructId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FieldId {
    pub structure: StructId,
    pub index: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructLayout {
    pub size: usize,
    pub align: usize,
    pub offsets: Vec<usize>,
    pub depth: usize,
    pub occurrences: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructShape {
    pub name: String,
    pub fields: Vec<StructField>,
    pub layout: Option<StructLayout>,
}
pub type StructRegistry = BTreeMap<StructId, StructShape>;
/// Acyclic dependency order is established by the checker. Returns the field
/// causing an invalid dependency or a resource limit; never unchecked arithmetic.
pub fn layout(fields: &[StructField], types: &StructRegistry) -> Result<StructLayout, usize> {
    if fields.len() > 1024 {
        return Err(1024);
    }
    let mut out = StructLayout {
        size: 0,
        align: 1,
        offsets: vec![],
        depth: 1,
        occurrences: 0,
    };
    for (index, field) in fields.iter().enumerate() {
        let (size, align, depth, occurrences) = match field.ty {
            Type::Struct(id) => {
                let l = types
                    .get(&id)
                    .and_then(|s| s.layout.as_ref())
                    .ok_or(index)?;
                if !matches!(l.align, 1 | 2 | 4 | 8)
                    || !(1..=128).contains(&l.depth)
                    || l.size > 1_048_576
                    || l.occurrences > 65_536
                    || l.size % l.align != 0
                {
                    return Err(index);
                }
                (l.size, l.align, l.depth, l.occurrences)
            }
            Type::Unit => (0, 1, 0, 0),
            Type::Bool => (1, 1, 0, 0),
            Type::Char | Type::Float32 => (4, 4, 0, 0),
            Type::Float64 => (8, 8, 0, 0),
            t if t.integer().is_some() => {
                let n = t.integer().ok_or(index)?.bits() as usize / 8;
                (n, n, 0, 0)
            }
            _ => return Err(index),
        };
        out.size = out.size.checked_add(align - 1).ok_or(index)? / align * align;
        out.offsets.push(out.size);
        out.size = out.size.checked_add(size).ok_or(index)?;
        out.align = out.align.max(align);
        out.depth = out.depth.max(depth + 1);
        out.occurrences = out
            .occurrences
            .checked_add(1)
            .and_then(|v| v.checked_add(occurrences))
            .ok_or(index)?;
        if out.depth > 128 || out.occurrences > 65_536 || out.size > 1_048_576 {
            return Err(index);
        }
    }
    out.size = out
        .size
        .checked_add(out.align - 1)
        .ok_or(fields.len().saturating_sub(1))?
        / out.align
        * out.align;
    if out.size > 1_048_576 {
        return Err(fields.len().saturating_sub(1));
    }
    Ok(out)
}
