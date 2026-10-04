use crate::*;
use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Violation {
    InvalidSource,
    InvalidCallee,
    DuplicateDefinition,
    InvalidBody,
    InvalidLocal,
    InvalidType,
    MissingTerminator,
    InvalidTarget,
    InvalidOperator,
    TypeMismatch,
    InvalidArguments,
    UninitializedRead,
    ReachableUnreachable,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationError {
    pub body: Option<usize>,
    pub block: Option<BlockId>,
    pub source: Option<SourceInfo>,
    pub violation: Violation,
}

/// Independent verification for both lowering output and future MIR transformations.
/// Invariant failures are compiler/API errors, not additional user diagnostics.
pub fn validate(module: &Module) -> Vec<ValidationError> {
    let mut validator = Validator {
        module,
        errors: vec![],
        body: None,
        block: None,
        source: None,
    };
    let mut definitions = BTreeSet::new();
    for (index, source) in module.sources.iter().enumerate() {
        if source.hir.0 != index {
            validator.report(Violation::InvalidSource);
        }
    }
    for callee in &module.callees {
        if !definitions.insert(callee.definition.0) {
            validator.report(Violation::DuplicateDefinition);
        }
        if !value_type(callee.return_type) || callee.parameters.iter().any(|&ty| !value_type(ty)) {
            validator.report(Violation::InvalidType);
        }
        if callee.builtin_print
            && (callee.name != "print"
                || callee.parameters != [Type::String]
                || callee.return_type != Type::Unit)
        {
            validator.report(Violation::InvalidCallee);
        }
    }
    let mut owners = vec![0usize; module.callees.len()];
    for (index, body) in module.bodies.iter().enumerate() {
        validator.body = Some(index);
        validator.block = None;
        validator.source = Some(body.source);
        validator.check_source(body, body.source);
        let Some(callee) = module.callees.get(body.callee.0) else {
            validator.report(Violation::InvalidCallee);
            continue;
        };
        owners[body.callee.0] += 1;
        if callee.builtin_print || body.entry.0 >= body.blocks.len() {
            validator.report(Violation::InvalidBody);
        }
        if body.parameters.len() != callee.parameters.len() {
            validator.report(Violation::InvalidArguments);
        }
        let mut parameters = BTreeSet::new();
        for (position, parameter) in body.parameters.iter().enumerate() {
            if !parameters.insert(parameter.0) {
                validator.report(Violation::InvalidLocal);
            }
            match body.locals.get(parameter.0) {
                Some(local)
                    if local.definition.is_some()
                        && callee.parameters.get(position) == Some(&local.ty) => {}
                _ => validator.report(Violation::InvalidLocal),
            }
        }
        for local in &body.locals {
            validator.source = Some(local.source);
            validator.check_source(body, local.source);
            if !value_type(local.ty) {
                validator.report(Violation::InvalidType);
            }
            if let Some(def) = local.definition {
                if !definitions.insert(def.0) {
                    validator.report(Violation::DuplicateDefinition);
                }
            }
        }
        for (block_index, block) in body.blocks.iter().enumerate() {
            validator.block = Some(BlockId(block_index));
            for statement in &block.statements {
                validator.source = Some(statement.source);
                validator.check_source(body, statement.source);
                let StatementKind::Assign(place, value) = &statement.kind;
                let expected = validator.place_type(body, *place);
                let actual = validator.rvalue_type(body, value);
                validator.same_type(expected, actual);
            }
            let Some(terminator) = &block.terminator else {
                validator.source = Some(body.source);
                validator.report(Violation::MissingTerminator);
                continue;
            };
            validator.source = Some(terminator.source);
            validator.check_source(body, terminator.source);
            for target in successors(&terminator.kind) {
                if target.0 >= body.blocks.len() {
                    validator.report(Violation::InvalidTarget);
                }
            }
            match &terminator.kind {
                TerminatorKind::Branch { condition, .. } => {
                    let ty = validator.operand_type(body, condition);
                    validator.same_type(Some(Type::Bool), ty);
                }
                TerminatorKind::Return(operand) => {
                    let ty = validator.operand_type(body, operand);
                    validator.same_type(Some(callee.return_type), ty);
                }
                TerminatorKind::Call {
                    callee,
                    arguments,
                    destination,
                    ..
                } => {
                    let destination_type = validator.place_type(body, *destination);
                    let argument_types = arguments
                        .iter()
                        .map(|arg| validator.operand_type(body, arg))
                        .collect::<Vec<_>>();
                    if let Some(signature) = module.callees.get(callee.0) {
                        validator.same_type(Some(signature.return_type), destination_type);
                        if arguments.len() != signature.parameters.len() {
                            validator.report(Violation::InvalidArguments);
                        }
                        for (&expected, actual) in signature.parameters.iter().zip(argument_types) {
                            validator.same_type(Some(expected), actual);
                        }
                    } else {
                        validator.report(Violation::InvalidCallee);
                    }
                }
                TerminatorKind::Goto(_) | TerminatorKind::Unreachable => {}
            }
        }
        validator.check_initialization(body);
    }
    validator.body = None;
    validator.block = None;
    validator.source = None;
    for (index, callee) in module.callees.iter().enumerate() {
        if owners[index] != usize::from(!callee.builtin_print) {
            validator.report(Violation::InvalidBody);
        }
    }
    validator.errors
}

fn value_type(ty: Type) -> bool {
    ty.numeric() || matches!(ty, Type::Unit | Type::Bool | Type::Char | Type::String)
}
fn successors(kind: &TerminatorKind) -> Vec<BlockId> {
    match kind {
        TerminatorKind::Goto(target) | TerminatorKind::Call { target, .. } => vec![*target],
        TerminatorKind::Branch {
            then_block,
            else_block,
            ..
        } => vec![*then_block, *else_block],
        TerminatorKind::Return(_) | TerminatorKind::Unreachable => vec![],
    }
}
struct Validator<'a> {
    module: &'a Module,
    errors: Vec<ValidationError>,
    body: Option<usize>,
    block: Option<BlockId>,
    source: Option<SourceInfo>,
}
impl Validator<'_> {
    fn report(&mut self, violation: Violation) {
        self.errors.push(ValidationError {
            body: self.body,
            block: self.block,
            source: self.source,
            violation,
        });
    }
    fn check_source(&mut self, body: &Body, info: SourceInfo) {
        if self.module.sources.get(info.hir.0) != Some(&info)
            || info.span.file() != body.source.span.file()
            || info.span.start() < body.source.span.start()
            || info.span.end() > body.source.span.end()
        {
            self.report(Violation::InvalidSource);
        }
    }
    fn place_type(&mut self, body: &Body, place: Place) -> Option<Type> {
        match body.locals.get(place.0 .0) {
            Some(local) => Some(local.ty),
            None => {
                self.report(Violation::InvalidLocal);
                None
            }
        }
    }
    fn operand_type(&mut self, body: &Body, operand: &Operand) -> Option<Type> {
        match operand {
            Operand::Place(place) => self.place_type(body, *place),
            Operand::Constant(constant) => Some(constant.ty()),
        }
    }
    fn same_type(&mut self, expected: Option<Type>, actual: Option<Type>) {
        if let (Some(expected), Some(actual)) = (expected, actual) {
            if expected != actual {
                self.report(Violation::TypeMismatch);
            }
        }
    }
    fn rvalue_type(&mut self, body: &Body, value: &Rvalue) -> Option<Type> {
        match value {
            Rvalue::Use(operand) => self.operand_type(body, operand),
            Rvalue::NumericConvert(operand, dest) => {
                let source = self.operand_type(body, operand);
                if dest.float().is_none() || !source.is_some_and(|source| source.widens_to(*dest)) {
                    self.report(Violation::TypeMismatch);
                }
                Some(*dest)
            }
            Rvalue::Widen(operand, dest) => {
                let source = self.operand_type(body, operand);
                if !source
                    .and_then(Type::integer)
                    .zip(dest.integer())
                    .is_some_and(|(s, d)| s.widens_to(d))
                {
                    self.report(Violation::TypeMismatch);
                }
                Some(*dest)
            }
            Rvalue::Unary(op, operand) => {
                let ty = self.operand_type(body, operand);
                let valid = match op {
                    Symbol::Plus => ty.is_some_and(Type::numeric),
                    Symbol::Minus => {
                        ty.and_then(Type::float).is_some()
                            || ty.and_then(Type::integer).is_some_and(|k| k.signed())
                    }
                    Symbol::Bang => ty == Some(Type::Bool),
                    _ => {
                        self.report(Violation::InvalidOperator);
                        return None;
                    }
                };
                if !valid {
                    self.report(Violation::TypeMismatch);
                }
                ty
            }
            Rvalue::Binary(op, left, right) => {
                let left = self.operand_type(body, left);
                let right = self.operand_type(body, right);
                let integer = left.and_then(Type::integer).is_some();
                let numeric = left.is_some_and(Type::numeric);
                let comparison = matches!(
                    op,
                    Symbol::Less
                        | Symbol::LessEqual
                        | Symbol::Greater
                        | Symbol::GreaterEqual
                        | Symbol::EqualEqual
                        | Symbol::BangEqual
                );
                let valid = match op {
                    Symbol::Plus | Symbol::Minus | Symbol::Star | Symbol::Slash => numeric,
                    Symbol::Percent => integer,
                    Symbol::Less | Symbol::LessEqual | Symbol::Greater | Symbol::GreaterEqual => {
                        numeric || left == Some(Type::Char)
                    }
                    Symbol::EqualEqual | Symbol::BangEqual => {
                        numeric || matches!(left, Some(Type::Bool | Type::Char))
                    }
                    _ => {
                        self.report(Violation::InvalidOperator);
                        return None;
                    }
                };
                if !valid {
                    self.report(Violation::TypeMismatch);
                }
                self.same_type(left, right);
                if comparison {
                    Some(Type::Bool)
                } else {
                    left
                }
            }
            Rvalue::Interpolate(parts) => {
                for part in parts {
                    let ty = self.operand_type(body, part);
                    if !ty.is_some_and(Type::numeric)
                        && !matches!(ty, Some(Type::Bool | Type::Char | Type::String))
                    {
                        self.report(Violation::TypeMismatch);
                    }
                }
                Some(Type::String)
            }
        }
    }

    fn check_initialization(&mut self, body: &Body) {
        let count = body.blocks.len();
        if body.entry.0 >= count {
            return;
        }
        let mut reachable = vec![false; count];
        let mut pending = vec![body.entry];
        let mut predecessors = vec![vec![]; count];
        while let Some(block) = pending.pop() {
            if reachable[block.0] {
                continue;
            }
            reachable[block.0] = true;
            if let Some(term) = &body.blocks[block.0].terminator {
                for target in successors(&term.kind) {
                    if target.0 < count {
                        predecessors[target.0].push(block);
                        pending.push(target);
                    }
                }
            }
        }
        // Must-initialized analysis: intersect incoming edge facts. Parameters
        // are the entry boundary; a Call destination is defined only on its edge.
        // Bitsets avoid a BTreeSet per (block, local) for large flat inputs.
        let words = body.locals.len().div_ceil(64);
        let top = vec![u64::MAX; words];
        let mut entry = vec![0; words];
        for local in &body.parameters {
            set(&mut entry, *local);
        }
        let mut incoming = vec![top.clone(); count];
        let mut outgoing = vec![top; count];
        let mut queue = (0..count)
            .filter(|&b| reachable[b])
            .collect::<VecDeque<_>>();
        let mut queued = reachable.clone();
        while let Some(index) = queue.pop_front() {
            queued[index] = false;
            let mut state = if index == body.entry.0 {
                entry.clone()
            } else {
                vec![u64::MAX; words]
            };
            for pred in &predecessors[index] {
                for (word, &other) in state.iter_mut().zip(&outgoing[pred.0]) {
                    *word &= other;
                }
            }
            incoming[index] = state.clone();
            for statement in &body.blocks[index].statements {
                let StatementKind::Assign(place, _) = statement.kind;
                set(&mut state, place.0);
            }
            if let Some(Terminator {
                kind: TerminatorKind::Call { destination, .. },
                ..
            }) = &body.blocks[index].terminator
            {
                set(&mut state, destination.0);
            }
            if outgoing[index] != state {
                outgoing[index] = state;
                if let Some(term) = &body.blocks[index].terminator {
                    for target in successors(&term.kind) {
                        if target.0 < count && !queued[target.0] {
                            queue.push_back(target.0);
                            queued[target.0] = true;
                        }
                    }
                }
            }
        }
        for index in 0..count {
            if !reachable[index] {
                continue;
            }
            self.block = Some(BlockId(index));
            let mut state = incoming[index].clone();
            for statement in &body.blocks[index].statements {
                self.source = Some(statement.source);
                let StatementKind::Assign(place, value) = &statement.kind;
                match value {
                    Rvalue::Use(op)
                    | Rvalue::Unary(_, op)
                    | Rvalue::Widen(op, _)
                    | Rvalue::NumericConvert(op, _) => self.read(&state, op),
                    Rvalue::Binary(_, left, right) => {
                        self.read(&state, left);
                        self.read(&state, right);
                    }
                    Rvalue::Interpolate(parts) => {
                        for part in parts {
                            self.read(&state, part);
                        }
                    }
                }
                set(&mut state, place.0);
            }
            if let Some(term) = &body.blocks[index].terminator {
                self.source = Some(term.source);
                match &term.kind {
                    TerminatorKind::Return(op) | TerminatorKind::Branch { condition: op, .. } => {
                        self.read(&state, op)
                    }
                    TerminatorKind::Call { arguments, .. } => {
                        for op in arguments {
                            self.read(&state, op);
                        }
                    }
                    TerminatorKind::Unreachable => self.report(Violation::ReachableUnreachable),
                    TerminatorKind::Goto(_) => {}
                }
            }
        }
    }
    fn read(&mut self, state: &[u64], operand: &Operand) {
        if let Operand::Place(Place(local)) = operand {
            if state
                .get(local.0 / 64)
                .map_or(true, |word| word & (1 << (local.0 % 64)) == 0)
            {
                self.report(Violation::UninitializedRead);
            }
        }
    }
}
fn set(state: &mut [u64], local: LocalId) {
    if let Some(word) = state.get_mut(local.0 / 64) {
        *word |= 1 << (local.0 % 64);
    }
}
