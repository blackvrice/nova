use nova_syntax::{EndOrigin, Keyword, Symbol, Token, TokenKind};

#[derive(Clone, Copy, PartialEq)]
enum Delimiter {
    Paren,
    Bracket,
    Brace,
    Interpolation,
    TypeArguments,
}

/// Produce parser-facing tokens; the raw stream remains available for source
/// reconstruction. END retains the original newline/semicolon byte span.
pub fn normalize_ends(tokens: &[Token]) -> Vec<Token> {
    let mut next = vec![None; tokens.len()];
    let mut following = None;
    for (index, token) in tokens.iter().enumerate().rev() {
        next[index] = following;
        if !token.kind.is_trivia() && token.kind != TokenKind::NewLine {
            following = Some(token.kind);
        }
    }
    let mut output = Vec::new();
    let mut delimiters = Vec::new();
    let mut previous = None;
    let mut previous_type_close = false;
    let mut header = false;
    let mut function_header = false;
    let mut binding = false;
    let mut type_context = false;
    let mut expecting_decl_name = false;
    let mut generic_candidate = false;
    for (index, token) in tokens.iter().enumerate() {
        let kind = token.kind;
        if kind.is_trivia() {
            continue;
        }
        if kind == TokenKind::NewLine {
            let nested_expression = matches!(
                delimiters.last(),
                Some(
                    Delimiter::Paren
                        | Delimiter::Bracket
                        | Delimiter::Interpolation
                        | Delimiter::TypeArguments
                )
            );
            let bare_jump = matches!(
                previous,
                Some(TokenKind::Keyword(
                    Keyword::Return | Keyword::Break | Keyword::Continue
                ))
            );
            let continuation = (!previous_type_close && previous.is_some_and(continues_after))
                || next[index].is_some_and(continues_before)
                || (header && next[index] == Some(TokenKind::LeftBrace));
            if !nested_expression
                && (previous_type_close || previous.is_some_and(can_finish))
                && (bare_jump || !continuation)
            {
                output.push(Token {
                    kind: TokenKind::End(EndOrigin::NewLine),
                    span: token.span,
                });
                previous = Some(TokenKind::End(EndOrigin::NewLine));
                previous_type_close = false;
                header = false;
                function_header = false;
                binding = false;
                type_context = false;
                expecting_decl_name = false;
                generic_candidate = false;
            }
            continue;
        }
        if kind == TokenKind::Semicolon {
            output.push(Token {
                kind: TokenKind::End(EndOrigin::Semicolon),
                span: token.span,
            });
            previous = Some(TokenKind::End(EndOrigin::Semicolon));
            previous_type_close = false;
            header = false;
            function_header = false;
            binding = false;
            type_context = false;
            expecting_decl_name = false;
            generic_candidate = false;
            continue;
        }
        // A generic closing `>` can finish a type annotation. The same raw
        // token used as a comparison operator must still continue the expression.
        let closes_type = kind == TokenKind::Symbol(Symbol::Greater)
            && delimiters.last() == Some(&Delimiter::TypeArguments);
        match kind {
            TokenKind::Keyword(Keyword::Func) => {
                header = true;
                function_header = true;
                expecting_decl_name = true;
            }
            TokenKind::Keyword(
                Keyword::Struct
                | Keyword::Class
                | Keyword::Enum
                | Keyword::Interface
                | Keyword::Type,
            ) => {
                header = true;
                expecting_decl_name = true;
            }
            TokenKind::Keyword(
                Keyword::If
                | Keyword::Else
                | Keyword::While
                | Keyword::For
                | Keyword::Loop
                | Keyword::Match
                | Keyword::Foreign
                | Keyword::Init
                | Keyword::Drop
                | Keyword::Unsafe
                | Keyword::Lambda,
            ) => header = true,
            TokenKind::Keyword(Keyword::Let | Keyword::Var | Keyword::Const) => binding = true,
            TokenKind::Identifier if expecting_decl_name => {
                expecting_decl_name = false;
                generic_candidate = true;
            }
            TokenKind::Colon if binding || function_header => type_context = true,
            TokenKind::Symbol(Symbol::Arrow) if function_header => type_context = true,
            TokenKind::Symbol(Symbol::Less) if type_context || generic_candidate => {
                delimiters.push(Delimiter::TypeArguments);
                generic_candidate = false;
                type_context = true;
            }
            TokenKind::Symbol(Symbol::Greater)
                if delimiters.last() == Some(&Delimiter::TypeArguments) =>
            {
                delimiters.pop();
            }
            TokenKind::Symbol(Symbol::GreaterEqual)
                if delimiters.last() == Some(&Delimiter::TypeArguments) =>
            {
                delimiters.pop();
                binding = false;
                type_context = false;
                generic_candidate = false;
            }
            TokenKind::Symbol(Symbol::Equal) => {
                binding = false;
                type_context = false;
                generic_candidate = false;
            }
            TokenKind::Comma if delimiters.last() != Some(&Delimiter::TypeArguments) => {
                type_context = false
            }
            TokenKind::LeftParen => {
                delimiters.push(Delimiter::Paren);
                generic_candidate = false;
            }
            TokenKind::LeftBracket => delimiters.push(Delimiter::Bracket),
            TokenKind::LeftBrace => {
                delimiters.push(Delimiter::Brace);
                header = false;
                function_header = false;
                type_context = false;
                binding = false;
                expecting_decl_name = false;
                generic_candidate = false;
            }
            TokenKind::InterpolationStart => delimiters.push(Delimiter::Interpolation),
            TokenKind::RightParen => pop_matching(&mut delimiters, Delimiter::Paren),
            TokenKind::RightBracket => pop_matching(&mut delimiters, Delimiter::Bracket),
            TokenKind::RightBrace => pop_matching(&mut delimiters, Delimiter::Brace),
            TokenKind::InterpolationEnd => pop_matching(&mut delimiters, Delimiter::Interpolation),
            _ => {}
        }
        output.push(*token);
        previous = Some(kind);
        previous_type_close = closes_type;
    }
    output
}

