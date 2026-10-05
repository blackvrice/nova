//! P11 filesystem discovery. Compiler Core only receives immutable source/HIR.
use nova_ast::NodeKind;
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, Severity};
use nova_hir::Module;
use nova_source::{FileId, SourceDatabase, Span};
use std::collections::{BTreeMap, VecDeque};
use std::path::{Path, PathBuf};

pub const MAX_MODULES: usize = 1024;
#[derive(Debug)]
pub struct Loaded {
    pub sources: SourceDatabase,
    pub module: Option<Module>,
    pub diagnostics: Vec<Diagnostic>,
}
#[derive(Debug)]
pub struct InputError {
    pub code: u16,
    pub message: String,
}
impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "N{:04}: {}", self.code, self.message)
    }
}
impl std::error::Error for InputError {}
fn input(message: impl std::fmt::Display) -> InputError {
    InputError {
        code: 8001,
        message: message.to_string(),
    }
}
fn internal(message: impl std::fmt::Display) -> InputError {
    InputError {
        code: 101,
        message: message.to_string(),
    }
}
struct File {
    path: PathBuf,
    text: String,
    discovery_id: FileId,
    first_use: Option<Span>,
}
fn diagnostic(code: u16, span: Span, message: String, previous: Option<Span>) -> Diagnostic {
    Diagnostic {
        code: DiagnosticCode::new(code).expect("stable module diagnostic"),
        severity: Severity::Error,
        primary: Label {
            span,
            message: message.clone(),
        },
        message,
        secondary: previous
            .into_iter()
            .map(|span| Label {
                span,
                message: "first declaration of this module path".into(),
            })
            .collect(),
        notes: vec![],
        suggestions: vec![],
    }
}
fn read(path: &Path) -> Result<String, InputError> {
    let bytes = std::fs::read(path).map_err(|e| input(format!("{}: {e}", path.display())))?;
    String::from_utf8(bytes).map_err(|e| {
        input(format!(
            "{}: invalid UTF-8 at byte {}",
            path.display(),
            e.utf8_error().valid_up_to()
        ))
    })
}
// Canonical paths detect junction/symlink aliases. File identity additionally
// detects hard links, which canonicalization deliberately leaves distinct.
#[cfg(windows)]
fn physical_identity(path: &Path) -> Result<(u64, u64), InputError> {
    use std::os::windows::io::AsRawHandle;
    #[repr(C)]
    #[derive(Default)]
    struct Information {
        attributes: u32,
        created: [u32; 2],
        accessed: [u32; 2],
        written: [u32; 2],
        volume: u32,
        size: [u32; 2],
        links: u32,
        index_high: u32,
        index_low: u32,
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn GetFileInformationByHandle(
            handle: *mut std::ffi::c_void,
            information: *mut Information,
        ) -> i32;
    }
    let file = std::fs::File::open(path).map_err(input)?;
    let mut information = Information::default();
    // SAFETY: live File owns the handle; the writable C-layout buffer has the
    // complete BY_HANDLE_FILE_INFORMATION layout and outlives the OS call.
    if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut information) } == 0 {
        return Err(input(std::io::Error::last_os_error()));
    }
    Ok((
        information.volume.into(),
        (u64::from(information.index_high) << 32) | u64::from(information.index_low),
    ))
}
#[cfg(unix)]
fn physical_identity(path: &Path) -> Result<(u64, u64), InputError> {
    use std::os::unix::fs::MetadataExt;
    let metadata = std::fs::metadata(path).map_err(input)?;
    Ok((metadata.dev(), metadata.ino()))
}
#[cfg(not(any(unix, windows)))]
fn physical_identity(path: &Path) -> Result<PathBuf, InputError> {
    std::fs::canonicalize(path).map_err(input)
}
// Verify spelling before canonicalization: case-insensitive Windows lookup is
// insufficient for Nova's case-sensitive logical module paths.
fn exact_path(root: &Path, logical: &str) -> Result<PathBuf, InputError> {
    let parts = logical.split("::").collect::<Vec<_>>();
    let mut path = root.to_path_buf();
    for (index, part) in parts.iter().enumerate() {
        let name = if index + 1 == parts.len() {
            format!("{part}.nova")
        } else {
            (*part).into()
        };
        let metadata = std::fs::metadata(&path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                InputError {
                    code: 2001,
                    message: format!("module {logical} does not exist"),
                }
            } else {
                input(e)
            }
        })?;
        if !metadata.is_dir() {
            return Err(InputError {
                code: 2001,
                message: format!("module {logical} does not exist"),
            });
        }
        let mut matched = false;
        for item in std::fs::read_dir(&path).map_err(|e| {
            if matches!(e.kind(), std::io::ErrorKind::NotFound) {
                InputError {
                    code: 2001,
                    message: format!("module {logical} does not exist"),
                }
            } else {
                input(e)
            }
        })? {
            if item.map_err(input)?.file_name() == std::ffi::OsStr::new(&name) {
                matched = true;
                break;
            }
        }
        if !matched {
            return Err(InputError {
                code: 2001,
                message: format!("module {logical} does not exist with exact spelling"),
            });
        }
        path.push(name);
    }
    let path = std::fs::canonicalize(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            InputError {
                code: 2001,
                message: format!("module {logical} does not exist"),
            }
        } else {
            input(e)
        }
    })?;
    if !path.starts_with(root) {
        return Err(input(format!(
            "module {logical} resolves outside source root"
        )));
    }
    Ok(path)
}

