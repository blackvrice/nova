//! P02 lexical name resolution. HIR identities and scopes remain immutable afterwards.
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, Severity};
use nova_hir::{HirId, HirKind, Module, Visibility};
use nova_source::Span;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DefId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionKind {
    BuiltinPrint,
    Function(HirId),
    GlobalConst(HirId),
    Parameter(HirId),
    Local(HirId),
}
#[derive(Debug, Eq, PartialEq)]
pub struct Definition {
    pub name: String,
    pub kind: DefinitionKind,
    pub span: Option<Span>,
    pub scope: ScopeId,
    /// True only for P04 local var declarations; parameters and let are Read.
    pub mutable: bool,
    pub constant: bool,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Scope {
    pub parent: Option<ScopeId>,
    pub definitions: BTreeMap<String, DefId>,
    pub failed_imports: BTreeSet<String>,
    pub import_spans: BTreeMap<String, Span>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resolution {
    Definition(DefId),
    Error,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Resolved {
    pub definitions: Vec<Definition>,
    pub scopes: Vec<Scope>,
    pub node_scopes: Vec<Option<ScopeId>>,
    pub declaration_ids: Vec<Option<DefId>>,
    pub references: Vec<Option<Resolution>>,
    pub diagnostics: Vec<Diagnostic>,
}
impl Resolved {
    pub fn dump(&self) -> String {
        let mut output = String::new();
        for (index, definition) in self.definitions.iter().enumerate() {
            let _ = writeln!(
                output,
                "def {index} {:?} {:?} scope {}",
                definition.name, definition.kind, definition.scope.0
            );
            if definition.mutable {
                let _ = writeln!(output, "mutable def {index}");
            }
            if definition.constant {
                let _ = writeln!(output, "const def {index}");
            }
        }
        for (index, resolution) in self.references.iter().enumerate() {
            if let Some(resolution) = resolution {
                let _ = writeln!(output, "ref {index} {resolution:?}");
            }
        }
        output
    }
}

enum Work {
    Visit(HirId, ScopeId),
    Body(HirId, ScopeId),
    Declare(HirId, ScopeId),
}

pub fn resolve(module: &Module) -> Resolved {
    let size = module.nodes().len();
    let mut result = Resolved {
        definitions: vec![],
        scopes: vec![Scope {
            parent: None,
            definitions: BTreeMap::new(),
            failed_imports: BTreeSet::new(),
            import_spans: BTreeMap::new(),
        }],
        node_scopes: vec![None; size],
        declaration_ids: vec![None; size],
        references: vec![None; size],
        diagnostics: vec![],
    };
    result.definitions.push(Definition {
        name: "print".into(),
        kind: DefinitionKind::BuiltinPrint,
        span: None,
        scope: ScopeId(0),
        mutable: false,
        constant: false,
    });
    result.scopes[0]
        .definitions
        .insert("print".into(), DefId(0));
    let scopes = module
        .units()
        .iter()
        .map(|unit| {
            let scope = new_scope(&mut result, ScopeId(0));
            result.node_scopes[unit.root.0] = Some(scope);
            scope
        })
        .collect::<Vec<_>>();
    let items = module.items().collect::<Vec<_>>();
    for &item in &items {
        let root_scope = scopes[module.owner(item).expect("file ownership")];
        if let HirKind::Function {
            name, name_span, ..
        } = module.nodes()[item.0].kind
        {
            declare(
                &mut result,
                root_scope,
                module.symbol(name).expect("lowered symbol exists"),
                DefinitionKind::Function(item),
                name_span,
                item,
            );
        } else if let HirKind::Binding {
            name,
            name_span,
            constant: true,
            ..
        } = module.nodes()[item.0].kind
        {
            // P06 reserves print only for new global consts. P02 user functions
            // still shadow the prelude in the closer source root scope.
            if module.symbol(name) == Some("print")
                && !result.scopes[root_scope.0]
                    .definitions
                    .contains_key("print")
            {
                result.diagnostics.push(diagnostic(
                    2002,
                    name_span,
                    "global const print conflicts with builtin print",
                    None,
                ));
            }
            declare(
                &mut result,
                root_scope,
                module.symbol(name).expect("global symbol exists"),
                DefinitionKind::GlobalConst(item),
                name_span,
                item,
            );
            if let Some(def) = result.declaration_ids[item.0] {
                result.definitions[def.0].constant = true;
            }
        }
    }
    // Capture direct declarations before any alias is inserted. Reexports do
    // not accidentally become eligible according to import processing order.
    let direct = scopes
        .iter()
        .map(|scope| result.scopes[scope.0].definitions.clone())
        .collect::<Vec<_>>();
    for &item in &items {
        let node = &module.nodes()[item.0];
        let HirKind::Import {
            alias, alias_span, ..
        } = node.kind
        else {
            continue;
        };
        let owner = module.owner(item).expect("import owner");
        let scope = scopes[owner];
        result.node_scopes[item.0] = Some(scope);
        let path = module.import_path(item).expect("validated import path");
        let name = alias
            .and_then(|symbol| module.symbol(symbol))
            .unwrap_or(path[path.len() - 1]);
        let binding_span =
            alias_span.unwrap_or(module.nodes()[node.children.last().expect("path").0].span);
        let target_path = path[..path.len() - 1].join("::");
        let target = module
            .units()
            .iter()
            .position(|unit| unit.path == target_path);
        let definition =
            target.and_then(|target| direct[target].get(path[path.len() - 1]).copied());
        if let Some(previous) = result.scopes[scope.0].definitions.get(name).copied() {
            let previous_span = result.scopes[scope.0]
                .import_spans
                .get(name)
                .copied()
                .or(result.definitions[previous.0].span);
            let (primary, secondary) = match previous_span {
                Some(previous)
                    if previous.file() == binding_span.file()
                        && previous.start() > binding_span.start() =>
                {
                    (previous, Some(binding_span))
                }
                _ => (binding_span, previous_span),
            };
            result.diagnostics.push(diagnostic(
                2002,
                primary,
                &format!("duplicate import binding {name}"),
                secondary,
            ));
            continue;
        }
        if result.scopes[scope.0].failed_imports.contains(name) {
            result.diagnostics.push(diagnostic(
                2002,
                binding_span,
                &format!("duplicate import binding {name}"),
                result.scopes[scope.0].import_spans.get(name).copied(),
            ));
            continue;
        }
        let reexport = target.is_some_and(|target| {
            module.nodes()[module.units()[target].root.0]
                .children
                .iter()
                .any(|&id| {
                    let HirKind::Import { alias, .. } = module.nodes()[id.0].kind else {
                        return false;
                    };
                    let imported = module.import_path(id).expect("import path");
                    alias
                        .and_then(|symbol| module.symbol(symbol))
                        .unwrap_or(imported[imported.len() - 1])
                        == path[path.len() - 1]
                })
        });
        let failure = if let Some(def) = definition {
            let declaration = &result.definitions[def.0];
            let id = match declaration.kind {
                DefinitionKind::Function(id) | DefinitionKind::GlobalConst(id) => id,
                _ => unreachable!("direct declaration"),
            };
            if target != Some(owner) && module.visibility(id) == Visibility::Private {
                Some(diagnostic(
                    2004,
                    node.span,
                    "import target is private",
                    declaration.span,
                ))
            } else if name == "print" && declaration.constant {
                Some(diagnostic(
                    2002,
                    binding_span,
                    "const import print conflicts with builtin print",
                    declaration.span,
                ))
            } else {
                None
            }
        } else {
            Some(diagnostic(
                if reexport { 1102 } else { 2001 },
                node.span,
                if reexport {
                    "importing another import alias is not supported"
                } else {
                    "undefined module or direct import target"
                },
                None,
            ))
        };
        result.scopes[scope.0]
            .import_spans
            .insert(name.into(), binding_span);
        if let Some(failure) = failure {
            result.diagnostics.push(failure);
            result.scopes[scope.0].failed_imports.insert(name.into());
        } else {
            result.scopes[scope.0]
                .definitions
                .insert(name.into(), definition.expect("valid import"));
        }
    }
    for &item in &items {
        let root_scope = scopes[module.owner(item).expect("file ownership")];
        let node = &module.nodes()[item.0];
        let mut pending;
        result.node_scopes[item.0] = Some(root_scope);
        if let HirKind::Function { parameters, .. } = node.kind {
            let function_scope = new_scope(&mut result, root_scope);
            for &parameter in &node.children[..parameters] {
                if let HirKind::Parameter { name, name_span } = module.nodes()[parameter.0].kind {
                    declare(
                        &mut result,
                        function_scope,
                        module.symbol(name).expect("parameter symbol exists"),
                        DefinitionKind::Parameter(parameter),
                        name_span,
                        parameter,
                    );
                    result.node_scopes[parameter.0] = Some(function_scope);
                }
            }
            pending = vec![Work::Body(
                *node.children.last().expect("function body exists"),
                function_scope,
            )];
        } else if matches!(node.kind, HirKind::Binding { constant: true, .. }) {
            pending = node
                .children
                .iter()
                .rev()
                .map(|&id| Work::Visit(id, root_scope))
                .collect();
        } else {
            continue;
        }
        while let Some(work) = pending.pop() {
            let (id, scope, body) = match work {
                Work::Declare(id, scope) => {
                    if let HirKind::Binding {
                        name, name_span, ..
                    } = module.nodes()[id.0].kind
                    {
                        declare(
                            &mut result,
                            scope,
                            module.symbol(name).expect("binding symbol exists"),
                            DefinitionKind::Local(id),
                            name_span,
                            id,
                        );
                        if let Some(def) = result.declaration_ids[id.0] {
                            result.definitions[def.0].mutable = matches!(
                                module.nodes()[id.0].kind,
                                HirKind::Binding { mutable: true, .. }
                            );
                            result.definitions[def.0].constant = matches!(
                                module.nodes()[id.0].kind,
                                HirKind::Binding { constant: true, .. }
                            );
                        }
                    }
                    continue;
                }
                Work::Visit(id, scope) => (id, scope, false),
                Work::Body(id, scope) => (id, scope, true),
            };
            let node = &module.nodes()[id.0];
            let scope = if node.kind == HirKind::Block && !body {
                new_scope(&mut result, scope)
            } else {
                scope
            };
            result.node_scopes[id.0] = Some(scope);
            match node.kind {
                HirKind::Name(name) => {
                    let spelling = module.symbol(name).expect("reference symbol exists");
                    let mut current = Some(scope);
                    let mut found = None;
                    while let Some(scope) = current {
                        if let Some(def) = result.scopes[scope.0].definitions.get(spelling) {
                            found = Some(*def);
                            break;
                        }
                        if result.scopes[scope.0].failed_imports.contains(spelling) {
                            break;
                        }
                        current = result.scopes[scope.0].parent;
                    }
                    result.references[id.0] = Some(if let Some(def) = found {
                        Resolution::Definition(def)
                    } else {
                        if !current.is_some_and(|scope| {
                            result.scopes[scope.0].failed_imports.contains(spelling)
                        }) {
                            result.diagnostics.push(diagnostic(
                                2001,
                                node.span,
                                &format!("undefined name {spelling}"),
                                None,
                            ));
                        }
                        Resolution::Error
                    });
                }
                HirKind::Binding { .. } => {
                    pending.push(Work::Declare(id, scope));
                    // Initializer is resolved before declaration, including shadowing initializers.
                    if let Some(&value) = node.children.last() {
                        pending.push(Work::Visit(value, scope));
                    }
                }
                _ => {
                    for &child in node.children.iter().rev() {
                        pending.push(Work::Visit(child, scope));
                    }
                }
            }
        }
    }
    result
}

fn new_scope(result: &mut Resolved, parent: ScopeId) -> ScopeId {
    let id = ScopeId(result.scopes.len());
    result.scopes.push(Scope {
        parent: Some(parent),
        definitions: BTreeMap::new(),
        failed_imports: BTreeSet::new(),
        import_spans: BTreeMap::new(),
    });
    id
}
fn declare(
    result: &mut Resolved,
    scope: ScopeId,
    name: &str,
    kind: DefinitionKind,
    span: Span,
    node: HirId,
) {
    let id = DefId(result.definitions.len());
    if let Some(previous) = result.scopes[scope.0].definitions.get(name) {
        let previous = &result.definitions[previous.0];
        result.diagnostics.push(diagnostic(
            2002,
            span,
            &format!("duplicate definition {name}"),
            previous.span,
        ));
    } else {
        result.scopes[scope.0].definitions.insert(name.into(), id);
    }
    result.definitions.push(Definition {
        name: name.into(),
        kind,
        span: Some(span),
        scope,
        mutable: false,
        constant: false,
    });
    result.declaration_ids[node.0] = Some(id);
}
fn diagnostic(code: u16, span: Span, message: &str, secondary: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::new(code).expect("approved name codes"),
        severity: Severity::Error,
        message: message.into(),
        primary: Label {
            span,
            message: message.into(),
        },
        secondary: secondary
            .into_iter()
            .map(|span| Label {
                span,
                message: "previous definition".into(),
            })
            .collect(),
        notes: vec![],
        suggestions: vec![],
    }
}
