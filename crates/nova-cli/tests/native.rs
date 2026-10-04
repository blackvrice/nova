//! Opt-in real LLVM/MSVC Native evidence. Run with NOVA_CLANG and --ignored.
#![cfg(all(target_os = "windows", target_arch = "x86_64"))]
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
fn program(source: &str) -> std::process::Output {
    program_profile(source, "debug")
}
fn program_profile(source: &str, profile: &str) -> std::process::Output {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/native-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&root).unwrap();
    let root = root.canonicalize().unwrap();
    let path = root.join("한글 프로그램.nova");
    std::fs::write(&path, source).unwrap();
    Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("run")
        .arg(path)
        .args(["--profile", profile])
        .current_dir(root)
        .output()
        .unwrap()
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn hello_is_exact_utf8_lf_and_exit_zero() {
    let result = program("func main(){print(\"Hello, Nova\")}");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(result.stdout, b"Hello, Nova\n");
    assert!(result.stderr.is_empty());
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn interpolation_unicode_nul_and_int_boundaries_are_exact() {
    let result = program(
        "func main(){print(\"한글 A\\0B {-2147483648} {2147483647} {true} {false}\");print(\"\")}",
    );
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        result.stdout,
        "한글 A\0B -2147483648 2147483647 true false\n\n".as_bytes()
    );
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn call_order_short_circuit_recursion_and_branches_are_native() {
    let result=program("func a()->int{print(\"a\");return 1} func b()->int{print(\"b\");return 2} func sum(x:int,y:int)->int{return x+y} func rhs()->bool{print(\"rhs\");return true} func r(x:int)->int{if x==0{return 0}else{return r(x-1)+1}} func main(){print(\"{sum(a(),b())}\");let x=false&&rhs();let y=true||rhs();print(\"{x} {y} {r(3)}\")}");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(result.stdout, b"a\nb\n3\nfalse true 3\n");
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn signed_division_remainder_and_safe_edges_are_native() {
    let result=program("func main(){let m=-2147483648;print(\"{-7/3} {-7%3} {7/-3} {7%-3} {m/1} {2147483646+1} {m+1} {2*3} {2-3}\")}");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        result.stdout,
        b"-2 -1 -2 1 -2147483648 2147483647 -2147483647 6 -1\n"
    );
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn all_checked_arithmetic_failures_abort_with_source() {
    for profile in ["debug", "release"] {
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
            let result = program_profile(
                &format!("func main(){{let x={expression};print(\"must not run\")}}"),
                profile,
            );
            assert!(!result.status.success(), "{expression}");
            assert!(result.stdout.is_empty(), "{expression}");
            let stderr = String::from_utf8_lossy(&result.stderr);
            assert!(
                stderr.contains("Nova panic:") && stderr.contains("file#0:"),
                "{expression}: {stderr}"
            );
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn optimized_execution_matches_debug_for_control_and_strings() {
    let source="func b(x:bool)->bool{print(\"rhs\");return !x} func text(x:int)->string{return \"한글 {x}\"} func main(){let x=true||b(false);let y=false&&b(true);print(\"{text(2147483647)} {x} {y} {-7%3}\")} ";
    let debug = program_profile(source, "debug");
    let optimized = program_profile(source, "release");
    assert!(
        debug.status.success(),
        "{}",
        String::from_utf8_lossy(&debug.stderr)
    );
    assert!(
        optimized.status.success(),
        "{}",
        String::from_utf8_lossy(&optimized.stderr)
    );
    assert_eq!(debug.stdout, optimized.stdout);
    assert_eq!(debug.stdout, "한글 2147483647 true false -1\n".as_bytes());
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn repeated_builds_are_deterministic_and_existing_outputs_are_preserved() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/native-build-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    let source = directory.join("한글 소스.nova");
    std::fs::write(&source, "func main(){print(\"Hello, Nova\")}").unwrap();
    let build = |path: &std::path::Path| {
        Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg("build")
            .arg(&source)
            .arg("-o")
            .arg(path)
            .current_dir(&directory)
            .output()
            .unwrap()
    };
    let first = directory.join("first.exe");
    let second = directory.join("second.exe");
    for path in [&first, &second] {
        let result = build(path);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    assert_eq!(
        std::fs::read(&first).unwrap(),
        std::fs::read(&second).unwrap()
    );
    let before = std::fs::read(&first).unwrap();
    let result = build(&first);
    assert_eq!(result.status.code(), Some(3));
    assert_eq!(std::fs::read(&first).unwrap(), before);
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn stdout_io_failure_aborts_with_the_call_span() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/native-io-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&directory).unwrap();
    let directory = directory.canonicalize().unwrap();
    let source = directory.join("source.nova");
    std::fs::write(&source, "func main(){print(\"hello\")}").unwrap();
    let executable = directory.join("program.exe");
    let built = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("build")
        .arg(source)
        .arg("-o")
        .arg(&executable)
        .current_dir(&directory)
        .output()
        .unwrap();
    assert!(
        built.status.success(),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );
    let sink = directory.join("readonly.txt");
    std::fs::write(&sink, b"sentinel").unwrap();
    let file = std::fs::File::open(&sink).unwrap();
    let result = Command::new(executable).stdout(file).output().unwrap();
    assert!(!result.status.success());
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(
        stderr.contains("stdout write failed") && stderr.contains("file#0:12..26"),
        "{stderr}"
    );
    assert_eq!(std::fs::read(&sink).unwrap(), b"sentinel");
}
