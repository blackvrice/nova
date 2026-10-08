use nova_ast::{Arena, NodeKind};

#[test]
fn p11_public_ast_import_keyword_alias_and_path_spelling_are_validated() {
    for (text, first, second, keyword, alias) in [
        ("use ..::x", (4, 6), (8, 9), (0, 3), None),
        ("use lib::x", (4, 7), (9, 10), (4, 7), None),
        ("use lib::x as y", (4, 7), (9, 10), (0, 3), Some((4, 7))),
        ("use lib::x as !", (4, 7), (9, 10), (0, 3), Some((14, 15))),
    ] {
        let mut sources = SourceDatabase::default();
        let file = sources.add("api.nova", text.into()).unwrap();
        let span = |range: (usize, usize)| Span::new(file, range.0, range.1).unwrap();
        let whole = span((0, text.len()));
        let mut arena = Arena::default();
        let a = arena
            .insert(NodeKind::ImportSegment, span(first), vec![])
            .unwrap();
        let b = arena
            .insert(NodeKind::ImportSegment, span(second), vec![])
            .unwrap();
        let import = arena
            .insert(
                NodeKind::Import {
                    keyword: span(keyword),
                    alias: alias.map(span),
                },
                whole,
                vec![a, b],
            )
            .unwrap();
        let root = arena.insert(NodeKind::Root, whole, vec![import]).unwrap();
        assert_eq!(
            lower(&sources, &arena, root),
            Err(LoweringError::MalformedAst),
            "{text}"
        );
    }
}

#[test]
fn p11_bundles_reject_duplicate_files_paths_and_empty_input() {
    assert_eq!(
        nova_hir::Module::bundle(vec![]),
        Err(LoweringError::MalformedAst)
    );
    assert_eq!(
        nova_hir::Module::bundle(vec![
            ("a".into(), lower_source("func a(){}")),
            ("b".into(), lower_source("func b(){}"))
        ]),
        Err(LoweringError::MalformedAst)
    );
    let mut sources = SourceDatabase::default();
    let mut inputs = vec![];
    for source in ["func a(){}", "func b(){}"] {
        let file = sources.add("api.nova", source.into()).unwrap();
        let lexed = lex(&sources, file).unwrap();
        let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
        inputs.push((
            "same".into(),
            lower(&sources, &parsed.arena, parsed.root).unwrap(),
        ));
    }
    assert_eq!(
        nova_hir::Module::bundle(inputs),
        Err(LoweringError::MalformedAst)
    );
    for paths in [
        ["main", "lib", "LIB"],
        ["main", "z", "a"],
        ["main", "bad/path", "z"],
    ] {
        let mut sources = SourceDatabase::default();
        let mut inputs = vec![];
        for path in paths {
            let file = sources.add("api.nova", "func f(){}".into()).unwrap();
            let lexed = lex(&sources, file).unwrap();
            let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
            inputs.push((
                path.into(),
                lower(&sources, &parsed.arena, parsed.root).unwrap(),
            ));
        }
        assert_eq!(
            nova_hir::Module::bundle(inputs),
            Err(LoweringError::MalformedAst)
        );
    }
}

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
    let first = lower(&sources, &parsed.arena, parsed.root)
        .unwrap_or_else(|e| panic!("{source:?}: {e:?}; {:?}", parsed.arena));
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

#[test]
fn p10_cast_lowering_preserves_alias_syntax_origin_and_rejects_forged_keyword() {
    let hir = lower_source("func f(){let x=1 as double}");
    let node = hir
        .nodes()
        .iter()
        .find(|n| matches!(n.kind, HirKind::Cast { .. }))
        .unwrap();
    let HirKind::TypeName(name) = hir.node(node.children[1]).unwrap().kind else {
        panic!("type child")
    };
    assert_eq!(hir.symbol(name), Some("float64"));
    assert!(matches!(node.origin, SourceOrigin::Source(_)));
    for (text, start, end) in [
        ("1 as int", 2, 4),
        ("1 xx int", 2, 4),
        ("1 as int", 0, 4),
        ("1 as int", 2, 8),
    ] {
        let mut sources = SourceDatabase::default();
        let file = sources.add("api.nova", text.into()).unwrap();
        let sp = |a, b| Span::new(file, a, b).unwrap();
        let mut arena = Arena::default();
        let value = arena.insert(NodeKind::Integer, sp(0, 1), vec![]).unwrap();
        let target = arena.insert(NodeKind::NamedType, sp(5, 8), vec![]).unwrap();
        let cast = arena
            .insert(
                NodeKind::Cast {
                    keyword: sp(start, end),
                },
                sp(0, 8),
                vec![value, target],
            )
            .unwrap();
        let error = arena.insert(NodeKind::Error, sp(0, 8), vec![cast]).unwrap();
        let root = arena.insert(NodeKind::Root, sp(0, 8), vec![error]).unwrap();
        let result = lower(&sources, &arena, root);
        if text == "1 as int" && start == 2 && end == 4 {
            assert!(result.is_ok());
        } else {
            assert_eq!(result, Err(LoweringError::MalformedAst));
        }
    }
}

