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
fn p09_float_context_peer_precision_and_raw_operation_width() {
    pass(include_str!("../../../examples/floats.nova"));
    for (source, name, expected) in [
        ("func f(a:float){let b=a+1}", "b", Type::Float64),
        ("func f(a:float){let b=a+1.0}", "b", Type::Float32),
        ("func f(a:double){let b=(1.0+2.0)+a}", "b", Type::Float64),
        ("func f(a:int16){let b=a+1.0}", "b", Type::Float32),
        ("func f(a:int){let b=a+1.0}", "b", Type::Float64),
        ("func f(){let b=1.0+2.0}", "b", Type::Float32),
    ] {
        let (_, resolved, checked) = pass(source).semantic.unwrap();
        let index = resolved
            .definitions
            .iter()
            .position(|d| d.name == name)
            .unwrap();
        assert_eq!(
            checked.types.get(checked.definition_types[index]),
            Some(expected),
            "{source}"
        );
    }
    let (hir, _, checked) = pass("func f(a:float,b:float){let x:double=a+b;let y:double=a+1.0}")
        .semantic
        .unwrap();
    let expressions: Vec<_> = hir
        .nodes()
        .iter()
        .enumerate()
        .filter(|(_, n)| matches!(n.kind, HirKind::Binary(_)))
        .map(|(i, _)| i)
        .collect();
    assert_eq!(
        checked.types.get(checked.type_table[expressions[0]]),
        Some(Type::Float32)
    );
    assert_eq!(
        checked.coercions[expressions[0]].and_then(|ty| checked.types.get(ty)),
        Some(Type::Float64)
    );
    assert_eq!(
        checked.types.get(checked.type_table[expressions[1]]),
        Some(Type::Float64)
    );
    // Expected double reaches REAL leaves directly, even with a float peer.
    pass("func f(a:float)->double{return a+1e100}");
    let (hir, _, checked) = pass("func f(a:int8)->double{return a+1}").semantic.unwrap();
    let binary = hir
        .nodes()
        .iter()
        .position(|n| matches!(n.kind, HirKind::Binary(_)))
        .unwrap();
    assert_eq!(
        checked.types.get(checked.type_table[binary]),
        Some(Type::Int8)
    );
    assert_eq!(
        codes(&frontend("func f(){let a:float=1.0;let b=a+1e100}")),
        ["N2102"]
    );
}

#[test]
fn p09_all_conversion_sites_enforce_whole_type_precision() {
    for (name, ty, ..) in INTEGER_CASES {
        for (dest, p) in [("float", 24), ("double", 53)] {
            let kind = ty.integer().unwrap();
            let allowed = kind.bits() - u32::from(kind.signed()) <= p;
            for source in [
                format!("func f(x:{name}){{let a:{dest}=x}}"),
                format!("func f(x:{name}){{var a:{dest}=1.0;a=x}}"),
                format!("func f(x:{name})->{dest}{{return x}}"),
                format!("func f(x:{dest}){{}}func g(x:{name}){{f(x)}}"),
                format!("const A:{name}=1;const B:{dest}=A"),
            ] {
                if allowed {
                    pass(&source);
                } else {
                    assert_eq!(codes(&frontend(&source)), ["N2101"], "{source}");
                }
            }
        }
    }
    pass("func f(x:float)->double{var y:double=x;y=x;return y}const C:double=1");
    for source in [
        "func f(){let x:float=1}",
        "func f(a:float){let b:float=a+1}",
        "func f(x:double)->float{return x}",
        "const C:int=1.0",
        "const C:char=1.0",
        "const C:bool=1.0",
        "const C:string=1.0",
    ] {
        assert_eq!(codes(&frontend(source)), ["N2101"], "{source}");
    }
}

#[test]
fn p09_float_const_ieee_results_nan_zero_and_short_circuit() {
    use nova_types::{FloatKind, FloatValue};
    for (expr, kind, bits) in [
        ("1.0/0.0", FloatKind::F32, 0x7f800000),
        ("-1.0/0.0", FloatKind::F32, 0xff800000),
        ("0.0/0.0", FloatKind::F32, 0x7fc00000),
        ("-(0.0/0.0)", FloatKind::F32, 0x7fc00000),
        ("-0.0", FloatKind::F32, 0x80000000),
        ("-1e-1000", FloatKind::F32, 0x80000000),
        ("1e-45", FloatKind::F32, 1),
        ("3.4028235e38*2.0", FloatKind::F32, 0x7f800000),
    ] {
        let (value, _) = constant_value(&format!("const C={expr}"), "C");
        assert_eq!(
            value,
            ConstValue::Float(FloatValue::from_bits(kind, bits).unwrap()),
            "{expr}"
        );
    }
    for (expr, expected) in [
        ("N==N", false),
        ("N!=N", true),
        ("N<N", false),
        ("N<=N", false),
        ("N>N", false),
        ("N>=N", false),
        ("-0.0==0.0", true),
        ("false&&(1.0/0.0==0.0)", false),
    ] {
        assert_eq!(
            constant_value(&format!("const N:double=0.0/0.0;const C={expr}"), "C").0,
            ConstValue::Bool(expected)
        );
    }
    assert_eq!(constant_value("const C:double=1.0+2.0", "C").1, 3);
    assert_eq!(constant_value("const A:int16=2;const C:float=A", "C").1, 1);
}

#[test]
fn p09_negative_operators_conditions_literal_spans_and_const_permission() {
    for expr in [
        "1.0%2.0",
        "!1.0",
        "1.0&&true",
        "true||1.0",
        "1.0+'a'",
        "1.0==true",
    ] {
        assert_eq!(
            codes(&frontend(&format!("func f(){{let x={expr}}}"))),
            ["N2101"],
            "{expr}"
        );
    }
    for source in ["func f(){if 1.0{}}", "func f(){while 1.0{}}"] {
        assert_eq!(codes(&frontend(source)), ["N3001"]);
    }
    let result = frontend("func f(){let x:float=1e100}");
    assert_eq!(codes(&result), ["N2102"]);
    let diagnostic = &result.diagnostics()[0];
    assert_eq!(
        result.sources.slice(diagnostic.primary.span).unwrap(),
        "1e100"
    );
    assert_eq!(
        result.sources.slice(diagnostic.secondary[0].span).unwrap(),
        "float"
    );
    for source in [
        "func f(){var a:float=1.0;const C=a}",
        "func f()->float{return 1.0}const C=f()",
        "func f()->float{return 1.0}const C=false&&(f()==1.0)",
        "const C=\"{1.0}\"",
    ] {
        assert_eq!(codes(&frontend(source)), ["N3201"], "{source}");
    }
    assert_eq!(
        codes(&frontend("const A:float=B;const B:float=A")),
        ["N3202"]
    );
    assert_eq!(codes(&frontend("const A=false&&(A||1.0==0.0)")), ["N3202"]);
}

#[test]
fn p09_direct_float_literal_bits_match_rational_oracle_in_annotations() {
    for row in include_str!("../../../tools/tests/fixtures/float-literals.tsv").lines() {
        let row: Vec<_> = row.split('\t').collect();
        let ty = if row[0] == "32" { "float" } else { "double" };
        let source = format!("const C:{ty}={}", row[1]);
        let ConstValue::Float(value) = constant_value(&source, "C").0 else {
            panic!("float const")
        };
        assert_eq!(value.bits(), u64::from_str_radix(row[2], 16).unwrap());
    }
}

