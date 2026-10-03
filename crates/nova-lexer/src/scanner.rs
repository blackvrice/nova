use crate::Lexed;
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, Severity};
use nova_source::{FileId, Span};
use nova_syntax::{Keyword, Symbol, Token, TokenKind};

#[derive(Clone, Copy)]
enum Mode {
    Code,
    String {
        open: usize,
        segment: usize,
        token_start: usize,
        interpolated: bool,
        invalid: bool,
    },
    Interpolation {
        open: usize,
        braces: usize,
    },
}

pub(crate) struct Scanner<'a> {
    file: FileId,
    text: &'a str,
    cursor: usize,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
    modes: Vec<Mode>,
}

impl<'a> Scanner<'a> {
    pub(crate) fn new(file: FileId, text: &'a str) -> Self {
        Self {
            file,
            text,
            cursor: 0,
            tokens: Vec::new(),
            diagnostics: Vec::new(),
            modes: vec![Mode::Code],
        }
    }

    pub(crate) fn run(mut self) -> Lexed {
        loop {
            let mode = *self.modes.last().expect("root code mode is never removed");
            match mode {
                Mode::String { .. } => self.string_segment(),
                Mode::Interpolation { open, .. } if self.cursor == self.text.len() => {
                    self.unterminated(open, open + 1, "unterminated interpolation");
                    self.modes.pop();
                    self.resume_string();
                }
                _ if self.cursor == self.text.len() => break,
                _ => self.code_token(),
            }
        }
        self.emit(TokenKind::Eof, self.cursor, self.cursor);
        Lexed {
            tokens: self.tokens,
            diagnostics: self.diagnostics,
        }
    }

    fn span(&self, start: usize, end: usize) -> Span {
        // All cursor advances use complete UTF-8 scalars or known ASCII bytes.
        Span::new(self.file, start, end).expect("scanner spans are ordered cursor ranges")
    }

    fn emit(&mut self, kind: TokenKind, start: usize, end: usize) {
        self.tokens.push(Token {
            kind,
            span: self.span(start, end),
        });
    }

    fn error(&mut self, code: u16, start: usize, end: usize, message: &str) {
        self.diagnostics.push(Diagnostic {
            code: DiagnosticCode::new(code).expect("lexer codes are in N1xxx"),
            severity: Severity::Error,
            message: message.into(),
            primary: Label {
                span: self.span(start, end),
                message: message.into(),
            },
            secondary: vec![],
            notes: vec![],
            suggestions: vec![],
        });
    }

    fn unterminated(&mut self, start: usize, end: usize, message: &str) {
        self.error(1003, start, end, message);
        if let Some(diagnostic) = self.diagnostics.last_mut() {
            diagnostic.secondary.push(Label {
                span: Span::new(self.file, self.text.len(), self.text.len())
                    .expect("EOF is an ordered empty span"),
                message: "end of file reached here".into(),
            });
        }
    }

    fn current(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let character = self.current()?;
        self.cursor += character.len_utf8();
        Some(character)
    }

    fn starts(&self, prefix: &str) -> bool {
        self.text[self.cursor..].starts_with(prefix)
    }

    fn newline(&mut self) {
        let start = self.cursor;
        if self.starts("\r\n") {
            self.cursor += 2;
        } else {
            self.cursor += 1;
        }
        self.emit(TokenKind::NewLine, start, self.cursor);
    }

    fn code_token(&mut self) {
        let start = self.cursor;
        let character = self.current().expect("code scanner runs before EOF");
        if character == '}' {
            if let Some(Mode::Interpolation { braces: 0, .. }) = self.modes.last() {
                self.cursor += 1;
                self.emit(TokenKind::InterpolationEnd, start, self.cursor);
                self.modes.pop();
                self.resume_string();
                return;
            }
            if let Some(Mode::Interpolation { braces, .. }) = self.modes.last_mut() {
                *braces -= 1;
            }
        } else if character == '{' {
            if let Some(Mode::Interpolation { braces, .. }) = self.modes.last_mut() {
                *braces += 1;
            }
        }
        match character {
            '\r' | '\n' => self.newline(),
            ' ' | '\t' => {
                while matches!(self.current(), Some(' ' | '\t')) {
                    self.cursor += 1;
                }
                self.emit(TokenKind::Whitespace, start, self.cursor);
            }
            '\u{feff}' if start == 0 => {
                self.advance();
                self.emit(TokenKind::Whitespace, start, self.cursor);
            }
            '/' if self.starts("//") => self.line_comment(),
            '/' if self.starts("/*") => self.block_comment(),
            '"' => {
                self.cursor += 1;
                let token_start = self.tokens.len();
                self.emit(TokenKind::StringStart, start, self.cursor);
                self.modes.push(Mode::String {
                    open: start,
                    segment: self.cursor,
                    token_start,
                    interpolated: false,
                    invalid: false,
                });
            }
            '\'' => self.character_literal(),
            '0'..='9' => self.number(),
            c if c == '_' || unicode_ident::is_xid_start(c) => {
                self.advance();
                while self.current().is_some_and(unicode_ident::is_xid_continue) {
                    self.advance();
                }
                let word = &self.text[start..self.cursor];
                let kind = Keyword::from_spelling(word)
                    .map(TokenKind::Keyword)
                    .unwrap_or(TokenKind::Identifier);
                self.emit(kind, start, self.cursor);
            }
            _ => self.punctuation(),
        }
    }

