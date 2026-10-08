use nova_lexer::{lex, normalize_ends, Lexed, UNICODE_VERSION};
use nova_source::{FileId, SourceDatabase, SourceError};
use nova_syntax::{EndOrigin, Keyword, Symbol, TokenKind};

fn scan(text: &str) -> (SourceDatabase, Lexed) {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", text.into()).unwrap();
    let result = lex(&sources, file).unwrap();
    (sources, result)
}

fn kinds(result: &Lexed) -> Vec<TokenKind> {
    result
        .tokens
        .iter()
        .filter(|token| !token.kind.is_trivia())
        .map(|token| token.kind)
        .collect()
}

#[test]
fn p08_character_boundaries_escapes_and_errors_preserve_d04() {
    for literal in [
        r"'\0'",
        r"'\n'",
        r"'\r'",
        r"'\t'",
        r"'\\'",
        r#"'\"'"#,
        r"'\''",
        "'{'",
        "'}'",
        "'가'",
        "'🙂'",
        r"'\u{7f}'",
        r"'\u{80}'",
        r"'\u{7ff}'",
        r"'\u{800}'",
        r"'\u{D7FF}'",
        r"'\u{E000}'",
        r"'\u{FFFF}'",
        r"'\u{10000}'",
        r"'\u{10FFFF}'",
        r"'\u{0378}'",
    ] {
        let (sources, result) = scan(literal);
        assert_lossless(literal, &sources, &result);
        assert!(!result.has_errors(), "{literal}: {:?}", result.diagnostics);
        assert_eq!(kinds(&result), [TokenKind::Character, TokenKind::Eof]);
        assert_eq!(sources.slice(result.tokens[0].span).unwrap(), literal);
        let normalized = normalize_ends(&scan(&format!("{literal}\nlet x=1")).1.tokens);
        assert!(normalized
            .iter()
            .any(|t| matches!(t.kind, TokenKind::End(_))));
    }
    for (literal, code) in [
        ("''", "N1002"),
        ("'ab'", "N1002"),
        ("'{{'", "N1002"),
        ("'e\u{301}'", "N1002"),
        (r"'\u{D800}'", "N1002"),
        (r"'\u{DFFF}'", "N1002"),
        (r"'\u{110000}'", "N1002"),
        (r"'\u{}'", "N1002"),
        (r"'\u{0000000}'", "N1002"),
        (r"'\q'", "N1002"),
        ("'\n'", "N1002"),
        ("'a", "N1003"),
    ] {
        let (sources, result) = scan(literal);
        assert_lossless(literal, &sources, &result);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.code.to_string() == code),
            "{literal}: {:?}",
            result.diagnostics
        );
        // D04 recovery stops before a raw newline and keeps the rest lossless.
        assert_eq!(
            sources.slice(result.tokens[0].span).unwrap(),
            if literal == "'\n'" { "'" } else { literal }
        );
        assert_eq!(result.tokens[0].kind, TokenKind::Error);
    }
}

fn assert_lossless(text: &str, sources: &SourceDatabase, result: &Lexed) {
    let mut cursor = 0;
    let mut reconstructed = String::new();
    for token in &result.tokens {
        assert_eq!(token.span.start(), cursor);
        assert!(text.is_char_boundary(token.span.start()));
        assert!(text.is_char_boundary(token.span.end()));
        reconstructed.push_str(sources.slice(token.span).unwrap());
        cursor = token.span.end();
    }
    assert_eq!(cursor, text.len());
    assert_eq!(reconstructed, text);
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|t| t.kind == TokenKind::Eof)
            .count(),
        1
    );
    for diagnostic in &result.diagnostics {
        assert!(sources.slice(diagnostic.primary.span).is_ok());
        for label in &diagnostic.secondary {
            assert!(sources.slice(label.span).is_ok());
        }
    }
}

