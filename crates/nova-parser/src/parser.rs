use crate::{Parsed, ParserOptions, SyntheticToken};
use nova_ast::{Arena, AstNodeId, NodeKind, Visibility};
use nova_diagnostics::{Diagnostic, DiagnosticCode, Label, Severity};
use nova_source::{FileId, Span};
use nova_syntax::{Keyword, Symbol, Token, TokenKind};

pub(crate) struct Parser<'a> {
    tokens: &'a [Token],
    text: &'a str,
    cursor: usize,
    last_end: usize,
    file: FileId,
    text_len: usize,
    arena: Arena,
    diagnostics: Vec<Diagnostic>,
    synthetic: Vec<SyntheticToken>,
    delimiters: Vec<Token>,
    depth: usize,
    loop_depth: usize,
    options: ParserOptions,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        tokens: &'a [Token],
        options: ParserOptions,
        file: FileId,
        text: &'a str,
    ) -> Self {
        Self {
            tokens,
            text,
            cursor: 0,
            last_end: 0,
            file,
            text_len: text.len(),
            arena: Arena::default(),
            diagnostics: vec![],
            synthetic: vec![],
            delimiters: vec![],
            depth: 0,
            loop_depth: 0,
            options,
        }
    }

    pub(crate) fn run(mut self) -> Parsed {
        let mut items = vec![];
        while self.kind() != TokenKind::Eof {
            if self.at_end() {
                self.bump();
                continue;
            }
            let start = self.current().span.start();
            if self.kind() == TokenKind::Keyword(Keyword::Use) {
                items.push(self.import());
            } else if matches!(
                self.kind(),
                TokenKind::Keyword(Keyword::Public | Keyword::Internal | Keyword::Private)
            ) {
                items.push(self.visible());
            } else if self.kind() == TokenKind::Keyword(Keyword::Func) {
                items.push(self.function());
            } else if self.kind() == TokenKind::Keyword(Keyword::Struct) {
                items.push(self.structure());
            } else if self.kind() == TokenKind::Keyword(Keyword::Enum) {
                items.push(self.enum_declaration());
            } else if self.kind() == TokenKind::Keyword(Keyword::Const) {
                items.push(self.statement());
            } else {
                if self.kind() != TokenKind::Error {
                    self.report(
                        1102,
                        self.current().span,
                        "only function and const declarations are supported at top level",
                    );
                }
                self.recover_item();
                items.push(self.node(NodeKind::Error, start, vec![]));
            }
        }
        let root = self
            .arena
            .insert(NodeKind::Root, self.span(0, self.text_len), items)
            .expect("all parser nodes are within their source file");
        Parsed {
            arena: self.arena,
            root,
            diagnostics: self.diagnostics,
            synthetic_tokens: self.synthetic,
        }
    }

    fn import(&mut self) -> AstNodeId {
        let keyword = self.bump().span;
        let mut children = vec![];
        loop {
            let token = self.expect(TokenKind::Identifier);
            if token.span.start() == token.span.end() {
                break;
            }
            children.push(self.node(NodeKind::ImportSegment, token.span.start(), vec![]));
            if !self.eat(TokenKind::Symbol(Symbol::ColonColon)) {
                break;
            }
        }
        let alias = if self.eat(TokenKind::Keyword(Keyword::As)) {
            Some(self.expect(TokenKind::Identifier).span)
        } else {
            None
        };
        if children.len() < 2 {
            self.report(1102, keyword, "P11 requires a direct module::item import");
        }
        if !self.at_end() && self.kind() != TokenKind::Eof {
            self.report(
                1102,
                self.current().span,
                "only direct item imports are supported",
            );
            self.recover_item_tail();
        } else if self.at_end() {
            self.bump();
        }
        self.node(
            NodeKind::Import { alias, keyword },
            keyword.start(),
            children,
        )
    }
    fn visible(&mut self) -> AstNodeId {
        let token = self.bump();
        let visibility = match token.kind {
            TokenKind::Keyword(Keyword::Public) => Visibility::Public,
            TokenKind::Keyword(Keyword::Private) => Visibility::Private,
            _ => Visibility::Internal,
        };
        let child = match self.kind() {
            TokenKind::Keyword(Keyword::Func) => self.function(),
            TokenKind::Keyword(Keyword::Const) => self.statement(),
            TokenKind::Keyword(Keyword::Struct) => self.structure(),
            TokenKind::Keyword(Keyword::Enum) => self.enum_declaration(),
            _ => {
                self.report(
                    if self.kind() == TokenKind::Keyword(Keyword::Use) {
                        1102
                    } else {
                        1101
                    },
                    self.current().span,
                    "visibility requires a direct function or global const declaration",
                );
                self.recover_item_tail();
                self.node(NodeKind::Error, token.span.end(), vec![])
            }
        };
        self.node(
            NodeKind::Visible {
                visibility,
                keyword: token.span,
            },
            token.span.start(),
            vec![child],
        )
    }
    fn enum_declaration(&mut self) -> AstNodeId {
        let start = self.bump().span.start();
        let name = self.expect(TokenKind::Identifier).span;
        self.open(TokenKind::LeftBrace);
        let mut variants = vec![];
        while !matches!(
            self.kind(),
            TokenKind::RightBrace
                | TokenKind::Eof
                | TokenKind::Keyword(Keyword::Func | Keyword::Enum | Keyword::Struct)
        ) {
            if self.at_end() {
                self.bump();
                continue;
            }
            if self.kind() != TokenKind::Identifier {
                self.report(
                    1102,
                    self.current().span,
                    "expected a positional Copy variant",
                );
                self.recover_statement();
                if self.kind() != TokenKind::RightBrace && !self.at_end() {
                    self.bump();
                }
                continue;
            }
            let v = self.bump().span;
            let mut payload = vec![];
            if self.kind() == TokenKind::LeftParen {
                self.open(TokenKind::LeftParen);
                if self.kind() == TokenKind::RightParen {
                    self.report(
                        1102,
                        self.current().span,
                        "nullary variant declarations omit parentheses",
                    );
                }
                while !matches!(
                    self.kind(),
                    TokenKind::RightParen | TokenKind::RightBrace | TokenKind::Eof
                ) {
                    let before = self.cursor;
                    payload.push(self.type_node());
                    if !self.eat(TokenKind::Comma) || before == self.cursor {
                        break;
                    }
                }
                self.close(TokenKind::RightParen);
            }
            variants.push(self.node(NodeKind::Variant { name: v }, v.start(), payload));
            if self.at_end() {
                self.bump();
            } else if self.kind() != TokenKind::RightBrace {
                self.report(
                    if unsupported(self.kind()) { 1102 } else { 1101 },
                    self.current().span,
                    "expected variant END",
                );
                self.recover_statement();
            }
        }
        self.close(TokenKind::RightBrace);
        if variants.is_empty() {
            self.report(1102, name, "empty enums are outside P14");
        }
        self.node(
            if name.start() == name.end() {
                NodeKind::Error
            } else {
                NodeKind::Enum { name }
            },
            start,
            variants,
        )
    }

    fn match_statement(&mut self) -> AstNodeId {
        let keyword = self.bump().span;
        if !self.enter() {
            self.recover_statement();
            return self.node(NodeKind::Error, keyword.start(), vec![]);
        }
        let value = self.expression(0);
        self.open(TokenKind::LeftBrace);
        let mut children = vec![value];
        while !matches!(
            self.kind(),
            TokenKind::RightBrace | TokenKind::Eof | TokenKind::Keyword(Keyword::Func)
        ) {
            if self.at_end() {
                self.bump();
                continue;
            }
            let before = self.cursor;
            let start = self.current().span.start();
            let pattern = self.pattern();
            if self.kind() != TokenKind::Symbol(Symbol::FatArrow) {
                self.report(
                    1102,
                    self.current().span,
                    "guards and nested patterns are outside P14",
                );
                while !matches!(
                    self.kind(),
                    TokenKind::Symbol(Symbol::FatArrow) | TokenKind::RightBrace | TokenKind::Eof
                ) && !self.at_end()
                {
                    self.bump();
                }
            }
            self.expect(TokenKind::Symbol(Symbol::FatArrow));
            let body = self.block();
            children.push(self.node(NodeKind::Arm, start, vec![pattern, body]));
            if !self.eat(TokenKind::Comma) && !self.at_end() && self.kind() != TokenKind::RightBrace
            {
                self.report(1101, self.current().span, "expected arm END or comma");
                self.recover_statement();
            }
            if before == self.cursor {
                self.bump();
            }
        }
        self.close(TokenKind::RightBrace);
        self.depth -= 1;
        self.node(NodeKind::Match { keyword }, keyword.start(), children)
    }

    fn pattern(&mut self) -> AstNodeId {
        let token = self.current();
        let start = token.span.start();
        if let TokenKind::Keyword(Keyword::True | Keyword::False) = token.kind {
            self.bump();
            return self.node(
                NodeKind::PatternBoolean(token.kind == TokenKind::Keyword(Keyword::True)),
                start,
                vec![],
            );
        }
        if token.kind == TokenKind::Identifier {
            self.bump();
            if self.text.get(token.span.start()..token.span.end()) == Some("_") {
                return self.node(NodeKind::Wildcard, start, vec![]);
            }
            self.expect(TokenKind::Symbol(Symbol::ColonColon));
            let name = self.expect(TokenKind::Identifier).span;
            if name.start() == name.end() {
                return self.node(NodeKind::Error, start, vec![]);
            }
            let arguments = self.kind() == TokenKind::LeftParen;
            let mut binders = vec![];
            if arguments {
                self.open(TokenKind::LeftParen);
                while !matches!(
                    self.kind(),
                    TokenKind::RightParen
                        | TokenKind::RightBrace
                        | TokenKind::Eof
                        | TokenKind::Symbol(Symbol::FatArrow)
                ) {
                    let before = self.cursor;
                    let b = self.expect(TokenKind::Identifier).span;
                    if b.start() != b.end() {
                        let kind = if self.text.get(b.start()..b.end()) == Some("_") {
                            NodeKind::Wildcard
                        } else {
                            NodeKind::Binder { name: b }
                        };
                        binders.push(self.node(kind, b.start(), vec![]));
                    }
                    if !self.eat(TokenKind::Comma) || before == self.cursor {
                        break;
                    }
                }
                self.close(TokenKind::RightParen);
            }
            return self.node(
                NodeKind::PatternVariant {
                    owner: token.span,
                    name,
                    arguments,
                },
                start,
                binders,
            );
        }
        self.report(
            1102,
            token.span,
            "only Enum/Bool and wildcard patterns are supported",
        );
        if !matches!(
            self.kind(),
            TokenKind::RightBrace | TokenKind::Eof | TokenKind::Symbol(Symbol::FatArrow)
        ) {
            self.bump();
        }
        self.node(NodeKind::Error, start, vec![])
    }

    fn structure(&mut self) -> AstNodeId {
        let start = self.bump().span.start();
        let name = self.expect(TokenKind::Identifier).span;
        self.open(TokenKind::LeftBrace);
        let mut fields = vec![];
        while !matches!(self.kind(), TokenKind::RightBrace | TokenKind::Eof) {
            if matches!(
                self.kind(),
                TokenKind::Keyword(Keyword::Func | Keyword::Struct | Keyword::Use | Keyword::Const)
            ) {
                self.report(
                    1102,
                    self.current().span,
                    "unsupported struct member or missing closing brace",
                );
                break;
            }
            if self.at_end() {
                self.bump();
                continue;
            }
            let field_start = self.current().span.start();
            let visibility = match self.kind() {
                TokenKind::Keyword(Keyword::Public) => {
                    self.bump();
                    Visibility::Public
                }
                TokenKind::Keyword(Keyword::Private) => {
                    self.bump();
                    Visibility::Private
                }
                TokenKind::Keyword(Keyword::Internal) => {
                    self.bump();
                    Visibility::Internal
                }
                _ => Visibility::Internal,
            };
            if !matches!(self.kind(), TokenKind::Keyword(Keyword::Let | Keyword::Var)) {
                self.report(
                    1102,
                    self.current().span,
                    "P12 struct members must be typed let/var fields",
                );
                let before = self.cursor;
                self.recover_statement();
                if before == self.cursor {
                    self.bump();
                }
                fields.push(self.node(NodeKind::Error, field_start, vec![]));
                continue;
            }
            let mutable = self.bump().kind == TokenKind::Keyword(Keyword::Var);
            let name = self.expect(TokenKind::Identifier).span;
            self.expect(TokenKind::Colon);
            let ty = self.type_node();
            fields.push(self.node(
                NodeKind::Field {
                    name,
                    mutable,
                    visibility,
                },
                field_start,
                vec![ty],
            ));
            if self.at_end() {
                self.bump();
            } else if !matches!(self.kind(), TokenKind::RightBrace | TokenKind::Eof) {
                self.report(
                    1102,
                    self.current().span,
                    "field initializers and unsupported members are outside P12",
                );
                self.recover_statement();
            }
        }
        self.close(TokenKind::RightBrace);
        self.node(NodeKind::Struct { name }, start, fields)
    }
    fn assignment_ahead(&self) -> bool {
        let mut at = self.cursor;
        if self.tokens[at].kind != TokenKind::Identifier {
            return false;
        }
        at += 1;
        while self
            .tokens
            .get(at)
            .is_some_and(|t| t.kind == TokenKind::Dot)
        {
            at += 1;
            if !self.tokens.get(at).is_some_and(|t| {
                matches!(
                    t.kind,
                    TokenKind::Identifier | TokenKind::Integer | TokenKind::Float
                )
            }) {
                return false;
            }
            at += 1;
        }
        self.tokens
            .get(at)
            .is_some_and(|t| t.kind == TokenKind::Symbol(Symbol::Equal))
    }
    fn projection(&mut self, receiver: AstNodeId) -> AstNodeId {
        let start = self.arena.get(receiver).expect("receiver").span.start();
        self.bump();
        if matches!(self.kind(), TokenKind::Integer | TokenKind::Float) {
            let token = self.bump();
            let spelling = &self.text[token.span.start()..token.span.end()];
            let parts = spelling.split('.').collect::<Vec<_>>();
            if parts.len() <= 2 && parts.iter().all(|part| canonical_index(part)) {
                let mut value = receiver;
                let mut offset = token.span.start();
                for part in parts {
                    let index = self.span(offset, offset + part.len());
                    value = self
                        .arena
                        .insert(
                            NodeKind::TupleProjection { index },
                            self.span(start, index.end()),
                            vec![value],
                        )
                        .expect("projection subspans");
                    offset = index.end() + 1;
                }
                return value;
            }
            self.report(
                1102,
                token.span,
                "tuple selector must be canonical decimal digits",
            );
            return self.node(NodeKind::Error, start, vec![receiver]);
        }
        if self.kind() == TokenKind::Symbol(Symbol::Minus) {
            let span = self.bump().span;
            self.report(1102, span, "tuple selector cannot be negative");
            if self.kind() == TokenKind::Integer {
                self.bump();
            }
            return self.node(NodeKind::Error, start, vec![receiver]);
        }
        let name = self.expect(TokenKind::Identifier).span;
        self.node(NodeKind::Projection { name }, start, vec![receiver])
    }
    fn current(&self) -> Token {
        self.tokens[self.cursor]
    }
    fn kind(&self) -> TokenKind {
        self.current().kind
    }
    fn at_end(&self) -> bool {
        matches!(self.kind(), TokenKind::End(_))
    }
    fn bump(&mut self) -> Token {
        let token = self.current();
        if token.kind != TokenKind::Eof {
            self.cursor += 1;
            self.last_end = token.span.end();
        }
        token
    }
    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.kind() == kind {
            self.bump();
            true
        } else {
            false
        }
    }
    fn span(&self, start: usize, end: usize) -> Span {
        Span::new(self.file, start, end).expect("parser constructs ordered source ranges")
    }
    fn point(&self) -> Span {
        let start = self.current().span.start();
        self.span(start, start)
    }
    fn node(&mut self, kind: NodeKind, mut start: usize, children: Vec<AstNodeId>) -> AstNodeId {
        let mut end = self.last_end.max(start);
        for &id in &children {
            let child = self
                .arena
                .get(id)
                .expect("parser only connects existing arena nodes");
            start = start.min(child.span.start());
            end = end.max(child.span.end());
        }
        self.arena
            .insert(kind, self.span(start, end), children)
            .expect("parent span is constructed to contain every child")
    }
    fn report(&mut self, code: u16, span: Span, message: &str) {
        self.diagnostics.push(Diagnostic {
            code: DiagnosticCode::new(code).expect("approved parser diagnostic code"),
            severity: Severity::Error,
            message: message.into(),
            primary: Label {
                span,
                message: message.into(),
            },
            secondary: vec![],
            notes: vec![],
            suggestions: vec![],
        });
    }
    fn expect(&mut self, kind: TokenKind) -> Token {
        if self.kind() == kind {
            return self.bump();
        }
        let span = self.point();
        if self.kind() != TokenKind::Error {
            self.report(1101, span, &format!("expected {kind:?}"));
            if let (Some(diagnostic), Some(open)) =
                (self.diagnostics.last_mut(), self.delimiters.last())
            {
                diagnostic.secondary.push(Label {
                    span: open.span,
                    message: "delimiter opened here".into(),
                });
            }
        }
        self.synthetic.push(SyntheticToken { kind, span });
        Token { kind, span }
    }
    fn open(&mut self, kind: TokenKind) -> Token {
        let token = self.expect(kind);
        self.delimiters.push(token);
        token
    }
    fn close(&mut self, kind: TokenKind) {
        self.expect(kind);
        self.delimiters.pop();
    }
    fn enter(&mut self) -> bool {
        if self.depth == self.options.max_nesting {
            self.report(
                if self.loop_depth > 0 { 8901 } else { 1102 },
                self.current().span,
                "parser nesting limit exceeded (P01 max_nesting)",
            );
            if let Some(diagnostic) = self.diagnostics.last_mut() {
                diagnostic.notes.push(format!(
                    "configured nesting limit: {}",
                    self.options.max_nesting
                ));
            }
            false
        } else {
            self.depth += 1;
            true
        }
    }
    fn recover_item(&mut self) {
        // Always consume the trigger before searching for the next declaration.
        self.bump();
        self.recover_item_tail();
    }
    fn recover_item_tail(&mut self) {
        while !matches!(
            self.kind(),
            TokenKind::Eof
                | TokenKind::Keyword(
                    Keyword::Func
                        | Keyword::Struct
                        | Keyword::Const
                        | Keyword::Use
                        | Keyword::Public
                        | Keyword::Internal
                        | Keyword::Private
                )
        ) {
            self.bump();
        }
    }
    fn statement_boundary(&self) -> bool {
        self.at_end()
            || matches!(
                self.kind(),
                TokenKind::RightBrace
                    | TokenKind::Eof
                    | TokenKind::Keyword(
                        Keyword::Func
                            | Keyword::Let
                            | Keyword::Var
                            | Keyword::Const
                            | Keyword::Return
                            | Keyword::If
                            | Keyword::While
                            | Keyword::Break
                            | Keyword::Continue
                            | Keyword::Else
                    )
            )
    }
    fn recover_statement(&mut self) {
        // Nested unsupported constructs are skipped as a unit. Their `}` must
        // not accidentally close the enclosing function.
        let mut braces = 0usize;
        while self.kind() != TokenKind::Eof {
            if braces == 0 && self.statement_boundary() {
                break;
            }
            match self.kind() {
                TokenKind::LeftBrace => braces += 1,
                TokenKind::RightBrace if braces > 0 => braces -= 1,
                _ => {}
            }
            self.bump();
        }
    }

    fn function(&mut self) -> AstNodeId {
        let start = self.bump().span.start();
        let name = self.expect(TokenKind::Identifier).span;
        if self.kind() == TokenKind::Symbol(Symbol::Less) {
            self.report(
                1102,
                self.current().span,
                "generic functions are outside Stage A",
            );
            self.recover_item();
            return self.node(NodeKind::Error, start, vec![]);
        }
        self.open(TokenKind::LeftParen);
        let mut children = vec![];
        while !matches!(
            self.kind(),
            TokenKind::RightParen
                | TokenKind::LeftBrace
                | TokenKind::Eof
                | TokenKind::Keyword(Keyword::Func)
        ) && !self.at_end()
        {
            let before = self.cursor;
            if self.kind() != TokenKind::Identifier {
                self.report(
                    1102,
                    self.current().span,
                    "only typed ordinary parameters are supported in Stage A",
                );
                self.skip_list_element();
            } else {
                let parameter_name = self.bump().span;
                self.expect(TokenKind::Colon);
                let ty = self.type_node();
                children.push(self.node(
                    NodeKind::Parameter {
                        name: parameter_name,
                    },
                    parameter_name.start(),
                    vec![ty],
                ));
                if self.kind() == TokenKind::Symbol(Symbol::Equal) {
                    self.report(
                        1102,
                        self.current().span,
                        "default parameters are outside Stage A",
                    );
                    self.skip_list_element();
                }
            }
            if self.eat(TokenKind::Comma) {
                if self.kind() == TokenKind::RightParen {
                    break;
                }
            } else if self.kind() == TokenKind::Identifier {
                self.expect(TokenKind::Comma);
            } else {
                break;
            }
            if self.cursor == before {
                self.bump();
            }
        }
        self.close(TokenKind::RightParen);
        let parameters = children.len();
        let has_return_type = self.eat(TokenKind::Symbol(Symbol::Arrow));
        if has_return_type {
            children.push(self.type_node());
        }
        let body = self.block();
        children.push(body);
        self.node(
            NodeKind::Function {
                name,
                parameters,
                has_return_type,
            },
            start,
            children,
        )
    }
    fn skip_list_element(&mut self) {
        let mut nested = 0usize;
        while !matches!(
            self.kind(),
            TokenKind::Eof | TokenKind::Keyword(Keyword::Func)
        ) {
            if nested == 0
                && (matches!(
                    self.kind(),
                    TokenKind::Comma | TokenKind::RightParen | TokenKind::LeftBrace
                ) || self.at_end())
            {
                break;
            }
            match self.kind() {
                TokenKind::LeftParen | TokenKind::LeftBracket => nested += 1,
                TokenKind::RightParen | TokenKind::RightBracket if nested > 0 => nested -= 1,
                _ => {}
            }
            self.bump();
        }
    }
    fn type_node(&mut self) -> AstNodeId {
        self.type_node_with_comparison(false)
    }

    fn type_node_with_comparison(&mut self, comparison: bool) -> AstNodeId {
        let start = self.current().span.start();
        match self.kind() {
            TokenKind::Identifier => {
                self.bump();
                if matches!(
                    self.kind(),
                    TokenKind::Symbol(Symbol::Question | Symbol::ColonColon)
                ) || (!comparison && self.kind() == TokenKind::Symbol(Symbol::Less))
                {
                    self.report(
                        1102,
                        self.current().span,
                        "only simple type names are supported in Stage A",
                    );
                    self.skip_list_element();
                    self.node(NodeKind::Error, start, vec![])
                } else {
                    self.node(NodeKind::NamedType, start, vec![])
                }
            }
            TokenKind::LeftParen => {
                if !self.enter() {
                    self.skip_list_element();
                    return self.node(NodeKind::Error, start, vec![]);
                }
                self.open(TokenKind::LeftParen);
                if self.kind() == TokenKind::RightParen {
                    self.close(TokenKind::RightParen);
                    self.depth -= 1;
                    return self.node(NodeKind::UnitType, start, vec![]);
                }
                let mut children = vec![self.type_node()];
                self.expect(TokenKind::Comma);
                while !matches!(
                    self.kind(),
                    TokenKind::RightParen | TokenKind::Eof | TokenKind::RightBrace
                ) {
                    let before = self.cursor;
                    children.push(self.type_node());
                    if !self.eat(TokenKind::Comma) {
                        break;
                    }
                    if before == self.cursor {
                        break;
                    }
                }
                self.close(TokenKind::RightParen);
                self.depth -= 1;
                self.node(NodeKind::TupleType, start, children)
            }
            _ => {
                self.expect(TokenKind::Identifier);
                self.node(NodeKind::Error, start, vec![])
            }
        }
    }

    fn block(&mut self) -> AstNodeId {
        let start = self.current().span.start();
        if !self.enter() {
            self.recover_statement();
            return self.node(NodeKind::Error, start, vec![]);
        }
        self.open(TokenKind::LeftBrace);
        let mut statements = vec![];
        while !matches!(
            self.kind(),
            TokenKind::RightBrace
                | TokenKind::Eof
                | TokenKind::Keyword(Keyword::Func | Keyword::Else)
        ) {
            if self.at_end() {
                self.bump();
                continue;
            }
            let before = self.cursor;
            statements.push(self.statement());
            if before == self.cursor {
                self.bump();
            }
        }
        self.close(TokenKind::RightBrace);
        self.depth -= 1;
        self.node(NodeKind::Block, start, statements)
    }
    fn statement(&mut self) -> AstNodeId {
        let start = self.current().span.start();
        if self.kind() == TokenKind::Keyword(Keyword::If) {
            return self.if_statement();
        }
        if self.kind() == TokenKind::Keyword(Keyword::While) {
            return self.while_statement();
        }
        if self.kind() == TokenKind::Keyword(Keyword::Match) {
            return self.match_statement();
        }
        let id = match self.kind() {
            TokenKind::Keyword(Keyword::Let | Keyword::Var | Keyword::Const) => {
                let keyword = self.bump().kind;
                let mutable = keyword == TokenKind::Keyword(Keyword::Var);
                let constant = keyword == TokenKind::Keyword(Keyword::Const);
                let name = self.expect(TokenKind::Identifier).span;
                let has_type = self.eat(TokenKind::Colon);
                let mut children = vec![];
                if has_type {
                    children.push(self.type_node());
                }
                self.expect(TokenKind::Symbol(Symbol::Equal));
                children.push(self.expression(0));
                self.node(
                    NodeKind::Binding {
                        name,
                        has_type,
                        mutable,
                        constant,
                    },
                    start,
                    children,
                )
            }
            TokenKind::Identifier if self.assignment_ahead() => {
                self.bump();
                let mut target = self.node(NodeKind::Name, start, vec![]);
                while self.kind() == TokenKind::Dot {
                    target = self.projection(target);
                }
                self.bump();
                let value = self.expression(0);
                self.node(NodeKind::Assignment, start, vec![target, value])
            }
            TokenKind::Keyword(Keyword::Break | Keyword::Continue) => {
                let kind = if self.bump().kind == TokenKind::Keyword(Keyword::Break) {
                    NodeKind::Break
                } else {
                    NodeKind::Continue
                };
                if !self.at_end() && !matches!(self.kind(), TokenKind::RightBrace | TokenKind::Eof)
                {
                    self.report(
                        1102,
                        self.current().span,
                        "jump labels and values are unsupported",
                    );
                    self.recover_statement();
                }
                self.node(kind, start, vec![])
            }
            TokenKind::Keyword(Keyword::Return) => {
                self.bump();
                let children = if self.statement_boundary() {
                    vec![]
                } else {
                    vec![self.expression(0)]
                };
                self.node(NodeKind::Return, start, children)
            }
            _ => {
                let expression = self.expression(0);
                self.node(NodeKind::ExpressionStatement, start, vec![expression])
            }
        };
        if self.at_end() {
            self.bump();
        } else if !matches!(self.kind(), TokenKind::RightBrace | TokenKind::Eof) {
            let code = if unsupported(self.kind()) { 1102 } else { 1101 };
            self.report(
                code,
                self.current().span,
                "expected statement END; syntax is outside the P01/P04 grammar",
            );
            self.recover_statement();
        }
        id
    }
    fn if_statement(&mut self) -> AstNodeId {
        let start = self.current().span.start();
        if !self.enter() {
            self.bump();
            self.recover_statement();
            return self.node(NodeKind::Error, start, vec![]);
        }
        self.bump();
        let condition = self.expression(0);
        let then_block = self.block();
        let mut children = vec![condition, then_block];
        if self.eat(TokenKind::Keyword(Keyword::Else)) {
            children.push(if self.kind() == TokenKind::Keyword(Keyword::If) {
                self.if_statement()
            } else {
                self.block()
            });
        }
        self.depth -= 1;
        self.node(NodeKind::If, start, children)
    }

    fn while_statement(&mut self) -> AstNodeId {
        let start = self.current().span.start();
        self.loop_depth += 1;
        if !self.enter() {
            self.bump();
            self.recover_statement();
            self.loop_depth -= 1;
            return self.node(NodeKind::Error, start, vec![]);
        }
        self.bump();
        let condition = self.expression(0);
        let body = self.block();
        self.depth -= 1;
        self.loop_depth -= 1;
        self.node(NodeKind::While, start, vec![condition, body])
    }

    fn expression(&mut self, minimum: u8) -> AstNodeId {
        let start = self.current().span.start();
        if !self.enter() {
            self.skip_list_element();
            return self.node(NodeKind::Error, start, vec![]);
        }
        let mut left = self.prefix();
        let mut comparison = None;
        loop {
            if self.kind() == TokenKind::LeftParen && minimum <= 15 {
                left = self.call(left);
                continue;
            }
            if self.kind() == TokenKind::Keyword(Keyword::As) && minimum <= 15 {
                let keyword = self.bump().span;
                let target = self.type_node_with_comparison(true);
                let cast_start = self.arena.get(left).expect("parsed operand").span.start();
                left = self.node(NodeKind::Cast { keyword }, cast_start, vec![left, target]);
                continue;
            }
            if self.kind() == TokenKind::Dot && minimum <= 15 {
                left = self.projection(left);
                continue;
            }
            let Some((operator, precedence)) = binary(self.kind()) else {
                break;
            };
            if precedence < minimum {
                break;
            }
            let token = self.bump();
            if precedence == 5 {
                if let Some(first) = comparison {
                    self.report(1103, token.span, "comparison operators cannot be chained");
                    if let Some(diagnostic) = self.diagnostics.last_mut() {
                        diagnostic.secondary.push(Label {
                            span: first,
                            message: "first comparison operator".into(),
                        });
                    }
                } else {
                    comparison = Some(token.span);
                }
            }
            let right = self.expression(precedence + 1);
            left = self.node(NodeKind::Binary(operator), start, vec![left, right]);
        }
        self.depth -= 1;
        left
    }
    fn prefix(&mut self) -> AstNodeId {
        let token = self.current();
        let start = token.span.start();
        let kind = match token.kind {
            TokenKind::Symbol(operator @ (Symbol::Plus | Symbol::Minus | Symbol::Bang)) => {
                self.bump();
                let operand = self.expression(13);
                return self.node(NodeKind::Prefix(operator), start, vec![operand]);
            }
            TokenKind::Integer => NodeKind::Integer,
            TokenKind::Float => NodeKind::Float,
            TokenKind::Character => NodeKind::Character,
            TokenKind::String => NodeKind::String,
            TokenKind::Identifier
                if self
                    .tokens
                    .get(self.cursor + 1)
                    .is_some_and(|t| t.kind == TokenKind::Symbol(Symbol::ColonColon)) =>
            {
                let owner = self.bump().span;
                self.bump();
                let name = self.expect(TokenKind::Identifier).span;
                if name.start() == name.end() {
                    return self.node(NodeKind::Error, start, vec![]);
                }
                return self.node(NodeKind::VariantPath { owner, name }, start, vec![]);
            }
            TokenKind::Identifier => NodeKind::Name,
            TokenKind::Keyword(Keyword::True) => NodeKind::Boolean(true),
            TokenKind::Keyword(Keyword::False) => NodeKind::Boolean(false),
            TokenKind::LeftParen => {
                self.open(TokenKind::LeftParen);
                if self.kind() == TokenKind::RightParen {
                    self.close(TokenKind::RightParen);
                    return self.node(NodeKind::Unit, start, vec![]);
                }
                let value = self.expression(0);
                if self.eat(TokenKind::Comma) {
                    let mut children = vec![value];
                    while !matches!(
                        self.kind(),
                        TokenKind::RightParen | TokenKind::Eof | TokenKind::RightBrace
                    ) {
                        let before = self.cursor;
                        children.push(self.expression(0));
                        if !self.eat(TokenKind::Comma) {
                            break;
                        }
                        if before == self.cursor {
                            break;
                        }
                    }
                    self.close(TokenKind::RightParen);
                    return self.node(NodeKind::Tuple, start, children);
                }
                self.close(TokenKind::RightParen);
                return self.node(NodeKind::Group, start, vec![value]);
            }
            TokenKind::StringStart => return self.interpolated_string(),
            TokenKind::Error => {
                self.bump();
                return self.node(NodeKind::Error, start, vec![]);
            }
            _ => {
                let code = if unsupported(token.kind) { 1102 } else { 1101 };
                self.report(code, token.span, "expected a Stage A expression");
                // Keep closing tokens and synchronization points for their owner.
                if !self.statement_boundary()
                    && !matches!(
                        token.kind,
                        TokenKind::LeftBrace
                            | TokenKind::RightParen
                            | TokenKind::Comma
                            | TokenKind::InterpolationEnd
                            | TokenKind::StringEnd
                    )
                {
                    self.recover_statement();
                }
                return self.node(NodeKind::Error, start, vec![]);
            }
        };
        self.bump();
        self.node(kind, start, vec![])
    }
    fn call(&mut self, callee: AstNodeId) -> AstNodeId {
        let start = self.arena.get(callee).expect("callee exists").span.start();
        self.open(TokenKind::LeftParen);
        let mut children = vec![callee];
        while !matches!(
            self.kind(),
            TokenKind::RightParen
                | TokenKind::Eof
                | TokenKind::RightBrace
                | TokenKind::Keyword(Keyword::Func)
        ) && !self.at_end()
        {
            let before = self.cursor;
            if self.kind() == TokenKind::Identifier
                && self
                    .tokens
                    .get(self.cursor + 1)
                    .is_some_and(|t| t.kind == TokenKind::Colon)
            {
                self.report(
                    1102,
                    self.current().span,
                    "named arguments are outside Stage A",
                );
                self.bump();
                self.bump();
            }
            children.push(self.expression(0));
            if !self.eat(TokenKind::Comma) {
                break;
            }
            if before == self.cursor {
                self.bump();
            }
        }
        self.close(TokenKind::RightParen);
        self.node(NodeKind::Call, start, children)
    }
    fn interpolated_string(&mut self) -> AstNodeId {
        let start = self.open(TokenKind::StringStart).span.start();
        let mut children = vec![];
        while !matches!(self.kind(), TokenKind::StringEnd | TokenKind::Eof) {
            let before = self.cursor;
            let part_start = self.current().span.start();
            match self.kind() {
                TokenKind::StringText => {
                    self.bump();
                    children.push(self.node(NodeKind::StringText, part_start, vec![]));
                }
                TokenKind::InterpolationStart => {
                    self.open(TokenKind::InterpolationStart);
                    let value = self.expression(0);
                    self.close(TokenKind::InterpolationEnd);
                    children.push(self.node(NodeKind::Interpolation, part_start, vec![value]));
                }
                TokenKind::Error => {
                    self.bump();
                    children.push(self.node(NodeKind::Error, part_start, vec![]));
                }
                _ => {
                    self.report(
                        1101,
                        self.current().span,
                        "expected string text or interpolation",
                    );
                    break;
                }
            }
            if before == self.cursor {
                self.bump();
            }
        }
        self.close(TokenKind::StringEnd);
        self.node(NodeKind::InterpolatedString, start, children)
    }
}

