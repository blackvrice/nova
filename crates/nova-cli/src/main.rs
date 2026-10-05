use nova_codegen::{
    CodegenBackend, CodegenError, CodegenOptions, CodegenUnit, OptimizationLevel, TargetSpec,
};
use nova_codegen_llvm::{emit_ir, toolchain::ClangTool, LlvmBackend};
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, RenderFormat, Severity};
use nova_source::SourceDatabase;
use std::ffi::{OsStr, OsString};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU64, Ordering};

static DIRECTORY_SEQUENCE: AtomicU64 = AtomicU64::new(0);
const RUNTIME: &str = include_str!("../runtime/stage_a.rs");
fn main() -> ExitCode {
    match execute(std::env::args_os().skip(1).collect()) {
        Ok(code) => ExitCode::from(code),
        Err((code, message)) => {
            eprintln!("{message}");
            ExitCode::from(code)
        }
    }
}
type Failure = (u8, String);
fn execute(args: Vec<OsString>) -> Result<u8, Failure> {
    if args.len() == 1 && args[0] == "version" {
        println!("Nova 0.1 Stage A / LLVM 21.1.8 adapter");
        return Ok(0);
    }
    if args.len() == 1 && (args[0] == "--help" || args[0] == "help") {
        println!("nova check <file.nova> [--source-root <directory>]\nnova build <file.nova> [-o <new-path>] [--profile debug|release] [--source-root <directory>]\nnova run <file.nova> [--profile debug|release] [--source-root <directory>]\nNOVA_CLANG / NOVA_RUSTC select native tools.");
        return Ok(0);
    }
    if args.len() < 2 {
        return Err((
            2,
            "usage: nova check|build|run <file.nova> [-o <new-path>]".into(),
        ));
    }
    let command = args[0].to_str().ok_or((2, "invalid command".into()))?;
    if !matches!(command, "check" | "build" | "run") {
        return Err((2, format!("unknown command: {command}")));
    }
    let source = PathBuf::from(&args[1]);
    if source.extension() != Some(OsStr::new("nova")) {
        return Err((2, "input must be a .nova file".into()));
    }
    let mut output = None;
    let mut source_root = None;
    let mut optimization = OptimizationLevel::None;
    let mut profile_seen = false;
    let mut remaining = args[2..].iter();
    while let Some(flag) = remaining.next() {
        let value = remaining
            .next()
            .ok_or((2, "option requires a value".into()))?;
        if flag == "-o" && command == "build" && output.is_none() {
            output = Some(PathBuf::from(value));
        } else if flag == "--source-root" && source_root.is_none() {
            source_root = Some(PathBuf::from(value));
        } else if flag == "--profile" && command != "check" && !profile_seen {
            optimization = if value == "debug" {
                OptimizationLevel::None
            } else if value == "release" {
                OptimizationLevel::Default
            } else {
                return Err((2, "Stage A profiles: debug, release".into()));
            };
            profile_seen = true;
        } else {
            return Err((2, "unsupported or repeated option".into()));
        }
    }
    let unit = frontend(&source, command != "check", source_root.as_deref())?;
    if command == "check" {
        return Ok(0);
    }
    let target = if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        TargetSpec::WindowsX64Msvc
    } else {
        return Err((
            3,
            "Native run/link is currently verified only for Windows x64 MSVC".into(),
        ));
    };
    // Entry validation happens before creating any output or invoking tools.
    let ir = emit_ir(&unit, target, true).map_err(backend_error)?;
    let directory = work_directory()?;
    let object = directory.join("module.obj");
    let backend = LlvmBackend {
        clang: ClangTool::new(
            std::env::var_os("NOVA_CLANG").unwrap_or_else(|| OsString::from("clang")),
        ),
    };
    backend
        .codegen_unit(
            &unit,
            &target,
            &CodegenOptions {
                object_path: object.clone(),
                optimization,
                executable: true,
            },
        )
        .map_err(backend_error)?;
    fs::write(directory.join("module.ll"), ir.text).map_err(|e| (3, e.to_string()))?;
    let runtime = directory.join("runtime.rs");
    fs::write(&runtime, RUNTIME).map_err(|e| (3, e.to_string()))?;
    let executable = directory.join("nova-program.exe");
    let rustc = std::env::var_os("NOVA_RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let object = fs::canonicalize(&object).map_err(|e| (3, e.to_string()))?;
    let status = Command::new(rustc)
        .arg(&runtime)
        .args([
            "--edition=2021",
            "--crate-name=nova_stage_a_runtime",
            "--target",
            target.triple(),
            "-Cpanic=abort",
            "-Copt-level=0",
            "-C",
        ])
        .arg(format!("link-arg={}", object.display()))
        .args(["-C", "link-arg=/Brepro"])
        .arg(format!(
            "--remap-path-prefix={}=/nova-stage-a",
            directory.display()
        ))
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|e| (3, format!("could not invoke Rust runtime linker: {e}")))?;
    if !status.status.success() {
        return Err((
            3,
            format!(
                "Runtime compile/link failed: {}",
                String::from_utf8_lossy(&status.stderr)
            ),
        ));
    }
    if command == "build" {
        let final_path = if let Some(output) = output {
            publish(&executable, &output)?;
            output
        } else {
            executable
        };
        println!("{}", final_path.display());
        return Ok(0);
    }
    let status = Command::new(&executable)
        .status()
        .map_err(|e| (3, format!("could not execute Native program: {e}")))?;
    // Portable process API does not expose a Windows abort code as u8. Report
    // it and return nonzero; preserve ordinary 0..255 child exit codes exactly.
    match status.code() {
        Some(code) if (0..=255).contains(&code) => Ok(code as u8),
        code => {
            eprintln!("Native process terminated: {code:?}");
            Ok(1)
        }
    }
}
fn backend_error(error: CodegenError) -> Failure {
    let code = match error {
        CodegenError::InvalidMir(_) | CodegenError::VerificationFailed { .. } => 101,
        CodegenError::InvalidEntry { .. } => 1,
        _ => 3,
    };
    (code, error.to_string())
}
fn frontend(
    path: &Path,
    executable: bool,
    source_root: Option<&Path>,
) -> Result<CodegenUnit, Failure> {
    let loaded = nova_driver::load(path, source_root)
        .map_err(|e| (if e.code == 101 { 101 } else { 1 }, e.to_string()))?;
    let sources = loaded.sources;
    let Some(hir) = loaded.module else {
        return Err((1, render(&sources, &loaded.diagnostics)?));
    };
    let resolved = nova_resolve::resolve(&hir);
    let checked = nova_typecheck::check(&hir, &resolved).map_err(|e| {
        let code = if e == nova_typecheck::CheckError::UnsupportedFloatHost {
            3
        } else {
            101
        };
        (code, e.to_string())
    })?;
    if checked.has_errors() {
        return Err((1, render(&sources, &checked.diagnostics)?));
    }
    let mir =
        nova_mir::lower(&hir, &resolved, &checked, false).map_err(|e| (101, e.to_string()))?;
    let unit = CodegenUnit::new(mir).map_err(backend_error)?;
    if executable {
        if let Err(error) = unit.executable_entry() {
            if let CodegenError::InvalidEntry {
                code,
                message,
                source: Some(source),
            } = error
            {
                let diagnostic = Diagnostic {
                    code: DiagnosticCode::new(code).expect("stable entry diagnostic code"),
                    severity: Severity::Error,
                    message: message.clone(),
                    primary: Label {
                        span: source.span,
                        message,
                    },
                    secondary: vec![],
                    notes: vec![],
                    suggestions: vec![],
                };
                return Err((1, render(&sources, &[diagnostic])?));
            }
            return Err(backend_error(error));
        }
    }
    Ok(unit)
}
fn render(sources: &SourceDatabase, diagnostics: &[Diagnostic]) -> Result<String, Failure> {
    let mut result = String::new();
    for diagnostic in diagnostics {
        result.push_str(
            &nova_diagnostics::render(diagnostic, sources, RenderFormat::Plain)
                .map_err(|e| (101, e.to_string()))?,
        );
        result.push('\n');
    }
    Ok(result)
}
fn work_directory() -> Result<PathBuf, Failure> {
    let root = PathBuf::from("target/nova-stage-a");
    fs::create_dir_all(&root).map_err(|e| (3, e.to_string()))?;
    for _ in 0..100 {
        let id = DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| (3, e.to_string()))?
            .as_nanos();
        let path = root.join(format!("{}-{stamp}-{id}", std::process::id()));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err((3, e.to_string())),
        }
    }
    Err((3, "could not create unique build directory".into()))
}
fn publish(source: &Path, destination: &Path) -> Result<(), Failure> {
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)
        .map_err(|e| (3, format!("{}: {e}", destination.display())))?;
    let result = fs::read(source)
        .and_then(|bytes| output.write_all(&bytes))
        .and_then(|_| output.flush());
    drop(output);
    if let Err(e) = result {
        let _ = fs::remove_file(destination);
        return Err((3, e.to_string()));
    }
    Ok(())
}
