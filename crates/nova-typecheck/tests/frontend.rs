use nova_diagnostics::Diagnostic;
use nova_hir::{lower, HirKind, Module};
use nova_lexer::{lex, normalize_ends};
use nova_parser::parse;
use nova_resolve::{resolve, DefinitionKind, Resolution, Resolved};
use nova_source::SourceDatabase;
use nova_typecheck::{check, CheckError, Checked};
use nova_types::Type;

struct Frontend {
    sources: SourceDatabase,
    upstream: Vec<Diagnostic>,
    semantic: Option<(Module, Resolved, Checked)>,
}
impl Frontend {
    fn diagnostics(&self) -> &[Diagnostic] {
        if let Some((_, _, checked)) = &self.semantic {
            &checked.diagnostics
        } else {
            &self.upstream
        }
    }
    fn passed(&self) -> bool {
        self.upstream.is_empty()
            && self
                .semantic
                .as_ref()
                .is_some_and(|(_, _, checked)| !checked.has_errors())
    }
}
fn frontend(source: &str) -> Frontend {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", source.into()).unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let mut upstream = lexed.diagnostics;
    upstream.extend(parsed.diagnostics.iter().cloned());
    if !upstream.is_empty() || parsed.has_errors() {
        return Frontend {
            sources,
            upstream,
            semantic: None,
        };
    }
    let module = lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = resolve(&module);
    let checked = check(&module, &resolved).unwrap();
    for diagnostic in &checked.diagnostics {
        sources.slice(diagnostic.primary.span).unwrap();
        for label in &diagnostic.secondary {
            sources.slice(label.span).unwrap();
        }
    }
    Frontend {
        sources,
        upstream,
        semantic: Some((module, resolved, checked)),
    }
}
fn pass(source: &str) -> Frontend {
    let result = frontend(source);
    assert!(result.passed(), "{}\n{:?}", source, result.diagnostics());
    result
}
fn codes(result: &Frontend) -> Vec<String> {
    result
        .diagnostics()
        .iter()
        .map(|d| d.code.to_string())
        .collect()
}

#[test]
fn hello_and_forward_function_fixtures_pass_frontend() {
    let hello = pass(include_str!("fixtures/pass/hello.nova"));
    let (_, resolved, checked) = hello.semantic.unwrap();
    assert_eq!(resolved.dump(), include_str!("snapshots/hello.resolved"));
    assert_eq!(checked.dump(), include_str!("snapshots/hello.typed"));
    pass(include_str!("fixtures/pass/functions.nova"));
    pass("func recurse(x: int) -> int { if x==0 { return 0 } else { return recurse(x-1) } }");
    pass("func f(x: int) -> int { return g(x) } func g(x: int) -> int { return f(x) }");
}

#[test]
fn exact_diagnostic_fixture_codes_and_byte_spans() {
    for (source, code, excerpt) in [
        (
            include_str!("fixtures/fail/undefined.nova"),
            "N2001",
            "missing",
        ),
        (include_str!("fixtures/fail/condition.nova"), "N3001", "1"),
        (
            include_str!("fixtures/fail/mismatch.nova"),
            "N2101",
            "\"wrong\"",
        ),
        (
            include_str!("fixtures/fail/range.nova"),
            "N2102",
            "2147483648",
        ),
        (include_str!("fixtures/fail/arity.nova"), "N2201", "print()"),
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), [code]);
        assert_eq!(
            result
                .sources
                .slice(result.diagnostics()[0].primary.span)
                .unwrap(),
            excerpt
        );
    }
    let result = frontend(include_str!("fixtures/fail/return.nova"));
    assert_eq!(codes(&result), ["N3003"]);
    assert_eq!(
        result
            .sources
            .slice(result.diagnostics()[0].secondary[0].span)
            .unwrap(),
        "int"
    );
}

