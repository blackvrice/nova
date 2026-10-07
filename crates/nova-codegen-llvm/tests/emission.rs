use nova_codegen::*;
use nova_codegen_llvm::{
    emit_ir,
    toolchain::{parse_version, ClangTool},
    LlvmBackend,
};
use nova_source::SourceDatabase;

fn module_corpus() -> CodegenUnit {
    let mut sources = SourceDatabase::default();
    let mut inputs = vec![];
    for (path,source) in [
        ("main", "use math::sum as plus;use math::main as helper;func twice(n:int)->int{return n+n};func main(){let v=plus(20,2);print(\"{v} {helper(1)}\")}"),
        ("math", "use main::twice;private const C=1;public func sum(a:int,b:int)->int{return twice(a)+b};func main(n:int)->int{return n+C}"),
    ] {
        let file=sources.add(format!("{path}.nova"),source.into()).unwrap();let lexed=nova_lexer::lex(&sources,file).unwrap();let parsed=nova_parser::parse(&sources,file,&nova_lexer::normalize_ends(&lexed.tokens)).unwrap();assert!(!lexed.has_errors()&&!parsed.has_errors());inputs.push((path.into(),nova_hir::lower(&sources,&parsed.arena,parsed.root).unwrap()));
    }
    let hir = nova_hir::Module::bundle(inputs).unwrap();
    let resolved = nova_resolve::resolve(&hir);
    let checked = nova_typecheck::check(&hir, &resolved).unwrap();
    assert!(!checked.has_errors(), "{:?}", checked.diagnostics);
    CodegenUnit::new(nova_mir::lower(&hir, &resolved, &checked, false).unwrap()).unwrap()
}

