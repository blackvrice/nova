use nova_diagnostics::Diagnostic;
use nova_hir::{lower, HirKind, Module};
use nova_lexer::{lex, normalize_ends};
use nova_parser::parse;
use nova_resolve::{resolve, DefinitionKind, Resolution, Resolved};
use nova_source::SourceDatabase;
use nova_typecheck::{check, CheckError, Checked, ConstEvaluation, CONST_NODE_LIMIT};
use nova_types::{ConstValue, Type};

const INTEGER_CASES: [(&str, Type, i128, i128); 8] = [
    ("int8", Type::Int8, -128, 127),
    ("uint8", Type::UInt8, 0, 255),
    ("int16", Type::Int16, -32768, 32767),
    ("uint16", Type::UInt16, 0, 65535),
    ("int32", Type::Int32, -2147483648, 2147483647),
    ("uint32", Type::UInt32, 0, 4294967295),
    (
        "int64",
        Type::Int64,
        -9223372036854775808,
        9223372036854775807,
    ),
    ("uint64", Type::UInt64, 0, 18446744073709551615),
];

#[test]
fn p08_char_binding_calls_returns_forward_const_and_scalar_comparisons() {
    pass(include_str!("../../../examples/characters.nova"));
    pass("func f(x:char)->char{var y:char=x;while y<'Z'{y='Z';continue};return (y)} func print(x:char)->char{return x} func g(){const print='a';let x=print}");
    for value in [
        0, 0x7f, 0x80, 0x7ff, 0x800, 0xd7ff, 0xe000, 0xffff, 0x10000, 0x10ffff, 0x378,
    ] {
        let source = format!(
            "const A:char=B;const B='\\u{{{value:X}}}';func f(){{const C:char=(A);let x:char=C}}"
        );
        assert_eq!(
            constant_value(&source, "A").0,
            ConstValue::Char(char::from_u32(value).unwrap())
        );
    }
    for (left, right) in [
        ('a', 'a'),
        ('a', 'b'),
        ('\u{d7ff}', '\u{e000}'),
        ('\u{10ffff}', '\0'),
    ] {
        for (op, expected) in [
            ("==", left == right),
            ("!=", left != right),
            ("<", left < right),
            ("<=", left <= right),
            (">", left > right),
            (">=", left >= right),
        ] {
            let source = format!("const A='\\u{{{:X}}}';const B='\\u{{{:X}}}';const C=A{op}B;func f(a:char,b:char)->bool{{return a{op}b}}", left as u32, right as u32);
            assert_eq!(constant_value(&source, "C").0, ConstValue::Bool(expected));
        }
    }
    assert_eq!(
        constant_value("const C=false&&('a'<'b'&&1/0==0)", "C").0,
        ConstValue::Bool(false)
    );
}

#[test]
fn p08_char_has_no_implicit_conversions_arithmetic_or_print_overload() {
    for (ty, value) in INTEGER_CASES.iter().map(|(name, ..)| (*name, "1")).chain([
        ("bool", "true"),
        ("string", "\"a\""),
        ("void", "()"),
    ]) {
        for source in [
            format!("func f(){{let x:{ty}='a'}}"),
            format!("func f(){{let x:char={value}}}"),
            format!("func f(x:char){{var y:{ty}={value};y=x}}"),
            format!("func f(x:{ty}){{var y:char='a';y=x}}"),
            format!("func f()->{ty}{{return 'a'}}"),
            format!("func f()->char{{return {value}}}"),
            format!("func f(x:{ty}){{}} func g(){{f('a')}}"),
            format!("func f(x:char){{}} func g(){{f({value})}}"),
            format!("const A:{ty}='a'"),
            format!("const A:char={value}"),
        ] {
            assert_eq!(codes(&frontend(&source)), ["N2101"], "{source}");
        }
        for op in ["==", "!=", "<", "<=", ">", ">="] {
            for expr in [format!("'a'{op}{value}"), format!("{value}{op}'a'")] {
                assert_eq!(
                    codes(&frontend(&format!("func f(){{let x={expr}}}"))),
                    ["N2101"],
                    "{expr}"
                );
            }
        }
    }
    for expr in [
        "'a'+'b'",
        "'a'-'b'",
        "'a'*'b'",
        "'a'/'b'",
        "'a'%'b'",
        "+'a'",
        "-'a'",
        "!'a'",
        "'a'&&'b'",
        "'a'||'b'",
        "print('a')",
    ] {
        assert_eq!(
            codes(&frontend(&format!("func f(){{{expr}}}"))),
            ["N2101"],
            "{expr}"
        );
    }
    assert_eq!(
        codes(&frontend("func f(){if 'a' {} while 'b' {}}")),
        ["N3001", "N3001"]
    );
    assert_eq!(codes(&frontend("func f(){let a='a';a='b'}")), ["N3004"]);
    assert_eq!(codes(&frontend("const print='a'")), ["N2002"]);
}