#[test]
fn integer_boundaries_bases_and_signed_magnitude() {
    for spelling in [
        "0",
        "2147483647",
        "-2147483648",
        "0x7fffffff",
        "-0x80000000",
        "0b11111111",
        "0o177",
        "2_147_483_647",
        "-2_147_483_648",
    ] {
        pass(&format!("func f() -> int {{ return {spelling} }}"));
    }
    for spelling in [
        "2147483648",
        "-2147483649",
        "0xffffffff",
        "-(2147483648)",
        "999999999999999999999999999999999999",
    ] {
        let result = frontend(&format!("func main() {{ let x={spelling} }}"));
        assert_eq!(codes(&result), ["N2102"], "{spelling}");
    }
    let result = pass("func main() { let min=-2147483648; let max=2147483647 }");
    let (_, _, checked) = result.semantic.unwrap();
    assert!(checked.integer_values.contains(&Some(i32::MIN)));
    assert!(checked.integer_values.contains(&Some(i32::MAX)));
}

#[test]
fn parameter_and_return_expected_types_flow_into_expressions() {
    pass("func f(x: bool) -> bool { let y: bool=(x && !false); return y } func main() { let z=f(true) }");
    for source in [
        "func f(x: int) {} func main() { f(\"bad\") }",
        "func f() -> string { return 1 }",
        "func main() { let x: bool=(1+2) }",
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), ["N2101"]);
        assert!(!result.diagnostics()[0].secondary.is_empty());
    }
}

#[test]
fn primitive_aliases_unknown_and_unsupported_types() {
    pass("func f(x: int32) -> int { return x } func main() -> void { let value: ()=(); return value }");
    assert_eq!(codes(&frontend("func f(x: Unknown) {}")), ["N2001"]);
    for ty in [
        "int8", "int16", "int64", "uint", "byte", "char", "float", "double", "never",
    ] {
        assert_eq!(
            codes(&frontend(&format!("func f(x: {ty}) {{}}"))),
            ["N1102"],
            "{ty}"
        );
    }
}

#[test]
fn lexical_scope_shadowing_and_initializer_visibility() {
    pass("func main() { let x=1; if true { let x=x+1; print(\"{x}\") } print(\"{x}\") }");
    assert_eq!(codes(&frontend("func main() { let x=x }")), ["N2001"]);
    assert_eq!(
        codes(&frontend("func main() { print(\"{later}\"); let later=1 }")),
        ["N2001"]
    );
    assert_eq!(
        codes(&frontend(
            "func main() { if true { let x=1 } print(\"{x}\") }"
        )),
        ["N2001"]
    );
    let result = pass("func f(x: int) { if true { let x=x+1 } }");
    let (module, resolved, _) = result.semantic.unwrap();
    let references = module
        .nodes()
        .iter()
        .enumerate()
        .filter(|(_, n)| matches!(n.kind, HirKind::Name(_)))
        .map(|(i, _)| resolved.references[i])
        .collect::<Vec<_>>();
    assert!(references.iter().any(|r| matches!(r, Some(Resolution::Definition(id)) if matches!(resolved.definitions[id.0].kind, DefinitionKind::Parameter(_)))));
}

#[test]
fn duplicate_functions_parameters_and_locals_have_secondary_labels() {
    for source in [
        "func f() {} func f() {}",
        "func f(x: int, x: int) {}",
        "func f(x: int) { let x=1 }",
        "func main() { let x=1; let x=2 }",
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), ["N2002"]);
        let diagnostic = &result.diagnostics()[0];
        assert_eq!(diagnostic.secondary.len(), 1);
        assert!(diagnostic.secondary[0].span.start() < diagnostic.primary.span.start());
    }
}

#[test]
fn builtin_print_is_string_only_and_can_be_shadowed() {
    assert_eq!(codes(&frontend("func main() { print(1) }")), ["N2101"]);
    pass("func print(x: int) {} func main() { print(1) }");
    assert_eq!(
        codes(&frontend("func main() { let print=1; print(\"x\") }")),
        ["N2101"]
    );
    pass("func main() { print(\"x {true} {1} {\"nested\"}\") }");
    assert_eq!(
        codes(&frontend("func main() { print(\"x {()}\") }")),
        ["N2101"]
    );
}

#[test]
fn function_values_and_noncallable_results_are_rejected() {
    assert_eq!(
        codes(&frontend("func f() {} func main() { let x=f }")),
        ["N1102"]
    );
    pass("func f() {} func main() { (f)() }");
    assert_eq!(
        codes(&frontend(
            "func f() -> int { return 1 } func main() { f()() }"
        )),
        ["N2101"]
    );
}

