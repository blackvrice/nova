//! Explicit tool identities and argument-array process calls; no shell execution.
use nova_codegen::CodegenError;
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClangTool {
    path: PathBuf,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClangVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub output: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolOutput {
    pub stdout: String,
    pub stderr: String,
}
impl ClangTool {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn run_with_input<I, S>(
        &self,
        arguments: I,
        input: &[u8],
    ) -> Result<ToolOutput, CodegenError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut child = Command::new(&self.path)
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| self.spawn_error(e))?;
        let mut stdin = child.stdin.take().expect("piped stdin");
        // Drain stdout/stderr while sending large modules; an early tool error
        // must not deadlock on a full pipe in either direction.
        let (write_result, output) = std::thread::scope(|scope| {
            let writer = scope.spawn(move || stdin.write_all(input));
            let output = child.wait_with_output();
            (writer.join(), output)
        });
        let output = output.map_err(|e| self.spawn_error(e))?;
        if !output.status.success() {
            return Err(CodegenError::ToolFailure {
                tool: self.path.clone(),
                status: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            });
        }
        write_result
            .map_err(|_| CodegenError::ToolInvocation {
                tool: self.path.clone(),
                message: "stdin worker panicked".into(),
            })?
            .map_err(|e| self.spawn_error(e))?;
        Ok(ToolOutput {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
    fn spawn_error(&self, error: std::io::Error) -> CodegenError {
        if error.kind() == std::io::ErrorKind::NotFound {
            CodegenError::MissingTool(self.path.clone())
        } else {
            CodegenError::ToolInvocation {
                tool: self.path.clone(),
                message: error.to_string(),
            }
        }
    }
    pub fn probe(&self) -> Result<ClangVersion, CodegenError> {
        let output = self.run([OsStr::new("--version")])?;
        parse_version(&output.stdout).ok_or_else(|| CodegenError::InvalidToolVersion {
            tool: self.path.clone(),
            output: output.stdout,
        })
    }
    /// Arguments stay individual OS strings, including spaces, Unicode and metacharacters.
    pub fn run<I, S>(&self, arguments: I) -> Result<ToolOutput, CodegenError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = Command::new(&self.path)
            .args(arguments)
            .output()
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    CodegenError::MissingTool(self.path.clone())
                } else {
                    CodegenError::ToolInvocation {
                        tool: self.path.clone(),
                        message: error.to_string(),
                    }
                }
            })?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        if !output.status.success() {
            return Err(CodegenError::ToolFailure {
                tool: self.path.clone(),
                status: output.status.code(),
                stdout,
                stderr,
            });
        }
        Ok(ToolOutput { stdout, stderr })
    }
}
/// Parse only a recognizable Clang banner, not arbitrary numeric tool output.
pub fn parse_version(output: &str) -> Option<ClangVersion> {
    let banner = output
        .lines()
        .find_map(|line| line.split_once("clang version ").map(|(_, rest)| rest))?;
    let version = banner.split_whitespace().next()?;
    let numeric = version.split('-').next()?;
    let mut parts = numeric.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    let patch = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(ClangVersion {
        major,
        minor,
        patch,
        output: output.to_owned(),
    })
}
