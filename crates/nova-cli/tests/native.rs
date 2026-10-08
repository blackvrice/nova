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

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p12_copy_struct_native_fixture_and_effect_order_o0_o2() {
    for profile in ["debug", "release"] {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/native-struct-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
        std::fs::create_dir_all(&root).unwrap();
        let root = root.canonicalize().unwrap();
        for (name, source) in [
            (
                "main.nova",
                include_str!("../../../docs/development-v0.1/struct-proposal-fixtures/main.nova"),
            ),
            (
                "geometry.nova",
                include_str!(
                    "../../../docs/development-v0.1/struct-proposal-fixtures/geometry.nova"
                ),
            ),
        ] {
            std::fs::write(root.join(name), source).unwrap();
        }
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg("run")
            .arg(root.join("main.nova"))
            .args(["--profile", profile])
            .current_dir(&root)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "original=21, snapshot=20, shifted=21, tag=🙂\n".as_bytes()
        );
        assert!(result.stderr.is_empty());
        let source = r#"struct Empty{} struct P{var x:int;var y:double;let c:char;let b:bool;let u:();let e:Empty}
func first()->int{print("first");return 1} func second()->double{print("second");return 2.5}
func copy(p:P)->P{return p} func empty()->Empty{return Empty()}
func main(){var p=P(first(),second(),'🙂',true,(),empty());let old=copy(p);while p.x<4{p.x=p.x+1};p.y=3.5;print("{old.x} {p.x} {old.y} {p.y} {old.c} {old.b}")}"#;
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "first\nsecond\n1 4 2.5 3.5 🙂 true\n".as_bytes()
        );
        assert!(result.stderr.is_empty());
        let source = r#"struct Widths{let a:int8;let b:uint8;let c:int16;let d:uint16;let e:int;let f:uint;let g:int64;let h:uint64;let i:float;let j:double}
const C=Widths(-128,255,-32768,65535,-2147483648,4294967295,-9223372036854775808,18446744073709551615,0.5,-0.0)
func echo(v:Widths)->Widths{return v} func main(){let v=echo(C);print("{v.a} {v.b} {v.c} {v.d} {v.e} {v.f} {v.g} {v.h} {v.i} {v.j}")}"#;
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout,b"-128 255 -32768 65535 -2147483648 4294967295 -9223372036854775808 18446744073709551615 0.5 -0\n");
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p12_struct_field_checked_failure_has_exact_utf8_source_o0_o2() {
    let source="// 한글 🙂\nstruct P{let x:int8} func main(){let p=P(127);let n=p.x+1;print(\"unreachable\")}";
    let start = source.find("p.x+1").unwrap();
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
        let stderr = String::from_utf8(result.stderr).unwrap();
        assert_eq!(
            stderr.lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#0:{start}..{}",
                    start + 5
                )
                .as_str()
            ),
            "{stderr}"
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p13_copy_tuple_native_fixture_and_effect_order_o0_o2() {
    for profile in ["debug", "release"] {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/native-tuple-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
        std::fs::create_dir_all(&root).unwrap();
        let root = root.canonicalize().unwrap();
        for (name, source) in [
            (
                "main.nova",
                include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/main.nova"),
            ),
            (
                "tuples.nova",
                include_str!("../../../docs/development-v0.1/tuple-proposal-fixtures/tuples.nova"),
            ),
        ] {
            std::fs::write(root.join(name), source).unwrap();
        }
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg("run")
            .arg(root.join("main.nova"))
            .args(["--profile", profile])
            .current_dir(&root)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "original=21, snapshot=20, shifted=21, one=7, tag=🙂\n".as_bytes()
        );
        assert!(result.stderr.is_empty());
        let source = r#"struct Empty{} struct P{var x:int8;let b:bool}
func first()->int8{print("first");return 1} func second()->double{print("second");return 2.5}
func echo(v:((P,double),Empty,(),char))->((P,double),Empty,(),char){return v}
func main(){var t=echo(((P(first(),true),second()),Empty(),(),'🙂'));let old=t;while t.0.0.x<4{t.0.0.x=t.0.0.x+1};t.0.1=3.5;print("{old.0.0.x} {t.0.0.x} {old.0.1} {t.0.1} {old.3} {old.0.0.b}")}"#;
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "first\nsecond\n1 4 2.5 3.5 🙂 true\n".as_bytes()
        );
        assert!(result.stderr.is_empty());
        let source = r#"const C:(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double)=(-128,255,-32768,65535,-2147483648,4294967295,-9223372036854775808,18446744073709551615,0.5,-0.0)
