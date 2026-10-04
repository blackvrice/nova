use nova_codegen::*;
use nova_source::SourceDatabase;

fn mir() -> nova_mir::Module {
    let mut sources = SourceDatabase::default();
    let file = sources.add("test.nova", "func f(){}".into()).unwrap();
    let lexed = nova_lexer::lex(&sources, file).unwrap();
    let parsed =
        nova_parser::parse(&sources, file, &nova_lexer::normalize_ends(&lexed.tokens)).unwrap();
    let hir = nova_hir::lower(&sources, &parsed.arena, parsed.root).unwrap();
    let resolved = nova_resolve::resolve(&hir);
    let checked = nova_typecheck::check(&hir, &resolved).unwrap();
    nova_mir::lower(&hir, &resolved, &checked, false).unwrap()
}
#[test]
fn unit_accepts_only_valid_mir_and_retains_immutable_provenance() {
    let input = mir();
    let unit = CodegenUnit::new(input.clone()).unwrap();
    assert_eq!(unit.mir(), &input);
    assert_eq!(unit, unit.clone());
    let mut bad = input;
    bad.bodies[0].blocks[0].terminator = None;
    assert!(matches!(
        CodegenUnit::new(bad),
        Err(CodegenError::InvalidMir(_))
    ));
}
#[test]
fn target_names_and_error_categories_are_backend_neutral() {
    assert_eq!(
        TargetSpec::WindowsX64Msvc.triple(),
        "x86_64-pc-windows-msvc"
    );
    assert_eq!(TargetSpec::LinuxX64Gnu.triple(), "x86_64-unknown-linux-gnu");
    assert_eq!(TargetSpec::LinuxArm64.triple(), "aarch64-unknown-linux-gnu");
    assert_eq!(TargetSpec::WindowsArm64.triple(), "aarch64-pc-windows-msvc");
    assert!(CodegenError::MissingTool("missing tool".into())
        .to_string()
        .contains("missing tool"));
}
