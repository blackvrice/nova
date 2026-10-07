//! P15 intrinsic Copy sum specialization and contextual construction.
use crate::Checker;
use nova_hir::{HirId, HirKind, SymbolId};
use nova_types::{
    EnumId, EnumShape, EnumVariant, StructField, StructId, StructShape, SumFamily, SumKey, Type,
    TypeId, VariantId,
};
impl Checker<'_> {
    pub(super) fn builtin_family(&self, id: HirId, name: SymbolId) -> Option<SumFamily> {
        if self.type_definition(id, name).is_some() {
            return None;
        }
        let scope = self.resolved.node_scopes[self.module.units()[self.module.owner(id)?].root.0]?;
        let spelling = self.module.symbol(name)?;
        if self.resolved.scopes[scope.0]
            .failed_types
            .contains(spelling)
        {
            return None;
        }
        match spelling {
            "Option" => Some(SumFamily::Option),
            "Result" => Some(SumFamily::Result),
            _ => None,
        }
    }
    pub(super) fn sum_head(&self, id: HirId) -> Option<(SumFamily, &str)> {
        match self.module.nodes()[id.0].kind {
            HirKind::None | HirKind::PatternNone => Some((SumFamily::Option, "None")),
            HirKind::VariantPath { owner, name, .. }
            | HirKind::PatternVariant { owner, name, .. } => {
                Some((self.builtin_family(id, owner)?, self.module.symbol(name)?))
            }
            _ => None,
        }
    }
    pub(super) fn sum_type_syntax(&mut self, id: HirId) -> Type {
        let node = &self.module.nodes()[id.0];
        let family = match node.kind {
            HirKind::NullableType => SumFamily::Option,
            HirKind::GenericType { name, name_span } => {
                if let Some(family) = self.builtin_family(id, name) {
                    family
                } else {
                    if self.type_definition(id, name).is_some()
                        || matches!(
                            self.module.symbol(name),
                            Some(
                                "int"
                                    | "uint"
                                    | "float"
                                    | "double"
                                    | "int8"
                                    | "int16"
                                    | "int32"
                                    | "int64"
                                    | "uint8"
                                    | "uint16"
                                    | "uint32"
                                    | "uint64"
                                    | "float32"
                                    | "float64"
                                    | "bool"
                                    | "char"
                                    | "string"
                                    | "void"
                                    | "never"
                            )
                        )
                    {
                        self.report(1102, name_span, "user generic types are unsupported", None);
                    } else {
                        self.struct_type_name(id, name);
                    }
                    return Type::Error;
                }
            }
            _ => return Type::Error,
        };
        let args = node
            .children
            .iter()
            .map(|c| self.ty(self.result.type_table[c.0]))
            .collect();
        self.intern_sum(
            id,
            SumKey {
                family,
                arguments: args,
            },
            false,
        )
    }
    fn source_key(&self, id: HirId) -> (u32, usize, usize) {
        let span = self.module.nodes()[id.0].span;
        (span.file().as_u32(), span.start(), span.end())
    }
    pub(super) fn intern_sum(&mut self, id: HirId, key: SumKey, ready: bool) -> Type {
        let expected = if key.family == SumFamily::Option {
            1
        } else {
            2
        };
        if key.arguments.len() != expected {
            self.report(
                2101,
                self.module.nodes()[id.0].span,
                "builtin family type arity does not match",
                None,
            );
            return Type::Error;
        }
        for (at, &ty) in key.arguments.iter().enumerate() {
            if ty == Type::Error {
                return Type::Error;
            }
            if !ty.numeric()
                && !matches!(ty, Type::Unit | Type::Bool | Type::Char)
                && ty.aggregate().is_none()
            {
                let node = &self.module.nodes()[id.0];
                let span = node
                    .children
                    .get(at + usize::from(node.kind == HirKind::Call))
                    .map_or(node.span, |c| self.module.nodes()[c.0].span);
                self.report(1102, span, "builtin payload must be Copy", None);
                return Type::Error;
            }
        }
        let first = self.sum_sources.get(&key).copied();
        if first.map_or(true, |first| self.source_key(id) < self.source_key(first)) {
            self.sum_sources.insert(key.clone(), id);
        }
        if let Some(&eid) = self.sum_shapes.get(&key) {
            self.result.sum_origins.insert(eid, self.sum_sources[&key]);
            return if ready && self.result.structs[&StructId(eid.0)].layout.is_none() {
                Type::Error
            } else {
                Type::Enum(eid)
            };
        }
        if self.result.sums.len() >= 4096 {
            return Type::Error;
        }
        let eid = EnumId(
            self.resolved.definitions.len() + self.result.tuple_ids.len() + self.result.sums.len(),
        );
        let names = if key.family == SumFamily::Option {
            ["Some", "None"]
        } else {
            ["Success", "Error"]
        };
        let mut variants = vec![];
        let mut fields = vec![];
        let mut sources = vec![];
        for (index, name) in names.into_iter().enumerate() {
            let fs = key
                .arguments
                .get(index)
                .map(|&ty| {
                    vec![StructField {
                        name: "0".into(),
                        ty,
                        mutable: false,
                    }]
                })
                .unwrap_or_default();
            fields.extend(fs.iter().cloned());
            if !fs.is_empty() {
                sources.push(
                    self.module.nodes()[id.0]
                        .children
                        .get(index)
                        .copied()
                        .unwrap_or(id),
                );
            }
            variants.push(EnumVariant {
                name: name.into(),
                fields: fs,
                layout: None,
            });
        }
        let name = format!("{:?}#{}", key.family, eid.0);
        let mut shape = EnumShape {
            name: name.clone(),
            variants,
        };
        let layout = if ready {
            match nova_types::enum_layout(&mut shape, &self.result.structs) {
                Ok(layout) => Some(layout),
                Err(_) => {
                    self.enum_limit(
                        self.module.nodes()[id.0].span,
                        "mixed aggregate limits: depth 128, size 1048576, expanded fields 65536",
                    );
                    return Type::Error;
                }
            }
        } else {
            None
        };
        self.result.enums.insert(eid, shape);
        self.result.structs.insert(
            StructId(eid.0),
            StructShape {
                name,
                fields,
                layout,
            },
        );
        self.result.field_sources.insert(StructId(eid.0), sources);
        self.result.sums.insert(eid, key.clone());
        self.result.sum_origins.insert(eid, self.sum_sources[&key]);
        self.sum_shapes.insert(key, eid);
        Type::Enum(eid)
    }
    fn record_sum_origin(&mut self, eid: EnumId, id: HirId) {
        let key = self.result.sums[&eid].clone();
        if self.source_key(id) < self.source_key(self.sum_sources[&key]) {
            self.sum_sources.insert(key, id);
            self.result.sum_origins.insert(eid, id);
        }
    }
    pub(super) fn check_sum_limit(&mut self) {
        if self.sum_sources.len() > 4096 {
            let mut sources = self.sum_sources.values().copied().collect::<Vec<_>>();
            sources.sort_by_key(|&id| self.source_key(id));
            self.enum_limit(
                self.module.nodes()[sources[4096].0].span,
                "builtin specialization limit: 4096",
            );
        }
    }
    pub(super) fn prepare_sum(
        &mut self,
        head: HirId,
        expected: Option<TypeId>,
    ) -> Option<VariantId> {
        let (family, name) = self.sum_head(head)?;
        let index = match (family, name) {
            (SumFamily::Option, "Some") | (SumFamily::Result, "Success") => 0,
            (SumFamily::Option, "None") | (SumFamily::Result, "Error") => 1,
            _ => return None,
        };
        let Type::Enum(eid) = expected.map(|t| self.ty(t))? else {
            return None;
        };
        if self.result.sums.get(&eid)?.family != family {
            return None;
        }
        let v = VariantId {
            enumeration: eid,
            index,
        };
        self.result.variants[head.0] = Some(v);
        Some(v)
    }
    pub(super) fn sum_error(&mut self, head: HirId, whole: HirId, expected: Option<TypeId>) {
        let (family, name) = self.sum_head(head).expect("intrinsic head");
        let known = matches!(
            (family, name),
            (SumFamily::Option, "Some" | "None") | (SumFamily::Result, "Success" | "Error")
        );
        let (code, span, message) = if !known {
            let span = match self.module.nodes()[head.0].kind {
                HirKind::VariantPath { name_span, .. }
                | HirKind::PatternVariant { name_span, .. } => name_span,
                _ => self.module.nodes()[head.0].span,
            };
            (2001, span, "undefined builtin variant")
        } else if expected.is_some_and(|t| self.ty(t) != Type::Error) {
            (
                2101,
                self.module.nodes()[whole.0].span,
                "constructor family differs from expected type",
            )
        } else {
            (
                2103,
                self.module.nodes()[whole.0].span,
                "constructor requires a complete expected sum type",
            )
        };
        if !expected.is_some_and(|t| self.ty(t) == Type::Error) {
            self.report(code, span, message, None);
        }
        self.set(whole, Type::Error);
    }
    pub(super) fn finish_sum_value(&mut self, id: HirId, expected: Option<TypeId>) {
        if matches!(
            self.sum_head(id),
            Some((SumFamily::Option, "Some") | (SumFamily::Result, "Success" | "Error"))
        ) {
            self.report(
                2201,
                self.module.nodes()[id.0].span,
                "payload variant requires arguments",
                None,
            );
            self.set(id, Type::Error);
            return;
        }
        if let Some(v) = self.prepare_sum(id, expected) {
            self.record_sum_origin(v.enumeration, id);
            self.finish_variant(id, false);
        } else {
            self.sum_error(id, id, expected);
        }
    }
    pub(super) fn finish_sum_call(&mut self, id: HirId, expected: Option<TypeId>) {
        let node = &self.module.nodes()[id.0];
        let head = node.children[0];
        let (family, name) = self.sum_head(head).expect("intrinsic head");
        let nullary = family == SumFamily::Option && name == "None";
        let known = matches!(
            (family, name),
            (SumFamily::Option, "Some" | "None") | (SumFamily::Result, "Success" | "Error")
        );
        if known && (nullary || node.children.len() != 2) {
            self.report(
                2201,
                node.span,
                "variant argument arity does not match",
                None,
            );
            self.set(id, Type::Error);
            return;
        }
        let some = family == SumFamily::Option && name == "Some";
        let mut variant = self.prepare_sum(head, expected);
        if variant.is_none() && some && expected.is_none() && node.children.len() == 2 {
            let payload = self.ty(self.result.type_table[node.children[1].0]);
            let ty = self.intern_sum(
                id,
                SumKey {
                    family,
                    arguments: vec![payload],
                },
                true,
            );
            let tid = self.result.types.intern(ty);
            variant = self.prepare_sum(head, Some(tid));
            if ty == Type::Error {
                self.set(id, Type::Error);
                return;
            }
        }
        self.result.variants[id.0] = variant;
        if let Some(v) = variant {
            self.record_sum_origin(v.enumeration, id);
            self.finish_variant_call(id);
        } else {
            self.sum_error(head, id, expected);
        }
    }
}