#[test]
fn p09_float_const_budget_counts_nodes_and_not_implicit_conversion() {
    let expression = std::iter::repeat("1.0")
        .take(5000)
        .collect::<Vec<_>>()
        .join("+");
    let (_, count) = constant_value(&format!("const C:double=({expression})"), "C");
    assert_eq!(count, CONST_NODE_LIMIT);
    assert_eq!(
        codes(&frontend(&format!("const C:double=(({expression}))"))),
        ["N3202"]
    );
}

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
    {
        let ty = "never";
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

#[test]
fn p10_cast_isolation_numeric_pairs_and_diagnostics() {
    pass("func f(a:double,b:int)->bool{return a as int<b}");
    for source in [
        "func f(){let x=\"1\" as int}",
        "func f(){let x=() as int}",
        "func f(){let x=1 as void}",
        "func f(){let x=(f) as int}",
    ] {
        assert!(
            codes(&frontend(source)).contains(&"N2101".into()),
            "{source}"
        );
    }

    pass(include_str!("../../../examples/casts.nova"));
    for s in [
        "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64", "float32",
        "float64",
    ] {
        for d in [
            "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64", "float32",
            "float64",
        ] {
            pass(&format!("func f(v:{s})->{d}{{return v as {d}}}"));
        }
    }
    for (s, code) in [
        ("func f(){let x=2147483648 as int64}", "N2102"),
        ("func f(){let x=1e100 as double}", "N2102"),
        ("func f(){let x=-1 as uint8}", "N2101"),
        ("func f(){let x=true as int}", "N2101"),
        ("func f(){let x='a' as int}", "N2101"),
        ("func f(){let x=1 as bool}", "N2101"),
        ("func f(){let x=1 as char}", "N2101"),
        ("func f(){let x=1 as string}", "N2101"),
        ("func f(){let x=1 as ()}", "N2101"),
        ("func f(){let x=1 as Missing}", "N2001"),
        ("func f(){let x=1 as int()}", "N2101"),
        ("func f(){} func g(){let x=f as int}", "N2101"),
    ] {
        assert!(
            codes(&frontend(s)).contains(&code.into()),
            "{s}: {:?}",
            codes(&frontend(s))
        );
    }
    pass("func f(){let x=(-2147483648) as int64;let y:double=1.0 as float;let z=128 as int8}");
    let (v, n) = constant_value("const X:double=0.1 as double;func f(){}", "X");
    assert_eq!(
        v,
        ConstValue::Float(
            nova_types::FloatValue::from_bits(nova_types::FloatKind::F64, 0x3fb99999a0000000)
                .unwrap()
        )
    );
    assert_eq!(n, 2);
}
#[test]
fn p10_const_checks_skipped_rhs_and_cast_source_span() {
    assert_eq!(
        constant_value("const X=false&&((128 as int8)==0);func f(){}", "X").0,
        ConstValue::Bool(false)
    );
    for source in [
        "const X=128 as int8;func f(){}",
        "const X=(-1) as uint8;func f(){}",
        "const X=(0.0/0.0) as int;func f(){}",
        "const X:double=1e100;const Y=X as float;func f(){}",
    ] {
        assert!(
            codes(&frontend(source)).contains(&"N3201".into()),
            "{source}"
        );
    }
    assert!(codes(&frontend(
        "func f()->int{return 1}const X=false&&((f() as int)==0)"
    ))
    .contains(&"N3201".into()));
    assert!(codes(&frontend(
        "const A=false&&((B as int)==0);const B=C as int;const C=B as int;func f(){}"
    ))
    .contains(&"N3202".into()));
    let source = "const X=128 as int8;func f(){}";
    let result = frontend(source);
    let diag = result
        .diagnostics()
        .iter()
        .find(|d| d.code.to_string() == "N3201")
        .unwrap();
    assert_eq!(
        &source[diag.primary.span.start()..diag.primary.span.end()],
        "128 as int8"
    );
    assert!(!diag.secondary.is_empty());
}
#[test]
fn p10_const_budget_counts_casts_but_not_type_syntax() {
    let expression = "1".to_owned() + &" as int".repeat(CONST_NODE_LIMIT - 1);
    let source = format!("const X={expression};func f(){{}}");
    assert_eq!(constant_value(&source, "X").1, CONST_NODE_LIMIT);
    let source = format!("const X={expression} as int;func f(){{}}");
    assert!(codes(&frontend(&source)).contains(&"N3202".into()));
}

#[test]
fn p12_nominal_const_copy_fields_context_and_diagnostics() {
    let source="struct P{var x:int8;let c:char;let u:()} const V=P(1,'🙂',());const X=V.x;func make(p:P)->P{return p} func main(){var p=make(V);let old=p;p.x=p.x+1;const C=P(2,'x',());const Y=C.x;let f=P(3,'x',()).x}";
    let (_, resolved, checked) = pass(source).semantic.unwrap();
    for name in ["V", "C"] {
        let id = resolved
            .definitions
            .iter()
            .position(|d| d.name == name)
            .unwrap();
        assert!(matches!(
            checked.const_values[id],
            ConstEvaluation::Value {
                nodes: 4,
                value: ConstValue::Struct(_, _)
            }
        ));
    }
    for name in ["X", "Y"] {
        let id = resolved
            .definitions
            .iter()
            .position(|d| d.name == name)
            .unwrap();
        assert!(matches!(
            checked.const_values[id],
            ConstEvaluation::Value { nodes: 2, .. }
        ));
    }
    for (source, code) in [
        ("struct A{let x:A}", "N2101"),
        ("struct A{let b:B} struct B{let a:A}", "N2101"),
        ("struct A{let x:string}", "N1102"),
        ("struct A{let x:int;var x:int}", "N2002"),
        ("struct int{}", "N2002"),
        ("struct A{let x:Missing}", "N2001"),
        ("struct A{var x:int} func f(p:A){p.x=1}", "N3004"),
        ("struct A{var x:int} func f(){let p=A(1);p.x=2}", "N3004"),
        (
            "struct A{let x:int} struct B{let a:A} func f(){var b=B(A(1));b.a.x=2}",
            "N3004",
        ),
        ("struct A{let x:int} func f(){let a=A()}", "N2201"),
        (
            "struct A{let x:int} struct B{let x:int} func f(){let a:A=B(1)}",
            "N2101",
        ),
        ("struct A{let x:int} func f(){let A=1;let a=A(2)}", "N2101"),
        (
            "struct A{let x:int} func f(){let a=A(1);let b=a.y}",
            "N2001",
        ),
        ("struct A{let x:int} const V=A(1/0)", "N3201"),
        ("struct A{let x:int} const V=A(W.x);const W=V", "N3202"),
    ] {
        let result = frontend(source);
        assert!(
            codes(&result).contains(&code.into()),
            "{source}: {:?}",
            result.diagnostics()
        );
    }
}
#[test]
fn p12_const_budget_and_transitive_layout_resource_limits() {
    for (terms, ok) in [(5000, true), (5001, false)] {
        let source = format!(
            "struct P{{let x:int}} const V=P({})",
            vec!["0"; terms].join("+")
        );
        let result = frontend(&source);
        assert_eq!(result.passed(), ok, "{:?}", result.diagnostics());
        if !ok {
            assert!(codes(&result).contains(&"N3202".into()));
        }
    }
    for (depth, ok) in [(128, true), (129, false)] {
        let mut source = String::from("struct S0{} ");
        for i in 1..depth {
            source += &format!("struct S{i}{{let child:S{}}} ", i - 1);
        }
        let result = frontend(&source);
        assert_eq!(result.passed(), ok, "{:?}", result.diagnostics());
        if !ok {
            assert!(codes(&result).contains(&"N8901".into()));
        }
    }
    let mut source = String::from("struct S0{} ");
    for i in 1..17 {
        source += &format!("struct S{i}{{let left:S{};let right:S{}}} ", i - 1, i - 1);
    }
    assert!(codes(&frontend(&source)).contains(&"N8901".into()));
    let source = format!(
        "struct Huge{{{}}}",
        (0..1025)
            .map(|i| format!("let f{i}:int;"))
            .collect::<String>()
    );
    assert!(codes(&frontend(&source)).contains(&"N8901".into()));
}

fn p12_bundle(files: &[(&str, &str)]) -> (Module, Resolved, Checked) {
    let mut db = SourceDatabase::default();
    let mut modules = vec![];
    for &(name, source) in files {
        let file = db.add(format!("{name}.nova"), source.into()).unwrap();
        let l = lex(&db, file).unwrap();
        let p = parse(&db, file, &normalize_ends(&l.tokens)).unwrap();
        assert!(!l.has_errors() && !p.has_errors(), "{:?}", p.diagnostics);
        modules.push((name.into(), lower(&db, &p.arena, p.root).unwrap()));
    }
    let hir = Module::bundle(modules).unwrap();
    let resolved = resolve(&hir);
    let checked = check(&hir, &resolved).unwrap();
    for d in &checked.diagnostics {
        db.slice(d.primary.span).unwrap();
        for l in &d.secondary {
            db.slice(l.span).unwrap();
        }
    }
    (hir, resolved, checked)
}
#[test]
fn p12_type_import_alias_private_factories_and_dual_namespace_atomicity() {
    let (hir, resolved, checked) = p12_bundle(&[
        (
            "main",
            include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/main.nova"),
        ),
        (
            "geometry",
            include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/geometry.nova"),
        ),
    ]);
    assert!(!checked.has_errors(), "{:?}", checked.diagnostics);
    let scope = resolved.node_scopes[hir.units()[0].root.0].unwrap();
    assert_eq!(checked.structs.len(), 2);
    assert_eq!(
        resolved.scopes[scope.0].types["P"],
        resolved.scopes[resolved.node_scopes[hir.units()[1].root.0].unwrap().0].types["Pair"]
    );
    for (main, lib, code) in [
        (
            "use lib::P;func f(){let p=P(1)}",
            "public struct P{private let x:int}",
            Some("N2004"),
        ),
        (
            "use lib::make;func f(){let p=make();let x=p.x}",
            "private struct P{internal let x:int} public func make()->P{return P(1)}",
            None,
        ),
        (
            "use lib::make;func f(){let p=make();let x=p.x}",
            "public struct P{private let x:int} public func make()->P{return P(1)}",
            Some("N2004"),
        ),
        (
            "use lib::P as Q;func f(p:Q){let x=Q()}",
            "public struct P{} public func P()->int{return 42}",
            None,
        ),
        (
            "use lib::P as Q;func f(p:Q){}",
            "private struct P{} public func P(){}",
            Some("N2004"),
        ),
        (
            "use lib::P as Q;const Q=1;func f(){}",
            "public struct P{} public func P(){}",
            Some("N2002"),
        ),
        (
            "use lib::P as int;func f(){}",
            "public struct P{}",
            Some("N2002"),
        ),
    ] {
        let (hir, r, c) = p12_bundle(&[("main", main), ("lib", lib)]);
        if let Some(code) = code {
            assert!(
                c.diagnostics.iter().any(|d| d.code.to_string() == code),
                "{main}: {:?}",
                c.diagnostics
            );
        } else {
            assert!(!c.has_errors(), "{main}: {:?}", c.diagnostics);
        }
        if main.starts_with("use lib::P as Q") && code.is_some() {
            let scope = r.node_scopes[hir.units()[0].root.0].unwrap();
            assert!(!r.scopes[scope.0].types.contains_key("Q"));
            assert!(!r.scopes[scope.0]
                .definitions
                .get("Q")
                .is_some_and(|d| matches!(r.definitions[d.0].kind, DefinitionKind::Function(_))));
        }
    }
}
#[test]
fn p12_all_scalar_layouts_and_struct_count_boundary() {
    let (_,_,c)=pass("struct Empty{} struct Mixed{let empty:Empty;let unit:();let b:bool;let i:int16;let c:char;let f:float;let d:double;let n:uint64}").semantic.unwrap();
    let shape = c.structs.values().find(|s| s.name == "Mixed").unwrap();
    let l = shape.layout.as_ref().unwrap();
    assert_eq!(l.offsets, [0, 0, 0, 2, 4, 8, 16, 24]);
    assert_eq!((l.size, l.align, l.depth, l.occurrences), (32, 8, 2, 8));
    for (count, ok) in [(1024, true), (1025, false)] {
        let src = (0..count)
            .map(|i| format!("struct S{i}{{}} "))
            .collect::<String>();
        let result = frontend(&src);
        assert_eq!(result.passed(), ok, "{:?}", result.diagnostics());
        if !ok {
            assert!(codes(&result).contains(&"N8901".into()));
        }
    }
}

#[test]
fn p12_documented_negative_fixtures_have_exact_code_and_source_spans() {
    for (source,code,start,end) in [
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/string-field.nova"),"N1102",23,29),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/recursive-field.nova"),"N2101",24,28),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/immutable-root.nova"),"N3004",52,53),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/immutable-field.nova"),"N3004",54,55),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/constructor-arity.nova"),"N2201",46,49),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/missing-field.nova"),"N2001",62,63),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/nominal-mismatch.nova"),"N2101",73,77),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/shadowed-constructor.nova"),"N2101",57,61),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/aggregate-interpolation.nova"),"N2101",60,61),
        (include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/user-init.nova"),"N1102",11,15),
    ] {let result=frontend(source);assert!(result.diagnostics().iter().any(|d|d.code.to_string()==code && d.primary.span.start()==start && d.primary.span.end()==end),"{source}: {:?}",result.diagnostics());}
}