fn pop_matching(delimiters: &mut Vec<Delimiter>, expected: Delimiter) {
    // Malformed delimiter diagnostics/recovery belong to the parser. Avoid a
    // repeated search through the stack for adversarial unmatched closers.
    if delimiters.last() == Some(&expected) {
        delimiters.pop();
    }
}

fn can_finish(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Identifier
            | TokenKind::Integer
            | TokenKind::Float
            | TokenKind::String
            | TokenKind::Character
            | TokenKind::StringEnd
            | TokenKind::RightParen
            | TokenKind::RightBracket
            | TokenKind::RightBrace
            | TokenKind::Symbol(Symbol::Question)
            | TokenKind::Keyword(
                Keyword::True
                    | Keyword::False
                    | Keyword::None
                    | Keyword::Exists
                    | Keyword::Return
                    | Keyword::Break
                    | Keyword::Continue
            )
    )
}

fn binary(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Symbol(
            Symbol::Plus
                | Symbol::Minus
                | Symbol::Star
                | Symbol::Slash
                | Symbol::Percent
                | Symbol::AndAnd
                | Symbol::OrOr
                | Symbol::Equal
                | Symbol::EqualEqual
                | Symbol::BangEqual
                | Symbol::Less
                | Symbol::LessEqual
                | Symbol::Greater
                | Symbol::GreaterEqual
                | Symbol::Arrow
                | Symbol::FatArrow
        ) | TokenKind::Keyword(Keyword::Until | Keyword::Through | Keyword::As)
    )
}

fn continues_before(kind: TokenKind) -> bool {
    binary(kind)
        || matches!(
            kind,
            TokenKind::Dot
                | TokenKind::Comma
                | TokenKind::Symbol(Symbol::ColonColon)
                | TokenKind::Keyword(Keyword::Else)
        )
}

fn continues_after(kind: TokenKind) -> bool {
    binary(kind)
        || matches!(
            kind,
            TokenKind::Dot
                | TokenKind::Comma
                | TokenKind::Colon
                | TokenKind::Symbol(Symbol::Bang | Symbol::ColonColon | Symbol::At)
                | TokenKind::Keyword(
                    Keyword::Else | Keyword::Try | Keyword::Take | Keyword::Change
                )
        )
}