#[test]
fn hello_nova_tokens() {
    let source = include_str!("fixtures/pass/hello.nova");
    let (sources, result) = scan(source);
    assert!(!result.has_errors());
    assert_lossless(source, &sources, &result);
    assert_eq!(
        kinds(&result),
        vec![
            TokenKind::Keyword(Keyword::Func),
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::LeftBrace,
            TokenKind::NewLine,
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::String,
            TokenKind::RightParen,
            TokenKind::NewLine,
            TokenKind::RightBrace,
            TokenKind::NewLine,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn unicode_snapshot_and_fixed_version() {
    assert_eq!(UNICODE_VERSION, (18, 0, 0));
    let (sources, result) = scan("let 이름 = \"🙂\"\r\n");
    assert_eq!(
        result.dump(&sources).unwrap(),
        include_str!("snapshots/unicode.tokens")
    );
    assert!(!result.has_errors());
}

#[test]
fn identifiers_keep_spelling_and_contextual_names() {
    let source = "\u{feff}let café = cafe\u{301}\rself c library int use type lambda";
    let (sources, result) = scan(source);
    assert!(!result.has_errors());
    assert_lossless(source, &sources, &result);
    let identifiers = result
        .tokens
        .iter()
        .filter(|t| t.kind == TokenKind::Identifier)
        .map(|t| sources.slice(t.span).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        identifiers,
        ["café", "cafe\u{301}", "self", "c", "library", "int"]
    );
    for word in ["use", "type", "lambda"] {
        assert!(result
            .tokens
            .iter()
            .any(|t| sources.slice(t.span).unwrap() == word
                && matches!(t.kind, TokenKind::Keyword(_))));
    }
}

#[test]
fn maximal_operator_matching_and_invalid_characters() {
    let source = "= == => - -> ! != < <= > >= && || : :: & | 🙂 \u{feff}";
    let (sources, result) = scan(source);
    assert_lossless(source, &sources, &result);
    assert_eq!(result.diagnostics.len(), 4);
    assert!(result
        .diagnostics
        .iter()
        .all(|d| d.code.to_string() == "N1001"));
    assert_eq!(
        &kinds(&result)[..3],
        &[
            TokenKind::Symbol(Symbol::Equal),
            TokenKind::Symbol(Symbol::EqualEqual),
            TokenKind::Symbol(Symbol::FatArrow),
        ]
    );
}

#[test]
fn numeric_spelling_without_type_range_checks() {
    let source = "0 1_000 0xff 0b10 0o77 999999999999999999999999999999 1.25 1e+3 2.5e-2 1.member .5 -2147483648";
    let (sources, result) = scan(source);
    assert!(!result.has_errors());
    assert_lossless(source, &sources, &result);
    let significant = kinds(&result);
    assert_eq!(
        significant
            .iter()
            .filter(|k| **k == TokenKind::Float)
            .count(),
        3
    );
    assert_eq!(
        significant.iter().filter(|k| **k == TokenKind::Dot).count(),
        2
    );
    assert!(significant.contains(&TokenKind::Symbol(Symbol::Minus)));
}

#[test]
fn malformed_numeric_atoms_are_kept_as_error_tokens() {
    for atom in [
        "0x", "0b2", "0o8", "0Xff", "1__2", "1_", "0x_1", "1e+", "12abc",
    ] {
        let (sources, result) = scan(atom);
        assert_eq!(kinds(&result), [TokenKind::Error, TokenKind::Eof], "{atom}");
        assert_eq!(result.diagnostics.len(), 1, "{atom}");
        assert_eq!(result.diagnostics[0].code.to_string(), "N1002");
        assert_eq!(
            sources.slice(result.diagnostics[0].primary.span).unwrap(),
            atom
        );
        assert_lossless(atom, &sources, &result);
    }
}

#[test]
fn nested_comments_keep_newline_events() {
    let source = "let a=1 /* first\r\n /* inner */\n end */ let b=2 // tail\rnext";
    let (sources, result) = scan(source);
    assert!(!result.has_errors());
    assert_lossless(source, &sources, &result);
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|t| t.kind == TokenKind::NewLine)
            .count(),
        3
    );
    assert_eq!(
        normalize_ends(&result.tokens)
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::End(_)))
            .count(),
        2
    );
}