#[test]
fn p13_structural_identity_context_coercions_copy_and_excluded_operations() {
    pass(include_str!("../../../examples/tuples.nova"));
    pass("struct A{var x:int} func echo(t:(A,()))->(A,()){return t} func f(){var t=echo((A(1),()));let old=t;t.0.x=2}");
    pass("func echo(t:(int32,bool))->(int,bool){return t} func f(){let a:(int,bool)=echo((1,true));let b:(int32,bool)=a}");
    let (hir, _, checked) =
        pass("func f(a:int8,b:float){const C:(int8,)=(127,);let t:(int16,double)=(a,b)}")
            .semantic
            .unwrap();
    let tuple = hir
        .nodes()
        .iter()
        .find(|n| n.kind == HirKind::Tuple && n.children.len() == 2)
        .unwrap();
    assert_eq!(
        checked.coercions[tuple.children[0].0].and_then(|t| checked.types.get(t)),
        Some(Type::Int16)
    );
    assert_eq!(
        checked.coercions[tuple.children[1].0].and_then(|t| checked.types.get(t)),
        Some(Type::Float64)
    );
    for source in [
        "func f(){let a=(1,);let b:(int64,)=a}",
        "func f(){let a=(1,);let b:int=a}",
        "func f(){let a=(1,);let b=a==a}",
        "func f(){let a=(1,);let b=a as int}",
        "func f(){let a=(1,);let b=a.x}",
        "func f(){let a=1;let b=a.0}",
    ] {
        assert_eq!(codes(&frontend(source)), ["N2101"], "{source}");
    }
    assert_eq!(
        codes(&frontend(
            "func f(){let t=(1,);let x=t.999999999999999999999999999999999999999999}"
        )),
        ["N2001"]
    );
}
#[test]
fn p13_bundle_const_alias_private_factory_and_mixed_cycles() {
    let (_, _, checked) = p12_bundle(&[
        (
            "main",
            include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/main.nova"),
        ),
        (
            "tuples",
            include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/tuples.nova"),
        ),
    ]);
    assert!(!checked.has_errors(), "{:?}", checked.diagnostics);
    let (_, _, checked) = p12_bundle(&[
        (
            "main",
            "use lib::make;func f(){var t=(make(),);let old=t;t.0.x=2}",
        ),
        (
            "lib",
            "private struct Hidden{public var x:int} public func make()->Hidden{return Hidden(1)}",
        ),
    ]);
    assert!(!checked.has_errors(), "{:?}", checked.diagnostics);
    for source in [
        "struct A{let child:(A,)} func f(){}",
        "struct A{let child:(B,)} struct B{let child:(A,)} func f(){}",
    ] {
        assert!(codes(&frontend(source)).contains(&"N2101".into()));
    }
    assert_eq!(
        codes(&frontend("const A=(B,);const B=A.0;func f(){}")),
        ["N3202"]
    );
    assert_eq!(codes(&frontend("func f(){const x=(1/0,true)}")), ["N3201"]);
    assert_eq!(
        constant_value("func f(){const C=(3,true);const X=C.0}", "X"),
        (ConstValue::Int32(3), 2)
    );
}
#[test]
fn p13_const_and_mixed_aggregate_resource_boundaries() {
    let terms = vec!["1"; 5000].join("+");
    let source = format!("const C=({terms},);func f(){{}}");
    let (_, _, checked) = pass(&source).semantic.unwrap();
    assert!(checked
        .const_values
        .iter()
        .any(|c| matches!(c, ConstEvaluation::Value { nodes: 10000, .. })));
    let source = format!("const C=({terms}+1,);func f(){{}}");
    assert_eq!(codes(&frontend(&source)), ["N3202"]);
    pass(&format!(
        "func f(){{let t=({},)}}",
        vec!["()"; 1024].join(",")
    ));
    assert_eq!(
        codes(&frontend(&format!(
            "func f(){{let t=({},)}}",
            vec!["()"; 1025].join(",")
        ))),
        ["N8901"]
    );
    for (count, expected) in [(63, false), (64, true)] {
        let mut source = "struct S0{}".to_owned();
        for i in 1..=count {
            source += &format!("struct S{i}{{let next:(S{},)}}", i - 1);
        }
        source += "func f(){}";
        assert_eq!(
            codes(&frontend(&source)).contains(&"N8901".into()),
            expected
        );
    }
    let mut source = "struct S0{}".to_owned();
    for i in 1..=16 {
        source += &format!("struct S{i}{{let next:(S{p},S{p})}}", p = i - 1);
    }
    source += "func f(){}";
    assert!(codes(&frontend(&source)).contains(&"N8901".into()));
}
#[test]
fn p13_unique_shape_limit_reuse_and_source_order() {
    let mut source = String::new();
    for n in 0..4096 {
        let types = (0..13)
            .map(|bit| if n & (1 << bit) == 0 { "int" } else { "bool" })
            .collect::<Vec<_>>()
            .join(",");
        let values = (0..13)
            .map(|bit| if n & (1 << bit) == 0 { "1" } else { "true" })
            .collect::<Vec<_>>()
            .join(",");
        source += &format!("const C{n}:({types})=({values});");
    }
    let (_, _, checked) = pass(&(source.clone() + "func f(){let t=C0;let z=C4095}"))
        .semantic
        .unwrap();
    assert_eq!(checked.tuple_ids.len(), 4096);
    let types = (0..13)
        .map(|bit| {
            if 4096 & (1 << bit) == 0 {
                "int"
            } else {
                "bool"
            }
        })
        .collect::<Vec<_>>()
        .join(",");
    let values = (0..13)
        .map(|bit| if 4096 & (1 << bit) == 0 { "1" } else { "true" })
        .collect::<Vec<_>>()
        .join(",");
    // A function body precedes annotations that are collected in an earlier pass.
    // The limit diagnostic must still name the first excess shape in source order.
    let earlier = format!("func earlier(){{let t=({values})}}{source}");
    let result = frontend(&earlier);
    assert_eq!(codes(&result), ["N8901"]);
    assert_eq!(
        result.diagnostics()[0].primary.span.start(),
        earlier.find("const C4095:").unwrap() + "const C4095:".len()
    );
    let annotation_start = source.len() + "const OVER:".len();
    source += &format!("const OVER:({types})=({values});func f(){{}}");
    let result = frontend(&source);
    assert_eq!(codes(&result), ["N8901"]);
    assert_eq!(
        result.diagnostics()[0].primary.span.start(),
        annotation_start
    );
}
#[test]
fn p13_tuple_layout_has_independent_padding_and_zero_size_oracle() {
    let (_,_,checked)=pass("struct Empty{} const C:((),Empty,uint8,uint16,char,float,double,(int8,uint64),bool)=((),Empty(),1,2,'🙂',0.5,-0.0,(-1,3),true);func f(){}").semantic.unwrap();
    let shape = checked
        .tuple_ids
        .iter()
        .map(|sid| &checked.structs[sid])
        .find(|s| s.fields.len() == 9)
        .unwrap();
    let l = shape.layout.as_ref().unwrap();
    assert_eq!(l.offsets, [0, 0, 0, 2, 4, 8, 16, 24, 40]);
    assert_eq!((l.size, l.align, l.depth, l.occurrences), (48, 8, 2, 11));
}

#[test]
fn p13_documented_negative_fixtures_have_exact_codes_and_byte_spans() {
    for (source,code,start,end) in [
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/immutable-root.nova"),"N3004",37,38),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/immutable-field.nova"),"N3004",76,80),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/index-range.nova"),"N2001",55,56),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/arity-mismatch.nova"),"N2101",38,42),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/element-mismatch.nova"),"N2101",43,44),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/nominal-element.nova"),"N2101",81,85),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/string-element.nova"),"N1102",27,33),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/selector-exponent.nova"),"N1102",53,58),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/aggregate-interpolation.nova"),"N2101",48,49),
(include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/recursive-tuple-field.nova"),"N2101",23,24),
    ] {
        let result=frontend(source);assert_eq!(codes(&result),[code],"{source}");
        let span=result.diagnostics()[0].primary.span;assert_eq!((span.start(),span.end()),(start,end),"{source}");
    }
}

