use crate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RangeInfo {
    pub binder: DefId,
    pub ty: TypeId,
}

impl Checker<'_> {
    pub(super) fn prepare_range(&mut self, id: HirId) {
        let node = &self.module.nodes()[id.0];
        let HirKind::For { range, .. } = node.kind else {
            return;
        };
        let left = node.children[1];
        let right = node.children[2];
        let a = self.ty(self.result.type_table[left.0]);
        let b = self.ty(self.result.type_table[right.0]);
        let mut valid = true;
        for (value, ty) in [(left, a), (right, b)] {
            if ty.integer().is_none() {
                valid = false;
                if ty != Type::Error {
                    self.report(
                        2101,
                        self.module.nodes()[value.0].span,
                        "range bound must be an integer",
                        None,
                    );
                }
            }
        }
        let common = if valid {
            a.common_numeric(b).filter(|t| t.integer().is_some())
        } else {
            None
        };
        let ty = if let Some(common) = common {
            let expected = self.result.types.intern(common);
            self.mismatch(left, expected, None);
            self.mismatch(right, expected, None);
            expected
        } else {
            if valid {
                self.report(
                    2101,
                    range,
                    "range bounds have no lossless common integer type",
                    None,
                );
            }
            self.result.types.intern(Type::Error)
        };
        let binder = node.children[0];
        self.result.type_table[binder.0] = ty;
        if let Some(definition) = self.resolved.declaration_ids[binder.0] {
            self.result.definition_types[definition.0] = ty;
            self.result.ranges[id.0] = Some(RangeInfo {
                binder: definition,
                ty,
            });
        }
    }
}
