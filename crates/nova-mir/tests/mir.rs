use nova_hir::HirKind;
use nova_lexer::{lex, normalize_ends};
use nova_mir::*;
use nova_parser::parse;
use nova_resolve::resolve;
use nova_source::SourceDatabase;
use nova_syntax::Symbol;
use nova_typecheck::check;
use nova_types::Type;

fn compile(source: &str) -> Result<Module, LoweringError> {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", source.into()).unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = resolve(&hir);
    let checked = check(&hir, &resolved).unwrap();
    let result = lower(
        &hir,
        &resolved,
        &checked,
        lexed.has_errors() || parsed.has_errors(),
    );
    if let Ok(module) = &result {
        for info in &module.sources {
            assert_eq!(info.span, hir.node(info.hir).unwrap().span);
            assert_eq!(info.origin, hir.node(info.hir).unwrap().origin);
            sources.slice(info.span).unwrap();
        }
        assert!(validate(module).is_empty());
    }
    result
}
fn pass(source: &str) -> Module {
    compile(source).unwrap()
}
fn body_named<'a>(module: &'a Module, name: &str) -> &'a Body {
    module
        .bodies
        .iter()
        .find(|body| module.callees[body.callee.0].name == name)
        .unwrap()
}

#[test]
fn p09_float_mir_conversions_preserve_raw_width_and_reject_illegal_values() {
    let source="func f(a:float,b:float,i:int16,u:uint32)->double{let c:double=a+b;let d:float=a+i;let e:double=b+u;return c+d+e}";
    let module = pass(source);
    let statements: Vec<_> = body_named(&module, "f")
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .collect();
    assert!(statements.iter().any(|s| matches!(
        s.kind,
        StatementKind::Assign(_, Rvalue::NumericConvert(_, Type::Float64))
    )));
    assert!(statements.iter().any(|s| matches!(
        s.kind,
        StatementKind::Assign(_, Rvalue::NumericConvert(_, Type::Float32))
    )));
    for bad in [
        Rvalue::Widen(
            Operand::Constant(Constant::Float(nova_types::FloatValue::from_f32(1.0))),
            Type::Float64,
        ),
        Rvalue::NumericConvert(Operand::Constant(Constant::Int32(1)), Type::Float32),
        Rvalue::NumericConvert(
            Operand::Constant(Constant::Float(nova_types::FloatValue::from_f64(1.0))),
            Type::Float32,
        ),
        Rvalue::NumericConvert(Operand::Constant(Constant::Bool(true)), Type::Float64),
        Rvalue::Binary(
            Symbol::Percent,
            Operand::Constant(Constant::Float(nova_types::FloatValue::from_f32(1.0))),
            Operand::Constant(Constant::Float(nova_types::FloatValue::from_f32(1.0))),
        ),
    ] {
        let mut module = pass(source);
        let body = module
            .bodies
            .iter_mut()
            .find(|b| module.callees[b.callee.0].name == "f")
            .unwrap();
        let statement = body
            .blocks
            .iter_mut()
            .flat_map(|b| &mut b.statements)
            .next()
            .unwrap();
        let StatementKind::Assign(_, ref mut value) = statement.kind;
        *value = bad;
        assert!(!validate(&module).is_empty());
    }
}

