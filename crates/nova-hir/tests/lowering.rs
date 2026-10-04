use nova_ast::{Arena, NodeKind};
use nova_hir::{lower, HirKind, LoweringError, SourceOrigin};
use nova_lexer::{lex, normalize_ends};
use nova_parser::parse;
use nova_source::{SourceDatabase, Span};

fn lower_source(source: &str) -> nova_hir::Module {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", source.into()).unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let first = lower(&sources, &parsed.arena, parsed.root).unwrap();
    let second = lower(&sources, &parsed.arena, parsed.root).unwrap();
    assert_eq!(first, second);
    for (index, node) in first.nodes().iter().enumerate() {
        sources.slice(node.span).unwrap();
        for child in &node.children {
            assert!(child.0 < index);
        }
    }
    first
}

#[test]
fn hello_hir_snapshot() {
    let module = lower_source("func main() {\n    print(\"Hello, Nova\")\n}\n");
    assert_eq!(module.dump(), include_str!("snapshots/hello.hir"));
}

#[test]
fn canonical_unit_return_and_alias_origins() {
    let module = lower_source("func f(x: int) -> void { return () }\nfunc main() {}");
    assert!(module
        .nodes()
        .iter()
        .any(|n| matches!(n.kind, HirKind::TypeName(s) if module.symbol(s) == Some("int32"))));
    let explicit = module
        .nodes()
        .iter()
        .filter(|n| n.kind == HirKind::UnitType && matches!(n.origin, SourceOrigin::Source(_)))
        .count();
    let implicit = module
        .nodes()
        .iter()
        .filter(|n| {
            n.kind == HirKind::UnitType && matches!(n.origin, SourceOrigin::ImplicitReturn(_))
        })
        .count();
    assert_eq!((explicit, implicit), (1, 1));
    let module = lower_source("func f() -> () { return () }");
    assert_eq!(
        module
            .nodes()
            .iter()
            .filter(|n| n.kind == HirKind::UnitType)
            .count(),
        1
    );
}

#[test]
fn strings_decode_escapes_braces_and_nul_without_interpolation_reordering() {
    let module = lower_source("func main() { print(\"{{x}}\\n\\u{1F642}\\0 {1+2} end\") }");
    let strings = module
        .nodes()
        .iter()
        .filter_map(|n| {
            if let HirKind::String(s) = &n.kind {
                Some(s.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(strings, ["{x}\n🙂\0 ", " end"]);
    let interpolation = module
        .nodes()
        .iter()
        .find(|n| n.kind == HirKind::InterpolatedString)
        .unwrap();
    assert_eq!(interpolation.children.len(), 3);
    assert!(matches!(
        module.nodes()[interpolation.children[0].0].kind,
        HirKind::String(_)
    ));
    assert_eq!(
        module.nodes()[interpolation.children[1].0].kind,
        HirKind::Interpolation
    );
}

#[test]
fn value_names_are_not_primitive_aliases() {
    let module = lower_source(
        "func int() -> int { return 1 }\nfunc main() { let float=int(); print(\"{float}\") }",
    );
    assert!(module.nodes().iter().any(
        |n| matches!(n.kind, HirKind::Function { name, .. } if module.symbol(name) == Some("int"))
    ));
    assert!(module.nodes().iter().any(
        |n| matches!(n.kind, HirKind::Binding { name, .. } if module.symbol(name) == Some("float"))
    ));
}

#[test]
fn malformed_ast_is_an_api_error_and_error_nodes_survive() {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", "x".into()).unwrap();
    let span = Span::new(file, 0, 1).unwrap();
    let mut arena = Arena::default();
    let bad = arena.insert(NodeKind::Call, span, vec![]).unwrap();
    let root = arena.insert(NodeKind::Root, span, vec![bad]).unwrap();
    assert_eq!(
        lower(&sources, &arena, root),
        Err(LoweringError::MalformedAst)
    );
    let module = lower_source("func main() { let x=🙂 }");
    assert!(module.has_errors());
    assert!(lower_source("func ( ) {}").has_errors());
}

#[test]
fn truncated_parser_recovery_lowers_without_panicking() {
    let source = "func f(x: int) -> int { if true { return x } else { return 1 } }\nfunc main() { print(\"value {f(1)}\") }";
    for end in 0..=source.len() {
        lower_source(&source[..end]);
    }
}