#[test]
fn p13_public_ast_numeric_selector_subspans_are_validated() {
    for digits in ["01", "0_1", "0x1", "0.1", "0"] {
        let text = format!("func f(){{t.{digits}}}");
        let mut sources = SourceDatabase::default();
        let file = sources.add("api.nova", text.clone()).unwrap();
        let span = |start, end| Span::new(file, start, end).unwrap();
        let mut arena = Arena::default();
        let receiver = arena.insert(NodeKind::Name, span(9, 10), vec![]).unwrap();
        let projection = arena
            .insert(
                NodeKind::TupleProjection {
                    index: span(11, 11 + digits.len()),
                },
                span(9, 11 + digits.len()),
                vec![receiver],
            )
            .unwrap();
        let stmt = arena
            .insert(
                NodeKind::ExpressionStatement,
                span(9, 11 + digits.len()),
                vec![projection],
            )
            .unwrap();
        let body = arena
            .insert(NodeKind::Block, span(8, text.len()), vec![stmt])
            .unwrap();
        let function = arena
            .insert(
                NodeKind::Function {
                    name: span(5, 6),
                    parameters: 0,
                    has_return_type: false,
                },
                span(0, text.len()),
                vec![body],
            )
            .unwrap();
        let root = arena
            .insert(NodeKind::Root, span(0, text.len()), vec![function])
            .unwrap();
        if digits == "0" {
            assert!(lower(&sources, &arena, root).is_ok());
        } else {
            assert_eq!(
                lower(&sources, &arena, root),
                Err(LoweringError::MalformedAst)
            );
        }
    }
}

#[test]
fn p14_public_ast_pattern_metadata_cannot_bypass_validation() {
    for case in 0..3 {
        let text = "E::A(x) true";
        let mut sources = SourceDatabase::default();
        let file = sources.add("api.nova", text.into()).unwrap();
        let span = |a, b| Span::new(file, a, b).unwrap();
        let mut arena = Arena::default();
        let binder = arena
            .insert(NodeKind::Binder { name: span(5, 6) }, span(5, 6), vec![])
            .unwrap();
        let (kind, range, children) = match case {
            0 => (
                NodeKind::PatternVariant {
                    owner: span(0, 1),
                    name: span(3, 4),
                    arguments: false,
                },
                span(0, 7),
                vec![binder],
            ),
            1 => (NodeKind::PatternBoolean(false), span(8, 12), vec![]),
            _ => (
                NodeKind::PatternVariant {
                    owner: span(0, 1),
                    name: span(2, 4),
                    arguments: true,
                },
                span(0, 7),
                vec![binder],
            ),
        };
        let pattern = arena.insert(kind, range, children).unwrap();
        let error = arena
            .insert(NodeKind::Error, span(0, text.len()), vec![pattern])
            .unwrap();
        let root = arena
            .insert(NodeKind::Root, span(0, text.len()), vec![error])
            .unwrap();
        assert_eq!(
            lower(&sources, &arena, root),
            Err(LoweringError::MalformedAst)
        );
    }
    let source = "enum E{A(int);B;}func f(){match E::A(1){E::A(x)=>{print(\"🙂{x}\")},E::B=>{}}}";
    for at in (0..=source.len()).filter(|&at| source.is_char_boundary(at)) {
        lower_source(&source[..at]);
    }
}

