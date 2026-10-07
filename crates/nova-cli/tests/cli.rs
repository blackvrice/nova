use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn p11_module_failures_gate_tools_and_preserve_outputs_for_all_commands() {
    for (main, lib, code) in [
        ("use missing::C;func main(){}", "const C=1", "N2001"),
        (
            "use lib::C;func main(){let x=C}",
            "private const C=1",
            "N2004",
        ),
        (
            "use lib::C;const A=false&&C;func main(){}",
            "use main::A;const C=A",
            "N3202",
        ),
        ("use lib::C;func main(){}", "const C=300 as uint8", "N3201"),
        ("use lib::C;use lib::C;func main(){}", "const C=1", "N2002"),
        (
            "use lib::C;func main(){}",
            "const C=1;func unused(){let x=missing}",
            "N2001",
        ),
        ("use lib::C;func main(){}", "const C=1", "N8001"),
    ] {
        let root = directory();
        let source = root.join("main.nova");
        let output = root.join("keep.exe");
        std::fs::write(&source, main).unwrap();
        std::fs::write(
            root.join("lib.nova"),
            if code == "N8001" {
                &[0xff][..]
            } else {
                lib.as_bytes()
            },
        )
        .unwrap();
        std::fs::write(&output, b"keep").unwrap();
        for command in ["check", "build", "run"] {
            let mut process = Command::new(env!("CARGO_BIN_EXE_nova"));
            process
                .arg(command)
                .arg(&source)
                .args(["--source-root"])
                .arg(&root)
                .env("NOVA_CLANG", "missing-clang")
                .env("NOVA_RUSTC", "missing-rustc")
                .current_dir(&root);
            if command == "build" {
                process.arg("-o").arg(&output);
            }
            let result = process.output().unwrap();
            assert_eq!(result.status.code(), Some(1));
            let stderr = String::from_utf8(result.stderr).unwrap();
            assert!(stderr.contains(code), "{stderr}");
            assert!(!root.join("target").exists());
            assert_eq!(std::fs::read(&output).unwrap(), b"keep");
        }
    }
}

#[test]
fn p11_source_root_usage_and_nested_entry_check_without_native_tools() {
    let root = directory();
    std::fs::create_dir(root.join("bin")).unwrap();
    std::fs::create_dir(root.join("helpers")).unwrap();
    let source = root.join("bin/main.nova");
    std::fs::write(&source, "use helpers::math::C;func main(){let c=C}").unwrap();
    std::fs::write(root.join("helpers/math.nova"), "public const C=1").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("check")
        .arg(&source)
        .arg("--source-root")
        .arg(&root)
        .env("NOVA_CLANG", "missing")
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(!root.join("target").exists());
    for command in ["check", "build", "run"] {
        for flags in [
            vec!["--source-root"],
            vec!["--source-root", ".", "--source-root", "."],
        ] {
            let result = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg(command)
                .arg(&source)
                .args(flags)
                .current_dir(&root)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(2));
            assert!(!root.join("target").exists());
        }
    }
    let source = root.join("fragment.nova");
    std::fs::write(&source, "use helpers::math::C;func f(){}").unwrap();
    for command in ["build", "run"] {
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg(command)
            .arg(&source)
            .env("NOVA_CLANG", "missing")
            .current_dir(&root)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(1));
        assert!(String::from_utf8(result.stderr).unwrap().contains("N2001"));
        assert!(!root.join("target").exists());
    }
}

#[test]
fn p09_float_failures_precede_tools_and_preserve_existing_output() {
    for (program, code) in [
        ("func main(){let x:float=1}", "N2101"),
        ("func main(){let x:float=1e100}", "N2102"),
        ("func main(){let x=1.0%2.0}", "N2101"),
        ("func main(){if 1.0{}}", "N3001"),
        ("const C:float=1.0f32;func main(){}", "N1002"),
        ("const A:float=B;const B:float=A;func main(){}", "N3202"),
    ] {
        let directory = directory();
        let source = directory.join("bad.nova");
        let output = directory.join("keep.exe");
        std::fs::write(&source, program).unwrap();
        std::fs::write(&output, b"keep").unwrap();
        for command in ["check", "build", "run"] {
            let mut process = Command::new(env!("CARGO_BIN_EXE_nova"));
            process
                .arg(command)
                .arg(&source)
                .env("NOVA_CLANG", "missing-clang")
                .current_dir(&directory);
            if command == "build" {
                process.arg("-o").arg(&output);
            }
            let result = process.output().unwrap();
            assert_eq!(result.status.code(), Some(1));
            assert!(String::from_utf8(result.stderr).unwrap().contains(code));
            assert!(!directory.join("target").exists());
            assert_eq!(std::fs::read(&output).unwrap(), b"keep");
        }
    }
    let directory = directory();
    let source = directory.join("float.nova");
    std::fs::write(&source, include_str!("../../../examples/floats.nova")).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("check")
        .arg(source)
        .env("NOVA_CLANG", "missing-clang")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(result.status.success());
    assert!(!directory.join("target").exists());
}

