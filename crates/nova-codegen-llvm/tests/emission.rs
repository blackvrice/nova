use nova_codegen::*;
use nova_codegen_llvm::{
    emit_ir,
    toolchain::{parse_version, ClangTool},
    LlvmBackend,
};
use nova_source::SourceDatabase;
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
#[ignore = "requires real LLVM 21.1.8"]
fn real_llvm_verifies_both_x64_objects_and_preserves_existing_output() {
    let clang = ClangTool::new(std::env::var_os("NOVA_CLANG").expect("NOVA_CLANG"));
    let backend = LlvmBackend { clang };
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/llvm-object-tests")
        .join(std::process::id().to_string());
    std::fs::create_dir_all(&root).unwrap();
    let unit=unit("func u(x:()){} func b(x:bool)->bool{return !x} func main(){u(());print(\"한글 {b(false)} {-7%3}\")}");
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