#[test]
fn p14_documented_negative_fixtures_preserve_exact_code_and_byte_spans() {
    for (source, code, start, end) in [
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/missing_variant.nova"
            ),
            "N3101",
            66,
            71,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/duplicate_arm.nova"
            ),
            "N3102",
            91,
            95,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/after_wildcard.nova"
            ),
            "N3102",
            85,
            89,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/invalid_scrutinee.nova"
            ),
            "N2101",
            55,
            56,
        ),
        (
            include_str!("../../../docs/development-v0.1/enum-proposal-fixtures/wrong_enum.nova"),
            "N2101",
            90,
            94,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/unknown_variant.nova"
            ),
            "N2001",
            74,
            81,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/payload_arity.nova"
            ),
            "N2201",
            84,
            94,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/immutable_binder.nova"
            ),
            "N3004",
            97,
            98,
        ),
        (
            include_str!("../../../docs/development-v0.1/enum-proposal-fixtures/binder_scope.nova"),
            "N2001",
            109,
            110,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/string_payload.nova"
            ),
            "N1102",
            46,
            52,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/recursive_value.nova"
            ),
            "N2101",
            46,
            47,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/duplicate_binder.nova"
            ),
            "N2002",
            100,
            101,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/duplicate_variant.nova"
            ),
            "N2002",
            47,
            48,
        ),
        (
            include_str!("../../../docs/development-v0.1/enum-proposal-fixtures/nullary_call.nova"),
            "N2201",
            71,
            77,
        ),
        (
            include_str!("../../../docs/development-v0.1/enum-proposal-fixtures/missing_bool.nova"),
            "N3101",
            49,
            54,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/enum-proposal-fixtures/guard_unsupported.nova"
            ),
            "N1102",
            81,
            83,
        ),
    ] {
        let result = frontend(source);
        assert!(
            result
                .diagnostics()
                .iter()
                .any(|d| d.code.to_string() == code
                    && (d.primary.span.start(), d.primary.span.end()) == (start, end)),
            "{source}\n{:?}",
            result.diagnostics()
        );
        assert!(!result.passed());
    }
}
#[test]
fn p14_copy_nominal_context_scope_flow_and_const_semantics() {
    pass("enum E{A(int8,(int,bool));B;} const C=E::A(7,(20,true));func f(e:E)->int{match e{E::A(x,t)=>{var y=x;y=y+1;return t.0},E::B=>{return 0}}}func main(){var e=C;let old=e;e=E::B;match old{E::A(x,_)=>{print(\"{x}\")},_=>{}}}");
    pass("func f(b:bool)->int{match b{false=>{return 1},true=>{return 2}}}func g(){var n=0;while n<3{n=n+1;match n==2{true=>{continue},false=>{if n==3{break}}}}}");
    pass("enum E{A(int);}func f(){let E=1;let x=E::A(2);let outer=3;match x{E::A(outer)=>{print(\"{outer}\")}}}");
    for (source, code) in [
        ("enum E{A;}enum F{A;}func f(){let x:E=F::A}", "N2101"),
        ("enum E{A;}func f(){let x=E::A==E::A}", "N2101"),
        ("enum E{A;}func f(){let x=E::A.0}", "N2101"),
        ("enum E{A;}func f(){print(\"{E::A}\")}", "N2101"),
        ("enum E{A(int8);}func f(){let x=E::A(128)}", "N2102"),
        (
            "enum E{A(int);}func f(){match E::A(1){E::A(x)=>{let x=2}}}",
            "N2002",
        ),
        (
            "func f(b:bool)->int{match b{true=>{return 1},false=>{}}}",
            "N3003",
        ),
        ("enum E{A(int);}const C=E::A(1/0);func f(){}", "N3201"),
        (
            "enum E{A(bool);}const C=E::A(false&&D);const D= C==C;func f(){}",
            "N3202",
        ),
        ("enum E{A((S,));}struct S{let e:E}func f(){}", "N2101"),
    ] {
        let result = frontend(source);
        assert!(
            codes(&result).contains(&code.into()),
            "{source}\n{:?}",
            result.diagnostics()
        );
    }
    let (value, nodes) = constant_value("enum E{A;}const X=E::A;func f(){}", "X");
    assert!(matches!(value,ConstValue::Enum(_,ref fields) if fields.is_empty()));
    assert_eq!(nodes, 1);
    let (value, nodes) =
        constant_value("enum E{A(int8,bool);}const X=E::A(7,true);func f(){}", "X");
    assert!(matches!(value,ConstValue::Enum(_,ref fields) if fields[0].ty()==Type::Int8));
    assert_eq!(nodes, 3);
}
#[test]
fn p14_enum_layout_has_independent_union_alignment_oracle() {
    let (_, _, c) =
        pass("struct Empty{}enum E{A;B(uint8,uint64,Empty,());C(char,bool);}func f(){}")
            .semantic
            .unwrap();
    let (eid, shape) = c.enums.iter().next().unwrap();
    let l = c.structs[&nova_types::StructId(eid.0)]
        .layout
        .as_ref()
        .unwrap();
    assert_eq!((l.size, l.align, l.depth, l.occurrences), (24, 8, 2, 6));
    assert_eq!(
        shape.variants[1].layout.as_ref().unwrap().offsets,
        [0, 8, 16, 16]
    );
    assert_eq!(shape.variants[2].layout.as_ref().unwrap().offsets, [0, 4]);
}

#[test]
fn p14_enum_resource_boundaries_and_const_node_budget() {
    let types = vec!["int"; 1024].join(",");
    let values = vec!["1"; 1024].join(",");
    pass(&format!(
        "enum E{{A({types});}}func f(){{let x=E::A({values})}}"
    ));
    let bad = frontend(&format!("enum E{{A({types},int);}}func f(){{}}"));
    assert!(codes(&bad).contains(&"N8901".into()));
    let diagnostic = bad
        .diagnostics()
        .iter()
        .find(|d| d.code.to_string() == "N8901")
        .unwrap();
    assert_eq!(
        diagnostic.primary.span.start(),
        format!("enum E{{A({types},").len()
    );
    assert!(diagnostic.notes.iter().any(|n| n.contains("1024")));

    let variants = (0..1024).map(|i| format!("V{i};")).collect::<String>();
    pass(&format!(
        "enum E{{{variants}}}func f(e:E){{match e{{_=>{{}}}}}}"
    ));
    assert!(
        codes(&frontend(&format!("enum E{{{variants}OVER;}}func f(){{}}")))
            .contains(&"N8901".into())
    );
    let declarations = (0..1024)
        .map(|i| format!("enum E{i}{{A;}}"))
        .collect::<String>();
    pass(&format!("{declarations}func f(){{}}"));
    assert!(codes(&frontend(&format!(
        "{declarations}enum OVER{{A;}}func f(){{}}"
    )))
    .contains(&"N8901".into()));
    let mut chain = "enum E0{A;}".to_string();
    for i in 1..128 {
        chain += &format!("enum E{i}{{A(E{});}}", i - 1);
    }
    pass(&format!("{chain}func f(){{}}"));
    assert!(codes(&frontend(&format!(
        "{chain}enum E128{{A(E127);}}func f(){{}}"
    )))
    .contains(&"N8901".into()));
    let mut tree = "enum E0{A;}".to_string();
    for i in 1..16 {
        tree += &format!("enum E{i}{{A(E{0},E{0});}}", i - 1);
    }
    pass(&format!("{tree}func f(){{}}"));
    assert!(codes(&frontend(&format!(
        "{tree}enum E16{{A(E15);B(E15);}}func f(){{}}"
    )))
    .contains(&"N8901".into()));
    // Enum constructor contributes one node and no separate qualified-path node.
    let sum = vec!["0"; 5000].join("+");
    let (_, n) = constant_value(
        &format!("enum E{{A(int);}}const C=E::A({sum});func f(){{}}"),
        "C",
    );
    assert_eq!(n, 10000);
    assert!(codes(&frontend(&format!(
        "enum E{{A(int);}}const C=E::A({sum}+0);func f(){{}}"
    )))
    .contains(&"N3202".into()));
    let arms = (0..1026).map(|_| "_=>{},").collect::<String>();
    let r = frontend(&format!("func f(){{match true{{{arms}}}}}"));
    assert!(codes(&r).contains(&"N8901".into()));
}

#[test]
fn p14_module_alias_identity_visibility_and_unused_cross_file_cycles() {
    let (_,_,c) = p12_bundle(&[
        ("main","use lib::Event as E;use lib::make;func f(){let E=1;let e:E=make();match e{E::A(x)=>{print(\"{x.value}\")},E::B=>{}}}"),
        ("lib","private struct Hidden{public let value:int}public enum Event{A(Hidden);B;}public func make()->Event{return Event::A(Hidden(1))}"),
    ]);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    for (main, lib, code) in [
        (
            "use lib::f;func main(){lib::f()}",
            "public func f(){}",
            "N2001",
        ),
        (
            "use lib::Event;func f(){}",
            "private enum Event{A;}",
            "N2004",
        ),
        (
            "use lib::Event;func f(e:Event){match e{Event::A(x)=>{print(\"{x.value}\")}}}",
            "private struct Hidden{private let value:int}public enum Event{A(Hidden);}",
            "N2004",
        ),
        (
            "use lib::B;public enum A{V((B,));}func f(){}",
            "use main::A;public struct B{let next:A}",
            "N2101",
        ),
        (
            "use lib::Event;const A=Event::V(B);use lib::B;func f(){}",
            "use main::A;public enum Event{V(bool);}public const B=false && A==A",
            "N3202",
        ),
    ] {
        let (_, _, c) = p12_bundle(&[("main", main), ("lib", lib)]);
        assert!(
            c.diagnostics.iter().any(|d| d.code.to_string() == code),
            "{main} / {lib}: {:?}",
            c.diagnostics
        );
    }
}

