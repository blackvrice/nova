//! P03 Stage A LLVM adapter. LLVM IR and tool invocation stay behind NOVA-093.
mod emit;
pub mod toolchain;
pub use emit::{emit_ir, IrArtifact};
use nova_codegen::{
    CodegenBackend, CodegenError, CodegenOptions, CodegenUnit, ObjectArtifact, OptimizationLevel,
    TargetSpec,
};
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use toolchain::ClangTool;

pub const LLVM_VERSION: (u32, u32, u32) = (21, 1, 8);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LlvmBackend {
    pub clang: ClangTool,
}
impl CodegenBackend for LlvmBackend {
    fn codegen_unit(
        &self,
        unit: &CodegenUnit,
        target: &TargetSpec,
        options: &CodegenOptions,
    ) -> Result<ObjectArtifact, CodegenError> {
        let ir = emit_ir(unit, *target, options.executable)?;
        let version = self.clang.probe()?;
        if (version.major, version.minor, version.patch) != LLVM_VERSION {
            return Err(CodegenError::InvalidToolVersion {
                tool: self.clang.path().into(),
                output: version.output,
            });
        }
        // Parse and verify before object creation. The second invocation only
        // emits the already verified module; object tool failures are toolchain
        // errors rather than being mislabeled as compiler verification bugs.
        let verify_args = [
            "-target",
            target.triple(),
            "-x",
            "ir",
            "-c",
            "-emit-llvm",
            "-S",
            "-",
            "-o",
            "-",
            "-O0",
            "-Xclang",
            "-llvm-verify-each",
        ];
        self.clang
            .run_with_input(verify_args, ir.text.as_bytes())
            .map_err(|error| match error {
                CodegenError::ToolFailure { stderr, .. } => {
                    CodegenError::VerificationFailed { message: stderr }
                }
                other => other,
            })?;
        let claim = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&options.object_path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    CodegenError::ExistingOutput(options.object_path.clone())
                } else {
                    CodegenError::Io {
                        path: options.object_path.clone(),
                        message: e.to_string(),
                    }
                }
            })?;
        drop(claim);
        let args = vec![
            OsString::from("-target"),
            OsString::from(target.triple()),
            OsString::from("-x"),
            OsString::from("ir"),
            OsString::from("-c"),
            OsString::from("-"),
            OsString::from("-o"),
            options.object_path.as_os_str().to_owned(),
            OsString::from(match options.optimization {
                OptimizationLevel::None => "-O0",
                OptimizationLevel::Default => "-O2",
            }),
            OsString::from("-Xclang"),
            OsString::from("-llvm-verify-each"),
        ];
        let result = self.clang.run_with_input(args, ir.text.as_bytes());
        if let Err(error) = result {
            let _ = fs::remove_file(&options.object_path);
            return Err(error);
        }
        let bytes = fs::read(&options.object_path).map_err(|e| CodegenError::Io {
            path: options.object_path.clone(),
            message: e.to_string(),
        })?;
        let valid = match target {
            TargetSpec::WindowsX64Msvc => bytes.starts_with(&[0x64, 0x86]),
            TargetSpec::LinuxX64Gnu => {
                bytes.starts_with(b"\x7fELF")
                    && bytes.get(4) == Some(&2)
                    && bytes.get(18..20) == Some(&[0x3e, 0][..])
            }
            _ => false,
        };
        if !valid {
            let _ = fs::remove_file(&options.object_path);
            return Err(CodegenError::VerificationFailed {
                message: "tool produced an invalid object or wrong machine type".into(),
            });
        }
        Ok(ObjectArtifact {
            path: options.object_path.clone(),
            target: *target,
        })
    }
}
