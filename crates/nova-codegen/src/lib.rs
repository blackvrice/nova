//! Backend-neutral interface for verified MIR (NOVA-093).
//! LLVM types and runtime policy do not belong in this crate.
use nova_mir::{CalleeId, Module, SourceInfo, ValidationError};
use nova_types::Type;
use std::fmt;
use std::path::PathBuf;

/// Immutable, validated ownership boundary. No mutable MIR access is exposed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodegenUnit {
    mir: Module,
}
impl CodegenUnit {
    pub fn new(mir: Module) -> Result<Self, CodegenError> {
        let errors = nova_mir::validate(&mir);
        if errors.is_empty() {
            Ok(Self { mir })
        } else {
            Err(CodegenError::InvalidMir(errors))
        }
    }
    pub fn mir(&self) -> &Module {
        &self.mir
    }
    /// P03 executable contract; fragment/object-only compilation does not call this.
    pub fn executable_entry(&self) -> Result<CalleeId, CodegenError> {
        let mains = self
            .mir
            .callees
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.builtin_print && c.name == "main")
            .collect::<Vec<_>>();
        let fail = |code, message: &str, source| CodegenError::InvalidEntry {
            code,
            message: message.into(),
            source,
        };
        if mains.is_empty() {
            return Err(fail(
                2001,
                "executable requires a user main function",
                self.mir.sources.last().copied(),
            ));
        }
        let (id, main) = mains[0];
        let source = self
            .mir
            .bodies
            .iter()
            .find(|b| b.callee.0 == id)
            .map(|b| b.source);
        if mains.len() != 1 {
            return Err(fail(
                2002,
                "executable has duplicate main functions",
                source,
            ));
        }
        if !main.parameters.is_empty() {
            return Err(fail(
                2201,
                "main must have no parameters and return Unit",
                source,
            ));
        }
        if main.return_type != Type::Unit {
            return Err(fail(
                2101,
                "main must have no parameters and return Unit",
                source,
            ));
        }
        Ok(CalleeId(id))
    }
}

/// Target identities already listed by NOVA-093. Listing does not imply a
/// working host linker, frozen ABI, or cross-run support.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetSpec {
    WindowsX64Msvc,
    LinuxX64Gnu,
    LinuxArm64,
    WindowsArm64,
}
impl TargetSpec {
    pub fn triple(self) -> &'static str {
        match self {
            Self::WindowsX64Msvc => "x86_64-pc-windows-msvc",
            Self::LinuxX64Gnu => "x86_64-unknown-linux-gnu",
            Self::LinuxArm64 => "aarch64-unknown-linux-gnu",
            Self::WindowsArm64 => "aarch64-pc-windows-msvc",
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptimizationLevel {
    None,
    Default,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodegenOptions {
    pub object_path: PathBuf,
    pub optimization: OptimizationLevel,
    /// Require main() -> Unit and emit the private startup bridge.
    pub executable: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectArtifact {
    pub path: PathBuf,
    pub target: TargetSpec,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CodegenError {
    InvalidMir(Vec<ValidationError>),
    UnsupportedTarget(TargetSpec),
    MissingTool(PathBuf),
    ToolInvocation {
        tool: PathBuf,
        message: String,
    },
    ToolFailure {
        tool: PathBuf,
        status: Option<i32>,
        stdout: String,
        stderr: String,
    },
    InvalidToolVersion {
        tool: PathBuf,
        output: String,
    },
    Io {
        path: PathBuf,
        message: String,
    },
    ExistingOutput(PathBuf),
    /// LLVM verifier errors are compiler invariant failures, not user syntax errors.
    VerificationFailed {
        message: String,
    },
    InvalidEntry {
        message: String,
        code: u16,
        source: Option<SourceInfo>,
    },
}
impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMir(errors) => {
                write!(f, "invalid MIR: {} invariant violation(s)", errors.len())
            }
            Self::UnsupportedTarget(target) => {
                write!(f, "unsupported backend target {}", target.triple())
            }
            Self::MissingTool(tool) => write!(f, "tool not found: {}", tool.display()),
            Self::ToolInvocation { tool, message } => {
                write!(f, "could not invoke {}: {message}", tool.display())
            }
            Self::ToolFailure {
                tool,
                status,
                stderr,
                ..
            } => write!(f, "{} failed ({status:?}): {stderr}", tool.display()),
            Self::InvalidToolVersion { tool, output } => {
                write!(
                    f,
                    "unsupported or unrecognized version from {}: {}",
                    tool.display(),
                    output.lines().next().unwrap_or("empty output")
                )
            }
            Self::Io { path, message } => write!(f, "{}: {message}", path.display()),
            Self::ExistingOutput(path) => write!(f, "output already exists: {}", path.display()),
            Self::VerificationFailed { message } => {
                write!(f, "LLVM verification failed: {message}")
            }
            Self::InvalidEntry { message, .. } => write!(f, "invalid executable entry: {message}"),
        }
    }
}
impl std::error::Error for CodegenError {}
pub trait CodegenBackend {
    fn codegen_unit(
        &self,
        unit: &CodegenUnit,
        target: &TargetSpec,
        options: &CodegenOptions,
    ) -> Result<ObjectArtifact, CodegenError>;
}