#[test]
fn p08_char_const_permission_cycles_and_budget_include_skipped_nodes() {
    for (source, code) in [
        ("func f(){let a='a';const b=a}", "N3201"),
        (
            "func f()->char{return 'a'} const C=false&&(f()=='a')",
            "N3201",
        ),
        ("const C=false&&('a'==1)", "N2101"),
        ("const C=true||(C&&'a'=='a')", "N3202"),
        ("const C=\"{'a'}\"", "N3201"),
    ] {
        assert_eq!(codes(&frontend(source)), [code], "{source}");
    }
    let chain = std::iter::repeat("'a'=='a'")
        .take(2500)
        .collect::<Vec<_>>()
        .join("&&");
    assert_eq!(
        constant_value(&format!("const C=({chain})"), "C"),
        (ConstValue::Bool(true), 10000)
    );
    assert_eq!(
        codes(&frontend(&format!("const C={chain}&&true"))),
        ["N3202"]
    );
    assert_eq!(
        codes(&frontend(&format!("const C=false&&({chain})"))),
        ["N3202"]
    );
}

#[test]
fn p07_all_integer_literal_boundaries_aliases_and_exact_range_spans() {
    for (name, ty, min, max) in INTEGER_CASES {
        let f = pass(&format!(
            "const MIN:{name}={min};const MAX:{name}={max};func f(x:{name})->{name}{{return x}}"
        ));
        let (_, resolved, checked) = f.semantic.unwrap();
        for (constant, expected) in [("MIN", min), ("MAX", max)] {
            let index = resolved
                .definitions
                .iter()
                .position(|d| d.name == constant)
                .unwrap();
            let ConstEvaluation::Value { value, .. } = &checked.const_values[index] else {
                panic!()
            };
            assert_eq!(value.ty(), ty);
            assert_eq!(value.integer().unwrap().value(), expected);
        }
        for value in [min - 1, max + 1] {
            let spelling = value.to_string();
            let f = frontend(&format!("const BAD:{name}={spelling}"));
            assert_eq!(codes(&f), ["N2102"], "{name}: {value}");
            assert_eq!(
                f.sources.slice(f.diagnostics()[0].primary.span).unwrap(),
                spelling
            );
            assert!(!f.diagnostics()[0].secondary.is_empty());
        }
        if min < 0 {
            assert_eq!(
                codes(&frontend(&format!("const BAD:{name}=-({})", -min))),
                ["N2102"]
            );
        } else {
            assert_eq!(codes(&frontend(&format!("const BAD:{name}=-0"))), ["N2102"]);
            assert_eq!(
                codes(&frontend(&format!("func f(x:{name}){{let y=-x}}"))),
                ["N2101"]
            );
        }
    }
    pass("const A:int=0x7fff_ffff;const B:uint=0xffff_ffff;const C:byte=0b1111_1111;const D:int64=-0x8000_0000_0000_0000;const E:uint64=0xffff_ffff_ffff_ffff;const F:uint16=0o177_777");
    assert_eq!(codes(&frontend("func f(){let x=2147483648}")), ["N2102"]);
    assert_eq!(
        codes(&frontend(&format!("const A:uint64={}", "9".repeat(10000)))),
        ["N2102"]
    );
}