#[test]
fn p09_float_analysis_bits_zero_nan_counts_and_tables_cannot_be_forged() {
    use nova_types::{ConstValue, FloatValue};
    let mut sources = SourceDatabase::default();
    let file = sources
        .add(
            "test.nova",
            "const C:double=-0.0;const N:double=0.0/0.0;func f()->double{return C+N}".into(),
        )
        .unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = resolve(&hir);
    let id = hir
        .nodes()
        .iter()
        .position(|n| matches!(n.kind, HirKind::Float(_)))
        .unwrap();
    let def = resolved
        .definitions
        .iter()
        .position(|d| d.name == "C")
        .unwrap();
    for mutation in 0..7 {
        let mut checked = check(&hir, &resolved).unwrap();
        match mutation {
            0 => checked.float_literals[id] = Some(FloatValue::from_f64(1.0)),
            1 => checked.float_literals.clear(),
            2 => checked.type_table[id] = checked.types.intern(Type::Float32),
            3 => checked.coercions[id] = Some(checked.types.intern(Type::Int32)),
            4 => {
                checked.const_values[def] = nova_typecheck::ConstEvaluation::Value {
                    value: ConstValue::Float(FloatValue::from_f64(0.0)),
                    nodes: 2,
                }
            }
            5 => {
                checked.const_values[def] = nova_typecheck::ConstEvaluation::Value {
                    value: ConstValue::Float(FloatValue::from_f64(-0.0)),
                    nodes: 3,
                }
            }
            _ => {
                checked.const_values[def] = nova_typecheck::ConstEvaluation::Value {
                    value: ConstValue::Float(FloatValue::from_f64(f64::NAN)),
                    nodes: 2,
                }
            }
        }
        assert_eq!(
            lower(&hir, &resolved, &checked, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
}

#[test]
fn p08_char_constants_places_calls_and_sources_are_preserved() {
    let module = pass(include_str!("../../../examples/characters.nova"));
    assert!(module
        .callees
        .iter()
        .any(|c| c.name == "echo" && c.parameters == [Type::Char] && c.return_type == Type::Char));
    assert!(module
        .bodies
        .iter()
        .flat_map(|b| &b.locals)
        .any(|l| l.ty == Type::Char));
    assert!(module.bodies.iter().flat_map(|b| &b.blocks).flat_map(|b| &b.statements).any(|s| matches!(&s.kind, StatementKind::Assign(_, Rvalue::Interpolate(parts)) if parts.contains(&Operand::Constant(Constant::Char('🙂'))))));
    assert!(!module
        .bodies
        .iter()
        .flat_map(|b| &b.blocks)
        .flat_map(|b| &b.statements)
        .any(|s| matches!(s.kind, StatementKind::Assign(_, Rvalue::Widen(_, _)))));
    for source in [
        "func f(){print('a')}",
        "func f(){let x='a'+1}",
        "func f(){const x='a'=='b';x=false}",
        "const C=true||(C&&'a'=='a')",
    ] {
        assert_eq!(compile(source), Err(LoweringError::FrontendErrors));
    }
}

#[test]
fn p08_mir_validator_rejects_char_arithmetic_mixed_types_and_widening() {
    for op in [
        Symbol::EqualEqual,
        Symbol::BangEqual,
        Symbol::Less,
        Symbol::LessEqual,
        Symbol::Greater,
        Symbol::GreaterEqual,
    ] {
        let mut module = pass("func f(a:char,b:char)->bool{return a<b}");
        let statement = &mut module.bodies[0].blocks[0].statements[0];
        let StatementKind::Assign(_, value) = &mut statement.kind;
        *value = Rvalue::Binary(
            op,
            Operand::Constant(Constant::Char('\u{d7ff}')),
            Operand::Constant(Constant::Char('\u{e000}')),
        );
        assert!(validate(&module).is_empty());
        let StatementKind::Assign(_, value) = &mut module.bodies[0].blocks[0].statements[0].kind;
        *value = Rvalue::Binary(
            op,
            Operand::Constant(Constant::Char('a')),
            Operand::Constant(Constant::Int32(97)),
        );
        error(&module, Violation::TypeMismatch);
    }
    for value in [
        Rvalue::Binary(
            Symbol::Plus,
            Operand::Constant(Constant::Char('a')),
            Operand::Constant(Constant::Char('b')),
        ),
        Rvalue::Unary(Symbol::Minus, Operand::Constant(Constant::Char('a'))),
        Rvalue::Widen(Operand::Constant(Constant::Char('a')), Type::Int64),
        Rvalue::Widen(Operand::Constant(Constant::Int32(97)), Type::Char),
    ] {
        let mut module = pass("func f(){let x='a'}");
        let StatementKind::Assign(_, original) = &mut module.bodies[0].blocks[0].statements[0].kind;
        *original = value;
        error(&module, Violation::TypeMismatch);
    }
}

#[test]
fn p08_char_analysis_tampering_cannot_reach_mir() {
    let mut sources = SourceDatabase::default();
    let file = sources
        .add("test.nova", "const C='a';func f()->char{return C}".into())
        .unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = resolve(&hir);
    let id = hir
        .nodes()
        .iter()
        .position(|n| matches!(n.kind, HirKind::Character(_)))
        .unwrap();
    let def = resolved
        .definitions
        .iter()
        .position(|d| d.name == "C")
        .unwrap();
    for mutation in 0..5 {
        let mut checked = check(&hir, &resolved).unwrap();
        match mutation {
            0 => checked.type_table[id] = checked.types.intern(Type::Int32),
            1 => checked.coercions[id] = Some(checked.types.intern(Type::Int64)),
            2 => {
                checked.const_values[def] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Char('b'),
                    nodes: 1,
                }
            }
            3 => {
                checked.const_values[def] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Int32(97),
                    nodes: 1,
                }
            }
            _ => {
                checked.const_values[def] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Char('a'),
                    nodes: 2,
                }
            }
        }
        assert_eq!(
            lower(&hir, &resolved, &checked, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
}
fn error(module: &Module, violation: Violation) {
    assert!(
        validate(module).iter().any(|e| e.violation == violation),
        "{}\n{:?}",
        module.dump(),
        validate(module)
    );
}

// A bounded test oracle for control flow. It intentionally supports no runtime
// formatting/overflow/ABI policy. Arithmetic fixtures only use in-range addition.
fn operand(op: &Operand, locals: &[Option<Constant>]) -> Constant {
    match op {
        Operand::Constant(value) => value.clone(),
        Operand::Place(Place(local)) => locals[local.0].clone().expect("initialized local"),
    }
}
fn execute(module: &Module, name: &str, args: Vec<Constant>) -> (Constant, Vec<String>) {
    execute_with_fuel(module, name, args, 1000)
}
fn execute_with_fuel(
    module: &Module,
    name: &str,
    args: Vec<Constant>,
    mut fuel: usize,
) -> (Constant, Vec<String>) {
    let mut calls = vec![];
    fn run(
        module: &Module,
        body: &Body,
        args: Vec<Constant>,
        calls: &mut Vec<String>,
        fuel: &mut usize,
    ) -> Constant {
        let mut locals = vec![None; body.locals.len()];
        for (parameter, arg) in body.parameters.iter().zip(args) {
            locals[parameter.0] = Some(arg);
        }
        let mut current = body.entry;
        loop {
            *fuel = fuel.checked_sub(1).expect("bounded oracle");
            let block = &body.blocks[current.0];
            for statement in &block.statements {
                let StatementKind::Assign(Place(local), value) = &statement.kind;
                let value = match value {
                    Rvalue::Enum(variant, fields) => Constant::Enum(
                        *variant,
                        fields.iter().map(|field| operand(field, &locals)).collect(),
                    ),
                    Rvalue::Use(op) => operand(op, &locals),
                    Rvalue::Unary(Symbol::Bang, op) => {
                        let Constant::Bool(value) = operand(op, &locals) else {
                            panic!("bool")
                        };
                        Constant::Bool(!value)
                    }
                    Rvalue::Binary(Symbol::Plus, left, right) => {
                        let (Constant::Int32(left), Constant::Int32(right)) =
                            (operand(left, &locals), operand(right, &locals))
                        else {
                            panic!("int")
                        };
                        Constant::Int32(left.checked_add(right).expect("in-range fixture"))
                    }
                    Rvalue::Binary(
                        op @ (Symbol::Less | Symbol::LessEqual | Symbol::EqualEqual),
                        left,
                        right,
                    ) => {
                        let (Constant::Int32(left), Constant::Int32(right)) =
                            (operand(left, &locals), operand(right, &locals))
                        else {
                            panic!("int")
                        };
                        Constant::Bool(if *op == Symbol::Less {
                            left < right
                        } else if *op == Symbol::LessEqual {
                            left <= right
                        } else {
                            left == right
                        })
                    }
                    _ => panic!("operation outside control-flow oracle"),
                };
                locals[local.0] = Some(value);
            }
            match &block.terminator.as_ref().unwrap().kind {
                TerminatorKind::Goto(target) => current = *target,
                TerminatorKind::Branch {
                    condition,
                    then_block,
                    else_block,
                } => {
                    let Constant::Bool(value) = operand(condition, &locals) else {
                        panic!("bool")
                    };
                    current = if value { *then_block } else { *else_block };
                }
                TerminatorKind::Return(op) => return operand(op, &locals),
                TerminatorKind::Call {
                    callee,
                    arguments,
                    destination: Place(local),
                    target,
                } => {
                    let callee = &module.callees[callee.0];
                    let args = arguments.iter().map(|op| operand(op, &locals)).collect();
                    calls.push(callee.name.clone());
                    let result = if callee.builtin_print {
                        Constant::Unit
                    } else {
                        run(module, body_named(module, &callee.name), args, calls, fuel)
                    };
                    locals[local.0] = Some(result);
                    current = *target;
                }
                TerminatorKind::Unreachable => panic!("reachable orphan join"),
                TerminatorKind::Match { scrutinee, arms } => {
                    let value = operand(scrutinee, &locals);
                    current = arms
                        .iter()
                        .find_map(|(pattern, target)| {
                            let matches = match (pattern, &value) {
                                (
                                    nova_typecheck::MatchPattern::Variant(expected),
                                    Constant::Enum(actual, _),
                                ) => expected == actual,
                                (
                                    nova_typecheck::MatchPattern::Bool(expected),
                                    Constant::Bool(actual),
                                ) => expected == actual,
                                (nova_typecheck::MatchPattern::Wildcard, _) => true,
                                _ => false,
                            };
                            matches.then_some(*target)
                        })
                        .expect("exhaustive independent tag oracle");
                }
                TerminatorKind::Try { .. } => {
                    panic!("tag dispatch is outside this arithmetic interpreter corpus")
                }
            }
        }
    }
    let result = run(
        module,
        body_named(module, name),
        args,
        &mut calls,
        &mut fuel,
    );
    (result, calls)
}

#[test]
fn hello_snapshot_and_deterministic_lowering() {
    let source = "func main() { print(\"Hello, Nova\") }";
    let module = pass(source);
    assert_eq!(module, pass(source));
    let dump = module.dump();
    if std::env::var_os("NOVA_UPDATE_SNAPSHOTS").is_some() {
        std::fs::write(
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots/hello.mir"),
            &dump,
        )
        .unwrap();
    } else {
        assert_eq!(dump, include_str!("snapshots/hello.mir"));
    }
    let body = body_named(&module, "main");
    assert!(matches!(
        body.blocks[0].terminator.as_ref().unwrap().kind,
        TerminatorKind::Call { .. }
    ));
    assert_eq!(
        execute(&module, "main", vec![]),
        (Constant::Unit, vec!["print".into()])
    );
}

#[test]
fn loops_update_places_and_nearest_jump_targets_execute_correctly() {
    let source = "func f()->int{var i=0;var sum=0;while i<6 {i=i+1;if i==2 {continue} if i==5 {break} sum=sum+i} return sum}";
    let module = pass(source);
    assert_eq!(module, pass(source));
    assert_eq!(execute(&module, "f", vec![]).0, Constant::Int32(8));
    let module = pass("func f()->int{var i=0;var sum=0;while i<3 {i=i+1;var j=0;while j<4 {j=j+1;if j==2 {continue} if j==3 {break} sum=sum+1} sum=sum+10} return sum}");
    assert_eq!(execute(&module, "f", vec![]).0, Constant::Int32(33));
}

#[test]
fn zero_iteration_condition_calls_and_initializer_calls_have_exact_counts() {
    let module = pass("func condition(x:int)->bool{print(\"test\");return x<3} func make()->int{print(\"init\");return 0} func f()->int{var i=0;while condition(i) {let x=make();i=i+1;continue} while false {print(\"never\")} return i}");
    let (result, calls) = execute(&module, "f", vec![]);
    assert_eq!(result, Constant::Int32(3));
    assert_eq!(calls.iter().filter(|c| *c == "condition").count(), 4);
    assert_eq!(calls.iter().filter(|c| *c == "make").count(), 3);
    assert_eq!(calls.iter().filter(|c| *c == "print").count(), 7);
}

#[test]
fn loop_exits_skip_unreachable_code_and_preserve_function_returns() {
    let module =
        pass("func f()->int{while true {if true {return 7}else{break};print(\"never\")} return 9}");
    assert_eq!(execute(&module, "f", vec![]), (Constant::Int32(7), vec![]));
    let module = pass("func f()->int{var i=0;while i<2 {i=i+1;if i==1 {continue}else{break};print(\"never\")} return i}");
    assert_eq!(execute(&module, "f", vec![]), (Constant::Int32(2), vec![]));
}

#[test]
fn loop_validator_rejects_missing_initialization_on_zero_iteration_path() {
    let mut module = pass("func f(x:bool)->int{var value=0;while x {value=1;break} return value}");
    let body = &mut module.bodies[0];
    body.blocks[body.entry.0].statements.clear();
    error(&module, Violation::UninitializedRead);
}

#[test]
fn mutable_control_errors_are_blocked_before_mir() {
    for source in [
        "func f(){let x=1;x=2}",
        "func f(){break}",
        "func f(){while 1 {continue}}",
        "func f(){var x=1;x=true}",
    ] {
        assert_eq!(compile(source), Err(LoweringError::FrontendErrors));
    }
}

#[test]
fn const_materialization_removes_initializer_arithmetic_and_skipped_rhs() {
    let module = pass("func f()->int{const x=1+2*3;const skipped=false&&(1/0==0);return x}");
    assert_eq!(execute(&module, "f", vec![]).0, Constant::Int32(7));
    let body = body_named(&module, "f");
    assert_eq!(body.blocks.len(), 1);
    assert!(body.blocks[0].statements.iter().all(|s| matches!(
        s.kind,
        StatementKind::Assign(_, Rvalue::Use(Operand::Constant(_)))
    )));
    assert!(body.blocks[0].statements.iter().any(|s| matches!(
        s.kind,
        StatementKind::Assign(_, Rvalue::Use(Operand::Constant(Constant::Int32(7))))
    )));
    let runtime = pass("func f()->int{let x=1+2*3;return x}");
    assert!(body_named(&runtime, "f")
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .any(|s| matches!(s.kind, StatementKind::Assign(_, Rvalue::Binary(_, _, _)))));
}

#[test]
fn const_in_loops_and_all_value_types_preserve_cfg_and_sources() {
    let module = pass("func f()->int{const limit=3;const text=\"x\";const unit=();var x=0;while x<limit {const one=1;x=x+one} return x}");
    assert_eq!(execute(&module, "f", vec![]).0, Constant::Int32(3));
    assert_eq!(module, pass("func f()->int{const limit=3;const text=\"x\";const unit=();var x=0;while x<limit {const one=1;x=x+one} return x}"));
    assert!(validate(&module).is_empty());
}

#[test]
fn const_evaluation_failures_block_mir_even_in_dead_source() {
    for source in [
        "func f(){const x=1/0}",
        "func f(){return;const x=1/0}",
        "func f(){let x=1;const y=x}",
        "func f(){const x=print(\"x\")}",
    ] {
        assert_eq!(compile(source), Err(LoweringError::FrontendErrors));
    }
}

#[test]
fn tampered_const_values_counts_and_missing_tables_cannot_reach_mir() {
    let mut sources = SourceDatabase::default();
    let file = sources
        .add("test.nova", "func f(){const x=1+2}".into())
        .unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = resolve(&hir);
    for mutation in 0..4 {
        let mut checked = check(&hir, &resolved).unwrap();
        let index = resolved
            .definitions
            .iter()
            .position(|d| d.constant)
            .unwrap();
        match mutation {
            0 => {
                checked.const_values[index] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Int32(4),
                    nodes: 3,
                }
            }
            1 => {
                checked.const_values[index] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Int32(3),
                    nodes: 2,
                }
            }
            2 => checked.const_values[index] = nova_typecheck::ConstEvaluation::NotConstant,
            _ => checked.const_values.clear(),
        }
        assert_eq!(
            lower(&hir, &resolved, &checked, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
}

#[test]
fn global_reads_are_constants_with_sources_and_no_initializer_bodies() {
    let source = "const A=B*2+1;func f()->int{return A} const B=3;const S=\"한글\\0\";const U=();const T=true;func g(){const X=A;let Y=A+1;print(S)}";
    let module = pass(source);
    assert_eq!(module.bodies.len(), 2);
    assert_eq!(execute(&module, "f", vec![]).0, Constant::Int32(7));
    let f = body_named(&module, "f");
    assert!(f.locals.is_empty());
    assert!(f.blocks[0].statements.is_empty());
    assert!(matches!(
        &f.blocks[0].terminator.as_ref().unwrap().kind,
        TerminatorKind::Return(Operand::Constant(Constant::Int32(7)))
    ));
    let g = body_named(&module, "g");
    assert!(g
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .any(|s| matches!(
            s.kind,
            StatementKind::Assign(
                _,
                Rvalue::Binary(
                    Symbol::Plus,
                    Operand::Constant(Constant::Int32(7)),
                    Operand::Constant(Constant::Int32(1))
                )
            )
        )));
    let only = pass("const A=B+1;const B=2");
    assert!(only.bodies.is_empty() && only.callees.len() == 1);
    assert_eq!(module, pass(source));
}

#[test]
fn global_cycle_unused_failure_and_readonly_assignment_block_mir() {
    for source in [
        "const A=A;func f(){}",
        "const A=false&&B;const B=A;func f(){}",
        "const BAD=1/0;func f(){}",
        "const A=1;func f(){A=2}",
        "const A=missing;func f(){}",
    ] {
        assert_eq!(compile(source), Err(LoweringError::FrontendErrors));
    }
}

#[test]
fn global_value_type_count_and_resolution_tampering_cannot_reach_mir() {
    let mut sources = SourceDatabase::default();
    let file = sources
        .add(
            "test.nova",
            "const A=B+1;func f()->int{return A} const B=2".into(),
        )
        .unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    for mutation in 0..6 {
        let mut resolved = resolve(&hir);
        let mut checked = check(&hir, &resolved).unwrap();
        let index = resolved
            .definitions
            .iter()
            .position(|d| d.name == "A")
            .unwrap();
        match mutation {
            0 => {
                checked.const_values[index] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Int32(4),
                    nodes: 3,
                }
            }
            1 => {
                checked.const_values[index] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Int32(3),
                    nodes: 2,
                }
            }
            2 => checked.definition_types[index] = checked.types.intern(Type::Bool),
            3 => resolved.definitions[index].scope = nova_resolve::ScopeId(0),
            4 => {
                let nova_resolve::DefinitionKind::GlobalConst(id) =
                    resolved.definitions[index].kind
                else {
                    panic!("global")
                };
                resolved.definitions[index].kind = nova_resolve::DefinitionKind::Local(id);
            }
            _ => checked.const_values.clear(),
        }
        assert_eq!(
            lower(&hir, &resolved, &checked, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
}

#[test]
fn argument_and_operand_calls_run_in_source_order() {
    let module = pass("func a() -> int { return 1 } func b() -> int { return 2 } func sum(x:int,y:int)->int { return x+y } func f()->int { return sum(a(), b()) + a() }");
    assert_eq!(
        execute(&module, "f", vec![]),
        (
            Constant::Int32(4),
            vec!["a".into(), "b".into(), "sum".into(), "a".into()]
        )
    );
}

#[test]
fn logical_rhs_is_only_evaluated_on_its_branch() {
    let module = pass("func rhs()->bool { return true } func f(x:bool)->bool { return x && rhs() } func g(x:bool)->bool { return x || rhs() } func nested(x:bool)->bool { return (x && rhs()) || rhs() }");
    for (name, input, value, calls) in [
        ("f", false, false, vec![]),
        ("f", true, true, vec!["rhs".into()]),
        ("g", true, true, vec![]),
        ("g", false, true, vec!["rhs".into()]),
        ("nested", false, true, vec!["rhs".into()]),
        ("nested", true, true, vec!["rhs".into()]),
    ] {
        assert_eq!(
            execute(&module, name, vec![Constant::Bool(input)]),
            (Constant::Bool(value), calls)
        );
    }
    for body in &module.bodies {
        for block in &body.blocks {
            for statement in &block.statements {
                assert!(!matches!(
                    statement.kind,
                    StatementKind::Assign(_, Rvalue::Binary(Symbol::AndAnd | Symbol::OrOr, _, _))
                ));
            }
        }
    }
}

#[test]
fn branch_returns_fallthrough_and_unreachable_source() {
    let module = pass("func f(x:bool)->int { if x { return 1 } else { return 2 } let ignored=99 } func g(x:bool)->int { if x { return 3 } return 4 } func h(x:bool) { if x { return } print(\"ok\") }");
    for (name, input, value) in [
        ("f", true, 1),
        ("f", false, 2),
        ("g", true, 3),
        ("g", false, 4),
    ] {
        assert_eq!(
            execute(&module, name, vec![Constant::Bool(input)]).0,
            Constant::Int32(value)
        );
    }
    assert_eq!(
        execute(&module, "h", vec![Constant::Bool(true)]).1,
        Vec::<String>::new()
    );
    assert_eq!(
        execute(&module, "h", vec![Constant::Bool(false)]).1,
        vec!["print"]
    );
    assert!(!body_named(&module, "f")
        .locals
        .iter()
        .any(|l| l.ty == Type::Int32));
}

#[test]
fn shadowing_uses_stable_definitions_and_initializer_sees_outer_value() {
    let module =
        pass("func f(x:int, b:bool)->int { let y=x; if b { let y=y+1; return y } return y }");
    assert_eq!(
        execute(&module, "f", vec![Constant::Int32(5), Constant::Bool(true)]).0,
        Constant::Int32(6)
    );
    assert_eq!(
        execute(
            &module,
            "f",
            vec![Constant::Int32(5), Constant::Bool(false)]
        )
        .0,
        Constant::Int32(5)
    );
}

#[test]
fn signed_minimum_is_one_valid_constant_and_runtime_operations_are_preserved() {
    let module = pass(
        "func f()->int { let m=-2147483648; let zero=1/0; let overflow=2147483647+1; return m/-1 }",
    );
    let body = body_named(&module, "f");
    assert!(body.blocks[0].statements.iter().any(|s| matches!(
        s.kind,
        StatementKind::Assign(_, Rvalue::Use(Operand::Constant(Constant::Int32(i32::MIN))))
    )));
    let ops = body.blocks[0]
        .statements
        .iter()
        .filter_map(|s| match s.kind {
            StatementKind::Assign(_, Rvalue::Binary(op, _, _)) => Some(op),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(ops, vec![Symbol::Slash, Symbol::Plus, Symbol::Slash]);
}

#[test]
fn interpolation_preserves_typed_components_and_call_order() {
    let module = pass("func a()->int { return 1 } func b()->bool { return true } func main() { print(\"value {a()} {b()} 한글\") }");
    let body = body_named(&module, "main");
    let calls = body
        .blocks
        .iter()
        .filter_map(|bb| match &bb.terminator.as_ref()?.kind {
            TerminatorKind::Call { callee, .. } => Some(module.callees[callee.0].name.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(calls, ["a", "b", "print"]);
    let parts = body
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .find_map(|s| match &s.kind {
            StatementKind::Assign(_, Rvalue::Interpolate(parts)) => Some(parts),
            _ => None,
        })
        .unwrap();
    assert!(parts
        .iter()
        .any(|p| matches!(p, Operand::Constant(Constant::String(s)) if s.contains("한글"))));
    assert_eq!(
        parts
            .iter()
            .filter(|p| matches!(p, Operand::Place(_)))
            .count(),
        2
    );
}

#[test]
fn fragments_forward_recursive_and_grouped_calls_lower() {
    pass("");
    pass("func f(x:int)->int { return (g)(x) } func g(x:int)->int { if x==0 { return 0 } return f(x-1) }");
    pass("func print(x:int)->int { return x } func f()->int { return print(1) }");
    pass("func 한글(값: int)->int {\r\n return 값\r\n}");
}

#[test]
fn every_upstream_error_blocks_successful_mir() {
    for source in [
        "func main() { print(\"hi\")",
        "func main() { @ }",
        "func f()->int { return missing }",
        "func main(){if 1 {}}",
        "func main(){let x=2147483648}",
        "func main(){print(1)}",
        "func f()->int{}",
    ] {
        assert_eq!(
            compile(source),
            Err(LoweringError::FrontendErrors),
            "{source}"
        );
    }
}

#[test]
fn tampered_analysis_is_rejected_without_indexing_panics() {
    let mut sources = SourceDatabase::default();
    let file = sources
        .add("test.nova", "func f()->int{return 1}".into())
        .unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let mut resolved = resolve(&hir);
    let mut checked = check(&hir, &resolved).unwrap();
    assert!(lower(&hir, &resolved, &checked, false).is_ok());
    assert_eq!(
        lower(&hir, &resolved, &checked, true),
        Err(LoweringError::FrontendErrors)
    );
    checked.calls.clear();
    assert_eq!(
        lower(&hir, &resolved, &checked, false),
        Err(LoweringError::InvalidAnalysis)
    );
    checked = check(&hir, &resolved).unwrap();
    let literal = hir
        .nodes()
        .iter()
        .position(|n| matches!(n.kind, HirKind::Integer(_)))
        .unwrap();
    checked.integer_values[literal] = Some(99);
    assert_eq!(
        lower(&hir, &resolved, &checked, false),
        Err(LoweringError::InvalidAnalysis)
    );
    checked = check(&hir, &resolved).unwrap();
    resolved.declaration_ids.clear();
    assert_eq!(
        lower(&hir, &resolved, &checked, false),
        Err(LoweringError::InvalidAnalysis)
    );
}

#[test]
fn p07_widening_is_explicit_and_illegal_conversions_are_rejected() {
    pass("func f(x:int64)->int64{return x} func g(x:uint64)->uint64{return x} func main(){let a:byte=255;let s:int8=-128;var r:int64=0;r=s;print(\"{f(s)} {g(a)} {r}\")}");
    let base = pass("func f(a:int8,b:uint8)->int64{let x=a+b;return x}");
    let body = &base.bodies[0];
    let widens: Vec<_> = body
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .filter_map(|s| {
            if let StatementKind::Assign(_, Rvalue::Widen(operand, dest)) = &s.kind {
                Some((operand, *dest, s.source))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(widens.len(), 3);
    assert_eq!(
        widens.iter().map(|(_, ty, _)| *ty).collect::<Vec<_>>(),
        [Type::Int16, Type::Int16, Type::Int64]
    );
    for (_, _, source) in widens {
        assert_eq!(base.sources[source.hir.0], source);
    }
    for dest in [Type::Int8, Type::UInt64, Type::Bool, Type::Error] {
        let mut module = base.clone();
        let statement = module.bodies[0].blocks[0]
            .statements
            .iter_mut()
            .find(|s| {
                matches!(
                    s.kind,
                    StatementKind::Assign(_, Rvalue::Widen(_, Type::Int16))
                )
            })
            .unwrap();
        let StatementKind::Assign(place, value) = &mut statement.kind;
        *value = Rvalue::Widen(Operand::Constant(Constant::Int32(1)), dest);
        let local = place.0 .0;
        module.bodies[0].locals[local].ty = dest;
        error(&module, Violation::TypeMismatch);
    }
}

#[test]
fn p07_payload_source_types_and_coercion_tables_have_exact_provenance() {
    let mut sources = SourceDatabase::default();
    let file = sources
        .add(
            "test.nova",
            "func f(x:uint8)->int64{let n:uint64=18446744073709551615;return x}".into(),
        )
        .unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = resolve(&hir);
    for mutation in 0..4 {
        let mut checked = check(&hir, &resolved).unwrap();
        match mutation {
            0 => checked.coercions.clear(),
            1 => {
                let i = checked.coercions.iter().position(Option::is_some).unwrap();
                checked.coercions[i] = None;
            }
            2 => {
                let i = checked
                    .integer_literals
                    .iter()
                    .position(Option::is_some)
                    .unwrap();
                checked.integer_literals[i] =
                    nova_types::IntegerValue::new(nova_types::IntKind::U64, 1);
            }
            _ => {
                let i = checked.coercions.iter().position(Option::is_some).unwrap();
                checked.type_table[i] = checked.types.intern(Type::Int64);
            }
        }
        assert_eq!(
            lower(&hir, &resolved, &checked, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
}

#[test]
fn validator_rejects_targets_terminators_sources_and_local_ids() {
    let base = pass("func f()->int { let x=1; return x }");
    let mut module = base.clone();
    module.bodies[0].blocks[0].terminator = None;
    error(&module, Violation::MissingTerminator);
    let mut module = base.clone();
    module.bodies[0].blocks[0].terminator.as_mut().unwrap().kind =
        TerminatorKind::Goto(BlockId(999));
    error(&module, Violation::InvalidTarget);
    let mut module = base.clone();
    module.bodies[0].blocks[0]
        .terminator
        .as_mut()
        .unwrap()
        .source
        .hir = nova_hir::HirId(999);
    error(&module, Violation::InvalidSource);
    let mut module = base.clone();
    module.bodies[0].blocks[0].terminator.as_mut().unwrap().kind =
        TerminatorKind::Return(Operand::Place(Place(LocalId(usize::MAX))));
    error(&module, Violation::InvalidLocal);
    let mut module = base;
    module.bodies[0].entry = BlockId(999);
    error(&module, Violation::InvalidBody);
}

#[test]
fn validator_rejects_call_arity_types_operators_and_error_type() {
    let base = pass("func main(){print(\"hi\")}");
    let mut module = base.clone();
    let TerminatorKind::Call { arguments, .. } =
        &mut module.bodies[0].blocks[0].terminator.as_mut().unwrap().kind
    else {
        panic!()
    };
    arguments.clear();
    error(&module, Violation::InvalidArguments);
    let mut module = base.clone();
    let TerminatorKind::Call { arguments, .. } =
        &mut module.bodies[0].blocks[0].terminator.as_mut().unwrap().kind
    else {
        panic!()
    };
    arguments[0] = Operand::Constant(Constant::Bool(true));
    error(&module, Violation::TypeMismatch);
    let mut module = base.clone();
    module.bodies[0].locals[0].ty = Type::Error;
    error(&module, Violation::InvalidType);
    let mut module = base;
    let source = module.bodies[0].source;
    module.bodies[0].blocks[0].statements.push(Statement {
        source,
        kind: StatementKind::Assign(
            Place(LocalId(0)),
            Rvalue::Binary(
                Symbol::AndAnd,
                Operand::Constant(Constant::Bool(true)),
                Operand::Constant(Constant::Bool(false)),
            ),
        ),
    });
    error(&module, Violation::InvalidOperator);
}

#[test]
fn validator_checks_definite_assignment_on_each_join_edge() {
    let mut module = pass("func f(x:bool)->bool {return x && true}");
    let body = &mut module.bodies[0];
    let short = body
        .blocks
        .iter_mut()
        .find(|b| {
            b.statements.iter().any(|s| {
                matches!(
                    s.kind,
                    StatementKind::Assign(_, Rvalue::Use(Operand::Constant(Constant::Bool(false))))
                )
            })
        })
        .unwrap();
    short.statements.clear();
    error(&module, Violation::UninitializedRead);
    let mut module = pass("func f()->int {let x=1;return x}");
    module.bodies[0].blocks[0].statements.clear();
    error(&module, Violation::UninitializedRead);
}

#[test]
fn validator_rejects_duplicate_definitions_and_missing_bodies() {
    let mut module = pass("func f(x:int)->int{return x}");
    module.bodies[0].parameters.push(LocalId(0));
    error(&module, Violation::InvalidLocal);
    let mut module = pass("func f(x:int)->int{return x}");
    module.bodies[0].locals[0].definition = Some(module.callees[0].definition);
    error(&module, Violation::DuplicateDefinition);
    let mut module = pass("func f(){}");
    module.bodies.clear();
    error(&module, Violation::InvalidBody);
    let mut module = pass("func f(){}");
    module.bodies[0].blocks[0].terminator.as_mut().unwrap().kind = TerminatorKind::Unreachable;
    error(&module, Violation::ReachableUnreachable);
}

#[test]
fn validator_handles_cycles_with_entry_boundary_and_call_edge_definitions() {
    let mut module = pass("func f(x:bool)->bool{return x}");
    let body = &mut module.bodies[0];
    let source = body.source;
    body.blocks[0].terminator = Some(Terminator {
        source,
        kind: TerminatorKind::Branch {
            condition: Operand::Place(Place(LocalId(0))),
            then_block: BlockId(0),
            else_block: BlockId(1),
        },
    });
    body.blocks.push(BasicBlockData {
        statements: vec![],
        terminator: Some(Terminator {
            source,
            kind: TerminatorKind::Return(Operand::Place(Place(LocalId(0)))),
        }),
    });
    assert!(validate(&module).is_empty());
    let mut module = pass("func value()->int{return 1} func f()->int{return value()}");
    let index = module
        .bodies
        .iter()
        .position(|b| module.callees[b.callee.0].name == "f")
        .unwrap();
    let body = &mut module.bodies[index];
    let source = body.source;
    let TerminatorKind::Call { destination, .. } = body.blocks[0].terminator.as_ref().unwrap().kind
    else {
        panic!()
    };
    body.blocks[0].statements.push(Statement {
        source,
        kind: StatementKind::Assign(destination, Rvalue::Use(Operand::Place(destination))),
    });
    error(&module, Violation::UninitializedRead);
}

#[test]
fn large_flat_expression_and_many_statements_use_iterative_lowering() {
    let chain = std::iter::repeat("1")
        .take(10000)
        .collect::<Vec<_>>()
        .join("+");
    let module = pass(&format!("func f()->int{{return {chain}}}"));
    assert_eq!(body_named(&module, "f").locals.len(), 9999);
    let mut source = String::from("func f(){");
    for i in 0..10000 {
        source.push_str(&format!("let x{i}={i};"));
    }
    source.push('}');
    let module = pass(&source);
    assert_eq!(body_named(&module, "f").locals.len(), 10000);
}

#[test]
fn all_stage_a_operator_types_and_invalid_return_condition_are_validated() {
    pass("func f(x:int,y:int,b:bool) { let p=+x;let n=-x;let neg=!b;let a=x+y;let s=x-y;let m=x*y;let d=x/y;let r=x%y;let lt=x<y;let le=x<=y;let gt=x>y;let ge=x>=y;let eq=x==y;let ne=x!=y;let be=b==true;let bn=b!=false; }");
    let mut module = pass("func f(x:bool)->int {if x{return 1}else{return 2}}");
    let TerminatorKind::Branch { condition, .. } =
        &mut module.bodies[0].blocks[0].terminator.as_mut().unwrap().kind
    else {
        panic!()
    };
    *condition = Operand::Constant(Constant::Int32(1));
    error(&module, Violation::TypeMismatch);
    let mut module = pass("func f()->int{return 1}");
    module.bodies[0].blocks[0].terminator.as_mut().unwrap().kind =
        TerminatorKind::Return(Operand::Constant(Constant::Bool(true)));
    error(&module, Violation::TypeMismatch);
    let mut module = pass("func main(){print(\"hi\")}");
    let TerminatorKind::Call { callee, .. } =
        &mut module.bodies[0].blocks[0].terminator.as_mut().unwrap().kind
    else {
        panic!()
    };
    *callee = CalleeId(usize::MAX);
    error(&module, Violation::InvalidCallee);
}

#[test]
fn truncated_sources_never_enter_successful_mir_with_recovery() {
    let source="func f(x:bool)->int{if x{return -2147483648}else{return 2}} func main(){print(\"한글 {f(true)}\")}";
    for end in source
        .char_indices()
        .map(|(end, _)| end)
        .chain(std::iter::once(source.len()))
    {
        // A prefix may itself be a valid fragment. Both outcomes must be safe.
        match compile(&source[..end]) {
            Ok(module) => assert!(validate(&module).is_empty()),
            Err(LoweringError::FrontendErrors) => {}
            Err(other) => panic!("prefix {end}: {other:?}"),
        }
    }
}

#[test]
fn p10_checked_cast_mir_is_separate_from_implicit_conversion_and_validated() {
    let source = "func f(a:double)->int64{return a as int8}func main(){let x=128 as int8}";
    let module = pass(source);
    let f = body_named(&module, "f");
    let casts: Vec<_> = f
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .filter(|s| {
            matches!(
                s.kind,
                StatementKind::Assign(_, Rvalue::CheckedCast(_, Type::Int8))
            )
        })
        .collect();
    assert_eq!(casts.len(), 1);
    assert_eq!(
        &source[casts[0].source.span.start()..casts[0].source.span.end()],
        "a as int8"
    );
    assert!(f
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .any(|s| matches!(
            s.kind,
            StatementKind::Assign(_, Rvalue::Widen(_, Type::Int64))
        )));
    assert!(body_named(&module, "main")
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .any(|s| matches!(
            s.kind,
            StatementKind::Assign(_, Rvalue::CheckedCast(_, Type::Int8))
        )));
    for bad in [
        Rvalue::CheckedCast(Operand::Constant(Constant::Bool(true)), Type::Int8),
        Rvalue::CheckedCast(Operand::Constant(Constant::Int32(1)), Type::Bool),
        Rvalue::CheckedCast(Operand::Constant(Constant::Int32(1)), Type::Int64),
    ] {
        let mut module = pass(source);
        module.bodies[0].blocks[0].statements[0].kind =
            StatementKind::Assign(Place(LocalId(1)), bad);
        error(&module, Violation::TypeMismatch);
    }
    let mut module = pass(source);
    module.bodies[0].blocks[0].statements[0].kind = StatementKind::Assign(
        Place(LocalId(1)),
        Rvalue::CheckedCast(Operand::Place(Place(LocalId(1))), Type::Int8),
    );
    error(&module, Violation::UninitializedRead);
}

#[test]
fn p10_analysis_cast_target_value_count_and_operand_context_cannot_be_forged() {
    let mut sources = SourceDatabase::default();
    let file = sources
        .add(
            "test.nova",
            "const C=1 as float;func f(a:double)->int64{return a as int8}".into(),
        )
        .unwrap();
    let lexed = lex(&sources, file).unwrap();
    let parsed = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = resolve(&hir);
    let cast = hir
        .nodes()
        .iter()
        .position(|n| matches!(n.kind, HirKind::Cast { .. }))
        .unwrap();
    let operand = hir.nodes()[cast].children[0].0;
    let target = hir.nodes()[cast].children[1].0;
    let def = resolved
        .definitions
        .iter()
        .position(|d| d.name == "C")
        .unwrap();
    for mutation in 0..6 {
        let mut checked = check(&hir, &resolved).unwrap();
        match mutation {
            0 => checked.type_table[cast] = checked.types.intern(Type::Float64),
            1 => checked.type_table[target] = checked.types.intern(Type::Float64),
            2 => checked.coercions[operand] = Some(checked.types.intern(Type::Float64)),
            3 => {
                checked.const_values[def] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Float(nova_types::FloatValue::from_f32(2.0)),
                    nodes: 2,
                }
            }
            4 => {
                checked.const_values[def] = nova_typecheck::ConstEvaluation::Value {
                    value: nova_types::ConstValue::Float(nova_types::FloatValue::from_f32(1.0)),
                    nodes: 3,
                }
            }
            _ => checked.type_table.clear(),
        }
        assert_eq!(
            lower(&hir, &resolved, &checked, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
}

#[test]
fn p12_aggregate_projection_update_metadata_and_invalid_payloads_are_checked() {
    let source="struct P{var x:int;let y:bool} const C=P(1,true);func f(p:P)->P{return p} func main(){var p=f(C);let old=p;p.x=p.x+1;print(\"{old.x} {p.x}\")}";
    let original = pass(source);
    assert_eq!(original.structs.len(), 1);
    let mut bad = pass(source);
    bad.structs
        .values_mut()
        .next()
        .unwrap()
        .layout
        .as_mut()
        .unwrap()
        .size += 1;
    assert!(!validate(&bad).is_empty());
    let mut bad = pass(source);
    let arg = bad
        .bodies
        .iter_mut()
        .flat_map(|b| &mut b.blocks)
        .filter_map(|b| b.terminator.as_mut())
        .find_map(|t| {
            if let TerminatorKind::Call { arguments, .. } = &mut t.kind {
                arguments
                    .iter_mut()
                    .find(|a| matches!(a, Operand::Constant(Constant::Struct(..))))
            } else {
                None
            }
        })
        .unwrap();
    if let Operand::Constant(Constant::Struct(_, fields)) = arg {
        fields.clear();
    }
    assert!(!validate(&bad).is_empty());
}

#[test]
fn p12_deeply_malformed_aggregate_payload_is_rejected_and_dropped_iteratively() {
    let mut module = pass("struct P{let x:int} const C=P(1);func main(){let p=C}");
    let sid = *module.structs.keys().next().unwrap();
    let mut value = nova_types::ConstValue::Unit;
    for _ in 0..30_000 {
        value = nova_types::ConstValue::Struct(sid, vec![value]);
    }
    let constant = Constant::from_const(&value);
    let body = &mut module.bodies[0];
    body.blocks[0].statements[0].kind =
        StatementKind::Assign(Place(LocalId(0)), Rvalue::Use(Operand::Constant(constant)));
    assert!(!validate(&module).is_empty());
    drop(module);
    drop(value);
}

#[test]
fn p13_tuple_metadata_path_arity_and_kind_corruption_is_rejected() {
    let source="const C=((1,),true);func echo(t:((int,),bool))->((int,),bool){return t} func main(){var t=echo(C);t.0.0=t.0.0+1;print(\"{t.0.0}\")}";
    let original = pass(source);
    assert!(validate(&original).is_empty());
    let mut bad = original.clone();
    let rv = bad
        .bodies
        .iter_mut()
        .flat_map(|b| &mut b.blocks)
        .flat_map(|b| &mut b.statements)
        .find_map(|s| {
            if let StatementKind::Assign(_, Rvalue::Update(_, path, _)) = &mut s.kind {
                Some(path)
            } else {
                None
            }
        })
        .unwrap();
    rv[0].index = 1;
    assert!(!validate(&bad).is_empty());
    let mut bad = original.clone();
    let arg = bad
        .bodies
        .iter_mut()
        .flat_map(|b| &mut b.blocks)
        .filter_map(|b| b.terminator.as_mut())
        .find_map(|t| {
            if let TerminatorKind::Call { arguments, .. } = &mut t.kind {
                arguments
                    .iter_mut()
                    .find(|a| matches!(a, Operand::Constant(Constant::Tuple(..))))
            } else {
                None
            }
        })
        .unwrap();
    if let Operand::Constant(Constant::Tuple(_, fields)) = arg {
        fields.clear();
    }
    assert!(!validate(&bad).is_empty());
    let mut bad = original.clone();
    let local = bad
        .bodies
        .iter_mut()
        .flat_map(|b| &mut b.locals)
        .find(|l| matches!(l.ty, Type::Tuple(_)))
        .unwrap();
    if let Type::Tuple(sid) = local.ty {
        local.ty = Type::Struct(sid);
    }
    assert!(!validate(&bad).is_empty());
    let mut bad = original.clone();
    bad.structs.values_mut().next().unwrap().fields[0].mutable = false;
    assert!(!validate(&bad).is_empty());
}
#[test]
fn p13_malformed_deep_tuple_constant_conversion_and_gate_are_iterative() {
    let mut module = pass("const C=(1,);func main(){let t=C}");
    let sid = *module.structs.keys().next().unwrap();
    let mut value = nova_types::ConstValue::Unit;
    for _ in 0..30000 {
        value = nova_types::ConstValue::Tuple(sid, vec![value]);
    }
    let constant = Constant::from_const(&value);
    module.bodies[0].blocks[0].statements[0].kind =
        StatementKind::Assign(Place(LocalId(0)), Rvalue::Use(Operand::Constant(constant)));
    assert!(!validate(&module).is_empty());
    drop(module);
    drop(value);
}

#[test]
fn p14_match_validation_rejects_inactive_stale_and_redirected_payload_reads() {
    let source =
        "enum E{A(int);B(int);}func f(v:E)->int{match v{E::A(x)=>{return x},E::B(y)=>{return y}}}";
    let module = pass(source);
    assert!(validate(&module).is_empty());
    for change in 0..4 {
        let mut m = pass(source);
        let body = &mut m.bodies[0];
        let bb = body
            .blocks
            .iter()
            .position(|b| {
                b.statements.iter().any(|s| {
                    matches!(
                        s.kind,
                        StatementKind::Assign(_, Rvalue::EnumPayload(_, _, _))
                    )
                })
            })
            .unwrap();
        let stmt = body.blocks[bb]
            .statements
            .iter()
            .position(|s| {
                matches!(
                    s.kind,
                    StatementKind::Assign(_, Rvalue::EnumPayload(_, _, _))
                )
            })
            .unwrap();
        let StatementKind::Assign(_, Rvalue::EnumPayload(Operand::Place(snapshot), v, _)) =
            body.blocks[bb].statements[stmt].kind.clone()
        else {
            panic!()
        };
        match change {
            0 => {
                if let StatementKind::Assign(_, Rvalue::EnumPayload(_, v, _)) =
                    &mut body.blocks[bb].statements[stmt].kind
                {
                    v.index = 1;
                }
            }
            1 => {
                let original = body.blocks[bb].statements[stmt].source;
                body.blocks[bb].statements.insert(
                    stmt,
                    Statement {
                        kind: StatementKind::Assign(
                            snapshot,
                            Rvalue::Use(Operand::Constant(Constant::Enum(
                                nova_types::VariantId {
                                    enumeration: v.enumeration,
                                    index: 1,
                                },
                                vec![Constant::Int32(2)],
                            ))),
                        ),
                        source: original,
                    },
                );
            }
            2 => {
                for block in &mut body.blocks {
                    if let Some(Terminator {
                        kind: TerminatorKind::Match { arms, .. },
                        ..
                    }) = &mut block.terminator
                    {
                        let a = arms[0].1;
                        arms[0].1 = arms[1].1;
                        arms[1].1 = a;
                    }
                }
            }
            _ => {
                if let StatementKind::Assign(_, Rvalue::EnumPayload(receiver, _, _)) =
                    &mut body.blocks[bb].statements[stmt].kind
                {
                    *receiver = Operand::Place(Place(body.parameters[0]));
                }
            }
        }
        assert!(
            validate(&m)
                .iter()
                .any(|e| e.violation == Violation::InactivePayload),
            "change {change}: {:?}",
            validate(&m)
        );
    }
    let mut bad = pass("enum E{A(int);}func f(){let e=E::A(1)}");
    let rv = bad.bodies[0]
        .blocks
        .iter_mut()
        .flat_map(|b| &mut b.statements)
        .find_map(|s| {
            let StatementKind::Assign(_, rv) = &mut s.kind;
            matches!(rv, Rvalue::Enum(..)).then_some(rv)
        })
        .unwrap();
    let Rvalue::Enum(variant, fields) = rv.clone() else {
        panic!()
    };
    *rv = Rvalue::Aggregate(nova_types::StructId(variant.enumeration.0), fields);
    assert!(validate(&bad)
        .iter()
        .any(|e| e.violation == Violation::InvalidType));
    let mut bad = pass(source);
    let rv = bad.bodies[0]
        .blocks
        .iter_mut()
        .flat_map(|b| &mut b.statements)
        .find_map(|s| {
            let StatementKind::Assign(_, rv) = &mut s.kind;
            matches!(rv, Rvalue::EnumPayload(..)).then_some(rv)
        })
        .unwrap();
    let Rvalue::EnumPayload(receiver, variant, at) = rv.clone() else {
        panic!()
    };
    *rv = Rvalue::Project(
        receiver,
        nova_types::FieldId {
            structure: nova_types::StructId(variant.enumeration.0),
            index: at,
        },
    );
    assert!(validate(&bad)
        .iter()
        .any(|e| e.violation == Violation::InvalidType));
    let mut m = pass(source);
    m.enums.values_mut().next().unwrap().variants[0].name = "bad".into();
    assert!(validate(&m)
        .iter()
        .any(|e| e.violation == Violation::InvalidType));
    let mut m = pass(source);
    for b in &mut m.bodies[0].blocks {
        if let Some(Terminator {
            kind: TerminatorKind::Match { arms, .. },
            ..
        }) = &mut b.terminator
        {
            arms.pop();
        }
    }
    assert!(!validate(&m).is_empty());
}
#[test]
fn p14_mixed_enum_constants_convert_validate_and_drop_without_host_recursion() {
    let mut malformed = pass("enum E{A(int);}func main(){let e=E::A(1)}");
    let enumeration = *malformed.enums.keys().next().unwrap();
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(move || {
            let v = nova_types::VariantId {
                enumeration,
                index: 0,
            };
            let mut value = nova_types::ConstValue::Int32(1);
            for _ in 0..30000 {
                value = nova_types::ConstValue::Enum(v, vec![value]);
            }
            let converted = Constant::from_const(&value);
            drop(value);
            let target = malformed.bodies[0]
                .blocks
                .iter_mut()
                .flat_map(|b| &mut b.statements)
                .find_map(|s| {
                    let StatementKind::Assign(_, rv) = &mut s.kind;
                    matches!(rv, Rvalue::Enum(..)).then_some(rv)
                })
                .unwrap();
            *target = Rvalue::Use(Operand::Constant(converted));
            assert!(!validate(&malformed).is_empty());
            drop(malformed);
        })
        .unwrap()
        .join()
        .unwrap();
    pass("enum E{A;B((int,bool));}struct P{var e:E}func f(){var p=P(E::A);p.e=E::B((1,true));let t=(p.e,);match t.0{E::A=>{},E::B(x)=>{print(\"{x.0}\")}}}");
}

#[test]
fn p15_sum_certificate_rejects_family_identity_schema_and_tag_forgery() {
    let source="enum User{Some(int);None;}const C:int?=Option::Some(1);func f(a:int?,b:Result<int,bool>)->int{match a{Option::Some(x)=>{return x},none=>{match b{Result::Success(x)=>{return x},Result::Error(_)=>{return 0}}}}}";
    let m = pass(source);
    assert_eq!(m.sums.len(), 2);
    for change in 0..6 {
        let mut m = pass(source);
        let ids = m.sums.keys().copied().collect::<Vec<_>>();
        match change {
            0 => m.sums.get_mut(&ids[0]).unwrap().family = nova_types::SumFamily::Result,
            1 => m.sums.get_mut(&ids[0]).unwrap().arguments[0] = Type::Bool,
            2 => {
                let key = m.sums[&ids[0]].clone();
                m.sums.insert(ids[1], key);
            }
            3 => {
                m.sums.clear();
            }
            4 => {
                let id = *m.enums.keys().find(|id| !m.sums.contains_key(id)).unwrap();
                let key = m.sums[&ids[0]].clone();
                m.sums.insert(id, key);
            }
            _ => {
                m.enums.get_mut(&ids[0]).unwrap().variants.swap(0, 1);
            }
        }
        assert!(
            validate(&m)
                .iter()
                .any(|e| e.violation == Violation::InvalidType),
            "{change}"
        );
    }
    let origin_source = "func f(x:Option<int>){}";
    let mut forged = pass(origin_source);
    let origin = forged
        .sources
        .iter_mut()
        .find(|s| s.span.start() == origin_source.find("Option<int>").unwrap())
        .unwrap();
    origin.span = nova_source::Span::new(
        origin.span.file(),
        origin.span.start() + 1,
        origin.span.end(),
    )
    .unwrap();
    assert!(validate(&forged)
        .iter()
        .any(|e| e.violation == Violation::InvalidType));
    let mut m = pass(source);
    for body in &mut m.bodies {
        for block in &mut body.blocks {
            if let Some(Terminator {
                kind: TerminatorKind::Match { arms, .. },
                ..
            }) = &mut block.terminator
            {
                let a = arms[0].1;
                arms[0].1 = arms[1].1;
                arms[1].1 = a;
            }
        }
    }
    assert!(!validate(&m).is_empty());
}
#[test]
fn p15_deep_sum_constant_gate_and_drop_are_iterative() {
    let mut m = pass("func main(){let x=Option::Some(1)}");
    let eid = *m.sums.keys().next().unwrap();
    std::thread::Builder::new()
        .stack_size(64 * 1024)
        .spawn(move || {
            let v = nova_types::VariantId {
                enumeration: eid,
                index: 0,
            };
            let mut c = nova_types::ConstValue::Unit;
            for _ in 0..30000 {
                c = nova_types::ConstValue::Enum(v, vec![c]);
            }
            let value = Constant::from_const(&c);
            drop(c);
            let target = m.bodies[0]
                .blocks
                .iter_mut()
                .flat_map(|b| &mut b.statements)
                .find_map(|s| {
                    let StatementKind::Assign(_, rv) = &mut s.kind;
                    matches!(rv, Rvalue::Enum(..)).then_some(rv)
                })
                .unwrap();
            *target = Rvalue::Use(Operand::Constant(value));
            assert!(!validate(&m).is_empty());
            drop(m);
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn p16_try_snapshot_active_payload_and_error_return_certificates_reject_forgery() {
    let source="func leaf(v:Result<bool,bool>)->Result<bool,bool>{return v}func f(r:Result<bool,bool>)->Result<int,bool>{let x=try leaf(r);return Result::Success(1)}";
    let original = pass(source);
    let bi = original
        .bodies
        .iter()
        .position(|b| original.callees[b.callee.0].name == "f")
        .unwrap();
    let dispatch = original.bodies[bi]
        .blocks
        .iter()
        .position(|b| {
            matches!(
                b.terminator.as_ref().unwrap().kind,
                TerminatorKind::Try { .. }
            )
        })
        .unwrap();
    let TerminatorKind::Try {
        snapshot,
        success,
        error,
    } = original.bodies[bi].blocks[dispatch]
        .terminator
        .as_ref()
        .unwrap()
        .kind
    else {
        unreachable!()
    };
    assert!(matches!(
        original.bodies[bi].blocks[error.0]
            .terminator
            .as_ref()
            .unwrap()
            .kind,
        TerminatorKind::Return(_)
    ));
    for forgery in 0..13 {
        let mut m = original.clone();
        let b = &mut m.bodies[bi];
        match forgery {
            0 => {
                b.blocks[dispatch].terminator.as_mut().unwrap().kind = TerminatorKind::Try {
                    snapshot,
                    success: error,
                    error: success,
                };
            }
            1 => {
                b.blocks[dispatch].terminator.as_mut().unwrap().source = b.source;
            }
            2 => {
                b.blocks[error.0].terminator.as_mut().unwrap().kind = TerminatorKind::Goto(success);
            }
            3 => {
                let StatementKind::Assign(_, Rvalue::Enum(_, fields)) =
                    &mut b.blocks[error.0].statements[1].kind
                else {
                    unreachable!()
                };
                fields[0] = Operand::Constant(Constant::Bool(false));
            }
            4 => {
                let StatementKind::Assign(_, Rvalue::Enum(v, _)) =
                    &mut b.blocks[error.0].statements[1].kind
                else {
                    unreachable!()
                };
                v.index = 0;
            }
            5 => {
                let statement = b.blocks[dispatch].statements.last_mut().unwrap();
                let StatementKind::Assign(_, v) = &mut statement.kind;
                *v = Rvalue::Use(Operand::Constant(Constant::Enum(
                    nova_types::VariantId {
                        enumeration: match b.locals[snapshot.0 .0].ty {
                            Type::Enum(e) => e,
                            _ => unreachable!(),
                        },
                        index: 0,
                    },
                    vec![Constant::Bool(true)],
                )));
            }
            6 => {
                let copy = b.blocks[dispatch].statements.last().unwrap().clone();
                b.blocks[success.0].statements.insert(0, copy);
            }
            7 => {
                let StatementKind::Assign(_, Rvalue::EnumPayload(_, v, _)) =
                    &mut b.blocks[success.0].statements[0].kind
                else {
                    unreachable!()
                };
                v.index = 1;
            }
            8 => {
                let StatementKind::Assign(_, Rvalue::EnumPayload(op, _, _)) =
                    &mut b.blocks[error.0].statements[0].kind
                else {
                    unreachable!()
                };
                *op = Operand::Place(Place(b.parameters[0]));
            }
            9 => {
                let block = b
                    .blocks
                    .iter()
                    .find(|b| {
                        matches!(
                            b.terminator.as_ref().unwrap().kind,
                            TerminatorKind::Call { .. }
                        )
                    })
                    .unwrap()
                    .clone();
                b.blocks.push(block);
            }
            10 => b.entry = success,
            11 => {
                b.blocks[dispatch].terminator.as_mut().unwrap().kind =
                    TerminatorKind::Goto(success);
            }
            12 => {
                let StatementKind::Assign(value, _) = b.blocks[success.0].statements[0].kind;
                let source = b.blocks[success.0].statements[0].source;
                b.blocks[success.0].statements.insert(
                    1,
                    Statement {
                        kind: StatementKind::Assign(
                            value,
                            Rvalue::Use(Operand::Constant(Constant::Bool(false))),
                        ),
                        source,
                    },
                );
            }
            _ => unreachable!(),
        }
        assert!(!validate(&m).is_empty(), "accepted forgery {forgery}");
    }
}
#[test]
fn p16_flat_try_cfg_is_iterative_and_nested_success_is_typed() {
    pass("func f(r:Result<Result<int8,()>,()>)->Result<int8,()>{let n=try try r;return Result::Success(n)}");
    pass("func f(r:Result<(),()>)->Result<(),()>{try r;return Result::Success(())}");
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let mut source = String::from("func f(r:Result<int,bool>)->Result<int,bool>{");
            for i in 0..512 {
                source.push_str(&format!("let x{i}=try r;"));
            }
            source.push_str("return Result::Success(0)}");
            let m = pass(&source);
            assert_eq!(
                m.bodies[0]
                    .blocks
                    .iter()
                    .filter(|b| matches!(
                        b.terminator.as_ref().unwrap().kind,
                        TerminatorKind::Try { .. }
                    ))
                    .count(),
                512
            );
        })
        .unwrap()
        .join()
        .unwrap();
}
#[test]
fn p17_public_typed_mapping_forgery_is_rejected_before_mir() {
    let source = "func f(a:int8,b:int8){}func main(){f(b:2,a:1)}";
    let mut db = SourceDatabase::default();
    let file = db.add("api", source.into()).unwrap();
    let l = lex(&db, file).unwrap();
    let p = parse(&db, file, &normalize_ends(&l.tokens)).unwrap();
    let hir = nova_hir::lower(&db, &p.arena, p.root).unwrap();
    let resolved = resolve(&hir);
    let mut checked = check(&hir, &resolved).unwrap();
    let id = checked
        .named_calls
        .iter()
        .position(Option::is_some)
        .unwrap();
    checked.named_calls[id]
        .as_mut()
        .unwrap()
        .parameters
        .swap(0, 1);
    assert_eq!(
        nova_mir::lower(&hir, &resolved, &checked, false),
        Err(nova_mir::LoweringError::InvalidAnalysis)
    );
}

#[test]
fn p17_named_call_order_snapshots_and_same_type_cfg_forgery_gate() {
    let source="func a()->int{ return 1 }func b()->int{ return 2 }func sum(left:int,right:int)->int{return left+left+right}func f()->int{return sum(right:b(),left:a())}";
    let original = pass(source);
    assert_eq!(
        execute(&original, "f", vec![]),
        (
            Constant::Int32(4),
            vec!["b".into(), "a".into(), "sum".into()]
        )
    );
    let body = original
        .bodies
        .iter()
        .position(|b| original.callees[b.callee.0].name == "f")
        .unwrap();
    let block=original.bodies[body].blocks.iter().position(|b|matches!(&b.terminator,Some(Terminator{kind:TerminatorKind::Call {callee,..},..}) if original.callees[callee.0].name=="sum")).unwrap();
    let mut bad = original.clone();
    let TerminatorKind::Call { arguments, .. } = &mut bad.bodies[body].blocks[block]
        .terminator
        .as_mut()
        .unwrap()
        .kind
    else {
        unreachable!()
    };
    arguments.swap(0, 1);
    assert!(!validate(&bad).is_empty());
    let mut bad = original.clone();
    let entry = bad.bodies[body].entry;
    bad.bodies[body].entry = BlockId(block);
    assert!(!validate(&bad).is_empty());
    let mut bad = original.clone();
    let first = bad.bodies[body].blocks[entry.0].terminator.clone();
    bad.bodies[body].blocks[block].terminator = first;
    assert!(!validate(&bad).is_empty());
    let mut bad = original.clone();
    let statement = bad.bodies[body].blocks[block]
        .statements
        .last()
        .unwrap()
        .clone();
    bad.bodies[body].blocks[block].statements.push(statement);
    assert!(!validate(&bad).is_empty());
    let mut bad = original.clone();
    bad.bodies.remove(body);
    assert!(!validate(&bad).is_empty());
    let mut bad = original.clone();
    let (target, at) = bad.bodies[body]
        .blocks
        .iter()
        .enumerate()
        .find_map(|(at, b)| b.statements.first().map(|_| (at, 0)))
        .unwrap();
    let StatementKind::Assign(_, value) = &mut bad.bodies[body].blocks[target].statements[at].kind;
    *value = Rvalue::Use(Operand::Constant(Constant::Int32(9)));
    assert!(!validate(&bad).is_empty());
}
#[test]
fn p17_flat_large_parameter_mapping_uses_bounded_host_stack() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let params = (0..1024)
                .map(|i| format!("p{i}:int"))
                .collect::<Vec<_>>()
                .join(",");
            let args = (0..1024)
                .rev()
                .map(|i| format!("p{i}:{i}"))
                .collect::<Vec<_>>()
                .join(",");
            let source = format!(
                "func many({params})->int{{return p0+p1023}}func f()->int{{return many({args})}}"
            );
            let module = pass(&source);
            assert_eq!(execute(&module, "f", vec![]).0, Constant::Int32(1023));
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn p18_public_default_values_and_omitted_mapping_forgery_are_rejected() {
    let source = "func sum(a:int=99,b:int=98)->int{return a+b}func main(){sum()}";
    for mutation in 0..3 {
        let mut db = SourceDatabase::default();
        let file = db.add("api", source.into()).unwrap();
        let l = lex(&db, file).unwrap();
        let p = parse(&db, file, &normalize_ends(&l.tokens)).unwrap();
        let hir = nova_hir::lower(&db, &p.arena, p.root).unwrap();
        let resolved = resolve(&hir);
        let mut checked = check(&hir, &resolved).unwrap();
        if mutation == 0 {
            let d = checked.defaults.iter_mut().flatten().next().unwrap();
            let nova_typecheck::ConstEvaluation::Value { value, .. } = &mut d.evaluation else {
                unreachable!()
            };
            *value = nova_types::ConstValue::Int32(97);
        } else {
            let c = checked.named_calls.iter_mut().flatten().next().unwrap();
            if mutation == 1 {
                c.defaults.swap(0, 1)
            } else {
                c.defaults[0].initializer = c.defaults[1].initializer;
            }
        }
        assert_eq!(
            lower(&hir, &resolved, &checked, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
}
#[test]
fn p18_defaults_snapshot_order_values_sources_and_cfg_are_certified() {
    let source="func side()->int{return 3}func sum(a:int=99,b:int,c:int=98)->int{return a+b+c}func f()->int{return sum(b:side())}";
    let original = pass(source);
    assert_eq!(
        execute(&original, "f", vec![]),
        (Constant::Int32(200), vec!["side".into(), "sum".into()])
    );
    let body = original
        .bodies
        .iter()
        .position(|b| original.callees[b.callee.0].name == "f")
        .unwrap();
    let block=original.bodies[body].blocks.iter().position(|b|matches!(&b.terminator,Some(Terminator{kind:TerminatorKind::Call{callee,..},..})if original.callees[callee.0].name=="sum")).unwrap();
    let at = original.bodies[body].blocks[block]
        .statements
        .iter()
        .position(|s| {
            matches!(
                s.kind,
                StatementKind::Assign(_, Rvalue::Use(Operand::Constant(Constant::Int32(99))))
            )
        })
        .unwrap();
    assert!(
        original.bodies[body].blocks[block].statements[at]
            .source
            .span
            .start()
            < original.bodies[body].source.span.start()
    );
    for mutation in 0..9 {
        let mut bad = original.clone();
        let b = &mut bad.bodies[body];
        match mutation {
            0 => {
                let TerminatorKind::Call { arguments, .. } =
                    &mut b.blocks[block].terminator.as_mut().unwrap().kind
                else {
                    unreachable!()
                };
                arguments.swap(0, 2);
            }
            1 => {
                let StatementKind::Assign(_, v) = &mut b.blocks[block].statements[at].kind;
                *v = Rvalue::Use(Operand::Constant(Constant::Int32(97)));
            }
            2 => b.blocks[block].statements.swap(at, at + 1),
            3 => {
                b.blocks[block].statements[at].source =
                    b.blocks[block].terminator.as_ref().unwrap().source
            }
            4 => {
                b.blocks[block].statements.remove(at);
            }
            5 => {
                let s = b.blocks[block].statements[at].clone();
                b.blocks[block].statements.push(s);
            }
            6 => b.entry = BlockId(block),
            7 => {
                let StatementKind::Assign(place, _) = b.blocks[block].statements[at].kind;
                b.locals[place.0 .0].source = b.source;
            }
            _ => {
                let s = b.blocks[block].statements[at].clone();
                b.blocks[b.entry.0].statements.push(s);
            }
        }
        assert!(!validate(&bad).is_empty(), "mutation {mutation}");
    }
    let source="func leaf()->Result<int,bool>{return Result::Error(true)}func sum(a:int=99,b:int)->int{return a+b}func f()->Result<int,bool>{let x=sum(b:try leaf());return Result::Success(x)}";
    let mut bad = pass(source);
    let b = bad
        .bodies
        .iter_mut()
        .find(|b| b.source.span.start() == source.find("func f()").unwrap())
        .unwrap();
    let target = b
        .blocks
        .iter()
        .position(|b| {
            matches!(
                b.terminator,
                Some(Terminator {
                    kind: TerminatorKind::Call { .. },
                    ..
                })
            ) && b.statements.iter().any(|s| {
                matches!(
                    s.kind,
                    StatementKind::Assign(_, Rvalue::Use(Operand::Constant(Constant::Int32(99))))
                )
            })
        })
        .unwrap();
    let error = b
        .blocks
        .iter_mut()
        .find(|b| {
            matches!(
                b.terminator,
                Some(Terminator {
                    kind: TerminatorKind::Return(_),
                    ..
                })
            )
        })
        .unwrap();
    error.terminator.as_mut().unwrap().kind = TerminatorKind::Goto(BlockId(target));
    assert!(!validate(&bad).is_empty());
}
#[test]
fn p18_flat_defaults_and_omissions_use_bounded_host_stack() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let params = (0..1024)
                .map(|i| format!("p{i}:int={i}"))
                .collect::<Vec<_>>()
                .join(",");
            let module = pass(&format!(
                "func many({params})->int{{return p0+p1023}}func f()->int{{return many()}}"
            ));
            assert_eq!(execute(&module, "f", vec![]).0, Constant::Int32(1023));
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn p19_range_loop_snapshots_advance_jump_and_mutation_certificates() {
    let source="func end()->int{return 4}func f()->int{var bound=1;var total=0;for i in bound until end(){bound=0;if i==2{continue}total=total+i}loop{total=total+1;break}return total}";
    let original = pass(source);
    assert_eq!(execute(&original, "f", vec![]).0, Constant::Int32(5));
    let b = body_named(&original, "f");
    assert!(b.locals.iter().any(|l| l.definition.is_none()
        && &source[l.source.span.start()..l.source.span.end()] == "bound"));
    let body = original
        .bodies
        .iter()
        .position(|b| original.callees[b.callee.0].name == "f")
        .unwrap();
    let guard = b
        .blocks
        .iter()
        .position(|b| {
            b.statements.iter().any(|s| {
                matches!(
                    s.kind,
                    StatementKind::Assign(_, Rvalue::Binary(Symbol::Less, _, _))
                )
            })
        })
        .unwrap();
    for mutation in 0..8 {
        let mut bad = original.clone();
        let b = &mut bad.bodies[body];
        match mutation {
            0 => b.entry = BlockId(guard),
            1 => {
                let TerminatorKind::Branch {
                    then_block,
                    else_block,
                    ..
                } = &mut b.blocks[guard].terminator.as_mut().unwrap().kind
                else {
                    unreachable!()
                };
                std::mem::swap(then_block, else_block);
            }
            2 => {
                let s = b.blocks[guard].statements.first_mut().unwrap();
                let StatementKind::Assign(_, v) = &mut s.kind;
                *v = Rvalue::Use(Operand::Constant(Constant::Bool(true)));
            }
            3 => {
                let s = b.blocks[guard].statements[0].clone();
                b.blocks[b.entry.0].statements.push(s);
            }
            4 => {
                b.blocks[guard].statements[0].source = b.source;
            }
            5 => {
                let t = b.blocks[guard].terminator.clone();
                b.blocks[b.entry.0].terminator = t;
            }
            6 => b.locals[0].source = b.source,
            _ => {
                let i = b
                    .blocks
                    .iter()
                    .position(|b| {
                        b.statements.iter().any(|s| {
                            matches!(
                                s.kind,
                                StatementKind::Assign(_, Rvalue::Binary(Symbol::Plus, _, _))
                            )
                        })
                    })
                    .unwrap();
                b.blocks[i].statements.clear();
            }
        }
        assert!(!validate(&bad).is_empty(), "mutation {mutation}");
    }
    let original = pass(
        "func f()->int{var n=0;for i in 2147483647 through 2147483647{n=n+1;continue}return n}",
    );
    assert_eq!(execute(&original, "f", vec![]).0, Constant::Int32(1));
    let b = body_named(&original, "f");
    let at = b
        .blocks
        .iter()
        .position(|b| {
            b.statements.iter().any(|s| {
                matches!(
                    s.kind,
                    StatementKind::Assign(_, Rvalue::Binary(Symbol::EqualEqual, _, _))
                )
            })
        })
        .unwrap();
    let mut bad = original.clone();
    let b = &mut bad.bodies[0];
    let TerminatorKind::Branch {
        then_block,
        else_block,
        ..
    } = &mut b.blocks[at].terminator.as_mut().unwrap().kind
    else {
        unreachable!()
    };
    std::mem::swap(then_block, else_block);
    assert!(!validate(&bad).is_empty());
}
#[test]
fn p19_public_range_metadata_and_try_bypass_are_rejected() {
    let mut sources = SourceDatabase::default();
    let file = sources
        .add("api", "func f(){for i in 0 until 3{}}".into())
        .unwrap();
    let lexed = lex(&sources, file).unwrap();
    let p = parse(&sources, file, &normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &p.arena, p.root).unwrap();
    let res = resolve(&hir);
    for mutation in 0..3 {
        let mut bad = check(&hir, &res).unwrap();
        let range = bad.ranges.iter_mut().find(|r| r.is_some()).unwrap();
        match mutation {
            0 => *range = None,
            1 => range.as_mut().unwrap().binder = nova_resolve::DefId(0),
            _ => range.as_mut().unwrap().ty = bad.types.intern(Type::Bool),
        }
        assert_eq!(
            lower(&hir, &res, &bad, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
    let original=pass("func leaf()->Result<int,bool>{return Result::Error(true)}func f()->Result<int,bool>{for i in try leaf() until 3 {}return Result::Success(7)}");
    let mut bad = original.clone();
    let b = bad
        .bodies
        .iter_mut()
        .find(|b| original.callees[b.callee.0].name == "f")
        .unwrap();
    let header = b
        .blocks
        .iter()
        .position(|b| {
            b.statements.iter().any(|s| {
                matches!(
                    s.kind,
                    StatementKind::Assign(_, Rvalue::Binary(Symbol::Less, _, _))
                )
            })
        })
        .unwrap();
    let error = b
        .blocks
        .iter_mut()
        .find(|b| {
            matches!(
                b.terminator,
                Some(Terminator {
                    kind: TerminatorKind::Return(_),
                    ..
                })
            )
        })
        .unwrap();
    error.terminator.as_mut().unwrap().kind = TerminatorKind::Goto(BlockId(header));
    assert!(!validate(&bad).is_empty());
}
#[test]
fn p19_flat_loops_small_host_stack_and_cfg_fixed_point() {
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let m = pass(&format!(
                "func f()->int{{var total=0;{}return total}}",
                "for i in 0 until 1 {total=total+i}loop{break}".repeat(256)
            ));
            assert_eq!(
                execute_with_fuel(&m, "f", vec![], 10000).0,
                Constant::Int32(0)
            );
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn p20_exists_predicate_snapshot_direction_source_effect_and_cfg_gates() {
    let original = pass("func f(value:int?)->bool{print(\"once\");return value exists}");
    let b = body_named(&original, "f");
    let Type::Enum(enumeration) = original.callees[b.callee.0].parameters[0] else {
        panic!()
    };
    for (index, payload, expected) in [
        (0, vec![Constant::Int32(0)], true),
        (0, vec![Constant::Int32(-1)], true),
        (1, vec![], false),
    ] {
        let result = execute(
            &original,
            "f",
            vec![Constant::Enum(
                nova_types::VariantId { enumeration, index },
                payload,
            )],
        );
        assert_eq!(result, (Constant::Bool(expected), vec!["print".into()]));
    }
    assert!(!b
        .blocks
        .iter()
        .flat_map(|b| &b.statements)
        .any(|s| matches!(s.kind, StatementKind::Assign(_, Rvalue::EnumPayload(..)))));
    let dispatch = b
        .blocks
        .iter()
        .position(|b| {
            matches!(
                b.terminator,
                Some(Terminator {
                    kind: TerminatorKind::Match { .. },
                    ..
                })
            )
        })
        .unwrap();
    let body_index = original
        .bodies
        .iter()
        .position(|b| original.callees[b.callee.0].name == "f")
        .unwrap();
    for mutation in 0..8 {
        let mut bad = original.clone();
        let b = &mut bad.bodies[body_index];
        match mutation {
            0 => {
                let TerminatorKind::Match { arms, .. } =
                    &mut b.blocks[dispatch].terminator.as_mut().unwrap().kind
                else {
                    panic!()
                };
                let target = arms[0].1;
                arms[0].1 = arms[1].1;
                arms[1].1 = target;
            }
            1 => {
                let statement = b
                    .blocks
                    .iter_mut()
                    .flat_map(|b| &mut b.statements)
                    .find(|s| {
                        matches!(
                            s.kind,
                            StatementKind::Assign(
                                _,
                                Rvalue::Use(Operand::Constant(Constant::Bool(true)))
                            )
                        )
                    })
                    .unwrap();
                let StatementKind::Assign(_, value) = &mut statement.kind;
                *value = Rvalue::Use(Operand::Constant(Constant::Bool(false)));
            }
            2 => {
                let TerminatorKind::Match { scrutinee, .. } =
                    &mut b.blocks[dispatch].terminator.as_mut().unwrap().kind
                else {
                    panic!()
                };
                *scrutinee = Operand::Constant(Constant::Enum(
                    nova_types::VariantId {
                        enumeration,
                        index: 1,
                    },
                    vec![],
                ));
            }
            3 => b.entry = BlockId(dispatch),
            4 => b.blocks[dispatch].terminator.as_mut().unwrap().source = b.source,
            5 => b.blocks[dispatch].statements.clear(),
            6 => {
                let source = b.source;
                b.locals
                    .iter_mut()
                    .find(|l| l.definition.is_none())
                    .unwrap()
                    .source = source;
            }
            _ => {
                let statement = b.blocks[dispatch].statements.last().unwrap().clone();
                b.blocks[dispatch].statements.push(statement);
            }
        }
        assert!(!validate(&bad).is_empty(), "mutation {mutation}");
    }
    let mut bad = original.clone();
    bad.sums.get_mut(&enumeration).unwrap().family = nova_types::SumFamily::Result;
    assert!(!validate(&bad).is_empty());
}
#[test]
fn p20_exists_public_typed_forgery_try_bypass_and_flat_stack() {
    let mut db = SourceDatabase::default();
    let file = db
        .add(
            "api",
            "func f(value:int?)->bool{return value exists}".into(),
        )
        .unwrap();
    let l = lex(&db, file).unwrap();
    let p = parse(&db, file, &normalize_ends(&l.tokens)).unwrap();
    let h = nova_hir::lower(&db, &p.arena, p.root).unwrap();
    let res = resolve(&h);
    let id = h
        .nodes()
        .iter()
        .position(|n| matches!(n.kind, nova_hir::HirKind::Exists { .. }))
        .unwrap();
    for mutation in 0..2 {
        let mut c = check(&h, &res).unwrap();
        if mutation == 0 {
            c.type_table[id] = c.types.intern(Type::Int32)
        } else {
            c.sums.values_mut().next().unwrap().family = nova_types::SumFamily::Result
        }
        assert_eq!(
            lower(&h, &res, &c, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
    let original=pass("func leaf()->Result<int?,bool>{return Result::Error(true)}func f()->Result<bool,bool>{let b=(try leaf()) exists;return Result::Success(b)}");
    let mut bad = original.clone();
    let b = bad
        .bodies
        .iter_mut()
        .find(|b| original.callees[b.callee.0].name == "f")
        .unwrap();
    let target = b
        .blocks
        .iter()
        .find_map(|b| {
            if let Some(Terminator {
                kind: TerminatorKind::Try { success, .. },
                ..
            }) = b.terminator
            {
                Some(success)
            } else {
                None
            }
        })
        .unwrap();
    let error = b
        .blocks
        .iter_mut()
        .find(|b| {
            matches!(
                b.terminator,
                Some(Terminator {
                    kind: TerminatorKind::Return(_),
                    ..
                })
            )
        })
        .unwrap();
    error.terminator.as_mut().unwrap().kind = TerminatorKind::Goto(target);
    assert!(!validate(&bad).is_empty());
    std::thread::Builder::new()
        .stack_size(256 * 1024)
        .spawn(|| {
            let m = pass(&format!(
                "func f(value:int?){{{}}}",
                (0..512)
                    .map(|i| format!("let b{i}=value exists;"))
                    .collect::<String>()
            ));
            assert_eq!(
                body_named(&m, "f")
                    .blocks
                    .iter()
                    .filter(|b| matches!(
                        b.terminator,
                        Some(Terminator {
                            kind: TerminatorKind::Match { .. },
                            ..
                        })
                    ))
                    .count(),
                512
            );
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn p21_alias_canonical_mir_has_no_runtime_alias_declaration_or_effect() {
    let m=pass("type A=int8;type Maybe=A?;type Reply=Result<Maybe,bool>;const C:A=7;func f(x:A=C)->Reply{return Result::Success(Option::Some(x))}func main(){let a:Maybe=none;let b=a exists;match f(){Result::Success(v)=>{print(\"{v exists}\")},Result::Error(_)=>{}}}");
    assert_eq!(m.bodies.len(), 2);
    assert_eq!(m.callees.len(), 3);
    assert_eq!(m.sums.len(), 2);
    for body in &m.bodies {
        assert!(body
            .locals
            .iter()
            .all(|l| !matches!(l.ty, Type::Error | Type::Function)));
    }
    assert!(validate(&m).is_empty());
}
#[test]
fn p21_alias_checked_and_resolution_metadata_forgeries_are_rejected() {
    let mut db = SourceDatabase::default();
    let f = db
        .add("a.nova", "type A=int8;func main(){let a:A=7}".into())
        .unwrap();
    let l = lex(&db, f).unwrap();
    let p = parse(&db, f, &normalize_ends(&l.tokens)).unwrap();
    let h = nova_hir::lower(&db, &p.arena, p.root).unwrap();
    let r = resolve(&h);
    let c = check(&h, &r).unwrap();
    assert!(!c.has_errors());
    let def = *c.aliases.keys().next().unwrap();
    for kind in 0..3 {
        let mut c = check(&h, &r).unwrap();
        let wrong = c.types.intern(Type::Bool);
        match kind {
            0 => {
                c.aliases.insert(def, wrong);
            }
            1 => c.definition_types[def] = wrong,
            _ => {
                c.aliases.clear();
            }
        }
        assert_eq!(
            lower(&h, &r, &c, false),
            Err(LoweringError::InvalidAnalysis)
        );
    }
    let mut bad = resolve(&h);
    bad.definitions[def].kind = nova_resolve::DefinitionKind::Enum(h.items().next().unwrap());
    assert_eq!(
        check(&h, &bad),
        Err(nova_typecheck::CheckError::InvalidResolution)
    );
}
