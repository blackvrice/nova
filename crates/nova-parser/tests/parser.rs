use nova_ast::{AstNode, AstNodeId, NodeKind, Visitor};
use nova_lexer::{lex, normalize_ends, Lexed};
use nova_parser::{parse, parse_with_options, ParseInputError, Parsed, ParserOptions};
use nova_source::{FileId, SourceDatabase, SourceError};
use nova_syntax::{Symbol, TokenKind};

fn run(source: &str) -> (SourceDatabase, Lexed, Parsed) {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", source.into()).unwrap();
    let lexed = lex(&sources, file).unwrap();
    let result = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    assert_spans(&sources, &result);
    (sources, lexed, result)
}

fn assert_spans(sources: &SourceDatabase, parsed: &Parsed) {
    for (id, node) in parsed.arena.iter() {
        sources.slice(node.span).unwrap();
        for &child in &node.children {
            assert!(child.index() < id.index());
            let span = parsed.arena.get(child).unwrap().span;
            assert!(node.span.start() <= span.start() && span.end() <= node.span.end());
        }
        match node.kind {
            NodeKind::Function { name, .. }
            | NodeKind::Parameter { name }
            | NodeKind::Binding { name, .. } => {
                sources.slice(name).unwrap();
            }
            _ => {}
        }
    }
    for diagnostic in &parsed.diagnostics {
        sources.slice(diagnostic.primary.span).unwrap();
        for secondary in &diagnostic.secondary {
            sources.slice(secondary.span).unwrap();
        }
    }
    for token in &parsed.synthetic_tokens {
        assert_eq!(token.span.start(), token.span.end());
        sources.slice(token.span).unwrap();
    }
}