#[test]
fn p07_all_typed_conversion_sites_and_binary_joins_follow_range_containment() {
    for (source, source_ty, min, max) in INTEGER_CASES {
        for (dest, dest_ty, dmin, dmax) in INTEGER_CASES {
            let allowed = min >= dmin && max <= dmax;
            for text in [
                format!("func f(x:{source}){{let y:{dest}=x}}"),
                format!("func f(x:{source})->{dest}{{return x}}"),
                format!("func g(x:{dest}){{}} func f(x:{source}){{g(x)}}"),
                format!("func f(x:{source}){{var y:{dest}=0;y=x}}"),
                format!("const A:{source}=1;const B:{dest}=A"),
            ] {
                let f = frontend(&text);
                assert_eq!(f.passed(), allowed, "{text}: {:?}", f.diagnostics());
                if !allowed {
                    assert_eq!(codes(&f), ["N2101"], "{text}");
                }
            }
            let common = [1, 0, 3, 2, 5, 4, 7, 6]
                .into_iter()
                .find(|&k| {
                    INTEGER_CASES[k].2 <= min.min(dmin) && INTEGER_CASES[k].3 >= max.max(dmax)
                })
                .map(|k| INTEGER_CASES[k].1);
            for op in ["+", "<", "=="] {
                let text = format!("func f(a:{source},b:{dest}){{let r=a{op}b}}");
                let f = frontend(&text);
                assert_eq!(f.passed(), common.is_some(), "{text}");
                if let Some(common) = common {
                    let (hir, _, checked) = f.semantic.unwrap();
                    let id = hir
                        .nodes()
                        .iter()
                        .position(|n| matches!(n.kind, HirKind::Binary(_)))
                        .unwrap();
                    assert_eq!(
                        checked.types.get(checked.type_table[id]),
                        Some(if op == "+" { common } else { Type::Bool })
                    );
                    for &child in &hir.nodes()[id].children {
                        let raw = checked.types.get(checked.type_table[child.0]).unwrap();
                        assert!(raw == source_ty || raw == dest_ty);
                        assert_eq!(
                            checked.types.get(
                                checked.coercions[child.0].unwrap_or(checked.type_table[child.0])
                            ),
                            Some(common)
                        );
                    }
                } else {
                    assert_eq!(codes(&f), ["N2101"]);
                }
            }
        }
    }
}

#[test]
fn p07_expected_peer_literal_subtrees_keep_typed_arithmetic_width() {
    pass(include_str!("../../../examples/integers.nova"));
    pass("func f(x:uint64)->uint64{return (1+(2*3))+x} func g(x:uint64)->bool{return (1+2)<x} func h(x:uint64)->bool{return x==(0+1)}");
    pass("func f(x:int8)->int64{return 2147483648+x} func g()->int64{return +(2147483648+1)}");
    let f = pass("func f(a:int8,b:int8){let x=a+1;let y:int64=a+1;let z:int64=(a+b)}");
    let (hir, _, checked) = f.semantic.unwrap();
    let binary: Vec<_> = hir
        .nodes()
        .iter()
        .enumerate()
        .filter(|(_, n)| matches!(n.kind, HirKind::Binary(_)))
        .map(|(i, _)| checked.types.get(checked.type_table[i]).unwrap())
        .collect();
    assert_eq!(binary, [Type::Int8, Type::Int64, Type::Int8]);
    assert_eq!(codes(&frontend("func f(a:int8){let x=a+128}")), ["N2102"]);
    assert_eq!(
        codes(&frontend("func f(a:uint64){let x=a+(-1)}")),
        ["N2102"]
    );
    assert_eq!(
        codes(&frontend("func f(a:int8){let x:bool=a+1}")),
        ["N2101"]
    );
    assert_eq!(codes(&frontend("func f(){let x:int64=1+true}")), ["N2101"]);
}

#[test]
fn p07_const_checked_failures_normal_values_and_short_circuit() {
    for (name, _, min, max) in INTEGER_CASES {
        for expr in [
            format!("{max}+1"),
            format!("{max}*2"),
            format!("{min}-1"),
            "1/0".into(),
            "1%0".into(),
        ] {
            let f = frontend(&format!("const X:{name}={expr}"));
            assert_eq!(codes(&f), ["N3201"], "{name}:{expr}");
            assert_eq!(
                f.sources.slice(f.diagnostics()[0].primary.span).unwrap(),
                expr
            );
        }
        if min < 0 {
            for op in ["/", "%"] {
                assert_eq!(
                    codes(&frontend(&format!("const X:{name}=({min}){op}-1"))),
                    ["N3201"]
                );
            }
        }
        pass(&format!(
            "const X:{name}=7/3;const Y:{name}=7%3;const W:int64=0;const SAFE=true||(X/0==0)"
        ));
    }
    let f = pass(
        "const B:int16=A;const A:byte=255;const C:int64=B+1;const D:uint64=18446744073709551615/3",
    );
    let (_, resolved, checked) = f.semantic.unwrap();
    for (name, ty, expected) in [
        ("B", Type::Int16, 255),
        ("C", Type::Int64, 256),
        ("D", Type::UInt64, 6148914691236517205),
    ] {
        let i = resolved
            .definitions
            .iter()
            .position(|d| d.name == name)
            .unwrap();
        let ConstEvaluation::Value { value, .. } = &checked.const_values[i] else {
            panic!()
        };
        assert_eq!(value.ty(), ty);
        assert_eq!(value.integer().unwrap().value(), expected);
    }
    assert_eq!(
        codes(&frontend("const A:uint64=B;const B:uint64=A")),
        ["N3202"]
    );
    assert_eq!(
        codes(&frontend("const A:byte=1;const BAD=false&&(A+256==0)")),
        ["N2102"]
    );
    assert_eq!(
        codes(&frontend(
            "func f()->uint64{return 0} const BAD=true||(f()==0)"
        )),
        ["N3201"]
    );
}