#[test]
fn p08_char_source_failures_precede_tools_and_outputs() {
    for (program, code) in [
        ("func main(){print('a')}", "N2101"),
        ("func main(){let x='a'+1}", "N2101"),
        ("const C='\\u{D800}';func main(){}", "N1002"),
        ("const C='ab';func main(){}", "N1002"),
        ("func main(){if 'a' {}}", "N3001"),
    ] {
        let directory = directory();
        let source = directory.join("bad.nova");
        std::fs::write(&source, program).unwrap();
        for command in ["check", "build", "run"] {
            let result = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg(command)
                .arg(&source)
                .env("NOVA_CLANG", "missing-clang")
                .current_dir(&directory)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(1));
            assert!(String::from_utf8(result.stderr).unwrap().contains(code));
            assert!(!directory.join("target").exists());
        }
    }
    let directory = directory();
    let source = directory.join("characters.nova");
    std::fs::write(&source, include_str!("../../../examples/characters.nova")).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("check")
        .arg(&source)
        .env("NOVA_CLANG", "missing-clang")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(result.status.success());
    assert!(!directory.join("target").exists());
}

#[test]
fn p07_integer_diagnostics_gate_tools_and_outputs() {
    for (program, code) in [
        ("func main(){let x:uint64=18446744073709551616}", "N2102"),
        ("func main(){let x:int64=1;let y:uint64=x}", "N2101"),
        ("const X:uint8=255+1;func main(){}", "N3201"),
        (
            "func main(){let a:int64=0;let b:uint64=0;let x=a+b}",
            "N2101",
        ),
    ] {
        let directory = directory();
        let source = directory.join("bad.nova");
        std::fs::write(&source, program).unwrap();
        for command in ["check", "build", "run"] {
            let result = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg(command)
                .arg(&source)
                .env("NOVA_CLANG", "missing-clang")
                .current_dir(&directory)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(1));
            assert!(String::from_utf8(result.stderr).unwrap().contains(code));
            assert!(!directory.join("target").exists());
        }
    }
    let directory = directory();
    let source = directory.join("integers.nova");
    std::fs::write(&source, include_str!("../../../examples/integers.nova")).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("check")
        .arg(&source)
        .env("NOVA_CLANG", "missing-clang")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(result.status.success());
    assert!(!directory.join("target").exists());
}
fn directory() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/cli-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&path).unwrap();
    path.canonicalize().unwrap()
}
fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_nova"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn usage_options_and_version_are_deterministic() {
    assert_eq!(run(&[]).status.code(), Some(2));
    assert_eq!(run(&["unknown", "a.nova"]).status.code(), Some(2));
    assert_eq!(run(&["check", "a.txt"]).status.code(), Some(2));
    assert_eq!(run(&["version"]).status.code(), Some(0));
    assert!(String::from_utf8(run(&["--help"]).stdout)
        .unwrap()
        .contains("nova check"));
}
#[test]
fn check_fragments_needs_no_tool_and_creates_no_artifacts() {
    let directory = directory();
    let source = directory.join("한글 파일.nova");
    std::fs::write(&source, "func f(x:int)->int{return x+1}").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .args(["check"])
        .arg(&source)
        .env("NOVA_CLANG", "missing-clang")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(result.status.success(), "{:?}", result);
    assert!(result.stdout.is_empty() && result.stderr.is_empty());
    assert!(!directory.join("target").exists());
}
#[test]
fn invalid_source_reports_stable_diagnostics_before_output_or_tools() {
    let directory = directory();
    let source = directory.join("bad.nova");
    std::fs::write(&source, "func main(){print(1)}").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("build")
        .arg(&source)
        .env("NOVA_CLANG", "missing-clang")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8(result.stderr).unwrap().contains("N2101"));
    assert!(!directory.join("target").exists());
}
#[test]
fn invalid_entry_is_rejected_before_tool_invocation_or_output() {
    let directory = directory();
    let source = directory.join("entry.nova");
    std::fs::write(&source, "func main(x:int){}").unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("run")
        .arg(&source)
        .env("NOVA_CLANG", "missing-clang")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8(result.stderr)
        .unwrap()
        .contains("main must"));
    assert!(!directory.join("target").exists());
}

