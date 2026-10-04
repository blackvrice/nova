use nova_ast::{Arena, NodeKind};

#[test]
fn p09_public_ast_float_leaves_cannot_bypass_literal_spelling_contract() {
    for literal in [
        "inf", "NaN", "1", ".5", "1.", "1__0.0", "1_.0", "1.0_", "1e", "1e+", "1e_1", "0x1.0",
        "1.0f32",
    ] {
        let mut sources = SourceDatabase::default();
        let file = sources.add("api.nova", literal.into()).unwrap();
        let span = Span::new(file, 0, literal.len()).unwrap();
        let mut arena = Arena::default();
        let value = arena.insert(NodeKind::Float, span, vec![]).unwrap();
        let error = arena.insert(NodeKind::Error, span, vec![value]).unwrap();
        let root = arena.insert(NodeKind::Root, span, vec![error]).unwrap();
        assert_eq!(
            lower(&sources, &arena, root),
            Err(LoweringError::MalformedAst),
            "{literal}"
        );
    }
}
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
fn p08_character_decoding_preserves_origin_without_string_brace_rules() {
    let module = lower_source(
        r#"func f(){let a='{';let b='}';let c='\'';let d='\0';let e='\u{1F642}';let f='\n';let g='\\';let h='\"';let i='\r';let j='\t'}"#,
    );
    let values = module
        .nodes()
        .iter()
        .filter_map(|n| {
            if let HirKind::Character(c) = n.kind {
                assert!(matches!(n.origin, SourceOrigin::Source(_)));
                assert!(n.children.is_empty());
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(
        values,
        ['{', '}', '\'', '\0', '🙂', '\n', '\\', '"', '\r', '\t']
    );
}

#[test]
fn p08_malformed_public_character_ast_is_rejected_without_panics() {
    for literal in [
        "a",
        "'",
        "''",
        "'''",
        "'ab'",
        "'{{'",
        "'\n'",
        "'e\u{301}'",
        r"'\q'",
        r"'\u{}'",
        r"'\u{D800}'",
        r"'\u{110000}'",
        r"'\u{0000000}'",
    ] {
        let mut sources = SourceDatabase::default();
        let file = sources.add("api.nova", literal.into()).unwrap();
        let span = Span::new(file, 0, literal.len()).unwrap();
        let mut arena = Arena::default();
        let value = arena.insert(NodeKind::Character, span, vec![]).unwrap();
        let error = arena.insert(NodeKind::Error, span, vec![value]).unwrap();
        let root = arena.insert(NodeKind::Root, span, vec![error]).unwrap();
        assert_eq!(
            lower(&sources, &arena, root),
            Err(LoweringError::MalformedAst),
            "{literal}"
        );
    }
}

#[test]
fn hello_hir_snapshot() {
    let module = lower_source("func main() {\n    print(\"Hello, Nova\")\n}\n");
    assert_eq!(module.dump(), include_str!("snapshots/hello.hir"));
}

#[test]
fn mutable_control_recovery_keeps_shapes_and_source_origins() {
    let source = "func f(){var x=0;while x<2 {x=x+1;if x==1 {continue}else{break}}} func next(){}";
    for end in 0..=source.len() {
        lower_source(&source[..end]);
    }
    let module = lower_source(source);
    assert!(module
        .nodes()
        .iter()
        .any(|n| matches!(n.kind, HirKind::Binding { mutable: true, .. })));
    for node in module.nodes().iter().filter(|n| {
        matches!(
            n.kind,
            HirKind::Assignment | HirKind::While | HirKind::Break | HirKind::Continue
        )
    }) {
        assert!(matches!(node.origin, SourceOrigin::Source(_)));
    }
}

#[test]
fn const_recovery_classification_and_conflicting_binding_flags() {
    let source = "func f(){const answer:int=1+2;const name=\"x\";while false {const u=()}}";
    for end in 0..=source.len() {
        lower_source(&source[..end]);
    }
    let module = lower_source(source);
    assert_eq!(
        module
            .nodes()
            .iter()
            .filter(|n| matches!(
                n.kind,
                HirKind::Binding {
                    constant: true,
                    mutable: false,
                    ..
                }
            ))
            .count(),
        3
    );
    let mut sources = SourceDatabase::default();
    let file = sources.add("api.nova", "x".into()).unwrap();
    let span = Span::new(file, 0, 1).unwrap();
    let mut arena = Arena::default();
    let value = arena.insert(NodeKind::Name, span, vec![]).unwrap();
    let wrong = arena
        .insert(
            NodeKind::Binding {
                name: span,
                has_type: false,
                mutable: true,
                constant: true,
            },
            span,
            vec![value],
        )
        .unwrap();
    let block = arena.insert(NodeKind::Block, span, vec![wrong]).unwrap();
    let function = arena
        .insert(
            NodeKind::Function {
                name: span,
                parameters: 0,
                has_return_type: false,
            },
            span,
            vec![block],
        )
        .unwrap();
    let root = arena.insert(NodeKind::Root, span, vec![function]).unwrap();
    assert_eq!(
        lower(&sources, &arena, root),
        Err(LoweringError::MalformedAst)
    );
}

#[test]
fn malformed_assignment_and_loop_ast_shapes_are_rejected() {
    let mut sources = SourceDatabase::default();
    let file = sources.add("api.nova", "1".into()).unwrap();
    let span = Span::new(file, 0, 1).unwrap();
    for kind in [NodeKind::Assignment, NodeKind::While] {
        let mut arena = Arena::default();
        let value = arena.insert(NodeKind::Integer, span, vec![]).unwrap();
        let wrong = arena.insert(kind, span, vec![value, value]).unwrap();
        let block = arena.insert(NodeKind::Block, span, vec![wrong]).unwrap();
        let function = arena
            .insert(
                NodeKind::Function {
                    name: span,
                    parameters: 0,
                    has_return_type: false,
                },
                span,
                vec![block],
            )
            .unwrap();
        let root = arena.insert(NodeKind::Root, span, vec![function]).unwrap();
        assert_eq!(
            lower(&sources, &arena, root),
            Err(LoweringError::MalformedAst)
        );
    }
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

#[test]
fn global_const_origins_and_truncated_recovery_are_valid() {
    let source = "const A:int=B+1\nfunc f(){const A=A+1}\nconst B=2";
    for end in 0..=source.len() {
        lower_source(&source[..end]);
    }
    let module = lower_source(source);
    assert!(!module.has_errors());
    let root = module.node(module.root()).unwrap();
    assert_eq!(root.children.len(), 3);
    for &id in &[root.children[0], root.children[2]] {
        assert!(matches!(
            module.node(id).unwrap().kind,
            HirKind::Binding {
                constant: true,
                mutable: false,
                ..
            }
        ));
        assert!(matches!(
            module.node(id).unwrap().origin,
            SourceOrigin::Source(_)
        ));
    }
}

#[test]
fn runtime_bindings_cannot_be_injected_at_ast_root() {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", "x".into()).unwrap();
    let span = Span::new(file, 0, 1).unwrap();
    for mutable in [false, true] {
        let mut arena = Arena::default();
        let value = arena.insert(NodeKind::Integer, span, vec![]).unwrap();
        let binding = arena
            .insert(
                NodeKind::Binding {
                    name: span,
                    has_type: false,
                    mutable,
                    constant: false,
                },
                span,
                vec![value],
            )
            .unwrap();
        let root = arena.insert(NodeKind::Root, span, vec![binding]).unwrap();
        assert_eq!(
            lower(&sources, &arena, root),
            Err(LoweringError::MalformedAst)
        );
    }
}
