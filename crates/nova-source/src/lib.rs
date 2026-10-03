//! Immutable UTF-8 source storage and half-open byte spans (NOVA-070).

use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub use nova_core_ids::FileId;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    file: FileId,
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(file: FileId, start: usize, end: usize) -> Result<Self, SourceError> {
        if start > end {
            return Err(SourceError::InvalidSpan);
        }
        Ok(Self { file, start, end })
    }

    pub const fn file(self) -> FileId {
        self.file
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceError {
    InvalidUtf8,
    InvalidSpan,
    UnknownFile,
    TooManyFiles,
}

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidUtf8 => "source is not valid UTF-8",
            Self::InvalidSpan => "source span is outside the file or splits a UTF-8 character",
            Self::UnknownFile => "unknown source file",
            Self::TooManyFiles => "source file ID capacity exceeded",
        })
    }
}

impl std::error::Error for SourceError {}

/// One-based line and Unicode scalar column for diagnostic display.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct LineIndex {
    starts: Vec<usize>,
}

impl LineIndex {
    fn new(text: &str) -> Self {
        let bytes = text.as_bytes();
        let mut starts = vec![0];
        let mut offset = 0;
        while offset < bytes.len() {
            match bytes[offset] {
                b'\r' => {
                    offset += 1;
                    if bytes.get(offset) == Some(&b'\n') {
                        offset += 1;
                    }
                    starts.push(offset);
                }
                b'\n' => {
                    offset += 1;
                    starts.push(offset);
                }
                _ => offset += 1,
            }
        }
        Self { starts }
    }

    pub fn line_count(&self) -> usize {
        self.starts.len()
    }
}

#[derive(Debug)]
pub struct SourceFile {
    path: PathBuf,
    text: String,
    lines: OnceLock<LineIndex>,
}

impl SourceFile {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn line_index(&self) -> &LineIndex {
        self.lines.get_or_init(|| LineIndex::new(&self.text))
    }

    pub fn location(&self, offset: usize) -> Result<Location, SourceError> {
        if !self.text.is_char_boundary(offset) {
            return Err(SourceError::InvalidSpan);
        }
        let starts = &self.line_index().starts;
        let line = starts.partition_point(|start| *start <= offset) - 1;
        let prefix = self
            .text
            .get(starts[line]..offset)
            .ok_or(SourceError::InvalidSpan)?;
        Ok(Location {
            line: line + 1,
            column: prefix.chars().count() + 1,
        })
    }

    pub fn line_text(&self, line: usize) -> Option<&str> {
        let index = line.checked_sub(1)?;
        let starts = &self.line_index().starts;
        let start = *starts.get(index)?;
        let end = starts.get(index + 1).copied().unwrap_or(self.text.len());
        self.text.get(start..end).map(|text| {
            text.strip_suffix("\r\n")
                .or_else(|| text.strip_suffix('\n'))
                .or_else(|| text.strip_suffix('\r'))
                .unwrap_or(text)
        })
    }
}

/// Append-only storage: existing IDs and spans never change when files are added.
#[derive(Debug, Default)]
pub struct SourceDatabase {
    files: Vec<SourceFile>,
}

impl SourceDatabase {
    pub fn add(&mut self, path: impl Into<PathBuf>, text: String) -> Result<FileId, SourceError> {
        let raw = u32::try_from(self.files.len()).map_err(|_| SourceError::TooManyFiles)?;
        self.files.push(SourceFile {
            path: path.into(),
            text,
            lines: OnceLock::new(),
        });
        Ok(FileId::from_raw(raw))
    }

    pub fn add_bytes(
        &mut self,
        path: impl Into<PathBuf>,
        bytes: Vec<u8>,
    ) -> Result<FileId, SourceError> {
        let text = String::from_utf8(bytes).map_err(|_| SourceError::InvalidUtf8)?;
        self.add(path, text)
    }

    pub fn file(&self, id: FileId) -> Result<&SourceFile, SourceError> {
        self.files
            .get(id.as_u32() as usize)
            .ok_or(SourceError::UnknownFile)
    }

    pub fn slice(&self, span: Span) -> Result<&str, SourceError> {
        self.file(span.file)?
            .text
            .get(span.start..span.end)
            .ok_or(SourceError::InvalidSpan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_byte_spans_and_scalar_columns() {
        let mut sources = SourceDatabase::default();
        let id = sources.add("unicode.nova", "가🙂x\r\nnext".into()).unwrap();
        assert_eq!(sources.slice(Span::new(id, 3, 7).unwrap()).unwrap(), "🙂");
        let file = sources.file(id).unwrap();
        assert!(file.lines.get().is_none());
        assert_eq!(
            file.location(7).unwrap(),
            Location { line: 1, column: 3 }
        );
        assert_eq!(
            file.location(10).unwrap(),
            Location { line: 2, column: 1 }
        );
        assert_eq!(file.line_text(1), Some("가🙂x"));
        assert!(file.lines.get().is_some());
        assert_eq!(file.location(4), Err(SourceError::InvalidSpan));
    }

    #[test]
    fn empty_eof_and_mixed_newlines() {
        let mut sources = SourceDatabase::default();
        let empty = sources.add("empty.nova", String::new()).unwrap();
        assert_eq!(
            sources.file(empty).unwrap().location(0).unwrap(),
            Location { line: 1, column: 1 }
        );
        assert_eq!(sources.slice(Span::new(empty, 0, 0).unwrap()), Ok(""));
        let id = sources.add("lines.nova", "a\r\nb\nc\r".into()).unwrap();
        let file = sources.file(id).unwrap();
        assert_eq!(file.line_index().line_count(), 4);
        assert_eq!(file.line_text(4), Some(""));
        assert_eq!(
            file.location(file.text().len()).unwrap(),
            Location { line: 4, column: 1 }
        );
        assert_eq!(file.location(100), Err(SourceError::InvalidSpan));
        assert_eq!(file.line_text(0), None);
    }

    #[test]
    fn invalid_input_does_not_modify_database() {
        let mut sources = SourceDatabase::default();
        assert_eq!(
            sources.add_bytes("bad.nova", vec![0xff]),
            Err(SourceError::InvalidUtf8)
        );
        let id = sources.add("good.nova", "ok".into()).unwrap();
        assert_eq!(id.as_u32(), 0);
        assert_eq!(Span::new(id, 2, 1), Err(SourceError::InvalidSpan));
        assert_eq!(
            sources.slice(Span::new(id, 0, 3).unwrap()),
            Err(SourceError::InvalidSpan)
        );
        assert!(matches!(
            sources.file(FileId::from_raw(99)),
            Err(SourceError::UnknownFile)
        ));
    }

    #[test]
    fn large_file_and_stable_ids() {
        let mut sources = SourceDatabase::default();
        let id = sources.add("large.nova", "가\n".repeat(100_000)).unwrap();
        sources.add("other.nova", "other".into()).unwrap();
        let file = sources.file(id).unwrap();
        assert_eq!(file.line_index().line_count(), 100_001);
        assert_eq!(
            file.location(400_000).unwrap(),
            Location {
                line: 100_001,
                column: 1
            }
        );
    }
}
