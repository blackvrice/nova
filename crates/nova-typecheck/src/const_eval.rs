//! P05 bounded evaluation after ordinary type checking, before MIR.
use crate::Checked;
use nova_hir::{HirId, HirKind, Module};
use nova_resolve::{Resolution, Resolved};
use nova_syntax::Symbol;
use nova_types::{ConstValue, FloatComparison, FloatOp, IntegerOp};

pub const CONST_NODE_LIMIT: usize = 10_000;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstEvaluation {
    NotConstant,
    Pending,
    /// An upstream syntax/name/type failure; do not emit a derived const error.
    Invalid,
    Value {
        value: ConstValue,
        nodes: usize,
    },
    Failed {
        code: u16,
        nodes: usize,
    },
}
pub(crate) struct Failure {
    pub code: u16,
    pub node: HirId,
    pub nodes: usize,
    pub message: &'static str,
}

enum Work {
    Evaluate(HirId),
    Convert(HirId),
    Cast(HirId),
    Unary(HirId, Symbol),
    Binary(HirId, Symbol),
    Logical(HirId, Symbol),
}

pub(crate) fn evaluate(
    module: &Module,
    resolved: &Resolved,
    checked: &Checked,
    root: HirId,
) -> Result<(ConstValue, usize), Failure> {
    // Permission checks include skipped logical RHS nodes. Counting this tree
    // bounds the subsequent stack machine as well; const references are cached.
    let mut pending = vec![root];
    let mut nodes = 0;
    while let Some(id) = pending.pop() {
        nodes += 1;
        if nodes > CONST_NODE_LIMIT {
            return Err(Failure {
                code: 3202,
                node: id,
                nodes,
                message: "const initializer node budget exceeded",
            });
        }
        let node = &module.nodes()[id.0];
        let allowed = match node.kind {
            HirKind::Integer(_)
            | HirKind::Float(_)
            | HirKind::Character(_)
            | HirKind::Boolean(_)
            | HirKind::String(_)
            | HirKind::Unit
            | HirKind::Cast { .. }
            | HirKind::Group
            | HirKind::Prefix(Symbol::Plus | Symbol::Minus | Symbol::Bang)
            | HirKind::Binary(
                Symbol::Plus
                | Symbol::Minus
                | Symbol::Star
                | Symbol::Slash
                | Symbol::Percent
                | Symbol::EqualEqual
                | Symbol::BangEqual
                | Symbol::Less
                | Symbol::LessEqual
                | Symbol::Greater
                | Symbol::GreaterEqual
                | Symbol::AndAnd
                | Symbol::OrOr,
            ) => true,
            HirKind::Name(_) => matches!(resolved.references[id.0],
                Some(Resolution::Definition(def)) if resolved.definitions[def.0].constant
                    && matches!(checked.const_values[def.0], ConstEvaluation::Value { .. })),
            _ => false,
        };
        if !allowed {
            return Err(Failure {
                code: 3201,
                node: id,
                nodes,
                message: "expression is not permitted in a P05 const initializer",
            });
        }
        if matches!(node.kind, HirKind::Cast { .. }) {
            pending.push(node.children[0]);
        } else {
            pending.extend(node.children.iter().rev().copied());
        }
    }

    let fail = |node, message| Failure {
        code: 3201,
        node,
        nodes,
        message,
    };
    let mut work = vec![Work::Evaluate(root)];
    let mut values = vec![];
    while let Some(task) = work.pop() {
        match task {
            Work::Cast(id) => {
                let value: ConstValue = values.pop().expect("cast operand");
                let dest = checked
                    .types
                    .get(checked.type_table[id.0])
                    .expect("cast target");
                values.push(
                    value
                        .checked_cast(dest)
                        .map_err(|_| fail(id, "numeric cast out of range"))?,
                );
            }
            Work::Convert(id) => {
                if let Some(dest) = checked.coercions[id.0] {
                    let value: ConstValue = values.pop().expect("converted operand");
                    let dest = checked.types.get(dest).expect("numeric coercion");
                    values.push(value.widen(dest).expect("checked lossless conversion"));
                }
            }
            Work::Evaluate(id) => {
                work.push(Work::Convert(id));
                let node = &module.nodes()[id.0];
                if let Some(value) = checked.float_literals[id.0] {
                    values.push(ConstValue::Float(value));
                    continue;
                }
                // Also handles the directly checked -2147483648 magnitude.
                if let Some(value) = checked.integer_literals[id.0] {
                    values.push(ConstValue::from_integer(value));
                    continue;
                }
                match &node.kind {
                    HirKind::String(text) => values.push(ConstValue::String(text.clone())),
                    HirKind::Character(value) => values.push(ConstValue::Char(*value)),
                    HirKind::Boolean(value) => values.push(ConstValue::Bool(*value)),
                    HirKind::Unit => values.push(ConstValue::Unit),
                    HirKind::Name(_) => {
                        let Some(Resolution::Definition(def)) = resolved.references[id.0] else {
                            unreachable!("permission checked const name")
                        };
                        let ConstEvaluation::Value { value, .. } = &checked.const_values[def.0]
                        else {
                            unreachable!("permission checked cached const value")
                        };
                        values.push(value.clone());
                    }
                    HirKind::Cast { .. } => {
                        work.push(Work::Cast(id));
                        work.push(Work::Evaluate(node.children[0]));
                    }
                    HirKind::Group => work.push(Work::Evaluate(node.children[0])),
                    HirKind::Prefix(op) => {
                        work.push(Work::Unary(id, *op));
                        work.push(Work::Evaluate(node.children[0]));
                    }
                    HirKind::Binary(op @ (Symbol::AndAnd | Symbol::OrOr)) => {
                        work.push(Work::Logical(id, *op));
                        work.push(Work::Evaluate(node.children[0]));
                    }
                    HirKind::Binary(op) => {
                        work.push(Work::Binary(id, *op));
                        work.push(Work::Evaluate(node.children[1]));
                        work.push(Work::Evaluate(node.children[0]));
                    }
                    _ => unreachable!("permission checked typed const node"),
                }
            }
            Work::Logical(id, op) => {
                let ConstValue::Bool(left) = values.pop().expect("logical left value") else {
                    unreachable!("typed logical operand")
                };
                if (op == Symbol::AndAnd && !left) || (op == Symbol::OrOr && left) {
                    values.push(ConstValue::Bool(left));
                } else {
                    // The right Bool becomes this expression's result directly.
                    work.push(Work::Evaluate(module.nodes()[id.0].children[1]));
                }
            }
            Work::Unary(id, op) => {
                let value = values.pop().expect("unary operand");
                let value = if let Some(integer) = value.integer() {
                    ConstValue::from_integer(match op {
                        Symbol::Plus => integer,
                        Symbol::Minus => integer
                            .negated()
                            .ok_or_else(|| fail(id, "integer overflow in const unary minus"))?,
                        _ => unreachable!("typed integer unary operation"),
                    })
                } else if let ConstValue::Float(value) = value {
                    ConstValue::Float(if op == Symbol::Minus {
                        value.negated()
                    } else {
                        value
                    })
                } else if let (Symbol::Bang, ConstValue::Bool(value)) = (op, value) {
                    ConstValue::Bool(!value)
                } else {
                    unreachable!("typed permitted unary operation")
                };
                values.push(value);
            }
            Work::Binary(id, op) => {
                let right = values.pop().expect("binary right operand");
                let left = values.pop().expect("binary left operand");
                let value = if let (Some(left), Some(right)) = (left.integer(), right.integer()) {
                    match op {
                        Symbol::EqualEqual => ConstValue::Bool(left.value() == right.value()),
                        Symbol::BangEqual => ConstValue::Bool(left.value() != right.value()),
                        Symbol::Less => ConstValue::Bool(left.value() < right.value()),
                        Symbol::LessEqual => ConstValue::Bool(left.value() <= right.value()),
                        Symbol::Greater => ConstValue::Bool(left.value() > right.value()),
                        Symbol::GreaterEqual => ConstValue::Bool(left.value() >= right.value()),
                        _ => {
                            if matches!(op, Symbol::Slash | Symbol::Percent) && right.value() == 0 {
                                return Err(fail(id, "division or remainder by zero in const"));
                            }
                            let op = match op {
                                Symbol::Plus => IntegerOp::Add,
                                Symbol::Minus => IntegerOp::Subtract,
                                Symbol::Star => IntegerOp::Multiply,
                                Symbol::Slash => IntegerOp::Divide,
                                Symbol::Percent => IntegerOp::Remainder,
                                _ => unreachable!("typed integer arithmetic"),
                            };
                            ConstValue::from_integer(
                                left.arithmetic(op, right).ok_or_else(|| {
                                    fail(id, "integer overflow in const operation")
                                })?,
                            )
                        }
                    }
                } else {
                    match (left, right) {
                        (ConstValue::Float(left), ConstValue::Float(right)) => {
                            let comparison = match op {
                                Symbol::EqualEqual => Some(FloatComparison::Equal),
                                Symbol::BangEqual => Some(FloatComparison::NotEqual),
                                Symbol::Less => Some(FloatComparison::Less),
                                Symbol::LessEqual => Some(FloatComparison::LessEqual),
                                Symbol::Greater => Some(FloatComparison::Greater),
                                Symbol::GreaterEqual => Some(FloatComparison::GreaterEqual),
                                _ => None,
                            };
                            if let Some(op) = comparison {
                                ConstValue::Bool(
                                    left.compare(op, right)
                                        .expect("controlled typed float comparison"),
                                )
                            } else {
                                let op = match op {
                                    Symbol::Plus => FloatOp::Add,
                                    Symbol::Minus => FloatOp::Subtract,
                                    Symbol::Star => FloatOp::Multiply,
                                    Symbol::Slash => FloatOp::Divide,
                                    _ => unreachable!("typed float arithmetic"),
                                };
                                ConstValue::Float(
                                    left.arithmetic(op, right)
                                        .expect("controlled typed float arithmetic"),
                                )
                            }
                        }
                        (ConstValue::Char(left), ConstValue::Char(right)) => {
                            ConstValue::Bool(match op {
                                Symbol::EqualEqual => left == right,
                                Symbol::BangEqual => left != right,
                                Symbol::Less => left < right,
                                Symbol::LessEqual => left <= right,
                                Symbol::Greater => left > right,
                                Symbol::GreaterEqual => left >= right,
                                _ => unreachable!("typed char comparison"),
                            })
                        }
                        (ConstValue::Bool(left), ConstValue::Bool(right)) => {
                            ConstValue::Bool(match op {
                                Symbol::EqualEqual => left == right,
                                Symbol::BangEqual => left != right,
                                _ => unreachable!("logical operations use separate work items"),
                            })
                        }
                        _ => unreachable!("typed permitted binary operands"),
                    }
                };
                values.push(value);
            }
        }
    }
    debug_assert_eq!(values.len(), 1);
    Ok((values.pop().expect("evaluated const root"), nodes))
}