#[test]
fn p15_ast_type_and_none_certificates_reject_forged_spelling() {
    for (text, kind, children) in [
        ("null", NodeKind::None, false),
        ("none", NodeKind::PatternNone, true),
        ("int!", NodeKind::NullableType, true),
        (
            "int<>",
            NodeKind::GenericType {
                name: Span::new(nova_source::FileId::from_raw(0), 0, 3).unwrap(),
            },
            true,
        ),
    ] {
        let mut db = SourceDatabase::default();
        let file = db.add("api", text.into()).unwrap();
        let mut a = Arena::default();
        let span = |x, y| Span::new(file, x, y).unwrap();
        let c = a.insert(NodeKind::NamedType, span(0, 3), vec![]).unwrap();
        let n = a
            .insert(
                kind,
                span(0, text.len()),
                if children { vec![c] } else { vec![] },
            )
            .unwrap();
        let e = a
            .insert(NodeKind::Error, span(0, text.len()), vec![n])
            .unwrap();
        let r = a
            .insert(NodeKind::Root, span(0, text.len()), vec![e])
            .unwrap();
        assert_eq!(lower(&db, &a, r), Err(LoweringError::MalformedAst));
    }
    let text = "func f(x:Result<Option<int>,bool,>){let y:int??=Option::Some(none)}";
    for at in (0..=text.len()).filter(|&at| text.is_char_boundary(at)) {
        lower_source(&text[..at]);
    }
}

#[test]
fn p16_try_public_ast_shape_spelling_and_keyword_boundaries_are_checked() {
    for (text, keyword, child, has_child) in [
        ("try 1", (0, 3), (4, 5), true),
        ("tryfoo", (0, 3), (3, 6), true),
        ("foo 1", (0, 3), (4, 5), true),
        ("try 1", (1, 3), (4, 5), true),
        ("try 1", (0, 3), (0, 3), true),
        ("try 1", (0, 3), (4, 5), false),
    ] {
        let mut db = SourceDatabase::default();
        let file = db.add("api", text.into()).unwrap();
        let span = |r: (usize, usize)| Span::new(file, r.0, r.1).unwrap();
        let whole = span((0, text.len()));
        let mut arena = Arena::default();
        let operand_kind = if text == "tryfoo" {
            NodeKind::Name
        } else {
            NodeKind::Integer
        };
        let operand = arena.insert(operand_kind, span(child), vec![]).unwrap();
        let t = arena
            .insert(
                NodeKind::Try {
                    keyword: span(keyword),
                },
                whole,
                if has_child { vec![operand] } else { vec![] },
            )
            .unwrap();
        let error = arena.insert(NodeKind::Error, whole, vec![t]).unwrap();
        let root = arena.insert(NodeKind::Root, whole, vec![error]).unwrap();
        if text == "try 1" && keyword == (0, 3) && child == (4, 5) && has_child {
            assert!(lower(&db, &arena, root).is_ok());
        } else {
            assert_eq!(
                lower(&db, &arena, root),
                Err(LoweringError::MalformedAst),
                "{text}"
            );
        }
    }
    let source="// 🙂\nfunc f(r:Result<int,bool>)->Result<int,bool>{let x=try try r;return Result::Success(x)}";
    for at in (0..=source.len()).filter(|&at| source.is_char_boundary(at)) {
        lower_source(&source[..at]);
    }
}

