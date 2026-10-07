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
    Parameter {
        name: SymbolId,
        name_span: Span,
    },
    Struct {
        name: SymbolId,
        name_span: Span,
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
    pub fn owner(&self, id: HirId) -> Option<usize> {
        let node = self.node(id)?;
        self.units
            .iter()
            .position(|unit| unit.file == node.span.file())
    }
    pub fn visibility(&self, id: HirId) -> Visibility {
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
                    | HirKind::Enum { name, .. }
                    | HirKind::Variant { name, .. }
                    | HirKind::Binder { name, .. }
                    | HirKind::Field { name, .. }
                    | HirKind::Projection { name, .. }
                    | HirKind::Parameter { name, .. }
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
        NodeKind::Visible { .. } => {
            kinds.len() == 1
                && matches!(
                    kinds[0],
                    NodeKind::Function { .. }
                        | NodeKind::Struct { .. }
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
        NodeKind::Struct { .. } => kinds
            .iter()
            .all(|k| matches!(k, NodeKind::Field { .. } | NodeKind::Error)),
        NodeKind::Field { .. } => kinds.len() == 1 && ty(kinds[0]),
        NodeKind::Projection { .. } | NodeKind::TupleProjection { .. } => {
            kinds.len() == 1 && expr(kinds[0])
        }
        NodeKind::Tuple => !kinds.is_empty() && kinds.iter().all(|k| expr(*k)),
        NodeKind::GenericType { .. } => !kinds.is_empty() && kinds.iter().all(|k| ty(*k)),
        NodeKind::NullableType => kinds.len() == 1 && ty(kinds[0]),
        NodeKind::TupleType => !kinds.is_empty() && kinds.iter().all(|k| ty(*k)),
        NodeKind::Parameter { .. } => kinds.len() == 1 && ty(kinds[0]),
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
            if !matches!(c, '<' | '>' | ',' | '?') || punctuation.len() > 2 {
                return false;
            }
            punctuation.push(c);
            text = &text[c.len_utf8()..];
        }
    }
    expected.contains(&punctuation.as_str())
}