#[test]
fn p07_coercions_do_not_charge_const_hir_budget() {
    let chain = (0..5000)
        .map(|i| if i % 2 == 0 { "A" } else { "B" })
        .collect::<Vec<_>>()
        .join("+");
    let source = format!("const A:int8=0;const B:uint8=0;const C:int64=({chain})");
    let (_, resolved, checked) = pass(&source).semantic.unwrap();
    let i = resolved
        .definitions
        .iter()
        .position(|d| d.name == "C")
        .unwrap();
    let ConstEvaluation::Value { value, nodes } = &checked.const_values[i] else {
        panic!()
    };
    assert_eq!(*nodes, 10000);
    assert_eq!(value.ty(), Type::Int64);
    assert!(checked.coercions.iter().flatten().count() > 2000);
    assert_eq!(
        codes(&frontend(&format!(
            "const A:int8=0;const B:uint8=0;const C:int64={chain}+A"
        ))),
        ["N3202"]
    );
}

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
    for ty in ["float", "double", "never"] {
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
        "func main() { for x in xs {} }",
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

fn constant_value(source: &str, name: &str) -> (ConstValue, usize) {
    let result = pass(source);
    let (_, resolved, checked) = result.semantic.unwrap();
    let index = resolved
        .definitions
        .iter()
        .position(|d| d.name == name && d.constant)
        .unwrap();
    let ConstEvaluation::Value { value, nodes } = &checked.const_values[index] else {
        panic!("const value")
    };
    (value.clone(), *nodes)
}

#[test]
fn const_value_types_operations_and_integer_boundaries_are_exact() {
    for (expression, expected) in [
        ("1+2*3-4", ConstValue::Int32(3)),
        ("+7", ConstValue::Int32(7)),
        ("-2147483648", ConstValue::Int32(i32::MIN)),
        ("0x7fff_ffff", ConstValue::Int32(i32::MAX)),
        ("-7/3", ConstValue::Int32(-2)),
        ("-7%3", ConstValue::Int32(-1)),
        ("7/-3", ConstValue::Int32(-2)),
        ("7%-3", ConstValue::Int32(1)),
        ("!false", ConstValue::Bool(true)),
        ("true!=false", ConstValue::Bool(true)),
        ("true==false", ConstValue::Bool(false)),
        ("1==1", ConstValue::Bool(true)),
        ("1!=2", ConstValue::Bool(true)),
        ("1<2", ConstValue::Bool(true)),
        ("1<=1", ConstValue::Bool(true)),
        ("2>1", ConstValue::Bool(true)),
        ("2>=2", ConstValue::Bool(true)),
        ("true&&false||true", ConstValue::Bool(true)),
        ("false||true&&true", ConstValue::Bool(true)),
        ("()", ConstValue::Unit),
        ("\"한글\\0{{x}}\"", ConstValue::String("한글\0{x}".into())),
    ] {
        assert_eq!(
            constant_value(&format!("func f(){{const x={expression}}}"), "x").0,
            expected,
            "{expression}"
        );
    }
    pass(include_str!("../../../examples/constants.nova"));
}

#[test]
fn const_dependencies_shadowing_duplicate_forward_and_immutability() {
    assert_eq!(
        constant_value("func f(){const a=2;const b=a*3}", "b"),
        (ConstValue::Int32(6), 3)
    );
    let result = pass("func f(){const x=1;if true {const x=x+1;print(\"{x}\")}}");
    let (_, resolved, checked) = result.semantic.unwrap();
    let values = resolved
        .definitions
        .iter()
        .enumerate()
        .filter(|(_, d)| d.constant)
        .map(|(i, _)| checked.const_values[i].clone())
        .collect::<Vec<_>>();
    assert!(matches!(
        values[1],
        ConstEvaluation::Value {
            value: ConstValue::Int32(2),
            ..
        }
    ));
    for (source, code) in [
        ("func f(){const x=x}", "N2001"),
        ("func f(){const x=y;const y=1}", "N2001"),
        ("func f(){const x=1;const x=2}", "N2002"),
        ("func f(){const x=1;x=2}", "N3004"),
        ("func f(){const x:int=true}", "N2101"),
    ] {
        assert_eq!(codes(&frontend(source)), [code]);
    }
}

#[test]
fn const_arithmetic_failures_use_operation_and_declaration_spans() {
    for expression in [
        "2147483647+1",
        "-2147483648-1",
        "2147483647*2",
        "1/0",
        "1%0",
        "-2147483648/-1",
        "-2147483648%-1",
        "-(-2147483648)",
    ] {
        let result = frontend(&format!("func f(){{const BAD={expression}}}"));
        assert_eq!(codes(&result), ["N3201"], "{expression}");
        let diagnostic = &result.diagnostics()[0];
        assert_eq!(
            result.sources.slice(diagnostic.primary.span).unwrap(),
            expression
        );
        assert_eq!(
            result.sources.slice(diagnostic.secondary[0].span).unwrap(),
            "BAD"
        );
    }
}

#[test]
fn const_short_circuit_skips_arithmetic_but_checks_all_types_and_permissions() {
    for (expression, expected) in [("false&&(1/0==0)", false), ("true||(1/0==0)", true)] {
        assert_eq!(
            constant_value(&format!("func f(){{const x={expression}}}"), "x").0,
            ConstValue::Bool(expected)
        );
    }
    for expression in ["true&&(1/0==0)", "false||(1/0==0)"] {
        assert_eq!(
            codes(&frontend(&format!("func f(){{const x={expression}}}"))),
            ["N3201"]
        );
    }
    assert_eq!(
        codes(&frontend("func f(){const x=false&&(2147483648==0)}")),
        ["N2102"]
    );
    assert_eq!(codes(&frontend("func f(){const x=false&&1}")), ["N2101"]);
    assert_eq!(
        codes(&frontend(
            "func rhs()->bool{return true} func f(){const x=false&&rhs()}"
        )),
        ["N3201"]
    );
    assert_eq!(
        codes(&frontend("func f(){var y=true;const x=true||y}")),
        ["N3201"]
    );
}

#[test]
fn const_forbidden_names_calls_interpolation_have_exact_spans() {
    for (source, excerpt) in [
        ("func f(){let y=1;const x=y}", "y"),
        ("func f(){var y=1;const x=y}", "y"),
        ("func f(y:int){const x=y}", "y"),
        (
            "func value()->int{return 1} func f(){const x=value()}",
            "value()",
        ),
        ("func f(){const x=print(\"x\")}", "print(\"x\")"),
        ("func f(){const x=\"value {1}\"}", "\"value {1}\""),
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), ["N3201"]);
        assert_eq!(
            result
                .sources
                .slice(result.diagnostics()[0].primary.span)
                .unwrap(),
            excerpt
        );
    }
    assert_eq!(
        codes(&frontend("func value(){} func f(){const x=value}")),
        ["N1102"]
    );
}

