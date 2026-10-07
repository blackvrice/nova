//! P01 Stage A and P04 control syntax. Parsing does not resolve names or types.

mod parser;

use nova_ast::{Arena, AstNodeId};
use nova_diagnostics::Diagnostic;
use nova_source::{FileId, SourceDatabase, SourceError, Span};
use nova_syntax::{Token, TokenKind};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParserOptions {
    /// P01 validated range: 1..=128. Smaller limits suit constrained callers.
    pub max_nesting: usize,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self { max_nesting: 128 }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParseInputError {
    Source(SourceError),
    InvalidTokenStream,
    InvalidOptions,
}

impl fmt::Display for ParseInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(error) => error.fmt(f),
            Self::InvalidTokenStream => {
                f.write_str("parser requires ordered normalized tokens ending in EOF")
            }
            Self::InvalidOptions => f.write_str("parser max_nesting must be in 1..=128"),
        }
    }
}

impl std::error::Error for ParseInputError {}

impl From<SourceError> for ParseInputError {
    fn from(error: SourceError) -> Self {
        Self::Source(error)
    }
}

/// Missing punctuation is recorded separately; its empty span is not real text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SyntheticToken {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Parsed {
    pub arena: Arena,
    pub root: AstNodeId,
    pub diagnostics: Vec<Diagnostic>,
    pub synthetic_tokens: Vec<SyntheticToken>,
}

impl Parsed {
    /// Lexer diagnostics must also be checked by the caller before lowering.
    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
            || self
                .arena
                .iter()
                .any(|(_, node)| node.kind == nova_ast::NodeKind::Error)
    }
}

pub fn parse(
    sources: &SourceDatabase,
    file: FileId,
    tokens: &[Token],
) -> Result<Parsed, ParseInputError> {
    parse_with_options(sources, file, tokens, ParserOptions::default())
}

pub fn parse_with_options(
    sources: &SourceDatabase,
    file: FileId,
    tokens: &[Token],
    options: ParserOptions,
) -> Result<Parsed, ParseInputError> {
    if !(1..=128).contains(&options.max_nesting) {
        return Err(ParseInputError::InvalidOptions);
    }
    let text_len = sources.file(file)?.text().len();
    let mut previous_end = 0;
    for (index, token) in tokens.iter().enumerate() {
        if token.span.file() != file || token.span.start() < previous_end {
            return Err(ParseInputError::InvalidTokenStream);
        }
        sources.slice(token.span)?;
        previous_end = token.span.end();
        if matches!(
            token.kind,
            TokenKind::Whitespace | TokenKind::Comment | TokenKind::NewLine | TokenKind::Semicolon
        ) || (token.kind == TokenKind::Eof
            && (index + 1 != tokens.len()
                || token.span.start() != text_len
                || token.span.end() != text_len))
        {
            return Err(ParseInputError::InvalidTokenStream);
        }
    }
    if tokens.last().map(|t| t.kind) != Some(TokenKind::Eof) {
        return Err(ParseInputError::InvalidTokenStream);
    }
    Ok(parser::Parser::new(tokens, options, file, sources.file(file)?.text()).run())
}
