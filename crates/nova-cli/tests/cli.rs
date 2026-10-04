use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

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
