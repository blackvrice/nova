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
    InactivePayload,
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
    if module.enums != module.enums_original
        || module.structs != module.structs_original
        || module.sums != module.sums_original
    {
        validator.report(Violation::InvalidType);
    }
    let mut sum_keys = std::collections::HashSet::new();
    for (&id, key) in &module.sums {
        let names = if key.family == nova_types::SumFamily::Option {
            ["Some", "None"]
        } else {
            ["Success", "Error"]
        };
        let arity = if key.family == nova_types::SumFamily::Option {
            1
        } else {
            2
        };
        let origin_valid = module
            .sum_origins
            .get(&id)
            .is_some_and(|at| module.sources.get(at.hir.0) == Some(at));
        let valid = key.arguments.len() == arity
            && sum_keys.insert(key)
            && origin_valid
            && !module.tuple_ids.contains(&StructId(id.0))
            && module.enums.get(&id).is_some_and(|shape| {
                shape.variants.len() == 2
                    && shape.variants.iter().enumerate().all(|(at, v)| {
                        v.name == names[at]
                            && match key.arguments.get(at) {
                                Some(ty) => {
                                    v.fields.len() == 1
                                        && v.fields[0].ty == *ty
                                        && !v.fields[0].mutable
                                }
                                None => v.fields.is_empty(),
                            }
                    })
            });
        if !valid {
            validator.report(Violation::InvalidType);
        }
    }
    if module.sums.len() > 4096 {
        validator.report(Violation::InvalidType);
    }
    if module.sources.get(module.entry.hir.0) != Some(&module.entry) {
        validator.report(Violation::InvalidSource);
    }
    if let Some((callee, source)) = &module.entry_main {
        if source.span.file() != module.entry.span.file()
            || module.sources.get(source.hir.0) != Some(source)
            || !module.callees.contains(callee)
            || !module.bodies.iter().any(|body| {
                body.source == *source && module.callees.get(body.callee.0) == Some(callee)
            })
        {
            validator.report(Violation::InvalidCallee);
        }
    }
    if module.callees != module.callee_provenance {
        validator.report(Violation::InvalidCallee);
    }
    let body_ids = module
        .bodies
        .iter()
        .map(|b| b.callee.0)
        .collect::<BTreeSet<_>>();
    if module
        .named_bodies
        .keys()
        .chain(module.loop_bodies.keys())
        .chain(module.exists_bodies.keys())
        .any(|id| !body_ids.contains(id))
    {
        validator.report(Violation::InvalidBody);
    }
    let mut definitions = module.structs.keys().map(|s| s.0).collect::<BTreeSet<_>>();
    for (index, source) in module.sources.iter().enumerate() {
        if source.hir.0 != index {
            validator.report(Violation::InvalidSource);
        }
    }
    for callee in &module.callees {
        if !definitions.insert(callee.definition.0) {
            validator.report(Violation::DuplicateDefinition);
        }
        if !value_type(callee.return_type, module)
            || callee.parameters.iter().any(|&ty| !value_type(ty, module))
        {
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
        let actual_entry = !callee.builtin_print
            && callee.name == "main"
            && body.source.span.file() == module.entry.span.file();
        let certified_entry = module
            .entry_main
            .as_ref()
            .is_some_and(|(original, source)| original == callee && *source == body.source);
        if actual_entry != certified_entry {
            validator.report(Violation::InvalidCallee);
        }
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
            if !value_type(local.ty, module) {
                validator.report(Violation::InvalidType);
            }
            if let Some(def) = local.definition {
                if !definitions.insert(def.0) {
                    validator.report(Violation::DuplicateDefinition);
                }
            }
        }
        validator.check_tries(body);
        validator.check_named_calls(body);
        if module
            .loop_bodies
            .get(&body.callee.0)
            .is_some_and(|original| original != body)
        {
            validator.report(Violation::InvalidBody);
        }
        if module
            .exists_bodies
            .get(&body.callee.0)
            .is_some_and(|original| original != body)
        {
            validator.report(Violation::InvalidBody);
        }
        for (block_index, block) in body.blocks.iter().enumerate() {
            validator.block = Some(BlockId(block_index));
            for statement in &block.statements {
                validator.source = Some(statement.source);
                validator.check_source(body, statement.source);
                let StatementKind::Assign(place, value) = &statement.kind;
                let expected = validator.place_type(body, *place);
                if let Rvalue::Update(Operand::Place(root), _, _) = value {
                    if *root != *place
                        || !body
                            .locals
                            .get(root.0 .0)
                            .and_then(|l| l.definition)
                            .is_some_and(|d| module.mutable_definitions.contains(&d.0))
                    {
                        validator.report(Violation::InvalidLocal);
                    }
                }
                let key = statement.source.hir.0;
                match value {
                    Rvalue::Enum(v, _)
                        if module.variant_provenance.get(&key) != Some(v)
                            && !try_statement(module, body, statement) =>
                    {
                        validator.report(Violation::InvalidSource);
                    }
                    Rvalue::EnumPayload(_, v, at)
                        if module.binder_provenance.get(&key) != Some(&(*v, *at))
                            && !try_statement(module, body, statement) =>
                    {
                        validator.report(Violation::InvalidSource);
                    }
                    Rvalue::Aggregate(id, _)
                        if module.constructor_provenance.get(&key) != Some(id) =>
                    {
                        validator.report(Violation::InvalidSource)
                    }
                    Rvalue::Project(_, field)
                        if module.projection_provenance.get(&key) != Some(field) =>
                    {
                        validator.report(Violation::InvalidSource)
                    }
                    Rvalue::Update(Operand::Place(root), path, _)
                        if !module.mutation_provenance.get(&key).is_some_and(
                            |(def, original)| {
                                body.locals
                                    .get(root.0 .0)
                                    .is_some_and(|l| l.definition == Some(*def))
                                    && original == path
                            },
                        ) =>
                    {
                        validator.report(Violation::InvalidSource);
                    }
                    _ => {}
                }
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
                TerminatorKind::Try { snapshot, .. } => {
                    let actual = validator.place_type(body, *snapshot);
                    if !matches!(actual, Some(Type::Enum(e)) if module.sums.get(&e)
                        .is_some_and(|k| k.family == nova_types::SumFamily::Result))
                        || !module
                            .try_certificates
                            .contains_key(&(body.callee.0, terminator.source.hir.0))
                    {
                        validator.report(Violation::InvalidType);
                    }
                }
                TerminatorKind::Match { scrutinee, arms } => {
                    let actual = validator.operand_type(body, scrutinee);
                    let patterns = arms.iter().map(|(p, _)| *p).collect::<Vec<_>>();
                    if !module
                        .match_provenance
                        .get(&terminator.source.hir.0)
                        .is_some_and(|(t, p)| Some(*t) == actual && p == &patterns)
                    {
                        validator.report(Violation::InvalidSource);
                    }
                    let domain = match actual {
                        Some(Type::Bool) => 2,
                        Some(Type::Enum(e)) => module.enums.get(&e).map_or(0, |s| s.variants.len()),
                        _ => 0,
                    };
                    let mut covered = vec![false; domain];
                    for (p, _) in arms {
                        match p {
                            nova_typecheck::MatchPattern::Wildcard => {
                                if covered.iter().all(|b| *b) {
                                    validator.report(Violation::InvalidArguments);
                                }
                                covered.fill(true);
                            }
                            nova_typecheck::MatchPattern::Bool(b) if actual == Some(Type::Bool) => {
                                if covered[usize::from(*b)] {
                                    validator.report(Violation::InvalidArguments);
                                }
                                covered[usize::from(*b)] = true;
                            }
                            nova_typecheck::MatchPattern::Variant(v)
                                if actual == Some(Type::Enum(v.enumeration))
                                    && v.index < domain =>
                            {
                                if covered[v.index] {
                                    validator.report(Violation::InvalidArguments);
                                }
                                covered[v.index] = true;
                            }
                            _ => validator.report(Violation::InvalidType),
                        }
                    }
                    if domain == 0 || !covered.iter().all(|b| *b) {
                        validator.report(Violation::InvalidArguments);
                    }
                }
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
        validator.check_enum_proofs(body);
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

fn value_type(ty: Type, module: &Module) -> bool {
    ty.numeric()
        || matches!(ty, Type::Unit | Type::Bool | Type::Char | Type::String)
        || ty.aggregate().is_some_and(|id| {
            module.aggregate_type(id) == ty
                && module.structs.get(&id).is_some_and(|s| s.layout.is_some())
        })
}
fn successors(kind: &TerminatorKind) -> Vec<BlockId> {
    match kind {
        TerminatorKind::Goto(target) | TerminatorKind::Call { target, .. } => vec![*target],
        TerminatorKind::Match { arms, .. } => arms.iter().map(|(_, b)| *b).collect(),
        TerminatorKind::Try { success, error, .. } => vec![*success, *error],
        TerminatorKind::Branch {
            then_block,
            else_block,
            ..
        } => vec![*then_block, *else_block],
        TerminatorKind::Return(_) | TerminatorKind::Unreachable => vec![],
    }
}
fn try_statement(module: &Module, body: &Body, statement: &Statement) -> bool {
    module
        .try_certificates
        .get(&(body.callee.0, statement.source.hir.0))
        .is_some_and(|c| {
            c.success_read == *statement || c.error_body.statements.contains(statement)
        })
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
            || (self
                .module
                .default_sources
                .get(&(body.callee.0, info.hir.0))
                != Some(&info)
                && (info.span.file() != body.source.span.file()
                    || info.span.start() < body.source.span.start()
                    || info.span.end() > body.source.span.end()))
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
            Operand::Constant(constant) => {
                let mut work = vec![(constant, constant.ty(), 0usize)];
                let mut count = 0;
                while let Some((v, expected, depth)) = work.pop() {
                    count += 1;
                    if depth > 128 || count > 65_537 {
                        self.report(Violation::InvalidType);
                        break;
                    }
                    if v.ty() != expected {
                        self.report(Violation::TypeMismatch);
                    }
                    if let Constant::Enum(variant, fields) = v {
                        if let Some(shape) = self
                            .module
                            .enums
                            .get(&variant.enumeration)
                            .and_then(|s| s.variants.get(variant.index))
                        {
                            if fields.len() != shape.fields.len() {
                                self.report(Violation::InvalidArguments);
                            }
                            work.extend(
                                fields
                                    .iter()
                                    .zip(&shape.fields)
                                    .map(|(v, f)| (v, f.ty, depth + 1)),
                            );
                        } else {
                            self.report(Violation::InvalidType);
                        }
                    }
                    if let Constant::Struct(id, fields) | Constant::Tuple(id, fields) = v {
                        if v.ty() != self.module.aggregate_type(*id) {
                            self.report(Violation::InvalidType);
                        }
                        if let Some(shape) = self.module.structs.get(id) {
                            if fields.len() != shape.fields.len() {
                                self.report(Violation::InvalidArguments);
                            }
                            work.extend(
                                fields
                                    .iter()
                                    .zip(&shape.fields)
                                    .map(|(v, f)| (v, f.ty, depth + 1)),
                            );
                        } else {
                            self.report(Violation::InvalidType);
                        }
                    }
                }
                Some(constant.ty())
            }
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
            Rvalue::Enum(v, fields) => {
                let expected = self
                    .module
                    .enums
                    .get(&v.enumeration)
                    .and_then(|s| s.variants.get(v.index))
                    .map(|v| v.fields.iter().map(|f| f.ty).collect::<Vec<_>>());
                if let Some(expected) = expected {
                    if fields.len() != expected.len() {
                        self.report(Violation::InvalidArguments);
                    }
                    for (&ty, op) in expected.iter().zip(fields) {
                        let actual = self.operand_type(body, op);
                        self.same_type(Some(ty), actual);
                    }
                } else {
                    self.report(Violation::InvalidType);
                }
                Some(Type::Enum(v.enumeration))
            }
            Rvalue::EnumPayload(receiver, v, at) => {
                let actual = self.operand_type(body, receiver);
                self.same_type(Some(Type::Enum(v.enumeration)), actual);
                let field = self
                    .module
                    .enums
                    .get(&v.enumeration)
                    .and_then(|s| s.variants.get(v.index))
                    .and_then(|v| v.fields.get(*at));
                if field.is_none() {
                    self.report(Violation::InvalidType);
                }
                field.map(|f| f.ty)
            }
            Rvalue::Use(operand) => self.operand_type(body, operand),
            Rvalue::Aggregate(id, fields) => {
                if matches!(self.module.aggregate_type(*id), Type::Enum(_)) {
                    self.report(Violation::InvalidType);
                }
                if let Some(shape) = self.module.structs.get(id) {
                    let expected = shape.fields.iter().map(|f| f.ty).collect::<Vec<_>>();
                    if expected.len() != fields.len() {
                        self.report(Violation::InvalidArguments);
                    }
                    for (&ty, field) in expected.iter().zip(fields) {
                        let actual = self.operand_type(body, field);
                        self.same_type(Some(ty), actual);
                    }
                } else {
                    self.report(Violation::InvalidType);
                }
                Some(self.module.aggregate_type(*id))
            }
            Rvalue::Project(receiver, field) => {
                if matches!(self.module.aggregate_type(field.structure), Type::Enum(_)) {
                    self.report(Violation::InvalidType);
                }
                let recv = self.operand_type(body, receiver);
                self.same_type(Some(self.module.aggregate_type(field.structure)), recv);
                match self
                    .module
                    .structs
                    .get(&field.structure)
                    .and_then(|s| s.fields.get(field.index))
                {
                    Some(f) => Some(f.ty),
                    None => {
                        self.report(Violation::InvalidType);
                        None
                    }
                }
            }
            Rvalue::Update(receiver, path, value) => {
                let root = self.operand_type(body, receiver);
                let mut ty = root;
                if path.is_empty() || path.len() > self.module.sources.len() {
                    self.report(Violation::InvalidArguments);
                }
                for field in path {
                    if matches!(self.module.aggregate_type(field.structure), Type::Enum(_)) {
                        self.report(Violation::InvalidType);
                    }
                    self.same_type(Some(self.module.aggregate_type(field.structure)), ty);
                    ty = match self
                        .module
                        .structs
                        .get(&field.structure)
                        .and_then(|s| s.fields.get(field.index))
                    {
                        Some(f) => {
                            let mutable = f.mutable;
                            let ty = f.ty;
                            if !mutable {
                                self.report(Violation::InvalidLocal);
                            }
                            Some(ty)
                        }
                        None => {
                            self.report(Violation::InvalidType);
                            None
                        }
                    };
                }
                let actual = self.operand_type(body, value);
                self.same_type(ty, actual);
                root
            }
            Rvalue::CheckedCast(operand, dest) => {
                if !dest.numeric() || !self.operand_type(body, operand).is_some_and(Type::numeric) {
                    self.report(Violation::TypeMismatch);
                }
                Some(*dest)
            }
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

    fn check_named_calls(&mut self, body: &Body) {
        let module = self.module;
        if let Some(original) = module.named_bodies.get(&body.callee.0) {
            if original != body {
                self.report(Violation::InvalidSource);
            }
        }
        let mut writes = vec![0usize; body.locals.len()];
        for block in &body.blocks {
            for statement in &block.statements {
                let StatementKind::Assign(place, _) = statement.kind;
                if let Some(n) = writes.get_mut(place.0 .0) {
                    *n += 1;
                }
            }
            if let Some(Terminator {
                kind: TerminatorKind::Call { destination, .. },
                ..
            }) = &block.terminator
            {
                if let Some(n) = writes.get_mut(destination.0 .0) {
                    *n += 1;
                }
            }
        }
        for (_, c) in module
            .named_calls
            .range((body.callee.0, 0)..=(body.callee.0, usize::MAX))
        {
            self.source = Some(c.source);
            self.block = Some(c.block);
            let TerminatorKind::Call {
                callee, arguments, ..
            } = &c.call.kind
            else {
                self.report(Violation::InvalidArguments);
                continue;
            };
            let Some(signature) = module.callees.get(callee.0) else {
                self.report(Violation::InvalidCallee);
                continue;
            };
            if signature.definition != c.mapping.callee
                || signature.builtin_print
                || module.sources.get(c.source.hir.0) != Some(&c.source)
                || body
                    .blocks
                    .get(c.block.0)
                    .and_then(|b| b.terminator.as_ref())
                    != Some(&c.call)
                || c.snapshots.len() + c.defaults.len() != arguments.len()
                || c.mapping.parameters.len() != c.snapshots.len()
                || c.mapping.arguments.len() != c.snapshots.len()
                || c.mapping.defaults.len() != c.defaults.len()
                || signature.parameters.len() != arguments.len()
            {
                self.report(Violation::InvalidArguments);
                continue;
            }
            let mut seen = vec![false; arguments.len()];
            for (source, ((block, at, snapshot), argument)) in
                c.snapshots.iter().zip(&c.mapping.arguments).enumerate()
            {
                let parameter = c.mapping.parameters[source];
                let StatementKind::Assign(place, Rvalue::Use(_)) = &snapshot.kind else {
                    self.report(Violation::InvalidArguments);
                    continue;
                };
                let valid = seen.get_mut(parameter).is_some_and(|s| {
                    let valid = !*s;
                    *s = true;
                    valid
                }) && module.sources.get(argument.0) == Some(&snapshot.source)
                    && snapshot.source.span.file() == c.source.span.file()
                    && snapshot.source.span.start() >= c.source.span.start()
                    && snapshot.source.span.end() <= c.source.span.end()
                    && body.blocks.get(block.0).and_then(|b| b.statements.get(*at))
                        == Some(snapshot)
                    && writes.get(place.0 .0) == Some(&1)
                    && arguments.get(parameter) == Some(&Operand::Place(*place))
                    && body.locals.get(place.0 .0).is_some_and(|l| {
                        l.definition.is_none()
                            && l.source == snapshot.source
                            && signature.parameters.get(parameter) == Some(&l.ty)
                    });
                if !valid {
                    self.report(Violation::InvalidArguments);
                }
            }
            let mut previous = None;
            for (omitted, (default, block, at, snapshot)) in
                c.mapping.defaults.iter().zip(&c.defaults)
            {
                let parameter = omitted.index;
                let StatementKind::Assign(place, Rvalue::Use(Operand::Constant(constant))) =
                    &snapshot.kind
                else {
                    self.report(Violation::InvalidArguments);
                    continue;
                };
                let nova_typecheck::ConstEvaluation::Value { value, .. } = &default.evaluation
                else {
                    self.report(Violation::InvalidArguments);
                    continue;
                };
                let valid = seen.get_mut(parameter).is_some_and(|s| {
                    let new = !*s;
                    *s = true;
                    new
                }) && previous.map_or(true, |(index, statement)| {
                    index < parameter && statement < *at
                }) && default.callee == signature.definition
                    && default.argument == *omitted
                    && Constant::from_const(value) == *constant
                    && *block == c.block
                    && module.sources.get(omitted.initializer.0) == Some(&snapshot.source)
                    && module.sources.get(omitted.parameter.0).is_some_and(|p| {
                        p.span.file() == snapshot.source.span.file()
                            && p.span.start() <= snapshot.source.span.start()
                            && snapshot.source.span.end() <= p.span.end()
                    })
                    && body.blocks.get(block.0).and_then(|b| b.statements.get(*at))
                        == Some(snapshot)
                    && writes.get(place.0 .0) == Some(&1)
                    && arguments.get(parameter) == Some(&Operand::Place(*place))
                    && body.locals.get(place.0 .0).is_some_and(|l| {
                        l.definition.is_none()
                            && l.source == snapshot.source
                            && signature.parameters.get(parameter) == Some(&l.ty)
                    });
                previous = Some((parameter, *at));
                if !valid {
                    self.report(Violation::InvalidArguments);
                }
            }
            if seen.iter().any(|filled| !filled) {
                self.report(Violation::InvalidArguments);
            }
        }
    }

    fn check_tries(&mut self, body: &Body) {
        let module = self.module;
        if let Some(control) = module.try_controls.get(&body.callee.0) {
            if control.source != body.source
                || control.entry != body.entry
                || control.terminators.len() != body.blocks.len()
                || control
                    .terminators
                    .iter()
                    .zip(&body.blocks)
                    .any(|(t, b)| t != &b.terminator)
            {
                self.report(Violation::InvalidSource);
            }
        }
        // Count static definitions once: a loop may execute one definition repeatedly.
        let mut writes = vec![0usize; body.locals.len()];
        for block in &body.blocks {
            for statement in &block.statements {
                let StatementKind::Assign(p, _) = &statement.kind;
                if let Some(n) = writes.get_mut(p.0 .0) {
                    *n += 1;
                }
            }
            if let Some(Terminator {
                kind: TerminatorKind::Call { destination, .. },
                ..
            }) = &block.terminator
            {
                if let Some(n) = writes.get_mut(destination.0 .0) {
                    *n += 1;
                }
            }
        }
        for (_, c) in module
            .try_certificates
            .range((body.callee.0, 0)..=(body.callee.0, usize::MAX))
        {
            self.source = Some(c.source);
            self.block = Some(c.dispatch);
            let StatementKind::Assign(snapshot, Rvalue::Use(_)) = &c.snapshot.kind else {
                self.report(Violation::InvalidSource);
                continue;
            };
            let valid_family = module
                .sums
                .get(&c.source_result)
                .zip(module.sums.get(&c.destination_result))
                .is_some_and(|(from, to)| {
                    from.family == nova_types::SumFamily::Result
                        && to.family == from.family
                        && from.arguments.len() == 2
                        && to.arguments.len() == 2
                        && from.arguments[1] == to.arguments[1]
                });
            let dispatch = body.blocks.get(c.dispatch.0);
            let StatementKind::Assign(success_value, _) = &c.success_read.kind;
            let expected = Terminator {
                source: c.source,
                kind: TerminatorKind::Try {
                    snapshot: *snapshot,
                    success: c.success,
                    error: c.error,
                },
            };
            if !valid_family
                || !module.try_controls.contains_key(&body.callee.0)
                || module.sources.get(c.source.hir.0) != Some(&c.source)
                || c.keyword.file() != c.source.span.file()
                || c.keyword.start() != c.source.span.start()
                || c.keyword.end() != c.keyword.start() + 3
                || module.callees.get(body.callee.0).map(|f| f.return_type)
                    != Some(Type::Enum(c.destination_result))
                || body
                    .locals
                    .get(snapshot.0 .0)
                    .map(|l| (l.ty, l.definition, l.source))
                    != Some((Type::Enum(c.source_result), None, c.source))
                || writes.get(snapshot.0 .0) != Some(&1)
                || writes.get(success_value.0 .0) != Some(&1)
                || dispatch.and_then(|b| b.statements.last()) != Some(&c.snapshot)
                || dispatch.and_then(|b| b.terminator.as_ref()) != Some(&expected)
                || body
                    .blocks
                    .get(c.success.0)
                    .and_then(|b| b.statements.first())
                    != Some(&c.success_read)
                || body.blocks.get(c.error.0) != Some(&c.error_body)
                || c.success == c.error
            {
                self.report(Violation::InvalidSource);
            }
        }
    }
    fn check_enum_proofs(&mut self, body: &Body) {
        use std::collections::BTreeMap;
        type Facts = BTreeMap<usize, nova_types::VariantId>;
        if body.entry.0 >= body.blocks.len() {
            return;
        }
        let mut incoming: Vec<Option<Facts>> = vec![None; body.blocks.len()];
        incoming[body.entry.0] = Some(Facts::new());
        let mut queue = VecDeque::from([body.entry]);
        while let Some(block) = queue.pop_front() {
            let mut facts = incoming[block.0].clone().expect("queued block");
            for statement in &body.blocks[block.0].statements {
                let StatementKind::Assign(place, _) = &statement.kind;
                facts.remove(&place.0 .0);
            }
            let Some(term) = &body.blocks[block.0].terminator else {
                continue;
            };
            if let TerminatorKind::Call { destination, .. } = &term.kind {
                facts.remove(&destination.0 .0);
            }
            for target in successors(&term.kind) {
                if target.0 >= incoming.len() {
                    continue;
                }
                let mut edge = facts.clone();
                if let TerminatorKind::Match {
                    scrutinee: Operand::Place(receiver),
                    arms,
                } = &term.kind
                {
                    // Merge facts independently for every dispatch edge, including
                    // multiple cases redirected to one block by a transformation.
                    let variants = arms
                        .iter()
                        .filter(|(_, b)| *b == target)
                        .map(|(p, _)| match p {
                            nova_typecheck::MatchPattern::Variant(v) => Some(*v),
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    edge.remove(&receiver.0 .0);
                    if let Some(Some(v)) = variants.first() {
                        if variants.iter().all(|other| *other == Some(*v)) {
                            edge.insert(receiver.0 .0, *v);
                        }
                    }
                }
                if let TerminatorKind::Try {
                    snapshot,
                    success,
                    error,
                } = &term.kind
                {
                    edge.remove(&snapshot.0 .0);
                    if success != error {
                        if let Some(Local {
                            ty: Type::Enum(e), ..
                        }) = body.locals.get(snapshot.0 .0)
                        {
                            edge.insert(
                                snapshot.0 .0,
                                nova_types::VariantId {
                                    enumeration: *e,
                                    index: usize::from(target == *error),
                                },
                            );
                        }
                    }
                }
                let next = if target == body.entry {
                    Facts::new()
                } else if let Some(old) = &incoming[target.0] {
                    old.iter()
                        .filter(|(local, v)| edge.get(local) == Some(v))
                        .map(|(&l, &v)| (l, v))
                        .collect()
                } else {
                    edge
                };
                if incoming[target.0].as_ref() != Some(&next) {
                    incoming[target.0] = Some(next);
                    queue.push_back(target);
                }
            }
        }
        for (index, block) in body.blocks.iter().enumerate() {
            let Some(mut facts) = incoming[index].clone() else {
                continue;
            };
            self.block = Some(BlockId(index));
            for statement in &block.statements {
                self.source = Some(statement.source);
                let StatementKind::Assign(place, value) = &statement.kind;
                if let Rvalue::EnumPayload(receiver, variant, _) = value {
                    let proven = match receiver {
                        Operand::Place(p) => facts.get(&p.0 .0) == Some(variant),
                        Operand::Constant(Constant::Enum(v, _)) => v == variant,
                        _ => false,
                    };
                    if !proven {
                        self.report(Violation::InactivePayload);
                    }
                }
                facts.remove(&place.0 .0);
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
                    Rvalue::EnumPayload(op, _, _)
                    | Rvalue::Use(op)
                    | Rvalue::Project(op, _)
                    | Rvalue::Unary(_, op)
                    | Rvalue::Widen(op, _)
                    | Rvalue::CheckedCast(op, _)
                    | Rvalue::NumericConvert(op, _) => self.read(&state, op),
                    Rvalue::Binary(_, left, right) => {
                        self.read(&state, left);
                        self.read(&state, right);
                    }
                    Rvalue::Update(receiver, _, value) => {
                        self.read(&state, receiver);
                        self.read(&state, value);
                    }
                    Rvalue::Enum(_, parts)
                    | Rvalue::Aggregate(_, parts)
                    | Rvalue::Interpolate(parts) => {
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
                    TerminatorKind::Try { snapshot, .. } => {
                        self.read(&state, &Operand::Place(*snapshot))
                    }
                    TerminatorKind::Match { scrutinee: op, .. }
                    | TerminatorKind::Return(op)
                    | TerminatorKind::Branch { condition: op, .. } => self.read(&state, op),
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
