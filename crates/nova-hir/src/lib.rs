//! Syntax lowering and source origins, independent of resolution and type checking.
pub use nova_ast::Visibility;
use nova_ast::{Arena, AstNode, AstNodeId, NodeKind};
use nova_source::{FileId, SourceDatabase, SourceError, Span};
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
    FileSource(FileId, AstNodeId),
    FileImplicitReturn(FileId, AstNodeId),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirKind {
    Module,
    Visible {
        visibility: Visibility,
        keyword: Span,
    },
    Import {
        alias: Option<SymbolId>,
        alias_span: Option<Span>,
        keyword: Span,
    },
    ImportSegment(SymbolId),
    Error,
    /// Children: parameters, canonical return type, body.
    Function {
        name: SymbolId,
        name_span: Span,
        parameters: usize,
    },
    Receiver {
        name: SymbolId,
        name_span: Span,
        visibility: Visibility,
    },
    Parameter {
        name: SymbolId,
        name_span: Span,
    },
    DefaultValue {
        equals: Span,
    },
    Struct {
        name: SymbolId,
        name_span: Span,
    },
    TypeAlias {
        keyword: Span,
        name: SymbolId,
        name_span: Span,
        equals: Span,
    },
    Field {
        name: SymbolId,
        name_span: Span,
        mutable: bool,
        visibility: Visibility,
    },
    Projection {
        name: SymbolId,
        name_span: Span,
    },
    TupleProjection {
        index: SymbolId,
        index_span: Span,
    },
    Enum {
        name: SymbolId,
        name_span: Span,
    },
    Variant {
        name: SymbolId,
        name_span: Span,
    },
    VariantPath {
        owner: SymbolId,
        owner_span: Span,
        name: SymbolId,
        name_span: Span,
    },
    Match {
        keyword: Span,
    },
    Arm,
    PatternVariant {
        owner: SymbolId,
        owner_span: Span,
        name: SymbolId,
        name_span: Span,
        arguments: bool,
    },
    PatternBoolean(bool),
    Wildcard,
    Binder {
        name: SymbolId,
        name_span: Span,
    },
    Tuple,
    TupleType,
    GenericType {
        name: SymbolId,
        name_span: Span,
    },
    NullableType,
    None,
    PatternNone,
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
    For {
        keyword: Span,
        in_keyword: Span,
        operator: Span,
        inclusive: bool,
        range: Span,
    },
    Loop {
        keyword: Span,
    },
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
    Try {
        keyword: Span,
    },
    Exists {
        keyword: Span,
    },
    Binary(Symbol),
    Call,
    NamedArgument {
        name: SymbolId,
        name_span: Span,
        colon: Span,
    },
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

/// P22 original method punctuation and nominal owner, retained across bundle rebasing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MethodSource {
    pub owner: HirId,
    pub keyword: Span,
    pub name: Span,
    pub visibility_keyword: Option<Span>,
    pub left_paren: Span,
    pub right_paren: Span,
    pub receiver_comma: Option<Span>,
}
#[derive(Debug, Eq, PartialEq)]
pub struct Module {
    method_sources: HashMap<usize, MethodSource>,
    nodes: Vec<HirNode>,
    symbols: Vec<String>,
    root: HirId,
    units: Vec<ModuleUnit>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleUnit {
    pub path: String,
    pub root: HirId,
    pub file: FileId,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImportEdge {
    pub from: usize,
    pub to: usize,
    pub source: HirId,
}
impl Module {
    pub fn units(&self) -> &[ModuleUnit] {
        &self.units
    }
    pub fn items(&self) -> impl Iterator<Item = HirId> + '_ {
        self.units
            .iter()
            .flat_map(|unit| self.nodes[unit.root.0].children.iter().copied())
            .map(|id| {
                if matches!(self.nodes[id.0].kind, HirKind::Visible { .. }) {
                    self.nodes[id.0].children[0]
                } else {
                    id
                }
            })
    }
    /// Top-level items and nested methods in declaration order.
    pub fn semantic_items(&self) -> Vec<HirId> {
        let mut out = vec![];
        for item in self.items() {
            out.push(item);
            if matches!(self.nodes[item.0].kind, HirKind::Struct { .. }) {
                out.extend(
                    self.nodes[item.0]
                        .children
                        .iter()
                        .copied()
                        .filter(|id| matches!(self.nodes[id.0].kind, HirKind::Function { .. })),
                );
            }
        }
        out
    }
    pub fn method_source(&self, id: HirId) -> Option<&MethodSource> {
        self.method_sources.get(&id.0)
    }
    pub fn method_owner(&self, id: HirId) -> Option<HirId> {
        self.method_source(id).map(|m| m.owner)
    }
    pub fn owner(&self, id: HirId) -> Option<usize> {
        let node = self.node(id)?;
        self.units
            .iter()
            .position(|unit| unit.file == node.span.file())
    }
    pub fn visibility(&self, id: HirId) -> Visibility {
        if self.method_owner(id).is_some() {
            if let Some(receiver) = self.nodes[id.0].children.first() {
                if let HirKind::Receiver { visibility, .. } = self.nodes[receiver.0].kind {
                    return visibility;
                }
            }
        }
        let Some(unit) = self.owner(id) else {
            return Visibility::Private;
        };
        self.nodes[self.units[unit].root.0]
            .children
            .iter()
            .find_map(|&wrapper| match self.nodes[wrapper.0].kind {
                HirKind::Visible { visibility, .. } if self.nodes[wrapper.0].children[0] == id => {
                    Some(visibility)
                }
                _ => None,
            })
            .unwrap_or(Visibility::Internal)
    }
    pub fn import_path(&self, id: HirId) -> Option<Vec<&str>> {
        let node = self.node(id)?;
        if !matches!(node.kind, HirKind::Import { .. }) {
            return None;
        }
        node.children
            .iter()
            .map(|&child| match self.node(child)?.kind {
                HirKind::ImportSegment(symbol) => self.symbol(symbol),
                _ => None,
            })
            .collect()
    }
    pub fn edges(&self) -> Vec<ImportEdge> {
        self.items()
            .filter_map(|id| {
                let path = self.import_path(id)?;
                let to_path = path[..path.len().checked_sub(1)?].join("::");
                Some(ImportEdge {
                    from: self.owner(id)?,
                    to: self.units.iter().position(|u| u.path == to_path)?,
                    source: id,
                })
            })
            .collect()
    }
    /// Combine file-local HIR arenas without fabricating cross-file source parents.
    /// Input order is entry first, then root-relative byte-sorted module paths.
    pub fn bundle(inputs: Vec<(String, Module)>) -> Result<Self, LoweringError> {
        if inputs.is_empty() || inputs.len() > 1024 {
            return Err(LoweringError::MalformedAst);
        }
        let mut out = Self {
            method_sources: HashMap::new(),
            nodes: vec![],
            symbols: vec![],
            root: HirId(0),
            units: vec![],
        };
        let multi = inputs.len() > 1;
        let mut symbols = HashMap::new();
        for (path, module) in inputs {
            if module.units.len() != 1
                || (!out.units.is_empty() && !path.split("::").all(identifier))
                || (out.units.len() > 1
                    && relative_path(&out.units.last().expect("unit").path) > relative_path(&path))
                || out
                    .units
                    .iter()
                    .any(|u| u.path.eq_ignore_ascii_case(&path) || u.file == module.units[0].file)
            {
                return Err(LoweringError::MalformedAst);
            }
            let offset = out.nodes.len();
            for (&id, original) in &module.method_sources {
                let mut source = original.clone();
                source.owner.0 += offset;
                out.method_sources.insert(id + offset, source);
            }
            let mapping = module
                .symbols
                .iter()
                .map(|s| intern_symbol(s, &mut out, &mut symbols))
                .collect::<Vec<_>>();
            for mut node in module.nodes {
                for child in &mut node.children {
                    child.0 += offset;
                }
                match &mut node.kind {
                    HirKind::Function { name, .. }
                    | HirKind::Struct { name, .. }
                    | HirKind::TypeAlias { name, .. }
                    | HirKind::Enum { name, .. }
                    | HirKind::Variant { name, .. }
                    | HirKind::Binder { name, .. }
                    | HirKind::Field { name, .. }
                    | HirKind::Projection { name, .. }
                    | HirKind::Receiver { name, .. }
                    | HirKind::Parameter { name, .. }
                    | HirKind::NamedArgument { name, .. }
                    | HirKind::Binding { name, .. }
                    | HirKind::Name(name)
                    | HirKind::TupleProjection { index: name, .. }
                    | HirKind::TypeName(name)
                    | HirKind::GenericType { name, .. }
                    | HirKind::ImportSegment(name) => *name = mapping[name.0],
                    HirKind::VariantPath { owner, name, .. }
                    | HirKind::PatternVariant { owner, name, .. } => {
                        *owner = mapping[owner.0];
                        *name = mapping[name.0];
                    }
                    HirKind::Import {
                        alias: Some(alias), ..
                    } => *alias = mapping[alias.0],
                    _ => {}
                }
                if multi {
                    node.origin = match node.origin {
                        SourceOrigin::Source(id) => SourceOrigin::FileSource(node.span.file(), id),
                        SourceOrigin::ImplicitReturn(id) => {
                            SourceOrigin::FileImplicitReturn(node.span.file(), id)
                        }
                        _ => return Err(LoweringError::MalformedAst),
                    };
                }
                out.nodes.push(node);
            }
            let root = HirId(module.root.0 + offset);
            if out.units.is_empty() {
                out.root = root;
            }
            out.units.push(ModuleUnit {
                path,
                root,
                file: module.units[0].file,
            });
        }
        Ok(out)
    }

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
        if self.units.len() > 1 {
            for (index, unit) in self.units.iter().enumerate() {
                let _ = writeln!(
                    output,
                    "module {index} {:?} root {} file {}",
                    unit.path,
                    unit.root.0,
                    unit.file.as_u32()
                );
            }
            for edge in self.edges() {
                let _ = writeln!(
                    output,
                    "import {} -> {} @ {}",
                    edge.from, edge.to, edge.source.0
                );
            }
        }
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
        | NodeKind::Method { name, .. }
        | NodeKind::Receiver { name }
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
        if matches!(node.kind, NodeKind::Struct { .. })
            && node
                .children
                .iter()
                .any(|c| matches!(arena.get(*c).unwrap().kind, NodeKind::Method { .. }))
            && node.children.windows(2).any(|pair| {
                arena.get(pair[0]).unwrap().span.end() > arena.get(pair[1]).unwrap().span.start()
            })
        {
            return Err(LoweringError::MalformedAst);
        }
        if let NodeKind::Receiver { name } = node.kind {
            let text = sources.file(name.file())?.text();
            if name != node.span
                || sources.slice(name)? != "self"
                || !token_boundary(text, name)
                || arena
                    .iter()
                    .filter(|(_, p)| {
                        matches!(p.kind, NodeKind::Method { .. }) && p.children.first() == Some(&id)
                    })
                    .count()
                    != 1
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::Method {
            keyword,
            name,
            visibility,
            visibility_keyword,
            left_paren,
            right_paren,
            receiver_comma,
            parameters,
            has_return_type,
        } = node.kind
        {
            if !valid_shape(arena, node)
                || arena
                    .iter()
                    .filter(|(_, p)| {
                        matches!(p.kind, NodeKind::Struct { .. }) && p.children.contains(&id)
                    })
                    .count()
                    != 1
            {
                return Err(LoweringError::MalformedAst);
            }
            let text = sources.file(node.span.file())?.text();
            let receiver = arena
                .get(node.children[0])
                .ok_or(LoweringError::MalformedAst)?
                .span;
            let body = arena
                .get(*node.children.last().ok_or(LoweringError::MalformedAst)?)
                .ok_or(LoweringError::MalformedAst)?
                .span;
            let gap = |a: Span, b: Span, punctuation: &[&str]| -> Result<bool, LoweringError> {
                Ok(a.file() == b.file()
                    && a.end() <= b.start()
                    && type_punctuation(
                        sources.slice(Span::new(a.file(), a.end(), b.start())?)?,
                        punctuation,
                    ))
            };
            for (span, spelling) in [(keyword, "func"), (left_paren, "("), (right_paren, ")")] {
                if !inside(node.span, span) || sources.slice(span)? != spelling {
                    return Err(LoweringError::MalformedAst);
                }
            }
            if !inside(node.span, name)
                || !identifier(sources.slice(name)?)
                || !token_boundary(text, name)
                || !token_boundary(text, keyword)
                || !gap(keyword, name, &[""])?
                || !gap(name, left_paren, &[""])?
                || !gap(left_paren, receiver, &[""])?
                || body.end() != node.span.end()
                || node.children.windows(2).any(|pair| {
                    arena.get(pair[0]).unwrap().span.end()
                        > arena.get(pair[1]).unwrap().span.start()
                })
            {
                return Err(LoweringError::MalformedAst);
            }
            if let Some(v) = visibility_keyword {
                let spelling = match visibility {
                    Visibility::Public => "public",
                    Visibility::Private => "private",
                    Visibility::Internal => "internal",
                };
                if !inside(node.span, v)
                    || sources.slice(v)? != spelling
                    || !token_boundary(text, v)
                    || v.start() != node.span.start()
                    || !gap(v, keyword, &[""])?
                {
                    return Err(LoweringError::MalformedAst);
                }
            } else if visibility != Visibility::Internal || keyword.start() != node.span.start() {
                return Err(LoweringError::MalformedAst);
            }
            let after_receiver = if parameters > 1 {
                arena.get(node.children[1]).unwrap().span
            } else {
                right_paren
            };
            if let Some(comma) = receiver_comma {
                if !inside(node.span, comma)
                    || sources.slice(comma)? != ","
                    || !gap(receiver, comma, &[""])?
                    || !gap(comma, after_receiver, &[""])?
                {
                    return Err(LoweringError::MalformedAst);
                }
            } else if parameters != 1 || !gap(receiver, right_paren, &[""])? {
                return Err(LoweringError::MalformedAst);
            }
            for pair in node.children[1..parameters].windows(2) {
                if !gap(
                    arena.get(pair[0]).unwrap().span,
                    arena.get(pair[1]).unwrap().span,
                    &[","],
                )? {
                    return Err(LoweringError::MalformedAst);
                }
            }
            if parameters > 1
                && !gap(
                    arena.get(node.children[parameters - 1]).unwrap().span,
                    right_paren,
                    &["", ","],
                )?
            {
                return Err(LoweringError::MalformedAst);
            }
            if has_return_type {
                let ty = arena.get(node.children[parameters]).unwrap().span;
                if !gap(right_paren, ty, &["->"])? || !gap(ty, body, &[""])? {
                    return Err(LoweringError::MalformedAst);
                }
            } else if !gap(right_paren, body, &[""])? {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::For {
            keyword,
            in_keyword,
            operator,
            inclusive,
            range,
        } = node.kind
        {
            if node.children.len() != 4 {
                return Err(LoweringError::MalformedAst);
            }
            let children = node
                .children
                .iter()
                .map(|&c| arena.get(c).ok_or(LoweringError::MalformedAst))
                .collect::<Result<Vec<_>, _>>()?;
            let NodeKind::Binder { name } = children[0].kind else {
                return Err(LoweringError::MalformedAst);
            };
            let left = children[1].span;
            let right = children[2].span;
            let body = children[3].span;
            for (span, spelling) in [
                (keyword, "for"),
                (in_keyword, "in"),
                (operator, if inclusive { "through" } else { "until" }),
            ] {
                if !inside(node.span, span) || sources.slice(span)? != spelling {
                    return Err(LoweringError::MalformedAst);
                }
            }
            if keyword.start() != node.span.start()
                || body.end() != node.span.end()
                || range != Span::new(node.span.file(), left.start(), right.end())?
            {
                return Err(LoweringError::MalformedAst);
            }
            for (before, after) in [
                (keyword, name),
                (name, in_keyword),
                (in_keyword, left),
                (left, operator),
                (operator, right),
                (right, body),
            ] {
                if before.end() > after.start()
                    || !type_punctuation(
                        sources.slice(Span::new(node.span.file(), before.end(), after.start())?)?,
                        &[""],
                    )
                {
                    return Err(LoweringError::MalformedAst);
                }
            }
        }
        if let NodeKind::Loop { keyword } = node.kind {
            if node.children.len() != 1 {
                return Err(LoweringError::MalformedAst);
            }
            let body = arena
                .get(node.children[0])
                .ok_or(LoweringError::MalformedAst)?
                .span;
            if !inside(node.span, keyword)
                || sources.slice(keyword)? != "loop"
                || keyword.start() != node.span.start()
                || body.end() != node.span.end()
                || keyword.end() > body.start()
                || !type_punctuation(
                    sources.slice(Span::new(node.span.file(), keyword.end(), body.start())?)?,
                    &[""],
                )
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::DefaultValue { equals } = node.kind {
            let value = arena
                .get(*node.children.first().ok_or(LoweringError::MalformedAst)?)
                .ok_or(LoweringError::MalformedAst)?;
            if !inside(node.span, equals)
                || sources.slice(equals)? != "="
                || equals.start() != node.span.start()
                || equals.end() > value.span.start()
                || value.span.end() != node.span.end()
                || !type_punctuation(
                    sources.slice(Span::new(
                        node.span.file(),
                        equals.end(),
                        value.span.start(),
                    )?)?,
                    &[""],
                )
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if matches!(node.kind, NodeKind::Parameter { .. }) && node.children.len() == 2 {
            let ty = arena
                .get(node.children[0])
                .ok_or(LoweringError::MalformedAst)?;
            let default = arena
                .get(node.children[1])
                .ok_or(LoweringError::MalformedAst)?;
            if ty.span.end() > default.span.start()
                || default.span.end() != node.span.end()
                || !type_punctuation(
                    sources.slice(Span::new(
                        node.span.file(),
                        ty.span.end(),
                        default.span.start(),
                    )?)?,
                    &[""],
                )
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::NamedArgument { name, colon } = node.kind {
            let value = arena
                .get(*node.children.first().ok_or(LoweringError::MalformedAst)?)
                .ok_or(LoweringError::MalformedAst)?;
            if !inside(node.span, name)
                || !inside(node.span, colon)
                || !identifier(sources.slice(name)?)
                || sources.slice(colon)? != ":"
                || name.start() != node.span.start()
                || name.end() > colon.start()
                || colon.end() > value.span.start()
                || value.span.end() != node.span.end()
                || !type_punctuation(
                    sources.slice(Span::new(node.span.file(), name.end(), colon.start())?)?,
                    &[""],
                )
                || !type_punctuation(
                    sources.slice(Span::new(
                        node.span.file(),
                        colon.end(),
                        value.span.start(),
                    )?)?,
                    &[""],
                )
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if node.kind == NodeKind::Call
            && node.children.windows(2).any(|pair| {
                arena
                    .get(pair[0])
                    .zip(arena.get(pair[1]))
                    .is_some_and(|(a, b)| a.span.end() > b.span.start())
            })
        {
            return Err(LoweringError::MalformedAst);
        }
        if let NodeKind::TupleProjection { index } = node.kind {
            let digits = sources.slice(index)?;
            if !inside(node.span, index)
                || !(digits == "0"
                    || digits
                        .as_bytes()
                        .first()
                        .is_some_and(|b| matches!(b, b'1'..=b'9'))
                        && digits.bytes().all(|b| b.is_ascii_digit()))
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::TypeAlias {
            keyword,
            name,
            equals,
        } = node.kind
        {
            if node.children.len() != 1 {
                return Err(LoweringError::MalformedAst);
            }
            let target = arena
                .get(node.children[0])
                .ok_or(LoweringError::MalformedAst)?;
            let text = sources.file(node.span.file())?.text();
            let boundary = |span: Span| {
                !text
                    .get(..span.start())
                    .and_then(|s| s.chars().next_back())
                    .is_some_and(unicode_ident::is_xid_continue)
                    && !text
                        .get(span.end()..)
                        .and_then(|s| s.chars().next())
                        .is_some_and(unicode_ident::is_xid_continue)
            };
            if !inside(node.span, keyword)
                || !inside(node.span, name)
                || !inside(node.span, equals)
                || sources.slice(keyword)? != "type"
                || !identifier(sources.slice(name)?)
                || sources.slice(equals)? != "="
                || !boundary(keyword)
                || !boundary(name)
                || keyword.start() != node.span.start()
                || keyword.end() > name.start()
                || name.end() > equals.start()
                || equals.end() > target.span.start()
                || target.span.end() != node.span.end()
                || !type_punctuation(
                    sources.slice(Span::new(node.span.file(), keyword.end(), name.start())?)?,
                    &[""],
                )
                || !type_punctuation(
                    sources.slice(Span::new(node.span.file(), name.end(), equals.start())?)?,
                    &[""],
                )
                || !type_punctuation(
                    sources.slice(Span::new(
                        node.span.file(),
                        equals.end(),
                        target.span.start(),
                    )?)?,
                    &[""],
                )
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::Struct { name }
        | NodeKind::Field { name, .. }
        | NodeKind::Enum { name }
        | NodeKind::Variant { name }
        | NodeKind::Binder { name }
        | NodeKind::Projection { name }
        | NodeKind::GenericType { name } = node.kind
        {
            if !inside(node.span, name) || !identifier(sources.slice(name)?) {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::VariantPath { owner, name }
        | NodeKind::PatternVariant { owner, name, .. } = node.kind
        {
            if !inside(node.span, owner)
                || !inside(node.span, name)
                || owner.end() > name.start()
                || !identifier(sources.slice(owner)?)
                || !identifier(sources.slice(name)?)
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::Match { keyword } = node.kind {
            if !inside(node.span, keyword) || sources.slice(keyword)? != "match" {
                return Err(LoweringError::MalformedAst);
            }
        }
        if node.kind == NodeKind::NullableType {
            let child = arena
                .get(*node.children.first().ok_or(LoweringError::MalformedAst)?)
                .ok_or(LoweringError::MalformedAst)?;
            if !inside(node.span, child.span)
                || !type_punctuation(
                    sources.slice(Span::new(
                        node.span.file(),
                        child.span.end(),
                        node.span.end(),
                    )?)?,
                    &["?"],
                )
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::GenericType { name } = node.kind {
            let first = arena
                .get(*node.children.first().ok_or(LoweringError::MalformedAst)?)
                .ok_or(LoweringError::MalformedAst)?;
            let last = arena
                .get(*node.children.last().ok_or(LoweringError::MalformedAst)?)
                .ok_or(LoweringError::MalformedAst)?;
            if !inside(node.span, first.span)
                || !inside(node.span, last.span)
                || name.end() > first.span.start()
                || !type_punctuation(
                    sources.slice(Span::new(node.span.file(), name.end(), first.span.start())?)?,
                    &["<"],
                )
                || !type_punctuation(
                    sources.slice(Span::new(
                        node.span.file(),
                        last.span.end(),
                        node.span.end(),
                    )?)?,
                    &[">", ",>"],
                )
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if matches!(node.kind, NodeKind::None | NodeKind::PatternNone)
            && sources.slice(node.span)? != "none"
        {
            return Err(LoweringError::MalformedAst);
        }
        if node.kind == NodeKind::Wildcard && sources.slice(node.span)? != "_" {
            return Err(LoweringError::MalformedAst);
        }
        if let NodeKind::PatternBoolean(value) = node.kind {
            if sources.slice(node.span)? != if value { "true" } else { "false" } {
                return Err(LoweringError::MalformedAst);
            }
        }
        if node.kind == NodeKind::ImportSegment && !identifier(sources.slice(node.span)?) {
            return Err(LoweringError::MalformedAst);
        }
        if let NodeKind::Import { alias, keyword } = node.kind {
            if sources.slice(keyword)? != "use"
                || !inside(node.span, keyword)
                || alias.is_some_and(|span| !inside(node.span, span))
            {
                return Err(LoweringError::MalformedAst);
            }
            if let Some(alias) = alias {
                if !identifier(sources.slice(alias)?)
                    || node
                        .children
                        .last()
                        .and_then(|&id| arena.get(id))
                        .is_some_and(|last| alias.start() < last.span.end())
                {
                    return Err(LoweringError::MalformedAst);
                }
            }
            if node
                .children
                .first()
                .and_then(|&id| arena.get(id))
                .is_some_and(|first| first.span.start() < keyword.end())
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::Visible {
            visibility,
            keyword,
        } = node.kind
        {
            let spelling = match visibility {
                Visibility::Public => "public",
                Visibility::Internal => "internal",
                Visibility::Private => "private",
            };
            if sources.slice(keyword)? != spelling || !inside(node.span, keyword) {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::Try { keyword } = node.kind {
            if sources.slice(keyword)? != "try"
                || sources
                    .slice(node.span)?
                    .get(3..)
                    .and_then(|s| s.chars().next())
                    .is_some_and(unicode_ident::is_xid_continue)
                || !inside(node.span, keyword)
                || keyword.start() != node.span.start()
                || node.children.len() != 1
                || arena
                    .get(node.children[0])
                    .map_or(true, |n| n.span.start() < keyword.end())
            {
                return Err(LoweringError::MalformedAst);
            }
        }
        if let NodeKind::Exists { keyword } = node.kind {
            if node.children.len() != 1 {
                return Err(LoweringError::MalformedAst);
            }
            let operand = arena
                .get(node.children[0])
                .ok_or(LoweringError::MalformedAst)?;
            let text = sources.file(node.span.file())?.text();
            if sources.slice(keyword)? != "exists"
                || !inside(node.span, keyword)
                || text
                    .get(..keyword.start())
                    .and_then(|s| s.chars().next_back())
                    .is_some_and(unicode_ident::is_xid_continue)
                || text
                    .get(keyword.end()..)
                    .and_then(|s| s.chars().next())
                    .is_some_and(unicode_ident::is_xid_continue)
                || operand.span.start() != node.span.start()
                || operand.span.end() > keyword.start()
                || keyword.end() != node.span.end()
                || !type_punctuation(
                    sources.slice(Span::new(
                        node.span.file(),
                        operand.span.end(),
                        keyword.start(),
                    )?)?,
                    &[""],
                )
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
        method_sources: HashMap::new(),
        nodes: vec![],
        symbols: vec![],
        root: HirId(0),
        units: vec![],
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
            NodeKind::Visible {
                visibility,
                keyword,
            } => HirKind::Visible {
                visibility,
                keyword,
            },
            NodeKind::Import { alias, keyword } => HirKind::Import {
                alias: alias
                    .map(|span| intern(span, &mut module, &mut interned))
                    .transpose()?,
                alias_span: alias,
                keyword,
            },
            NodeKind::ImportSegment => {
                HirKind::ImportSegment(intern(node.span, &mut module, &mut interned)?)
            }
            NodeKind::Error => HirKind::Error,
            NodeKind::Function { name, .. }
            | NodeKind::Method { name, .. }
            | NodeKind::Receiver { name }
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
            }
            | NodeKind::Method {
                name,
                parameters,
                has_return_type,
                ..
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
            NodeKind::Enum { name } => HirKind::Enum {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
            },
            NodeKind::Variant { name } => HirKind::Variant {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
            },
            NodeKind::Binder { name } => HirKind::Binder {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
            },
            NodeKind::VariantPath { owner, name } => HirKind::VariantPath {
                owner: intern(owner, &mut module, &mut interned)?,
                owner_span: owner,
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
            },
            NodeKind::PatternVariant {
                owner,
                name,
                arguments,
            } => HirKind::PatternVariant {
                owner: intern(owner, &mut module, &mut interned)?,
                owner_span: owner,
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
                arguments,
            },
            NodeKind::Match { keyword } => HirKind::Match { keyword },
            NodeKind::Arm => HirKind::Arm,
            NodeKind::PatternBoolean(b) => HirKind::PatternBoolean(b),
            NodeKind::Wildcard => HirKind::Wildcard,
            NodeKind::Struct { name } => HirKind::Struct {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
            },
            NodeKind::TypeAlias {
                keyword,
                name,
                equals,
            } => HirKind::TypeAlias {
                keyword,
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
                equals,
            },
            NodeKind::Field {
                name,
                mutable,
                visibility,
            } => HirKind::Field {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
                mutable,
                visibility,
            },
            NodeKind::Projection { name } => HirKind::Projection {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
            },
            NodeKind::DefaultValue { equals } => HirKind::DefaultValue { equals },
            NodeKind::Receiver { name } => {
                let visibility = arena
                    .iter()
                    .find_map(|(_, parent)| {
                        if let NodeKind::Method { visibility, .. } = parent.kind {
                            if parent.children.first() == Some(&id) {
                                return Some(visibility);
                            }
                        }
                        None
                    })
                    .ok_or(LoweringError::MalformedAst)?;
                HirKind::Receiver {
                    name: intern(name, &mut module, &mut interned)?,
                    name_span: name,
                    visibility,
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
            NodeKind::TupleProjection { index } => HirKind::TupleProjection {
                index: intern(index, &mut module, &mut interned)?,
                index_span: index,
            },
            NodeKind::Tuple => HirKind::Tuple,
            NodeKind::TupleType => HirKind::TupleType,
            NodeKind::GenericType { name } => HirKind::GenericType {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
            },
            NodeKind::NullableType => HirKind::NullableType,
            NodeKind::None => HirKind::None,
            NodeKind::PatternNone => HirKind::PatternNone,
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
            NodeKind::For {
                keyword,
                in_keyword,
                operator,
                inclusive,
                range,
            } => HirKind::For {
                keyword,
                in_keyword,
                operator,
                inclusive,
                range,
            },
            NodeKind::Loop { keyword } => HirKind::Loop { keyword },
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
            NodeKind::Try { keyword } => HirKind::Try { keyword },
            NodeKind::Exists { keyword } => HirKind::Exists { keyword },
            NodeKind::Binary(op) => HirKind::Binary(op),
            NodeKind::Call => HirKind::Call,
            NodeKind::NamedArgument { name, colon } => HirKind::NamedArgument {
                name: intern(name, &mut module, &mut interned)?,
                name_span: name,
                colon,
            },
            NodeKind::Cast { keyword } => HirKind::Cast { keyword },
            NodeKind::InterpolatedString => HirKind::InterpolatedString,
            NodeKind::Interpolation => HirKind::Interpolation,
        };
        let hir = HirId(module.nodes.len());
        if let NodeKind::Method {
            keyword,
            name,
            visibility_keyword,
            left_paren,
            right_paren,
            receiver_comma,
            ..
        } = node.kind
        {
            module.method_sources.insert(
                hir.0,
                MethodSource {
                    owner: HirId(0),
                    keyword,
                    name,
                    visibility_keyword,
                    left_paren,
                    right_paren,
                    receiver_comma,
                },
            );
        }
        module.nodes.push(HirNode {
            kind,
            span: node.span,
            origin: SourceOrigin::Source(id),
            children,
        });
        mapping.push(hir);
    }
    for (owner, node) in module.nodes.iter().enumerate() {
        if matches!(node.kind, HirKind::Struct { .. }) {
            for member in &node.children {
                if let Some(source) = module.method_sources.get_mut(&member.0) {
                    source.owner = HirId(owner);
                }
            }
        }
    }
    module.root = mapping[root.index()];
    module.units.push(ModuleUnit {
        path: String::new(),
        root: module.root,
        file: root_node.span.file(),
    });
    Ok(module)
}

fn relative_path(path: &str) -> String {
    format!("{}.nova", path.replace("::", "/"))
}

fn identifier(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|c| c == '_' || unicode_ident::is_xid_start(c))
        && chars.all(unicode_ident::is_xid_continue)
        && nova_syntax::Keyword::from_spelling(text).is_none()
}

fn token_boundary(text: &str, span: Span) -> bool {
    !text
        .get(..span.start())
        .and_then(|s| s.chars().next_back())
        .is_some_and(unicode_ident::is_xid_continue)
        && !text
            .get(span.end()..)
            .and_then(|s| s.chars().next())
            .is_some_and(unicode_ident::is_xid_continue)
}
fn inside(parent: Span, child: Span) -> bool {
    parent.file() == child.file() && parent.start() <= child.start() && child.end() <= parent.end()
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
            NodeKind::NamedType
                | NodeKind::UnitType
                | NodeKind::TupleType
                | NodeKind::GenericType { .. }
                | NodeKind::NullableType
                | NodeKind::Error
        )
    };
    let expr = |kind: NodeKind| {
        matches!(
            kind,
            NodeKind::VariantPath { .. }
                | NodeKind::None
                | NodeKind::Name
                | NodeKind::Integer
                | NodeKind::Float
                | NodeKind::Character
                | NodeKind::String
                | NodeKind::Boolean(_)
                | NodeKind::Unit
                | NodeKind::Group
                | NodeKind::Try { .. }
                | NodeKind::Exists { .. }
                | NodeKind::Prefix(_)
                | NodeKind::Binary(_)
                | NodeKind::Cast { .. }
                | NodeKind::Call
                | NodeKind::Tuple
                | NodeKind::TupleProjection { .. }
                | NodeKind::Projection { .. }
                | NodeKind::InterpolatedString
                | NodeKind::Error
        )
    };
    match node.kind {
        NodeKind::Root => kinds.iter().all(|k| {
            matches!(
                k,
                NodeKind::Function { .. }
                    | NodeKind::Struct { .. }
                    | NodeKind::TypeAlias { .. }
                    | NodeKind::Enum { .. }
                    | NodeKind::Visible { .. }
                    | NodeKind::Import { .. }
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
        NodeKind::Method {
            parameters,
            has_return_type,
            ..
        } => {
            parameters > 0
                && kinds.len() == parameters + usize::from(has_return_type) + 1
                && matches!(kinds.first(), Some(NodeKind::Receiver { .. }))
                && kinds[1..parameters]
                    .iter()
                    .all(|k| matches!(k, NodeKind::Parameter { .. } | NodeKind::Error))
                && (!has_return_type || ty(kinds[parameters]))
                && matches!(kinds.last(), Some(NodeKind::Block | NodeKind::Error))
        }
        NodeKind::Receiver { .. } => kinds.is_empty(),
        NodeKind::Visible { .. } => {
            kinds.len() == 1
                && matches!(
                    kinds[0],
                    NodeKind::Function { .. }
                        | NodeKind::Struct { .. }
                        | NodeKind::TypeAlias { .. }
                        | NodeKind::Enum { .. }
                        | NodeKind::Binding {
                            constant: true,
                            mutable: false,
                            ..
                        }
                        | NodeKind::Error
                )
        }
        NodeKind::Import { .. } => {
            kinds.len() >= 2 && kinds.iter().all(|k| *k == NodeKind::ImportSegment)
        }
        NodeKind::Enum { .. } => kinds
            .iter()
            .all(|k| matches!(k, NodeKind::Variant { .. } | NodeKind::Error)),
        NodeKind::Variant { .. } => kinds.iter().all(|k| ty(*k)),
        NodeKind::VariantPath { .. }
        | NodeKind::Binder { .. }
        | NodeKind::None
        | NodeKind::PatternNone
        | NodeKind::PatternBoolean(_)
        | NodeKind::Wildcard => kinds.is_empty(),
        NodeKind::PatternVariant { arguments, .. } => {
            (arguments || kinds.is_empty())
                && kinds.iter().all(|k| {
                    matches!(
                        k,
                        NodeKind::Binder { .. } | NodeKind::Wildcard | NodeKind::Error
                    )
                })
        }
        NodeKind::Match { .. } => {
            !kinds.is_empty()
                && expr(kinds[0])
                && kinds[1..]
                    .iter()
                    .all(|k| matches!(k, NodeKind::Arm | NodeKind::Error))
        }
        NodeKind::Arm => {
            kinds.len() == 2
                && matches!(
                    kinds[0],
                    NodeKind::PatternVariant { .. }
                        | NodeKind::None
                        | NodeKind::PatternNone
                        | NodeKind::PatternBoolean(_)
                        | NodeKind::Wildcard
                        | NodeKind::Error
                )
                && matches!(kinds[1], NodeKind::Block | NodeKind::Error)
        }
        NodeKind::Struct { .. } => kinds.iter().all(|k| {
            matches!(
                k,
                NodeKind::Field { .. } | NodeKind::Method { .. } | NodeKind::Error
            )
        }),
        NodeKind::Field { .. } | NodeKind::TypeAlias { .. } => kinds.len() == 1 && ty(kinds[0]),
        NodeKind::Projection { .. } | NodeKind::TupleProjection { .. } => {
            kinds.len() == 1 && expr(kinds[0])
        }
        NodeKind::Tuple => !kinds.is_empty() && kinds.iter().all(|k| expr(*k)),
        NodeKind::GenericType { .. } => !kinds.is_empty() && kinds.iter().all(|k| ty(*k)),
        NodeKind::NullableType => kinds.len() == 1 && ty(kinds[0]),
        NodeKind::TupleType => !kinds.is_empty() && kinds.iter().all(|k| ty(*k)),
        NodeKind::Parameter { .. } => {
            (kinds.len() == 1 || kinds.len() == 2)
                && ty(kinds[0])
                && (kinds.len() == 1 || matches!(kinds[1], NodeKind::DefaultValue { .. }))
        }
        NodeKind::DefaultValue { .. } => kinds.len() == 1 && expr(kinds[0]),
        NodeKind::Binding { has_type, .. } => {
            kinds.len() == 1 + usize::from(has_type)
                && (!has_type || ty(kinds[0]))
                && expr(*kinds.last().expect("length checked"))
        }
        NodeKind::Return => kinds.len() <= 1 && kinds.iter().all(|k| expr(*k)),
        NodeKind::Assignment => {
            if kinds.len() != 2 || !expr(kinds[1]) {
                return false;
            }
            let mut target = node.children[0];
            while let Some(n) = arena.get(target) {
                if matches!(
                    n.kind,
                    NodeKind::Projection { .. } | NodeKind::TupleProjection { .. }
                ) && n.children.len() == 1
                {
                    target = n.children[0];
                } else {
                    return n.kind == NodeKind::Name;
                }
            }
            false
        }
        NodeKind::While => {
            kinds.len() == 2
                && expr(kinds[0])
                && matches!(kinds[1], NodeKind::Block | NodeKind::Error)
        }
        NodeKind::For { .. } => {
            kinds.len() == 4
                && matches!(kinds[0], NodeKind::Binder { .. })
                && expr(kinds[1])
                && expr(kinds[2])
                && matches!(kinds[3], NodeKind::Block | NodeKind::Error)
        }
        NodeKind::Loop { .. } => {
            kinds.len() == 1 && matches!(kinds[0], NodeKind::Block | NodeKind::Error)
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
                    | NodeKind::For { .. }
                    | NodeKind::Loop { .. }
                    | NodeKind::Match { .. }
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
        | NodeKind::Try { .. }
        | NodeKind::Exists { .. }
        | NodeKind::NamedArgument { .. }
        | NodeKind::Interpolation => kinds.len() == 1 && expr(kinds[0]),
        NodeKind::Binary(_) => kinds.len() == 2 && kinds.iter().all(|k| expr(*k)),
        NodeKind::Cast { .. } => kinds.len() == 2 && expr(kinds[0]) && ty(kinds[1]),
        NodeKind::Call => {
            !kinds.is_empty()
                && expr(kinds[0])
                && kinds[1..]
                    .iter()
                    .all(|k| expr(*k) || matches!(k, NodeKind::NamedArgument { .. }))
        }
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

// Validate punctuation certificates while admitting the scanner's whitespace
// and nested-comment trivia, without importing the lexer into semantic IR.
fn type_punctuation(mut text: &str, expected: &[&str]) -> bool {
    let mut punctuation = String::new();
    while !text.is_empty() {
        text = text.trim_start();
        if text.is_empty() {
            break;
        }
        if text.starts_with("//") {
            text = text.find(['\n', '\r']).map_or("", |at| &text[at..]);
        } else if text.starts_with("/*") {
            let mut depth = 1usize;
            text = &text[2..];
            while depth > 0 {
                if text.starts_with("/*") {
                    depth += 1;
                    text = &text[2..];
                } else if text.starts_with("*/") {
                    depth -= 1;
                    text = &text[2..];
                } else if let Some(c) = text.chars().next() {
                    text = &text[c.len_utf8()..];
                } else {
                    return false;
                }
            }
        } else {
            let c = text.chars().next().expect("nonempty text");
            if !matches!(c, '<' | '>' | ',' | '?' | '-') || punctuation.len() > 2 {
                return false;
            }
            punctuation.push(c);
            text = &text[c.len_utf8()..];
        }
    }
    expected.contains(&punctuation.as_str())
}
