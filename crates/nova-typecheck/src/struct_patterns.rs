//! P34 label correspondence. HIR stays source-ordered; the valid typed plan is field-ordered.
use crate::Checker;
use nova_hir::{HirId, HirKind};
use nova_types::StructId;
use std::collections::BTreeMap;
impl Checker<'_> {
    pub(super) fn struct_pattern_mapping(
        &mut self,
        id: HirId,
        sid: StructId,
    ) -> Option<Vec<usize>> {
        let node = &self.module.nodes()[id.0];
        let names = self.result.structs[&sid]
            .fields
            .iter()
            .enumerate()
            .map(|(i, f)| (f.name.clone(), i))
            .collect::<BTreeMap<_, _>>();
        let mut filled: Vec<Option<HirId>> = vec![None; names.len()];
        let mut mapping = vec![];
        let mut first_label = None;
        let mut valid = true;
        for (ordinal, &child) in node.children.iter().enumerate() {
            let arg = &self.module.nodes()[child.0];
            let at = if let HirKind::PatternField {
                name, name_span, ..
            } = arg.kind
            {
                first_label.get_or_insert(name_span);
                let at = names
                    .get(self.module.symbol(name).expect("field label"))
                    .copied();
                if at.is_none() {
                    self.report(2201, name_span, "unknown struct pattern field label", None);
                    valid = false;
                }
                at
            } else if let Some(label) = first_label {
                self.report(
                    2201,
                    arg.span,
                    "positional pattern follows a named field",
                    Some(label),
                );
                valid = false;
                None
            } else if ordinal >= filled.len() {
                self.report(2201, arg.span, "too many positional struct patterns", None);
                valid = false;
                None
            } else {
                Some(ordinal)
            };
            if let Some(at) = at {
                if let Some(first) = filled[at] {
                    let span = match arg.kind {
                        HirKind::PatternField { name_span, .. } => name_span,
                        _ => arg.span,
                    };
                    self.report(
                        2201,
                        span,
                        "struct pattern field is supplied more than once",
                        Some(self.module.nodes()[first.0].span),
                    );
                    valid = false;
                } else {
                    filled[at] = Some(child);
                }
                mapping.push(at);
            }
        }
        if valid {
            if let Some(at) = filled.iter().position(Option::is_none) {
                self.report(
                    2201,
                    node.span,
                    "missing required struct pattern field",
                    Some(self.module.nodes()[self.result.field_sources[&sid][at].0].span),
                );
                return None;
            }
            Some(mapping)
        } else {
            None
        }
    }
}