/// Read only modules reached by direct item imports. Entry is FileId 0; other
/// files are assigned IDs in root-relative UTF-8 byte order after discovery.
pub fn load(entry: &Path, source_root: Option<&Path>) -> Result<Loaded, InputError> {
    if entry.extension() != Some(std::ffi::OsStr::new("nova")) {
        return Err(input("input must be a .nova file"));
    }
    let default_root = entry
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let root_path = source_root.unwrap_or(default_root);
    let root = std::fs::canonicalize(root_path)
        .map_err(|e| input(format!("{}: {e}", root_path.display())))?;
    if !root.is_dir() {
        return Err(input("source root is not a directory"));
    }
    let physical =
        std::fs::canonicalize(entry).map_err(|e| input(format!("{}: {e}", entry.display())))?;
    if !physical.starts_with(&root) {
        return Err(input(format!(
            "{}: entry resolves outside source root",
            entry.display()
        )));
    }
    let entry_parent = std::fs::canonicalize(default_root).map_err(input)?;
    let logical_entry = entry_parent.join(
        entry
            .file_name()
            .ok_or_else(|| input("entry has no filename"))?,
    );
    let relative = logical_entry
        .strip_prefix(&root)
        .map_err(|_| input("entry resolves outside source root"))?;
    let logical = relative
        .with_extension("")
        .components()
        .map(|c| {
            c.as_os_str()
                .to_str()
                .ok_or_else(|| input("module path must be UTF-8"))
        })
        .collect::<Result<Vec<_>, _>>()?
        .join("::");
    let text = read(&physical)?;
    let mut discovery = SourceDatabase::default();
    let discovery_id = discovery.add(entry, text.clone()).map_err(internal)?;
    let mut files = BTreeMap::from([(
        logical.clone(),
        File {
            path: entry.to_path_buf(),
            text,
            discovery_id,
            first_use: None,
        },
    )]);
    let mut physical_paths = BTreeMap::from([(physical_identity(&physical)?, logical.clone())]);
    let mut queue = VecDeque::from([logical.clone()]);
    let mut diagnostics = vec![];
    while let Some(current) = queue.pop_front() {
        let file = files[&current].discovery_id;
        let lexed = nova_lexer::lex(&discovery, file).map_err(internal)?;
        let parsed =
            nova_parser::parse(&discovery, file, &nova_lexer::normalize_ends(&lexed.tokens))
                .map_err(internal)?;
        for (_, node) in parsed.arena.iter() {
            if !matches!(node.kind, NodeKind::Import { .. }) || node.children.len() < 2 {
                continue;
            }
            let segments = node
                .children
                .iter()
                .map(|&id| discovery.slice(parsed.arena.get(id).expect("parsed child").span))
                .collect::<Result<Vec<_>, _>>()
                .map_err(internal)?;
            let target = segments[..segments.len() - 1].join("::");
            if files.contains_key(&target) {
                continue;
            }
            if let Some((_, previous)) = files
                .iter()
                .find(|(path, _)| path.eq_ignore_ascii_case(&target))
            {
                diagnostics.push(diagnostic(
                    2002,
                    node.span,
                    format!("ASCII case-only module path collision: {target}"),
                    previous.first_use,
                ));
                continue;
            }
            let path = match exact_path(&root, &target) {
                Ok(path) => path,
                Err(error) => {
                    diagnostics.push(diagnostic(error.code, node.span, error.message, None));
                    continue;
                }
            };
            let identity = match physical_identity(&path) {
                Ok(identity) => identity,
                Err(error) => {
                    diagnostics.push(diagnostic(error.code, node.span, error.message, None));
                    continue;
                }
            };
            if let Some(previous) = physical_paths.get(&identity) {
                diagnostics.push(diagnostic(
                    2002,
                    node.span,
                    format!("module {target} and {previous} resolve to the same physical file"),
                    files[previous].first_use,
                ));
                continue;
            }
            if files.len() == MAX_MODULES {
                diagnostics.push(diagnostic(
                    8901,
                    node.span,
                    "source bundle exceeds 1,024 modules".into(),
                    None,
                ));
                continue;
            }
            let text = match read(&path) {
                Ok(text) => text,
                Err(error) => {
                    diagnostics.push(diagnostic(error.code, node.span, error.message, None));
                    continue;
                }
            };
            let discovery_id = discovery.add(&path, text.clone()).map_err(internal)?;
            physical_paths.insert(identity, target.clone());
            files.insert(
                target.clone(),
                File {
                    path,
                    text,
                    discovery_id,
                    first_use: Some(node.span),
                },
            );
            queue.push_back(target);
        }
    }
    let mut order = files
        .keys()
        .filter(|&name| name != &logical)
        .cloned()
        .collect::<Vec<_>>();
    order.sort_by_cached_key(|name| format!("{}.nova", name.replace("::", "/")));
    order.insert(0, logical);
    let mut sources = SourceDatabase::default();
    let mut mapping = BTreeMap::new();
    for path in &order {
        let file = &files[path];
        mapping.insert(
            file.discovery_id,
            sources
                .add(&file.path, file.text.clone())
                .map_err(internal)?,
        );
    }
    for diagnostic in &mut diagnostics {
        for label in std::iter::once(&mut diagnostic.primary).chain(&mut diagnostic.secondary) {
            label.span = Span::new(
                mapping[&label.span.file()],
                label.span.start(),
                label.span.end(),
            )
            .map_err(internal)?;
        }
    }
    let mut inputs = vec![];
    for path in order {
        let file = mapping[&files[&path].discovery_id];
        let lexed = nova_lexer::lex(&sources, file).map_err(internal)?;
        let parsed = nova_parser::parse(&sources, file, &nova_lexer::normalize_ends(&lexed.tokens))
            .map_err(internal)?;
        diagnostics.extend(lexed.diagnostics.iter().cloned());
        diagnostics.extend(parsed.diagnostics.iter().cloned());
        if !lexed.has_errors() && !parsed.has_errors() {
            inputs.push((
                path,
                nova_hir::lower(&sources, &parsed.arena, parsed.root).map_err(internal)?,
            ));
        }
    }
    diagnostics.sort_by_key(|d| {
        (
            d.primary.span.file(),
            d.primary.span.start(),
            d.code.to_string(),
        )
    });
    let module = if diagnostics.iter().any(|d| d.severity == Severity::Error) {
        None
    } else {
        Some(Module::bundle(inputs).map_err(internal)?)
    };
    Ok(Loaded {
        sources,
        module,
        diagnostics,
    })
}