#[test]
fn bool_conditions_logical_and_numeric_operator_types() {
    pass("func main() { let x=1+2*3/1%2-4; let c= x<3 && true || !false; if c { return } }");
    for expression in [
        "true+1",
        "\"x\"+\"y\"",
        "1&&2",
        "true<false",
        "!1",
        "-true",
        "() == ()",
        "\"x\"==\"x\"",
    ] {
        let result = frontend(&format!("func main() {{ let x={expression} }}"));
        assert_eq!(codes(&result), ["N2101"], "{expression}");
    }
    pass("func main() { if 1==1 { print(\"equal\") } if true!=false { return } }");
}

#[test]
fn all_return_paths_and_unreachable_errors() {
    pass("func f(x: bool) -> int { if x { return 1 } else if !x { return 2 } else { return 3 } }");
    pass("func f(x: bool) -> int { if x { return 1 } return 2 }");
    assert_eq!(codes(&frontend("func f() -> int { return }")), ["N3002"]);
    assert_eq!(codes(&frontend("func f() -> int {}")), ["N3003"]);
    assert_eq!(codes(&frontend("func main() { return 1 }")), ["N2101"]);
    assert_eq!(
        codes(&frontend("func main() { return; missing() }")),
        ["N2001"]
    );
}

#[test]
fn error_types_suppress_derived_errors() {
    assert_eq!(
        codes(&frontend(
            "func main() { let x=missing; print(x); if x { return } }"
        )),
        ["N2001"]
    );
    assert_eq!(
        codes(&frontend(
            "func main() { let x: int=\"bad\"; print(x); if x { return } }"
        )),
        ["N2101"]
    );
}

#[test]
fn syntax_errors_never_enter_successful_semantic_analysis() {
    for source in [
        "func main() { let x=🙂 }",
        "func main() { let x }",
        "func main() { print(\"x\")",
        "func main() { const x=1 }",
    ] {
        let result = frontend(source);
        assert!(!result.passed());
        assert!(result.semantic.is_none());
        assert!(!result.diagnostics().is_empty());
    }
}

#[test]
fn unicode_and_crlf_semantic_spans() {
    let result = frontend("func main()\r\n{\r\nlet 이름=없음\r\n}\r\n");
    assert_eq!(codes(&result), ["N2001"]);
    assert_eq!(
        result
            .sources
            .slice(result.diagnostics()[0].primary.span)
            .unwrap(),
        "없음"
    );
}

#[test]
fn repeated_results_and_large_flat_body_are_deterministic() {
    let source = include_str!("fixtures/pass/functions.nova");
    let first = pass(source).semantic.unwrap();
    let second = pass(source).semantic.unwrap();
    assert_eq!(first, second);
    assert_eq!(first.0.dump(), second.0.dump());
    assert_eq!(first.1.dump(), second.1.dump());
    assert_eq!(first.2.dump(), second.2.dump());
    let source = format!("func main() {{ {} }}", "print(\"x\");".repeat(10_000));
    pass(&source);
    pass(""); // Fragment checking does not require an entry function.
}

#[test]
fn mismatched_resolution_tables_are_api_errors() {
    let result = pass("func main() {}");
    let (module, mut resolved, _) = result.semantic.unwrap();
    resolved.references.clear();
    assert_eq!(
        check(&module, &resolved),
        Err(CheckError::InvalidResolution)
    );
}

#[test]
fn integer_literal_type_table_is_fixed_width() {
    let result = pass("func main() { let x=42 }");
    let (module, _, checked) = result.semantic.unwrap();
    for (index, node) in module.nodes().iter().enumerate() {
        if matches!(node.kind, HirKind::Integer(_)) {
            assert_eq!(
                checked.types.get(checked.type_table[index]),
                Some(Type::Int32)
            );
        }
    }
}

#[test]
fn mutable_locals_of_all_value_types_and_loop_scopes_pass() {
    pass(include_str!("../../../examples/loops.nova"));
    let result = pass("func f(x:int){var a=x;var b=true;var c=\"x\";var u:()=();a=a+1;b=false;c=\"y\";u=();while b {var a=a+1;a=2;break}}");
    let (_, resolved, _) = result.semantic.unwrap();
    assert_eq!(resolved.definitions.iter().filter(|d| d.mutable).count(), 5);
    assert!(resolved
        .definitions
        .iter()
        .filter(|d| matches!(d.kind, DefinitionKind::Parameter(_)))
        .all(|d| !d.mutable));
    pass("func f(){var x=1;while true {var x=x+1;x=2;break} x=3}");
}

