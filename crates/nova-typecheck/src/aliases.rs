//! P21 transparent aliases. Both graph passes and type normalization are iterative.
use crate::Checker;
use nova_hir::HirKind;
use nova_resolve::DefinitionKind;
use nova_types::Type;
use std::collections::{BTreeMap, BTreeSet};

impl Checker<'_> {
    pub(super) fn collect_aliases(&mut self) {
        let aliases = self
            .resolved
            .definitions
            .iter()
            .enumerate()
            .filter_map(|(def, d)| {
                if let DefinitionKind::TypeAlias(id) = d.kind {
                    Some((def, id))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let error = self.result.types.intern(Type::Error);
        let positions = aliases
            .iter()
            .take(1024)
            .enumerate()
            .map(|(i, &(def, _))| (def, i))
            .collect::<BTreeMap<_, _>>();
        for (position, &(def, id)) in aliases.iter().enumerate() {
            self.set(id, Type::Unit);
            if position >= 1024 {
                self.result.aliases.insert(def, error);
                if position == 1024 {
                    self.enum_limit(
                        self.resolved.definitions[def].span.expect("alias name"),
                        "alias count limit: 1024",
                    );
                }
            }
        }
        let count = aliases.len().min(1024);
        let mut graph = vec![vec![]; count];
        let mut reverse = vec![vec![]; count];
        for (from, &(_, id)) in aliases.iter().take(count).enumerate() {
            let mut pending = self.module.nodes()[id.0].children.clone();
            let mut seen = BTreeSet::new();
            while let Some(at) = pending.pop() {
                let node = &self.module.nodes()[at.0];
                if let HirKind::TypeName(name) = node.kind {
                    if let Some(def) = self.type_definition(at, name) {
                        if let Some(&to) = positions.get(&def.0) {
                            if seen.insert(to) {
                                graph[from].push(to);
                                reverse[to].push(from);
                            }
                        }
                    }
                }
                pending.extend(node.children.iter().rev().copied());
            }
        }
        let mut seen = vec![false; count];
        let mut order = vec![];
        for root in 0..count {
            if seen[root] {
                continue;
            }
            seen[root] = true;
            let mut work = vec![(root, 0)];
            while let Some((at, next)) = work.last_mut() {
                if *next == graph[*at].len() {
                    order.push(*at);
                    work.pop();
                } else {
                    let to = graph[*at][*next];
                    *next += 1;
                    if !seen[to] {
                        seen[to] = true;
                        work.push((to, 0));
                    }
                }
            }
        }
        seen.fill(false);
        for &root in order.iter().rev() {
            if seen[root] {
                continue;
            }
            let mut component = vec![];
            let mut work = vec![root];
            seen[root] = true;
            while let Some(at) = work.pop() {
                component.push(at);
                for &to in &reverse[at] {
                    if !seen[to] {
                        seen[to] = true;
                        work.push(to);
                    }
                }
            }
            component.sort_unstable();
            if component.len() > 1 || graph[root].contains(&root) {
                for &at in &component {
                    let (def, id) = aliases[at];
                    self.result.aliases.insert(def, error);
                    let target = self.module.nodes()[id.0].children[0];
                    self.report(
                        2103,
                        self.module.nodes()[target.0].span,
                        "cyclic type alias",
                        None,
                    );
                    let diagnostic = self
                        .result
                        .diagnostics
                        .last_mut()
                        .expect("cycle diagnostic");
                    for &member in &component {
                        let declaration = &self.resolved.definitions[aliases[member].0];
                        diagnostic.secondary.push(nova_diagnostics::Label {
                            span: declaration.span.expect("alias declaration"),
                            message: format!("alias {} in this cycle", declaration.name),
                        });
                    }
                }
            }
        }
        // Postorder normalizes dependencies before references. Cyclic and excess
        // aliases already cache ErrorType, suppressing diagnostics at their uses.
        for at in order {
            let (def, id) = aliases[at];
            if !self.result.aliases.contains_key(&def) {
                let target = self.module.nodes()[id.0].children[0];
                let ty = self.type_syntax(target);
                self.result.aliases.insert(def, ty);
            }
            self.result.definition_types[def] = self.result.aliases[&def];
        }
        // Visit invalid targets too, keeping source tables complete without
        // reporting another cycle or treating an invalid alias as nominal.
        for &(def, id) in &aliases {
            self.result.definition_types[def] = self.result.aliases[&def];
            self.type_syntax(self.module.nodes()[id.0].children[0]);
        }
    }
}