fn binary(kind: TokenKind) -> Option<(Symbol, u8)> {
    let TokenKind::Symbol(operator) = kind else {
        return None;
    };
    let precedence = match operator {
        Symbol::OrOr => 1,
        Symbol::AndAnd => 3,
        Symbol::EqualEqual
        | Symbol::BangEqual
        | Symbol::Less
        | Symbol::LessEqual
        | Symbol::Greater
        | Symbol::GreaterEqual => 5,
        Symbol::Plus | Symbol::Minus => 7,
        Symbol::Star | Symbol::Slash | Symbol::Percent => 9,
        _ => return None,
    };
    Some((operator, precedence))
}

fn unsupported(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::LeftBracket
            | TokenKind::Dot
            | TokenKind::Symbol(
                Symbol::Equal
                    | Symbol::Question
                    | Symbol::ColonColon
                    | Symbol::At
                    | Symbol::FatArrow
            )
            | TokenKind::Keyword(
                Keyword::Struct
                    | Keyword::Class
                    | Keyword::Enum
                    | Keyword::Interface
                    | Keyword::For
                    | Keyword::Loop
                    | Keyword::Match
                    | Keyword::None
                    | Keyword::Change
                    | Keyword::Take
                    | Keyword::Shared
                    | Keyword::Weak
                    | Keyword::View
                    | Keyword::Using
                    | Keyword::Try
                    | Keyword::Pure
                    | Keyword::Static
                    | Keyword::Foreign
                    | Keyword::Unsafe
                    | Keyword::Until
                    | Keyword::Through
                    | Keyword::Exists
                    | Keyword::Use
                    | Keyword::Type
                    | Keyword::Lambda
                    | Keyword::Public
                    | Keyword::Private
                    | Keyword::Internal
            )
    )
}

fn canonical_index(text: &str) -> bool {
    text == "0"
        || text
            .as_bytes()
            .first()
            .is_some_and(|b| matches!(b, b'1'..=b'9'))
            && text.bytes().all(|b| b.is_ascii_digit())
}