#[test]
fn p15_proposal_failures_have_exact_utf8_byte_spans() {
    for (source, code, start, end) in [
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/untyped_none.nova"), "N2103", 37, 41),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/untyped_result.nova"), "N2103", 37, 55),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/untyped_inner_none.nova"), "N2103", 50, 54),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/raw_wrapping.nova"), "N2101", 42, 43),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/payload_type.nova"), "N2101", 55, 59),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/existing_value_widen.nova"), "N2101", 73, 74),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/missing_option.nova"), "N3101", 34, 39),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/duplicate_none.nova"), "N3102", 71, 83),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/missing_result.nova"), "N3101", 46, 51),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/wrong_family.nova"), "N2101", 54, 66),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/wrong_error_type.nova"), "N2101", 68, 73),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/string_payload.nova"), "N1102", 35, 41),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/generic_arity.nova"), "N2101", 28, 44),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/unknown_variant.nova"), "N2001", 50, 57),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/immutable_binder.nova"), "N3004", 60, 61),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/binder_scope.nova"), "N2001", 79, 80),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/recursive_value.nova"), "N2101", 44, 45),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/user_generic.nova"), "N1102", 40, 43),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/try_unsupported.nova"), "N2103", 56, 74),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/nullary_call.nova"), "N2201", 42, 56),
        (include_str!("../../../docs/development-v0.1/option-result-proposal-fixtures/nullary_pattern_binder.nova"), "N2201", 62, 77),
    ] {
        let r = frontend(source);
        assert!(!r.passed());
        assert!(r.diagnostics().iter().any(|d| d.code.to_string() == code && (d.primary.span.start(), d.primary.span.end()) == (start, end)), "{source}\n{:?}", r.diagnostics());
    }
}
#[test]
fn p15_context_identity_shadow_const_and_match_flow() {
    pass("const X:int8?=Option::Some(7);const N:int8?=none;func f(v:int8?)->int8{match v{Option::Some(x)=>{return x},none=>{return 0}}}func g(){var x:Option<int8>=X;let old=x;x=N;let t:(int8?,)= (Option::Some(1),);let z:int??=Option::Some(none);let r:Result<(),int8>=Result::Success(());match r{Result::Success(_)=>{},Result::Error(e)=>{print(\"{e}\")}}}");
    pass("enum Option{Some(int8);None;}enum Result{Success(bool);Error;}func f(){let x=Option::Some(2);let y=Result::Success(true);let z:int?=none;match x{Option::Some(v)=>{},Option::None=>{}}match z{none=>{},_=>{}}}");
    pass("func Option()->int{return 2}func Some()->int{return 3}func f(){let Option=1;let x:Option<int8>=Option::Some(2);let y=Some();let z=Option::Some(4);let q:Option<int32>=z}");
    pass("struct S{var x:int8?}enum E{A(Result<int8,bool>);B;}func f(){var p=(S(none),);p.0.x=Option::Some(3);let e=E::A(Result::Success(1));match e{E::A(r)=>{match r{Result::Success(x)=>{},Result::Error(e)=>{}}},E::B=>{}}}");
    for (source, code) in [
        ("func f(){let x=Option}", "N2101"),
        ("func f(a:Option){}", "N2101"),
        ("func f(a:Array<int>){}", "N2001"),
        ("func f(a:int<int>){}", "N1102"),
        ("func f(a:Option<int>){let x=a==a}", "N2101"),
        ("func f(a:int?){print(\"{a}\")}", "N2101"),
        ("func f(a:int?){let x=a.0}", "N2101"),
        ("func f(){let x:Result<int,bool>=Option::Some(1)}", "N2101"),
        ("func f(){let x:Option<int>=Result::Error(true)}", "N2101"),
        ("func f(){let x=Option::Some()}", "N2201"),
        ("func f(){let x=Result::Success}", "N2201"),
        ("const C:int8?=Option::Some(127+1);func f(){}", "N3201"),
        (
            "const C:int?=Option::Some(D);const D:int=C;func f(){}",
            "N3202",
        ),
        (
            "func f(x:int?)->int{match x{Option::Some(y)=>{return y},none=>{}}}",
            "N3003",
        ),
    ] {
        let r = frontend(source);
        assert!(
            codes(&r).contains(&code.into()),
            "{source}\n{:?}",
            r.diagnostics()
        );
    }
    let (_, _, c) =
        pass("func f(a:int?,b:Option<int32>,c:Result<int8,bool>,d:Result<int8,bool>,e:int??){}")
            .semantic
            .unwrap();
    assert_eq!(c.sums.len(), 3);
    assert_eq!(c.definition_types[2], c.definition_types[3]);
    let (v, n) = constant_value("const C:int??=Option::Some(none);func f(){}", "C");
    assert!(
        matches!(v,ConstValue::Enum(_,ref f) if matches!(f[0],ConstValue::Enum(_,ref f) if f.is_empty()))
    );
    assert_eq!(n, 2);
    let (_, n) = constant_value(
        "const C:Result<(),bool>=Result::Success(());func f(){}",
        "C",
    );
    assert_eq!(n, 2);
}
#[test]
fn p15_layout_oracles_and_unique_specialization_limits() {
    let (_,_,c)=pass("func f(a:()?,b:Option<(int8,uint64)>,c:Option<Option<(int8,uint64)>>,d:Result<(),uint64>){}").semantic.unwrap();
    let layouts = c
        .sums
        .iter()
        .map(|(id, key)| {
            (
                key.clone(),
                c.structs[&nova_types::StructId(id.0)]
                    .layout
                    .clone()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(layouts.len(), 4);
    for (key, l) in &layouts {
        match (key.family, key.arguments.as_slice()) {
            (nova_types::SumFamily::Option, [Type::Unit]) => assert_eq!((l.size, l.align), (4, 4)),
            (nova_types::SumFamily::Option, [Type::Tuple(_)]) => {
                assert_eq!((l.size, l.align), (24, 8))
            }
            (nova_types::SumFamily::Option, [Type::Enum(_)]) => {
                assert_eq!((l.size, l.align), (32, 8))
            }
            (nova_types::SumFamily::Result, [Type::Unit, Type::UInt64]) => {
                assert_eq!((l.size, l.align), (16, 8))
            }
            _ => panic!("unexpected {key:?}"),
        }
    }
    let declarations = (0..512)
        .map(|i| format!("struct S{i}{{}}"))
        .collect::<String>();
    let payloads = [
        "int8", "uint8", "int16", "uint16", "int32", "uint32", "int64", "uint64",
    ];
    let types = (0..512)
        .flat_map(|i| payloads.map(move |p| format!("func f{i}_{p}(x:Result<S{i},{p}>){{}}")))
        .collect::<String>();
    pass(&format!("{declarations}{types}"));
    let source = format!("{declarations}{types}func over(x:Option<S0>){{}}");
    let r = frontend(&source);
    let diagnostics = r.diagnostics();
    let d = diagnostics
        .iter()
        .find(|d| d.code.to_string() == "N8901")
        .unwrap();
    assert_eq!(d.primary.span.start(), source.find("Option<S0>").unwrap());
    assert!(d.notes.iter().any(|n| n.contains("4096")));
    let sum = vec!["0"; 5000].join("+");
    let (_, n) = constant_value(
        &format!("const C:int?=Option::Some({sum});func f(){{}}"),
        "C",
    );
    assert_eq!(n, 10000);
    assert!(codes(&frontend(&format!(
        "const C:int?=Option::Some({sum}+0);func f(){{}}"
    )))
    .contains(&"N3202".into()));
}
#[test]
fn p15_module_alias_private_factory_and_failed_import_shadow() {
    let (_, _, c) = p12_bundle(&[
        (
            "main",
            include_str!(
                "../../../docs/development-v0.1/option-result-proposal-fixtures/main.nova"
            ),
        ),
        (
            "values",
            include_str!(
                "../../../docs/development-v0.1/option-result-proposal-fixtures/values.nova"
            ),
        ),
    ]);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    let (_, _, c) = p12_bundle(&[
        (
            "main",
            "use lib::Choice as Option;func f(){let x=Option::Some(1);let y:int?=none;}",
        ),
        ("lib", "public enum Choice{Some(int8);None;}"),
    ]);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    let (_, _, c) = p12_bundle(&[
        ("main", "use lib::Option;func f(x:Option<int>){}"),
        ("lib", "private enum Option{Some(int);None;}"),
    ]);
    assert!(c.diagnostics.iter().any(|d| d.code.to_string() == "N2004"));
    assert!(!c.diagnostics.iter().any(|d| d.code.to_string() == "N2001"));
    let (_,_,c)=p12_bundle(&[("main","use lib::make;func f(){match make(){Option::Some(x)=>{print(\"{x.value}\")},none=>{}}}"),("lib","private struct H{private let value:int}public func make()->H?{return Option::Some(H(1))}")]);
    assert!(c.diagnostics.iter().any(|d| d.code.to_string() == "N2004"));
}

#[test]
fn p15_mixed_cycles_depth_occurrence_and_first_origin_are_bounded() {
    pass("func f(a:Option /*a*/ <int /*b*/, /*c*/ > /*d*/ ?){let x:int /*e*/ ?=none}");
    let mut chain = "struct S0{}".to_string();
    for i in 1..64 {
        chain += &format!("struct S{i}{{let x:S{}?}}", i - 1);
    }
    pass(&format!("{chain}func f(x:S63?){{}}"));
    let r = frontend(&format!("{chain}struct S64{{let x:S63?}}func f(){{}}"));
    assert!(codes(&r).contains(&"N8901".into()));
    let mut tree = "struct S0{let x:int?}".to_string();
    for i in 1..14 {
        tree += &format!("struct S{i}{{let a:S{0}?;let b:S{0}?}}", i - 1);
    }
    pass(&format!("{tree}func f(){{}}"));
    assert!(codes(&frontend(&format!(
        "{tree}struct S14{{let a:S13?;let b:S13?}}func f(){{}}"
    )))
    .contains(&"N8901".into()));
    for source in [
        "struct S{let x:Result<(),S>}func f(){}",
        "struct S{let x:(S?,)}func f(){}",
        "enum E{A(Option<S>);}struct S{let x:Result<E,bool>}func f(){}",
    ] {
        assert!(
            codes(&frontend(source)).contains(&"N2101".into()),
            "{source}"
        );
    }
    let (hir, _, c) = pass("func early(){let x=Option::Some(1)}func later(x:int?){}")
        .semantic
        .unwrap();
    let eid = *c.sums.keys().next().unwrap();
    let at = c.sum_origins[&eid];
    assert!(matches!(hir.nodes()[at.0].kind, HirKind::Call));
    assert_eq!(hir.nodes()[at.0].span.start(), 19);
    for source in [
        "func f(x:Option<string>){}",
        "func f(x:Result<int,string>){}",
        "func f(){let x=Option::Some(\"text\")}",
        "func f(){let x=Option<int>::Some(1)}",
    ] {
        assert!(
            codes(&frontend(source)).contains(&"N1102".into()),
            "{source}"
        );
    }
}

#[test]
fn p16_contract_diagnostics_exact_utf8_spans_and_cascade_suppression() {
    let cases: &[(&str, &str, usize, usize, &[&str])] = &[
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/non_result_operand.nova"
            ),
            "N2101",
            56,
            57,
            &[],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/option_operand.nova"
            ),
            "N2101",
            62,
            63,
            &[],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/nominal_lookalike.nova"
            ),
            "N2101",
            92,
            93,
            &[],
        ),
        (
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/unit_return.nova"),
            "N3002",
            52,
            55,
            &[],
        ),
        (
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/option_return.nova"),
            "N3002",
            58,
            61,
            &[],
        ),
        (
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/error_width.nova"),
            "N2101",
            71,
            74,
            &[],
        ),
        (
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/error_nominal.nova"),
            "N2101",
            88,
            91,
            &[],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/return_raw_success.nova"
            ),
            "N2101",
            71,
            76,
            &[],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/missing_return.nova"
            ),
            "N3003",
            63,
            76,
            &[],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/untyped_constructor.nova"
            ),
            "N2103",
            60,
            78,
            &[],
        ),
        (
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/global_const.nova"),
            "N3201",
            71,
            74,
            &["N3002"],
        ),
        (
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/local_const.nova"),
            "N3201",
            72,
            75,
            &["N3002"],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/skipped_const_rhs.nova"
            ),
            "N3201",
            82,
            85,
            &["N3002"],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/wrong_condition.nova"
            ),
            "N3001",
            67,
            72,
            &[],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/success_narrowing.nova"
            ),
            "N2101",
            78,
            83,
            &[],
        ),
        (
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/unit_value.nova"),
            "N2101",
            73,
            78,
            &[],
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/try-proposal-fixtures/nested_error_mismatch.nova"
            ),
            "N2101",
            83,
            86,
            &[],
        ),
        (
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/missing_name.nova"),
            "N2001",
            56,
            63,
            &["N2101", "N3002", "N2103"],
        ),
    ];
    for &(source, code, start, end, forbidden) in cases {
        let result = frontend(source);
        assert!(!result.passed(), "{source}");
        assert!(
            result
                .diagnostics()
                .iter()
                .any(|d| d.code.to_string() == code
                    && (d.primary.span.start(), d.primary.span.end()) == (start, end)),
            "{source}\n{:?}",
            result.diagnostics()
        );
        assert!(
            !result
                .diagnostics()
                .iter()
                .any(|d| forbidden.contains(&d.code.to_string().as_str())),
            "{source}\n{:?}",
            result.diagnostics()
        );
        if code == "N3201" {
            let d = result
                .diagnostics()
                .iter()
                .find(|d| d.code.to_string() == code)
                .unwrap();
            assert!(d.secondary.iter().any(|l| result
                .sources
                .slice(l.span)
                .unwrap()
                .starts_with("const")));
        }
    }
}
#[test]
fn p16_exact_normalized_error_context_nested_flow_and_aliases() {
    for source in [
        "func f(r:Result<int8,Option<int>>)->Result<int16,int?>{let n:int16=try r;return Result::Success(n)}",
        "func f(r:Result<Result<int8,bool>,bool>)->Result<int8,bool>{return try r}",
        "func f(r:Result<Result<int8,bool>,bool>)->Result<int16,bool>{let n:int16=try try r;return Result::Success(n)}",
        "func f(r:Result<(),()>)->Result<(),()>{try r;return Result::Success(())}",
        "func f(r:Result<int,bool>)->Result<int,bool>{let Result=3;let n=try r;return Result::Success(n)}",
        "const R:Result<int,bool>=Result::Error(true);func f()->Result<int,bool>{let n=try R;return Result::Success(n)}",
        "func f(r:Result<int,bool>,b:bool)->Result<int,bool>{match b{true=>{return Result::Success(try r)},false=>{return Result::Error(false)}}}",
    ] { pass(source); }
    let r=pass("func f(r:Result<int8,bool>)->Result<int16,bool>{let n:int16=try r;return Result::Success(n)}");
    let (hir, _, checked) = r.semantic.unwrap();
    let at = hir
        .nodes()
        .iter()
        .position(|n| matches!(n.kind, HirKind::Try { .. }))
        .unwrap();
    assert_eq!(checked.types.get(checked.type_table[at]), Some(Type::Int8));
    assert_eq!(
        checked.coercions[at].and_then(|t| checked.types.get(t)),
        Some(Type::Int16)
    );
    for source in [
        "func f(r:Result<int,int8>)->Result<int,int>{let n=try r;return Result::Success(n)}",
        "func f()->Result<int8,bool>{let n:int8=try Result::Error(true);return Result::Success(n)}",
        "func f(r:Result<int,bool>)->Result<int,bool>{let n=try r as int8;return Result::Success(n)}",
    ] { assert!(!frontend(source).passed(),"{source}"); }
    pass("func f(r:Result<int,bool>)->Result<int8,bool>{let n=(try r) as int8;return Result::Success(n)}");
}
#[test]
fn p16_const_cycles_and_normal_path_return_remain_required() {
    for source in [
        "const R:Result<bool,bool>=Result::Success(B);const B=false&&try R;func main(){}",
        "const R:Result<bool,bool>=Result::Error(B);const B=true||try R;func main(){}",
    ] {
        let r = frontend(source);
        assert!(codes(&r).contains(&"N3202".into()), "{:?}", r.diagnostics());
    }
    let r = frontend(
        "const R:Result<int,bool>=Result::Error(true);func f()->Result<int,bool>{let n=try R}",
    );
    assert!(codes(&r).contains(&"N3003".into()));
}