#[test]
fn const_failed_and_unreachable_values_do_not_generate_cascades() {
    let result = frontend("func f(){const x=1/0;const y=x+1;print(\"{y}\")}");
    assert_eq!(codes(&result), ["N3201"]);
    let (_, resolved, checked) = result.semantic.unwrap();
    for (index, definition) in resolved.definitions.iter().enumerate() {
        match definition.name.as_str() {
            "x" => assert!(matches!(
                checked.const_values[index],
                ConstEvaluation::Failed { code: 3201, .. }
            )),
            "y" => assert_eq!(checked.const_values[index], ConstEvaluation::Invalid),
            _ => {}
        }
    }
    for source in [
        "func f(){return;const x=1/0}",
        "func f(){while false {const x=1/0}}",
    ] {
        assert_eq!(codes(&frontend(source)), ["N3201"]);
    }
    assert_eq!(
        codes(&frontend("func f(){const x=missing;const y=x+1}")),
        ["N2001"]
    );
    assert_eq!(
        codes(&frontend("func f(){const x=\"a\"==\"b\"}")),
        ["N2101"]
    );
}

#[test]
fn const_node_budget_boundary_skipped_subtree_and_cached_names_are_deterministic() {
    assert_eq!(CONST_NODE_LIMIT, 10_000);
    let chain = std::iter::repeat("1")
        .take(5000)
        .collect::<Vec<_>>()
        .join("+");
    let source = format!("func f(){{const x=({chain});const y=x;const z=({chain})}}");
    assert_eq!(
        constant_value(&source, "x"),
        (ConstValue::Int32(5000), 10_000)
    );
    assert_eq!(constant_value(&source, "y"), (ConstValue::Int32(5000), 1));
    let overflow = format!("func f(){{const x={chain}+1}}");
    let first = frontend(&overflow);
    let second = frontend(&overflow);
    assert_eq!(codes(&first), ["N3202"]);
    assert_eq!(first.diagnostics(), second.diagnostics());
    assert_eq!(
        first
            .sources
            .slice(first.diagnostics()[0].primary.span)
            .unwrap(),
        "1"
    );
    assert!(first.diagnostics()[0].notes[0].contains("10000"));
    assert_eq!(
        codes(&frontend(&format!(
            "func f(){{const x=false&&({chain}==0)}}"
        ))),
        ["N3202"]
    );
}

