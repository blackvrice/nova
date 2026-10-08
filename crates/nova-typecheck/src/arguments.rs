use crate::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamedCall {
    pub callee: DefId,
    pub arguments: Vec<HirId>,
    /// Source argument index -> parameter index.
    pub parameters: Vec<usize>,
    /// Omitted defaults in declaration order; provided arguments remain source ordered.
    pub defaults: Vec<DefaultArgument>,
}

pub(super) struct ArgumentMapping {
    pub parameters: Vec<Option<usize>>,
    pub valid: bool,
}

impl Checker<'_> {
    pub(super) fn collect_parameter_indices(&mut self) {
        for (index, definition) in self.resolved.definitions.iter().enumerate() {
            let DefinitionKind::Function(id) = definition.kind else {
                continue;
            };
            let node = &self.module.nodes()[id.0];
            let HirKind::Function { parameters, .. } = node.kind else {
                continue;
            };
            let names = node.children[..parameters]
                .iter()
                .enumerate()
                .filter_map(|(at, id)| {
                    let HirKind::Parameter { name, .. } = self.module.nodes()[id.0].kind else {
                        return None;
                    };
                    Some((name, at))
                })
                .collect();
            self.parameter_indices.insert(index, names);
            if node.children[..parameters]
                .iter()
                .any(|p| self.module.nodes()[p.0].children.len() == 2)
            {
                self.default_functions.insert(index);
            }
        }
    }

    pub(super) fn has_named(&self, id: HirId) -> bool {
        self.module.nodes()[id.0].children[1..]
            .iter()
            .any(|c| matches!(self.module.nodes()[c.0].kind, HirKind::NamedArgument { .. }))
    }

    pub(super) fn argument_value(&self, id: HirId) -> HirId {
        let node = &self.module.nodes()[id.0];
        if matches!(node.kind, HirKind::NamedArgument { .. }) {
            node.children[0]
        } else {
            id
        }
    }

    pub(super) fn reject_named_constructor(&mut self, id: HirId) {
        let node = &self.module.nodes()[id.0];
        let label =
            node.children[1..]
                .iter()
                .find_map(|arg| match self.module.nodes()[arg.0].kind {
                    HirKind::NamedArgument { name_span, .. } => Some(name_span),
                    _ => None,
                });
        if let Some(span) = label {
            self.report(
                2201,
                span,
                "this callee accepts positional arguments only",
                None,
            );
            self.argument_mappings[id.0] = Some(ArgumentMapping {
                parameters: vec![None; node.children.len() - 1],
                valid: false,
            });
        }
    }

    pub(super) fn prepare_named_call(&mut self, id: HirId) {
        let named = self.has_named(id);
        let node = &self.module.nodes()[id.0];
        let Some(def) = self.callee_definition(node.children[0]) else {
            return;
        };
        let DefinitionKind::Function(function) = self.resolved.definitions[def.0].kind else {
            if named {
                self.reject_named_constructor(id);
            }
            return;
        };
        let offset = usize::from(self.resolved.method_owners.contains_key(&def.0));
        if !named && offset == 0 && !self.default_functions.contains(&def.0) {
            return;
        }
        let declaration = &self.module.nodes()[function.0];
        let HirKind::Function { parameters, .. } = declaration.kind else {
            return;
        };
        if !named && node.children.len() - 1 > parameters - offset {
            self.report(
                2201,
                node.span,
                "too many positional arguments",
                Some(declaration.span),
            );
            self.argument_mappings[id.0] = Some(ArgumentMapping {
                parameters: (0..node.children.len() - 1)
                    .map(|i| (i + offset < parameters).then_some(i + offset))
                    .collect(),
                valid: false,
            });
            return;
        }
        let mut filled: Vec<Option<HirId>> = vec![None; parameters];
        if offset == 1 {
            filled[0] = Some(self.module.nodes()[node.children[0].0].children[0]);
        }
        let mut mapping = Vec::with_capacity(node.children.len() - 1);
        let mut first_label = None;
        let mut valid = true;
        for (ordinal, &arg) in node.children[1..].iter().enumerate() {
            let argument = &self.module.nodes()[arg.0];
            let parameter = if let HirKind::NamedArgument {
                name, name_span, ..
            } = argument.kind
            {
                first_label.get_or_insert(name_span);
                let at = self
                    .parameter_indices
                    .get(&def.0)
                    .and_then(|names| names.get(&name))
                    .copied();
                if at.is_none() {
                    self.report(
                        2201,
                        name_span,
                        "unknown parameter label",
                        Some(declaration.span),
                    );
                    let labels = declaration.children[..parameters]
                        .iter()
                        .take(8)
                        .filter_map(|p| {
                            let HirKind::Parameter { name, .. } = self.module.nodes()[p.0].kind
                            else {
                                return None;
                            };
                            self.module.symbol(name)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    self.result
                        .diagnostics
                        .last_mut()
                        .expect("reported diagnostic")
                        .notes
                        .push(format!(
                            "known parameter labels: {labels}{}",
                            if parameters > 8 {
                                ", ... (see declaration)"
                            } else {
                                ""
                            }
                        ));
                    valid = false;
                }
                at
            } else if let Some(label) = first_label {
                self.report(
                    2201,
                    argument.span,
                    "positional argument follows a named argument",
                    Some(label),
                );
                valid = false;
                None
            } else if ordinal + offset >= parameters {
                self.report(
                    2201,
                    argument.span,
                    "too many positional arguments",
                    Some(declaration.span),
                );
                valid = false;
                None
            } else {
                Some(ordinal + offset)
            };
            let parameter = parameter.and_then(|at| {
                if let Some(first) = filled[at] {
                    self.report(
                        2201,
                        match argument.kind {
                            HirKind::NamedArgument { name_span, .. } => name_span,
                            _ => argument.span,
                        },
                        "parameter is supplied more than once",
                        Some(self.module.nodes()[first.0].span),
                    );
                    let parameter_span = self.module.nodes()[declaration.children[at].0].span;
                    self.result
                        .diagnostics
                        .last_mut()
                        .expect("reported diagnostic")
                        .secondary
                        .push(Label {
                            span: parameter_span,
                            message: "parameter declared here".into(),
                        });
                    valid = false;
                    None
                } else {
                    filled[at] = Some(arg);
                    Some(at)
                }
            });
            mapping.push(parameter);
        }
        if valid {
            if let Some(at) = filled.iter().enumerate().find_map(|(at, value)| {
                (value.is_none()
                    && self.module.nodes()[declaration.children[at].0]
                        .children
                        .len()
                        == 1)
                    .then_some(at)
            }) {
                self.report(
                    2201,
                    node.span,
                    "missing required argument",
                    Some(self.module.nodes()[declaration.children[at].0].span),
                );
                if let HirKind::Parameter { name, .. } =
                    self.module.nodes()[declaration.children[at].0].kind
                {
                    let name = self.module.symbol(name).unwrap_or("?");
                    self.result
                        .diagnostics
                        .last_mut()
                        .expect("reported diagnostic")
                        .notes
                        .push(format!("required parameter: {name}"));
                }
                valid = false;
            }
        }
        self.argument_mappings[id.0] = Some(ArgumentMapping {
            parameters: mapping,
            valid,
        });
    }

    pub(super) fn finish_named_call(&mut self, id: HirId) {
        let node = &self.module.nodes()[id.0];
        let Some(def) = self.callee_definition(node.children[0]) else {
            self.set(id, Type::Error);
            return;
        };
        let signature = self.result.signatures[def.0]
            .clone()
            .expect("collected signature");
        let mapping = self.argument_mappings[id.0]
            .as_ref()
            .expect("prepared named call");
        let parameters = mapping.parameters.clone();
        let mut wrong = !mapping.valid || !self.constructor_visible(def, node.span);
        for (&argument, parameter) in node.children[1..].iter().zip(&parameters) {
            if let Some(at) = parameter {
                wrong |= self.mismatch(
                    self.argument_value(argument),
                    signature.parameters[*at],
                    signature.parameter_spans[*at],
                );
            }
            wrong |= self.ty(self.result.type_table[argument.0]) == Type::Error;
        }
        self.result.calls[id.0] = Some(def);
        if wrong {
            self.set(id, Type::Error)
        } else {
            let DefinitionKind::Function(function) = self.resolved.definitions[def.0].kind else {
                self.set(id, Type::Error);
                return;
            };
            let declaration = &self.module.nodes()[function.0];
            let mut supplied = vec![false; signature.parameters.len()];
            if self.resolved.method_owners.contains_key(&def.0) {
                supplied[0] = true;
            }
            for at in parameters.iter().flatten() {
                supplied[*at] = true;
            }
            let defaults = supplied
                .iter()
                .enumerate()
                .filter_map(|(index, &supplied)| {
                    if supplied {
                        return None;
                    }
                    let parameter = declaration.children[index];
                    let wrapper = self.module.nodes()[parameter.0].children[1];
                    Some(DefaultArgument {
                        index,
                        parameter,
                        initializer: self.module.nodes()[wrapper.0].children[0],
                    })
                })
                .collect();
            self.result.named_calls[id.0] = Some(NamedCall {
                callee: def,
                defaults,
                arguments: node.children[1..].to_vec(),
                parameters: parameters
                    .into_iter()
                    .map(|p| p.expect("valid complete mapping"))
                    .collect(),
            });
            self.result.type_table[id.0] = signature.return_type;
        }
    }
}