#[test]
fn p17_diagnostics_exact_spans_mapping_and_error_cascades() {
    for (source,code,start,end) in [
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/unknown_label.nova"), "N2201", 35, 40),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/duplicate_label.nova"), "N2201", 43, 48),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/positional_collision.nova"), "N2201", 37, 42),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/positional_after_named.nova"), "N2201", 42, 43),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/missing_argument.nova"), "N2201", 36, 42),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/excess_positional.nova"), "N2201", 33, 34),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/wrong_expected_type.nova"), "N2101", 42, 43),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/mapped_literal_range.nova"), "N2102", 47, 50),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/builtin_label.nova"), "N2201", 18, 23),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/struct_label.nova"), "N2201", 42, 43),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/enum_label.nova"), "N2201", 40, 45),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/sum_label.nova"), "N2201", 52, 57),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/unresolved_callee.nova"), "N2001", 12, 19),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/unicode_unknown.nova"), "N2201", 58, 67),
        (include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/const_call.nova"), "N3201", 47, 57),
    ] {
        let result=frontend(source);assert!(!result.passed(),"{source}");
        assert!(result.diagnostics().iter().any(|d|d.code.to_string()==code&&d.primary.span.start()==start&&d.primary.span.end()==end),"{source} {:?}",result.diagnostics());
    }
    for source in [
        "func main(){missing(value:1)}",
        "func f(a:int8){}func main(){f(unknown:1000)}",
    ] {
        let result = frontend(source);
        assert!(!codes(&result).iter().any(|c| c == "N2101" || c == "N2102"));
    }
    assert_eq!(codes(&frontend("func main(){missing(value:1)}")), ["N2001"]);
    assert_eq!(
        codes(&frontend(
            "func f(a:int8,b:int8){}func main(){f(unknown:1)}"
        )),
        ["N2201"]
    );
    assert_eq!(
        codes(&frontend("func f(a:int8){}func main(){let f=1;f(a:1)}")),
        ["N2101"]
    );
}
#[test]
fn p17_context_forward_shadow_alias_spelling_and_const_are_preserved() {
    for source in [
        include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/function_print_shadow.nova"),
        include_str!("../../../docs/development-v0.1/named-arguments-proposal-fixtures/forward_recursive_grouped.nova"),
        "func f(a:int8?,b:(int8,bool),c:Result<char,()>){ }func main(){f(c:Result::Success('🙂'),b:(1,true),a:none)}",
        "func f(int:int8,int32:int16,float:float,double:double,_:bool){}func main(){f(int32:300,int:1,double:1e100,float:0.5,_:true)}",
    ] { pass(source); }
    for source in [
        "func f(a:int8)->int8{return a}func main(){const x=f(a:1)}",
        "func f(a:int8)->bool{return true}func main(){const x=false&&f(a:1)}",
    ] {
        assert_eq!(codes(&frontend(source)), ["N3201"]);
    }
    let (hir, _, checked) = pass("func f(a:int8,b:int16){}func main(){f(b:300,a:-128)}")
        .semantic
        .unwrap();
    let mapping = checked.named_calls.iter().flatten().next().unwrap();
    assert_eq!(mapping.parameters, [1, 0]);
    assert!(matches!(
        hir.nodes()[mapping.arguments[0].0].kind,
        HirKind::NamedArgument { .. }
    ));
}

#[test]
fn p18_diagnostics_exact_spans_and_default_error_cascades() {
    for (source,code,start,end,forbidden) in [
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/unknown_label.nova"), "N2201", 40, 45, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/duplicate_label.nova"), "N2201", 48, 53, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/positional_after_named.nova"), "N2201", 49, 50, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/missing_required.nova"), "N2201", 41, 44, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/excess_positional.nova"), "N2201", 38, 44, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/default_type.nova"), "N2101", 18, 19, &["N3201"]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/default_range.nova"), "N2102", 18, 21, &["N3201"]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/unused_overflow.nova"), "N3201", 18, 23, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/default_divide_zero.nova"), "N3201", 19, 22, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/parameter_reference.nova"), "N2001", 28, 32, &["N3201"]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/self_reference.nova"), "N2001", 18, 23, &["N3201"]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/caller_local_reference.nova"), "N2001", 18, 28, &["N3201"]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/default_function_call.nova"), "N3201", 46, 49, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/default_interpolation.nova"), "N3201", 20, 27, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/skipped_function_call.nova"), "N3201", 66, 77, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/default_try.nova"), "N3201", 79, 82, &["N3002"]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/undefined_default.nova"), "N2001", 18, 27, &["N3201"]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/const_function_call.nova"), "N3201", 56, 59, &[] as &[&str]),
        (include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/builtin_label.nova"), "N2201", 44, 49, &[] as &[&str]),
    ] {
        let r=frontend(source);assert!(!r.passed(),"{source}");
        assert!(r.diagnostics().iter().any(|d|d.code.to_string()==code&&d.primary.span.start()==start&&d.primary.span.end()==end),"{source} {:?}",r.diagnostics());
        assert!(!r.diagnostics().iter().any(|d|forbidden.contains(&d.code.to_string().as_str())),"{source} {:?}",r.diagnostics());
        if code=="N3201" && !source.contains("const VALUE") {
            let d=r.diagnostics().iter().find(|d|d.code.to_string()==code).unwrap();
            assert!(d.secondary.iter().any(|l|r.sources.slice(l.span).unwrap().starts_with("value:")));
        }
    }
    let r = frontend("func f(value:int8=127+1){}func main(){f(0);f(value:1)}");
    assert_eq!(codes(&r), ["N3201"]);
    assert_eq!(
        codes(&frontend(
            "func f(a:int8=1,b:int8=2){}func main(){f(other:1000)}"
        )),
        ["N2201"]
    );
}
#[test]
fn p18_declaration_scope_type_context_forward_aliases_and_const_rules() {
    for source in [
        include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/unicode_print_shadow.nova"),
        include_str!("../../../docs/development-v0.1/default-arguments-proposal-fixtures/forward_recursive_grouped.nova"),
        "func f(a:int8=BASE,b:bool=false&&(1/0==0)){}const BASE:int8=7;func main(){f()}",
        "struct S{let n:int8;}enum E{One(S)}func f(s:S=S(7),e:E=E::One(S(8)),r:Result<int8,bool>=Result::Success(9),o:int8?=none,u:()=(),text:string=\"a\\0🙂\"){}func main(){f()}",
        "func f(a:int8=-128,b:uint8=255,c:int16=-32768,d:uint16=65535,e:int32=-2147483648,f:uint32=4294967295,g:int64=-9223372036854775808,h:uint64=18446744073709551615,i:float=0.5,j:double=1e100,k:char='🙂',flag:bool=true){}func main(){f()}",
        "func f(value:int8=127 as int8)->int8{return value}func main(){let n=f()}",
    ] {pass(source);}
    let (hir,_,c)=p12_bundle(&[
        ("main","use lib::f as renamed;const VALUE:int8=99;func main(){let VALUE:int8=100;renamed()}"),
        ("lib","use other::VALUE as imported;private const VALUE:int8=7;public func f(VALUE:int8=VALUE,other:int8=imported){}"),
        ("other","public const VALUE:int8=8"),
    ]);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    let values = c
        .defaults
        .iter()
        .flatten()
        .map(|d| match &d.evaluation {
            ConstEvaluation::Value {
                value: ConstValue::Integer(v),
                ..
            } => v.value(),
            _ => panic!("unexpected {:?}", d.evaluation),
        })
        .collect::<Vec<_>>();
    assert_eq!(values, [7, 8]);
    for d in c.defaults.iter().flatten() {
        assert_eq!(
            hir.nodes()[d.argument.initializer.0].span.file().as_u32(),
            1
        );
    }
    for source in [
        "const A:int8=B;const B:int8=A;func f(v:int8=A){}",
        "const A:int8=127+1;func f(v:int8=A){}",
    ] {
        let r = frontend(source);
        assert_eq!(
            r.diagnostics()
                .iter()
                .filter(|d| d.code.to_string() == "N3201")
                .count(),
            usize::from(source.contains("127+1"))
        );
        assert!(!r.passed());
    }
}
#[test]
fn p18_default_budget_is_per_declaration_and_includes_skipped_rhs() {
    let terms = vec!["1"; 5000].join("+");
    let r = pass(&format!(
        "func f(a:int=({terms}),b:int=({terms})){{}}func main(){{f();f();f(0,0)}}"
    ));
    let (_, _, c) = r.semantic.unwrap();
    assert!(c
        .defaults
        .iter()
        .flatten()
        .all(|d| matches!(d.evaluation, ConstEvaluation::Value { nodes: 10000, .. })));
    let too_many = vec!["1"; 5001].join("+");
    for expr in [too_many, format!("false&&({terms}==0)")] {
        let ty = if expr.starts_with("false") {
            "bool"
        } else {
            "int"
        };
        let r = frontend(&format!("func f(value:{ty}={expr}){{}}func main(){{}}"));
        let d = r
            .diagnostics()
            .iter()
            .find(|d| d.code.to_string() == "N3202")
            .unwrap();
        assert!(d.notes.iter().any(|n| n.contains("10000")));
        assert!(!codes(&r).contains(&"N3201".into()));
    }
}