#[test]
fn characters_and_escape_validation() {
    let source = "'가' '🙂' '\\n' '\\u{1F642}' '\\'' \"{{x}}\\n\\u{AC00}\\0\"";
    let (sources, result) = scan(source);
    assert!(!result.has_errors());
    assert_lossless(source, &sources, &result);
    for invalid in [
        "''",
        "'ab'",
        "'e\u{301}'",
        "'\\q'",
        "'\\u{D800}'",
        "\"\\u{110000}\"",
        "\"\\u{1234567}\"",
        "\"x}\"",
    ] {
        let (sources, result) = scan(invalid);
        assert!(result.has_errors(), "{invalid}");
        assert!(result.tokens.iter().any(|t| t.kind == TokenKind::Error));
        assert_lossless(invalid, &sources, &result);
    }
}

#[test]
fn interpolation_uses_an_iterative_mode_stack() {
    let source = "\"outer {call(\"inner {value}\", {x}) /* } */)} end\"";
    let (sources, result) = scan(source);
    assert!(!result.has_errors());
    assert_lossless(source, &sources, &result);
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|t| t.kind == TokenKind::InterpolationStart)
            .count(),
        2
    );
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|t| t.kind == TokenKind::InterpolationEnd)
            .count(),
        2
    );
    assert_eq!(
        result
            .tokens
            .iter()
            .filter(|t| t.kind == TokenKind::StringEnd)
            .count(),
        2
    );
}

#[test]
fn truncated_modes_report_opening_and_eof_without_panicking() {
    for source in [
        "/*",
        "/*\n",
        "\"",
        "'",
        "\"text {f(\"nested",
        "\"\\",
        "\"text\nnext",
        "'x\nnext",
    ] {
        let (sources, result) = scan(source);
        assert!(result.has_errors());
        assert_lossless(source, &sources, &result);
    }
    let (sources, result) = scan("/* unfinished");
    let diagnostic = &result.diagnostics[0];
    assert_eq!(diagnostic.code.to_string(), "N1003");
    assert_eq!(sources.slice(diagnostic.primary.span).unwrap(), "/*");
    assert_eq!(diagnostic.secondary[0].span.start(), 13);
}

#[test]
fn newline_normalization_preserves_origins_and_headers() {
    let source = "func main()\n{\n if true\n { print(\n\"a\",\n\"b\") }\n else\n { return\n print(\"later\") }\n}\n";
    let (_, result) = scan(source);
    assert!(!result.has_errors());
    let normalized = normalize_ends(&result.tokens);
    for pair in normalized.windows(2) {
        assert!(
            !(matches!(pair[0].kind, TokenKind::End(_))
                && pair[1].kind == TokenKind::Keyword(Keyword::Else))
        );
    }
    assert!(normalized
        .windows(2)
        .any(|p| p[0].kind == TokenKind::Keyword(Keyword::Return)
            && p[1].kind == TokenKind::End(EndOrigin::NewLine)));
    let ends = normalized
        .iter()
        .filter(|t| matches!(t.kind, TokenKind::End(_)))
        .map(|t| t.span.start())
        .collect::<Vec<_>>();
    // The call ends at the closing brace boundary, and the following `else`
    // suppresses that newline. Only the bare return and two block exits end.
    assert_eq!(
        ends,
        [
            source.find("return\n").unwrap() + "return".len(),
            source.rfind("}\n}\n").unwrap() + 1,
            source.len() - 1,
        ]
    );
    let (_, semicolon) = scan("let a=1;let b=2\r\n");
    let normalized = normalize_ends(&semicolon.tokens);
    assert!(normalized
        .iter()
        .any(|t| t.kind == TokenKind::End(EndOrigin::Semicolon)));
    let newline = normalized
        .iter()
        .find(|t| t.kind == TokenKind::End(EndOrigin::NewLine))
        .unwrap();
    assert_eq!(newline.span.end() - newline.span.start(), 2);
}

