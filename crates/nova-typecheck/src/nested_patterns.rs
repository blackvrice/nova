//! P24 flat typed pattern arena and bounded, memoized constructor usefulness.
//! No value-domain Cartesian expansion and no recursively owned pattern trees.
use crate::Checker;
use nova_hir::{HirId, HirKind};
use nova_types::{EnumRegistry, StructId, StructRegistry, SumFamily, SumRegistry, Type, VariantId};
use std::collections::{BTreeSet, HashMap};
use std::hash::{Hash, Hasher};

pub const PATTERN_NODE_LIMIT: usize = 10_000;
pub const MATRIX_TASK_LIMIT: usize = 100_000;
pub const MATRIX_CELL_LIMIT: usize = 1_000_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecursivePatternKind {
    Wildcard,
    Bool(bool),
    Unit,
    Tuple(StructId),
    Variant(VariantId),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecursivePattern {
    pub ty: Type,
    pub kind: RecursivePatternKind,
    pub children: Vec<HirId>,
}

impl Checker<'_> {
    pub(super) fn is_nested_match(&self, id: HirId) -> bool {
        let node = &self.module.nodes()[id.0];
        if matches!(
            self.ty(self.result.type_table[node.children[0].0]),
            Type::Tuple(_) | Type::Unit
        ) {
            return true;
        }
        self.module.nodes()[id.0].children[1..].iter().any(|&arm| {
            if self.module.nodes()[arm.0].kind != HirKind::Arm {
                return false;
            }
            let pattern = &self.module.nodes()[self.module.nodes()[arm.0].children[0].0];
            match pattern.kind {
                HirKind::Wildcard | HirKind::PatternBoolean(_) | HirKind::PatternNone => false,
                HirKind::PatternVariant { .. } => pattern.children.iter().any(|c| {
                    !matches!(
                        self.module.nodes()[c.0].kind,
                        HirKind::Binder { .. } | HirKind::Wildcard
                    )
                }),
                _ => true,
            }
        })
    }

    pub(super) fn prepare_nested_match(&mut self, id: HirId) {
        self.result.nested_matches.insert(id.0);
        let node = &self.module.nodes()[id.0];
        let HirKind::Match { keyword } = node.kind else {
            return;
        };
        let ty = self.ty(self.result.type_table[node.children[0].0]);
        let mut valid = matches!(ty, Type::Bool | Type::Enum(_) | Type::Tuple(_) | Type::Unit);
        if !valid && ty != Type::Error {
            self.report(
                2101,
                self.module.nodes()[node.children[0].0].span,
                "match requires Bool, Copy sum, Copy Tuple or Unit",
                None,
            );
        }
        let before = self.result.diagnostics.len();
        let mut roots = vec![];
        let mut all = vec![];
        let mut count = 0;
        for (ordinal, &arm) in node.children[1..].iter().enumerate() {
            if self.module.nodes()[arm.0].kind != HirKind::Arm {
                valid = false;
                continue;
            }
            let root = self.module.nodes()[arm.0].children[0];
            roots.push(root);
            if ordinal == 1025 {
                self.enum_limit(self.module.nodes()[root.0].span, "match arm limit: 1025");
                valid = false;
            }
            let mut names = BTreeSet::new();
            let mut work = vec![(root, ty, None)];
            while let Some((pattern, expected, annotation)) = work.pop() {
                count += 1;
                all.push(pattern);
                if count == PATTERN_NODE_LIMIT + 1 {
                    self.enum_limit(
                        self.module.nodes()[pattern.0].span,
                        "match source pattern node limit: 10000",
                    );
                    valid = false;
                }
                let p = &self.module.nodes()[pattern.0];
                let mut fields = vec![];
                let mut field_annotations = vec![];
                let kind = match p.kind {
                    HirKind::Binder { name, .. } => {
                        if !names.insert(name.0) {
                            valid = false;
                        }
                        RecursivePatternKind::Wildcard
                    }
                    HirKind::Wildcard => RecursivePatternKind::Wildcard,
                    HirKind::PatternBoolean(value) => {
                        if expected != Type::Bool && expected != Type::Error {
                            self.report(
                                2101,
                                p.span,
                                "Bool pattern requires Bool component",
                                annotation,
                            );
                        }
                        RecursivePatternKind::Bool(value)
                    }
                    HirKind::PatternUnit => {
                        if expected != Type::Unit && expected != Type::Error {
                            self.report(
                                2101,
                                p.span,
                                "Unit pattern requires Unit component",
                                annotation,
                            );
                        }
                        RecursivePatternKind::Unit
                    }
                    HirKind::PatternTuple => {
                        if let Type::Tuple(tuple) = expected {
                            fields = self.result.structs[&tuple]
                                .fields
                                .iter()
                                .map(|f| f.ty)
                                .collect();
                            field_annotations = self
                                .result
                                .field_sources
                                .get(&tuple)
                                .cloned()
                                .unwrap_or_default();
                            if fields.len() != p.children.len() {
                                self.report(
                                    2201,
                                    p.span,
                                    "tuple pattern arity does not match",
                                    annotation,
                                );
                            }
                            RecursivePatternKind::Tuple(tuple)
                        } else {
                            if expected != Type::Error {
                                self.report(
                                    2101,
                                    p.span,
                                    "tuple pattern requires Tuple component",
                                    annotation,
                                );
                            }
                            RecursivePatternKind::Wildcard
                        }
                    }
                    HirKind::PatternVariant { .. } | HirKind::PatternNone => {
                        let (arguments, path_span) = match p.kind {
                            HirKind::PatternVariant {
                                arguments,
                                owner_span,
                                name_span,
                                ..
                            } => (
                                arguments,
                                nova_source::Span::new(
                                    owner_span.file(),
                                    owner_span.start(),
                                    name_span.end(),
                                )
                                .expect("pattern path"),
                            ),
                            _ => (false, p.span),
                        };
                        let variant = if expected == Type::Error {
                            None
                        } else if self.sum_head(pattern).is_some() {
                            let tid = self.result.types.intern(expected);
                            let v = self.prepare_sum(pattern, Some(tid));
                            if v.is_none() {
                                let (_, name) = self.sum_head(pattern).expect("sum head");
                                if matches!(name, "Some" | "None" | "Success" | "Error") {
                                    self.report(
                                        2101,
                                        path_span,
                                        "pattern sum family differs from component",
                                        annotation,
                                    );
                                } else {
                                    self.sum_error(pattern, pattern, Some(tid));
                                }
                            }
                            v
                        } else {
                            let alias = if let HirKind::PatternVariant { owner, .. } = p.kind {
                                self.type_definition(pattern, owner).is_some_and(|def| {
                                    matches!(
                                        self.resolved.definitions[def.0].kind,
                                        nova_resolve::DefinitionKind::TypeAlias(_)
                                    )
                                })
                            } else {
                                false
                            };
                            if alias {
                                self.report(
                                    1102,
                                    path_span,
                                    "alias variant heads are unsupported",
                                    None,
                                );
                                None
                            } else {
                                self.resolve_variant(pattern)
                            }
                        };
                        if let Some(v) = variant {
                            if expected != Type::Enum(v.enumeration) {
                                self.report(
                                    2101,
                                    path_span,
                                    "pattern Enum differs from component",
                                    annotation,
                                );
                            } else {
                                let shape = &self.result.enums[&v.enumeration];
                                fields = shape.variants[v.index]
                                    .fields
                                    .iter()
                                    .map(|f| f.ty)
                                    .collect();
                                let offset = shape.variants[..v.index]
                                    .iter()
                                    .map(|v| v.fields.len())
                                    .sum::<usize>();
                                field_annotations = self
                                    .result
                                    .field_sources
                                    .get(&StructId(v.enumeration.0))
                                    .map(|s| {
                                        s.iter().skip(offset).take(fields.len()).copied().collect()
                                    })
                                    .unwrap_or_default();
                                if fields.len() != p.children.len()
                                    || arguments == fields.is_empty()
                                {
                                    self.report(
                                        2201,
                                        p.span,
                                        "variant pattern arity does not match",
                                        annotation,
                                    );
                                }
                            }
                            RecursivePatternKind::Variant(v)
                        } else {
                            valid = false;
                            RecursivePatternKind::Wildcard
                        }
                    }
                    _ => {
                        valid = false;
                        RecursivePatternKind::Wildcard
                    }
                };
                if let Some(def) = self.resolved.declaration_ids[pattern.0] {
                    self.result.definition_types[def.0] = self.result.types.intern(expected);
                }
                self.set(pattern, expected);
                self.result.recursive_patterns[pattern.0] = Some(RecursivePattern {
                    ty: expected,
                    kind,
                    children: p.children.clone(),
                });
                for (at, &child) in p.children.iter().enumerate().rev() {
                    work.push((
                        child,
                        fields.get(at).copied().unwrap_or(Type::Error),
                        field_annotations
                            .get(at)
                            .map(|s| self.module.nodes()[s.0].span),
                    ));
                }
            }
            self.set(arm, Type::Unit);
        }
        valid &= self.result.diagnostics.len() == before && ty != Type::Error;
        if !valid {
            return;
        }
        let result = analyze(
            &self.result.structs,
            &self.result.enums,
            &self.result.sums,
            &self.result.recursive_patterns,
            &all,
            &roots,
            ty,
        );
        let report = match result {
            Ok(report) => report,
            Err(limit) => {
                self.enum_limit(keyword, limit.message());
                return;
            }
        };
        for (at, covering) in report.unreachable {
            let pattern = roots[at];
            self.report(
                3102,
                self.module.nodes()[pattern.0].span,
                "unreachable match arm",
                None,
            );
            let diagnostic = self.result.diagnostics.last_mut().expect("coverage error");
            for &previous in covering.iter().take(8) {
                diagnostic.secondary.push(nova_diagnostics::Label {
                    span: self.module.nodes()[roots[previous].0].span,
                    message: "covering pattern".into(),
                });
            }
            if covering.len() > 8 {
                diagnostic
                    .notes
                    .push("additional covering patterns omitted".into());
            }
        }
        if !report.missing.is_empty() {
            self.report(3101, keyword, "incomplete match", None);
            let diagnostic = self.result.diagnostics.last_mut().expect("coverage error");
            for witness in report.missing {
                diagnostic.notes.push(format!("missing: {witness}"));
            }
            if report.more_missing {
                diagnostic
                    .notes
                    .push("additional missing cases omitted".into());
            }
        } else {
            self.result.exhaustive_matches.insert(id.0);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Constructor {
    Bool(bool),
    Unit,
    Tuple(StructId),
    Variant(VariantId),
    Opaque,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Pattern {
    constructor: Option<Constructor>,
    children: Vec<usize>,
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Key {
    types: Vec<Type>,
    rows: Vec<Vec<usize>>,
    candidate: Vec<usize>,
}
#[derive(Clone, Copy, Debug)]
struct Answer {
    useful: bool,
    winner: Option<(Constructor, usize)>,
}
struct Task {
    key: std::sync::Arc<Key>,
    answer: Option<Answer>,
    choice: usize,
    waiting: Option<(Constructor, usize)>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Limit {
    Tasks,
    Cells,
}
impl Limit {
    fn message(self) -> &'static str {
        match self {
            Self::Tasks => "match matrix subproblem limit: 100000",
            Self::Cells => "match live matrix cell limit: 1000000",
        }
    }
}
struct Engine<'a> {
    structs: &'a StructRegistry,
    enums: &'a EnumRegistry,
    sums: &'a SumRegistry,
    patterns: Vec<Pattern>,
    interned: HashMap<Pattern, usize>,
    tasks: Vec<Task>,
    memo: HashMap<u64, Vec<usize>>,
    cells: usize,
}
impl<'a> Engine<'a> {
    fn new(structs: &'a StructRegistry, enums: &'a EnumRegistry, sums: &'a SumRegistry) -> Self {
        let wildcard = Pattern {
            constructor: None,
            children: vec![],
        };
        Self {
            structs,
            enums,
            sums,
            patterns: vec![wildcard.clone()],
            interned: HashMap::from([(wildcard, 0)]),
            tasks: vec![],
            memo: HashMap::new(),
            cells: 0,
        }
    }
    fn pattern(&mut self, p: Pattern) -> usize {
        if let Some(&id) = self.interned.get(&p) {
            return id;
        }
        let id = self.patterns.len();
        self.patterns.push(p.clone());
        self.interned.insert(p, id);
        id
    }
    fn task(&mut self, key: Key) -> Result<usize, Limit> {
        let fingerprint = Self::fingerprint(&key);
        if let Some(ids) = self.memo.get(&fingerprint) {
            if let Some(&id) = ids.iter().find(|&&id| *self.tasks[id].key == key) {
                return Ok(id);
            }
        }
        if self.tasks.len() == MATRIX_TASK_LIMIT {
            return Err(Limit::Tasks);
        }
        let cells = key
            .rows
            .iter()
            .map(Vec::len)
            .sum::<usize>()
            .checked_add(key.candidate.len())
            .ok_or(Limit::Cells)?;
        if cells > MATRIX_CELL_LIMIT - self.cells {
            return Err(Limit::Cells);
        }
        self.cells += cells;
        let id = self.tasks.len();
        let key = std::sync::Arc::new(key);
        self.memo.entry(fingerprint).or_default().push(id);
        self.tasks.push(Task {
            key,
            answer: None,
            choice: 0,
            waiting: None,
        });
        Ok(id)
    }
    fn fields(&self, constructor: Constructor) -> Vec<Type> {
        match constructor {
            Constructor::Tuple(id) => self.structs[&id].fields.iter().map(|f| f.ty).collect(),
            Constructor::Variant(id) => self.enums[&id.enumeration].variants[id.index]
                .fields
                .iter()
                .map(|f| f.ty)
                .collect(),
            _ => vec![],
        }
    }
    fn constructors(&self, key: &Key) -> Vec<Constructor> {
        if let Some(c) = self.patterns[key.candidate[0]].constructor {
            return vec![c];
        }
        // Default matrix: an unconstrained column need not enumerate its domain.
        if key
            .rows
            .iter()
            .all(|r| self.patterns[r[0]].constructor.is_none())
        {
            return vec![Constructor::Opaque];
        }
        match key.types[0] {
            Type::Bool => vec![Constructor::Bool(false), Constructor::Bool(true)],
            Type::Unit => vec![Constructor::Unit],
            Type::Tuple(id) => vec![Constructor::Tuple(id)],
            Type::Enum(id) => {
                let mut indexes = (0..self.enums[&id].variants.len()).collect::<Vec<_>>();
                if self
                    .sums
                    .get(&id)
                    .is_some_and(|key| key.family == SumFamily::Option)
                {
                    indexes.reverse();
                }
                indexes
                    .into_iter()
                    .map(|index| {
                        Constructor::Variant(VariantId {
                            enumeration: id,
                            index,
                        })
                    })
                    .collect()
            }
            _ => vec![Constructor::Opaque],
        }
    }
    fn fingerprint(key: &Key) -> u64 {
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        key.types.len().hash(&mut hash);
        for ty in &key.types {
            ty.hash(&mut hash);
        }
        key.rows.len().hash(&mut hash);
        for row in &key.rows {
            row.len().hash(&mut hash);
            for pattern in row {
                pattern.hash(&mut hash);
            }
        }
        key.candidate.len().hash(&mut hash);
        for pattern in &key.candidate {
            pattern.hash(&mut hash);
        }
        hash.finish()
    }
    fn projected_slot(&self, row: &[usize], arity: usize, column: usize) -> usize {
        if column >= arity {
            return row[column - arity + 1];
        }
        let pattern = &self.patterns[row[0]];
        if pattern.constructor.is_none() {
            0
        } else {
            pattern.children[column]
        }
    }
    fn specialized_task(
        &mut self,
        parent: usize,
        constructor: Constructor,
    ) -> Result<usize, Limit> {
        let key = self.tasks[parent].key.clone();
        let fields = self.fields(constructor);
        let arity = fields.len();
        let columns = arity + key.candidate.len() - 1;
        let selected = key
            .rows
            .iter()
            .enumerate()
            .filter_map(|(at, row)| {
                let p = &self.patterns[row[0]];
                (p.constructor.is_none() || p.constructor == Some(constructor)).then_some(at)
            })
            .collect::<Vec<_>>();
        let cells = columns
            .checked_mul(selected.len() + 1)
            .ok_or(Limit::Cells)?;
        if cells > MATRIX_CELL_LIMIT {
            return Err(Limit::Cells);
        }
        // Hash/compare a borrowed specialization before allocating its rows.
        // Cache hits allocate no matrix cells and consume no subproblem budget.
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        columns.hash(&mut hash);
        for ty in fields.iter().chain(&key.types[1..]) {
            ty.hash(&mut hash);
        }
        selected.len().hash(&mut hash);
        for &at in &selected {
            columns.hash(&mut hash);
            for column in 0..columns {
                self.projected_slot(&key.rows[at], arity, column)
                    .hash(&mut hash);
            }
        }
        columns.hash(&mut hash);
        for column in 0..columns {
            self.projected_slot(&key.candidate, arity, column)
                .hash(&mut hash);
        }
        if let Some(ids) = self.memo.get(&hash.finish()) {
            for &id in ids {
                let old = &self.tasks[id].key;
                if old.types.iter().eq(fields.iter().chain(&key.types[1..]))
                    && old.rows.len() == selected.len()
                    && old.candidate.len() == columns
                    && old.rows.iter().zip(&selected).all(|(row, &at)| {
                        row.len() == columns
                            && row.iter().enumerate().all(|(column, &p)| {
                                p == self.projected_slot(&key.rows[at], arity, column)
                            })
                    })
                    && old
                        .candidate
                        .iter()
                        .enumerate()
                        .all(|(column, &p)| p == self.projected_slot(&key.candidate, arity, column))
                {
                    return Ok(id);
                }
            }
        }
        if self.tasks.len() == MATRIX_TASK_LIMIT {
            return Err(Limit::Tasks);
        }
        if cells > MATRIX_CELL_LIMIT - self.cells {
            return Err(Limit::Cells);
        }
        let rows = selected
            .iter()
            .map(|&at| {
                (0..columns)
                    .map(|column| self.projected_slot(&key.rows[at], arity, column))
                    .collect()
            })
            .collect();
        let candidate = (0..columns)
            .map(|column| self.projected_slot(&key.candidate, arity, column))
            .collect();
        let types = fields
            .into_iter()
            .chain(key.types[1..].iter().copied())
            .collect();
        self.task(Key {
            types,
            rows,
            candidate,
        })
    }
    fn useful(&mut self, key: Key) -> Result<usize, Limit> {
        let root = self.task(key)?;
        let mut stack = vec![root];
        while let Some(&id) = stack.last() {
            if self.tasks[id].answer.is_some() {
                stack.pop();
                continue;
            }
            if self.tasks[id].key.rows.is_empty() {
                self.tasks[id].answer = Some(Answer {
                    useful: true,
                    winner: None,
                });
                continue;
            }
            if self.tasks[id]
                .key
                .rows
                .iter()
                .any(|row| row.iter().all(|&p| self.patterns[p].constructor.is_none()))
            {
                self.tasks[id].answer = Some(Answer {
                    useful: false,
                    winner: None,
                });
                continue;
            }
            if self.tasks[id].key.candidate.is_empty() {
                self.tasks[id].answer = Some(Answer {
                    useful: self.tasks[id].key.rows.is_empty(),
                    winner: None,
                });
                continue;
            }
            if let Some((constructor, child)) = self.tasks[id].waiting {
                let answer = self.tasks[child].answer.expect("completed child");
                if answer.useful {
                    self.tasks[id].answer = Some(Answer {
                        useful: true,
                        winner: Some((constructor, child)),
                    });
                    continue;
                }
                self.tasks[id].waiting = None;
                self.tasks[id].choice += 1;
            }
            let choices = self.constructors(&self.tasks[id].key);
            let Some(&constructor) = choices.get(self.tasks[id].choice) else {
                self.tasks[id].answer = Some(Answer {
                    useful: false,
                    winner: None,
                });
                continue;
            };
            let child = self.specialized_task(id, constructor)?;
            self.tasks[id].waiting = Some((constructor, child));
            stack.push(child);
        }
        Ok(root)
    }
    fn witness(&mut self, mut id: usize) -> Vec<usize> {
        let mut work = vec![];
        while let Some((constructor, child)) = self.tasks[id].answer.expect("answer").winner {
            work.push(constructor);
            id = child;
        }
        let mut values: std::collections::VecDeque<_> =
            self.tasks[id].key.candidate.iter().copied().collect();
        while let Some(constructor) = work.pop() {
            let arity = self.fields(constructor).len();
            let children = values.drain(..arity).collect();
            let pattern = if constructor == Constructor::Opaque {
                0
            } else {
                self.pattern(Pattern {
                    constructor: Some(constructor),
                    children,
                })
            };
            values.push_front(pattern);
        }
        values.into_iter().collect()
    }
    fn render(&self, root: usize) -> String {
        enum Part {
            Node(usize),
            Text(String),
        }
        let mut work = vec![Part::Node(root)];
        let mut out = String::new();
        while let Some(part) = work.pop() {
            match part {
                Part::Text(text) => out.push_str(&text),
                Part::Node(id) => {
                    let p = &self.patterns[id];
                    let head = match p.constructor {
                        None | Some(Constructor::Opaque) => {
                            out.push('_');
                            continue;
                        }
                        Some(Constructor::Bool(value)) => {
                            out.push_str(if value { "true" } else { "false" });
                            continue;
                        }
                        Some(Constructor::Unit) => {
                            out.push_str("()");
                            continue;
                        }
                        Some(Constructor::Tuple(_)) => String::new(),
                        Some(Constructor::Variant(v)) => format!(
                            "{}::{}",
                            self.sums
                                .get(&v.enumeration)
                                .map(|key| match key.family {
                                    SumFamily::Option => "Option",
                                    SumFamily::Result => "Result",
                                })
                                .unwrap_or(&self.enums[&v.enumeration].name),
                            self.enums[&v.enumeration].variants[v.index].name
                        ),
                    };
                    out.push_str(&head);
                    if p.children.is_empty() {
                        continue;
                    }
                    out.push('(');
                    work.push(Part::Text(
                        if matches!(p.constructor, Some(Constructor::Tuple(_)))
                            && p.children.len() == 1
                        {
                            ",)"
                        } else {
                            ")"
                        }
                        .into(),
                    ));
                    for (at, &child) in p.children.iter().enumerate().rev() {
                        work.push(Part::Node(child));
                        if at > 0 {
                            work.push(Part::Text(", ".into()));
                        }
                    }
                }
            }
        }
        out
    }
    fn intersects(&self, first: usize, second: usize) -> bool {
        let mut work = vec![(first, second)];
        while let Some((first, second)) = work.pop() {
            let a = &self.patterns[first];
            let b = &self.patterns[second];
            if a.constructor.is_none() || b.constructor.is_none() {
                continue;
            }
            if a.constructor != b.constructor {
                return false;
            }
            work.extend(a.children.iter().zip(&b.children).map(|(&a, &b)| (a, b)));
        }
        true
    }
}
struct Report {
    unreachable: Vec<(usize, Vec<usize>)>,
    missing: Vec<String>,
    more_missing: bool,
}
fn analyze(
    structs: &StructRegistry,
    enums: &EnumRegistry,
    sums: &SumRegistry,
    patterns: &[Option<RecursivePattern>],
    all: &[HirId],
    roots: &[HirId],
    ty: Type,
) -> Result<Report, Limit> {
    let mut engine = Engine::new(structs, enums, sums);
    let mut mapped = HashMap::new();
    for &id in all.iter().rev() {
        let p = patterns[id.0].as_ref().expect("typed pattern");
        let constructor = match p.kind {
            RecursivePatternKind::Wildcard => None,
            RecursivePatternKind::Bool(b) => Some(Constructor::Bool(b)),
            RecursivePatternKind::Unit => Some(Constructor::Unit),
            RecursivePatternKind::Tuple(id) => Some(Constructor::Tuple(id)),
            RecursivePatternKind::Variant(v) => Some(Constructor::Variant(v)),
        };
        let children = p.children.iter().map(|id| mapped[&id.0]).collect();
        let pattern = engine.pattern(Pattern {
            constructor,
            children,
        });
        mapped.insert(id.0, pattern);
    }
    let mut rows = vec![];
    let mut reachable: Vec<usize> = vec![];
    let mut unreachable = vec![];
    for (at, root) in roots.iter().enumerate() {
        let pattern = mapped[&root.0];
        let task = engine.useful(Key {
            types: vec![ty],
            rows: rows.clone(),
            candidate: vec![pattern],
        })?;
        if !engine.tasks[task].answer.expect("usefulness").useful {
            unreachable.push((
                at,
                reachable
                    .iter()
                    .copied()
                    .filter(|&previous| engine.intersects(pattern, mapped[&roots[previous].0]))
                    .collect(),
            ));
        } else {
            rows.push(vec![pattern]);
            reachable.push(at);
        }
    }
    let mut missing = vec![];
    let mut more_missing = false;
    loop {
        let task = engine.useful(Key {
            types: vec![ty],
            rows: rows.clone(),
            candidate: vec![0],
        })?;
        if !engine.tasks[task].answer.expect("exhaustiveness").useful {
            break;
        }
        if missing.len() == 8 {
            more_missing = true;
            break;
        }
        let witness = engine.witness(task);
        missing.push(engine.render(witness[0]));
        // Each witness describes an uncovered region, including opaque/default
        // columns. Exclude that region to find the next deterministic example.
        rows.push(witness);
    }
    Ok(Report {
        unreachable,
        missing,
        more_missing,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn key(mut number: usize, width: usize) -> Key {
        let atoms = [
            Type::Bool,
            Type::Unit,
            Type::Char,
            Type::Int8,
            Type::UInt8,
            Type::Int16,
            Type::UInt16,
            Type::Int32,
            Type::UInt32,
            Type::Int64,
            Type::UInt64,
            Type::Float32,
            Type::Float64,
        ];
        let mut types = vec![Type::Bool; width];
        for slot in types.iter_mut().take(5) {
            *slot = atoms[number % atoms.len()];
            number /= atoms.len();
        }
        Key {
            types,
            rows: vec![],
            candidate: vec![0; width],
        }
    }
    #[test]
    fn p24_matrix_task_budget_exact_boundary_and_cache_hits() {
        let structs = StructRegistry::new();
        let enums = EnumRegistry::new();
        let sums = SumRegistry::new();
        let mut engine = Engine::new(&structs, &enums, &sums);
        for n in 0..MATRIX_TASK_LIMIT {
            engine.useful(key(n, 5)).unwrap();
        }
        assert_eq!(engine.tasks.len(), 100_000);
        assert_eq!(engine.cells, 500_000);
        let previous = engine.useful(key(0, 5)).unwrap();
        assert_eq!(previous, 0);
        assert_eq!(engine.tasks.len(), 100_000);
        assert_eq!(engine.useful(key(100_000, 5)), Err(Limit::Tasks));
    }
    #[test]
    fn p24_matrix_cell_budget_exact_boundary_and_preallocation_guard() {
        let structs = StructRegistry::new();
        let enums = EnumRegistry::new();
        let sums = SumRegistry::new();
        let mut engine = Engine::new(&structs, &enums, &sums);
        for n in 0..1000 {
            engine.useful(key(n, 1000)).unwrap();
        }
        assert_eq!(engine.cells, 1_000_000);
        engine.useful(key(0, 1000)).unwrap();
        assert_eq!(engine.useful(key(1000, 1000)), Err(Limit::Cells));
        assert_eq!(engine.tasks.len(), 1000);
    }
}

#[cfg(test)]
mod specialization_tests {
    use super::*;
    #[test]
    fn p24_borrowed_specialization_cache_and_expansion_preallocation_guard() {
        let tuple = StructId(0);
        let mut structs = StructRegistry::new();
        structs.insert(
            tuple,
            nova_types::StructShape {
                name: "tuple".into(),
                fields: (0..1024)
                    .map(|i| nova_types::StructField {
                        name: i.to_string(),
                        ty: Type::Bool,
                        mutable: false,
                    })
                    .collect(),
                layout: None,
            },
        );
        let enums = EnumRegistry::new();
        let sums = SumRegistry::new();
        let mut engine = Engine::new(&structs, &enums, &sums);
        let pattern = engine.pattern(Pattern {
            constructor: Some(Constructor::Tuple(tuple)),
            children: vec![0; 1024],
        });
        let parent = engine
            .task(Key {
                types: vec![Type::Tuple(tuple)],
                rows: vec![vec![pattern]; 1000],
                candidate: vec![0],
            })
            .unwrap();
        assert_eq!(
            engine.specialized_task(parent, Constructor::Tuple(tuple)),
            Err(Limit::Cells)
        );
        assert_eq!(engine.tasks.len(), 1);
        assert_eq!(engine.cells, 1001);
        let parent = engine
            .task(Key {
                types: vec![Type::Tuple(tuple)],
                rows: vec![vec![pattern]],
                candidate: vec![0],
            })
            .unwrap();
        let child = engine
            .specialized_task(parent, Constructor::Tuple(tuple))
            .unwrap();
        let cells = engine.cells;
        let tasks = engine.tasks.len();
        // At the live-cell limit the already-interned expansion is still free.
        engine.cells = MATRIX_CELL_LIMIT;
        assert_eq!(
            engine.specialized_task(parent, Constructor::Tuple(tuple)),
            Ok(child)
        );
        assert_eq!(engine.tasks.len(), tasks);
        assert_eq!(engine.cells, MATRIX_CELL_LIMIT);
        engine.cells = cells;
    }
}
