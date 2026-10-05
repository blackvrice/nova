//! Opt-in real LLVM/MSVC Native evidence. Run with NOVA_CLANG and --ignored.
#![cfg(all(target_os = "windows", target_arch = "x86_64"))]
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn module_program(main: &str, lib: &str, profile: &str) -> std::process::Output {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/native-module-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&root).unwrap();
    let root = root.canonicalize().unwrap();
    std::fs::create_dir(root.join("bin")).unwrap();
    std::fs::write(root.join("bin/main.nova"), main).unwrap();
    std::fs::write(root.join("lib.nova"), lib).unwrap();
    Command::new(env!("CARGO_BIN_EXE_nova"))
        .arg("run")
        .arg(root.join("bin/main.nova"))
        .arg("--source-root")
        .arg(&root)
        .args(["--profile", profile])
        .current_dir(root)
        .output()
        .unwrap()
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p11_native_fixture_cycles_recursion_effect_order_string_lifetime_and_entry() {
    for profile in ["debug", "release"] {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/native-module-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
        std::fs::create_dir_all(&root).unwrap();
        let root = root.canonicalize().unwrap();
        for (name, text) in [
            (
                "main.nova",
                include_str!("../../../docs/development-v0.1/module-proposal-fixtures/main.nova"),
            ),
            (
                "math.nova",
                include_str!("../../../docs/development-v0.1/module-proposal-fixtures/math.nova"),
            ),
        ] {
            std::fs::write(root.join(name), text).unwrap();
        }
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg("run")
            .arg(root.join("main.nova"))
            .args(["--profile", profile])
            .current_dir(root)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, b"value=42\n");
        assert!(result.stderr.is_empty());
        let main = r#"use lib::left;use lib::right;use lib::rec;use lib::echo;use lib::main as peer
use lib::values;use lib::F;use lib::C;use lib::B;use lib::U
func twice(n:int)->int{return n+n}
func main(){let x=left()+right();var s="before";let saved=echo(s);s="after";print("{x} {rec(4)} {saved} {s} {peer(3)}");print(values(F,C,B,U))}"#;
        let lib = r#"use bin::main::twice
func left()->int{print("left");return 1}
func right()->int{print("right");return 2}
func rec(n:int)->int{if n==0{return 0}else{return 1+rec(n-1)}}
func echo(s:string)->string{return "{s}"}
func main(n:int)->int{return twice(n)}
const F:double=7 as double;const C='🙂';const B=true;const U=()
func values(x:double,c:char,b:bool,u:())->string{return "{x} {c} {b}"}"#;
        let result = module_program(main, lib, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "left\nright\n3 4 before after 6\n7 🙂 true\n".as_bytes()
        );
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p11_cross_file_checked_arithmetic_and_cast_abort_keep_exact_file_and_utf8_span() {
    for (expression, body, reason) in [
        (
            "a+b",
            "let a:int8=127;let b:int8=1;let x=a+b",
            "integer overflow",
        ),
        (
            "a as uint8",
            "let a:int=300;let x=a as uint8",
            "numeric cast out of range",
        ),
    ] {
        let lib = format!("// 한글 🙂\r\npublic func fail(){{{body};print(\"unreachable\")}}\r\n");
        let start = lib.find(expression).unwrap();
        for profile in ["debug", "release"] {
            let result = module_program(
                "use lib::fail;func main(){fail();print(\"unreachable\")}",
                &lib,
                profile,
            );
            assert!(!result.status.success());
            assert!(result.stdout.is_empty());
            let stderr = String::from_utf8(result.stderr).unwrap();
            assert_eq!(
                stderr.lines().next(),
                Some(
                    format!(
                        "Nova panic: {reason} at file#1:{start}..{}",
                        start + expression.len()
                    )
                    .as_str()
                ),
                "{stderr}"
            );
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p09_float_native_example_and_rational_operation_oracle_in_o0_o2() {
    for profile in ["debug", "release"] {
        for (source,expected) in [
            (include_str!("../../../examples/floats.nova"),"value=0.75, mixed=2.75, total=3.75\ninf=inf, nan=NaN, negzero=-0, eq=false, ne=true\n"),
            ("const A=0.1;const B:double=-0.0;func main(){print(\"{A} {B}\")}","0.1 -0\n"),
            (include_str!("../../../tools/tests/fixtures/float-native.nova"),include_str!("../../../tools/tests/fixtures/float-native.stdout.txt")),
        ] {
            let result=program_profile(source,profile);
            assert!(result.status.success(),"{profile}: {}",String::from_utf8_lossy(&result.stderr));
            assert_eq!(result.stdout,expected.as_bytes(),"{profile}");
            assert!(result.stderr.is_empty());
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p09_float_native_ieee_abi_order_mutation_and_no_contraction() {
    let source = r#"
func f(a:float)->float{return a}
func d(a:double)->double{return a}
func left()->float{print("left");return 1.0}
func right()->double{print("right");return 2.0}
func main(){
let z=f(0.0);let nz=f(-0.0);let n=z/z;let inf=f(1.0)/z
print("{n} {-n} {inf} {f(1.0)/nz} {n==n} {n!=n} {n<z} {n<=z} {n>z} {n>=z} {z==nz}")
let sub=f(1e-45);print("{sub*f(1.0)} {f(3.4028235e38)*f(2.0)}")
let a=f(16777216.0);let b=f(1.0);let c=f(-16777216.0);print("{(a+b)+c} {a+(b+c)}")
let x=f(1.00000011920928955078125);let y=f(0.99999988079071044921875);let neg=f(-1.0);print("{x*y+neg}")
let aa=d(9007199254740992.0);let bb=d(1.0);let cc=d(-9007199254740992.0);print("{(aa+bb)+cc} {aa+(bb+cc)}")
let dx=d(1.0000000000000002220446049250313080847263336181640625);let dy=d(0.9999999999999997779553950749686919152736663818359375);print("{dx*dy+d(-1.0)}")
let order=left()+right();print("{order}")
var value:float=0.5;let saved="{value}";while value<1.0{value=value+0.25;continue};print("{saved} {value}")
let i:int16=-2;let u:uint32=4294967295;let mix:double=value+i+u;print("{mix}")
print("{f(2097152.25)} {d(562949953421312.25)}")
}

"#;
    let expected="NaN NaN inf -inf false true false false false false true\n0.000000000000000000000000000000000000000000001 inf\n0 1\n0\n0 1\n0\nleft\nright\n3\n0.5 1\n4294967294\n2097152.2 562949953421312.2\n";
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{profile}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, expected.as_bytes(), "{profile}");
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires Windows x64 MSVC Rust linker"]
fn p09_runtime_bits_formatter_oracle_and_host_initialization_o0_o2() {
    let runtime = include_str!("../runtime/stage_a.rs")
        .lines()
        .filter(|line| !line.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n");
    let fixture = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/tests/fixtures/float-format.tsv")
        .canonicalize()
        .unwrap();
    let fixture = fixture.to_string_lossy().replace('\\', "/");
    let source = format!(
        r#"
#[allow(dead_code)]mod runtime{{
{runtime}
pub unsafe fn print_value(v:NovaString){{unsafe{{nova_print(v.ptr,v.len,7,11,19);}}}}
}}
#[no_mangle]pub extern "C" fn nova_stage_a_entry(){{}}
fn main(){{
let bad=0xffc0u32;unsafe{{std::arch::asm!("ldmxcsr [{{p}}]",p=in(reg)&bad,options(nostack));}}
runtime::initialize_float_environment();
let mut actual=0u32;unsafe{{std::arch::asm!("stmxcsr [{{p}}]",p=in(reg)&mut actual,options(nostack));}}
assert_eq!(actual,0x1f80);
for row in include_str!("{fixture}").lines(){{
let row:Vec<_>=row.split('\t').collect();let bits=u64::from_str_radix(row[1],16).unwrap();
let mut out=std::mem::MaybeUninit::<runtime::NovaString>::uninit();
unsafe{{if row[0]=="32"{{runtime::nova_format_f32(out.as_mut_ptr(),bits as u32,7,11,19);}}
else{{runtime::nova_format_f64(out.as_mut_ptr(),bits,7,11,19);}}runtime::print_value(out.assume_init());}}
}}
}}
"#
    );
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/runtime-float-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&root).unwrap();
    let path = root.join("harness.rs");
    std::fs::write(&path, source).unwrap();
    let expected = include_str!("../../../tools/tests/fixtures/float-format.tsv")
        .lines()
        .map(|row| row.split('\t').nth(2).unwrap())
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    for level in ["0", "2"] {
        let executable = root.join(format!("float-o{level}.exe"));
        let compiler =
            Command::new(std::env::var_os("NOVA_RUSTC").unwrap_or_else(|| "rustc".into()))
                .arg(&path)
                .args([
                    "--edition=2021",
                    "-Cpanic=abort",
                    "-C",
                    &format!("opt-level={level}"),
                    "-o",
                ])
                .arg(&executable)
                .output()
                .unwrap();
        assert!(
            compiler.status.success(),
            "{}",
            String::from_utf8_lossy(&compiler.stderr)
        );
        let result = Command::new(executable).output().unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, expected.as_bytes());
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p08_char_example_and_global_only_formatter_match_in_both_profiles() {
    for profile in ["debug", "release"] {
        for (source, expected) in [
            (
                include_str!("../../../examples/characters.nova"),
                "letter=가, face=🙂, brace={, ordered=true\n",
            ),
            ("const C='🙂';func main(){print(\"{C}\")}", "🙂\n"),
        ] {
            let result = program_profile(source, profile);
            assert!(
                result.status.success(),
                "{profile}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert_eq!(result.stdout, expected.as_bytes());
            assert!(result.stderr.is_empty());
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p08_char_utf8_controls_comparison_abi_order_and_string_lifetimes_are_native() {
    let mut source = String::from("func echo(x:char)->char{return x} func left()->char{print(\"left\");return 'a'} func right()->char{print(\"right\");return 'b'} func main(){");
    let mut expected = Vec::new();
    for (i, value) in [
        0u32, 0x7f, 0x80, 0x7ff, 0x800, 0xd7ff, 0xe000, 0xffff, 0x10000, 0x10ffff, 0x378, 10, 13,
        9, 39, 34, 92, 123, 125,
    ]
    .iter()
    .enumerate()
    {
        source.push_str(&format!("const c{i}:char='\\u{{{value:X}}}';var v{i}:char=echo(c{i});let old{i}=\"{{v{i}}}\";v{i}='x';print(old{i});print(\"{{v{i}}}\");"));
        let scalar = char::from_u32(*value).unwrap();
        expected.extend_from_slice(scalar.encode_utf8(&mut [0; 4]).as_bytes());
        expected.extend_from_slice(b"\nx\n");
    }
    for (i, op) in ["==", "!=", "<", "<=", ">", ">="].iter().enumerate() {
        for (j, (a, b)) in [
            (0x61u32, 0x61u32),
            (0x61, 0x62),
            (0xd7ff, 0xe000),
            (0x10ffff, 0),
        ]
        .iter()
        .enumerate()
        {
            source.push_str(&format!("const c{i}_{j}='\\u{{{a:X}}}'{op}'\\u{{{b:X}}}';let a{i}_{j}=echo('\\u{{{a:X}}}');let b{i}_{j}=echo('\\u{{{b:X}}}');let r{i}_{j}=a{i}_{j}{op}b{i}_{j};print(\"{{c{i}_{j}}} {{r{i}_{j}}}\");"));
            let value = match *op {
                "==" => a == b,
                "!=" => a != b,
                "<" => a < b,
                "<=" => a <= b,
                ">" => a > b,
                ">=" => a >= b,
                _ => unreachable!(),
            };
            expected.extend_from_slice(format!("{value} {value}\n").as_bytes());
        }
    }
    source.push_str("let ordered=left()<right();print(\"{ordered}\");var ch='a';while ch<'z'{ch='z';continue};print(\"{ch}\")}");
    expected.extend_from_slice(b"left\nright\ntrue\nz\n");
    for profile in ["debug", "release"] {
        let result = program_profile(&source, profile);
        assert!(
            result.status.success(),
            "{profile}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, expected);
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires Windows x64 MSVC Rust linker"]
fn p08_private_runtime_invalid_scalar_aborts_with_source_in_o0_o2() {
    // Compile the actual runtime inside a module; the test supplies its link bridge.
    // This exercises invalid ABI values without introducing a Nova source cast.
    let runtime = include_str!("../runtime/stage_a.rs")
        .lines()
        .filter(|line| !line.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n");
    let source = format!(
        r#"
#[allow(dead_code)] mod runtime {{
{runtime}
pub unsafe fn print_value(value:NovaString) {{ unsafe {{nova_print(value.ptr,value.len,7,11,19)}} }}
}}
#[no_mangle] pub extern "C" fn nova_stage_a_entry() {{}}
fn main() {{
    let value:u32=std::env::args().nth(1).unwrap().parse().unwrap();
    let mut out=std::mem::MaybeUninit::<runtime::NovaString>::uninit();
    unsafe {{runtime::nova_format_char(out.as_mut_ptr(),value,7,11,19);runtime::print_value(out.assume_init());}}
}}
"#
    );
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/runtime-char-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&root).unwrap();
    let path = root.join("harness.rs");
    std::fs::write(&path, source).unwrap();
    for level in ["0", "2"] {
        let executable = root.join(format!("char-o{level}.exe"));
        let compiler =
            Command::new(std::env::var_os("NOVA_RUSTC").unwrap_or_else(|| "rustc".into()))
                .arg(&path)
                .args([
                    "--edition=2021",
                    "-Cpanic=abort",
                    "-C",
                    &format!("opt-level={level}"),
                    "-o",
                ])
                .arg(&executable)
                .output()
                .unwrap();
        assert!(
            compiler.status.success(),
            "{}",
            String::from_utf8_lossy(&compiler.stderr)
        );
        for value in [
            0u32, 0x7f, 0x80, 0x7ff, 0x800, 0xd7ff, 0xe000, 0xffff, 0x10000, 0x10ffff,
        ] {
            let result = Command::new(&executable)
                .arg(value.to_string())
                .output()
                .unwrap();
            assert!(result.status.success());
            let mut expected = char::from_u32(value).unwrap().to_string().into_bytes();
            expected.push(b'\n');
            assert_eq!(result.stdout, expected);
            assert!(result.stderr.is_empty());
        }
        for value in [0xd800u32, 0xdfff, 0x110000, u32::MAX] {
            let result = Command::new(&executable)
                .arg(value.to_string())
                .output()
                .unwrap();
            assert!(!result.status.success());
            assert!(result.stdout.is_empty());
            assert_eq!(
                result.stderr,
                b"Nova panic: invalid char scalar at file#7:11..19\n"
            );
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p07_example_and_all_width_arithmetic_abi_formatting_match_const_in_both_profiles() {
    let mut source = String::new();
    let cases = [
        ("int8", -128i128, 127i128),
        ("uint8", 0, 255),
        ("int16", -32768, 32767),
        ("uint16", 0, 65535),
        ("int32", -2147483648, 2147483647),
        ("uint32", 0, 4294967295),
        ("int64", -9223372036854775808, 9223372036854775807),
        ("uint64", 0, 18446744073709551615),
    ];
    for (i, (ty, _, _)) in cases.iter().enumerate() {
        source.push_str(&format!("func f{i}(x:{ty})->{ty}{{return x}} "));
    }
    source.push_str("func main(){");
    let mut expected = String::new();
    for (i, (ty, min, max)) in cases.iter().enumerate() {
        source.push_str(&format!("let min{i}:{ty}={min};let max{i}:{ty}=f{i}({max});let a{i}:{ty}=7;let b{i}:{ty}=3;const c{i}:{ty}=7+3;let add{i}=a{i}+b{i};let sub{i}=a{i}-b{i};let mul{i}=a{i}*b{i};let div{i}=a{i}/b{i};let rem{i}=a{i}%b{i};let cmp{i}=max{i}>0;print(\"{{min{i}}} {{max{i}}} {{add{i}}} {{sub{i}}} {{mul{i}}} {{div{i}}} {{rem{i}}} {{cmp{i}}} {{c{i}}}\");"));
        expected.push_str(&format!("{min} {max} 10 4 21 2 1 true 10\n"));
        if *min < 0 {
            source.push_str(&format!("let n{i}=-a{i};let q{i}=n{i}/b{i};let r{i}=n{i}%b{i};print(\"{{n{i}}} {{q{i}}} {{r{i}}}\");"));
            expected.push_str("-7 -2 -1\n");
        }
    }
    source.push('}');
    for profile in ["debug", "release"] {
        for (source, expected) in [
            (
                include_str!("../../../examples/integers.nova"),
                "mixed=254, wide=253, min=-9223372036854775808, max=18446744073709551615\n",
            ),
            (source.as_str(), expected.as_str()),
            (
                "const X:uint64=18446744073709551615;func main(){print(\"{X}\")}",
                "18446744073709551615\n",
            ),
        ] {
            let result = program_profile(source, profile);
            assert!(
                result.status.success(),
                "{profile}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert_eq!(result.stdout, expected.as_bytes());
            assert!(result.stderr.is_empty());
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p07_all_width_abort_boundaries_have_exact_source_in_both_profiles() {
    for (ty, min, max) in [
        ("int8", -128i128, 127i128),
        ("uint8", 0, 255),
        ("int16", -32768, 32767),
        ("uint16", 0, 65535),
        ("int32", -2147483648, 2147483647),
        ("uint32", 0, 4294967295),
        ("int64", -9223372036854775808, 9223372036854775807),
        ("uint64", 0, 18446744073709551615),
    ] {
        let mut cases = vec![
            (max, 1, "a+b"),
            (min, 1, "a-b"),
            (max, 2, "a*b"),
            (1, 0, "a/b"),
            (1, 0, "a%b"),
        ];
        if min < 0 {
            cases.extend([(min, -1, "a/b"), (min, -1, "a%b"), (min, 0, "-a")]);
        }
        for (a, b, expr) in cases {
            let source = format!(
                "func main(){{let a:{ty}={a};let b:{ty}={b};let x={expr};print(\"unreachable\")}}"
            );
            let start = source.find(&format!("={expr};")).unwrap() + 1;
            let zero = b == 0 && matches!(expr, "a/b" | "a%b");
            let reason = if zero {
                "division or remainder by zero"
            } else if ty == "int32" {
                "Int32 overflow"
            } else {
                "integer overflow"
            };
            for profile in ["debug", "release"] {
                let result = program_profile(&source, profile);
                assert!(!result.status.success(), "{ty} {expr} {profile}");
                assert!(result.stdout.is_empty());
                let stderr = String::from_utf8(result.stderr).unwrap();
                assert!(
                    stderr.starts_with("Nova panic:"),
                    "{ty} {expr} {profile}: {stderr}"
                );
                let mut lines = stderr.lines();
                assert_eq!(
                    lines.next(),
                    Some(
                        format!(
                            "Nova panic: {reason} at file#0:{start}..{}",
                            start + expr.len()
                        )
                        .as_str()
                    )
                );
                // P03 CLI also reports Windows abort status when it cannot fit u8.
                assert!(
                    lines.all(|line| line.starts_with("Native process terminated: Some(")),
                    "{stderr}"
                );
            }
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p07_result_widening_preserves_operation_width_and_operand_source_order() {
    for profile in ["debug", "release"] {
        let result=program_profile("func f(x:int64)->int64{return x} func g(x:uint64)->uint64{return x} func main(){let a:byte=255;let s:int8=-128;var r:int64=0;r=s;print(\"{f(s)} {g(a)} {r}\")}",profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, b"-128 255 -128\n");
        let result=program_profile("func a()->int8{print(\"left\");return 100} func b()->uint8{print(\"right\");return 200} func main(){let x:int64=a()+b();print(\"{x}\")}",profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, b"left\nright\n300\n");
        let result = program_profile(
            "func main(){let a:int8=127;let b:int8=1;let x:int64=a+b}",
            profile,
        );
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("integer overflow"));
        let result = program_profile(
            "func main(){let a:int8=127;let x:int64=a+1;print(\"{x}\")}",
            profile,
        );
        assert!(result.status.success());
        assert_eq!(result.stdout, b"128\n");
    }
}
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

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p10_cast_native_example_and_exact_rational_const_runtime_oracle_o0_o2() {
    for profile in ["debug", "release"] {
        for (source,expected) in [
            (include_str!("../../../examples/casts.nova"),"small=127, whole=127, zero=0, skipped=false\nrounded=16777216, value=2.25, back=2\n"),
            (include_str!("../../../tools/tests/fixtures/cast-native.nova"),include_str!("../../../tools/tests/fixtures/cast-native.stdout.txt")),
            ("func value()->double{print(\"once\");return 127.9}func main(){var x=value() as int8;print(\"{x}\");x=(x as int16+1) as int8}","once\n127\n"),
        ] {
            let result=program_profile(source,profile);
            if source.contains("var x=value()") { assert!(!result.status.success());assert!(String::from_utf8_lossy(&result.stderr).contains("numeric cast out of range")); }
            else {assert!(result.status.success(),"{profile}: {}",String::from_utf8_lossy(&result.stderr));assert!(result.stderr.is_empty());}
            assert_eq!(result.stdout,expected.as_bytes(),"{profile}");
        }
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p10_cast_native_abort_boundaries_nan_infinity_and_full_unicode_span_o0_o2() {
    let mut cases = vec![
        ("int", "int8", "128"),
        ("int", "uint8", "-1"),
        ("uint64", "int64", "18446744073709551615"),
        ("double", "float", "1e100"),
        ("double", "float", "-1e100"),
        ("double", "int64", "1.0/0.0"),
        ("double", "uint64", "-1.0/0.0"),
        ("float", "int64", "1.0/0.0"),
        ("float", "uint64", "-1.0/0.0"),
        ("double", "int64", "9223372036854775808.0"),
        ("double", "uint64", "18446744073709551616.0"),
        ("double", "int8", "128.0"),
        ("double", "int8", "-129.0"),
        ("double", "uint8", "-1.0"),
    ];
    for s in ["float", "double"] {
        for d in [
            "int8", "int16", "int32", "int64", "uint8", "uint16", "uint32", "uint64",
        ] {
            cases.push((s, d, "0.0/0.0"));
        }
    }
    for profile in ["debug", "release"] {
        for (s, d, value) in &cases {
            let source=format!("func get()->{s}{{print(\"한번🙂\");return {value}}}func main(){{let 실패=get() as {d};print(\"after\")}}");
            let cast = format!("get() as {d}");
            let start = source.find(&cast).unwrap();
            let end = start + cast.len();
            let result = program_profile(&source, profile);
            assert!(!result.status.success());
            assert_eq!(result.stdout, "한번🙂\n".as_bytes());
            let stderr = String::from_utf8_lossy(&result.stderr);
            assert_eq!(
                stderr.lines().next().unwrap(),
                format!("Nova panic: numeric cast out of range at file#0:{start}..{end}"),
                "{profile} {s}->{d} {value}: {stderr}"
            );
        }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p10_cast_native_truncation_signed_zero_subnormal_nan_infinity_and_chain_o0_o2() {
    let source = r#"
func d(x:double)->double{return x}
func u(x:uint64)->uint64{return x}
func main(){
let a=d(127.9) as int8;let b=d(-128.9) as int8;let c=d(-0.9) as uint8
let z=d(-0.0) as float;let t=d(-7e-46) as float;let sub=d(1.401298464324817e-45) as float
let inf=d(1.0)/d(0.0);let ni=d(-1.0)/d(0.0);let n=d(0.0)/d(0.0)
print("{a} {b} {c} {z} {t} {sub>0.0} {inf as float} {ni as float} {n as float}")
let i=d(9223372036854774784.0) as int64;let j=d(18446744073709549568.0) as uint64
let k=u(18446744073709551615) as double;let chain=(d(127.9) as int8) as double
let less=d(127.9) as int<128
let skipped=false&&((128 as int8)==0)
print("{i} {j} {k} {chain} {d(-0.0) as int} {less} {skipped}")
}
"#;
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(result.stderr.is_empty());
        assert_eq!(result.stdout,b"127 -128 0 -0 -0 true inf -inf NaN\n9223372036854774784 18446744073709549568 18446744073709552000 127 0 true false\n");
    }
}
