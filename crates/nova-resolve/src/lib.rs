//! P02 lexical name resolution. HIR identities and scopes remain immutable afterwards.
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, Severity};
use nova_hir::{HirId, HirKind, Module};
use nova_source::Span;
use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DefId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScopeId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionKind {
    BuiltinPrint,
    Function(HirId),
    Parameter(HirId),
    Local(HirId),
}
#[derive(Debug, Eq, PartialEq)]
pub struct Definition {
    pub name: String,
    pub kind: DefinitionKind,
    pub span: Option<Span>,
    pub scope: ScopeId,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Scope {
    pub parent: Option<ScopeId>,
    pub definitions: BTreeMap<String, DefId>,
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
    });
    result.scopes[0]
        .definitions
        .insert("print".into(), DefId(0));
    let root_scope = new_scope(&mut result, ScopeId(0));
    result.node_scopes[module.root().0] = Some(root_scope);
    let items = &module.nodes()[module.root().0].children;
    for &item in items {
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
        }
    }
    for &item in items {
        let node = &module.nodes()[item.0];
        let HirKind::Function { parameters, .. } = node.kind else {
            continue;
        };
        result.node_scopes[item.0] = Some(root_scope);
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
        let mut pending = vec![Work::Body(
            *node.children.last().expect("function body exists"),
            function_scope,
        )];
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
                        current = result.scopes[scope.0].parent;
                    }
                    result.references[id.0] = Some(if let Some(def) = found {
                        Resolution::Definition(def)
                    } else {
                        result.diagnostics.push(diagnostic(
                            2001,
                            node.span,
                            &format!("undefined name {spelling}"),
                            None,
                        ));
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