#[test]
fn const_resolution_flags_and_successful_dumps_are_stable() {
    let source = "func f(){const a=1+2;const text=\"x\";const empty=()}";
    let first = pass(source).semantic.unwrap();
    let second = pass(source).semantic.unwrap();
    assert_eq!(first, second);
    assert!(first.1.dump().contains("const def"));
    assert!(first.2.dump().contains("value: Int32(3), nodes: 3"));
    let (module, mut resolved, _) = first;
    resolved
        .definitions
        .iter_mut()
        .find(|d| d.constant)
        .unwrap()
        .constant = false;
    assert_eq!(
        check(&module, &resolved),
        Err(CheckError::InvalidResolution)
    );
}

#[test]
fn globals_infer_all_types_before_functions_and_independent_of_item_order() {
    pass(include_str!("../../../examples/global_constants.nova"));
    for source in [
        "const A:int=B*2;func f()->int{return A} const B=3",
        "func f()->int{return A} const B=3;const A:int=B*2",
    ] {
        assert_eq!(constant_value(source, "A"), (ConstValue::Int32(6), 3));
    }
    let source = "func f(){const local=T;print(local)} const T=TEXT;const TEXT=\"한글\\0🙂\";const B=!false;const U:()=()";
    assert_eq!(
        constant_value(source, "T").0,
        ConstValue::String("한글\0🙂".into())
    );
    assert_eq!(constant_value(source, "B").0, ConstValue::Bool(true));
    assert_eq!(constant_value(source, "U").0, ConstValue::Unit);
    let (hir, resolved, checked) = pass(source).semantic.unwrap();
    assert_eq!((hir, resolved, checked), pass(source).semantic.unwrap());
}

#[test]
fn global_scope_collisions_shadowing_and_readonly_assignment_have_exact_spans() {
    assert_eq!(
        constant_value("const X=2;func f(){const X=X+1}", "X").0,
        ConstValue::Int32(2)
    );
    pass("const X=2;func f(X:int){let Y=X;if true {var X=Y;X=3}}");
    for (source, code, excerpt) in [
        ("const A=1;const A=2", "N2002", "A"),
        ("func A(){} const A=2", "N2002", "A"),
        ("const A=2;func A(){}", "N2002", "A"),
        ("const print=2", "N2002", "print"),
        ("const A=missing", "N2001", "missing"),
        ("const A=x;func f(x:int){}", "N2001", "x"),
        ("const A=1;func f(){A=2}", "N3004", "A"),
        ("func f(){const A=B;const B=2}", "N2001", "B"),
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), [code], "{source}");
        assert_eq!(
            result
                .sources
                .slice(result.diagnostics()[0].primary.span)
                .unwrap(),
            excerpt
        );
        if code == "N3004" {
            assert_eq!(
                result
                    .sources
                    .slice(result.diagnostics()[0].secondary[0].span)
                    .unwrap(),
                "A"
            );
        }
    }
    let (_, resolved, checked) = pass("const X=2;func f(){const X=X+1;var Y=X;Y=Y+1}")
        .semantic
        .unwrap();
    let local = resolved
        .definitions
        .iter()
        .position(|d| d.name == "X" && matches!(d.kind, DefinitionKind::Local(_)))
        .unwrap();
    assert_eq!(
        checked.const_values[local],
        ConstEvaluation::Value {
            value: ConstValue::Int32(3),
            nodes: 3
        }
    );
}

#[test]
fn static_global_cycles_include_skipped_edges_and_annotated_dependencies() {
    for (source, chain) in [
        ("const A=A", "A -> A"),
        ("const A=false&&A", "A -> A"),
        ("const A=true||B;const B=A", "A -> B -> A"),
        ("const A:int=B;const B:int=A", "A -> B -> A"),
        ("const A=B;const B=C;const C=A", "A -> B -> C -> A"),
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), ["N3202"]);
        let diagnostic = &result.diagnostics()[0];
        assert_eq!(
            diagnostic.notes,
            [format!("const dependency cycle: {chain}")]
        );
        assert_eq!(result.sources.slice(diagnostic.primary.span).unwrap(), "A");
        assert_eq!(diagnostic.primary.span.start(), source.rfind('A').unwrap());
        assert_eq!(
            diagnostic.secondary.len(),
            chain.split(" -> ").count() * 2 - 3
        );
        assert_eq!(result.diagnostics(), frontend(source).diagnostics());
    }
}

