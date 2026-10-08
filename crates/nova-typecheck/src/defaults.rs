use crate::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefaultArgument {
    pub index: usize,
    pub parameter: HirId,
    pub initializer: HirId,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParameterDefault {
    pub callee: DefId,
    pub argument: DefaultArgument,
    pub evaluation: ConstEvaluation,
}

impl Checker<'_> {
    pub(super) fn check_defaults(&mut self) {
        for function in self.module.semantic_items() {
            let declaration = &self.module.nodes()[function.0];
            let HirKind::Function { parameters, .. } = declaration.kind else {
                continue;
            };
            let Some(callee) = self.resolved.declaration_ids[function.0] else {
                continue;
            };
            for (index, &parameter) in declaration.children[..parameters].iter().enumerate() {
                let node = &self.module.nodes()[parameter.0];
                let Some(&wrapper) = node.children.get(1) else {
                    continue;
                };
                let initializer = self.module.nodes()[wrapper.0].children[0];
                let annotation = node.children[0];
                let expected = self.result.type_table[annotation.0];
                let before = self.result.diagnostics.len();
                let return_type = self.result.types.intern(Type::Unit);
                self.walk(
                    initializer,
                    Context {
                        expected: Some(expected),
                        expected_span: Some(self.module.nodes()[annotation.0].span),
                        return_type,
                        return_span: None,
                        direct_callee: false,
                        loop_depth: 0,
                        const_declaration: Some(parameter),
                    },
                );
                let mismatch = self.mismatch(
                    initializer,
                    expected,
                    Some(self.module.nodes()[annotation.0].span),
                );
                let invalid = mismatch
                    || self.ty(self.result.type_table[initializer.0]) == Type::Error
                    || self.ty(expected) == Type::Error
                    || self.result.diagnostics.len() != before;
                let evaluation = if invalid {
                    ConstEvaluation::Invalid
                } else {
                    match const_eval::evaluate(
                        self.module,
                        self.resolved,
                        &self.result,
                        initializer,
                    ) {
                        Ok((value, nodes)) => ConstEvaluation::Value { value, nodes },
                        Err(error) => {
                            self.report(
                                error.code,
                                self.module.nodes()[error.node.0].span,
                                error.message,
                                Some(node.span),
                            );
                            if error.code == 3202 {
                                self.result
                                    .diagnostics
                                    .last_mut()
                                    .expect("reported budget")
                                    .notes
                                    .push(format!(
                                        "default initializer node limit: {CONST_NODE_LIMIT}"
                                    ));
                            }
                            ConstEvaluation::Failed {
                                code: error.code,
                                nodes: error.nodes,
                            }
                        }
                    }
                };
                self.result.type_table[wrapper.0] =
                    if matches!(evaluation, ConstEvaluation::Value { .. }) {
                        expected
                    } else {
                        self.result.types.intern(Type::Error)
                    };
                self.result.defaults[parameter.0] = Some(ParameterDefault {
                    callee,
                    argument: DefaultArgument {
                        index,
                        parameter,
                        initializer,
                    },
                    evaluation,
                });
            }
        }
    }
}