#[test]
fn p17_named_argument_source_metadata_and_external_ast_are_checked() {
    let source = "func f(){g(뒤 /*x*/ : 1,앞:2)}";
    let hir = lower_source(source);
    let names = hir
        .nodes()
        .iter()
        .filter_map(|n| match n.kind {
            HirKind::NamedArgument { name, .. } => Some(hir.symbol(name).unwrap()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(names, ["뒤", "앞"]);
    for at in (0..=source.len()).filter(|at| source.is_char_boundary(*at)) {
        lower_source(&source[..at]);
    }
    for (text, name, colon, child, count, valid) in [
        ("x:1", (0, 1), (1, 2), (2, 3), 1, true),
        ("x:1", (0, 1), (0, 1), (2, 3), 1, false),
        ("x:1", (1, 2), (1, 2), (2, 3), 1, false),
        ("xx:1", (0, 1), (2, 3), (3, 4), 1, false),
        ("x:1", (0, 1), (1, 2), (2, 3), 0, false),
        ("x:1", (0, 1), (1, 2), (2, 3), 2, false),
    ] {
        let mut db = SourceDatabase::default();
        let file = db.add("api", text.into()).unwrap();
        let span = |r: (usize, usize)| Span::new(file, r.0, r.1).unwrap();
        let whole = span((0, text.len()));
        let mut arena = Arena::default();
        let value = arena
            .insert(NodeKind::Integer, span(child), vec![])
            .unwrap();
        let arg = arena
            .insert(
                NodeKind::NamedArgument {
                    name: span(name),
                    colon: span(colon),
                },
                whole,
                vec![value; count],
            )
            .unwrap();
        let error = arena.insert(NodeKind::Error, whole, vec![arg]).unwrap();
        let root = arena.insert(NodeKind::Root, whole, vec![error]).unwrap();
        assert_eq!(lower(&db, &arena, root).is_ok(), valid, "{text}");
    }
}

#[test]
fn p18_default_source_metadata_and_external_ast_are_checked() {
    let source = "func f(앞:int8 /*ty*/ = /*v*/ 1,뒤:string=\"🙂\"){}";
    let hir = lower_source(source);
    assert_eq!(
        hir.nodes()
            .iter()
            .filter(|n| matches!(n.kind, HirKind::DefaultValue { .. }))
            .count(),
        2
    );
    for at in (0..=source.len()).filter(|at| source.is_char_boundary(*at)) {
        lower_source(&source[..at]);
    }
    for (text, equals, child, count, valid) in [
        ("x:int8=1", (6, 7), (7, 8), 1, true),
        ("x:int8+1", (6, 7), (7, 8), 1, false),
        ("x:int8x=1", (7, 8), (8, 9), 1, false),
        ("x:int8=1", (6, 7), (7, 8), 0, false),
        ("x:int8=1", (6, 7), (7, 8), 2, false),
        ("x:int8=1", (5, 6), (7, 8), 1, false),
    ] {
        let mut db = SourceDatabase::default();
        let file = db.add("api", text.into()).unwrap();
        let span = |r: (usize, usize)| Span::new(file, r.0, r.1).unwrap();
        let mut arena = Arena::default();
        let ty = arena
            .insert(NodeKind::NamedType, span((2, 6)), vec![])
            .unwrap();
        let value = arena
            .insert(NodeKind::Integer, span(child), vec![])
            .unwrap();
        let default = arena
            .insert(
                NodeKind::DefaultValue {
                    equals: span(equals),
                },
                span((equals.0, text.len())),
                vec![value; count],
            )
            .unwrap();
        let parameter = arena
            .insert(
                NodeKind::Parameter { name: span((0, 1)) },
                span((0, text.len())),
                vec![ty, default],
            )
            .unwrap();
        let error = arena
            .insert(NodeKind::Error, span((0, text.len())), vec![parameter])
            .unwrap();
        let root = arena
            .insert(NodeKind::Root, span((0, text.len())), vec![error])
            .unwrap();
        assert_eq!(lower(&db, &arena, root).is_ok(), valid, "{text}");
    }
}

#[test]
fn p19_range_loop_original_source_and_external_ast_forgery_gate() {
    let source = "func f(){for 値 in 0 through 1 {loop {break}}}";
    let hir = lower_source(source);
    assert_eq!(
        hir.nodes()
            .iter()
            .filter(|n| matches!(n.kind, HirKind::For { .. } | HirKind::Loop { .. }))
            .count(),
        2
    );
    for end in (0..=source.len()).filter(|&i| source.is_char_boundary(i)) {
        lower_source(&source[..end]);
    }
    let text = "for i in 0 through 1 {}";
    for mutation in 0..9 {
        let mut db = SourceDatabase::default();
        let file = db.add("api", text.into()).unwrap();
        let s = |a, b| Span::new(file, a, b).unwrap();
        let mut arena = Arena::default();
        let binder = arena
            .insert(NodeKind::Binder { name: s(4, 5) }, s(4, 5), vec![])
            .unwrap();
        let left = arena.insert(NodeKind::Integer, s(9, 10), vec![]).unwrap();
        let right = arena.insert(NodeKind::Integer, s(19, 20), vec![]).unwrap();
        let body = arena.insert(NodeKind::Block, s(21, 23), vec![]).unwrap();
        let mut children = vec![binder, left, right, body];
        let mut kind = NodeKind::For {
            keyword: s(0, 3),
            in_keyword: s(6, 8),
            operator: s(11, 18),
            inclusive: true,
            range: s(9, 20),
        };
        match mutation {
            1 => children.clear(),
            2 => children.swap(1, 2),
            3 => children[0] = left,
            4 => {
                if let NodeKind::For { inclusive, .. } = &mut kind {
                    *inclusive = false
                }
            }
            5 => {
                if let NodeKind::For { in_keyword, .. } = &mut kind {
                    *in_keyword = s(4, 5)
                }
            }
            6 => {
                if let NodeKind::For { range, .. } = &mut kind {
                    *range = s(9, 18)
                }
            }
            7 => children.push(body),
            8 => {
                if let NodeKind::For { keyword, .. } = &mut kind {
                    *keyword = s(1, 3)
                }
            }
            _ => {}
        }
        let node = arena.insert(kind, s(0, text.len()), children).unwrap();
        let error = arena
            .insert(NodeKind::Error, s(0, text.len()), vec![node])
            .unwrap();
        let root = arena
            .insert(NodeKind::Root, s(0, text.len()), vec![error])
            .unwrap();
        assert_eq!(
            lower(&db, &arena, root).is_ok(),
            mutation == 0,
            "{mutation}"
        );
    }
    let mut db = SourceDatabase::default();
    let file = db.add("api", "loop {}".into()).unwrap();
    let mut arena = Arena::default();
    let whole = Span::new(file, 0, 7).unwrap();
    let node = arena
        .insert(
            NodeKind::Loop {
                keyword: Span::new(file, 0, 4).unwrap(),
            },
            whole,
            vec![],
        )
        .unwrap();
    let error = arena.insert(NodeKind::Error, whole, vec![node]).unwrap();
    let root = arena.insert(NodeKind::Root, whole, vec![error]).unwrap();
    assert_eq!(lower(&db, &arena, root), Err(LoweringError::MalformedAst));
}

#[test]
fn p20_exists_source_origin_trivia_shape_and_external_ast_gate() {
    let source = "func f(){let 値=some /* outer /* inner */ */ exists}";
    let hir = lower_source(source);
    assert_eq!(
        hir.nodes()
            .iter()
            .filter(|n| matches!(n.kind, HirKind::Exists { .. }))
            .count(),
        1
    );
    for at in (0..=source.len()).filter(|at| source.is_char_boundary(*at)) {
        lower_source(&source[..at]);
    }
    for (text, operand, keyword, whole, count, valid) in [
        ("x exists", (0, 1), (2, 8), (0, 8), 1, true),
        ("()exists", (0, 2), (2, 8), (0, 8), 1, true),
        (
            "x /* nested /* ok */ */ exists",
            (0, 1),
            (24, 30),
            (0, 30),
            1,
            true,
        ),
        ("x exists", (0, 1), (2, 8), (0, 8), 0, false),
        ("x exists", (0, 1), (2, 8), (0, 8), 2, false),
        ("x exists", (0, 1), (3, 8), (0, 8), 1, false),
        ("x + exists", (0, 1), (4, 10), (0, 10), 1, false),
        ("x exists", (2, 8), (2, 8), (0, 8), 1, false),
        ("x exists ", (0, 1), (2, 8), (0, 9), 1, false),
        ("xexists", (0, 1), (1, 7), (0, 7), 1, false),
        ("x exists_extra", (0, 1), (2, 8), (0, 8), 1, false),
        ("x exists1", (0, 1), (2, 8), (0, 8), 1, false),
    ] {
        let mut db = SourceDatabase::default();
        let file = db.add("api", text.into()).unwrap();
        let span = |r: (usize, usize)| Span::new(file, r.0, r.1).unwrap();
        let mut arena = Arena::default();
        let value = arena
            .insert(
                if text.starts_with("()") {
                    NodeKind::Unit
                } else {
                    NodeKind::Name
                },
                span(operand),
                vec![],
            )
            .unwrap();
        let exists = arena
            .insert(
                NodeKind::Exists {
                    keyword: span(keyword),
                },
                span(whole),
                vec![value; count],
            )
            .unwrap();
        let error = arena
            .insert(NodeKind::Error, span((0, text.len())), vec![exists])
            .unwrap();
        let root = arena
            .insert(NodeKind::Root, span((0, text.len())), vec![error])
            .unwrap();
        assert_eq!(lower(&db, &arena, root).is_ok(), valid, "{text}");
    }
}