#[test]
fn p19_range_loop_fixture_diagnostics_exact_spans_and_error_cascades() {
    for (source,code,start,end,forbidden) in [
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/float_bound.nova"),"N2101",22,25,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/string_bound.nova"),"N2101",30,35,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/bool_bound.nova"),"N2101",22,26,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/char_bound.nova"),"N2101",22,25,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/unit_bound.nova"),"N2101",22,24,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/aggregate_bound.nova"),"N2101",22,27,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/no_common_integer.nova"),"N2101",55,66,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/peer_literal_range.nova"),"N2102",40,43,&["N2101"] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/undefined_bound.nova"),"N2001",22,29,&["N2101"] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/binder_not_in_bound_scope.nova"),"N2001",26,31,&["N2101"] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/binder_outside.nova"),"N2001",45,48,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/immutable_binder.nova"),"N3004",38,43,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/duplicate_binder.nova"),"N2002",42,47,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/chained_range.nova"),"N1103",32,39,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/range_value.nova"),"N1102",25,30,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/iterable_value.nova"),"N1102",22,27,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/wildcard_binder.nova"),"N1102",17,18,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/loop_missing_return.nova"),"N3003",14,35,&[] as &[&str])
    ] {
        let result=frontend(source);assert!(!result.passed(),"{source}");
        assert!(result.diagnostics().iter().any(|d|d.code.to_string()==code&&d.primary.span.start()==start&&d.primary.span.end()==end),"{source} {:?}",result.diagnostics());
        assert!(!result.diagnostics().iter().any(|d|forbidden.contains(&d.code.to_string().as_str())),"{source} {:?}",result.diagnostics());
    }
}
#[test]
fn p19_integer_peer_promotion_binder_scope_and_conservative_return() {
    for (name, ty, min, max) in INTEGER_CASES {
        let r=pass(&format!("func f(){{const LOW:{name}={min};const HIGH:{name}={max};for value in LOW until HIGH {{let n:{name}=value;break}}for value in HIGH through HIGH {{continue}}}}"));
        let (_, _, c) = r.semantic.unwrap();
        assert!(c
            .ranges
            .iter()
            .flatten()
            .all(|i| c.types.get(i.ty) == Some(ty)));
    }
    for source in [include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/unicode_scope_and_snapshots.nova"),include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/integer_edges_and_mixed_jumps.nova"),
        "func f()->int{loop{return 1}return 2}","func f()->int{for i in 0 through 1{return i}return 2}",
        "func f(){for i in 0 until 1 {break;let never=1}loop {break;continue}}",
        "func f(){let hi:int8=3;for i in (0+1) until hi {let v:int8=i}let lo:int8=-1;let end:uint8=2;for i in lo until end {let v:int16=i}}",
    ] {pass(source);}
    let (_, _, c) = p12_bundle(&[
        (
            "main",
            include_str!("../../../docs/development-v0.1/range-loop-proposal-fixtures/main.nova"),
        ),
        (
            "helpers",
            include_str!(
                "../../../docs/development-v0.1/range-loop-proposal-fixtures/helpers.nova"
            ),
        ),
    ]);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    for source in [
        "func f()->int{for i in 0 through 1{return i}}",
        "func f()->int{loop{break}}",
    ] {
        assert!(codes(&frontend(source)).contains(&"N3003".into()));
    }
    let r = pass("func f(){var value=3;for value in value until 4 {if true {let value=99}}}");
    let (_, resolved, c) = r.semantic.unwrap();
    let binders = resolved
        .definitions
        .iter()
        .enumerate()
        .filter(|(_, d)| d.name == "value" && !d.mutable)
        .collect::<Vec<_>>();
    assert_eq!(binders.len(), 2);
    assert_eq!(c.ranges.iter().flatten().count(), 1);
    assert_eq!(
        codes(&frontend(
            "func f(){for i in 0 until 1 {break;let never=missing}}"
        )),
        ["N2001"]
    );
}

#[test]
fn p20_exists_fixture_diagnostics_exact_utf8_spans_and_cascades() {
    for (source,code,start,end,forbidden) in [
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/integer_operand.nova"),"N2101",55,56,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/float_operand.nova"),"N2101",55,58,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/bool_operand.nova"),"N2101",55,60,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/string_operand.nova"),"N2101",55,60,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/unit_operand.nova"),"N2101",69,74,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/tuple_operand.nova"),"N2101",55,63,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/user_enum.nova"),"N2101",85,98,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/result_operand.nova"),"N2101",101,106,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/shadowed_option.nova"),"N2101",87,102,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/none_without_context.nova"),"N2103",55,59,&["N2101","N3201"] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/nested_none_without_context.nova"),"N2103",68,72,&["N2101","N3201"] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/repeated_exists.nova"),"N2101",76,88,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/bool_numeric_cast.nova"),"N2101",76,95,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/try_precedence.nova"),"N2101",153,160,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/undefined_operand.nova"),"N2001",55,64,&["N2101","N3201"] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/const_runtime_call.nova"),"N3201",83,90,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/skipped_const_runtime_call.nova"),"N3201",92,99,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/default_runtime_call.nova"),"N3201",85,92,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/newline_before_exists.nova"),"N1102",93,99,&[] as &[&str]),
(include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/bool_not_callable.nova"),"N2101",76,88,&[] as &[&str]),
    ] {
        let r=frontend(source);assert!(!r.passed(),"{source}");
        assert!(r.diagnostics().iter().any(|d|d.code.to_string()==code&&d.primary.span.start()==start&&d.primary.span.end()==end),"{source} {:?}",r.diagnostics());
        assert!(!r.diagnostics().iter().any(|d|forbidden.contains(&d.code.to_string().as_str())),"{source} {:?}",r.diagnostics());
    }
}
#[test]
fn p20_exists_copy_families_context_isolation_truthiness_and_scopes() {
    let (_, _, checked) = p12_bundle(&[
        (
            "main",
            include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/main.nova"),
        ),
        (
            "helpers",
            include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/helpers.nova"),
        ),
    ]);
    assert!(!checked.has_errors(), "{:?}", checked.diagnostics);
    for source in [
        include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/newline_and_shadow.nova"),
        include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/skipped_abort.nova"),
        include_str!("../../../docs/development-v0.1/exists-proposal-fixtures/operand_abort.nova"),
        "func f(x:int?)->bool{return x exists}func g(x:int?){if x exists {}while x exists {break}loop{if x exists{break}break}for i in 0 until 1 {if x exists{continue}}match x exists {true=>{},false=>{}}}",
        "const ABSENT:int?=none;const A=Option::Some(0) exists;const B=Option::Some(false) exists;const C=Option::Some(()) exists;const D:int??=Option::Some(none);const E=D exists;const F=ABSENT exists;func f(x:bool=A){}",
    ]{pass(source);}
    for (ty, value) in [
        ("int8", "-128"),
        ("uint8", "255"),
        ("int16", "-32768"),
        ("uint16", "65535"),
        ("int", "0"),
        ("uint32", "0"),
        ("int64", "0"),
        ("uint64", "0"),
        ("float", "0.0"),
        ("double", "-0.0"),
        ("bool", "false"),
        ("char", "'🙂'"),
        ("()", "()"),
        ("(int,bool)", "(0,false)"),
    ] {
        let r=pass(&format!("const O:{ty}?=Option::Some({value});const PRESENT=O exists;func f(x:{ty}?)->bool{{return x exists}}"));
        let (_, res, c) = r.semantic.unwrap();
        let id = res
            .definitions
            .iter()
            .position(|d| d.name == "PRESENT")
            .unwrap();
        assert!(matches!(
            c.const_values[id],
            ConstEvaluation::Value {
                value: ConstValue::Bool(true),
                ..
            }
        ));
    }
    let r =
        pass("struct S{let n:int}enum E{V(S)}const O:E?=Option::Some(E::V(S(0)));const P=O exists");
    let (_, res, c) = r.semantic.unwrap();
    let id = res.definitions.iter().position(|d| d.name == "P").unwrap();
    assert!(matches!(
        c.const_values[id],
        ConstEvaluation::Value {
            value: ConstValue::Bool(true),
            ..
        }
    ));
    assert_eq!(
        codes(&frontend("func f(){let x:bool=none exists}")),
        ["N2103"]
    );
    assert_eq!(
        codes(&frontend("func f(){let x=Option::None exists}")),
        ["N2103"]
    );
    assert_eq!(
        codes(&frontend(
            "func f(){let x=Option::Some(1) exists exists exists}"
        )),
        ["N2101"]
    );
    assert_eq!(
        codes(&frontend("const O:int?=none;const P=O exists")),
        Vec::<String>::new()
    );
    let (_,_,c)=p12_bundle(&[
        ("main","use lib::Option as Custom;func main(){let a:int?=none;let b=a exists;let c=Custom::Some(1)}"),
        ("lib","public enum Option{Some(int);None}"),
    ]);
    assert!(!c.has_errors());
}
#[test]
fn p20_exists_const_cycles_checked_evaluation_defaults_and_exact_budget() {
    for expr in ["Option::Some(B) exists", "false && Option::Some(B) exists"] {
        let r = frontend(&format!(
            "const A:bool={expr};const B:bool=A;func main(){{}}"
        ));
        assert!(!r.passed());
        assert!(codes(&r).contains(&"N3202".into()));
    }
    let (_, _, c) = p12_bundle(&[
        (
            "main",
            "use lib::B;public const A:bool=Option::Some(B) exists;func main(){}",
        ),
        ("lib", "use main::A;public const B:bool=A"),
    ]);
    assert!(c.diagnostics.iter().any(|d| d.code.to_string() == "N3202"));
    for source in [
        "const P=Option::Some(1/0) exists;func main(){}",
        "func f(p:bool=Option::Some(1/0) exists){}",
    ] {
        let r = frontend(source);
        let d = r
            .diagnostics()
            .iter()
            .find(|d| d.code.to_string() == "N3201")
            .unwrap();
        assert_eq!(r.sources.slice(d.primary.span).unwrap(), "1/0");
    }
    pass("const P=false && Option::Some(1/0) exists;func f(p:bool=true || Option::Some(1/0) exists){}");
    let terms = vec!["0"; 4999].join("+");
    let expr = format!("(Option::Some({terms}) exists)");
    let r = pass(&format!(
        "const P={expr};func f(p:bool={expr},q:bool={expr}){{const LOCAL={expr}}}"
    ));
    let (_, res, c) = r.semantic.unwrap();
    for (i, d) in res
        .definitions
        .iter()
        .enumerate()
        .filter(|(_, d)| d.constant)
    {
        assert!(
            matches!(
                c.const_values[i],
                ConstEvaluation::Value {
                    value: ConstValue::Bool(true),
                    nodes: 10000
                }
            ),
            "{} {:?}",
            d.name,
            c.const_values[i]
        );
    }
    assert!(c
        .defaults
        .iter()
        .flatten()
        .all(|d| matches!(d.evaluation, ConstEvaluation::Value { nodes: 10000, .. })));
    for source in [
        format!("const P=({expr});func main(){{}}"),
        format!("func f(p:bool=({expr})){{}}"),
        format!("const P=false&&{expr};func main(){{}}"),
    ] {
        let r = frontend(&source);
        assert!(codes(&r).contains(&"N3202".into()));
        assert!(!codes(&r).contains(&"N3201".into()));
    }
}

