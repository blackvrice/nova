//! Syntax lowering and source origins, independent of resolution and type checking.
use nova_ast::{Arena, AstNode, AstNodeId, NodeKind};
use nova_source::{SourceDatabase, SourceError, Span};
use nova_syntax::Symbol;
use std::collections::HashMap;
use std::fmt::{self, Write};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct HirId(pub usize);
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct SymbolId(pub usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceOrigin {
    Source(AstNodeId),
    ImplicitReturn(AstNodeId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirKind {
    Module,
    Error,
    /// Children: parameters, canonical return type, body.
    Function {
        name: SymbolId,
        name_span: Span,
        parameters: usize,
    },
    Parameter {
        name: SymbolId,
        name_span: Span,
    },
    TypeName(SymbolId),
    UnitType,
    Block,
    Binding {
        name: SymbolId,
        name_span: Span,
        has_type: bool,
        mutable: bool,
        constant: bool,
    },
    Assignment,
    Return,
    If,
    While,
    Break,
    Continue,
    ExpressionStatement,
    Name(SymbolId),
    Integer(String),
    Float(String),
    Character(char),
    String(String),
    Boolean(bool),
    Unit,
    Group,
    Prefix(Symbol),
    Binary(Symbol),
    Call,
    Cast {
        keyword: Span,
    },
    InterpolatedString,
    Interpolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HirNode {
    pub kind: HirKind,
    pub span: Span,
    pub origin: SourceOrigin,
    pub children: Vec<HirId>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Module {
    nodes: Vec<HirNode>,
    symbols: Vec<String>,
    root: HirId,
}
impl Module {
    pub fn root(&self) -> HirId {
        self.root
    }
    pub fn nodes(&self) -> &[HirNode] {
        &self.nodes
    }
    pub fn node(&self, id: HirId) -> Option<&HirNode> {
        self.nodes.get(id.0)
    }
    pub fn symbol(&self, id: SymbolId) -> Option<&str> {
        self.symbols.get(id.0).map(String::as_str)
    }
    pub fn has_errors(&self) -> bool {
        self.nodes.iter().any(|n| n.kind == HirKind::Error)
    }
    pub fn dump(&self) -> String {
        let mut output = String::new();
        for (index, spelling) in self.symbols.iter().enumerate() {
            let _ = writeln!(output, "symbol {index} {spelling:?}");
        }
        for (index, node) in self.nodes.iter().enumerate() {
            let _ = writeln!(
                output,
                "{index} {:?} {}..{} {:?} {:?}",
                node.kind,
                node.span.start(),
                node.span.end(),
                node.children,
                node.origin
            );
        }
        output
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoweringError {
    Source(SourceError),
    MalformedAst,
}
impl fmt::Display for LoweringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(error) => error.fmt(f),
            Self::MalformedAst => f.write_str("invalid AST shape or literal spelling"),
        }
    }
}
impl std::error::Error for LoweringError {}
impl From<SourceError> for LoweringError {
    fn from(error: SourceError) -> Self {
        Self::Source(error)
    }
}

/// Caller retains Lexer/Parser diagnostics and gates later successful compilation.
pub fn lower(
    sources: &SourceDatabase,
    arena: &Arena,
    root: AstNodeId,
) -> Result<Module, LoweringError> {
    let root_node = arena.get(root).ok_or(LoweringError::MalformedAst)?;
    if root_node.kind != NodeKind::Root {
        return Err(LoweringError::MalformedAst);
    }
    // Validate all nodes before indexing the postorder map. Source tree edges
    // always point backward; malformed API input is not a user semantic error.
    for (id, node) in arena.iter() {
        sources.slice(node.span)?;
        if let NodeKind::Function { name, .. }
        | NodeKind::Parameter { name }
        | NodeKind::Binding { name, .. } = node.kind
        {
            sources.slice(name)?;
            if name.file() != node.span.file()
                || name.start() < node.span.start()
                || name.end() > node.span.end()
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::Cast { keyword } = node.kind {
            if sources.slice(keyword)? != "as"
                || keyword.file() != node.span.file()
                || keyword.start() < node.span.start()
                || keyword.end() > node.span.end()
                || node.children.len() != 2
                || arena
                    .get(node.children[0])
                    .map_or(true, |value| value.span.end() > keyword.start())
                || arena
                    .get(node.children[1])
                    .map_or(true, |target| target.span.start() < keyword.end())
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if node.span.file() != root_node.span.file()
            || !valid_shape(arena, node)
            || node.children.iter().any(|c| c.index() >= id.index())
        {
            return Err(LoweringError::MalformedAst);
        }
    }
    let mut module = Module {
        nodes: vec![],
        symbols: vec![],
        root: HirId(0),
    };
    let mut interned = HashMap::<String, SymbolId>::new();
    let mut mapping = vec![];
    for (id, node) in arena.iter() {
        let mut children = node
            .children
            .iter()
            .map(|c| mapping[c.index()])
            .collect::<Vec<HirId>>();
        let intern = |span: Span,
                      module: &mut Module,
                      interned: &mut HashMap<String, SymbolId>|
         -> Result<SymbolId, LoweringError> {
            let spelling = sources.slice(span)?;
            if spelling.is_empty() {
                return Err(LoweringError::MalformedAst);
            }
            let canonical = match spelling {
                "int" => "int32",
                "uint" => "uint32",
                "byte" => "uint8",
                "float" => "float32",
                "double" => "float64",
                _ => spelling,
            };
            // Alias normalization belongs to type syntax only, not value names.
            let spelling = if node.kind == NodeKind::NamedType {
                canonical
            } else {
                spelling
            };
            Ok(intern_symbol(spelling, module, interned))
        };
        let kind = match node.kind {
            NodeKind::Root => HirKind::Module,
            NodeKind::Error => HirKind::Error,
            NodeKind::Function { name, .. }
            | NodeKind::Parameter { name }
            | NodeKind::Binding { name, .. }
                if name.start() == name.end() =>
            {
                HirKind::Error
            }
            NodeKind::Function {
                name,
                parameters,
                has_return_type,
            } => {
                if !has_return_type {
                    let unit = HirId(module.nodes.len());
                    let point = Span::new(node.span.file(), name.end(), name.end())
                        .map_err(LoweringError::Source)?;
                    module.nodes.push(HirNode {
                        kind: HirKind::UnitType,
                        span: point,
                        origin: SourceOrigin::ImplicitReturn(id),
                        children: vec![],
                    });
                    children.insert(parameters, unit);
                }
                HirKind::Function {
                    name: intern(name, &mut module, &mut interned)?,
                    name_span: name,
                    parameters,
                }
            }
            NodeKind::Parameter { name } => HirKind::Parameter {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
            },
            NodeKind::NamedType if sources.slice(node.span)? == "void" => HirKind::UnitType,
            NodeKind::NamedType => {
                HirKind::TypeName(intern(node.span, &mut module, &mut interned)?)
            }
            NodeKind::UnitType => HirKind::UnitType,
            NodeKind::Block => HirKind::Block,
            NodeKind::Binding {
                name,
                has_type,
                mutable,
                constant,
            } => HirKind::Binding {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
                has_type,
                mutable,
                constant,
            },
            NodeKind::Assignment => HirKind::Assignment,
            NodeKind::Return => HirKind::Return,
            NodeKind::If => HirKind::If,
            NodeKind::While => HirKind::While,
            NodeKind::Break => HirKind::Break,
            NodeKind::Continue => HirKind::Continue,
            NodeKind::ExpressionStatement => HirKind::ExpressionStatement,
            NodeKind::Name => HirKind::Name(intern(node.span, &mut module, &mut interned)?),
            NodeKind::Integer => HirKind::Integer(sources.slice(node.span)?.into()),
            NodeKind::Float => {
                let spelling = sources.slice(node.span)?;
                if !float_spelling(spelling) {
                    return Err(LoweringError::MalformedAst);
                }
                HirKind::Float(spelling.into())
            }
            NodeKind::Character => HirKind::Character(decode_character(sources.slice(node.span)?)?),
            NodeKind::String => {
                let spelling = sources.slice(node.span)?;
                let body = spelling
                    .strip_prefix('"')
                    .and_then(|s| s.strip_suffix('"'))
                    .ok_or(LoweringError::MalformedAst)?;
                HirKind::String(decode_text(body)?)
            }
            NodeKind::StringText => HirKind::String(decode_text(sources.slice(node.span)?)?),
            NodeKind::Boolean(value) => HirKind::Boolean(value),
            NodeKind::Unit => HirKind::Unit,
            NodeKind::Group => HirKind::Group,
            NodeKind::Prefix(op) => HirKind::Prefix(op),
            NodeKind::Binary(op) => HirKind::Binary(op),
            NodeKind::Call => HirKind::Call,
            NodeKind::Cast { keyword } => HirKind::Cast { keyword },
            NodeKind::InterpolatedString => HirKind::InterpolatedString,
            NodeKind::Interpolation => HirKind::Interpolation,
        };
        let hir = HirId(module.nodes.len());
        module.nodes.push(HirNode {
            kind,
            span: node.span,
            origin: SourceOrigin::Source(id),
            children,
        });
        mapping.push(hir);
    }
    module.root = mapping[root.index()];
    Ok(module)
}

fn float_spelling(text: &str) -> bool {
    fn digits(bytes: &[u8], at: &mut usize) -> bool {
        let start = *at;
        while *at < bytes.len() && (bytes[*at].is_ascii_digit() || bytes[*at] == b'_') {
            if bytes[*at] == b'_'
                && (*at == start
                    || !bytes[*at - 1].is_ascii_digit()
                    || !bytes.get(*at + 1).is_some_and(u8::is_ascii_digit))
            {
                return false;
            }
            *at += 1;
        }
        *at > start
    }
    let bytes = text.as_bytes();
    let mut at = 0;
    if !digits(bytes, &mut at) {
        return false;
    }
    let mut real = false;
    if bytes.get(at) == Some(&b'.') {
        real = true;
        at += 1;
        if !digits(bytes, &mut at) {
            return false;
        }
    }
    if matches!(bytes.get(at), Some(b'e' | b'E')) {
        real = true;
        at += 1;
        if matches!(bytes.get(at), Some(b'+' | b'-')) {
            at += 1;
        }
        if !digits(bytes, &mut at) {
            return false;
        }
    }
    real && at == bytes.len()
}

fn intern_symbol(
    spelling: &str,
    module: &mut Module,
    interned: &mut HashMap<String, SymbolId>,
) -> SymbolId {
    if let Some(id) = interned.get(spelling) {
        return *id;
    }
    let id = SymbolId(module.symbols.len());
    module.symbols.push(spelling.into());
    interned.insert(spelling.into(), id);
    id
}

fn valid_shape(arena: &Arena, node: &AstNode) -> bool {
    if matches!(
        node.kind,
        NodeKind::Binding {
            mutable: true,
            constant: true,
            ..
        }
    ) {
        return false;
    }
    let kinds = node
        .children
        .iter()
        .map(|c| arena.get(*c).map(|n| n.kind))
        .collect::<Option<Vec<_>>>();
    let Some(kinds) = kinds else {
        return false;
    };
    let ty = |kind: NodeKind| {
        matches!(
            kind,
            NodeKind::NamedType | NodeKind::UnitType | NodeKind::Error
        )
    };
    let expr = |kind: NodeKind| {
        matches!(
            kind,
            NodeKind::Name
                | NodeKind::Integer
                | NodeKind::Float
                | NodeKind::Character
                | NodeKind::String
                | NodeKind::Boolean(_)
                | NodeKind::Unit
                | NodeKind::Group
                | NodeKind::Prefix(_)
                | NodeKind::Binary(_)
                | NodeKind::Cast { .. }
                | NodeKind::Call
                | NodeKind::InterpolatedString
                | NodeKind::Error
        )
    };
    match node.kind {
        NodeKind::Root => kinds.iter().all(|k| {
            matches!(
                k,
                NodeKind::Function { .. }
                    | NodeKind::Binding {
                        constant: true,
                        mutable: false,
                        ..
                    }
                    | NodeKind::Error
            )
        }),
        NodeKind::Function {
            parameters,
            has_return_type,
            ..
        } => {
            let Some(expected) = parameters.checked_add(usize::from(has_return_type) + 1) else {
                return false;
            };
            kinds.len() == expected
                && kinds[..parameters]
                    .iter()
                    .all(|k| matches!(k, NodeKind::Parameter { .. } | NodeKind::Error))
                && (!has_return_type || ty(kinds[parameters]))
                && matches!(kinds.last(), Some(NodeKind::Block | NodeKind::Error))
        }
        NodeKind::Parameter { .. } => kinds.len() == 1 && ty(kinds[0]),
        NodeKind::Binding { has_type, .. } => {
            kinds.len() == 1 + usize::from(has_type)
                && (!has_type || ty(kinds[0]))
                && expr(*kinds.last().expect("length checked"))
        }
        NodeKind::Return => kinds.len() <= 1 && kinds.iter().all(|k| expr(*k)),
        NodeKind::Assignment => kinds.len() == 2 && kinds[0] == NodeKind::Name && expr(kinds[1]),
        NodeKind::While => {
            kinds.len() == 2
                && expr(kinds[0])
                && matches!(kinds[1], NodeKind::Block | NodeKind::Error)
        }
        NodeKind::If => {
            (kinds.len() == 2 || kinds.len() == 3)
                && expr(kinds[0])
                && matches!(kinds[1], NodeKind::Block | NodeKind::Error)
                && (kinds.len() == 2
                    || matches!(kinds[2], NodeKind::Block | NodeKind::If | NodeKind::Error))
        }
        NodeKind::Block => kinds.iter().all(|k| {
            matches!(
                k,
                NodeKind::Binding { .. }
                    | NodeKind::Assignment
                    | NodeKind::While
                    | NodeKind::Break
                    | NodeKind::Continue
                    | NodeKind::Return
                    | NodeKind::If
                    | NodeKind::ExpressionStatement
                    | NodeKind::Error
            )
        }),
        NodeKind::ExpressionStatement
        | NodeKind::Group
        | NodeKind::Prefix(_)
        | NodeKind::Interpolation => kinds.len() == 1 && expr(kinds[0]),
        NodeKind::Binary(_) => kinds.len() == 2 && kinds.iter().all(|k| expr(*k)),
        NodeKind::Cast { .. } => kinds.len() == 2 && expr(kinds[0]) && ty(kinds[1]),
        NodeKind::Call => !kinds.is_empty() && kinds.iter().all(|k| expr(*k)),
        NodeKind::InterpolatedString => kinds.iter().all(|k| {
            matches!(
                k,
                NodeKind::StringText | NodeKind::Interpolation | NodeKind::Error
            )
        }),
        NodeKind::Error => true,
        _ => kinds.is_empty(),
    }
}

fn decode_text(text: &str) -> Result<String, LoweringError> {
    decode_literal_body(text, false)
}

fn decode_character(spelling: &str) -> Result<char, LoweringError> {
    let body = spelling
        .strip_prefix('\'')
        .and_then(|s| s.strip_suffix('\''))
        .ok_or(LoweringError::MalformedAst)?;
    let decoded = decode_literal_body(body, true)?;
    let mut scalars = decoded.chars();
    let scalar = scalars.next().ok_or(LoweringError::MalformedAst)?;
    if scalars.next().is_some() {
        return Err(LoweringError::MalformedAst);
    }
    Ok(scalar)
}

fn decode_literal_body(text: &str, character_literal: bool) -> Result<String, LoweringError> {
    let mut chars = text.chars().peekable();
    let mut output = String::new();
    while let Some(character) = chars.next() {
        let decoded = match character {
            '\\' => match chars.next() {
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some('0') => '\0',
                Some(c @ ('\\' | '"' | '\'')) => c,
                Some('u') => {
                    if chars.next() != Some('{') {
                        return Err(LoweringError::MalformedAst);
                    }
                    let mut value = 0u32;
                    let mut digits = 0;
                    while chars.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
                        digits += 1;
                        if digits > 6 {
                            return Err(LoweringError::MalformedAst);
                        }
                        value = value * 16
                            + chars
                                .next()
                                .and_then(|c| c.to_digit(16))
                                .ok_or(LoweringError::MalformedAst)?;
                    }
                    if digits == 0 || chars.next() != Some('}') {
                        return Err(LoweringError::MalformedAst);
                    }
                    char::from_u32(value).ok_or(LoweringError::MalformedAst)?
                }
                _ => return Err(LoweringError::MalformedAst),
            },
            c @ ('{' | '}') if !character_literal => {
                if chars.next() != Some(c) {
                    return Err(LoweringError::MalformedAst);
                }
                c
            }
            '\r' | '\n' => return Err(LoweringError::MalformedAst),
            '\'' if character_literal => return Err(LoweringError::MalformedAst),
            c => c,
        };
        output.push(decoded);
    }
    Ok(output)
}