#[test]
fn assignment_errors_have_exact_target_value_and_declaration_spans() {
    for (source, code, excerpt, secondary) in [
        ("func f(){let 한글=1;한글=2}", "N3004", "한글", true),
        ("func f(x:int){x=2}", "N3004", "x", true),
        ("func f(){f=2}", "N3004", "f", true),
        ("func f(){print=2}", "N3004", "print", false),
        ("func f(){missing=1}", "N2001", "missing", false),
        ("func f(){var x=1;x=true}", "N2101", "true", true),
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), [code], "{source}");
        let diagnostic = &result.diagnostics()[0];
        assert_eq!(
            result.sources.slice(diagnostic.primary.span).unwrap(),
            excerpt
        );
        assert_eq!(!diagnostic.secondary.is_empty(), secondary);
        if secondary {
            assert!(diagnostic.secondary[0].span.start() < diagnostic.primary.span.start());
        }
    }
}

#[test]
fn nearer_immutable_shadow_blocks_outer_mutable_assignment() {
    assert_eq!(
        codes(&frontend(
            "func f(){var x=1;while true {let x=x;x=2;break}}"
        )),
        ["N3004"]
    );
    assert_eq!(
        codes(&frontend("func f(){while false {var inner=1} inner=2}")),
        ["N2001"]
    );
    assert_eq!(codes(&frontend("func f(){var x=x}")), ["N2001"]);
    assert_eq!(codes(&frontend("func f(){var x=1;let x=2}")), ["N2002"]);
}

#[test]
fn while_conditions_and_jump_boundaries_use_control_diagnostics() {
    let result = frontend("func f(){while 1 {break}}");
    assert_eq!(codes(&result), ["N3001"]);
    assert_eq!(
        result
            .sources
            .slice(result.diagnostics()[0].primary.span)
            .unwrap(),
        "1"
    );
    for jump in ["break", "continue"] {
        let result = frontend(&format!("func f(){{if true {{{jump}}}}}"));
        assert_eq!(codes(&result), ["N3002"]);
        assert_eq!(
            result
                .sources
                .slice(result.diagnostics()[0].primary.span)
                .unwrap(),
            jump
        );
    }
    assert_eq!(
        codes(&frontend("func f(){while false {break} continue}")),
        ["N3002"]
    );
    pass("func f(){while true {if true {continue}else{break}}}");
}

#[test]
fn loop_returns_are_conservative_and_unreachable_source_is_checked() {
    for source in [
        "func f()->int{while true {return 1}}",
        "func f()->int{while true {break;return 1}}",
    ] {
        assert_eq!(codes(&frontend(source)), ["N3003"]);
    }
    pass("func f(x:bool)->int{while x {if x {return 1}else{break}} return 2}");
    assert_eq!(
        codes(&frontend("func f(){while true {break;missing()}}")),
        ["N2001"]
    );
    assert_eq!(
        codes(&frontend("func f(){while true {continue;let x:int=true}}")),
        ["N2101"]
    );
    let result = pass("func f(){while true {if true {break}else{continue};return}}");
    let (module, _, checked) = result.semantic.unwrap();
    for (index, node) in module.nodes().iter().enumerate() {
        if node.kind == HirKind::Block {
            assert!(
                !checked.always_returns[index],
                "unreachable return cannot prove return"
            );
        }
    }
}

#[test]
fn assignment_error_types_suppress_derived_type_diagnostics() {
    assert_eq!(
        codes(&frontend("func f(){var x=missing;x=true;while x {break}}")),
        ["N2001"]
    );
    assert_eq!(codes(&frontend("func f(){var x=1;x=missing}")), ["N2001"]);
}

#[test]
fn tampered_mutability_is_an_invalid_resolution_table() {
    let result = pass("func f(){let x=1}");
    let (module, mut resolved, _) = result.semantic.unwrap();
    resolved
        .definitions
        .iter_mut()
        .find(|d| matches!(d.kind, DefinitionKind::Local(_)))
        .unwrap()
        .mutable = true;
    assert_eq!(
        check(&module, &resolved),
        Err(CheckError::InvalidResolution)
    );
}
