//! Source-preserving token data, independent of the lexer and parser.

use nova_source::Span;

macro_rules! keywords {
    ($($variant:ident => $spelling:literal),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Keyword { $($variant),+ }

        impl Keyword {
            pub fn from_spelling(text: &str) -> Option<Self> {
                match text {
                    $($spelling => Some(Self::$variant),)+
                    _ => None,
                }
            }

            pub const fn spelling(self) -> &'static str {
                match self { $(Self::$variant => $spelling,)+ }
            }
        }
    };
}

keywords! {
    Func => "func", Let => "let", Var => "var", Const => "const",
    Struct => "struct", Class => "class", Enum => "enum",
    Interface => "interface", Implements => "implements", Init => "init", Drop => "drop",
    If => "if", Else => "else", While => "while", For => "for", In => "in", Loop => "loop",
    Break => "break", Continue => "continue", Return => "return", Match => "match",
    True => "true", False => "false", None => "none", Change => "change", Take => "take",
    Shared => "shared", Weak => "weak", View => "view", Using => "using", Try => "try",
    Panic => "panic", Pure => "pure", Public => "public", Internal => "internal",
    Private => "private", Static => "static", Where => "where", Foreign => "foreign",
    Unsafe => "unsafe", Until => "until", Through => "through", As => "as", Exists => "exists",
    Use => "use", Type => "type", Lambda => "lambda",
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndOrigin {
    NewLine,
    Semicolon,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Identifier,
    Keyword(Keyword),
    Integer,
    Float,
    String,
    Character,
    StringStart,
    StringText,
    StringEnd,
    InterpolationStart,
    InterpolationEnd,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Dot,
    Semicolon,
    Symbol(Symbol),
    NewLine,
    Whitespace,
    Comment,
    Error,
    End(EndOrigin),
    Eof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    AndAnd,
    OrOr,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Arrow,
    FatArrow,
    Question,
    ColonColon,
    At,
}

impl TokenKind {
    pub const fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }
}

/// Spelling is obtained from the source database, without copying token text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_keywords_match_the_original_table() {
        let specification =
            include_str!("../../../docs/00_Governance/NOVA-004_Nova_용어_키워드_Canonical_표.md");
        let table = specification
            .split("```text")
            .nth(1)
            .unwrap()
            .split("```")
            .next()
            .unwrap();
        for word in table.split_whitespace() {
            let keyword = Keyword::from_spelling(word).unwrap();
            assert_eq!(keyword.spelling(), word);
            assert_eq!(Keyword::from_spelling(&format!("{word}Suffix")), None);
        }
    }

    #[test]
    fn contextual_names_and_primitive_types_are_not_canonical_keywords() {
        for name in [
            "self", "c", "library", "int", "string", "void", "Read", "trait", "external",
        ] {
            assert_eq!(Keyword::from_spelling(name), None);
        }
    }

    #[test]
    fn newlines_and_errors_are_not_discardable_trivia() {
        assert!(TokenKind::Whitespace.is_trivia());
        assert!(TokenKind::Comment.is_trivia());
        assert!(!TokenKind::NewLine.is_trivia());
        assert!(!TokenKind::Error.is_trivia());
    }

    #[test]
    fn accepted_additions_are_keywords() {
        for word in ["use", "type", "lambda"] {
            assert_eq!(Keyword::from_spelling(word).unwrap().spelling(), word);
        }
    }
}