    fn punctuation(&mut self) {
        let start = self.cursor;
        let pairs = [
            ("&&", Symbol::AndAnd),
            ("||", Symbol::OrOr),
            ("==", Symbol::EqualEqual),
            ("!=", Symbol::BangEqual),
            ("<=", Symbol::LessEqual),
            (">=", Symbol::GreaterEqual),
            ("->", Symbol::Arrow),
            ("=>", Symbol::FatArrow),
            ("::", Symbol::ColonColon),
        ];
        for (spelling, symbol) in pairs {
            if self.starts(spelling) {
                self.cursor += 2;
                self.emit(TokenKind::Symbol(symbol), start, self.cursor);
                return;
            }
        }
        let kind = match self.advance().expect("punctuation scanner runs before EOF") {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semicolon,
            '+' => TokenKind::Symbol(Symbol::Plus),
            '-' => TokenKind::Symbol(Symbol::Minus),
            '*' => TokenKind::Symbol(Symbol::Star),
            '/' => TokenKind::Symbol(Symbol::Slash),
            '%' => TokenKind::Symbol(Symbol::Percent),
            '!' => TokenKind::Symbol(Symbol::Bang),
            '=' => TokenKind::Symbol(Symbol::Equal),
            '<' => TokenKind::Symbol(Symbol::Less),
            '>' => TokenKind::Symbol(Symbol::Greater),
            '?' => TokenKind::Symbol(Symbol::Question),
            '@' => TokenKind::Symbol(Symbol::At),
            _ => {
                self.error(1001, start, self.cursor, "invalid source character");
                TokenKind::Error
            }
        };
        self.emit(kind, start, self.cursor);
    }

    fn line_comment(&mut self) {
        let start = self.cursor;
        while !matches!(self.current(), None | Some('\r' | '\n')) {
            self.advance();
        }
        self.emit(TokenKind::Comment, start, self.cursor);
    }

    fn block_comment(&mut self) {
        let open = self.cursor;
        let mut segment = open;
        let mut depth = 1usize;
        self.cursor += 2;
        while self.cursor < self.text.len() {
            if self.starts("/*") {
                depth += 1;
                self.cursor += 2;
            } else if self.starts("*/") {
                self.cursor += 2;
                depth -= 1;
                if depth == 0 {
                    self.emit(TokenKind::Comment, segment, self.cursor);
                    return;
                }
            } else if matches!(self.current(), Some('\r' | '\n')) {
                if segment < self.cursor {
                    self.emit(TokenKind::Comment, segment, self.cursor);
                }
                self.newline();
                segment = self.cursor;
            } else {
                self.advance();
            }
        }
        if segment < self.cursor {
            self.emit(TokenKind::Error, segment, self.cursor);
        }
        self.unterminated(open, open + 2, "unterminated block comment");
    }

    fn digits(&mut self, radix: u32) -> bool {
        let mut seen_digit = false;
        let mut previous_digit = false;
        let mut valid = true;
        while let Some(character) = self.current() {
            if character.is_digit(radix) {
                seen_digit = true;
                previous_digit = true;
                self.advance();
            } else if character == '_' {
                valid &= previous_digit;
                previous_digit = false;
                self.advance();
            } else {
                break;
            }
        }
        valid && seen_digit && previous_digit
    }

    fn number(&mut self) {
        let start = self.cursor;
        let mut floating = false;
        let mut valid;
        let radix = if self.starts("0x") {
            16
        } else if self.starts("0b") {
            2
        } else if self.starts("0o") {
            8
        } else {
            10
        };
        if radix != 10 {
            self.cursor += 2;
            valid = self.digits(radix);
        } else {
            valid = self.digits(10);
            let suffix = self.text.as_bytes().get(self.cursor + 1).copied();
            if self.current() == Some('.') && suffix.is_some_and(|b| b.is_ascii_digit()) {
                floating = true;
                self.cursor += 1;
                valid &= self.digits(10);
            }
            if matches!(self.current(), Some('e' | 'E')) {
                floating = true;
                self.cursor += 1;
                if matches!(self.current(), Some('+' | '-')) {
                    self.cursor += 1;
                }
                valid &= self.digits(10);
            }
        }
        if self.current().is_some_and(unicode_ident::is_xid_continue) {
            valid = false;
            while self.current().is_some_and(unicode_ident::is_xid_continue) {
                self.advance();
            }
        }
        let kind = if !valid {
            self.error(1002, start, self.cursor, "malformed numeric literal");
            TokenKind::Error
        } else if floating {
            TokenKind::Float
        } else {
            TokenKind::Integer
        };
        self.emit(kind, start, self.cursor);
    }