#[test]
fn cycles_are_reported_once_per_scc_with_source_order_path_and_independent_errors() {
    let source = "const A=B+C;const B=A+A;const C=A;const D=E;const E=D;const F=A;const BAD=1/0";
    let result = frontend(source);
    assert_eq!(codes(&result), ["N3202", "N3202", "N3201"]);
    assert_eq!(
        result.diagnostics()[0].notes[0],
        "const dependency cycle: A -> B -> A"
    );
    assert_eq!(
        result.diagnostics()[1].notes[0],
        "const dependency cycle: D -> E -> D"
    );
    let (_, resolved, checked) = result.semantic.unwrap();
    for (id, def) in resolved.definitions.iter().enumerate() {
        if ["A", "B", "C", "D", "E"].contains(&def.name.as_str()) {
            assert_eq!(
                checked.const_values[id],
                ConstEvaluation::Failed {
                    code: 3202,
                    nodes: 0
                }
            );
        } else if def.name == "F" {
            assert_eq!(checked.const_values[id], ConstEvaluation::Invalid);
        }
    }
}

#[test]
fn failed_global_dependencies_and_unused_initializers_never_hide_errors_or_cascade() {
    for (source, code) in [
        (
            "const USE=BAD+1;func f(){const X=USE;print(\"{X}\")} const BAD=1/0",
            "N3201",
        ),
        ("const A=B;const B=missing", "N2001"),
        ("const A=B;const B=A+missing", "N2001"),
        ("const A=B;const B=\"a\"==\"b\"", "N2101"),
        ("func main(){} const UNUSED=1/0", "N3201"),
        ("const A=true||BAD;const BAD=1/0==0", "N3201"),
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), [code], "{source}");
        assert!(!result
            .semantic
            .unwrap()
            .2
            .const_values
            .contains(&ConstEvaluation::Pending));
    }
}

#[test]
fn long_global_chains_cycles_and_many_components_do_not_recurse() {
    let mut source = String::new();
    for i in 0..10_000 {
        source.push_str(&format!("const C{i}=C{}+1\n", i + 1));
    }
    source.push_str("const C10000=0");
    assert_eq!(
        constant_value(&source, "C0"),
        (ConstValue::Int32(10_000), 3)
    );
    let mut cycle = String::new();
    for i in 0..2048 {
        cycle.push_str(&format!("const C{i}=C{}\n", (i + 1) % 2048));
    }
    let result = frontend(&cycle);
    assert_eq!(codes(&result), ["N3202"]);
    assert_eq!(result.diagnostics()[0].secondary.len(), 4095);
    let source = (0..1024)
        .map(|i| format!("const C{i}=C{i}\n"))
        .collect::<String>();
    let result = frontend(&source);
    assert_eq!(result.diagnostics().len(), 1024);
    for (index, diagnostic) in result.diagnostics().iter().enumerate() {
        assert_eq!(
            diagnostic.notes[0],
            format!("const dependency cycle: C{index} -> C{index}")
        );
    }
}

#[test]
fn global_budget_counts_cached_names_once_and_resets_for_local_and_global_initializers() {
    let chain = std::iter::repeat("1")
        .take(5000)
        .collect::<Vec<_>>()
        .join("+");
    let source = format!("const A=({chain});const B=A;const C=({chain});func f(){{const X=B}} ");
    assert_eq!(
        constant_value(&source, "A"),
        (ConstValue::Int32(5000), 10_000)
    );
    assert_eq!(constant_value(&source, "B"), (ConstValue::Int32(5000), 1));
    assert_eq!(constant_value(&source, "X"), (ConstValue::Int32(5000), 1));
    for source in [
        format!("const A={chain}+1"),
        format!("const A=false&&({chain}==0)"),
    ] {
        let result = frontend(&source);
        assert_eq!(codes(&result), ["N3202"]);
        assert!(result.diagnostics()[0].notes[0].contains("10000"));
        assert_eq!(
            result
                .sources
                .slice(result.diagnostics()[0].primary.span)
                .unwrap(),
            "1"
        );
    }
}