#[test]
fn mutable_control_diagnostics_precede_tool_invocation_and_outputs() {
    for (program, code) in [
        ("func main(){let x=1;x=2}", "N3004"),
        ("func main(){break}", "N3002"),
        ("func main(){while 1 {continue}}", "N3001"),
        ("func main(){var x=1;x=true}", "N2101"),
    ] {
        for command in ["check", "build", "run"] {
            let directory = directory();
            let source = directory.join("bad-control.nova");
            std::fs::write(&source, program).unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg(command)
                .arg(&source)
                .env("NOVA_CLANG", "missing-clang")
                .current_dir(&directory)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(1), "{command}: {program}");
            assert!(String::from_utf8(result.stderr).unwrap().contains(code));
            assert!(result.stdout.is_empty());
            assert!(!directory.join("target").exists());
        }
    }
}

#[test]
fn loop_check_needs_no_native_tools_or_outputs() {
    let directory = directory();
    let source = directory.join("loops.nova");
    std::fs::write(&source, include_str!("../../../examples/loops.nova")).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("check")
        .arg(&source)
        .env("NOVA_CLANG", "missing-clang")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(result.status.success(), "{:?}", result);
    assert!(result.stdout.is_empty() && result.stderr.is_empty());
    assert!(!directory.join("target").exists());
}

#[test]
fn const_failures_precede_tools_and_artifacts_in_all_commands() {
    let chain = std::iter::repeat("1")
        .take(5001)
        .collect::<Vec<_>>()
        .join("+");
    for (source_text, code) in [
        ("func main(){const x=1/0}".to_owned(), "N3201"),
        (format!("func main(){{const x={chain}}}"), "N3202"),
        ("func main(){const x=1;x=2}".to_owned(), "N3004"),
        ("var x=1\nfunc main(){}".to_owned(), "N1102"),
    ] {
        for command in ["check", "build", "run"] {
            let directory = directory();
            let source = directory.join("bad-const.nova");
            std::fs::write(&source, &source_text).unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg(command)
                .arg(&source)
                .env("NOVA_CLANG", "missing-clang")
                .current_dir(&directory)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(1), "{command}: {:?}", result);
            assert!(String::from_utf8(result.stderr).unwrap().contains(code));
            assert!(result.stdout.is_empty());
            assert!(!directory.join("target").exists());
        }
    }
}

#[test]
fn const_check_uses_no_llvm_or_runtime_execution() {
    let directory = directory();
    let source = directory.join("constants.nova");
    std::fs::write(&source, include_str!("../../../examples/constants.nova")).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("check")
        .arg(&source)
        .env("NOVA_CLANG", "missing-clang")
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(result.status.success(), "{:?}", result);
    assert!(result.stdout.is_empty() && result.stderr.is_empty());
    assert!(!directory.join("target").exists());
}

#[test]
fn global_const_failures_precede_llvm_and_leave_no_artifacts() {
    let chain = std::iter::repeat("1")
        .take(5001)
        .collect::<Vec<_>>()
        .join("+");
    for (source_text, code) in [
        ("const A=B;const B=A;func main(){}".to_owned(), "N3202"),
        (format!("const A={chain};func main(){{}}"), "N3202"),
        ("const UNUSED=1/0;func main(){}".to_owned(), "N3201"),
        ("const A=1;func main(){A=2}".to_owned(), "N3004"),
    ] {
        for command in ["check", "build", "run"] {
            let directory = directory();
            let source = directory.join("globals.nova");
            std::fs::write(&source, &source_text).unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg(command)
                .arg(&source)
                .env("NOVA_CLANG", "missing-clang")
                .current_dir(&directory)
                .output()
                .unwrap();
            assert_eq!(result.status.code(), Some(1));
            assert!(String::from_utf8(result.stderr).unwrap().contains(code));
            assert!(result.stdout.is_empty());
            assert!(!directory.join("target").exists());
        }
    }
}