#[test]
fn p21_alias_fixture_exact_diagnostics_and_cycle_cascade_suppression() {
    for (source, code, start, end) in [
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/unknown_target.nova"
            ),
            "N2001",
            38,
            45,
        ),
        (
            include_str!("../../../docs/development-v0.1/alias-proposal-fixtures/self_cycle.nova"),
            "N2103",
            38,
            39,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/nested_cycle.nova"
            ),
            "N2103",
            38,
            47,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/duplicate_alias.nova"
            ),
            "N2002",
            47,
            48,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/duplicate_struct.nova"
            ),
            "N2002",
            57,
            58,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/primitive_name.nova"
            ),
            "N2002",
            34,
            37,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/generic_alias.nova"
            ),
            "N1102",
            35,
            36,
        ),
        (
            include_str!("../../../docs/development-v0.1/alias-proposal-fixtures/local_alias.nova"),
            "N1102",
            43,
            47,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/generic_application.nova"
            ),
            "N1102",
            62,
            68,
        ),
        (
            include_str!("../../../docs/development-v0.1/alias-proposal-fixtures/bare_family.nova"),
            "N2101",
            38,
            44,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/constructor_head.nova"
            ),
            "N1102",
            83,
            87,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/variant_head.nova"
            ),
            "N1102",
            73,
            77,
        ),
        (
            include_str!("../../../docs/development-v0.1/alias-proposal-fixtures/alias_value.nova"),
            "N2001",
            62,
            63,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/string_payload.nova"
            ),
            "N1102",
            64,
            68,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/layout_cycle.nova"
            ),
            "N2101",
            66,
            67,
        ),
        (
            include_str!(
                "../../../docs/development-v0.1/alias-proposal-fixtures/const_type_mismatch.nova"
            ),
            "N2101",
            53,
            54,
        ),
    ] {
        let f = frontend(source);
        assert!(!f.passed(), "{source}");
        assert!(
            f.diagnostics().iter().any(|d| d.code.to_string() == code
                && d.primary.span.start() == start
                && d.primary.span.end() == end),
            "{source} {:?}",
            f.diagnostics()
        );
    }
    let f = frontend("type A=B\ntype B=Option<A>\nfunc f(x:A)->B{return x}");
    assert_eq!(
        f.diagnostics()
            .iter()
            .filter(|d| d.code.to_string() == "N2103")
            .count(),
        2
    );
    assert!(!f
        .diagnostics()
        .iter()
        .any(|d| matches!(d.code.to_string().as_str(), "N2001" | "N2101")));
    for d in f.diagnostics() {
        assert_eq!(d.secondary.len(), 2);
        assert_eq!(d.secondary[0].message, "alias A in this cycle");
    }
}
#[test]
fn p21_alias_canonical_identity_all_primitives_sum_cache_const_and_defaults() {
    let (_, _, c) = p12_bundle(&[
        (
            "main",
            include_str!("../../../docs/development-v0.1/alias-proposal-fixtures/main.nova"),
        ),
        (
            "types",
            include_str!("../../../docs/development-v0.1/alias-proposal-fixtures/types.nova"),
        ),
    ]);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    pass(include_str!(
        "../../../docs/development-v0.1/alias-proposal-fixtures/multiline.nova"
    ));
    for (ty, value) in [
        ("int8", "-128"),
        ("uint8", "255"),
        ("int16", "-32768"),
        ("uint16", "65535"),
        ("int64", "-9223372036854775808"),
        ("uint64", "18446744073709551615"),
        ("int", "1"),
        ("uint", "1"),
        ("float", "1.5"),
        ("double", "-0.0"),
        ("char", "'🙂'"),
        ("bool", "false"),
        ("()", "()"),
        ("string", "\"x\""),
    ] {
        let (_,r,c)=pass(&format!("type A=B\ntype B={ty}\nconst C:A={value}\nfunc f(x:A=C)->B{{return x}}func main(){{let a:A=f();let b:{ty}=a}}")).semantic.unwrap();
        let index = |name: &str| r.definitions.iter().position(|d| d.name == name).unwrap();
        assert_eq!(c.aliases[&index("A")], c.aliases[&index("B")]);
        assert_eq!(
            c.definition_types[index("a")],
            c.definition_types[index("b")]
        );
    }
    let (_,r,c)=pass("type A=int?\ntype B=Option<int32>\nconst X:A=Option::Some(0);func f(a:B=none)->A{return a}").semantic.unwrap();
    assert_eq!(c.sums.len(), 1);
    let ids = r
        .definitions
        .iter()
        .enumerate()
        .filter(|(_, d)| matches!(d.kind, DefinitionKind::TypeAlias(_)))
        .map(|(i, _)| c.aliases[&i])
        .collect::<Vec<_>>();
    assert_eq!(ids[0], ids[1]);
    pass("type Option=int;func f(x:Option)->int{return x}func g(x:int?)->bool{return x exists}");
    let f = frontend("type Option=int;type A=Option<int>");
    assert!(f
        .diagnostics()
        .iter()
        .any(|d| d.code.to_string() == "N1102"));
}
#[test]
fn p21_alias_declaration_scopes_visibility_dual_import_atomicity_and_cycles() {
    let (_,_,c)=p12_bundle(&[("main","use lib::A;use lib::factory;type T=bool;func f(x:A)->A{return x}func main(){let a:A=factory();let b=f(a)}"),("lib","type T=int8;private struct Hidden{private let n:T}public type A=Hidden;public func factory()->A{return Hidden(7)}")]);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    let (_, _, c) = p12_bundle(&[
        ("main", "use lib::A;func f(x:A){}"),
        ("lib", "private type A=int"),
    ]);
    assert!(c.diagnostics.iter().any(|d| d.code.to_string() == "N2004"));
    assert!(!c.diagnostics.iter().any(|d| d.code.to_string() == "N2001"));
    let (_, r, c) = p12_bundle(&[
        ("main", "use lib::A;func f(x:A){}"),
        ("lib", "public type A=int;private func A(){}"),
    ]);
    assert!(c.diagnostics.iter().any(|d| d.code.to_string() == "N2004"));
    assert!(r.scopes.iter().any(|s| s.failed_types.contains("A")
        && !s.types.contains_key("A")
        && s.failed_imports.contains("A")
        && !s.definitions.contains_key("A")));
    let (_, _, c) = p12_bundle(&[
        ("main", "use lib::B;public type A=B;func f(x:A){}"),
        ("lib", "use main::A;public type B=(A,)"),
    ]);
    assert_eq!(
        c.diagnostics
            .iter()
            .filter(|d| d.code.to_string() == "N2103")
            .count(),
        2
    );
    assert!(!c.diagnostics.iter().any(|d| d.code.to_string() == "N2101"));
    let (_, _, c) = p12_bundle(&[
        ("main", "use lib::A;func main(){let a:A=none}"),
        ("dep", "public type X=int?"),
        ("lib", "use dep::X;public type A=X"),
    ]);
    assert!(!c.has_errors());
    let f = frontend("type A=Node;struct Node{let n:A}");
    assert!(f
        .diagnostics()
        .iter()
        .any(|d| d.code.to_string() == "N2101"));
    assert!(!f
        .diagnostics()
        .iter()
        .any(|d| d.code.to_string() == "N2103"));
}
#[test]
fn p21_alias_1024_1025_small_stack_and_expansion_limits() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let mut s = String::new();
            for n in 0..1023 {
                s.push_str(&format!("type A{n}=A{}\n", n + 1));
            }
            s.push_str("type A1023=int8\nfunc main(){let a:A0=7}");
            pass(&s);
            s.push_str("type Excess=int\n");
            let f = frontend(&s);
            let d = f
                .diagnostics()
                .iter()
                .find(|d| d.code.to_string() == "N8901")
                .unwrap();
            assert_eq!(f.sources.slice(d.primary.span).unwrap(), "Excess");
            assert!(!d.notes.is_empty());
        })
        .unwrap()
        .join()
        .unwrap();
    let mut s = "type A0=int\n".to_string();
    for n in 1..25 {
        s.push_str(&format!("type A{n}=(A{},A{})\n", n - 1, n - 1));
    }
    let f = frontend(&s);
    assert!(
        f.diagnostics()
            .iter()
            .any(|d| d.code.to_string() == "N8901"),
        "{:?}",
        f.diagnostics()
    );
    let f = frontend("type Text=string;struct S{let s:Text}");
    assert!(f
        .diagnostics()
        .iter()
        .any(|d| d.code.to_string() == "N1102"));
}
