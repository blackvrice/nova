//! Lossless UTF-8 tokenization. No parsing, type checking, or LLVM dependencies.

mod normalize;
mod scanner;

use nova_diagnostics::Diagnostic;
use nova_source::{FileId, SourceDatabase, SourceError};
use nova_syntax::Token;

pub use normalize::normalize_ends;
pub use unicode_ident::UNICODE_VERSION;

#[derive(Debug)]
pub struct Lexed {
    /// Includes every source byte exactly once, trivia and error tokens included.
    pub tokens: Vec<Token>,
    pub diagnostics: Vec<Diagnostic>,
}

impl Lexed {
    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Stable byte-oriented snapshot; literal spelling is preserved, not decoded.
    pub fn dump(&self, sources: &SourceDatabase) -> Result<String, SourceError> {
        let mut output = String::new();
        for token in &self.tokens {
            output.push_str(&format!(
                "{:?} {}..{} {:?}\n",
                token.kind,
                token.span.start(),
                token.span.end(),
                sources.slice(token.span)?,
            ));
        }
        Ok(output)
    }
}

pub fn lex(sources: &SourceDatabase, file: FileId) -> Result<Lexed, SourceError> {
    let text = sources.file(file)?.text();
    Ok(scanner::Scanner::new(file, text).run())
}