#[test]
fn p11_module_callees_have_distinct_private_symbols_and_entry_file_identity() {
    let unit = module_corpus();
    let entry = unit.executable_entry().unwrap();
    assert_eq!(
        unit.mir()
            .bodies
            .iter()
            .find(|b| b.callee == entry)
            .unwrap()
            .source
            .span
            .file()
            .as_u32(),
        0
    );
    for target in [TargetSpec::WindowsX64Msvc, TargetSpec::LinuxX64Gnu] {
        let ir = emit_ir(&unit, target, true).unwrap();
        assert_eq!(ir, emit_ir(&unit, target, true).unwrap());
        assert_eq!(ir.text.matches("define internal fastcc ").count(), 4);
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 via NOVA_CLANG"]
fn p11_real_llvm_bundle_coff_elf_o0_o2() {
    let unit = module_corpus();
    let backend = LlvmBackend {
        clang: ClangTool::new(std::env::var_os("NOVA_CLANG").expect("NOVA_CLANG")),
    };
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/llvm-p11-tests")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&root).unwrap();
    for target in [TargetSpec::WindowsX64Msvc, TargetSpec::LinuxX64Gnu] {
        for optimization in [OptimizationLevel::None, OptimizationLevel::Default] {
            let object_path = root.join(format!("{}-{optimization:?}.obj", target.triple()));
            backend
                .codegen_unit(
                    &unit,
                    &target,
                    &CodegenOptions {
                        object_path: object_path.clone(),
                        optimization,
                        executable: true,
                    },
                )
                .unwrap();
            let bytes = std::fs::read(object_path).unwrap();
            assert!(bytes.len() > 100);
            if target == TargetSpec::WindowsX64Msvc {
                assert_eq!(&bytes[..2], b"\x64\x86");
            } else {
                assert_eq!(&bytes[..4], b"\x7fELF");
            }
        }
    }
}
fn unit(source: &str) -> CodegenUnit {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", source.into()).unwrap();
    let lexed = nova_lexer::lex(&sources, file).unwrap();
    let parsed =
        nova_parser::parse(&sources, file, &nova_lexer::normalize_ends(&lexed.tokens)).unwrap();
    assert!(!lexed.has_errors() && !parsed.has_errors());
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = nova_resolve::resolve(&hir);
    let checked = nova_typecheck::check(&hir, &resolved).unwrap();
    assert!(!checked.has_errors(), "{:?}", checked.diagnostics);
    CodegenUnit::new(nova_mir::lower(&hir, &resolved, &checked, false).unwrap()).unwrap()
}

fn integer_corpus() -> String {
    let mut source = include_str!("../../../examples/integers.nova").to_owned();
    for (i, ty) in [
        "int8", "uint8", "int16", "uint16", "int32", "uint32", "int64", "uint64",
    ]
    .iter()
    .enumerate()
    {
        source.push_str(&format!("func calc{i}(a:{ty},b:{ty})->{ty}{{return a/b+a%b+a*b-a+b}} func compare{i}(a:{ty},b:{ty})->bool{{return a<b}}"));
    }
    source
}

fn char_corpus() -> String {
    let mut source = include_str!("../../../examples/characters.nova").to_owned();
    for (i, op) in ["==", "!=", "<", "<=", ">", ">="].iter().enumerate() {
        source.push_str(&format!(
            "func compare{i}(a:char,b:char)->bool{{return a{op}b}}"
        ));
    }
    source
}

fn float_corpus() -> String {
    let mut source = include_str!("../../../examples/floats.nova").to_owned();
    for ty in ["float", "double"] {
        for (i, op) in ["+", "-", "*", "/", "==", "!=", "<", "<=", ">", ">="]
            .iter()
            .enumerate()
        {
            let result = if i < 4 { ty } else { "bool" };
            source.push_str(&format!(
                "func {ty}_{i}(a:{ty},b:{ty})->{result}{{return a{op}b}}"
            ));
        }
        source.push_str(&format!("func negate_{ty}(a:{ty})->{ty}{{return -a}}"));
    }
    source.push_str("func signed(a:int16)->float{return a}func unsigned(a:uint32)->double{return a}func widen(a:float)->double{return a}");
    source
}

#[test]
fn p09_ir_exact_bits_predicates_scalar_abi_and_no_fast_math() {
    let corpus = unit(&float_corpus());
    for target in [TargetSpec::WindowsX64Msvc, TargetSpec::LinuxX64Gnu] {
        let ir = emit_ir(&corpus, target, true).unwrap();
        assert_eq!(ir, emit_ir(&corpus, target, true).unwrap());
        for llvm in ["float", "double"] {
            for op in ["fadd", "fsub", "fmul", "fdiv", "fneg"] {
                assert!(ir.text.contains(&format!("{op} {llvm}")));
            }
            for predicate in ["oeq", "une", "olt", "ole", "ogt", "oge", "uno"] {
                assert!(ir.text.contains(&format!("fcmp {predicate} {llvm}")));
            }
            assert!(ir.text.contains(&format!("define internal fastcc {llvm}")));
        }
        for op in [
            "fpext float",
            "sitofp i16",
            "uitofp i32",
            "bitcast (i32 2147483648 to float)",
            "bitcast (i64 9221120237041090560 to double)",
        ] {
            assert!(ir.text.contains(op), "{op}");
        }
        for flag in [
            " fast ",
            " nnan ",
            " ninf ",
            " nsz ",
            " arcp ",
            " contract ",
            " reassoc ",
            " afn ",
            "llvm.fma",
            "fptrunc",
            "fptosi",
            "fptoui",
            "frem",
        ] {
            assert!(!ir.text.contains(flag), "{flag}");
        }
        assert_eq!(ir.sources, corpus.mir().sources);
        let global_only = unit("const A=0.1;const B:double=-0.0;func main(){print(\"{A} {B}\")}");
        let ir = emit_ir(&global_only, target, true).unwrap();
        assert!(ir
            .text
            .contains("declare void @nova_format_f32(ptr, i32, i32, i32, i32)"));
        assert!(ir
            .text
            .contains("declare void @nova_format_f64(ptr, i64, i32, i32, i32)"));
        assert!(ir.text.contains("bitcast (i32 1036831949 to float)"));
    }
}

#[test]
#[ignore = "requires real LLVM 21.1.8"]
fn p09_real_llvm_verifies_float_coff_elf_o0_o2() {
    let backend = LlvmBackend {
        clang: ClangTool::new(std::env::var_os("NOVA_CLANG").expect("NOVA_CLANG")),
    };
    let corpus = unit(&float_corpus());
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/llvm-float-tests")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&root).unwrap();
    for (target, ext) in [
        (TargetSpec::WindowsX64Msvc, "obj"),
        (TargetSpec::LinuxX64Gnu, "o"),
    ] {
        for (optimization, name) in [
            (OptimizationLevel::None, "o0"),
            (OptimizationLevel::Default, "o2"),
        ] {
            let output = root.join(format!("{name}.{ext}"));
            backend
                .codegen_unit(
                    &corpus,
                    &target,
                    &CodegenOptions {
                        object_path: output.clone(),
                        optimization,
                        executable: true,
                    },
                )
                .unwrap();
            let bytes = std::fs::read(output).unwrap();
            assert!(bytes.len() > 100);
            if ext == "o" {
                assert_eq!(&bytes[..4], b"\x7fELF");
            } else {
                assert_eq!(&bytes[..2], b"\x64\x86");
            }
        }
    }
}

#[test]
fn p08_ir_uses_scalar_abi_unsigned_comparisons_and_char_formatter() {
    let corpus = unit(&char_corpus());
    for target in [TargetSpec::WindowsX64Msvc, TargetSpec::LinuxX64Gnu] {
        let ir = emit_ir(&corpus, target, true).unwrap();
        assert_eq!(ir, emit_ir(&corpus, target, true).unwrap());
        assert!(ir
            .text
            .contains("define internal fastcc i32 @nova_fn_1(i32 %arg0)"));
        for predicate in ["eq", "ne", "ult", "ule", "ugt", "uge"] {
            assert!(ir.text.contains(&format!("icmp {predicate} i32")));
        }
        assert!(ir.text.contains("call void @nova_format_char"));
        assert!(ir.text.contains("i32 128578"));
        assert!(!ir.text.contains(" = sext ") && !ir.text.contains(" = sdiv "));
        assert_eq!(ir.sources, corpus.mir().sources);
        let global_only = unit("const C='🙂';func main(){print(\"{C}\")}");
        let ir = emit_ir(&global_only, target, true).unwrap();
        assert!(ir
            .text
            .contains("declare void @nova_format_char(ptr, i32, i32, i32, i32)"));
    }
}

#[test]
#[ignore = "requires real LLVM 21.1.8"]
fn p08_real_llvm_verifies_char_for_coff_elf_o0_o2() {
    let backend = LlvmBackend {
        clang: ClangTool::new(std::env::var_os("NOVA_CLANG").expect("NOVA_CLANG")),
    };
    let corpus = unit(&char_corpus());
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/llvm-char-tests")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&root).unwrap();
    for (target, extension) in [
        (TargetSpec::WindowsX64Msvc, "obj"),
        (TargetSpec::LinuxX64Gnu, "o"),
    ] {
        for (optimization, name) in [
            (OptimizationLevel::None, "o0"),
            (OptimizationLevel::Default, "o2"),
        ] {
            let output = root.join(format!("{name}.{extension}"));
            backend
                .codegen_unit(
                    &corpus,
                    &target,
                    &CodegenOptions {
                        object_path: output.clone(),
                        optimization,
                        executable: true,
                    },
                )
                .unwrap();
            let bytes = std::fs::read(output).unwrap();
            assert!(bytes.len() > 100);
            if extension == "o" {
                assert_eq!(&bytes[..4], b"\x7fELF");
            } else {
                assert_eq!(&bytes[..2], &[0x64, 0x86]);
            }
        }
    }
}

#[test]
fn p07_ir_selects_width_sign_extension_guards_and_formatters() {
    let corpus = unit(&integer_corpus());
    for target in [TargetSpec::WindowsX64Msvc, TargetSpec::LinuxX64Gnu] {
        let ir = emit_ir(&corpus, target, true).unwrap();
        assert_eq!(ir, emit_ir(&corpus, target, true).unwrap());
        for width in [8, 16, 32, 64] {
            for sign in ["s", "u"] {
                for op in ["add", "sub", "mul"] {
                    assert!(ir.text.contains(&format!(
                        "call {{ i{width}, i1 }} @llvm.{sign}{op}.with.overflow.i{width}"
                    )));
                }
                for op in ["div", "rem"] {
                    assert!(ir.text.contains(&format!(" = {sign}{op} i{width}")));
                }
                assert!(ir.text.contains(&format!("icmp {sign}lt i{width}")));
            }
            assert!(ir.text.contains(&format!("icmp eq i{width}")));
        }
        assert!(ir.text.contains("sext i8") && ir.text.contains("zext i8"));
        assert!(
            ir.text.contains("call void @nova_format_i64")
                && ir.text.contains("call void @nova_format_u64")
        );
        assert!(ir.text.contains("icmp eq i64") && ir.text.contains("-9223372036854775808"));
        assert!(!ir.text.contains("nsw") && !ir.text.contains("nuw"));
    }
    let unit = unit("const X:uint64=18446744073709551615;func main(){print(\"{X}\")}");
    assert!(emit_ir(&unit, TargetSpec::WindowsX64Msvc, true)
        .unwrap()
        .text
        .contains("declare void @nova_format_u64"));
}

#[test]
#[ignore = "requires real LLVM 21.1.8"]
fn p07_real_llvm_verifies_all_integer_widths_for_coff_elf_o0_o2() {
    let backend = LlvmBackend {
        clang: ClangTool::new(std::env::var_os("NOVA_CLANG").expect("NOVA_CLANG")),
    };
    let unit = unit(&integer_corpus());
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/llvm-integer-tests")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&root).unwrap();
    for (target, extension) in [
        (TargetSpec::WindowsX64Msvc, "obj"),
        (TargetSpec::LinuxX64Gnu, "o"),
    ] {
        for (optimization, name) in [
            (OptimizationLevel::None, "o0"),
            (OptimizationLevel::Default, "o2"),
        ] {
            let output = root.join(format!("{name}.{extension}"));
            let options = CodegenOptions {
                object_path: output.clone(),
                optimization,
                executable: true,
            };
            backend.codegen_unit(&unit, &target, &options).unwrap();
            let bytes = std::fs::read(output).unwrap();
            assert!(bytes.len() > 100);
            if extension == "o" {
                assert_eq!(&bytes[..4], b"\x7fELF");
            } else {
                assert_eq!(&bytes[..2], &[0x64, 0x86]);
            }
        }
    }
}
#[test]
fn deterministic_ir_has_private_calls_and_source_origins() {
    let unit = unit("func f(x:int)->int{return x+1} func main(){print(\"한글 {f(2)}\")}");
    let ir = emit_ir(&unit, TargetSpec::WindowsX64Msvc, true).unwrap();
    assert_eq!(
        ir,
        emit_ir(&unit, TargetSpec::WindowsX64Msvc, true).unwrap()
    );
    assert!(ir
        .text
        .contains("define internal fastcc i32 @nova_fn_1(i32 %arg0)"));
    assert!(ir.text.contains("@nova_stage_a_entry"));
    assert!(ir.text.contains("hir "));
    assert_eq!(ir.sources, unit.mir().sources);
}

#[test]
fn global_constant_reads_emit_no_runtime_initialization_or_arithmetic() {
    let unit = unit("const A=B*2+1;func f()->int{return A} const B=3;const SAFE=false&&(1/0==0)");
    for target in [TargetSpec::WindowsX64Msvc, TargetSpec::LinuxX64Gnu] {
        let ir = emit_ir(&unit, target, false).unwrap();
        assert_eq!(ir, emit_ir(&unit, target, false).unwrap());
        assert!(ir.text.contains("ret i32 7"));
        assert!(!ir.text.contains("call void @nova_panic"));
        assert!(!ir.text.contains("call { i32, i1 } @llvm"));
        assert!(!ir.text.contains(" = sdiv "));
        assert!(!ir.text.contains(" global ") && !ir.text.contains("@llvm.global_ctors"));
        assert_eq!(ir.sources, unit.mir().sources);
    }
}
#[test]
fn arithmetic_guards_precede_division_and_checked_intrinsics() {
    let unit =
        unit("func main(){let a=2147483647+1;let m=-2147483648;let b=m/-1;let c=m%-1;let n=-m}");
    let ir = emit_ir(&unit, TargetSpec::WindowsX64Msvc, true).unwrap();
    assert!(ir.text.contains("@llvm.sadd.with.overflow.i32"));
    assert!(ir.text.contains("@llvm.ssub.with.overflow.i32"));
    let div = ir.text.find(" = sdiv ").unwrap();
    let rem = ir.text.find(" = srem ").unwrap();
    assert!(ir.text[..div].contains("icmp eq i32 -1, 0"));
    assert!(ir.text[..rem].contains("call void @nova_panic(i32 1"));
    assert!(!ir.text.contains("nsw"));
    assert!(!ir.text.contains("add i32 2147483647, 1"));
}
#[test]
fn string_bytes_are_escaped_and_never_form_llvm_syntax() {
    let ir = emit_ir(
        &unit("func main(){print(\"\" );print(\"A\\0B\\\"한글\")}"),
        TargetSpec::WindowsX64Msvc,
        true,
    )
    .unwrap();
    assert!(ir.text.contains("[0 x i8] c\"\""));
    assert!(ir.text.contains("\\41\\00\\42\\22\\ED\\95\\9C\\EA\\B8\\80"));
    assert!(ir.text.contains("call void @nova_print(ptr"));
}
#[test]
fn executable_entry_is_checked_but_fragments_are_allowed() {
    for source in [
        "",
        "func f(){}",
        "func main(x:int){}",
        "func main()->int{return 0}",
    ] {
        let unit = unit(source);
        assert!(matches!(
            emit_ir(&unit, TargetSpec::WindowsX64Msvc, true),
            Err(CodegenError::InvalidEntry { .. })
        ));
        assert!(emit_ir(&unit, TargetSpec::WindowsX64Msvc, false).is_ok());
    }
    assert!(emit_ir(
        &unit("func main()->void{}"),
        TargetSpec::WindowsX64Msvc,
        true
    )
    .is_ok());
}
#[test]
fn unsupported_targets_and_missing_tools_are_explicit_errors() {
    let unit = unit("func main(){}");
    assert!(matches!(
        emit_ir(&unit, TargetSpec::LinuxArm64, true),
        Err(CodegenError::UnsupportedTarget(_))
    ));
    let options = CodegenOptions {
        object_path: "must-not-exist.obj".into(),
        optimization: OptimizationLevel::None,
        executable: true,
    };
    let backend = LlvmBackend {
        clang: ClangTool::new("a-nonexistent-nova-clang-executable"),
    };
    assert!(matches!(
        backend.codegen_unit(&unit, &TargetSpec::WindowsX64Msvc, &options),
        Err(CodegenError::MissingTool(_))
    ));
    assert!(!options.object_path.exists());
}
#[test]
fn clang_banner_parsing_rejects_unrelated_and_malformed_versions() {
    for text in [
        "clang version 21.1.8\nTarget: x86_64-pc-windows-msvc",
        "Ubuntu clang version 21.1.8",
        "Apple clang version 21.1.8 (build)",
    ] {
        let version = parse_version(text).unwrap();
        assert_eq!((version.major, version.minor, version.patch), (21, 1, 8));
    }
    for text in [
        "rustc 21.1.8",
        "clang version broken",
        "clang version 21.1",
        "clang version 21.1.8.2",
        "",
    ] {
        assert!(parse_version(text).is_none());
    }
}
#[test]
fn unit_boolean_parameters_and_return_lower_without_a_void_value() {
    let ir = emit_ir(
        &unit(
            "func u(x:()){} func b(x:bool)->bool{return !x} func main(){u(());let value=b(false)}",
        ),
        TargetSpec::WindowsX64Msvc,
        true,
    )
    .unwrap();
    assert!(ir.text.contains("fastcc void @nova_fn_1({} %arg0)"));
    assert!(ir.text.contains("fastcc i1 @nova_fn_2(i1 %arg0)"));
    assert!(!ir.text.contains("store void"));
}

#[test]
fn hello_ir_snapshot_is_stable() {
    let ir = emit_ir(
        &unit("func main(){print(\"Hello, Nova\")}"),
        TargetSpec::WindowsX64Msvc,
        true,
    )
    .unwrap();
    if std::env::var_os("NOVA_UPDATE_SNAPSHOTS").is_some() {
        std::fs::write(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots/hello.ll"),
            ir.text,
        )
        .unwrap();
    } else {
        assert_eq!(ir.text, include_str!("snapshots/hello.ll"));
    }
}

#[test]
fn const_ir_has_materialized_values_and_no_initializer_runtime_guards() {
    let source = "func f()->int{const x=1+2*3;const skipped=true||(1/0==0);return x}";
    for target in [TargetSpec::WindowsX64Msvc, TargetSpec::LinuxX64Gnu] {
        let ir = emit_ir(&unit(source), target, false).unwrap();
        assert!(ir.text.contains("store i32 7"));
        assert!(!ir.text.contains("call void @nova_panic"));
        assert!(!ir.text.contains(" sdiv "));
        assert!(!ir.text.contains("call { i32, i1 } @llvm."));
        assert_eq!(ir, emit_ir(&unit(source), target, false).unwrap());
    }
}

#[test]
#[ignore = "requires real LLVM 21.1.8"]
fn real_llvm_verifies_both_x64_objects_and_preserves_existing_output() {
    let clang = ClangTool::new(std::env::var_os("NOVA_CLANG").expect("NOVA_CLANG"));
    let backend = LlvmBackend { clang };
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/llvm-object-tests")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&root).unwrap();
    let unit=unit("const VALUE=BASE*2+1;func u(x:()){} func b(x:bool)->bool{return !x} func main(){u(());print(\"한글 {b(false)} {-7%3} {VALUE}\")} const BASE=3");
    for (target, name) in [
        (TargetSpec::WindowsX64Msvc, "windows.obj"),
        (TargetSpec::LinuxX64Gnu, "linux.o"),
    ] {
        let output = root.join(name);
        if output.exists() {
            std::fs::remove_file(&output).unwrap();
        }
        let options = CodegenOptions {
            object_path: output.clone(),
            optimization: OptimizationLevel::Default,
            executable: true,
        };
        assert_eq!(
            backend
                .codegen_unit(&unit, &target, &options)
                .unwrap()
                .target,
            target
        );
        let before = std::fs::read(&output).unwrap();
        assert!(matches!(
            backend.codegen_unit(&unit, &target, &options),
            Err(CodegenError::ExistingOutput(_))
        ));
        assert_eq!(std::fs::read(output).unwrap(), before);
    }
}

#[test]
#[ignore = "requires real LLVM 21.1.8"]
fn real_llvm_rejects_invalid_ir() {
    let clang = ClangTool::new(std::env::var_os("NOVA_CLANG").expect("NOVA_CLANG"));
    let result = clang.run_with_input(
        ["-x", "ir", "-c", "-emit-llvm", "-S", "-", "-o", "-"],
        b"define void @broken() { entry: ret i32 1 }",
    );
    assert!(matches!(result, Err(CodegenError::ToolFailure { .. })));
}

fn cast_corpus() -> String {
    let mut source = include_str!("../../../examples/casts.nova").to_owned();
    for (i, s) in [
        "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64", "float", "double",
    ]
    .iter()
    .enumerate()
    {
        for (j, d) in [
            "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64", "float",
            "double",
        ]
        .iter()
        .enumerate()
        {
            source.push_str(&format!("func c{i}_{j}(v:{s})->{d}{{return v as {d}}}"));
        }
    }
    source
}
#[test]
fn p10_cast_ir_guards_fptoi_before_conversion_and_uses_direct_rounding() {
    let text = emit_ir(&unit(&cast_corpus()), TargetSpec::WindowsX64Msvc, true)
        .unwrap()
        .text;
    assert!(text.contains("sitofp i64") && text.contains("uitofp i64"));
    assert!(text.contains("fptrunc double") && text.contains("fpext float"));
    assert!(text.contains("call double @llvm.trunc.f64"));
    for op in ["fptosi", "fptoui"] {
        for (at, _) in text.match_indices(op) {
            let preceding = &text[..at];
            let start = preceding.rfind("define ").unwrap();
            let preceding = &preceding[start..];
            assert!(preceding.contains("fcmp oge") && preceding.contains("fcmp olt"));
            assert!(preceding.contains("call void @nova_panic(i32 4"));
            assert!(preceding.rfind("ok.").unwrap() > preceding.rfind("unreachable").unwrap());
        }
    }
    assert!(!text.contains(" fast ") && !text.contains(" nsw ") && !text.contains(" nuw "));
}
#[test]
#[ignore = "requires real LLVM 21.1.8"]
fn p10_real_llvm_verifies_all_cast_pairs_coff_elf_o0_o2() {
    let backend = LlvmBackend {
        clang: ClangTool::new(std::env::var_os("NOVA_CLANG").expect("NOVA_CLANG")),
    };
    let corpus = unit(&cast_corpus());
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/llvm-cast-tests")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&root).unwrap();
    for (target, ext) in [
        (TargetSpec::WindowsX64Msvc, "obj"),
        (TargetSpec::LinuxX64Gnu, "o"),
    ] {
        for (optimization, name) in [
            (OptimizationLevel::None, "o0"),
            (OptimizationLevel::Default, "o2"),
        ] {
            let output = root.join(format!("{name}.{ext}"));
            backend
                .codegen_unit(
                    &corpus,
                    &target,
                    &CodegenOptions {
                        object_path: output.clone(),
                        optimization,
                        executable: true,
                    },
                )
                .unwrap();
            let bytes = std::fs::read(output).unwrap();
            assert!(bytes.len() > 100);
            if ext == "o" {
                assert_eq!(&bytes[..4], b"\x7fELF")
            } else {
                assert_eq!(&bytes[..2], b"\x64\x86")
            }
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 via NOVA_CLANG"]
fn p12_real_llvm_struct_coff_elf_o0_o2() {
    let corpus=include_str!("../../../examples/structs.nova").to_owned()+"struct Empty{} struct Z{let e:Empty;let u:();let flag:bool;let c:char;let d:double} func z(x:Z)->Z{return x} func zero()->Empty{return Empty()} func more(){let e=zero();let v=z(Z(e,(),true,'🙂',1.5));print(\"{v.flag} {v.c} {v.d}\")} ";
    let unit = unit(&corpus);
    let backend = LlvmBackend {
        clang: ClangTool::new(std::env::var_os("NOVA_CLANG").unwrap()),
    };
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/llvm-p12-tests")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&root).unwrap();
    for target in [TargetSpec::WindowsX64Msvc, TargetSpec::LinuxX64Gnu] {
        for optimization in [OptimizationLevel::None, OptimizationLevel::Default] {
            let path = root.join(format!("{}-{optimization:?}.obj", target.triple()));
            backend
                .codegen_unit(
                    &unit,
                    &target,
                    &CodegenOptions {
                        object_path: path.clone(),
                        optimization,
                        executable: true,
                    },
                )
                .unwrap();
            let bytes = std::fs::read(path).unwrap();
            assert!(if target == TargetSpec::WindowsX64Msvc {
                bytes.starts_with(b"\x64\x86")
            } else {
                bytes.starts_with(b"\x7fELF")
            });
        }
    }
}