#[test]
fn global_const_check_fragments_need_no_tools_but_native_entry_still_requires_main() {
    for source_text in [
        "const A=B+1;const B=2",
        include_str!("../../../examples/global_constants.nova"),
    ] {
        let directory = directory();
        let source = directory.join("globals.nova");
        std::fs::write(&source, source_text).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg("check")
            .arg(&source)
            .env("NOVA_CLANG", "missing-clang")
            .current_dir(&directory)
            .output()
            .unwrap();
        assert!(result.status.success(), "{:?}", result);
        assert!(result.stdout.is_empty() && result.stderr.is_empty());
        assert!(!directory.join("target").exists());
    }
    for command in ["build", "run"] {
        let directory = directory();
        let source = directory.join("globals.nova");
        std::fs::write(&source, "const A=1").unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg(command)
            .arg(&source)
            .env("NOVA_CLANG", "missing-clang")
            .current_dir(&directory)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(1));
        assert!(String::from_utf8(result.stderr).unwrap().contains("main"));
        assert!(!directory.join("target").exists());
    }
}

#[test]
fn p10_cast_source_failures_precede_tools_but_runtime_failures_pass_check() {
    for (program, code) in [
        ("func main(){let x=true as int}", "N2101"),
        ("func main(){let x=1 as Missing}", "N2001"),
        ("func main(){let x=2147483648 as int64}", "N2102"),
        ("const X=128 as int8;func main(){}", "N3201"),
    ] {
        let directory = directory();
        let source = directory.join("bad.nova");
        let output = directory.join("keep.exe");
        std::fs::write(&source, program).unwrap();
        std::fs::write(&output, b"keep").unwrap();
        for command in ["check", "build", "run"] {
            let mut process = Command::new(env!("CARGO_BIN_EXE_nova"));
            process
                .arg(command)
                .arg(&source)
                .env("NOVA_CLANG", "missing-clang")
                .current_dir(&directory);
            if command == "build" {
                process.arg("-o").arg(&output);
            }
            let result = process.output().unwrap();
            assert_eq!(result.status.code(), Some(1));
            assert!(String::from_utf8_lossy(&result.stderr).contains(code));
            assert_eq!(std::fs::read(&output).unwrap(), b"keep");
            assert!(!directory.join("target").exists());
        }
    }
    for program in [
        include_str!("../../../examples/casts.nova"),
        "func main(){let x=128 as int8;let y=(-1) as uint8}",
    ] {
        let directory = directory();
        let source = directory.join("cast.nova");
        std::fs::write(&source, program).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg("check")
            .arg(&source)
            .env("NOVA_CLANG", "missing-clang")
            .current_dir(&directory)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(!directory.join("target").exists());
    }
}

#[test]
fn p12_struct_failures_preserve_outputs_and_gate_native_tools() {
    for (source, code) in [
        ("struct P{let text:string} func main(){}", "N1102"),
        ("struct P{let p:P} func main(){}", "N2101"),
        ("struct P{var x:int} func main(){let p=P(1);p.x=2}", "N3004"),
        ("struct P{let x:int} const P0=P(1/0);func main(){}", "N3201"),
        ("struct P{let x:int} func main(){let p=P()}", "N2201"),
    ] {
        let root = directory();
        let path = root.join("main.nova");
        let output = root.join("keep.exe");
        std::fs::write(&path, source).unwrap();
        std::fs::write(&output, b"keep").unwrap();
        for command in ["check", "build", "run"] {
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_nova"));
            cmd.arg(command)
                .arg(&path)
                .env("NOVA_CLANG", "missing-clang")
                .env("NOVA_RUSTC", "missing-rustc")
                .current_dir(&root);
            if command == "build" {
                cmd.arg("-o").arg(&output);
            }
            let result = cmd.output().unwrap();
            assert_eq!(result.status.code(), Some(1));
            assert!(
                String::from_utf8_lossy(&result.stderr).contains(code),
                "{:?}",
                result
            );
            assert_eq!(std::fs::read(&output).unwrap(), b"keep");
        }
    }
}

#[test]
fn p13_tuple_failures_preserve_outputs_and_gate_native_tools() {
    for (source, code) in [
        ("func main(){let t=(1,);let x=t.9}", "N2001"),
        ("func main(){let t=(1,);t.0=2}", "N3004"),
        ("const C=(1/0,);func main(){}", "N3201"),
        ("func main(){let t=(1,);let x=t.0.1e2}", "N1102"),
        ("func main(){let t:(int,bool)=(1,2)}", "N2101"),
    ] {
        let root = directory();
        let path = root.join("main.nova");
        let output = root.join("keep.exe");
        std::fs::write(&path, source).unwrap();
        std::fs::write(&output, b"keep").unwrap();
        for command in ["check", "build", "run"] {
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_nova"));
            cmd.arg(command)
                .arg(&path)
                .env("NOVA_CLANG", "missing-clang")
                .env("NOVA_RUSTC", "missing-rustc")
                .current_dir(&root);
            if command == "build" {
                cmd.arg("-o").arg(&output);
            }
            let result = cmd.output().unwrap();
            assert_eq!(result.status.code(), Some(1));
            assert!(
                String::from_utf8_lossy(&result.stderr).contains(code),
                "{:?}",
                result
            );
            assert_eq!(std::fs::read(&output).unwrap(), b"keep");
        }
    }
}