func echo(v:(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double))->(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double){return v}
func main(){let v=echo(C);print("{v.0} {v.1} {v.2} {v.3} {v.4} {v.5} {v.6} {v.7} {v.8} {v.9}")}"#;
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout,b"-128 255 -32768 65535 -2147483648 4294967295 -9223372036854775808 18446744073709551615 0.5 -0\n");
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p13_tuple_element_checked_failure_has_exact_utf8_source_o0_o2() {
    let source = "// 한글 🙂\nfunc main(){let t:(int8,)=(127,);let n=t.0+1;print(\"unreachable\")}";
    let start = source.find("t.0+1").unwrap();
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
        let stderr = String::from_utf8(result.stderr).unwrap();
        assert_eq!(
            stderr.lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#0:{start}..{}",
                    start + 5
                )
                .as_str()
            ),
            "{stderr}"
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p14_copy_enum_native_fixture_and_effect_order_o0_o2() {
    for profile in ["debug", "release"] {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/native-enum-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
        std::fs::create_dir_all(&root).unwrap();
        let root = root.canonicalize().unwrap();
        for (name, source) in [
            (
                "main.nova",
                include_str!("../../../docs/development-v0.1/enum-proposal-fixtures/main.nova"),
            ),
            (
                "events.nova",
                include_str!("../../../docs/development-v0.1/enum-proposal-fixtures/events.nova"),
            ),
        ] {
            std::fs::write(root.join(name), source).unwrap();
        }
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg("run")
            .arg(root.join("main.nova"))
            .args(["--profile", profile])
            .current_dir(&root)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            "make\nsum=30, code=7, flag=true\noriginal=empty\nok\n".as_bytes()
        );
        assert!(result.stderr.is_empty());
        let source = r#"struct Z{} enum E{Empty;Data(int8,int16,double,char,(),Z);}
func first()->int8{print("first");return 1}func second()->int16{print("second");return 2}func echo(v:E)->E{return v}
func main(){var holder=(echo(E::Data(first(),second(),2.5,'🙂',(),Z())),);let snapshot=holder;holder.0=E::Empty;var sum=0;
match snapshot.0{E::Empty=>{print("bad")},E::Data(a,b,d,c,_,_)=>{let old=a;holder.0=E::Data(9,9,9.0,'x',(),Z());sum=a+b;print("{old} {sum} {d} {c}")}}
while sum<5{sum=sum+1;match sum==4{true=>{continue},false=>{match holder.0{E::Empty=>{},E::Data(_,_,_,_,_,_)=>{break}}}}}print("{sum}")}"#;
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, "first\nsecond\n1 3 2.5 🙂\n5\n".as_bytes());
        assert!(result.stderr.is_empty());
        let source = r#"enum Widths{Empty;Data(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double,bool,char);}
