//! P12 declarations, nominal fields and finite by-value layouts.
use crate::Checker;
use nova_hir::{HirId, HirKind, SymbolId, Visibility};
use nova_resolve::{DefId, DefinitionKind, Resolution};
use nova_source::Span;
use nova_types::{struct_layout, FieldId, StructField, StructId, StructShape, Type};
use std::collections::{BTreeMap, BTreeSet};
impl Checker<'_> {
    pub(super) fn type_definition(&self, id: HirId, name: SymbolId) -> Option<DefId> {
        let owner = self.module.owner(id)?;
        let scope = self.resolved.node_scopes[self.module.units()[owner].root.0]?;
        self.resolved.scopes[scope.0]
            .types
            .get(self.module.symbol(name)?)
            .copied()
    }
    pub(super) fn struct_type_name(&mut self, id: HirId, name: SymbolId) -> Type {
        if let Some(def) = self.type_definition(id, name) {
            return if matches!(
                self.resolved.definitions[def.0].kind,
                DefinitionKind::Enum(_)
            ) {
                Type::Enum(nova_types::EnumId(def.0))
            } else {
                Type::Struct(StructId(def.0))
            };
        }
        let owner = self.module.owner(id).expect("type owner");
        let scope =
            self.resolved.node_scopes[self.module.units()[owner].root.0].expect("root scope");
        if !self.resolved.scopes[scope.0]
            .failed_types
            .contains(self.module.symbol(name).expect("type symbol"))
        {
            self.report(
                2001,
                self.module.nodes()[id.0].span,
                "undefined type name",
                None,
            );
        }
        Type::Error
    }
    pub(super) fn collect_structs(&mut self) {
        let declarations = self
            .resolved
            .definitions
            .iter()
            .enumerate()
            .filter_map(|(i, d)| match d.kind {
                DefinitionKind::Struct(id) => Some((StructId(i), id)),
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut invalid = BTreeSet::new();
        for (position, &(sid, id)) in declarations.iter().enumerate() {
            let node = &self.module.nodes()[id.0];
            let HirKind::Struct { name, name_span } = node.kind else {
                continue;
            };
            if position >= 1024 || node.children.len() > 1024 {
                self.report(
                    8901,
                    name_span,
                    "P12 limit: 1024 structs per bundle and 1024 fields per struct",
                    None,
                );
                invalid.insert(sid);
            }
            let mut fields = vec![];
            let mut sources = vec![];
            let mut names = BTreeMap::new();
            for &field in &node.children {
                let f = &self.module.nodes()[field.0];
                let HirKind::Field {
                    name,
                    name_span,
                    mutable,
                    ..
                } = f.kind
                else {
                    invalid.insert(sid);
                    self.set(field, Type::Error);
                    continue;
                };
                let spelling = self.module.symbol(name).expect("field symbol");
                if let Some(previous) = names.insert(spelling, name_span) {
                    self.report(2002, name_span, "duplicate field", Some(previous));
                    invalid.insert(sid);
                }
                let ty_id = f.children[0];
                let tid = self.type_syntax(ty_id);
                let ty = self.ty(tid);
                if !ty.numeric()
                    && !matches!(
                        ty,
                        Type::Bool
                            | Type::Char
                            | Type::Unit
                            | Type::Struct(_)
                            | Type::Tuple(_)
                            | Type::Enum(_)
                            | Type::Error
                    )
                {
                    self.report(
                        1102,
                        self.module.nodes()[ty_id.0].span,
                        "P12 fields must be Copy scalar or struct values",
                        None,
                    );
                    invalid.insert(sid);
                }
                if ty == Type::Error {
                    invalid.insert(sid);
                }
                fields.push(StructField {
                    name: spelling.into(),
                    ty,
                    mutable,
                });
                sources.push(field);
                self.set(field, Type::Unit);
            }
            self.result.structs.insert(
                sid,
                StructShape {
                    name: self.module.symbol(name).expect("struct symbol").into(),
                    fields,
                    layout: None,
                },
            );
            self.result.field_sources.insert(sid, sources);
            self.set(id, Type::Unit);
        }
        self.collect_enums(&mut invalid);
        // Include signature/local annotations before visiting the mixed aggregate graph.
        for (index, node) in self.module.nodes().iter().enumerate() {
            if node.kind == HirKind::TupleType {
                self.type_syntax(HirId(index));
            }
        }
        for (&sid, shape) in &self.result.structs {
            if shape.fields.iter().any(|f| f.ty == Type::Error) {
                invalid.insert(sid);
            }
        }
        let roots = self.result.structs.keys().copied().collect::<Vec<_>>();
        let mut colors = BTreeMap::new();
        for root in roots {
            if colors.contains_key(&root) {
                continue;
            }
            colors.insert(root, 1u8);
            let mut stack = vec![(root, 0usize)];
            while let Some(&(sid, next)) = stack.last() {
                let fields = &self.result.structs[&sid].fields;
                if next == fields.len() {
                    stack.pop();
                    colors.insert(sid, 2);
                    if !invalid.contains(&sid) {
                        if fields
                            .iter()
                            .any(|f| f.ty.aggregate().is_some_and(|dep| invalid.contains(&dep)))
                        {
                            invalid.insert(sid);
                            continue;
                        }
                        let computed = if let Some(enumeration) =
                            self.result.enums.get_mut(&nova_types::EnumId(sid.0))
                        {
                            nova_types::enum_layout(enumeration, &self.result.structs)
                        } else {
                            struct_layout(fields, &self.result.structs)
                        };
                        match computed {
                            Ok(layout) => {
                                self.result.structs.get_mut(&sid).expect("shape").layout =
                                    Some(layout)
                            }
                            Err(field) => {
                                invalid.insert(sid);
                                let span = self.aggregate_span(sid);
                                let source = self.result.field_sources[&sid].get(field).copied();
                                self.report(8901, span, "aggregate layout limit: depth 128, size 1048576, expanded fields 65536", source.map(|f| self.module.nodes()[f.0].span));
                                if !self.result.enums.is_empty() {
                                    self.result.diagnostics.last_mut().expect("layout diagnostic").notes.push("mixed aggregate limits: depth 128, size 1048576, expanded fields 65536".into());
                                }
                            }
                        }
                    }
                    continue;
                }
                stack.last_mut().expect("DFS frame").1 += 1;
                let Some(dep) = fields[next].ty.aggregate() else {
                    continue;
                };
                match colors.get(&dep).copied().unwrap_or(0) {
                    0 => {
                        colors.insert(dep, 1);
                        stack.push((dep, 0));
                    }
                    1 => {
                        let begin = stack
                            .iter()
                            .position(|(s, _)| *s == dep)
                            .expect("active dependency");
                        let cycle = stack[begin..].to_vec();
                        let source = self.aggregate_field_source(sid, next);
                        self.report(
                            2101,
                            self.module.nodes()[source.0].span,
                            "recursive by-value aggregate layout",
                            None,
                        );
                        for &(s, at) in &cycle {
                            invalid.insert(s);
                            let source_at = self.aggregate_field_source(s, at.saturating_sub(1));
                            if source_at != source {
                                self.result
                                    .diagnostics
                                    .last_mut()
                                    .expect("cycle diagnostic")
                                    .secondary
                                    .push(nova_diagnostics::Label {
                                        span: self.module.nodes()[source_at.0].span,
                                        message: "cycle element".into(),
                                    });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    fn aggregate_field_source(&self, sid: StructId, index: usize) -> HirId {
        let at = self.result.field_sources[&sid][index];
        if matches!(self.module.nodes()[at.0].kind, HirKind::Field { .. }) {
            self.module.nodes()[at.0].children[0]
        } else {
            at
        }
    }
    fn aggregate_span(&self, sid: StructId) -> Span {
        if self.result.tuple_ids.contains(&sid) {
            self.module.nodes()[self.result.tuple_origins[&sid].0].span
        } else {
            self.resolved.definitions[sid.0].span.expect("struct span")
        }
    }
    pub(super) fn intern_tuple(&mut self, id: HirId, elements: Vec<Type>, ready: bool) -> Type {
        let node = &self.module.nodes()[id.0];
        let mut valid = true;
        if elements.len() > 1024 {
            self.report(8901, node.span, "tuple element limit: 1024", None);
            valid = false;
        }
        for (index, &ty) in elements.iter().enumerate() {
            if ty == Type::Error {
                valid = false;
            } else if !ty.numeric()
                && !matches!(
                    ty,
                    Type::Unit
                        | Type::Bool
                        | Type::Char
                        | Type::Struct(_)
                        | Type::Tuple(_)
                        | Type::Enum(_)
                )
            {
                self.report(
                    1102,
                    self.module.nodes()[node.children[index].0].span,
                    "tuple elements must be Copy scalar or aggregate values",
                    None,
                );
                valid = false;
            }
        }
        if !valid {
            return Type::Error;
        }
        let key = |at: HirId| {
            let span = self.module.nodes()[at.0].span;
            (span.file().as_u32(), span.start(), at.0)
        };
        let first = self
            .tuple_shape_sources
            .entry(elements.clone())
            .or_insert(id);
        if key(id) < key(*first) {
            *first = id;
        }
        if let Some(&sid) = self.tuple_shapes.get(&elements) {
            return if ready && self.result.structs[&sid].layout.is_none() {
                Type::Error
            } else {
                Type::Tuple(sid)
            };
        }
        if self.result.tuple_ids.len() >= 4096 {
            return Type::Error;
        }
        let fields = elements
            .iter()
            .enumerate()
            .map(|(index, &ty)| StructField {
                name: index.to_string(),
                ty,
                mutable: true,
            })
            .collect::<Vec<_>>();
        let sid = StructId(self.resolved.definitions.len() + self.result.tuple_ids.len());
        let layout = if ready {
            match struct_layout(&fields, &self.result.structs) {
                Ok(layout) => Some(layout),
                Err(field) => {
                    self.report(
                        8901,
                        node.span,
                        "aggregate layout limit: depth 128, size 1048576, expanded fields 65536",
                        node.children
                            .get(field)
                            .map(|f| self.module.nodes()[f.0].span),
                    );
                    return Type::Error;
                }
            }
        } else {
            None
        };
        self.tuple_shapes.insert(elements, sid);
        self.result.tuple_ids.insert(sid);
        self.result.tuple_origins.insert(sid, id);
        self.result.field_sources.insert(sid, node.children.clone());
        self.result.structs.insert(
            sid,
            StructShape {
                name: format!("tuple#{}", sid.0),
                fields,
                layout,
            },
        );
        Type::Tuple(sid)
    }
    pub(super) fn check_tuple_limit(&mut self) {
        if self.tuple_shape_sources.len() > 4096 {
            let mut sources = self
                .tuple_shape_sources
                .values()
                .copied()
                .collect::<Vec<_>>();
            sources.sort_by_key(|at| {
                let span = self.module.nodes()[at.0].span;
                (span.file().as_u32(), span.start(), at.0)
            });
            self.report(
                8901,
                self.module.nodes()[sources[4096].0].span,
                "unique tuple shape limit: 4096",
                None,
            );
        }
    }
    pub(super) fn tuple_projection(&mut self, id: HirId, index: SymbolId, span: Span) {
        let recv = self.module.nodes()[id.0].children[0];
        let ty = self.ty(self.result.type_table[recv.0]);
        let Type::Tuple(sid) = ty else {
            if ty != Type::Error {
                self.report(2101, span, "numeric selector requires a tuple", None);
            }
            self.set(id, Type::Error);
            return;
        };
        let index = self
            .module
            .symbol(index)
            .expect("selector symbol")
            .parse::<usize>()
            .ok();
        let Some(index) = index.filter(|&i| i < self.result.structs[&sid].fields.len()) else {
            self.report(2001, span, "tuple index is out of range", None);
            self.set(id, Type::Error);
            return;
        };
        let ty = self.result.structs[&sid].fields[index].ty;
        self.result.projections[id.0] = Some(FieldId {
            structure: sid,
            index,
        });
        self.set(id, ty);
    }
    pub(super) fn constructor_visible(&mut self, def: DefId, span: Span) -> bool {
        let DefinitionKind::Struct(id) = self.resolved.definitions[def.0].kind else {
            return true;
        };
        let shape = &self.result.structs[&StructId(def.0)];
        if shape.layout.is_none() {
            return false;
        }
        let mut valid = true;
        for &field in &self.module.nodes()[id.0].children {
            if matches!(
                self.module.nodes()[field.0].kind,
                HirKind::Field {
                    visibility: Visibility::Private,
                    ..
                }
            ) && self.module.nodes()[field.0].span.file() != span.file()
            {
                self.report(
                    2004,
                    span,
                    "constructor requires access to every field",
                    Some(self.module.nodes()[field.0].span),
                );
                valid = false;
            }
        }
        valid
    }
    pub(super) fn projection(&mut self, id: HirId, name: SymbolId, span: Span) {
        let recv = self.module.nodes()[id.0].children[0];
        let ty = self.ty(self.result.type_table[recv.0]);
        let Type::Struct(sid) = ty else {
            if ty != Type::Error {
                self.report(2101, span, "field receiver must be a struct", None);
            }
            self.set(id, Type::Error);
            return;
        };
        let spelling = self.module.symbol(name).expect("field symbol");
        let Some(index) = self.result.structs[&sid]
            .fields
            .iter()
            .position(|f| f.name == spelling)
        else {
            self.report(2001, span, "undefined struct field", None);
            self.set(id, Type::Error);
            return;
        };
        let field = self.result.field_sources[&sid][index];
        if matches!(
            self.module.nodes()[field.0].kind,
            HirKind::Field {
                visibility: Visibility::Private,
                ..
            }
        ) && self.module.nodes()[field.0].span.file() != span.file()
        {
            self.report(
                2004,
                span,
                "struct field is private",
                Some(self.module.nodes()[field.0].span),
            );
            self.set(id, Type::Error);
            return;
        }
        let ty = self.result.structs[&sid].fields[index].ty;
        self.result.projections[id.0] = Some(FieldId {
            structure: sid,
            index,
        });
        self.set(id, ty);
    }
    pub(super) fn assignment_target(&mut self, mut id: HirId) -> bool {
        let mut path = vec![];
        while matches!(
            self.module.nodes()[id.0].kind,
            HirKind::Projection { .. } | HirKind::TupleProjection { .. }
        ) {
            path.push(id);
            id = self.module.nodes()[id.0].children[0];
        }
        let Some(Resolution::Definition(def)) = self.resolved.references[id.0] else {
            return false;
        };
        let d = &self.resolved.definitions[def.0];
        if !d.mutable {
            self.report(
                3004,
                self.module.nodes()[id.0].span,
                "assignment requires a mutable local var",
                d.span,
            );
            return false;
        }
        for id in path.into_iter().rev() {
            let Some(field) = self.result.projections[id.0] else {
                return false;
            };
            if !self.result.structs[&field.structure].fields[field.index].mutable {
                let HirKind::Projection { name_span, .. } = self.module.nodes()[id.0].kind else {
                    unreachable!()
                };
                let source = self.result.field_sources[&field.structure][field.index];
                self.report(
                    3004,
                    name_span,
                    "assignment path crosses a let field",
                    Some(self.module.nodes()[source.0].span),
                );
                return false;
            }
        }
        true
    }
}