#[test]
fn operators_and_nested_blocks_continue_correctly() {
    for source in [
        "let a=1\n+2\n",
        "let a=1+\n2\n",
        "f()\n.member()\n",
        "let a=[1,\n2]\n",
        "let a=\"{x\n+1}\"\n",
    ] {
        let (_, result) = scan(source);
        assert!(!result.has_errors(), "{source}");
        assert_eq!(
            normalize_ends(&result.tokens)
                .iter()
                .filter(|t| matches!(t.kind, TokenKind::End(_)))
                .count(),
            1,
            "{source}"
        );
    }
    let (_, result) = scan("call(lambda () => {\n let x=1\n return x\n})\n");
    assert_eq!(
        normalize_ends(&result.tokens)
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::End(_)))
            .count(),
        3
    );
}

#[test]
fn generic_header_newlines_do_not_treat_comparisons_as_delimiters() {
    let (_, result) =
        scan("func f<T,\nU>()\n{}\nlet x: Map<int,\nArray<int>>\n=make()\nlet y=1<2\nlet z=3>2\n");
    let normalized = normalize_ends(&result.tokens);
    assert_eq!(
        normalized
            .iter()
            .filter(|t| matches!(t.kind, TokenKind::End(_)))
            .count(),
        4
    );
}

#[test]
fn closing_generic_type_can_end_a_binding_without_an_initializer() {
    let source = "let x: Map<int,\nArray<int>>\nlet y=3>\n2\n";
    let (_, result) = scan(source);
    assert!(!result.has_errors());
    let ends = normalize_ends(&result.tokens)
        .into_iter()
        .filter(|t| matches!(t.kind, TokenKind::End(_)))
        .map(|t| t.span.start())
        .collect::<Vec<_>>();
    assert_eq!(ends, [source.find("\nlet y").unwrap(), source.len() - 1]);
}

#[test]
fn lexical_pass_and_fail_fixtures() {
    let passing = [
        include_str!("fixtures/pass/hello.nova"),
        include_str!("fixtures/pass/literals.nova"),
        include_str!("fixtures/pass/interpolation.nova"),
    ];
    for source in passing {
        let (sources, result) = scan(source);
        assert!(!result.has_errors());
        assert_lossless(source, &sources, &result);
    }
    for (source, code, span) in [
        (include_str!("fixtures/fail/number.nova"), "N1002", "1__2"),
        (include_str!("fixtures/fail/character.nova"), "N1001", "🙂"),
        (include_str!("fixtures/fail/comment.nova"), "N1003", "/*"),
    ] {
        let (sources, result) = scan(source);
        assert!(result.has_errors());
        assert_eq!(result.diagnostics[0].code.to_string(), code);
        assert_eq!(
            sources.slice(result.diagnostics[0].primary.span).unwrap(),
            span
        );
        assert_lossless(source, &sources, &result);
    }
}

#[test]
fn empty_large_deep_and_unknown_file_inputs() {
    let (sources, result) = scan("");
    assert_eq!(kinds(&result), [TokenKind::Eof]);
    assert_eq!(
        lex(&sources, FileId::from_raw(99)).unwrap_err(),
        SourceError::UnknownFile
    );
    let source = "/*".repeat(20_000) + &"*/".repeat(20_000);
    let (sources, result) = scan(&source);
    assert!(!result.has_errors());
    assert_lossless(&source, &sources, &result);
    let source = "\"{".repeat(2_000) + "value" + &"}\"".repeat(2_000);
    let (sources, result) = scan(&source);
    assert!(!result.has_errors());
    assert_lossless(&source, &sources, &result);
}

