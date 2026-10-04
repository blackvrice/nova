//! P06 static global dependencies. All graph walks use explicit stacks.
use crate::{Checker, ConstEvaluation, Context};
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, Severity};
use nova_hir::{HirId, HirKind};
use nova_resolve::{DefId, DefinitionKind, Resolution};
use nova_types::Type;
use std::collections::BTreeSet;

#[derive(Clone, Copy)]
struct Edge {
    target: usize,
    reference: HirId,
}
struct Global {
    definition: DefId,
    node: HirId,
    edges: Vec<Edge>,
    invalid: bool,
}

impl Checker<'_> {
    pub(super) fn check_globals(&mut self) {
        let mut globals = Vec::new();
        let mut indices = vec![None; self.resolved.definitions.len()];
        for &node in &self.module.nodes()[self.module.root().0].children {
            let Some(definition) = self.resolved.declaration_ids[node.0] else {
                continue;
            };
            if matches!(
                self.resolved.definitions[definition.0].kind,
                DefinitionKind::GlobalConst(_)
            ) {
                indices[definition.0] = Some(globals.len());
                globals.push(Global {
                    definition,
                    node,
                    edges: vec![],
                    invalid: false,
                });
            }
        }
        // Collect even skipped RHS references, preserving first source-order edges.
        let duplicates = self
            .resolved
            .diagnostics
            .iter()
            .filter(|d| d.code == DiagnosticCode::new(2002).expect("approved duplicate code"))
            .map(|d| (d.primary.span.start(), d.primary.span.end()))
            .collect::<BTreeSet<_>>();
        for global in &mut globals {
            let mut seen = BTreeSet::new();
            let mut pending = self.module.nodes()[global.node.0]
                .children
                .iter()
                .rev()
                .copied()
                .collect::<Vec<_>>();
            while let Some(id) = pending.pop() {
                let node = &self.module.nodes()[id.0];
                match self.resolved.references[id.0] {
                    Some(Resolution::Error) => global.invalid = true,
                    Some(Resolution::Definition(def)) => {
                        if let Some(target) = indices[def.0] {
                            if seen.insert(target) {
                                global.edges.push(Edge {
                                    target,
                                    reference: id,
                                });
                            }
                        }
                    }
                    None => {}
                }
                global.invalid |= node.kind == HirKind::Error;
                pending.extend(node.children.iter().rev().copied());
            }
            global.invalid |= self.resolved.definitions[global.definition.0]
                .span
                .is_some_and(|span| duplicates.contains(&(span.start(), span.end())));
            if global.invalid {
                global.edges.clear();
                self.result.const_values[global.definition.0] = ConstEvaluation::Invalid;
            }
        }
        let order = postorder(&globals);
        let mut reverse = vec![vec![]; globals.len()];
        for (from, global) in globals.iter().enumerate() {
            for edge in &global.edges {
                reverse[edge.target].push(from);
            }
        }
        // Kosaraju components; both passes are iterative and O(V+E).
        let mut component_ids = vec![None; globals.len()];
        let mut components = Vec::new();
        for &root in order.iter().rev() {
            if component_ids[root].is_some() {
                continue;
            }
            let component = components.len();
            let mut members = vec![];
            let mut pending = vec![root];
            component_ids[root] = Some(component);
            while let Some(id) = pending.pop() {
                members.push(id);
                for &next in &reverse[id] {
                    if component_ids[next].is_none() {
                        component_ids[next] = Some(component);
                        pending.push(next);
                    }
                }
            }
            members.sort_unstable();
            components.push(members);
        }
        let mut cyclic = components
            .iter()
            .enumerate()
            .filter(|(_, members)| {
                members.len() > 1
                    || globals[members[0]]
                        .edges
                        .iter()
                        .any(|e| e.target == members[0])
            })
            .collect::<Vec<_>>();
        cyclic.sort_by_key(|(_, members)| members[0]);
        let mut colors = vec![0; globals.len()];
        let mut positions = vec![0; globals.len()];
        for (component, members) in cyclic {
            let (path, references) = cycle_path(
                &globals,
                &component_ids,
                component,
                members[0],
                &mut colors,
                &mut positions,
            );
            let closing = *references.last().expect("cyclic SCC has a closing edge");
            let mut secondary = Vec::new();
            let mut names = Vec::new();
            for &id in &path {
                let definition = &self.resolved.definitions[globals[id].definition.0];
                names.push(definition.name.as_str());
                if let Some(span) = definition.span {
                    secondary.push(Label {
                        span,
                        message: "const in dependency cycle".into(),
                    });
                }
            }
            for &reference in &references[..references.len() - 1] {
                secondary.push(Label {
                    span: self.module.nodes()[reference.0].span,
                    message: "dependency reference".into(),
                });
            }
            names.push(names[0]);
            self.result.diagnostics.push(Diagnostic {
                code: DiagnosticCode::new(3202).expect("approved const code"),
                severity: Severity::Error,
                message: "global const dependency cycle".into(),
                primary: Label {
                    span: self.module.nodes()[closing.0].span,
                    message: "reference closes dependency cycle".into(),
                },
                secondary,
                notes: vec![format!("const dependency cycle: {}", names.join(" -> "))],
                suggestions: vec![],
            });
            for &id in members {
                // No initializer evaluation occurred; zero is not a successful node count.
                self.result.const_values[globals[id].definition.0] = ConstEvaluation::Failed {
                    code: 3202,
                    nodes: 0,
                };
            }
        }
        let context = Context {
            expected: None,
            expected_span: None,
            return_type: self.result.types.intern(Type::Unit),
            return_span: None,
            direct_callee: false,
            loop_depth: 0,
        };
        for index in order {
            let global = &globals[index];
            if self.result.const_values[global.definition.0] != ConstEvaluation::Pending {
                continue;
            }
            if global.edges.iter().any(|edge| {
                !matches!(
                    self.result.const_values[globals[edge.target].definition.0],
                    ConstEvaluation::Value { .. }
                )
            }) {
                self.result.const_values[global.definition.0] = ConstEvaluation::Invalid;
                continue;
            }
            self.walk(global.node, context);
        }
    }
}

