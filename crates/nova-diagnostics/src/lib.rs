//! Structured diagnostics and deterministic rendering (NOVA-078).

use nova_source::{SourceDatabase, SourceError, Span};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticCode(u16);

impl DiagnosticCode {
    /// Accept only codes from the NOVA-078 category ranges.
    pub fn new(value: u16) -> Option<Self> {
        match value / 1000 {
            1..=5 | 8..=9 => Some(Self(value)),
            _ => None,
        }
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{:04}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

impl Severity {
    fn name(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Label {
    pub span: Span,
    pub message: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Applicability {
    MachineApplicable,
    MaybeIncorrect,
    HasPlaceholders,
    Unspecified,
}

impl Applicability {
    fn name(self) -> &'static str {
        match self {
            Self::MachineApplicable => "MachineApplicable",
            Self::MaybeIncorrect => "MaybeIncorrect",
            Self::HasPlaceholders => "HasPlaceholders",
            Self::Unspecified => "Unspecified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Suggestion {
    pub span: Span,
    pub message: String,
    pub replacement: String,
    pub applicability: Applicability,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub message: String,
    pub primary: Label,
    pub secondary: Vec<Label>,
    pub notes: Vec<String>,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderFormat {
    Plain,
    Ansi,
    Json,
    Snapshot,
}

/// Rendering validates all spans before emitting anything. Invalid compiler
/// locations are returned as errors rather than panicking or being clamped.
pub fn render(
    diagnostic: &Diagnostic,
    sources: &SourceDatabase,
    format: RenderFormat,
) -> Result<String, SourceError> {
    sources.slice(diagnostic.primary.span)?;
    for label in &diagnostic.secondary {
        sources.slice(label.span)?;
    }
    for suggestion in &diagnostic.suggestions {
        sources.slice(suggestion.span)?;
    }
    if format == RenderFormat::Json {
        return render_json(diagnostic, sources);
    }
    let severity = diagnostic.severity.name();
    let heading = format!("{severity}[{}]: {}", diagnostic.code, diagnostic.message);
    let mut output = if format == RenderFormat::Ansi {
        let color = match diagnostic.severity {
            Severity::Error => 31,
            Severity::Warning => 33,
            Severity::Note => 36,
        };
        format!("\x1b[{color}m{heading}\x1b[0m\n")
    } else {
        format!("{heading}\n")
    };
    append_label(&mut output, &diagnostic.primary, sources, true)?;
    for label in &diagnostic.secondary {
        append_label(&mut output, label, sources, false)?;
    }
    for note in &diagnostic.notes {
        output.push_str(&format!("  = note: {note}\n"));
    }
    for suggestion in &diagnostic.suggestions {
        let file = sources.file(suggestion.span.file())?;
        let location = file.location(suggestion.span.start())?;
        output.push_str(&format!(
            "  = help: {} at {}:{}:{}; replace bytes {}..{} with {:?} ({})\n",
            suggestion.message,
            file.path().to_string_lossy().replace('\\', "/"),
            location.line,
            location.column,
            suggestion.span.start(),
            suggestion.span.end(),
            suggestion.replacement,
            suggestion.applicability.name(),
        ));
    }
    Ok(output)
}

fn append_label(
    output: &mut String,
    label: &Label,
    sources: &SourceDatabase,
    primary: bool,
) -> Result<(), SourceError> {
    let file = sources.file(label.span.file())?;
    let start = file.location(label.span.start())?;
    let end = file.location(label.span.end())?;
    let kind = if primary { "-->" } else { ":::" };
    output.push_str(&format!(
        " {kind} {}:{}:{} (bytes {}..{})\n",
        file.path().to_string_lossy().replace('\\', "/"),
        start.line,
        start.column,
        label.span.start(),
        label.span.end(),
    ));
    // Each line is shown separately, keeping multi-line byte ranges intact.
    let final_line = if end.line > start.line && end.column == 1 {
        end.line - 1
    } else {
        end.line
    };
    for line in start.line..=final_line {
        let text = file.line_text(line).ok_or(SourceError::InvalidSpan)?;
        let column = if line == start.line { start.column } else { 1 };
        let end_column = if line == end.line {
            end.column
        } else {
            text.chars().count() + 1
        };
        let width = end_column.saturating_sub(column).max(1);
        output.push_str(&format!(" {line} | {text}\n"));
        output.push_str(&format!(
            "   | {}{}",
            " ".repeat(column - 1),
            "^".repeat(width),
        ));
        if line == start.line && !label.message.is_empty() {
            output.push(' ');
            output.push_str(&label.message);
        }
        output.push('\n');
    }
    Ok(())
}

fn json_string(text: &str) -> String {
    let mut output = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            c if c < '\u{20}' => output.push_str(&format!("\\u{:04x}", c as u32)),
            c => output.push(c),
        }
    }
    output.push('"');
    output
}

fn span_json(span: Span, sources: &SourceDatabase) -> Result<String, SourceError> {
    let file = sources.file(span.file())?;
    let start = file.location(span.start())?;
    let end = file.location(span.end())?;
    Ok(format!(
        "{{\"file_id\":{},\"path\":{},\"start\":{},\"end\":{},\"line\":{},\"column\":{},\"end_line\":{},\"end_column\":{}}}",
        span.file().as_u32(),
        json_string(&file.path().to_string_lossy().replace('\\', "/")),
        span.start(),
        span.end(),
        start.line,
        start.column,
        end.line,
        end.column,
    ))
}

fn label_json(label: &Label, sources: &SourceDatabase) -> Result<String, SourceError> {
    Ok(format!(
        "{{\"span\":{},\"message\":{}}}",
        span_json(label.span, sources)?,
        json_string(&label.message),
    ))
}

fn render_json(diagnostic: &Diagnostic, sources: &SourceDatabase) -> Result<String, SourceError> {
    let secondary = diagnostic
        .secondary
        .iter()
        .map(|label| label_json(label, sources))
        .collect::<Result<Vec<_>, _>>()?
        .join(",");
    let notes = diagnostic
        .notes
        .iter()
        .map(|note| json_string(note))
        .collect::<Vec<_>>()
        .join(",");
    let suggestions = diagnostic
        .suggestions
        .iter()
        .map(|suggestion| {
            Ok(format!(
                "{{\"span\":{},\"message\":{},\"replacement\":{},\"applicability\":{}}}",
                span_json(suggestion.span, sources)?,
                json_string(&suggestion.message),
                json_string(&suggestion.replacement),
                json_string(suggestion.applicability.name()),
            ))
        })
        .collect::<Result<Vec<_>, SourceError>>()?
        .join(",");
    Ok(format!(
        "{{\"code\":{},\"severity\":{},\"message\":{},\"primary\":{},\"secondary\":[{}],\"notes\":[{}],\"suggestions\":[{}]}}\n",
        json_string(&diagnostic.code.to_string()),
        json_string(diagnostic.severity.name()),
        json_string(&diagnostic.message),
        label_json(&diagnostic.primary, sources)?,
        secondary,
        notes,
        suggestions,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (SourceDatabase, Diagnostic) {
        let mut sources = SourceDatabase::default();
        let file = sources.add("sample.nova", "가x\n".into()).unwrap();
        let diagnostic = Diagnostic {
            code: DiagnosticCode::new(1001).unwrap(),
            severity: Severity::Error,
            message: "unexpected token".into(),
            primary: Label {
                span: Span::new(file, 3, 4).unwrap(),
                message: "remove this".into(),
            },
            secondary: vec![],
            notes: vec!["a note".into()],
            suggestions: vec![Suggestion {
                span: Span::new(file, 3, 4).unwrap(),
                message: "remove token".into(),
                replacement: String::new(),
                applicability: Applicability::MachineApplicable,
            }],
        };
        (sources, diagnostic)
    }

    #[test]
    fn diagnostic_snapshot() {
        let (sources, diagnostic) = fixture();
        let expected = concat!(
            "error[N1001]: unexpected token\n",
            " --> sample.nova:1:2 (bytes 3..4)\n",
            " 1 | 가x\n",
            "   |  ^ remove this\n",
            "  = note: a note\n",
            "  = help: remove token at sample.nova:1:2; replace bytes 3..4 with \"\" (MachineApplicable)\n",
        );
        assert_eq!(
            render(&diagnostic, &sources, RenderFormat::Snapshot).unwrap(),
            expected
        );
        assert_eq!(
            render(&diagnostic, &sources, RenderFormat::Plain).unwrap(),
            expected
        );
        assert!(render(&diagnostic, &sources, RenderFormat::Ansi)
            .unwrap()
            .starts_with("\x1b[31merror[N1001]"));
    }

    #[test]
    fn json_escapes_control_characters_and_preserves_metadata() {
        let (sources, mut diagnostic) = fixture();
        diagnostic.message = "\"quoted\"\n\t\u{0}\\".into();
        let output = render(&diagnostic, &sources, RenderFormat::Json).unwrap();
        assert!(output.contains("\"message\":\"\\\"quoted\\\"\\n\\t\\u0000\\\\\""));
        assert!(output.contains("\"start\":3,\"end\":4,\"line\":1,\"column\":2"));
        assert!(output.contains("\"applicability\":\"MachineApplicable\""));
        assert!(output.contains("\"notes\":[\"a note\"]"));
    }

    #[test]
    fn invalid_labels_and_suggestions_return_errors_in_all_formats() {
        let (sources, mut diagnostic) = fixture();
        diagnostic.suggestions[0].span =
            Span::new(diagnostic.primary.span.file(), 1, 2).unwrap();
        for format in [
            RenderFormat::Plain,
            RenderFormat::Ansi,
            RenderFormat::Json,
            RenderFormat::Snapshot,
        ] {
            assert_eq!(
                render(&diagnostic, &sources, format),
                Err(SourceError::InvalidSpan)
            );
        }
    }

    #[test]
    fn secondary_labels_multiline_and_eof() {
        let (sources, mut diagnostic) = fixture();
        let file = diagnostic.primary.span.file();
        diagnostic.primary.span = Span::new(file, 0, 5).unwrap();
        diagnostic.secondary.push(Label {
            span: Span::new(file, 5, 5).unwrap(),
            message: "EOF".into(),
        });
        let output = render(&diagnostic, &sources, RenderFormat::Plain).unwrap();
        assert!(output.contains("^^ remove this"));
        assert!(output.contains(" ::: sample.nova:2:1 (bytes 5..5)\n 2 | \n   | ^ EOF"));
    }

    #[test]
    fn diagnostic_code_ranges() {
        for value in [0, 999, 6000, 7000, 10000, u16::MAX] {
            assert_eq!(DiagnosticCode::new(value), None);
        }
        for value in [1000, 2999, 5999, 8000, 9999] {
            assert_eq!(
                DiagnosticCode::new(value).unwrap().to_string(),
                format!("N{value}")
            );
        }
    }
}