#[test]
fn global_const_operations_permissions_and_type_failures_reuse_p05() {
    let fixture = frontend(include_str!(
        "../../../docs/development-v0.1/fixtures/const-divzero.nova"
    ));
    assert_eq!(codes(&fixture), ["N3201"]);
    assert_eq!(
        (
            fixture.diagnostics()[0].primary.span.start(),
            fixture.diagnostics()[0].primary.span.end()
        ),
        (17, 22)
    );
    for (expression, value) in [
        ("-2147483648", ConstValue::Int32(i32::MIN)),
        ("2147483647", ConstValue::Int32(i32::MAX)),
        ("7/-3", ConstValue::Int32(-2)),
        ("-7%3", ConstValue::Int32(-1)),
        ("1+2*3-4", ConstValue::Int32(3)),
        ("1<2&&2>=2", ConstValue::Bool(true)),
        ("false&&(1/0==0)", ConstValue::Bool(false)),
        ("true||(1/0==0)", ConstValue::Bool(true)),
    ] {
        assert_eq!(
            constant_value(&format!("const A={expression}"), "A").0,
            value
        );
    }
    for (source, code, excerpt) in [
        ("const A=2147483647+1", "N3201", "2147483647+1"),
        ("const A=-2147483648/-1", "N3201", "-2147483648/-1"),
        ("const A=-2147483648%-1", "N3201", "-2147483648%-1"),
        ("const A=1/0", "N3201", "1/0"),
        ("const A=1%0", "N3201", "1%0"),
        (
            "const A=false&&f();func f()->bool{return true}",
            "N3201",
            "f()",
        ),
        ("const A=print(\"x\")", "N3201", "print(\"x\")"),
        ("const A=\"{1}\"", "N3201", "\"{1}\""),
        ("const A=f;func f(){}", "N1102", "f"),
        ("const A:bool=1", "N2101", "1"),
        ("const A=false&&(2147483648==0)", "N2102", "2147483648"),
    ] {
        let result = frontend(source);
        assert_eq!(codes(&result), [code], "{source}");
        assert_eq!(
            result
                .sources
                .slice(result.diagnostics()[0].primary.span)
                .unwrap(),
            excerpt
        );
    }
}

#[test]
fn public_global_resolution_kind_scope_reference_and_flags_are_verified() {
    let source = "const A=B;const B=2;func f(){let X=A}";
    for mutation in 0..4 {
        let (hir, mut resolved, _) = pass(source).semantic.unwrap();
        let global = resolved
            .definitions
            .iter()
            .position(|d| d.name == "A")
            .unwrap();
        let DefinitionKind::GlobalConst(id) = resolved.definitions[global].kind else {
            panic!("global")
        };
        match mutation {
            0 => resolved.definitions[global].kind = DefinitionKind::Local(id),
            1 => resolved.definitions[global].scope = nova_resolve::ScopeId(0),
            2 => resolved.definitions[global].constant = false,
            _ => resolved
                .references
                .iter_mut()
                .find(|r| matches!(r, Some(Resolution::Definition(def)) if def.0 == global))
                .map(|r| *r = None)
                .unwrap(),
        }
        assert_eq!(check(&hir, &resolved), Err(CheckError::InvalidResolution));
    }
}

#[test]
fn every_three_node_dependency_graph_agrees_with_reachability_cycle_oracle() {
    for bits in 0..512 {
        let mut reach = [[false; 3]; 3];
        let mut source = String::new();
        for (i, row) in reach.iter_mut().enumerate() {
            let mut terms = vec!["0".to_owned()];
            for (j, edge) in row.iter_mut().enumerate() {
                *edge = bits & (1 << (i * 3 + j)) != 0;
                if *edge {
                    terms.push(format!("C{j}"));
                }
            }
            source.push_str(&format!("const C{i}={}\n", terms.join("+")));
        }
        // Independent transitive-closure oracle, not a second DFS/SCC implementation.
        for k in 0..3 {
            for i in 0..3 {
                for j in 0..3 {
                    reach[i][j] |= reach[i][k] && reach[k][j];
                }
            }
        }
        let count = (0..3)
            .filter(|&i| reach[i][i] && !(0..i).any(|j| reach[i][j] && reach[j][i]))
            .count();
        let result = frontend(&source);
        assert_eq!(result.diagnostics().len(), count, "{source}");
        assert!(result
            .diagnostics()
            .iter()
            .all(|d| d.code.to_string() == "N3202"));
        let (_, resolved, checked) = result.semantic.unwrap();
        for i in 0..3 {
            let id = resolved
                .definitions
                .iter()
                .position(|d| d.name == format!("C{i}"))
                .unwrap();
            let value = &checked.const_values[id];
            if reach[i][i] {
                assert_eq!(
                    *value,
                    ConstEvaluation::Failed {
                        code: 3202,
                        nodes: 0
                    }
                );
            } else if (0..3).any(|j| reach[i][j] && reach[j][j]) {
                assert_eq!(*value, ConstEvaluation::Invalid);
            } else {
                assert!(matches!(
                    value,
                    ConstEvaluation::Value {
                        value: ConstValue::Int32(0),
                        ..
                    }
                ));
            }
        }
    }
}
