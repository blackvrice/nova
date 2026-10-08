use crate::Checker;
use nova_hir::{HirId, HirKind};
use nova_resolve::DefinitionKind;
use nova_types::{
    EnumId, EnumShape, EnumVariant, StructField, StructId, StructShape, Type, VariantId,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchPattern {
    Variant(VariantId),
    Bool(bool),
    Wildcard,
}

impl Checker<'_> {
    pub(super) fn enum_limit(&mut self, span: nova_source::Span, message: &str) {
        self.report(8901, span, message, None);
        self.result
            .diagnostics
            .last_mut()
            .expect("limit diagnostic")
            .notes
            .push(message.into());
    }
    pub(super) fn collect_enums(&mut self, invalid: &mut BTreeSet<StructId>) {
        let declarations = self
            .resolved
            .definitions
            .iter()
            .enumerate()
            .filter_map(|(i, d)| {
                if let DefinitionKind::Enum(id) = d.kind {
                    Some((EnumId(i), id))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (position, (eid, id)) in declarations.into_iter().enumerate() {
            let node = &self.module.nodes()[id.0];
            let HirKind::Enum { name, name_span } = node.kind else {
                continue;
            };
            let sid = StructId(eid.0);
            let mut variants = vec![];
            let mut flat = vec![];
            let mut sources = vec![];
            let mut names = BTreeMap::new();
            if position >= 1024 {
                if position == 1024 {
                    self.enum_limit(name_span, "Enum count limit: 1024");
                }
                invalid.insert(sid);
            }
            for (index, &variant) in node.children.iter().enumerate() {
                let v = &self.module.nodes()[variant.0];
                let HirKind::Variant { name, name_span } = v.kind else {
                    invalid.insert(sid);
                    continue;
                };
                let spelling = self.module.symbol(name).expect("variant name");
                if let Some(previous) = names.insert(spelling, name_span) {
                    self.report(2002, name_span, "duplicate variant", Some(previous));
                }
                if index == 1024 {
                    self.enum_limit(name_span, "variant count limit: 1024");
                    invalid.insert(sid);
                }
                let mut fields = vec![];
                for (at, &ty_node) in v.children.iter().enumerate() {
                    let tid = self.type_syntax(ty_node);
                    let ty = self.ty(tid);
                    if at == 1024 {
                        self.enum_limit(
                            self.module.nodes()[ty_node.0].span,
                            "payload component limit: 1024",
                        );
                        invalid.insert(sid);
                    }
                    if ty == Type::Error {
                        invalid.insert(sid);
                    } else if !ty.numeric()
                        && !matches!(ty, Type::Unit | Type::Bool | Type::Char)
                        && ty.aggregate().is_none()
                    {
                        self.report(
                            1102,
                            self.module.nodes()[ty_node.0].span,
                            "Enum payload must be Copy",
                            None,
                        );
                        invalid.insert(sid);
                    }
                    fields.push(StructField {
                        name: at.to_string(),
                        ty,
                        mutable: false,
                    });
                    sources.push(ty_node);
                }
                flat.extend(fields.iter().cloned());
                variants.push(EnumVariant {
                    name: spelling.into(),
                    fields,
                    layout: None,
                });
                self.set(variant, Type::Unit);
            }
            self.result.enums.insert(
                eid,
                EnumShape {
                    name: self.module.symbol(name).expect("enum name").into(),
                    variants,
                },
            );
            self.result.structs.insert(
                sid,
                StructShape {
                    name: self.module.symbol(name).expect("enum name").into(),
                    fields: flat,
                    layout: None,
                },
            );
            self.result.field_sources.insert(sid, sources);
            self.set(id, Type::Unit);
        }
    }

    pub(super) fn resolve_variant(&mut self, id: HirId) -> Option<VariantId> {
        if self.result.variants[id.0].is_some() {
            return self.result.variants[id.0];
        }
        if !self.attempted_variants.insert(id.0) {
            return None;
        }
        let (owner, owner_span, name, name_span) = match self.module.nodes()[id.0].kind {
            HirKind::VariantPath {
                owner,
                owner_span,
                name,
                name_span,
            }
            | HirKind::PatternVariant {
                owner,
                owner_span,
                name,
                name_span,
                ..
            } => (owner, owner_span, name, name_span),
            _ => return None,
        };
        let Some(def) = self.type_definition(id, owner) else {
            let root = self.module.units()[self.module.owner(id).expect("owner")].root;
            let scope = self.resolved.node_scopes[root.0].expect("root scope");
            if !self.resolved.scopes[scope.0]
                .failed_types
                .contains(self.module.symbol(owner).expect("type name"))
            {
                self.report(2001, owner_span, "undefined Enum type", None);
            }
            return None;
        };
        if matches!(
            self.resolved.definitions[def.0].kind,
            DefinitionKind::TypeAlias(_)
        ) {
            self.report(
                1102,
                self.module.nodes()[id.0].span,
                "alias variant heads are unsupported",
                None,
            );
            return None;
        }
        if !matches!(
            self.resolved.definitions[def.0].kind,
            DefinitionKind::Enum(_)
        ) {
            self.report(2101, owner_span, "variant owner must be Enum", None);
            return None;
        }
        let enumeration = EnumId(def.0);
        let shape = &self.result.enums[&enumeration];
        let Some(index) = shape
            .variants
            .iter()
            .position(|v| Some(v.name.as_str()) == self.module.symbol(name))
        else {
            self.report(2001, name_span, "undefined variant", None);
            return None;
        };
        let variant = VariantId { enumeration, index };
        self.result.variants[id.0] = Some(variant);
        Some(variant)
    }

    pub(super) fn finish_variant(&mut self, id: HirId, callee: bool) {
        if let Some(variant) = self.resolve_variant(id) {
            if callee {
                self.set(id, Type::Unit);
            } else if !self.result.enums[&variant.enumeration].variants[variant.index]
                .fields
                .is_empty()
            {
                self.report(
                    2201,
                    self.module.nodes()[id.0].span,
                    "payload variant requires arguments",
                    None,
                );
                self.set(id, Type::Error);
            } else {
                self.set(id, Type::Enum(variant.enumeration));
            }
        } else {
            self.set(id, Type::Error);
        }
    }
    pub(super) fn finish_variant_call(&mut self, id: HirId) {
        let v = self.result.variants[id.0].expect("checked variant");
        let node = &self.module.nodes()[id.0];
        let fields = self.result.enums[&v.enumeration].variants[v.index]
            .fields
            .clone();
        let mut wrong = fields.is_empty() || fields.len() != node.children.len() - 1;
        if wrong {
            self.report(
                2201,
                node.span,
                "variant argument arity does not match",
                None,
            );
        }
        for (&arg, field) in node.children[1..].iter().zip(fields) {
            let tid = self.result.types.intern(field.ty);
            wrong |= self.mismatch(arg, tid, None)
                || self.ty(self.result.type_table[arg.0]) == Type::Error;
        }
        self.set(
            id,
            if wrong {
                Type::Error
            } else {
                Type::Enum(v.enumeration)
            },
        );
    }

    pub(super) fn prepare_match(&mut self, id: HirId) {
        let node = &self.module.nodes()[id.0];
        let HirKind::Match { keyword } = node.kind else {
            return;
        };
        let ty = self.ty(self.result.type_table[node.children[0].0]);
        let domain = match ty {
            Type::Bool => 2,
            Type::Enum(e) => self.result.enums[&e].variants.len(),
            Type::Error => 0,
            _ => {
                self.report(
                    2101,
                    self.module.nodes()[node.children[0].0].span,
                    "match requires Bool or Enum",
                    None,
                );
                0
            }
        };
        let mut covered = vec![None; domain];
        for (index, &arm) in node.children[1..].iter().enumerate() {
            if self.module.nodes()[arm.0].kind != HirKind::Arm {
                continue;
            }
            let pattern = self.module.nodes()[arm.0].children[0];
            let p = &self.module.nodes()[pattern.0];
            if index == 1025 {
                self.enum_limit(p.span, "match arm limit: 1025");
            }
            let mut cases = vec![];
            match p.kind {
                HirKind::Wildcard => {
                    self.result.patterns[pattern.0] = Some(MatchPattern::Wildcard);
                    cases.extend(0..domain);
                }
                HirKind::PatternBoolean(b) => {
                    self.result.patterns[pattern.0] = Some(MatchPattern::Bool(b));
                    if ty == Type::Bool {
                        cases.push(usize::from(b));
                    } else if ty != Type::Error {
                        self.report(2101, p.span, "Bool pattern requires Bool scrutinee", None);
                    }
                }
                HirKind::PatternNone | HirKind::PatternVariant { .. } => {
                    let (arguments, owner_span, name_span) = match p.kind {
                        HirKind::PatternVariant {
                            arguments,
                            owner_span,
                            name_span,
                            ..
                        } => (arguments, owner_span, name_span),
                        _ => (false, p.span, p.span),
                    };
                    let builtin = self.sum_head(pattern).is_some();
                    let expected = self.result.types.intern(ty);
                    let variant = if builtin {
                        self.prepare_sum(pattern, Some(expected))
                    } else {
                        self.resolve_variant(pattern)
                    };
                    if builtin && variant.is_none() {
                        self.sum_error(pattern, pattern, Some(expected));
                    }
                    if let Some(v) = variant {
                        self.result.patterns[pattern.0] = Some(MatchPattern::Variant(v));
                        if ty == Type::Enum(v.enumeration) {
                            cases.push(v.index);
                        } else if ty != Type::Error {
                            self.report(
                                2101,
                                nova_source::Span::new(
                                    owner_span.file(),
                                    owner_span.start(),
                                    name_span.end(),
                                )
                                .expect("path span"),
                                "pattern Enum differs from scrutinee",
                                None,
                            );
                        }
                        let fields = self.result.enums[&v.enumeration].variants[v.index]
                            .fields
                            .clone();
                        if fields.len() != p.children.len() || arguments == fields.is_empty() {
                            self.report(2201, p.span, "variant pattern arity does not match", None);
                        }
                        for (at, &binder) in p.children.iter().enumerate() {
                            let bt = fields.get(at).map_or(Type::Error, |f| f.ty);
                            if let Some(def) = self.resolved.declaration_ids[binder.0] {
                                self.result.definition_types[def.0] = self.result.types.intern(bt);
                            }
                            self.set(binder, bt);
                        }
                    }
                }
                _ => {}
            }
            if !cases.is_empty() && cases.iter().all(|&at| covered[at].is_some()) {
                self.report(3102, p.span, "unreachable match arm", None);
                let mut previous = cases
                    .iter()
                    .filter_map(|&at| covered[at])
                    .collect::<Vec<HirId>>();
                previous.sort_by_key(|at| self.module.nodes()[at.0].span.start());
                previous.dedup();
                for at in previous {
                    self.result
                        .diagnostics
                        .last_mut()
                        .expect("arm diagnostic")
                        .secondary
                        .push(nova_diagnostics::Label {
                            span: self.module.nodes()[at.0].span,
                            message: "covering pattern".into(),
                        });
                }
            }
            for at in cases {
                if covered[at].is_none() {
                    covered[at] = Some(pattern);
                }
            }
            self.set(pattern, Type::Unit);
            self.set(arm, Type::Unit);
        }
        if domain > 0 && covered.iter().any(Option::is_none) {
            self.report(3101, keyword, "incomplete match", None);
            let missing = covered
                .iter()
                .enumerate()
                .filter(|(_, v)| v.is_none())
                .map(|(at, _)| match ty {
                    Type::Enum(e) => self.result.enums[&e].variants[at].name.clone(),
                    _ => if at == 0 { "false" } else { "true" }.into(),
                })
                .collect::<Vec<_>>();
            self.result
                .diagnostics
                .last_mut()
                .expect("coverage diagnostic")
                .notes
                .push(format!("missing: {}", missing.join(", ")));
        } else if domain > 0 {
            self.result.exhaustive_matches.insert(id.0);
        }
    }
}
