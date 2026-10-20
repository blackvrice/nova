//! P28 evaluates one compound initializer, then caches its Copy components.
use crate::{Checker, ConstEvaluation, CONST_NODE_LIMIT};
use nova_hir::HirId;
use nova_types::{ConstValue, Type};

impl Checker<'_> {
    pub(super) fn evaluate_const_destructuring(&mut self, id: HirId, valid: bool) {
        let plan = self.result.destructuring_bindings[id.0].expect("prepared const pattern");
        let mut patterns = Vec::new();
        let mut work = vec![plan.pattern];
        while let Some(pattern) = work.pop() {
            patterns.push(pattern);
            work.extend(
                self.module.nodes()[pattern.0]
                    .children
                    .iter()
                    .rev()
                    .copied(),
            );
        }
        let outcome = if valid {
            match crate::const_eval::evaluate(
                self.module,
                self.resolved,
                &self.result,
                plan.initializer,
            ) {
                Ok((value, nodes)) => ConstEvaluation::Value { value, nodes },
                Err(error) => {
                    self.report(
                        error.code,
                        self.module.nodes()[error.node.0].span,
                        error.message,
                        Some(self.module.nodes()[id.0].span),
                    );
                    if error.code == 3202 {
                        self.result
                            .diagnostics
                            .last_mut()
                            .expect("const error")
                            .notes
                            .push(format!("const initializer node limit: {CONST_NODE_LIMIT}"));
                    }
                    ConstEvaluation::Failed {
                        code: error.code,
                        nodes: error.nodes,
                    }
                }
            }
        } else {
            ConstEvaluation::Invalid
        };
        if let ConstEvaluation::Value { value, nodes } = &outcome {
            let mut work = vec![(plan.pattern, value)];
            while let Some((pattern, value)) = work.pop() {
                if let Some(def) = self.resolved.declaration_ids[pattern.0] {
                    self.result.const_values[def.0] = ConstEvaluation::Value {
                        value: value.clone(),
                        nodes: *nodes,
                    };
                }
                let typed = self.result.recursive_patterns[pattern.0]
                    .as_ref()
                    .expect("valid typed pattern");
                if !typed.children.is_empty() {
                    let fields: &[ConstValue] = match (typed.kind, value) {
                        (crate::RecursivePatternKind::NamedField, value) => {
                            std::slice::from_ref(value)
                        }
                        (
                            crate::RecursivePatternKind::Struct(expected),
                            ConstValue::Struct(actual, fields),
                        ) if expected == *actual => fields,
                        (
                            crate::RecursivePatternKind::Tuple(expected),
                            ConstValue::Tuple(actual, fields),
                        ) if expected == *actual => fields,
                        (
                            crate::RecursivePatternKind::Variant(expected),
                            ConstValue::Enum(actual, fields),
                        ) if expected == *actual => fields,
                        _ => unreachable!("typed irrefutable const component"),
                    };
                    for (at, &child) in typed.children.iter().enumerate().rev() {
                        let index = if matches!(
                            typed.kind,
                            crate::RecursivePatternKind::Struct(_)
                                | crate::RecursivePatternKind::Tuple(_)
                                | crate::RecursivePatternKind::Variant(_)
                        ) {
                            typed.field_indices[at]
                        } else {
                            at
                        };
                        work.push((child, &fields[index]));
                    }
                }
            }
        } else {
            let error_type = self.result.types.intern(Type::Error);
            for pattern in patterns {
                if let Some(def) = self.resolved.declaration_ids[pattern.0] {
                    self.result.const_values[def.0] = outcome.clone();
                    self.result.definition_types[def.0] = error_type;
                }
            }
        }
        // Keep the single whole value even for Unit/only-discard declarations.
        self.result.const_destructuring_values[id.0] = outcome;
    }
}
