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
                    Rvalue::Binary(op @ (Symbol::Less | Symbol::EqualEqual), left, right) => {
                        let (Constant::Int32(left), Constant::Int32(right)) =
                            (operand(left, &locals), operand(right, &locals))
                        else {
                            panic!("int")
                        };
                        Constant::Bool(if *op == Symbol::Less {
                            left < right
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
            }
        }
    }
    let result = run(
        module,
        body_named(module, name),
        args,
        &mut calls,
        &mut 1000,
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
