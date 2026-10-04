//! Opt-in real LLVM/MSVC Native evidence. Run with NOVA_CLANG and --ignored.
#![cfg(all(target_os = "windows", target_arch = "x86_64"))]
use std::path::PathBuf;
use std::process::{Command, Stdio};
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
    let mut child = Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("run")
        .arg(path)
        .args(["--profile", profile])
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let started = std::time::Instant::now();
    while child.try_wait().unwrap().is_none() {
        if started.elapsed() > std::time::Duration::from_secs(40) {
            // Kill the test-owned CLI and its native child if a loop regresses.
            let _ = Command::new("taskkill")
                .args(["/PID", &child.id().to_string(), "/T", "/F"])
                .output();
            let _ = child.kill();
            let result = child.wait_with_output().unwrap();
            panic!(
                "Native test timed out: {}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    child.wait_with_output().unwrap()
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
fn global_const_example_is_exact_in_both_native_profiles() {
    for profile in ["debug", "release"] {
        let result = program_profile(
            include_str!("../../../examples/global_constants.nova"),
            profile,
        );
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, "합계: 15, skipped=false\n".as_bytes());
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn global_boundaries_unicode_unit_shadowing_and_runtime_arithmetic_are_native() {
    let source = "func unit(u:()){print(\"unit\")} func get()->int{return MAX} func main(){const MIN=MIN;var MAX=MAX;MAX=MAX-1;print(\"{MIN} {MAX} {DIV} {REM} {SAFE} {get()}\");print(TEXT);unit(U)} const MIN=-2147483648;const MAX=2147483647;const DIV=-7/3;const REM=-7%3;const SAFE=true||(1/0==0);const TEXT=ALIAS;const ALIAS=\"한글\\0🙂\";const U=()";
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "-2147483648 2147483646 -2 -1 true 2147483647\n한글\0🙂\nunit\n".as_bytes()
        );
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn loops_mutable_places_and_nested_jumps_match_in_both_profiles() {
    let nested = "func main(){var i=0;var sum=0;while i<3 {i=i+1;var j=0;while j<4 {j=j+1;if j==2 {continue} if j==3 {break} sum=sum+1} sum=sum+10} print(\"{sum} {i}\")}";
    for profile in ["debug", "release"] {
        for (source, stdout) in [
            (
                include_str!("../../../examples/loops.nova"),
                b"sum=8, index=5\n".as_slice(),
            ),
            (nested, b"33 3\n".as_slice()),
        ] {
            let result = program_profile(source, profile);
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert_eq!(result.stdout, stdout);
            assert!(result.stderr.is_empty());
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn loop_condition_initializer_and_unreachable_effect_counts_are_native() {
    let source = "func cond(i:int)->bool{print(\"cond {i}\");return i<3} func make(i:int)->int{print(\"init {i}\");return i} func f()->int{var i=0;while cond(i) {let x=make(i);i=i+1;if x<2 {continue}else{return i};print(\"never\")} return 9} func main(){while false {print(\"never\")} print(\"return={f()}\")}";
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            b"cond 0\ninit 0\ncond 1\ninit 1\ncond 2\ninit 2\nreturn=3\n"
        );
        let result = program_profile("func cond(i:int)->bool{print(\"cond\");return i<2} func main(){var i=0;while cond(i) {i=i+1;continue} print(\"{i}\")}", profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, b"cond\ncond\ncond\n2\n");
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn bool_string_unit_mutation_shadowing_and_previous_strings_are_preserved() {
    let source = "func main(){var run=true;var text=\"초기{0}\\0\";let old=text;var unit:()=();var i=0;while run {var text=\"내부 {i}\";print(text);i=i+1;run=i<2;unit=()} text=\"끝 {i}\";print(\"{old}|{text}|{run}\");print(\"done\")}";
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "내부 0\n내부 1\n초기0\0|끝 2|false\ndone\n".as_bytes()
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn repeated_checked_failure_aborts_before_following_effects() {
    for profile in ["debug", "release"] {
        for source in [
            "func main(){var i=2147483646;while true {print(\"{i}\");i=i+1} print(\"never\")}",
            "func main(){var i=1;while 1/i==1 {print(\"body\");i=0} print(\"never\")}",
        ] {
            let result = program_profile(source, profile);
            assert!(!result.status.success());
            let stderr = String::from_utf8_lossy(&result.stderr);
            assert!(stderr.contains("file#0:"), "{stderr}");
            assert_eq!(
                result.stdout,
                if source.contains("2147483646") {
                    b"2147483646\n2147483647\n".as_slice()
                } else {
                    b"body\n".as_slice()
                }
            );
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn const_example_executes_identically_in_both_profiles() {
    for profile in ["debug", "release"] {
        let result = program_profile(include_str!("../../../examples/constants.nova"), profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, "합계: 15, skipped=false\n".as_bytes());
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn const_boundaries_signed_arithmetic_strings_unit_and_shadowing_are_native() {
    let source = "func value()->int{const base=7;const q=-base/3;const r=-base%3;return q*10+r} func keep(x:()){return x} func main(){const min=-2147483648;const max=2147483647;const text=\"한글\\0\";const alias=(text);const empty=();const skipped=true||(1/0==0);keep(empty);var i=0;while i<2 {const shadow=3;if i==0 {const shadow=shadow+1;print(\"{shadow}\")} i=i+1} print(\"{alias}|{min}|{max}|{value()}|{skipped}\")}";
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "4\n한글\0|-2147483648|2147483647|-21|true\n".as_bytes()
        );
        assert!(result.stderr.is_empty());
    }
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
