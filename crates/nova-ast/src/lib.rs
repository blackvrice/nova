//! Source syntax storage (NOVA-016/073), independent of semantic types and LLVM.
//!
//! Child IDs refer to earlier arena entries. Walking, dumping, and dropping a
//! deeply nested recovery tree do not recurse on the host stack.

use nova_source::Span;
use nova_syntax::Symbol;
use std::fmt::{self, Write};

/// Stable within one arena. IDs are not persistent cache keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AstNodeId(usize);

impl AstNodeId {
    pub const fn index(self) -> usize {
        self.0
    }
}

/// P01/P04 syntax only. Child lists contain source-ordered IDs, never semantic IDs.
/// The required child order for each construct is documented below.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeKind {
    Root,
    Error,
    /// Parameters, optional return type, then body.
    Function {
        name: Span,
        parameters: usize,
        has_return_type: bool,
    },
    /// One type child.
    Parameter {
        name: Span,
    },
    NamedType,
    UnitType,
    Block,
    /// Optional type, then initializer.
    Binding {
        name: Span,
        has_type: bool,
        mutable: bool,
        constant: bool,
    },
    /// Target Name, then value. Assignment is a statement, never an expression.
    Assignment,
    /// Zero or one value child.
    Return,
    /// Condition, then block, optional else block/if.
    If,
    /// Condition, then loop body.
    While,
    Break,
    Continue,
    ExpressionStatement,
    Name,
    Integer,
    String,
    Boolean(bool),
    Unit,
    Group,
    Prefix(Symbol),
    /// Left, then right operand.
    Binary(Symbol),
    /// Callee, followed by positional arguments.
    Call,
    InterpolatedString,
    StringText,
    Interpolation,
}

#[derive(Debug, Eq, PartialEq)]
pub struct AstNode {
    pub kind: NodeKind,
    pub span: Span,
    pub children: Vec<AstNodeId>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AstError {
    UnknownNode,
    ChildOutsideParent,
}

impl fmt::Display for AstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnknownNode => "unknown AST node ID",
            Self::ChildOutsideParent => "AST child span is outside its parent",
        })
    }
}

impl std::error::Error for AstError {}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Arena {
    nodes: Vec<AstNode>,
}

impl Arena {
    pub fn insert(
        &mut self,
        kind: NodeKind,
        span: Span,
        children: Vec<AstNodeId>,
    ) -> Result<AstNodeId, AstError> {
        for &id in &children {
            let child = self.get(id).ok_or(AstError::UnknownNode)?;
            if child.span.file() != span.file()
                || child.span.start() < span.start()
                || child.span.end() > span.end()
            {
                return Err(AstError::ChildOutsideParent);
            }
        }
        let id = AstNodeId(self.nodes.len());
        self.nodes.push(AstNode {
            kind,
            span,
            children,
        });
        Ok(id)
    }

    pub fn get(&self, id: AstNodeId) -> Option<&AstNode> {
        self.nodes.get(id.0)
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (AstNodeId, &AstNode)> {
        self.nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (AstNodeId(index), node))
    }

    /// Preorder, respecting the source order of the parent's child list.
    pub fn walk(&self, root: AstNodeId, visitor: &mut impl Visitor) -> Result<(), AstError> {
        let mut pending = vec![root];
        while let Some(id) = pending.pop() {
            let node = self.get(id).ok_or(AstError::UnknownNode)?;
            visitor.visit(id, node);
            pending.extend(node.children.iter().rev().copied());
        }
        Ok(())
    }

    /// Stable arena order and source byte ranges, without addresses or paths.
    pub fn dump(&self) -> String {
        let mut output = String::new();
        for (id, node) in self.iter() {
            // Writing to String is infallible.
            let _ = write!(
                output,
                "{} {:?} {}..{} [",
                id.index(),
                node.kind,
                node.span.start(),
                node.span.end()
            );
            for (index, child) in node.children.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                let _ = write!(output, "{}", child.index());
            }
            output.push_str("]\n");
        }
        output
    }
}

pub trait Visitor {
    fn visit(&mut self, id: AstNodeId, node: &AstNode);
}

#[cfg(test)]
mod tests {
    use super::*;
    use nova_source::FileId;

    fn span(start: usize, end: usize) -> Span {
        Span::new(FileId::from_raw(0), start, end).unwrap()
    }

    #[derive(Default)]
    struct Recorder(Vec<usize>);

    impl Visitor for Recorder {
        fn visit(&mut self, id: AstNodeId, _: &AstNode) {
            self.0.push(id.index());
        }
    }

    #[test]
    fn ids_source_order_and_recovery_snapshot_are_stable() {
        let mut arena = Arena::default();
        let first = arena.insert(NodeKind::Error, span(0, 1), vec![]).unwrap();
        let eof = arena.insert(NodeKind::Error, span(3, 3), vec![]).unwrap();
        let root = arena
            .insert(NodeKind::Root, span(0, 3), vec![first, eof])
            .unwrap();
        let mut visitor = Recorder::default();
        arena.walk(root, &mut visitor).unwrap();
        assert_eq!(visitor.0, [2, 0, 1]);
        assert_eq!(arena.get(first).unwrap().span, span(0, 1));
        assert_eq!(
            arena.dump(),
            "0 Error 0..1 []\n1 Error 3..3 []\n2 Root 0..3 [0,1]\n"
        );
    }

    #[test]
    fn invalid_children_do_not_modify_the_arena() {
        let mut arena = Arena::default();
        let child = arena.insert(NodeKind::Error, span(2, 4), vec![]).unwrap();
        for invalid in [span(0, 3), span(3, 5)] {
            assert_eq!(
                arena.insert(NodeKind::Root, invalid, vec![child]),
                Err(AstError::ChildOutsideParent)
            );
        }
        let other_file = Span::new(FileId::from_raw(1), 0, 5).unwrap();
        assert_eq!(
            arena.insert(NodeKind::Root, other_file, vec![child]),
            Err(AstError::ChildOutsideParent)
        );
        assert_eq!(
            arena.insert(NodeKind::Root, span(0, 5), vec![AstNodeId(99)]),
            Err(AstError::UnknownNode)
        );
        assert_eq!(arena.iter().len(), 1);
        assert_eq!(arena.get(AstNodeId(99)), None);
        assert_eq!(
            arena.walk(AstNodeId(99), &mut Recorder::default()),
            Err(AstError::UnknownNode)
        );
    }

    #[test]
    fn deep_recovery_tree_walk_and_drop_are_iterative() {
        let mut arena = Arena::default();
        let mut last = arena.insert(NodeKind::Error, span(0, 0), vec![]).unwrap();
        for _ in 0..32_000 {
            last = arena
                .insert(NodeKind::Error, span(0, 0), vec![last])
                .unwrap();
        }
        let mut visitor = Recorder::default();
        arena.walk(last, &mut visitor).unwrap();
        assert_eq!(visitor.0.len(), 32_001);
        assert_eq!(visitor.0.first(), Some(&32_000));
        assert_eq!(visitor.0.last(), Some(&0));
    }
}