const C=Widths::Data(-128,255,-32768,65535,-2147483648,4294967295,-9223372036854775808,18446744073709551615,0.5,-0.0,true,'🙂')
func echo(v:Widths)->Widths{return v}func main(){match echo(C){Widths::Empty=>{},Widths::Data(a,b,c,d,e,f,g,h,i,j,k,l)=>{print("{a} {b} {c} {d} {e} {f} {g} {h} {i} {j} {k} {l}")}}}"#;
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout,"-128 255 -32768 65535 -2147483648 4294967295 -9223372036854775808 18446744073709551615 0.5 -0 true 🙂\n".as_bytes());
        assert!(result.stderr.is_empty());
        // Payload-free enums, Unit-only payload, two nested nominal sums and
        // all-return arms verify default tag alignment and return CFG joins.
        let source = r#"enum Inner{A;B;}enum Outer{No;Yes(Inner,());}func score(v:Outer)->int{match v{Outer::No=>{return 0},Outer::Yes(i,_)=>{match i{Inner::A=>{return 1},Inner::B=>{return 2}}}}}func main(){print("{score(Outer::Yes(Inner::B,()))} {score(Outer::No)}")}"#;
        let result = program_profile(source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, b"2 0\n");
        assert!(result.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p14_enum_payload_checked_failure_has_exact_utf8_source_o0_o2() {
    let source = "// 한글 🙂\nenum E{A(int8);B;}func main(){match E::A(127){E::A(x)=>{let n=x+1;print(\"unreachable\")},E::B=>{}}}";
    let start = source.find("x+1").unwrap();
    for profile in ["debug", "release"] {
        let result = program_profile(source, profile);
        assert!(!result.status.success());
        assert!(result.stdout.is_empty());
        let stderr = String::from_utf8(result.stderr).unwrap();
        assert_eq!(
            stderr.lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#0:{start}..{}",
                    start + 3
                )
                .as_str()
            ),
            "{stderr}"
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p15_copy_option_result_fixture_numeric_payloads_effect_order_and_snapshot_o0_o2() {
    for profile in ["debug", "release"] {
        let main = include_str!(
            "../../../docs/development-v0.1/option-result-proposal-fixtures/main.nova"
        )
        .replace("values::", "lib::");
        let r = module_program(
            &main,
            include_str!(
                "../../../docs/development-v0.1/option-result-proposal-fixtures/values.nova"
            ),
            profile,
        );
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert!(r.stderr.is_empty());
        assert_eq!(r.stdout,b"maybe\nsome=7\noriginal=none\nsuccess=8, flag=true\nerror=-1\nnested=none\nprivate=9\nunit=success\n");
        let source = r#"struct H{var v:Result<int8,bool>}func value()->int8{print("value");return 7}func echo(v:Result<int8,bool>)->Result<int8,bool>{return v}
func score(v:int??)->int{match v{none=>{return 0},Option::Some(inner)=>{match inner{none=>{return 1},Option::Some(x)=>{return x}}}}}
func main(){var p=(H(echo(Result::Success(value()))),);let old=p;p.0.v=Result::Error(true);match old.0.v{Result::Success(x)=>{p.0.v=Result::Success(9);print("{x}")},Result::Error(_)=>{}}var n=0;while n<3{n=n+1;match p.0.v{Result::Success(_)=>{if n==2{continue}if n==3{break}},Result::Error(_)=>{break}}}print("{n} {score(Option::Some(none))} {score(none)} {score(Option::Some(Option::Some(8)))}");let u:Result<(),uint64>=Result::Error(18446744073709551615);match u{Result::Success(_)=>{},Result::Error(e)=>{print("{e}")}}}"#;
        let r = program_profile(source, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, b"value\n7\n3 1 0 8\n18446744073709551615\n");
        assert!(r.stderr.is_empty());
        let source = r#"const C:Option<(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double,bool,char)>=Option::Some((-128,255,-32768,65535,-2147483648,4294967295,-9223372036854775808,18446744073709551615,0.5,-0.0,true,'🙂'))
func echo(v:Option<(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double,bool,char)>)->Option<(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double,bool,char)>{return v}
func main(){match echo(C){none=>{},Option::Some(t)=>{print("{t.0} {t.1} {t.2} {t.3} {t.4} {t.5} {t.6} {t.7} {t.8} {t.9} {t.10} {t.11}")}}}"#;
        let r = program_profile(source, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout,"-128 255 -32768 65535 -2147483648 4294967295 -9223372036854775808 18446744073709551615 0.5 -0 true 🙂\n".as_bytes());
        assert!(r.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p15_option_payload_checked_failure_has_exact_utf8_source_o0_o2() {
    let source="// 한글 🙂\nfunc main(){let x:int8?=Option::Some(127);match x{none=>{},Option::Some(y)=>{let n=y+1;print(\"unreachable\")}}}";
    let start = source.find("y+1").unwrap();
    for profile in ["debug", "release"] {
        let r = program_profile(source, profile);
        assert!(!r.status.success());
        assert!(r.stdout.is_empty());
        let stderr = String::from_utf8(r.stderr).unwrap();
        assert_eq!(
            stderr.lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#0:{start}..{}",
                    start + 3
                )
                .as_str()
            )
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p16_native_try_fixture_scalar_mixed_payloads_effects_and_string_arena_o0_o2() {
    for profile in ["debug", "release"] {
        let main = include_str!("../../../docs/development-v0.1/try-proposal-fixtures/main.nova")
            .replace("effects::", "lib::");
        let r = module_program(
            &main,
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/effects.nova"),
            profile,
        );
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert!(r.stderr.is_empty());
        assert_eq!(r.stdout,b"start\nleaf\nsecond\nafter\nok=8\nstart\nleaf\nerror=-1\nleaf\nnested=7\nleaf\nnested-error=-1\nping\nunit=success\nsnapshot=9\nshort=false\nleaf\nleaf\nloop=7\n");
        let source = r#"const C:Result<(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double,bool,char),()>=Result::Success((-128,255,-32768,65535,-2147483648,4294967295,-9223372036854775808,18446744073709551615,0.5,-0.0,true,'🙂'))
func peel(r:Result<(int8,uint8,int16,uint16,int,uint,int64,uint64,float,double,bool,char),()>)->Result<bool,()>{let t=try r;print("{t.0} {t.1} {t.2} {t.3} {t.4} {t.5} {t.6} {t.7} {t.8} {t.9} {t.10} {t.11}");return Result::Success(t.10)}
func unit(r:Result<(),()>)->Result<(),()>{try r;return Result::Success(())}
func saved()->string{print("saved");return "kept"}
func later()->int16{print("later");return 1}
func combine(a:int16,b:int16)->int16{return a+b}
func fail(s:string,r:Result<int8,char>)->Result<int16,char>{print("before={s}");let n:int16=try r;print("after={s}");return Result::Success(n)}
func relay(s:string)->Result<int16,char>{let n=combine(try fail("{s}",Result::Error('🙂')),later());return Result::Success(n)}
func main(){match peel(C){Result::Success(b)=>{print("ok={b}")},Result::Error(_)=>{}}match unit(Result::Error(())){Result::Success(_)=>{},Result::Error(_)=>{print("unit=error")}}let s=saved();match relay(s){Result::Success(_)=>{},Result::Error(c)=>{print("error={c}, saved={s}")}}}"#;
        let r = program_profile(source, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert!(r.stderr.is_empty());
        assert_eq!(r.stdout,"-128 255 -32768 65535 -2147483648 4294967295 -9223372036854775808 18446744073709551615 0.5 -0 true 🙂\nok=true\nunit=error\nsaved\nbefore=kept\nerror=🙂, saved=kept\n".as_bytes());
        let main = r#"use lib::get;func f(flag:bool)->Result<int8,bool>{let n=try get(flag);return Result::Success(n)}func main(){match f(false){Result::Success(_)=>{},Result::Error(b)=>{print("{b}")}}match f(true){Result::Success(n)=>{print("{n}")},Result::Error(_)=>{}}}"#;
        let lib = r#"private struct Hidden{let n:int8}public func get(flag:bool)->Result<int8,bool>{let h=Hidden(9);if flag{return Result::Success(h.n)}else{return Result::Error(true)}}"#;
        let r = module_program(main, lib, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, b"true\n9\n");
        assert!(r.stderr.is_empty());
        let main = r#"use lib::get;use lib::describe;func main(){match get(false){Result::Success(_)=>{},Result::Error(e)=>{print(describe(e))}}match get(true){Result::Success(n)=>{print("{n}")},Result::Error(_)=>{}}}"#;
        let lib = r#"private struct Hidden{let n:int8;let tag:char}private func inner(flag:bool)->Result<bool,Hidden>{if flag{return Result::Success(true)}else{return Result::Error(Hidden(-9,'🙂'))}}public func get(flag:bool)->Result<int8,Hidden>{let b=try inner(flag);return Result::Success(9)}public func describe(h:Hidden)->string{return "hidden={h.n} {h.tag}"}"#;
        let r = module_program(main, lib, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, "hidden=-9 🙂\n9\n".as_bytes());
        assert!(r.stderr.is_empty());
        let source = r#"func cond(flag:bool)->Result<bool,bool>{print("cond");if flag{return Result::Success(true)}else{return Result::Error(false)}}func tryLoop(flag:bool)->Result<int,bool>{while try cond(flag){print("body");break}print("done");return Result::Success(7)}func nested(r:Result<Result<int,bool>,bool>)->Result<int,bool>{return try r}func main(){match tryLoop(false){Result::Success(_)=>{},Result::Error(b)=>{print("error={b}")}}match tryLoop(true){Result::Success(n)=>{print("loop={n}")},Result::Error(_)=>{}}let r:Result<Result<int,bool>,bool>=Result::Success(Result::Error(true));match nested(r){Result::Success(_)=>{},Result::Error(b)=>{print("nested={b}")}}}"#;
        let r = program_profile(source, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(
            r.stdout,
            b"cond\nerror=false\ncond\nbody\ndone\nloop=7\nnested=true\n"
        );
        assert!(r.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p16_try_success_checked_failure_keeps_exact_utf8_span_o0_o2() {
    let source="// 한글 🙂\nfunc f(r:Result<int8,bool>)->Result<int8,bool>{let x=try r;return Result::Success(x+1)}func main(){let r:Result<int8,bool>=Result::Success(127);let n=f(r)}";
    let start = source.find("x+1").unwrap();
    for profile in ["debug", "release"] {
        let r = program_profile(source, profile);
        assert!(!r.status.success());
        assert!(r.stdout.is_empty());
        let stderr = String::from_utf8(r.stderr).unwrap();
        assert_eq!(
            stderr.lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#0:{start}..{}",
                    start + 3
                )
                .as_str()
            )
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p17_native_named_fixture_context_effects_try_and_private_abi_o0_o2() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/named-arguments-proposal-fixtures")
        .canonicalize()
        .unwrap();
    let expected=b"right\nleft\nreverse=702\npositional\nnamed\nmixed=102\nsecond\nfirst\ntext=first/second\nunit=3\nunicode=304\nminimum=-12500\ncopy=4/5\noption=7\nleaf\nlater\nafter\nsuccess=102\nleaf\nerror=-1\nbefore\nleaf\nprior-error=-1\nshort=false\n";
    for profile in ["debug", "release"] {
        let result = Command::new(env!("CARGO_BIN_EXE_nova"))
            .arg("run")
            .arg(fixtures.join("main.nova"))
            .arg("--source-root")
            .arg(&fixtures)
            .args(["--profile", profile])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, expected);
        assert!(result.stderr.is_empty());
        for name in [
            "function_print_shadow.nova",
            "forward_recursive_grouped.nova",
        ] {
            let r = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg("run")
                .arg(fixtures.join(name))
                .arg("--source-root")
                .arg(&fixtures)
                .args(["--profile", profile])
                .output()
                .unwrap();
            assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
            assert!(r.stdout.is_empty() && r.stderr.is_empty());
        }
        let source = r#"struct S{let n:int8;let c:char}func takeValue(s:S,t:(int8,char),u:(),r:Result<int8,bool>)->int8{print("{s.n} {s.c} {t.0} {t.1}");match r{Result::Success(n)=>{return n},Result::Error(_)=>{return 0}}}func make()->S{print("struct");return S(3,'🙂')}func numbers(a:float,b:double,c:uint64,d:int16){print("{a} {b} {c} {d}")}func main(){let n=takeValue(r:Result::Success(7),u:(),t:(2,'가'),s:make());print("value={n}");numbers(d:300,c:18446744073709551615,b:1e100,a:0.5)}"#;
        let r = program_profile(source, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert!(r.stderr.is_empty());
        assert_eq!(
            String::from_utf8(r.stdout).unwrap(),
            format!(
                "struct\n3 🙂 2 가\nvalue=7\n0.5 1{} 18446744073709551615 300\n",
                "0".repeat(100)
            )
        );
        let main = r#"use lib::compute as renamed;func main(){let n=renamed(right:2,left:1);print("{n}")}"#;
        let lib="private func hidden(a:int8,b:int8)->int8{return a+a+b}public func compute(left:int8,right:int8)->int8{return hidden(b:right,a:left)}";
        let r = module_program(main, lib, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, b"4\n");
        assert!(r.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p17_native_named_argument_overflow_preserves_effect_order_and_span_o0_o2() {
    let source="// 한글 🙂\nfunc first()->int8{print(\"first\");return 1}func second()->int8{print(\"second\");return 127}func joined(left:int8,right:int8){print(\"callee\")}func main(){joined(right:first(),left:second()+1)}";
    let start = source.find("second()+1").unwrap();
    for profile in ["debug", "release"] {
        let r = program_profile(source, profile);
        assert!(!r.status.success());
        assert_eq!(r.stdout, b"first\nsecond\n");
        assert_eq!(
            String::from_utf8(r.stderr).unwrap().lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#0:{start}..{}",
                    start + 10
                )
                .as_str()
            )
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p18_native_default_fixture_scope_contexts_effects_try_and_private_abi_o0_o2() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/default-arguments-proposal-fixtures")
        .canonicalize()
        .unwrap();
    let expected="defaults=620\nright\nnamed=602\nleft\npositional=120\nsecond\nfirst\nreverse=702\nholes=456\ntext=default/provided\nunit=3\ncopy=4/🙂\noption=none\nresult=7\nleaf\nafter\nsuccess=602\nleaf\nerror=-1\nshort=false\n";
    for profile in ["debug", "release"] {
        for (name, expected) in [
            ("main.nova", expected),
            ("unicode_print_shadow.nova", ""),
            ("forward_recursive_grouped.nova", ""),
        ] {
            let r = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg("run")
                .arg(fixtures.join(name))
                .arg("--source-root")
                .arg(&fixtures)
                .args(["--profile", profile])
                .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
                .output()
                .unwrap();
            assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
            assert_eq!(r.stdout, expected.as_bytes());
            assert!(r.stderr.is_empty());
        }
        let source = r#"struct Box{let n:int8}enum Choice{One(Box)}
func defaults(a:int8=-128,b:uint8=255,c:int16=-32768,d:uint16=65535,e:int32=-2147483648,f:uint32=4294967295,g:int64=-9223372036854775808,h:uint64=18446744073709551615,i:float=0.5,j:double=-0.0,k:char='🙂',flag:bool=true,u:()=(),text:string="x\0🙂",s:Box=Box(7),choice:Choice=Choice::One(Box(8)),t:(int8,bool)=(9,false),r:Result<int8,()>=Result::Success(10)){
print("{a}/{b}/{c}/{d}/{e}/{f}/{g}/{h}/{i}/{j}/{k}/{flag}");print("{text}/{s.n}/{t.0}/{t.1}");match choice{Choice::One(v)=>{print("choice={v.n}")}}match r{Result::Success(n)=>{print("result={n}")},Result::Error(_)=>{}}}
func main(){defaults();defaults(a:1,text:"override")}"#;
        let r = program_profile(source, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert!(r.stderr.is_empty());
        let expected="-128/255/-32768/65535/-2147483648/4294967295/-9223372036854775808/18446744073709551615/0.5/-0/🙂/true\nx\0🙂/7/9/false\nchoice=8\nresult=10\n1/255/-32768/65535/-2147483648/4294967295/-9223372036854775808/18446744073709551615/0.5/-0/🙂/true\noverride/7/9/false\nchoice=8\nresult=10\n";
        assert_eq!(r.stdout, expected.as_bytes());
        let main="use lib::show;use lib::saved;use lib::early;func main(){let text=saved();match early(){Result::Success(_)=>{},Result::Error(_)=>{}}show();print(text)}";
        let lib="private struct H{private let value:int8}public func show(value:H=H(7)){print(\"private={value.value}\")}public func saved(value:string=\"kept\")->string{return value}func leaf()->Result<int,bool>{return Result::Error(true)}func sum(a:int=99,b:int)->int{return a+b}public func early()->Result<int,bool>{let x=sum(b:try leaf());return Result::Success(x)}";
        let r = module_program(main, lib, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, b"private=7\nkept\n");
        assert!(r.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p18_default_callee_overflow_has_exact_cross_file_span_and_effects_o0_o2() {
    let main="use lib::overflow;func effect(){print(\"provided\")}func main(){overflow(marker:effect());print(\"after\")}";
    let lib="// 한글 🙂\npublic func overflow(value:int8=127,marker:()=()){let x=value+1;print(\"callee-after\")}";
    let start = lib.find("value+1").unwrap();
    for profile in ["debug", "release"] {
        let r = module_program(main, lib, profile);
        assert!(!r.status.success());
        assert_eq!(r.stdout, b"provided\n");
        assert_eq!(
            String::from_utf8(r.stderr).unwrap().lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#1:{start}..{}",
                    start + 7
                )
                .as_str()
            )
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p19_native_range_loop_fixture_edges_effects_snapshots_jumps_try_o0_o2() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/range-loop-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for profile in ["debug", "release"] {
        for (name,expected) in [("main.nova","range=8\nstart\nend\nbounds=23\nempty-start\nempty-end\nempty=0\nmaximum=2/255\nmixed=0\nshadow=99/3\nnested=4\nloop=3\ntry-start\ntry-end\ntry-ok=3\ntry-start\ntry-end\ntry-error=-1\n"),("unicode_scope_and_snapshots.nova",""),("integer_edges_and_mixed_jumps.nova","")] {
            let r=Command::new(env!("CARGO_BIN_EXE_nova")).arg("run").arg(root.join(name)).args(["--profile",profile]).current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")).output().unwrap();
            assert!(r.status.success(),"{name}: {}",String::from_utf8_lossy(&r.stderr));assert_eq!(r.stdout,expected.as_bytes());assert!(r.stderr.is_empty());
        }
        let edges = [
            ("int8", "-128", "127"),
            ("uint8", "0", "255"),
            ("int16", "-32768", "32767"),
            ("uint16", "0", "65535"),
            ("int32", "-2147483648", "2147483647"),
            ("uint32", "0", "4294967295"),
            ("int64", "-9223372036854775808", "9223372036854775807"),
            ("uint64", "0", "18446744073709551615"),
        ];
        // Distinct helper functions keep each type's declarations in their own scope.
        let mut source = String::new();
        let mut calls = String::new();
        for (n, lo, hi) in edges {
            source.push_str(&format!("func test_{n}(){{const LOW:{n}={lo};const HIGH:{n}={hi};var count=0;for i in LOW until LOW+1{{count=count+1}}for i in HIGH through HIGH{{count=count+1;continue}}for i in HIGH through HIGH{{count=count+1}}for i in HIGH until HIGH{{count=100}}for i in HIGH through LOW{{count=100}}print(\"{n}={{count}}\")}}"));
            calls.push_str(&format!("test_{n}();"));
        }
        source.push_str(&format!("func main(){{{calls}}}"));
        let result = program_profile(&source, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            result.stdout,
            b"int8=3\nuint8=3\nint16=3\nuint16=3\nint32=3\nuint32=3\nint64=3\nuint64=3\n"
        );
        assert!(result.stderr.is_empty());
        let early="func edge(ok:bool,tag:string)->Result<int,bool>{print(tag);if ok{return Result::Success(1)}return Result::Error(true)}func f()->Result<int,bool>{for i in try edge(false,\"start\") until try edge(true,\"end\"){print(\"body\")}return Result::Success(0)}func main(){match f(){Result::Success(_)=>{},Result::Error(_)=>{print(\"error\")}}}";
        let result = program_profile(early, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, b"start\nerror\n");
        assert!(result.stderr.is_empty());
        let returns="func f()->int{for i in 1 through 3{loop{if i==2{return i}break}}return 9}func main(){print(\"{f()}\")}";
        let result = program_profile(returns, profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, b"2\n");
        let result=program_profile("struct S{let n:int;}func main(){var text=\"first{0}\";let saved=text;var unit:()=();for i in 0 through 1 {let s=S(i);let t=(s,i);text=\"next{t.0.n}\";unit=()}loop{unit=();break}print(\"{saved}/{text}\")}",profile);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(result.stdout, b"first0/next1\n");
        assert!(result.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p19_range_loop_checked_failure_exact_cross_file_utf8_spans_and_effects_o0_o2() {
    for profile in ["debug", "release"] {
        for (body,expression,stdout) in [
        ("let 値:int8=127;for i in begin() until 値+1{print(\"body\")}","値+1","start\n"),
        ("let 値:int8=127;for i in 値+1 until 値{print(\"body\")}","値+1",""),
        ("let 値:int8=127;for i in 0 through 1{print(\"before\");let bad=値+1;print(\"after\")}","値+1","before\n"),
    ] {
        let lib=format!("// 한글 🙂\nfunc begin()->int8{{print(\"start\");return 0}}public func fail(){{{body}}}");let start=lib.find(expression).unwrap();
        let result=module_program("use lib::fail;func main(){fail();print(\"after-main\")}",&lib,profile);
        assert!(!result.status.success());assert_eq!(result.stdout,stdout.as_bytes());
        assert_eq!(String::from_utf8(result.stderr).unwrap().lines().next(),Some(format!("Nova panic: integer overflow at file#1:{start}..{}",start+expression.len()).as_str()));
    }
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p20_native_exists_fixture_truthiness_effects_snapshots_contexts_and_try_o0_o2() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/exists-proposal-fixtures")
        .canonicalize()
        .unwrap();
    let expected="basic=true/false\npayload=true/true\nnested=true/true\nconst=true/true\ndefault=true\nonce\nonce=true\nshort=false/true\nsnapshot=true/false\ntuple=true\nnegate=true\nsecond\nfirst\norder=false\ntry\ntry-ok=true\ntry\ntry-error=-1\n";
    for profile in ["debug", "release"] {
        for (name, output) in [
            ("main.nova", expected),
            ("newline_and_shadow.nova", ""),
            ("skipped_abort.nova", ""),
        ] {
            let r = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg("run")
                .arg(fixtures.join(name))
                .arg("--source-root")
                .arg(&fixtures)
                .args(["--profile", profile])
                .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
                .output()
                .unwrap();
            assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
            assert_eq!(r.stdout, output.as_bytes());
            assert!(r.stderr.is_empty());
        }
        let mut widths = String::new();
        let mut calls = String::new();
        let mut expected_widths = String::new();
        for (name, min, max) in [
            ("int8", "-128", "127"),
            ("uint8", "0", "255"),
            ("int16", "-32768", "32767"),
            ("uint16", "0", "65535"),
            ("int32", "-2147483648", "2147483647"),
            ("uint32", "0", "4294967295"),
            ("int64", "-9223372036854775808", "9223372036854775807"),
            ("uint64", "0", "18446744073709551615"),
        ] {
            widths.push_str(&format!(
                "func present_{name}(value:{name}?)->bool{{return value exists}}"
            ));
            calls.push_str(&format!("print(\"{name}={{present_{name}(Option::Some({min}))}}/{{present_{name}(Option::Some({max}))}}/{{present_{name}(none)}}\");"));
            expected_widths.push_str(&format!("{name}=true/true/false\n"));
        }
        widths.push_str(&format!("func main(){{{calls}}}"));
        let r = program_profile(&widths, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, expected_widths.as_bytes());
        assert!(r.stderr.is_empty());
        let source = r#"struct S{let n:int}enum E{V(S)}
func main(){let zero:double?=Option::Some(-0.0);let character:char?=Option::Some('\0');let result:Result<int,()>?=Option::Some(Result::Error(()));print("values={zero exists}/{character exists}/{result exists}");var value:int?=Option::Some(0);var count=0;while value exists{count=count+1;value=none}loop{if !value exists{break}}for i in 0 until 2{let e:E?=Option::Some(E::V(S(i)));if e exists{count=count+1}}print("loops={count}");match value exists{true=>{print("bad")},false=>{print("absent")}}}"#;
        let r = program_profile(source, profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, b"values=true/true/true\nloops=3\nabsent\n");
        assert!(r.stderr.is_empty());
        let r=module_program("use lib::fail;func main(){match fail(){Result::Success(_)=>{},Result::Error(_)=>{print(\"error\")}}}",
            "func leaf()->Result<int?,bool>{print(\"leaf\");return Result::Error(true)}public func fail()->Result<bool,bool>{let b=(try leaf()) exists;print(\"after\");return Result::Success(b)}",profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, b"leaf\nerror\n");
        assert!(r.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p20_exists_operand_abort_cross_file_utf8_span_and_effects_o0_o2() {
    for profile in ["debug", "release"] {
        let lib="// 한글 🙂\nfunc danger()->int?{let 値:int8=127;return Option::Some((値+1) as int)}public func fail(){print(\"before\");let present=danger() exists;print(\"after\")}";
        let start = lib.find("値+1").unwrap();
        let r = module_program(
            "use lib::fail;func main(){fail();print(\"after-main\")}",
            lib,
            profile,
        );
        assert!(!r.status.success());
        assert_eq!(r.stdout, b"before\n");
        assert_eq!(
            String::from_utf8(r.stderr).unwrap().lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#1:{start}..{}",
                    start + "値+1".len()
                )
                .as_str()
            )
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p21_native_alias_fixture_opaque_scope_canonical_abi_o0_o2() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/alias-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for profile in ["debug", "release"] {
        for (name, output) in [
            (
                "main.nova",
                "small=7\npair=7/2\nexists=true\nflag=on\ncast=7\ntext=한글\ntry=true\nerror=-1\n",
            ),
            ("multiline.nova", ""),
        ] {
            let r = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg("run")
                .arg(fixtures.join(name))
                .arg("--source-root")
                .arg(&fixtures)
                .args(["--profile", profile])
                .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
                .output()
                .unwrap();
            assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
            assert_eq!(r.stdout, output.as_bytes());
            assert!(r.stderr.is_empty());
        }
        let r=module_program("use lib::A;use lib::factory;use lib::read;type T=bool;func echo(x:A)->A{return x}func main(){print(\"opaque={read(echo(factory()))}\")}", "type T=int8;private struct Hidden{private let n:T}public type A=Hidden;public func factory()->A{return Hidden(7)}public func read(x:A)->T{return x.n}", profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, b"opaque=7\n");
        assert!(r.stderr.is_empty());
        let r=program_profile("type N=uint8;type C=char;type F=double;type U=();type T=(N,C,F,U);type O=Option<T>;const VALUE:T=(255,'🙂',-0.0,());func f(x:T=VALUE)->O{return Option::Some(x)}func main(){match f(){Option::Some(t)=>{print(\"{t.0}/{t.1}/{t.2}\")},Option::None=>{}}}",profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, "255/🙂/-0\n".as_bytes());
        assert!(r.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p21_alias_checked_abort_cross_file_utf8_span_o0_o2() {
    for profile in ["debug", "release"] {
        let lib="// 한글 🙂\ntype Small=int8;public func fail(){print(\"before\");let 値:Small=127;let bad=値+1;print(\"after\")}";
        let start = lib.find("値+1").unwrap();
        let r = module_program(
            "use lib::fail;func main(){fail();print(\"after-main\")}",
            lib,
            profile,
        );
        assert!(!r.status.success());
        assert_eq!(r.stdout, b"before\n");
        assert_eq!(
            String::from_utf8(r.stderr).unwrap().lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#1:{start}..{}",
                    start + "値+1".len()
                )
                .as_str()
            )
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p22_native_methods_receiver_default_try_opaque_and_abi_o0_o2() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/method-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for profile in ["debug", "release"] {
        for (name,output) in [("main.nova","sum=8\nalias=7\ncopy=3/5\ntext=3/4\nprivate=3\nmember-main=4\ntuple=7\nreceiver\narg\norder=9\nfetch\nafter\nok=11\nfetch\nerror=-1\n"),("newline_and_self.nova","")] {
            let r=Command::new(env!("CARGO_BIN_EXE_nova")).arg("run").arg(fixtures.join(name)).arg("--source-root").arg(&fixtures).args(["--profile",profile]).current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")).output().unwrap();
            assert!(r.status.success(),"{}",String::from_utf8_lossy(&r.stderr));assert_eq!(r.stdout,output.as_bytes());assert!(r.stderr.is_empty());
        }
        let r=module_program("use lib::factory;func main(){let p=factory();print(p.text())}","private struct Hidden{private let x:int;public func text(self)->string{return self.secret()};private func secret(self)->string{return \"{self.x}\"}}public func factory()->Hidden{return Hidden(7)}",profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, b"7\n");
        assert!(r.stderr.is_empty());
        let r=program_profile("struct P{var x:int;func rec(self,n:int)->int{if n==0{return self.x}else{return self.rec(n-1)}}func copied(self)->P{var p=self;p.x=9;return p}func payload(self,f:double,c:char)->(double,char,Option<int>){return (f,c,Option::Some(self.x))}}func main(){let p=P(3);let q=p.copied();let t=p.payload(1.5,'🙂');print(\"{p.rec(4)}/{q.x}/{p.x}/{t.0}/{t.1}/{t.2 exists}\")}",profile);
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, "3/9/3/1.5/🙂/true\n".as_bytes());
        assert!(r.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p22_method_abort_skips_arguments_and_cross_file_utf8_source_o0_o2() {
    for profile in ["debug", "release"] {
        let lib="// 한글 🙂\npublic struct P{public let x:int8;public func fail(self,n:int8=0)->int8{return self.x+1}}public func make()->P{print(\"before\");let p=P(127);let x=p.fail();print(\"after\");return p}";
        let start = lib.find("self.x+1").unwrap();
        let r=module_program("use lib::make;func arg()->int8{print(\"arg\");return 1}func main(){let v=make().fail(arg());print(\"after-main\")}",lib,profile);
        assert!(!r.status.success());
        assert_eq!(r.stdout, b"before\n");
        assert_eq!(
            String::from_utf8(r.stderr).unwrap().lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#1:{start}..{}",
                    start + "self.x+1".len()
                )
                .as_str()
            )
        );
        let source = include_str!(
            "../../../docs/development-v0.1/method-proposal-fixtures/method_abort.nova"
        );
        let r = program_profile(source, profile);
        assert!(!r.status.success());
        assert_eq!(r.stdout, b"before\n");
        let start = source.find("self.x+1").unwrap();
        assert_eq!(
            String::from_utf8(r.stderr).unwrap().lines().next(),
            Some(
                format!(
                    "Nova panic: integer overflow at file#0:{start}..{}",
                    start + "self.x+1".len()
                )
                .as_str()
            )
        );
    }
}

#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p23_native_named_constructors_const_defaults_try_and_aggregate_o0_o2() {
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/struct-named-arguments-proposal-fixtures")
        .canonicalize()
        .unwrap();
    let expected="y\nx\norder=1/2\nprefix\nnamed\nmixed=3/4\nmapped=7/300\ndefault=5/6\nconst=3/20\nnested=3/true/한/true\nself=7\nprivate=8\nempty=0\ncopy=1/6\nfetch\nlater\nconstructed\nok=1\nfetch\nerror=-1\n";
    for profile in ["debug", "release"] {
        for (name, out) in [
            ("main.nova", expected),
            ("multiline.nova", ""),
            ("value_namespace.nova", ""),
        ] {
            let r = Command::new(env!("CARGO_BIN_EXE_nova"))
                .arg("run")
                .arg(fixtures.join(name))
                .arg("--source-root")
                .arg(&fixtures)
                .args(["--profile", profile])
                .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
                .output()
                .unwrap();
            assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
            assert_eq!(r.stdout, out.as_bytes());
            assert!(r.stderr.is_empty());
        }
        let r = program_profile(
            r#"struct P{let unit:();let flag:bool;let value:float32}struct Q{let p:P;let pair:(char,double)}func use_unit(v:()){}func main(){let p=P(value:1.5,flag:true,unit:());let q=Q(pair:('🙂',2.5),p:p);use_unit(q.p.unit);print("{q.p.value}/{q.p.flag}/{q.pair.0}/{q.pair.1}")}"#,
            profile,
        );
        assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
        assert_eq!(r.stdout, "1.5/true/🙂/2.5\n".as_bytes());
        assert!(r.stderr.is_empty());
    }
}
#[test]
#[ignore = "requires LLVM 21.1.8 and Windows x64 MSVC"]
fn p23_constructor_abort_uses_first_source_value_and_skips_later_o0_o2() {
    let source=include_str!("../../../docs/development-v0.1/struct-named-arguments-proposal-fixtures/constructor_abort.nova");
    let at = source.find("n+1").unwrap();
    for profile in ["debug", "release"] {
        let r = program_profile(source, profile);
        assert_eq!(r.status.code(), Some(1));
        assert_eq!(r.stdout, b"before\n");
        assert_eq!(
            String::from_utf8(r.stderr).unwrap().lines().next(),
            Some(format!("Nova panic: integer overflow at file#0:{at}..{}", at + 3).as_str())
        );
    }
}