#[test]
fn p14_enum_failures_preserve_outputs_and_gate_native_tools() {
    for (source, code) in [
        ("enum E{A;B;}func main(){match E::A{E::A=>{}}}", "N3101"),
        ("enum E{A;}func main(){match E::A{_=>{},E::A=>{}}}", "N3102"),
        (
            "enum E{A(int);}func main(){match E::A(1){E::A(x)=>{x=2}}}",
            "N3004",
        ),
        ("enum E{A(int);}const C=E::A(1/0);func main(){}", "N3201"),
        ("enum E{A(string);}func main(){}", "N1102"),
    ] {
        let root = directory();
        let path = root.join("main.nova");
        let output = root.join("keep.exe");
        std::fs::write(&path, source).unwrap();
        std::fs::write(&output, b"keep").unwrap();
        for command in ["check", "build", "run"] {
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_nova"));
            cmd.arg(command)
                .arg(&path)
                .env("NOVA_CLANG", "missing-clang")
                .env("NOVA_RUSTC", "missing-rustc")
                .current_dir(&root);
            if command == "build" {
                cmd.arg("-o").arg(&output);
            }
            let result = cmd.output().unwrap();
            assert_eq!(result.status.code(), Some(1));
            assert!(
                String::from_utf8_lossy(&result.stderr).contains(code),
                "{:?}",
                result
            );
            assert_eq!(std::fs::read(&output).unwrap(), b"keep");
        }
    }
}

#[test]
fn p15_sum_failures_preserve_outputs_and_gate_native_tools() {
    for (source, code) in [
        ("func main(){let x=none}", "N2103"),
        ("func main(){let x=Result::Success(1)}", "N2103"),
        ("func main(){let x:int?=1}", "N2101"),
        ("func main(x:int?){match x{none=>{}}}", "N3101"),
        ("const C:int8?=Option::Some(127+1);func main(){}", "N3201"),
    ] {
        let root = directory();
        let path = root.join("main.nova");
        let output = root.join("keep.exe");
        std::fs::write(&path, source).unwrap();
        std::fs::write(&output, b"keep").unwrap();
        for command in ["check", "build", "run"] {
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_nova"));
            cmd.arg(command)
                .arg(&path)
                .env("NOVA_CLANG", "missing-clang")
                .env("NOVA_RUSTC", "missing-rustc")
                .current_dir(&root);
            if command == "build" {
                cmd.arg("-o").arg(&output);
            }
            let r = cmd.output().unwrap();
            assert_eq!(r.status.code(), Some(1));
            assert!(String::from_utf8_lossy(&r.stderr).contains(code), "{r:?}");
            assert_eq!(std::fs::read(&output).unwrap(), b"keep");
        }
    }
}

#[test]
fn p16_try_errors_gate_native_tools_and_preserve_existing_outputs() {
    for (source, code) in [
        ("func main(){let x=try 1}", "N2101"),
        ("func f(r:Result<int,bool>){let x=try r}", "N3002"),
        (
            "const R:Result<int,bool>=Result::Success(1);const X=try R;func main(){}",
            "N3201",
        ),
        (
            "func f(r:Result<int,int8>)->Result<int,int16>{let x=try r;return Result::Success(x)}",
            "N2101",
        ),
    ] {
        let root = directory();
        let path = root.join("main.nova");
        let output = root.join("keep.exe");
        std::fs::write(&path, source).unwrap();
        std::fs::write(&output, b"keep").unwrap();
        for command in ["check", "build", "run"] {
            let mut cmd = Command::new(env!("CARGO_BIN_EXE_nova"));
            cmd.arg(command)
                .arg(&path)
                .env("NOVA_CLANG", "missing-clang")
                .env("NOVA_RUSTC", "missing-rustc")
                .current_dir(&root);
            if command == "build" {
                cmd.arg("-o").arg(&output);
            }
            let r = cmd.output().unwrap();
            assert_eq!(r.status.code(), Some(1));
            assert!(String::from_utf8_lossy(&r.stderr).contains(code), "{r:?}");
            assert_eq!(std::fs::read(&output).unwrap(), b"keep");
        }
    }
}