    /// Validate one escape without allocating decoded string data.
    fn escape(&mut self) -> Option<char> {
        let start = self.cursor;
        self.cursor += 1;
        let value = match self.current() {
            Some('n') => Some('\n'),
            Some('r') => Some('\r'),
            Some('t') => Some('\t'),
            Some('0') => Some('\0'),
            Some('\\') => Some('\\'),
            Some('"') => Some('"'),
            Some('\'') => Some('\''),
            Some('u') => return self.unicode_escape(start),
            Some('\r' | '\n') | None => {
                self.error(1002, start, self.cursor, "incomplete escape");
                return None;
            }
            Some(_) => None,
        };
        self.advance();
        if value.is_none() {
            self.error(1002, start, self.cursor, "invalid escape");
        }
        value
    }

    fn unicode_escape(&mut self, start: usize) -> Option<char> {
        self.cursor += 1; // u
        let mut value = 0u32;
        let mut count = 0usize;
        let mut valid = self.current() == Some('{');
        if valid {
            self.cursor += 1;
            while let Some(digit) = self.current().and_then(|c| c.to_digit(16)) {
                count += 1;
                if count <= 6 {
                    value = value * 16 + digit;
                }
                self.advance();
            }
            valid &= (1..=6).contains(&count) && self.current() == Some('}');
            if self.current() == Some('}') {
                self.cursor += 1;
            }
        }
        let scalar = if valid { char::from_u32(value) } else { None };
        if scalar.is_none() {
            self.error(1002, start, self.cursor, "invalid Unicode scalar escape");
        }
        scalar
    }

    fn character_literal(&mut self) {
        let start = self.cursor;
        self.cursor += 1;
        let mut count = 0usize;
        let mut valid = true;
        while !matches!(self.current(), None | Some('\'' | '\r' | '\n')) {
            if self.current() == Some('\\') {
                valid &= self.escape().is_some();
            } else {
                self.advance();
            }
            count += 1;
        }
        let closed = self.current() == Some('\'');
        if closed {
            self.cursor += 1;
        }
        if !closed && self.cursor == self.text.len() {
            self.unterminated(start, start + 1, "unterminated character literal");
        } else if !closed || count != 1 {
            self.error(
                1002,
                start,
                self.cursor,
                "character literal requires one Unicode scalar",
            );
        }
        valid &= closed && count == 1;
        self.emit(
            if valid {
                TokenKind::Character
            } else {
                TokenKind::Error
            },
            start,
            self.cursor,
        );
    }

    fn resume_string(&mut self) {
        if let Some(Mode::String { segment, .. }) = self.modes.last_mut() {
            *segment = self.cursor;
        }
    }

    fn string_segment(&mut self) {
        let Mode::String {
            open,
            segment,
            token_start,
            mut interpolated,
            mut invalid,
        } = *self.modes.last().expect("string scanner has a mode")
        else {
            unreachable!()
        };
        loop {
            match self.current() {
                Some('"') => {
                    if segment < self.cursor {
                        self.emit(
                            if invalid {
                                TokenKind::Error
                            } else {
                                TokenKind::StringText
                            },
                            segment,
                            self.cursor,
                        );
                    }
                    let closing = self.cursor;
                    self.cursor += 1;
                    self.modes.pop();
                    if interpolated {
                        self.emit(TokenKind::StringEnd, closing, self.cursor);
                    } else {
                        self.tokens.truncate(token_start);
                        self.emit(
                            if invalid {
                                TokenKind::Error
                            } else {
                                TokenKind::String
                            },
                            open,
                            self.cursor,
                        );
                    }
                    return;
                }
                Some('{') if !self.starts("{{") => {
                    if segment < self.cursor {
                        self.emit(
                            if invalid {
                                TokenKind::Error
                            } else {
                                TokenKind::StringText
                            },
                            segment,
                            self.cursor,
                        );
                    }
                    interpolated = true;
                    *self.modes.last_mut().expect("string mode exists") = Mode::String {
                        open,
                        segment: self.cursor,
                        token_start,
                        interpolated,
                        invalid: false,
                    };
                    let brace = self.cursor;
                    self.cursor += 1;
                    self.emit(TokenKind::InterpolationStart, brace, self.cursor);
                    self.modes.push(Mode::Interpolation {
                        open: brace,
                        braces: 0,
                    });
                    return;
                }
                Some('{' | '}') if self.starts("{{") || self.starts("}}") => self.cursor += 2,
                Some('}') => {
                    let start = self.cursor;
                    self.cursor += 1;
                    self.error(
                        1002,
                        start,
                        self.cursor,
                        "unescaped closing brace in string",
                    );
                    invalid = true;
                }
                Some('\\') => invalid |= self.escape().is_none(),
                Some('\r' | '\n') | None => {
                    if !interpolated {
                        self.tokens.truncate(token_start);
                        self.emit(TokenKind::Error, open, self.cursor);
                    } else if segment < self.cursor {
                        self.emit(TokenKind::Error, segment, self.cursor);
                    }
                    if self.cursor == self.text.len() {
                        self.unterminated(open, open + 1, "unterminated string literal");
                    } else {
                        self.error(1002, open, open + 1, "newline in string literal");
                    }
                    self.modes.pop();
                    return;
                }
                Some(_) => {
                    self.advance();
                }
            }
        }
    }
}