#[test]
fn p08_character_leaves_and_recovery_preserve_exact_spans() {
    let (sources, lexed, parsed) =
        run("func f(x:char)->char{let a='가';let b='{';return '\\u{1F642}'}");
    assert!(!lexed.has_errors() && !parsed.has_errors());
    let leaves = parsed
        .arena
        .iter()
        .filter(|(_, n)| n.kind == NodeKind::Character)
        .map(|(_, n)| {
            assert!(n.children.is_empty());
            sources.slice(n.span).unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(leaves, ["'가'", "'{'", "'\\u{1F642}'"]);
    for literal in ["''", "'ab'", "'\\u{D800}'"] {
        let (sources, lexed, parsed) =
            run(&format!("func f(){{let a={literal}}} func later(){{}}"));
        assert!(lexed.has_errors());
        assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
        assert!(parsed.arena.iter().any(|(_, n)| matches!(n.kind, NodeKind::Function { name, .. } if sources.slice(name).unwrap() == "later")));
    }
    let source = "func f(){let x='🙂'} func later(){}";
    for end in (0..=source.len()).filter(|&i| source.is_char_boundary(i)) {
        run(&source[..end]);
    }
    assert_pass(include_str!("../../../examples/characters.nova"));
}

fn assert_pass(source: &str) -> Parsed {
    let (_, lexed, parsed) = run(source);
    assert!(!lexed.has_errors(), "{:?}", lexed.diagnostics);
    assert!(
        !parsed.has_errors(),
        "{:?}\n{}",
        parsed.diagnostics,
        parsed.arena.dump()
    );
    assert!(parsed.synthetic_tokens.is_empty());
    parsed
}

#[test]
fn hello_and_stage_a_function_fixtures() {
    let parsed = assert_pass(include_str!("fixtures/pass/hello.nova"));
    assert_eq!(parsed.arena.dump(), include_str!("snapshots/hello.ast"));
    assert_eq!(
        parsed.arena.iter().map(|(_, n)| n.kind).collect::<Vec<_>>(),
        [
            NodeKind::Name,
            NodeKind::String,
            NodeKind::Call,
            NodeKind::ExpressionStatement,
            NodeKind::Block,
            NodeKind::Function {
                name: nova_source::Span::new(FileId::from_raw(0), 5, 9).unwrap(),
                parameters: 0,
                has_return_type: false
            },
            NodeKind::Root
        ]
    );
    assert_pass(include_str!("fixtures/pass/functions.nova"));
}

#[test]
fn expression_precedence_associativity_and_call_binding() {
    let parsed = assert_pass(
        "func main() { let x=1+2*3-4; let y=-f(1)(2); if x<3 && true || false { return () } }",
    );
    let binding = parsed
        .arena
        .iter()
        .find(|(_, n)| matches!(n.kind, NodeKind::Binding { .. }))
        .unwrap()
        .1;
    let subtraction = parsed.arena.get(*binding.children.last().unwrap()).unwrap();
    assert_eq!(subtraction.kind, NodeKind::Binary(Symbol::Minus));
    let addition = parsed.arena.get(subtraction.children[0]).unwrap();
    assert_eq!(addition.kind, NodeKind::Binary(Symbol::Plus));
    assert_eq!(
        parsed.arena.get(addition.children[1]).unwrap().kind,
        NodeKind::Binary(Symbol::Star)
    );
    let prefix = parsed
        .arena
        .iter()
        .find(|(_, n)| n.kind == NodeKind::Prefix(Symbol::Minus))
        .unwrap()
        .1;
    let outer_call = parsed.arena.get(prefix.children[0]).unwrap();
    assert_eq!(outer_call.kind, NodeKind::Call);
    assert_eq!(
        parsed.arena.get(outer_call.children[0]).unwrap().kind,
        NodeKind::Call
    );
    let condition = parsed
        .arena
        .iter()
        .find(|(_, n)| n.kind == NodeKind::If)
        .unwrap()
        .1;
    let or = parsed.arena.get(condition.children[0]).unwrap();
    assert_eq!(or.kind, NodeKind::Binary(Symbol::OrOr));
    assert_eq!(
        parsed.arena.get(or.children[0]).unwrap().kind,
        NodeKind::Binary(Symbol::AndAnd)
    );
}

fn shape(parsed: &Parsed, sources: &SourceDatabase) -> String {
    let mut output = String::new();
    for (_, node) in parsed.arena.iter() {
        let kind = match node.kind {
            NodeKind::Function {
                name,
                parameters,
                has_return_type,
            } => format!(
                "Function {} {parameters} {has_return_type}",
                sources.slice(name).unwrap()
            ),
            NodeKind::Parameter { name } => format!("Parameter {}", sources.slice(name).unwrap()),
            NodeKind::Binding { name, has_type, .. } => {
                format!("Binding {} {has_type}", sources.slice(name).unwrap())
            }
            kind => format!("{kind:?}"),
        };
        output.push_str(&format!("{kind} {:?}\n", node.children));
    }
    output
}

#[test]
fn newline_semicolon_and_boundary_have_the_same_syntax_shape() {
    let (left_sources, _, left) = run("func main() {\n let x=1\n print(x)\n return\n}\n");
    let (right_sources, _, right) = run("func main() { let x=1; print(x); return }");
    assert!(!left.has_errors());
    assert!(!right.has_errors());
    assert_eq!(shape(&left, &left_sources), shape(&right, &right_sources));
    let parsed = assert_pass("func main() { return\nprint(1) }");
    assert!(parsed
        .arena
        .iter()
        .any(|(_, n)| n.kind == NodeKind::Return && n.children.is_empty()));
}

#[test]
fn grouping_unit_simple_types_and_unknown_type_spelling() {
    let parsed = assert_pass("func f(x: Unknown,) -> () { let y: void=(); let z=(1+2); return y }");
    assert!(parsed
        .arena
        .iter()
        .any(|(_, n)| n.kind == NodeKind::UnitType));
    assert!(parsed.arena.iter().any(|(_, n)| n.kind == NodeKind::Group));
    // Parser does not decide whether Unknown exists or whether void binding is well typed.
}

#[test]
fn if_else_and_nested_string_interpolation() {
    let parsed = assert_pass("func main() { if true { print(\"a {f(\"b {1+2}\")} z\") } else if false { return } else { print(1) } }");
    assert_eq!(
        parsed
            .arena
            .iter()
            .filter(|(_, n)| n.kind == NodeKind::If)
            .count(),
        2
    );
    assert_eq!(
        parsed
            .arena
            .iter()
            .filter(|(_, n)| n.kind == NodeKind::InterpolatedString)
            .count(),
        2
    );
    assert_eq!(
        parsed
            .arena
            .iter()
            .filter(|(_, n)| n.kind == NodeKind::Interpolation)
            .count(),
        2
    );
}

#[test]
fn unicode_crlf_spans_and_source_anchors() {
    let source = "func 이름(값: int)\r\n{\r\nlet 결과=값+1\r\nprint(\"🙂 {결과}\")\r\n}\r\n";
    let (sources, lexed, parsed) = run(source);
    assert!(!lexed.has_errors());
    assert!(!parsed.has_errors());
    let function = parsed
        .arena
        .iter()
        .find_map(|(_, n)| {
            if let NodeKind::Function { name, .. } = n.kind {
                Some(name)
            } else {
                None
            }
        })
        .unwrap();
    assert_eq!(sources.slice(function).unwrap(), "이름");
    assert_eq!(function.start(), 5);
    let reconstructed = lexed
        .tokens
        .iter()
        .map(|t| sources.slice(t.span).unwrap())
        .collect::<String>();
    assert_eq!(reconstructed, source);
}

#[test]
fn malformed_fixture_codes_and_exact_comparison_span() {
    let source = include_str!("fixtures/fail/chained.nova");
    let (_, _, parsed) = run(source);
    let diagnostic = parsed
        .diagnostics
        .iter()
        .find(|d| d.code.to_string() == "N1103")
        .unwrap();
    assert_eq!(diagnostic.primary.span.start(), source.rfind('<').unwrap());
    assert_eq!(
        diagnostic.primary.span.end(),
        source.rfind('<').unwrap() + 1
    );
    assert_eq!(
        diagnostic.secondary[0].span.start(),
        source.find('<').unwrap()
    );
    let (_, _, parsed) = run(include_str!("fixtures/fail/initializer.nova"));
    assert!(parsed.has_errors());
    assert!(parsed
        .diagnostics
        .iter()
        .any(|d| d.code.to_string() == "N1101"));
    assert!(parsed.arena.iter().any(|(_, n)| n.kind == NodeKind::Call));
}

#[test]
fn incomplete_parameter_and_missing_delimiter_are_synthetic() {
    for source in [
        "func f(x) {}",
        "func main( { print(1) }",
        "func main() { print(1",
        "func main()",
    ] {
        let (_, _, parsed) = run(source);
        assert!(parsed.has_errors(), "{source}");
        assert!(!parsed.synthetic_tokens.is_empty(), "{source}");
        assert!(parsed
            .diagnostics
            .iter()
            .any(|d| d.code.to_string() == "N1101"));
    }
    let (_, _, parsed) = run("func main() { print(1");
    assert!(parsed.diagnostics.iter().any(|d| !d.secondary.is_empty()));
}

#[test]
fn missing_brace_preserves_the_next_function() {
    let (sources, _, parsed) = run("func broken() { let x=1\nfunc later() { print(2) }");
    let names = parsed
        .arena
        .iter()
        .filter_map(|(_, node)| match node.kind {
            NodeKind::Function { name, .. } => Some(sources.slice(name).unwrap()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(names, ["broken", "later"]);
    assert!(parsed
        .synthetic_tokens
        .iter()
        .any(|t| t.kind == TokenKind::RightBrace));
}

#[test]
fn unsupported_stage_features_are_not_silently_accepted() {
    for body in [
        "struct X {}",
        "let x=1.5",
        "let x=none",
        "let x=[1,2]",
        "let x=lambda () => 1",
        "let x=f(name: 1)",
        "let x=f<int>(1)",
        "(x)=1",
        "f().member",
        "let x=1 as int",
        "for x in xs { print(1) }",
    ] {
        let source = format!("func main() {{ {body}; print(2) }}\nfunc later() {{ print(3) }}");
        let (sources, _, parsed) = run(&source);
        assert!(parsed.has_errors(), "{body}");
        assert!(
            parsed
                .diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N1102" | "N1103")),
            "{body}"
        );
        assert!(parsed.arena.iter().any(|(_, n)| matches!(n.kind, NodeKind::Function { name, .. } if sources.slice(name).unwrap() == "later")), "{body}");
    }
    for source in [
        "use std",
        "let x=1",
        "func f<T>() {}",
        "func f(x: int=1) {}",
        "func f(take x: int) {}",
        "func f(x: Array<int>) {}",
    ] {
        let (_, _, parsed) = run(source);
        assert!(
            parsed
                .diagnostics
                .iter()
                .any(|d| d.code.to_string() == "N1102"),
            "{source}"
        );
    }
}

#[test]
fn mutable_binding_assignment_and_nested_loops_have_source_order() {
    let (sources, lexed, parsed) = run(include_str!("../../../examples/loops.nova"));
    assert!(!lexed.has_errors() && !parsed.has_errors());
    let bindings = parsed
        .arena
        .iter()
        .filter(|(_, n)| matches!(n.kind, NodeKind::Binding { mutable: true, .. }))
        .count();
    assert_eq!(bindings, 2);
    let assignments = parsed
        .arena
        .iter()
        .filter(|(_, n)| n.kind == NodeKind::Assignment)
        .map(|(_, n)| {
            sources
                .slice(parsed.arena.get(n.children[0]).unwrap().span)
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert_eq!(assignments, ["index", "sum"]);
    assert_pass("func f(){var x:()=();while true {while false {break} if true {continue} x=()}}");
}

#[test]
fn local_const_bindings_keep_declaration_spans_and_classification() {
    let parsed = assert_pass(include_str!("../../../examples/constants.nova"));
    assert_eq!(
        parsed
            .arena
            .iter()
            .filter(|(_, n)| matches!(
                n.kind,
                NodeKind::Binding {
                    constant: true,
                    mutable: false,
                    ..
                }
            ))
            .count(),
        3
    );
    assert_pass("func f(){const unit:()=();if true {const x=1} while false {const x=2}}");
    let (a_sources, _, a) = run("func f(){\nconst x=1\nconst y=x+1\n}");
    let (b_sources, _, b) = run("func f(){const x=1;const y=x+1}");
    assert!(!a.has_errors() && !b.has_errors());
    assert_eq!(shape(&a, &a_sources), shape(&b, &b_sources));
}

#[test]
fn const_initializers_are_required_and_global_var_is_unsupported() {
    for (source, code) in [
        ("var x=1\nfunc later(){}", "N1102"),
        ("func f(){const x}\nfunc later(){}", "N1101"),
    ] {
        let (sources, _, parsed) = run(source);
        assert!(parsed
            .diagnostics
            .iter()
            .any(|d| d.code.to_string() == code));
        assert!(parsed.arena.iter().any(|(_, n)| matches!(n.kind,
            NodeKind::Function { name, .. } if sources.slice(name).unwrap() == "later")));
    }
}

#[test]
fn global_const_root_items_end_forms_and_annotation_are_preserved() {
    let parsed = assert_pass(include_str!("../../../examples/global_constants.nova"));
    let root = parsed.arena.get(parsed.root).unwrap();
    assert_eq!(root.children.len(), 5);
    assert!(matches!(
        parsed.arena.get(root.children[0]).unwrap().kind,
        NodeKind::Binding {
            constant: true,
            mutable: false,
            has_type: true,
            ..
        }
    ));
    let (a_sources, _, a) = run("const A=1\nfunc f(){}\nconst B:()=()\n");
    let (b_sources, _, b) = run("const A=1;func f(){}const B:()=()");
    assert!(!a.has_errors() && !b.has_errors());
    assert_eq!(shape(&a, &a_sources), shape(&b, &b_sources));
}

#[test]
fn malformed_global_const_recovers_at_next_declaration() {
    for source in [
        "const A\nconst B=2\nfunc later(){}",
        "const A=\nconst B=2\nfunc later(){}",
        "const A=1 const B=2\nfunc later(){}",
    ] {
        let (sources, _, parsed) = run(source);
        assert!(parsed.has_errors());
        assert!(
            parsed.arena.iter().any(|(_, n)| matches!(n.kind,
            NodeKind::Binding { name, constant: true, .. } if sources.slice(name).unwrap() == "B"))
        );
        assert!(parsed.arena.iter().any(|(_, n)| matches!(n.kind,
            NodeKind::Function { name, .. } if sources.slice(name).unwrap() == "later")));
    }
}

#[test]
fn loop_end_forms_and_jump_newlines_preserve_statement_shapes() {
    let (a_sources, _, a) = run("func f(){\nvar x=0\nwhile true\n{\nx=x+1\nbreak\nprint(x)\n}\n}");
    let (b_sources, _, b) = run("func f(){var x=0;while true {x=x+1;break;print(x)}}");
    assert!(!a.has_errors() && !b.has_errors());
    assert_eq!(shape(&a, &a_sources), shape(&b, &b_sources));
    assert_pass("func f(){while true {continue\nprint(1)}}");
}

#[test]
fn unsupported_assignment_and_jump_forms_preserve_later_functions() {
    for body in [
        "(x)=1",
        "f()=1",
        "x.member=1",
        "x[0]=1",
        "x= y=1",
        "x+=1",
        "f(x=1)",
        "break 1",
        "continue label",
        "var x",
    ] {
        let source = format!("func f(){{{body}}} func later(){{}}");
        let (sources, _, parsed) = run(&source);
        assert!(parsed.has_errors(), "{body}");
        assert!(
            parsed
                .diagnostics
                .iter()
                .any(|d| d.code.to_string() == if body == "var x" { "N1101" } else { "N1102" }),
            "{body}: {:?}",
            parsed.diagnostics
        );
        assert!(parsed.arena.iter().any(|(_, n)| matches!(n.kind,
            NodeKind::Function { name, .. } if sources.slice(name).unwrap() == "later")));
    }
}

#[test]
fn truncated_and_deep_loop_sources_recover_with_bounded_nesting() {
    let source = "func f(){var x=0;while x<2 {x=x+1;if x==1 {continue}else{break}}} func later(){}";
    for end in 0..=source.len() {
        let (_, _, first) = run(&source[..end]);
        let (_, _, second) = run(&source[..end]);
        assert_eq!(first, second);
    }
    let source = format!(
        "func f(){{{}break{}}} func later(){{}}",
        "while true {".repeat(200),
        "}".repeat(200)
    );
    let (sources, _, parsed) = run(&source);
    assert!(parsed
        .diagnostics
        .iter()
        .any(|d| d.code.to_string() == "N8901"));
    assert!(parsed.arena.iter().any(|(_, n)| matches!(n.kind,
        NodeKind::Function { name, .. } if sources.slice(name).unwrap() == "later")));
}

#[derive(Default)]
struct Counter {
    visited: usize,
    errors: usize,
}
impl Visitor for Counter {
    fn visit(&mut self, _: AstNodeId, node: &AstNode) {
        self.visited += 1;
        self.errors += usize::from(node.kind == NodeKind::Error);
    }
}

#[test]
fn lexer_errors_are_preserved_without_duplicate_parser_diagnostics() {
    let (_, lexed, parsed) = run("func main() { let x=🙂 }");
    assert_eq!(lexed.diagnostics.len(), 1);
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.has_errors());
    let mut counter = Counter::default();
    parsed.arena.walk(parsed.root, &mut counter).unwrap();
    assert_eq!(counter.visited, parsed.arena.iter().len());
    assert_eq!(counter.errors, 1);
}

#[test]
fn invalid_stream_and_options_return_errors() {
    let mut sources = SourceDatabase::default();
    let file = sources.add("empty.nova", "".into()).unwrap();
    assert_eq!(
        parse(&sources, file, &[]),
        Err(ParseInputError::InvalidTokenStream)
    );
    assert_eq!(
        parse(&sources, FileId::from_raw(99), &[]),
        Err(ParseInputError::Source(SourceError::UnknownFile))
    );
    let lexed = lex(&sources, file).unwrap();
    for max_nesting in [0, 129, usize::MAX] {
        assert_eq!(
            parse_with_options(&sources, file, &lexed.tokens, ParserOptions { max_nesting }),
            Err(ParseInputError::InvalidOptions)
        );
    }
    let file = sources.add("bad.nova", "func main() {}\n".into()).unwrap();
    let raw = lex(&sources, file).unwrap();
    assert_eq!(
        parse(&sources, file, &raw.tokens),
        Err(ParseInputError::InvalidTokenStream)
    );
    let mut tokens = normalize_ends(&raw.tokens);
    tokens.swap(0, 1);
    assert_eq!(
        parse(&sources, file, &tokens),
        Err(ParseInputError::InvalidTokenStream)
    );
}

#[test]
fn configured_limit_and_interpolation_depth_are_checked() {
    let source = "func main() { let x=-(1) }";
    let mut sources = SourceDatabase::default();
    let file = sources.add("limit.nova", source.into()).unwrap();
    let raw = lex(&sources, file).unwrap();
    let parsed = parse_with_options(
        &sources,
        file,
        &normalize_ends(&raw.tokens),
        ParserOptions { max_nesting: 2 },
    )
    .unwrap();
    assert!(parsed
        .diagnostics
        .iter()
        .any(|d| d.notes.iter().any(|n| n.contains("2"))));
    assert_spans(&sources, &parsed);
    let source = "func main() { print(".to_owned()
        + &"\"{".repeat(2_000)
        + "value"
        + &"}\"".repeat(2_000)
        + ") }";
    let (_, _, parsed) = run(&source);
    assert!(parsed
        .diagnostics
        .iter()
        .any(|d| d.message.contains("nesting limit")));
}

#[test]
fn empty_and_all_truncated_prefixes_are_deterministic() {
    assert_pass("");
    let source = include_str!("fixtures/pass/functions.nova");
    for end in 0..=source.len() {
        let (_, _, first) = run(&source[..end]);
        let (_, _, second) = run(&source[..end]);
        assert_eq!(first, second, "prefix {end}");
    }
}

#[test]
fn deep_nesting_limits_and_large_flat_input_do_not_overflow() {
    for source in [
        format!(
            "func main() {{ let x={}1{} }}",
            "(".repeat(10_000),
            ")".repeat(10_000)
        ),
        format!("func main() {{ let x={}1 }}", "-".repeat(10_000)),
        format!(
            "func main() {{ {}return{} }}",
            "if true {".repeat(2_000),
            "}".repeat(2_000)
        ),
    ] {
        let (_, _, parsed) = run(&source);
        assert!(parsed
            .diagnostics
            .iter()
            .any(|d| d.message.contains("nesting limit")));
    }
    let source = format!("func main() {{ {} }}", "print(1);".repeat(10_000));
    assert_pass(&source);
}

#[test]
fn adversarial_token_sequences_always_make_progress() {
    let alphabet = [
        "func",
        "let",
        "if",
        "else",
        "return",
        "x",
        "int",
        "0",
        "true",
        "{",
        "}",
        "(",
        ")",
        ",",
        ":",
        ";",
        "\n",
        "=",
        "+",
        "<",
        "->",
        "\"x {y}\"",
        "🙂",
        "while",
        "lambda",
    ];
    let mut seed = 0x9418_57abu32;
    for _ in 0..1_000 {
        let mut source = String::from("func main() { ");
        for _ in 0..60 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            source.push_str(alphabet[seed as usize % alphabet.len()]);
            source.push(' ');
        }
        source.push_str("}\nfunc later() {}\n");
        let (_, _, first) = run(&source);
        let (_, _, second) = run(&source);
        assert_eq!(first, second);
    }
}