#[test]
fn deterministic_adversarial_inputs_preserve_all_bytes() {
    let alphabet = [
        'a', '0', '_', '\u{301}', '🙂', '가', '\r', '\n', ' ', '\t', '"', '\'', '\\', '{', '}',
        '[', ']', '(', ')', '/', '*', ';', '<', '>',
    ];
    let mut seed = 0x1234_5678u32;
    for _ in 0..500 {
        let mut source = String::new();
        for _ in 0..80 {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            source.push(alphabet[(seed as usize) % alphabet.len()]);
        }
        let (sources, first) = scan(&source);
        let (_, second) = scan(&source);
        assert_eq!(first.tokens, second.tokens);
        assert_eq!(first.diagnostics, second.diagnostics);
        assert_eq!(
            normalize_ends(&first.tokens),
            normalize_ends(&second.tokens)
        );
        assert_lossless(&source, &sources, &first);
    }
}

#[test]
fn p15_type_close_assignment_preserves_raw_and_normalized_greater_equal() {
    let (db, l) = scan("func f(){let x:Option<int>=none\nlet n=2>=1\nprint(\"done\")}");
    let n = normalize_ends(&l.tokens);
    assert_eq!(
        l.tokens
            .iter()
            .filter(|t| t.kind == TokenKind::Symbol(Symbol::GreaterEqual))
            .count(),
        2
    );
    assert_eq!(
        n.iter()
            .filter(|t| t.kind == TokenKind::Symbol(Symbol::GreaterEqual))
            .count(),
        2
    );
    let ends = n
        .iter()
        .filter(|t| matches!(t.kind, TokenKind::End(EndOrigin::NewLine)))
        .collect::<Vec<_>>();
    assert_eq!(ends.len(), 2);
    for t in n
        .iter()
        .filter(|t| t.kind == TokenKind::Symbol(Symbol::GreaterEqual))
    {
        assert_eq!(db.slice(t.span).unwrap(), ">=");
    }
}

#[test]
fn p21_alias_rhs_end_generics_and_recovery_preserve_comparisons() {
    let (db,l)=scan("type A = Option<\n(int8,Result<bool,int>)\n>\nfunc f(){let b=1<2\nlet c=3>=2}\ntype B=int\n{}");
    assert!(!l.has_errors());
    let tokens = normalize_ends(&l.tokens);
    let texts = tokens
        .iter()
        .map(|t| (t.kind, db.slice(t.span).unwrap()))
        .collect::<Vec<_>>();
    let f = texts
        .iter()
        .position(|t| t.0 == TokenKind::Keyword(Keyword::Func))
        .unwrap();
    assert!(matches!(texts[f - 1].0, TokenKind::End(EndOrigin::NewLine)));
    assert!(texts
        .iter()
        .any(|t| t.0 == TokenKind::Symbol(Symbol::GreaterEqual) && t.1 == ">="));
    let brace = texts
        .iter()
        .rposition(|t| t.0 == TokenKind::LeftBrace)
        .unwrap();
    assert!(matches!(
        texts[brace - 1].0,
        TokenKind::End(EndOrigin::NewLine)
    ));
    assert_eq!(
        l.tokens
            .iter()
            .filter(|t| t.kind != TokenKind::Eof)
            .map(|t| db.slice(t.span).unwrap())
            .collect::<String>(),
        db.file(l.tokens[0].span.file()).unwrap().text()
    );
    for input in [
        "type A=Option<\nfunc f(){let x=1<2\nlet y=3}",
        "type A=Option<;func f(){let x=1<2\nlet y=3}",
    ] {
        let (_, l) = scan(input);
        let tokens = normalize_ends(&l.tokens);
        let last_let = tokens
            .iter()
            .rposition(|t| t.kind == TokenKind::Keyword(Keyword::Let))
            .unwrap();
        assert!(
            matches!(tokens[last_let - 1].kind, TokenKind::End(_)),
            "{input}"
        );
    }
}