fn postorder(globals: &[Global]) -> Vec<usize> {
    let mut seen = vec![false; globals.len()];
    let mut order = Vec::new();
    for root in 0..globals.len() {
        if seen[root] {
            continue;
        }
        seen[root] = true;
        let mut stack = vec![(root, 0)];
        while let Some((id, next)) = stack.last_mut() {
            if let Some(edge) = globals[*id].edges.get(*next) {
                *next += 1;
                if !seen[edge.target] {
                    seen[edge.target] = true;
                    stack.push((edge.target, 0));
                }
            } else {
                order.push(*id);
                stack.pop();
            }
        }
    }
    order
}

/// First source-order DFS back edge within a cyclic SCC, starting at its earliest declaration.
fn cycle_path(
    globals: &[Global],
    components: &[Option<usize>],
    component: usize,
    root: usize,
    colors: &mut [u8],
    positions: &mut [usize],
) -> (Vec<usize>, Vec<HirId>) {
    let mut stack = vec![(root, 0)];
    let mut references = Vec::new();
    colors[root] = 1;
    while let Some((id, next)) = stack.last_mut() {
        if let Some(edge) = globals[*id].edges.get(*next) {
            *next += 1;
            if components[edge.target] != Some(component) {
                continue;
            }
            match colors[edge.target] {
                0 => {
                    colors[edge.target] = 1;
                    positions[edge.target] = stack.len();
                    references.push(edge.reference);
                    stack.push((edge.target, 0));
                }
                1 => {
                    let start = positions[edge.target];
                    let path = stack[start..].iter().map(|&(id, _)| id).collect();
                    let mut cycle_references = references[start..].to_vec();
                    cycle_references.push(edge.reference);
                    return (path, cycle_references);
                }
                _ => {}
            }
        } else {
            colors[*id] = 2;
            stack.pop();
            if !stack.is_empty() {
                references.pop();
            }
        }
    }
    unreachable!("cyclic SCC must contain a DFS back edge")
}
