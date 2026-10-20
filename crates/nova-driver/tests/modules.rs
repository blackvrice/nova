use nova_codegen::{CodegenError, CodegenUnit};
use nova_driver::{load, Loaded};
use nova_hir::{HirKind, SourceOrigin};
use nova_resolve::{DefinitionKind, Resolution};
use nova_typecheck::ConstEvaluation;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
fn directory() -> PathBuf {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/module-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&path).unwrap();
    path
}
fn fixture(files: &[(&str, &str)]) -> Loaded {
    let root = directory();
    for &(path, text) in files {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
    load(&root.join("main.nova"), Some(&root)).unwrap()
}
fn analyzed(loaded: &Loaded) -> (nova_resolve::Resolved, nova_typecheck::Checked) {
    assert!(loaded.diagnostics.is_empty(), "{:?}", loaded.diagnostics);
    let module = loaded.module.as_ref().unwrap();
    let resolved = nova_resolve::resolve(module);
    let checked = nova_typecheck::check(module, &resolved).unwrap();
    (resolved, checked)
}
fn unit(loaded: &Loaded) -> CodegenUnit {
    let (resolved, checked) = analyzed(loaded);
    assert!(!checked.has_errors(), "{:?}", checked.diagnostics);
    CodegenUnit::new(
        nova_mir::lower(loaded.module.as_ref().unwrap(), &resolved, &checked, false).unwrap(),
    )
    .unwrap()
}
fn codes(checked: &nova_typecheck::Checked) -> Vec<String> {
    checked
        .diagnostics
        .iter()
        .map(|d| d.code.to_string())
        .collect()
}

#[test]
fn cyclic_functions_alias_original_definitions_and_preserve_file_origins() {
    let loaded = fixture(&[
        (
            "main.nova",
            include_str!("../../../docs/development-v0.1/module-proposal-fixtures/main.nova"),
        ),
        (
            "math.nova",
            include_str!("../../../docs/development-v0.1/module-proposal-fixtures/math.nova"),
        ),
    ]);
    let module = loaded.module.as_ref().unwrap();
    assert_eq!(
        module
            .units()
            .iter()
            .map(|u| u.path.as_str())
            .collect::<Vec<_>>(),
        ["main", "math"]
    );
    assert_eq!(module.edges().len(), 3);
    assert!(module.nodes().iter().all(|n| matches!(n.origin, SourceOrigin::FileSource(f, _) | SourceOrigin::FileImplicitReturn(f, _) if f == n.span.file())));
    for node in module.nodes() {
        for child in &node.children {
            assert_eq!(node.span.file(), module.nodes()[child.0].span.file());
        }
    }
    let (resolved, checked) = analyzed(&loaded);
    assert!(!checked.has_errors());
    let plus = module
        .nodes()
        .iter()
        .position(|n| matches!(n.kind, HirKind::Name(s) if module.symbol(s)==Some("plus")))
        .unwrap();
    let add = resolved
        .definitions
        .iter()
        .position(|d| d.name == "add")
        .unwrap();
    assert_eq!(
        resolved.references[plus],
        Some(Resolution::Definition(nova_resolve::DefId(add)))
    );
    assert!(!resolved.definitions.iter().any(|d| d.name == "plus"));
    assert_eq!(unit(&loaded).mir().bodies.len(), 3);
}

#[test]
fn imports_deduplicate_files_and_sort_relative_paths_independent_of_discovery_order() {
    let root = directory();
    for (path, source) in [
        ("a/z.nova", "const Z=1"),
        ("a0.nova", "const Z=2"),
        ("unused.nova", "not valid Nova!"),
    ] {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, source).unwrap();
    }
    let main = root.join("main.nova");
    std::fs::write(
        &main,
        "use a0::Z as two;use a::z::Z as one;use a::z::Z as same;func main(){let z=one+two+same}",
    )
    .unwrap();
    let first = load(&main, Some(&root)).unwrap();
    let module = first.module.as_ref().unwrap();
    assert_eq!(
        module
            .units()
            .iter()
            .map(|u| (&*u.path, u.file.as_u32()))
            .collect::<Vec<_>>(),
        [("main", 0), ("a::z", 1), ("a0", 2)]
    );
    let first_unit = unit(&first);
    std::fs::write(
        &main,
        "use a::z::Z as same;use a::z::Z as one;use a0::Z as two;func main(){let z=one+two+same}",
    )
    .unwrap();
    let second = load(&main, Some(&root)).unwrap();
    assert_eq!(
        second
            .module
            .as_ref()
            .unwrap()
            .units()
            .iter()
            .map(|u| (&*u.path, u.file.as_u32()))
            .collect::<Vec<_>>(),
        [("main", 0), ("a::z", 1), ("a0", 2)]
    );
    assert_eq!(first_unit.mir().callees, unit(&second).mir().callees);
    assert_eq!(
        load(&main, None).unwrap().module.as_ref().unwrap().dump(),
        second.module.as_ref().unwrap().dump()
    );
}

#[test]
fn visibility_private_self_access_and_failed_import_cascade() {
    let loaded = fixture(&[
        ("main.nova", "use lib::hidden;func main(){let x=hidden}"),
        ("lib.nova", "private const hidden=1"),
    ]);
    let (resolved, checked) = analyzed(&loaded);
    assert_eq!(codes(&checked), ["N2004"]);
    let d = &resolved.diagnostics[0];
    assert_eq!(d.primary.span.file().as_u32(), 0);
    assert_eq!(d.secondary[0].span.file().as_u32(), 1);
    assert!(nova_mir::lower(loaded.module.as_ref().unwrap(), &resolved, &checked, false).is_err());
    let loaded=fixture(&[("main.nova","use lib::a;use lib::b;use lib::c;func main(){let v=a+b+c}"),("lib.nova","const a=1;internal const b=2;public const c=3;use lib::hidden as local;private const hidden=4;func f()->int{return local}")]);
    assert!(unit(&loaded).executable_entry().is_ok());
}

#[test]
fn duplicate_alias_diagnostics_point_to_both_bindings_and_later_own_declaration() {
    for main in [
        "use lib::x as y;use lib::x as y;func main(){}",
        "use lib::x as y;const y=2;func main(){}",
        "const y=2;use lib::x as y;func main(){}",
    ] {
        let loaded = fixture(&[("main.nova", main), ("lib.nova", "const x=1")]);
        let (resolved, checked) = analyzed(&loaded);
        assert_eq!(codes(&checked), ["N2002"]);
        let d = &resolved.diagnostics[0];
        assert_eq!(d.primary.span.file().as_u32(), 0);
        assert_eq!(d.secondary[0].span.file().as_u32(), 0);
        assert_eq!(loaded.sources.slice(d.primary.span).unwrap(), "y");
        assert_eq!(loaded.sources.slice(d.secondary[0].span).unwrap(), "y");
        assert!(d.primary.span.start() > d.secondary[0].span.start());
    }
}

#[test]
fn missing_direct_items_and_reexports_have_one_primary_error() {
    for (main, lib, expected) in [
        (
            "use lib::absent as x;func main(){let y=x}",
            "const a=1",
            "N2001",
        ),
        (
            "use lib::x;func main(){let y=x}",
            "use other::a as x",
            "N1102",
        ),
    ] {
        let loaded = fixture(&[
            ("main.nova", main),
            ("lib.nova", lib),
            ("other.nova", "const a=1"),
        ]);
        let (_, checked) = analyzed(&loaded);
        assert_eq!(codes(&checked), [expected]);
    }
}

#[test]
fn print_functions_shadow_per_module_const_aliases_are_rejected_and_locals_shadow_imports() {
    for main in [
        "use lib::f as print;func main(){print(1)}",
        "func print(x:int){};use lib::f;func main(){print(1);f(2)}",
        "use lib::C;func main(){let C=C;let x=C}",
    ] {
        let loaded = fixture(&[
            ("main.nova", main),
            ("lib.nova", "const C=7;func f(x:int){print(\"builtin\")}"),
        ]);
        assert!(unit(&loaded).executable_entry().is_ok());
    }
    let loaded = fixture(&[
        ("main.nova", "use lib::C as print;func main(){print(\"x\")}"),
        ("lib.nova", "const C=7"),
    ]);
    assert_eq!(codes(&analyzed(&loaded).1), ["N2002"]);
}

#[test]
fn signatures_and_unused_function_errors_keep_cross_file_labels() {
    let loaded=fixture(&[("main.nova","use lib::F;use lib::C;use lib::B;use lib::U;use lib::values;func main(){let s=values(F,C,B,U);print(s)}"),("lib.nova",r#"const F:double=7 as double;const C='🙂';const B=true;const U=();func values(x:double,c:char,b:bool,u:())->string{return "{x} {c} {b}"}"#)]);
    assert!(unit(&loaded).executable_entry().is_ok());

    let loaded = fixture(&[
        ("main.nova", "use lib::f;func main(){f(true)}"),
        ("lib.nova", "func f(x:int){}"),
    ]);
    let (_, checked) = analyzed(&loaded);
    assert_eq!(codes(&checked), ["N2101"]);
    assert_eq!(checked.diagnostics[0].primary.span.file().as_u32(), 0);
    assert!(checked.diagnostics[0]
        .secondary
        .iter()
        .any(|l| l.span.file().as_u32() == 1));
    let loaded = fixture(&[
        ("main.nova", "use lib::ok;func main(){}"),
        ("lib.nova", "func ok(){};func unused(){let x=missing}"),
    ]);
    let (_, checked) = analyzed(&loaded);
    assert_eq!(codes(&checked), ["N2001"]);
    assert_eq!(checked.diagnostics[0].primary.span.file().as_u32(), 1);
}

#[test]
fn cross_file_static_const_cycles_include_skipped_rhs_and_error_locations() {
    let loaded = fixture(&[
        ("main.nova", "use lib::B;const A=false&&B;func main(){}"),
        ("lib.nova", "use main::A;const B=A"),
    ]);
    let (_, checked) = analyzed(&loaded);
    assert_eq!(codes(&checked), ["N3202"]);
    let d = &checked.diagnostics[0];
    assert!(std::iter::once(&d.primary)
        .chain(&d.secondary)
        .any(|l| l.span.file().as_u32() == 0));
    assert!(std::iter::once(&d.primary)
        .chain(&d.secondary)
        .any(|l| l.span.file().as_u32() == 1));
    let loaded = fixture(&[
        ("main.nova", "use lib::A;const B=A;func main(){}"),
        ("lib.nova", "const A=300 as uint8"),
    ]);
    let (_, checked) = analyzed(&loaded);
    assert_eq!(codes(&checked), ["N3201"]);
    assert_eq!(checked.diagnostics[0].primary.span.file().as_u32(), 1);
}

#[test]
fn imported_const_is_cached_once_and_each_initializer_has_its_own_budget() {
    let chain = std::iter::repeat("1")
        .take(5000)
        .collect::<Vec<_>>()
        .join("+");
    let main = "use lib::A;use lib::A as same;const B=A;func main(){const C=same}";
    let loaded = fixture(&[
        ("main.nova", main),
        ("lib.nova", &format!("const A=({chain})")),
    ]);
    let (resolved, checked) = analyzed(&loaded);
    assert!(!checked.has_errors());
    for (name, nodes) in [("A", 10000), ("B", 1), ("C", 1)] {
        let id = resolved
            .definitions
            .iter()
            .position(|d| d.name == name)
            .unwrap();
        assert!(matches!(checked.const_values[id],ConstEvaluation::Value {nodes:n,..} if n==nodes));
    }
    let loaded = fixture(&[
        ("main.nova", main),
        ("lib.nova", &format!("const A=({chain}+1)")),
    ]);
    let (_, checked) = analyzed(&loaded);
    assert_eq!(codes(&checked), ["N3202"]);
    assert_eq!(checked.diagnostics[0].primary.span.file().as_u32(), 1);
}

#[test]
fn entry_is_directly_declared_in_entry_file_and_same_named_module_functions_coexist() {
    for main in [
        "use lib::main as helper;func root(){}",
        "use lib::helper as main;func root(){}",
    ] {
        let loaded = fixture(&[
            ("main.nova", main),
            ("lib.nova", "func main(){};func helper(){}"),
        ]);
        let codegen = unit(&loaded);
        let error = codegen.executable_entry().unwrap_err();
        assert!(
            matches!(error,CodegenError::InvalidEntry {code:2001,source:Some(s),..} if s.span.file().as_u32()==0)
        );
    }
    let loaded = fixture(&[
        (
            "main.nova",
            "use lib::main as helper;func main(){let n=helper(2)}",
        ),
        ("lib.nova", "func main(x:int)->int{return x}"),
    ]);
    let codegen = unit(&loaded);
    assert!(codegen.executable_entry().is_ok());
    let mut forged = codegen.mir().clone();
    let entry = codegen.executable_entry().unwrap().0;
    forged.callees[entry].name = "fake".into();
    assert!(CodegenUnit::new(forged).is_err());
    let fragment = fixture(&[
        ("main.nova", "use lib::main as helper;func f(){}"),
        ("lib.nova", "func main(){}"),
    ]);
    let mut forged = unit(&fragment).mir().clone();
    let root = forged
        .bodies
        .iter()
        .find(|b| b.source.span.file().as_u32() == 0)
        .unwrap()
        .callee
        .0;
    forged.callees[root].name = "main".into();
    assert!(CodegenUnit::new(forged).is_err());
    assert_eq!(
        codegen
            .mir()
            .callees
            .iter()
            .filter(|c| c.name == "main")
            .count(),
        2
    );
}

#[test]
fn resolution_type_const_and_entry_provenance_tampering_is_rejected() {
    let loaded = fixture(&[
        ("main.nova", "use lib::f as alias;func main(){alias()}"),
        ("lib.nova", "public func f(){}"),
    ]);
    let module = loaded.module.as_ref().unwrap();
    let (mut resolved, checked) = analyzed(&loaded);
    let scope = resolved
        .scopes
        .iter_mut()
        .find(|s| s.definitions.contains_key("alias"))
        .unwrap();
    scope
        .definitions
        .insert("alias".into(), nova_resolve::DefId(0));
    assert_eq!(
        nova_typecheck::check(module, &resolved),
        Err(nova_typecheck::CheckError::InvalidResolution)
    );
    assert!(nova_mir::lower(module, &resolved, &checked, false).is_err());
    let (resolved, mut checked) = analyzed(&loaded);
    let call = checked.calls.iter_mut().find(|c| c.is_some()).unwrap();
    *call = Some(nova_resolve::DefId(0));
    assert!(nova_mir::lower(module, &resolved, &checked, false).is_err());
    let codegen = unit(&loaded);
    let mut mir = codegen.mir().clone();
    let index = mir.entry_source().hir.0;
    mir.sources[index] = mir.sources[0];
    assert!(CodegenUnit::new(mir).is_err());
    let invalid_entry = fixture(&[("main.nova", "func main()->int{return 1}")]);
    let mut forged = unit(&invalid_entry).mir().clone();
    let main = forged
        .callees
        .iter()
        .position(|c| Some(c.definition) == forged.entry_definition())
        .unwrap();
    forged.callees[main].return_type = forged.callees[0].return_type;
    for block in &mut forged.bodies[0].blocks {
        if let Some(terminator) = &mut block.terminator {
            if matches!(terminator.kind, nova_mir::TerminatorKind::Return(_)) {
                terminator.kind = nova_mir::TerminatorKind::Return(nova_mir::Operand::Constant(
                    nova_mir::Constant::Unit,
                ));
            }
        }
    }
    assert!(CodegenUnit::new(forged).is_err());
    assert!(resolved
        .definitions
        .iter()
        .any(|d| matches!(d.kind, DefinitionKind::Function(_))));
}

#[test]
fn discovery_missing_case_utf8_and_root_errors_are_source_errors() {
    for main in [
        "use missing::x;func main(){}",
        "use nested::missing::x;func main(){}",
        "use Lib::x;func main(){}",
    ] {
        let loaded = fixture(&[("main.nova", main), ("lib.nova", "const x=1")]);
        assert!(loaded.module.is_none());
        assert_eq!(loaded.diagnostics[0].code.to_string(), "N2001");
    }
    let loaded = fixture(&[
        ("main.nova", "use lib::x;use LIB::x as y;func main(){}"),
        ("lib.nova", "const x=1"),
    ]);
    assert_eq!(loaded.diagnostics[0].code.to_string(), "N2002");
    let root = directory();
    std::fs::write(root.join("main.nova"), "use lib::x;func main(){}").unwrap();
    std::fs::write(root.join("lib.nova"), [0xff]).unwrap();
    let loaded = load(&root.join("main.nova"), None).unwrap();
    assert_eq!(loaded.diagnostics[0].code.to_string(), "N8001");
    assert!(loaded.diagnostics[0].message.contains("byte 0"));
    loaded
        .sources
        .slice(loaded.diagnostics[0].primary.span)
        .unwrap();
    let other = directory();
    assert_eq!(
        load(&root.join("main.nova"), Some(&other))
            .unwrap_err()
            .code,
        8001
    );
    assert_eq!(
        load(&root.join("absent.nova"), None).unwrap_err().code,
        8001
    );
    std::fs::write(root.join("main.nova"), [0xff]).unwrap();
    assert!(load(&root.join("main.nova"), None)
        .unwrap_err()
        .message
        .contains("byte 0"));
}

#[test]
fn graph_chain_is_iterative_at_1024_modules_and_rejects_1025() {
    let root = directory();
    for i in 0..1024 {
        let name = if i == 0 {
            "main".into()
        } else {
            format!("m{i}")
        };
        let text = if i == 1023 {
            "const C=1".into()
        } else {
            format!("use m{}::C as next;const C=1", i + 1)
        };
        std::fs::write(root.join(format!("{name}.nova")), text).unwrap();
    }
    let loaded = load(&root.join("main.nova"), None).unwrap();
    assert_eq!(loaded.module.as_ref().unwrap().units().len(), 1024);
    assert!(loaded.diagnostics.is_empty());
    std::fs::write(root.join("m1023.nova"), "use m1024::C;func f(){}").unwrap();
    std::fs::write(root.join("m1024.nova"), "const C=1").unwrap();
    let loaded = load(&root.join("main.nova"), None).unwrap();
    assert!(loaded.module.is_none());
    assert_eq!(loaded.diagnostics.len(), 1);
    assert_eq!(loaded.diagnostics[0].code.to_string(), "N8901");
    assert!(loaded
        .sources
        .file(loaded.diagnostics[0].primary.span.file())
        .unwrap()
        .path()
        .ends_with("m1023.nova"));
}

#[test]
fn hard_link_aliases_are_physical_module_collisions() {
    let root = directory();
    std::fs::write(
        root.join("main.nova"),
        "use a::C as first;use b::C as second;func main(){}",
    )
    .unwrap();
    std::fs::write(root.join("a.nova"), "const C=1").unwrap();
    std::fs::hard_link(root.join("a.nova"), root.join("b.nova")).unwrap();
    let loaded = load(&root.join("main.nova"), None).unwrap();
    assert!(loaded.module.is_none());
    assert_eq!(loaded.diagnostics[0].code.to_string(), "N2002");
    assert_eq!(loaded.diagnostics[0].secondary.len(), 1);
}

#[cfg(windows)]
#[test]
fn junctions_cannot_escape_root_or_register_the_same_file_twice() {
    fn junction(link: &Path, target: &Path) {
        let result = std::process::Command::new("cmd")
            .args(["/c", "mklink", "/J"])
            .arg(link)
            .arg(target)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    let root = directory();
    let outside = directory();
    std::fs::write(outside.join("lib.nova"), "const C=1").unwrap();
    junction(&root.join("escape"), &outside);
    std::fs::write(root.join("main.nova"), "use escape::lib::C;func main(){}").unwrap();
    let loaded = load(&root.join("main.nova"), None).unwrap();
    assert_eq!(loaded.diagnostics[0].code.to_string(), "N8001");
    assert!(loaded.diagnostics[0].message.contains("outside"));
    let actual = root.join("actual");
    std::fs::create_dir(&actual).unwrap();
    std::fs::write(actual.join("lib.nova"), "const C=1").unwrap();
    junction(&root.join("alias"), &actual);
    std::fs::write(
        root.join("main.nova"),
        "use actual::lib::C as x;use alias::lib::C as y;func main(){}",
    )
    .unwrap();
    let loaded = load(&root.join("main.nova"), None).unwrap();
    assert_eq!(loaded.diagnostics[0].code.to_string(), "N2002");
}

#[cfg(unix)]
#[test]
fn symlinks_cannot_escape_source_root() {
    let root = directory();
    let outside = directory();
    std::fs::write(outside.join("lib.nova"), "const C=1").unwrap();
    std::os::unix::fs::symlink(&outside, root.join("escape")).unwrap();
    std::fs::write(root.join("main.nova"), "use escape::lib::C;func main(){}").unwrap();
    let loaded = load(&root.join("main.nova"), None).unwrap();
    assert_eq!(loaded.diagnostics[0].code.to_string(), "N8001");
}

#[test]
fn p16_two_file_alias_fixture_reaches_verified_mir() {
    let loaded = fixture(&[
        (
            "main.nova",
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/main.nova"),
        ),
        (
            "effects.nova",
            include_str!("../../../docs/development-v0.1/try-proposal-fixtures/effects.nova"),
        ),
    ]);
    let unit = unit(&loaded);
    assert!(unit
        .mir()
        .bodies
        .iter()
        .flat_map(|b| &b.blocks)
        .any(|b| matches!(
            b.terminator.as_ref().unwrap().kind,
            nova_mir::TerminatorKind::Try { .. }
        )));
}

#[test]
fn p22_method_contract_bundle_and_exact_diagnostic_spans() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/method-proposal-fixtures");
    for name in ["main.nova", "newline_and_self.nova", "method_abort.nova"] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        let unit = unit(&loaded);
        assert!(nova_mir::validate(unit.mir()).is_empty());
        let module = loaded.module.as_ref().unwrap();
        for method in module
            .semantic_items()
            .into_iter()
            .filter(|id| module.method_owner(*id).is_some())
        {
            let source = module.method_source(method).unwrap();
            assert!(matches!(
                module.nodes()[source.owner.0].kind,
                HirKind::Struct { .. }
            ));
            assert_eq!(loaded.sources.slice(source.keyword).unwrap(), "func");
            assert_eq!(loaded.sources.slice(source.left_paren).unwrap(), "(");
            assert_eq!(loaded.sources.slice(source.right_paren).unwrap(), ")");
            assert_eq!(source.keyword.file(), module.nodes()[method.0].span.file());
        }
    }
    for (name, code, start, end) in [
        ("missing_receiver.nova", "N1102", 62, 63),
        ("typed_receiver.nova", "N1102", 66, 67),
        ("change_receiver.nova", "N1102", 62, 68),
        ("take_receiver.nova", "N1102", 62, 66),
        ("receiver_not_first.nova", "N1102", 62, 63),
        ("duplicate_method.nova", "N2002", 87, 90),
        ("field_method_collision.nova", "N2002", 70, 73),
        ("mutate_self.nova", "N3004", 81, 85),
        ("private_method.nova", "N2004", 98, 104),
        ("unknown_method.nova", "N2001", 113, 120),
        ("non_struct_receiver.nova", "N2101", 101, 108),
        ("bound_method.nova", "N1102", 111, 116),
        ("const_method_call.nova", "N3201", 106, 116),
        ("default_method_call.nova", "N3201", 111, 121),
        ("unknown_label.nova", "N2201", 125, 129),
        ("self_label.nova", "N2201", 117, 121),
        ("missing_argument.nova", "N2201", 117, 124),
        ("duplicate_self_parameter.nova", "N2002", 67, 71),
        ("return_type_mismatch.nova", "N2101", 80, 84),
        ("enum_method.nova", "N1102", 53, 57),
    ] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        let diagnostic = if let Some(module) = &loaded.module {
            let resolved = nova_resolve::resolve(module);
            let checked = nova_typecheck::check(module, &resolved).unwrap();
            resolved
                .diagnostics
                .into_iter()
                .chain(checked.diagnostics)
                .next()
                .unwrap()
        } else {
            loaded.diagnostics[0].clone()
        };
        assert_eq!(diagnostic.code.to_string(), code, "{name}");
        assert_eq!(
            (
                diagnostic.primary.span.start(),
                diagnostic.primary.span.end()
            ),
            (start, end),
            "{name}: {diagnostic:?}"
        );
        loaded.sources.slice(diagnostic.primary.span).unwrap();
    }
}
#[test]
fn p22_opaque_factory_methods_use_original_owner_and_declaration_scope() {
    let loaded = fixture(&[
        ("main.nova", "use lib::factory;func main(){let p=factory();print(p.text())}"),
        ("lib.nova", "private struct Hidden{private let x:int;public func text(self)->string{return self.secret()};private func secret(self)->string{return \"{self.x}\"}}public func factory()->Hidden{return Hidden(7)}"),
    ]);
    unit(&loaded);
}

#[test]
fn p23_bundle_constructor_fixture_exact_spans_and_cascade_suppression() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/struct-named-arguments-proposal-fixtures");
    for name in [
        "main.nova",
        "multiline.nova",
        "value_namespace.nova",
        "constructor_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    for (name, code, start, end, forbidden) in [
        (
            "unknown_label.nova",
            "N2201",
            140,
            145,
            &["N2101", "N2102"] as &[&str],
        ),
        (
            "unicode_unknown.nova",
            "N2201",
            148,
            154,
            &["N2101", "N2102"] as &[&str],
        ),
        (
            "method_label.nova",
            "N2201",
            140,
            143,
            &[] as &[&str] as &[&str],
        ),
        (
            "duplicate_label.nova",
            "N2201",
            144,
            145,
            &[] as &[&str] as &[&str],
        ),
        (
            "positional_collision.nova",
            "N2201",
            142,
            143,
            &[] as &[&str] as &[&str],
        ),
        (
            "positional_after_named.nova",
            "N2201",
            140,
            141,
            &[] as &[&str] as &[&str],
        ),
        (
            "missing_field.nova",
            "N2201",
            130,
            140,
            &[] as &[&str] as &[&str],
        ),
        (
            "excess_positional.nova",
            "N2201",
            140,
            141,
            &[] as &[&str] as &[&str],
        ),
        (
            "mapped_literal_range.nova",
            "N2102",
            144,
            147,
            &[] as &[&str] as &[&str],
        ),
        (
            "wrong_type.nova",
            "N2101",
            142,
            146,
            &[] as &[&str] as &[&str],
        ),
        (
            "nominal_mismatch.nova",
            "N2101",
            97,
            103,
            &[] as &[&str] as &[&str],
        ),
        ("private_field.nova", "N2004", 36, 51, &["N2201"] as &[&str]),
        ("alias_head.nova", "N1102", 147, 161, &["N2201"] as &[&str]),
        (
            "enum_label.nova",
            "N2201",
            54,
            59,
            &[] as &[&str] as &[&str],
        ),
        (
            "option_label.nova",
            "N2201",
            44,
            49,
            &[] as &[&str] as &[&str],
        ),
        (
            "result_label.nova",
            "N2201",
            52,
            57,
            &[] as &[&str] as &[&str],
        ),
        (
            "builtin_label.nova",
            "N2201",
            18,
            23,
            &[] as &[&str] as &[&str],
        ),
        (
            "value_shadow.nova",
            "N2101",
            142,
            147,
            &["N2201"] as &[&str],
        ),
        (
            "unresolved_head.nova",
            "N2001",
            18,
            25,
            &["N2201", "N2101"] as &[&str],
        ),
        (
            "const_runtime_call.nova",
            "N3201",
            169,
            178,
            &[] as &[&str] as &[&str],
        ),
        (
            "skipped_const_runtime_call.nova",
            "N3201",
            180,
            189,
            &[] as &[&str] as &[&str],
        ),
        (
            "default_runtime_call.nova",
            "N3201",
            170,
            179,
            &[] as &[&str] as &[&str],
        ),
        (
            "undefined_value.nova",
            "N2001",
            142,
            149,
            &["N2101", "N2102"] as &[&str],
        ),
        (
            "empty_method_label.nova",
            "N2201",
            72,
            77,
            &[] as &[&str] as &[&str],
        ),
    ] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        let (_, checked) = analyzed(&loaded);
        let diagnostics = &checked.diagnostics;
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && d.primary.span.start() == start
                && d.primary.span.end() == end),
            "{name}: {diagnostics:?}"
        );
        assert!(
            forbidden
                .iter()
                .all(|code| diagnostics.iter().all(|d| d.code.to_string() != *code)),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            loaded.sources.slice(d.primary.span).unwrap();
        }
    }
}
#[test]
fn p23_private_constructor_precedes_mapping_and_original_imported_field_identity() {
    let loaded = fixture(&[
        (
            "main.nova",
            "use lib::P as Q;func main(){let p=Q(y:2,x:1);print(p.text())}",
        ),
        (
            "lib.nova",
            r#"public struct P{public let x:int8;public func text(self)->string{return "{self.x}/{self.y}"};public let y:int16}"#,
        ),
    ]);
    let (_, c) = analyzed(&loaded);
    let m = c.named_constructors.iter().flatten().next().unwrap();
    assert_eq!(m.fields.iter().map(|f| f.index).collect::<Vec<_>>(), [1, 0]);
    assert!(m.fields.iter().all(|f| f.structure == m.structure));
    unit(&loaded);
    let loaded = fixture(&[
        ("main.nova", "use lib::P;func main(){let p=P(other:1000)}"),
        ("lib.nova", "public struct P{private let x:int8}"),
    ]);
    let (_, c) = analyzed(&loaded);
    assert_eq!(codes(&c), ["N2004"]);
    assert!(c.diagnostics[0].notes.is_empty());
    let loaded = fixture(&[
        (
            "main.nova",
            "use lib::factory;func main(){let p=factory();print(p.get())}",
        ),
        (
            "lib.nova",
            r#"private struct P{private let x:int8;public func get(self)->string{return "{self.x}"}}public func factory()->P{return P(x:7)}"#,
        ),
    ]);
    unit(&loaded);
}

#[test]
fn p24_fixture_bundle_and_all_proposed_positive_units() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/nested-pattern-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for name in [
        "main.nova",
        "partial_overlap.nova",
        "union_coverage.nova",
        "bare_binder.nova",
        "nested_abort.nova",
    ] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        unit(&loaded);
        for node in loaded.module.as_ref().unwrap().nodes() {
            loaded.sources.slice(node.span).unwrap();
        }
    }
}
#[test]
fn p24_fixed_diagnostics_utf8_primary_and_cascade_suppression() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/nested-pattern-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for (name, code, start, end, forbidden) in [
        (
            "tuple_arity.nova",
            "N2201",
            77,
            84,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "tuple_on_bool.nova",
            "N2101",
            69,
            81,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "bool_on_integer.nova",
            "N2101",
            74,
            78,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "unit_on_bool.nova",
            "N2101",
            74,
            76,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "variant_mismatch.nova",
            "N2101",
            88,
            103,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "unknown_variant.nova",
            "N2001",
            96,
            103,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "payload_arity.nova",
            "N2201",
            88,
            105,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "nullary_parentheses.nova",
            "N2201",
            106,
            120,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "duplicate_binder.nova",
            "N2002",
            83,
            86,
            &["N3101", "N3102"] as &[&str],
        ),
        ("missing_nested.nova", "N3101", 48, 53, &[] as &[&str]),
        ("duplicate_nested.nova", "N3102", 99, 111, &[] as &[&str]),
        ("union_unreachable.nova", "N3102", 130, 138, &[] as &[&str]),
        (
            "grouped_pattern.nova",
            "N1102",
            82,
            88,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "literal_pattern.nova",
            "N1102",
            74,
            75,
            &["N3101", "N3102"] as &[&str],
        ),
        ("immutable_binder.nova", "N3004", 55, 56, &[] as &[&str]),
        ("binder_escape.nova", "N2001", 67, 68, &[] as &[&str]),
        ("root_scalar.nova", "N2101", 33, 34, &[] as &[&str]),
        (
            "alias_head.nova",
            "N1102",
            86,
            94,
            &["N3101", "N3102"] as &[&str],
        ),
    ] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        // P30 historical override; original source and metadata remain unchanged.
        if name == "root_scalar.nova" {
            unit(&loaded);
            continue;
        }
        // P32 override: original INT rejection source/metadata remain frozen.
        if name == "literal_pattern.nova" {
            unit(&loaded);
            continue;
        }
        let mut diagnostics = loaded.diagnostics.clone();
        if let Some(hir) = &loaded.module {
            let resolved = nova_resolve::resolve(hir);
            let checked = nova_typecheck::check(hir, &resolved).unwrap();
            diagnostics.extend(checked.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && d.primary.span.start() == start
                && d.primary.span.end() == end),
            "{name}: {diagnostics:?}"
        );
        assert!(
            forbidden
                .iter()
                .all(|code| diagnostics.iter().all(|d| d.code.to_string() != *code)),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            loaded.sources.slice(d.primary.span).unwrap();
            for note in d.secondary {
                loaded.sources.slice(note.span).unwrap();
            }
        }
    }
}

#[test]
fn p25_fixed_guard_negative_utf8_spans_and_cascade_boundaries() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/guard-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for (name, code, start, end, forbidden) in [
        (
            "guard_integer.nova",
            "N2101",
            96,
            99,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "guard_unit.nova",
            "N2101",
            91,
            93,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "guard_unknown.nova",
            "N2001",
            91,
            98,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "duplicate_binder.nova",
            "N2002",
            92,
            95,
            &["N3101", "N3102"] as &[&str],
        ),
        ("immutable_binder.nova", "N3004", 106, 107, &[] as &[&str]),
        ("binder_escape.nova", "N2001", 123, 124, &[] as &[&str]),
        ("guarded_wildcard.nova", "N3101", 72, 77, &[] as &[&str]),
        (
            "literal_true_coverage.nova",
            "N3101",
            72,
            77,
            &[] as &[&str],
        ),
        ("covered_guard.nova", "N3102", 92, 96, &[] as &[&str]),
        (
            "union_covered_guard.nova",
            "N3102",
            123,
            131,
            &[] as &[&str],
        ),
        ("missing_return.nova", "N3003", 79, 141, &[] as &[&str]),
        (
            "try_unit.nova",
            "N3002",
            152,
            155,
            &["N3101", "N3102"] as &[&str],
        ),
        (
            "try_error_type.nova",
            "N2101",
            169,
            172,
            &["N3101", "N3102"] as &[&str],
        ),
        ("newline_before_if.nova", "N1101", 87, 88, &[] as &[&str]),
        ("double_guard.nova", "N1101", 96, 98, &[] as &[&str]),
        (
            "root_scalar.nova",
            "N2101",
            78,
            79,
            &["N3101", "N3102"] as &[&str],
        ),
    ] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        // P30 historical override; original source and metadata remain unchanged.
        if name == "root_scalar.nova" {
            unit(&loaded);
            continue;
        }
        let mut diagnostics = loaded.diagnostics.clone();
        if let Some(hir) = &loaded.module {
            let resolved = nova_resolve::resolve(hir);
            let checked = nova_typecheck::check(hir, &resolved).unwrap();
            diagnostics.extend(resolved.diagnostics);
            diagnostics.extend(checked.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && d.primary.span.start() == start
                && d.primary.span.end() == end),
            "{name}: {diagnostics:?}"
        );
        assert!(
            forbidden
                .iter()
                .all(|code| diagnostics.iter().all(|d| d.code.to_string() != *code)),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            loaded.sources.slice(d.primary.span).unwrap();
            for s in d.secondary {
                loaded.sources.slice(s.span).unwrap();
            }
        }
    }
}
#[test]
fn p25_guard_bundle_positive_and_runtime_units() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/guard-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for name in [
        "main.nova",
        "literal_guards.nova",
        "multiline.nova",
        "binder_scope.nova",
        "guard_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
}

#[test]
fn p26_fixed_destructuring_negative_utf8_spans_and_cascades() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/destructuring-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for (name, code, start, end, forbidden) in [
        ("tuple_arity.nova", "N2201", 76, 81, &["N3103"] as &[&str]),
        ("tuple_type.nova", "N2101", 76, 81, &["N3103"] as &[&str]),
        ("unit_type.nova", "N2101", 76, 78, &["N3103"] as &[&str]),
        (
            "duplicate_binder.nova",
            "N2002",
            81,
            84,
            &["N3103"] as &[&str],
        ),
        ("same_scope.nova", "N2002", 85, 86, &["N3103"] as &[&str]),
        ("refutable_bool.nova", "N3103", 76, 84, &[] as &[&str]),
        ("refutable_enum.nova", "N3103", 93, 100, &[] as &[&str]),
        ("refutable_option.nova", "N3103", 76, 91, &[] as &[&str]),
        ("refutable_result.nova", "N3103", 76, 94, &[] as &[&str]),
        ("refutable_nested.nova", "N3103", 101, 118, &[] as &[&str]),
        (
            "unknown_variant.nova",
            "N2001",
            88,
            95,
            &["N3103"] as &[&str],
        ),
        (
            "variant_mismatch.nova",
            "N2101",
            94,
            98,
            &["N3103"] as &[&str],
        ),
        (
            "payload_arity.nova",
            "N2201",
            91,
            100,
            &["N3103"] as &[&str],
        ),
        (
            "nullary_parentheses.nova",
            "N2201",
            85,
            91,
            &["N3103"] as &[&str],
        ),
        ("immutable_binder.nova", "N3004", 94, 95, &[] as &[&str]),
        ("binder_escape.nova", "N2001", 111, 112, &[] as &[&str]),
        (
            "initializer_self.nova",
            "N2001",
            86,
            87,
            &["N3103"] as &[&str],
        ),
        ("var_pattern.nova", "N1102", 76, 77, &[] as &[&str]),
        ("const_pattern.nova", "N1102", 78, 79, &[] as &[&str]),
        ("guard_pattern.nova", "N1102", 85, 87, &[] as &[&str]),
        (
            "annotation_range.nova",
            "N2102",
            98,
            101,
            &["N3103"] as &[&str],
        ),
        ("alias_head.nova", "N1102", 104, 112, &["N3103"] as &[&str]),
    ] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        // P27/P28 extend var/const; original P26 bytes/metadata remain historical.
        if matches!(name, "var_pattern.nova" | "const_pattern.nova") {
            unit(&loaded);
            continue;
        }
        let mut diagnostics = loaded.diagnostics.clone();
        if let Some(hir) = &loaded.module {
            let resolved = nova_resolve::resolve(hir);
            let checked = nova_typecheck::check(hir, &resolved).unwrap();
            diagnostics.extend(resolved.diagnostics);
            diagnostics.extend(checked.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && d.primary.span.start() == start
                && d.primary.span.end() == end),
            "{name}: {diagnostics:?}"
        );
        assert!(
            forbidden
                .iter()
                .all(|code| diagnostics.iter().all(|d| d.code.to_string() != *code)),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            loaded.sources.slice(d.primary.span).unwrap();
            for s in d.secondary {
                loaded.sources.slice(s.span).unwrap();
            }
        }
    }
}
#[test]
fn p26_positive_bundle_and_abort_codegen_units() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/destructuring-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for name in [
        "main.nova",
        "discard_effect.nova",
        "opaque_copy.nova",
        "annotation_context.nova",
        "initializer_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
}

#[test]
fn p27_fixed_var_destructuring_negative_utf8_spans_and_cascades() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/var-destructuring-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for (name, code, start, end, forbidden) in [
        ("let_immutable.nova", "N3004", 79, 80, &[] as &[&str]),
        (
            "duplicate_binder.nova",
            "N2002",
            71,
            74,
            &["N3103"] as &[&str],
        ),
        ("same_scope.nova", "N2002", 75, 76, &["N3103"] as &[&str]),
        (
            "initializer_self.nova",
            "N2001",
            73,
            74,
            &["N3103"] as &[&str],
        ),
        ("binder_escape.nova", "N2001", 94, 95, &["N3103"] as &[&str]),
        ("refutable_bool.nova", "N3103", 66, 74, &[] as &[&str]),
        ("refutable_enum.nova", "N3103", 84, 91, &[] as &[&str]),
        ("refutable_option.nova", "N3103", 67, 82, &[] as &[&str]),
        ("refutable_result.nova", "N3103", 67, 85, &[] as &[&str]),
        ("tuple_arity.nova", "N2201", 66, 71, &["N3103"] as &[&str]),
        ("tuple_type.nova", "N2101", 66, 71, &["N3103"] as &[&str]),
        (
            "annotation_range.nova",
            "N2102",
            88,
            91,
            &["N3103"] as &[&str],
        ),
        ("const_pattern.nova", "N1102", 68, 69, &["N3103"] as &[&str]),
        ("immutable_field.nova", "N3004", 104, 105, &[] as &[&str]),
        ("immutable_path.nova", "N3004", 121, 125, &[] as &[&str]),
        ("private_field.nova", "N2004", 107, 112, &[] as &[&str]),
        ("binding_guard.nova", "N1102", 72, 74, &["N3103"] as &[&str]),
        (
            "unknown_initializer.nova",
            "N2001",
            75,
            82,
            &["N3103"] as &[&str],
        ),
    ] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        // P28 permits local compound const; preserve P27 historical fixture.
        if name == "const_pattern.nova" {
            unit(&loaded);
            continue;
        }
        let mut diagnostics = loaded.diagnostics.clone();
        if let Some(hir) = &loaded.module {
            let resolved = nova_resolve::resolve(hir);
            let checked = nova_typecheck::check(hir, &resolved).unwrap();
            diagnostics.extend(resolved.diagnostics);
            diagnostics.extend(checked.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && d.primary.span.start() == start
                && d.primary.span.end() == end),
            "{name}: {diagnostics:?}"
        );
        assert!(
            forbidden
                .iter()
                .all(|code| diagnostics.iter().all(|d| d.code.to_string() != *code)),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            loaded.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                loaded.sources.slice(label.span).unwrap();
            }
        }
    }
}
#[test]
fn p27_var_bundle_positive_runtime_units_and_historical_var_acceptance() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/var-destructuring-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for name in [
        "main.nova",
        "scalars.nova",
        "mixed_paths.nova",
        "initializer_abort.nova",
        "binder_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    let old = root
        .join("../destructuring-proposal-fixtures/var_pattern.nova")
        .canonicalize()
        .unwrap();
    unit(&load(&old, Some(old.parent().unwrap())).unwrap());
}

#[test]
fn p28_fixed_const_destructuring_negative_utf8_spans_and_cascades() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/const-destructuring-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for (name, code, start, end, forbidden) in [
        ("immutable_binder.nova", "N3004", 80, 81, &[] as &[&str]),
        (
            "duplicate_binder.nova",
            "N2002",
            73,
            76,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        (
            "same_scope.nova",
            "N2002",
            77,
            78,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        (
            "initializer_self.nova",
            "N2001",
            75,
            76,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        (
            "binder_escape.nova",
            "N2001",
            96,
            97,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        (
            "tuple_arity.nova",
            "N2201",
            68,
            73,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        (
            "annotation_range.nova",
            "N2102",
            90,
            93,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        (
            "refutable_bool.nova",
            "N3103",
            68,
            76,
            &["N3201", "N3202"] as &[&str],
        ),
        (
            "refutable_enum.nova",
            "N3103",
            86,
            93,
            &["N3201", "N3202"] as &[&str],
        ),
        (
            "refutable_option.nova",
            "N3103",
            69,
            84,
            &["N3201", "N3202"] as &[&str],
        ),
        ("user_call.nova", "N3201", 85, 91, &[] as &[&str]),
        ("parameter_ref.nova", "N3201", 77, 78, &[] as &[&str]),
        ("let_ref.nova", "N3201", 83, 84, &[] as &[&str]),
        (
            "unknown_initializer.nova",
            "N2001",
            77,
            84,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        ("overflow_const.nova", "N3201", 92, 95, &[] as &[&str]),
        ("discard_div0.nova", "N3201", 76, 79, &[] as &[&str]),
        ("unreachable_const.nova", "N3201", 83, 86, &[] as &[&str]),
        (
            "global_pattern.nova",
            "N1102",
            56,
            64,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        (
            "visible_global_pattern.nova",
            "N1102",
            63,
            71,
            &["N3103", "N3201", "N3202"] as &[&str],
        ),
        ("const_try.nova", "N3201", 165, 168, &[] as &[&str]),
    ] {
        let loaded = load(&root.join(name), Some(&root)).unwrap();
        // P29 explicit override: original P28 negative source/metadata remain frozen.
        if matches!(name, "global_pattern.nova" | "visible_global_pattern.nova") {
            unit(&loaded);
            continue;
        }
        let mut diagnostics = loaded.diagnostics.clone();
        if let Some(hir) = &loaded.module {
            let resolved = nova_resolve::resolve(hir);
            let checked = nova_typecheck::check(hir, &resolved).unwrap();
            diagnostics.extend(resolved.diagnostics);
            diagnostics.extend(checked.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && d.primary.span.start() == start
                && d.primary.span.end() == end),
            "{name}: {diagnostics:?}"
        );
        assert!(
            forbidden
                .iter()
                .all(|code| diagnostics.iter().all(|d| d.code.to_string() != *code)),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            loaded.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                loaded.sources.slice(label.span).unwrap();
            }
        }
    }
}
#[test]
fn p28_const_bundle_positive_runtime_and_historical_const_units() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/const-destructuring-proposal-fixtures")
        .canonicalize()
        .unwrap();
    for name in [
        "main.nova",
        "short_circuit.nova",
        "opaque_const.nova",
        "runtime_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    for dir in [
        "destructuring-proposal-fixtures",
        "var-destructuring-proposal-fixtures",
    ] {
        let old = root
            .join(format!("../{dir}/const_pattern.nova"))
            .canonicalize()
            .unwrap();
        unit(&load(&old, Some(old.parent().unwrap())).unwrap());
    }
}

#[test]
fn p29_component_import_visibility_alias_and_default_materialization() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/global-const-destructuring-proposal-fixtures");
    let loaded = load(&root.join("main.nova"), Some(&root)).unwrap();
    unit(&loaded);
    let (r, c) = analyzed(&loaded);
    let count = r.definitions.iter().position(|d| d.name == "n").unwrap();
    assert!(matches!(
        r.definitions[count].kind,
        DefinitionKind::GlobalConst(_)
    ));
    assert!(matches!(
        c.const_values[count],
        ConstEvaluation::Value { .. }
    ));
    for (name, code) in [
        ("cycle.nova", "N3202"),
        ("private_import.nova", "N2004"),
        ("duplicate.nova", "N2002"),
        ("discard_failure.nova", "N3201"),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        let (_, c) = analyzed(&l);
        assert_eq!(codes(&c), [code]);
        let expected = match name {
            "cycle.nova" => (14, 15),
            "discard_failure.nova" => (12, 15),
            "private_import.nova" => (0, 19),
            _ => (8, 9),
        };
        assert_eq!(
            (
                c.diagnostics[0].primary.span.start(),
                c.diagnostics[0].primary.span.end()
            ),
            expected
        );
        for d in c.diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}
#[test]
fn p29_cross_file_group_scc_invalidates_components_and_suppresses_dependents() {
    let loaded = fixture(&[
        (
            "main.nova",
            "use other::q;public const(a,b)=(1,q);const DEP=a+1;func main(){}",
        ),
        ("other.nova", "use main::b;public const(p,q)=(b,2)"),
    ]);
    let (r, c) = analyzed(&loaded);
    assert_eq!(codes(&c), ["N3202"]);
    for (i, d) in r.definitions.iter().enumerate().filter(|(_, d)| d.constant) {
        assert!(
            matches!(
                c.const_values[i],
                ConstEvaluation::Failed {
                    code: 3202,
                    nodes: 0
                } | ConstEvaluation::Invalid
            ),
            "{}",
            d.name
        );
    }
    assert_eq!(
        c.const_destructuring_values
            .iter()
            .filter(|v| matches!(
                v,
                ConstEvaluation::Failed {
                    code: 3202,
                    nodes: 0
                }
            ))
            .count(),
        2
    );
}
#[test]
fn p29_internal_components_import_and_private_same_module_reference() {
    let loaded = fixture(&[
        ("main.nova", "use other::n as N;func main(){let x=N}"),
        (
            "other.nova",
            "private const(secret,_)=(3,());const(n,_)=(secret+1,())",
        ),
    ]);
    unit(&loaded);
    let loaded = fixture(&[
        (
            "main.nova",
            "use other::n;use other::n as alias;func main(){let x=n+alias}",
        ),
        ("other.nova", "public const(n,_)=(4,())"),
    ]);
    unit(&loaded);
}

#[test]
fn p30_copy_match_bundle_import_alias_private_fields_and_negative_spans() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/copy-value-match-proposal-fixtures");
    let loaded = load(&root.join("main.nova"), Some(&root)).unwrap();
    unit(&loaded);
    for (name, code) in [
        ("guard_only.nova", "N3101"),
        ("after_binder.nova", "N3102"),
        ("binder_mutation.nova", "N3004"),
        ("binder_escape.nova", "N2001"),
        ("wrong_bool_pattern.nova", "N2101"),
        ("wrong_tuple_pattern.nova", "N2101"),
        ("literal_pattern.nova", "N1102"),
        ("char_pattern.nova", "N1102"),
        ("string_scrutinee.nova", "N2101"),
        ("non_bool_guard.nova", "N2101"),
        ("missing_return.nova", "N3003"),
        ("private_field.nova", "N2004"),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        // P31 historical override: original P30 char literal rejection remains frozen.
        if name == "char_pattern.nova" {
            unit(&l);
            continue;
        }
        // P32 override: original INT rejection source/metadata remain frozen.
        if name == "literal_pattern.nova" {
            unit(&l);
            continue;
        }
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code),
            "{name}: {diagnostics:?}"
        );
        let expected = match name {
            "guard_only.nova" => (12, 17),
            "after_binder.nova" => (26, 27),
            "binder_mutation.nova" => (24, 25),
            "binder_escape.nova" => (32, 33),
            "wrong_bool_pattern.nova" => (20, 24),
            "wrong_tuple_pattern.nova" => (20, 25),
            "literal_pattern.nova" => (20, 21),
            "char_pattern.nova" => (25, 31),
            "string_scrutinee.nova" => (18, 24),
            "non_bool_guard.nova" => (25, 26),
            "missing_return.nova" => (18, 34),
            "private_field.nova" => (53, 54),
            _ => unreachable!(),
        };
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}
#[test]
fn p30_private_copy_struct_whole_binder_is_allowed_without_field_access() {
    unit(&fixture(&[
        (
            "main.nova",
            "use other::make;func main(){match make(){copy=>{}}}",
        ),
        (
            "other.nova",
            "private struct S{private let x:int8}public func make()->S{return S(7)}",
        ),
    ]));
    let old = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/enum-proposal-fixtures/invalid_scrutinee.nova");
    unit(&load(&old, None).unwrap());
}

#[test]
fn p31_char_pattern_bundle_alias_nested_guard_and_diagnostic_sources() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/char-pattern-proposal-fixtures");
    for name in ["main.nova", "controls.nova", "guard_skipped.nova"] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    for (name, code) in [
        ("incomplete.nova", "N3101"),
        ("duplicate_escape.nova", "N3102"),
        ("after_wildcard.nova", "N3102"),
        ("wrong_type.nova", "N2101"),
        ("wrong_nested_type.nova", "N2101"),
        ("let_refutable.nova", "N3103"),
        ("var_refutable.nova", "N3103"),
        ("const_refutable.nova", "N3103"),
        ("global_refutable.nova", "N3103"),
        ("guard_only.nova", "N3101"),
        ("non_bool_guard.nova", "N2101"),
        ("bad_scalar.nova", "N1002"),
        ("integer_pattern.nova", "N1102"),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        // P32 override: original INT rejection source/metadata remain frozen.
        if name == "integer_pattern.nova" {
            unit(&l);
            continue;
        }
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        let expected = match name {
            "incomplete.nova" => (12, 17),
            "duplicate_escape.nova" => (30, 38),
            "after_wildcard.nova" => (28, 31),
            "wrong_type.nova" => (20, 23),
            "wrong_nested_type.nova" => (28, 31),
            "let_refutable.nova" => (15, 22),
            "var_refutable.nova" => (15, 22),
            "const_refutable.nova" => (17, 24),
            "global_refutable.nova" => (5, 12),
            "guard_only.nova" => (12, 17),
            "non_bool_guard.nova" => (29, 30),
            "bad_scalar.nova" => (23, 31),
            "integer_pattern.nova" => (20, 21),
            _ => unreachable!(),
        };
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p32_integer_pattern_fixed_bundle_boundaries_full_domain_and_errors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/integer-pattern-proposal-fixtures");
    for name in [
        "main.nova",
        "boundaries.nova",
        "full_uint8.nova",
        "guard_skipped.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    for (name, code) in [
        ("incomplete.nova", "N3101"),
        ("duplicate_radix.nova", "N3102"),
        ("duplicate_zero.nova", "N3102"),
        ("after_wildcard.nova", "N3102"),
        ("wrong_type.nova", "N2101"),
        ("too_large.nova", "N2102"),
        ("too_negative.nova", "N2102"),
        ("unsigned_negative.nova", "N2102"),
        ("uint64_overflow.nova", "N2102"),
        ("let_refutable.nova", "N3103"),
        ("var_refutable.nova", "N3103"),
        ("const_refutable.nova", "N3103"),
        ("global_refutable.nova", "N3103"),
        ("guard_only.nova", "N3101"),
        ("non_bool_guard.nova", "N2101"),
        ("plus_pattern.nova", "N1102"),
        ("float_pattern.nova", "N1102"),
        ("minus_without_int.nova", "N1102"),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        let expected = match name {
            "incomplete.nova" => (12, 17),
            "duplicate_radix.nova" => (26, 29),
            "duplicate_zero.nova" => (34, 36),
            "after_wildcard.nova" => (26, 27),
            "wrong_type.nova" => (23, 24),
            "too_large.nova" => (28, 31),
            "too_negative.nova" => (28, 32),
            "unsigned_negative.nova" => (29, 31),
            "uint64_overflow.nova" => (30, 50),
            "let_refutable.nova" => (15, 20),
            "var_refutable.nova" => (15, 20),
            "const_refutable.nova" => (17, 22),
            "global_refutable.nova" => (5, 10),
            "guard_only.nova" => (12, 17),
            "non_bool_guard.nova" => (25, 26),
            "plus_pattern.nova" => (20, 21),
            "float_pattern.nova" => (22, 25),
            "minus_without_int.nova" => (21, 23),
            _ => unreachable!(),
        };
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p33_struct_pattern_fixed_bundle_source_origins_and_code_specific_spans() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/struct-pattern-proposal-fixtures");
    for name in [
        "main.nova",
        "private_local.nova",
        "alias_annotation.nova",
        "guard_skipped.nova",
        "guard_abort.nova",
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        unit(&l);
    }
    for (name, code, expected) in [
        ("private.nova", "N2004", (65, 71)),
        ("import_fail.nova", "N2001", (0, 20)),
        ("incomplete.nova", "N3101", (50, 55)),
        ("duplicate.nova", "N3102", (83, 95)),
        ("nominal.nova", "N2101", (108, 113)),
        ("arity.nova", "N2201", (69, 76)),
        ("too_many.nova", "N2201", (54, 65)),
        ("alias_head.nova", "N1102", (85, 90)),
        ("enum_head.nova", "N2101", (86, 92)),
        ("unknown.nova", "N2001", (69, 81)),
        ("duplicate_binder.nova", "N2002", (61, 62)),
        ("immutable.nova", "N3004", (80, 81)),
        ("component_type.nova", "N2101", (74, 78)),
        ("let_refutable.nova", "N3103", (54, 66)),
        ("var_refutable.nova", "N3103", (54, 66)),
        ("const_refutable.nova", "N3103", (56, 68)),
        ("global_refutable.nova", "N3103", (44, 56)),
        ("guard_only.nova", "N3101", (50, 55)),
        ("const_failure.nova", "N3201", (62, 65)),
        ("const_cycle.nova", "N3202", (84, 85)),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p33_struct_patterns_import_identity_private_fields_and_exported_const_binders() {
    let l = fixture(&[
        (
            "main.nova",
            "use values::S as Imported;use values::N;func main(){let Imported(n)=Imported(N)}",
        ),
        (
            "values.nova",
            "public struct S{public let n:int}public const S(N)=S(8)",
        ),
    ]);
    let m = unit(&l);
    assert!(nova_mir::validate(m.mir()).is_empty());
    let l = fixture(&[
        (
            "main.nova",
            "use values::S;use values::make;func main(){match make(){S(_)=>{}}}",
        ),
        (
            "values.nova",
            "public struct S{private let n:int}public func make()->S{return S(1)}",
        ),
    ]);
    let (_, c) = analyzed(&l);
    assert_eq!(codes(&c), ["N2004"]);
    assert_eq!(l.sources.slice(c.diagnostics[0].primary.span).unwrap(), "S");
    assert_eq!(
        l.sources.slice(c.diagnostics[0].secondary[0].span).unwrap(),
        "private let n:int"
    );
}

#[test]
fn p34_named_struct_fixed_bundle_source_origins_and_code_specific_spans() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/struct-named-pattern-proposal-fixtures");
    for name in [
        "main.nova",
        "private_local.nova",
        "alias_annotation.nova",
        "guard_skipped.nova",
        "guard_abort.nova",
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        unit(&l);
    }
    for (name, code, expected) in [
        ("private.nova", "N2004", (65, 71)),
        ("import_fail.nova", "N2001", (0, 20)),
        ("nominal.nova", "N2101", (108, 113)),
        ("alias_head.nova", "N1102", (85, 90)),
        ("enum_head.nova", "N2101", (86, 92)),
        ("unknown.nova", "N2001", (69, 88)),
        ("immutable.nova", "N3004", (87, 88)),
        ("guard_only.nova", "N3101", (50, 55)),
        ("const_failure.nova", "N3201", (69, 72)),
        ("const_cycle.nova", "N3202", (91, 92)),
        ("unknown_label.nova", "N2201", (74, 79)),
        ("duplicate_label.nova", "N2201", (78, 79)),
        ("positional_overlap.nova", "N2201", (76, 77)),
        ("positional_after_named.nova", "N2201", (81, 82)),
        ("missing_field.nova", "N2201", (69, 81)),
        ("method_label.nova", "N2201", (113, 119)),
        ("component_type.nova", "N2101", (79, 80)),
        ("duplicate_binder.nova", "N2002", (68, 69)),
        ("incomplete.nova", "N3101", (50, 55)),
        ("duplicate.nova", "N3102", (83, 102)),
        ("let_refutable.nova", "N3103", (54, 73)),
        ("var_refutable.nova", "N3103", (54, 73)),
        ("const_refutable.nova", "N3103", (56, 75)),
        ("global_refutable.nova", "N3103", (44, 63)),
        ("enum_named.nova", "N1101", (57, 57)),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p34_named_struct_import_identity_label_namespace_private_and_global_binder_exports() {
    let l=fixture(&[("main.nova","use values::S as Imported;use values::N;use values::B;func main(){let Imported(b:flag,n:n)=Imported(N,B)}"),("values.nova","public struct S{public let n:int;public let b:bool}public const S(b:B,n:N)=S(8,true)")]);
    let m = unit(&l);
    assert!(nova_mir::validate(m.mir()).is_empty());
    let l = fixture(&[
        (
            "main.nova",
            "use values::S;use values::make;func main(){match make(){S(n:_)=>{}}}",
        ),
        (
            "values.nova",
            "public struct S{private let n:int}public func make()->S{return S(1)}",
        ),
    ]);
    let (_, c) = analyzed(&l);
    assert_eq!(codes(&c), ["N2004"]);
    assert_eq!(l.sources.slice(c.diagnostics[0].primary.span).unwrap(), "S");
}

#[test]
fn p35_rest_struct_bundle_source_origins_and_exact_diagnostics() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/struct-rest-pattern-proposal-fixtures");
    for name in [
        "main.nova",
        "private_local.nova",
        "full_and_empty.nova",
        "guard_skipped.nova",
        "guard_abort.nova",
        "omitted_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    for (name, code, expected) in [
        ("private.nova", "N2004", (65, 71)),
        ("incomplete.nova", "N3101", (61, 66)),
        ("duplicate.nova", "N3102", (93, 102)),
        ("missing_no_rest.nova", "N2201", (83, 92)),
        ("unknown_label.nova", "N2201", (85, 90)),
        ("duplicate_label.nova", "N2201", (89, 90)),
        ("after_named.nova", "N2201", (89, 90)),
        ("overlap.nova", "N2201", (87, 91)),
        ("component_type.nova", "N2101", (90, 91)),
        ("too_large.nova", "N2102", (87, 90)),
        ("immutable.nova", "N3004", (91, 92)),
        ("let_refutable.nova", "N3103", (65, 80)),
        ("const_refutable.nova", "N3103", (67, 76)),
        ("const_omitted_failure.nova", "N3201", (66, 69)),
        ("const_cycle.nova", "N3202", (95, 96)),
        ("middle_rest.nova", "N1102", (70, 71)),
        ("double_rest.nova", "N1102", (70, 71)),
        ("spaced_dots.nova", "N1102", (67, 68)),
        ("rest_value.nova", "N1102", (69, 70)),
        ("tuple_rest.nova", "N1102", (18, 19)),
        ("alias_head.nova", "N1102", (74, 75)),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        // P36: preserve the original P35 rejection fixture/metadata and validate current success.
        if name == "tuple_rest.nova" {
            unit(&l);
            continue;
        }
        // P40 preserves the historical rejection bytes and checks current acceptance.
        if name == "middle_rest.nova" {
            unit(&l);
            continue;
        }
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p35_rest_import_alias_private_omission_and_exported_sparse_const() {
    let l = fixture(&[
        (
            "main.nova",
            "use values::S as P;use values::N;func main(){let P(n:n,..)=P(true,N)}",
        ),
        (
            "values.nova",
            "public struct S{public let b:bool;public let n:int}public const S(n:N,..)=S(true,8)",
        ),
    ]);
    unit(&l);
    let l=fixture(&[("main.nova","use values::S;use values::make;func main(){match make(){S(b:_,..)=>{}}}"),("values.nova","public struct S{private let n:int;public let b:bool}public func make()->S{return S(1,true)}")]);
    let (_, c) = analyzed(&l);
    assert_eq!(codes(&c), ["N2004"]);
    assert_eq!(l.sources.slice(c.diagnostics[0].primary.span).unwrap(), "S");
}

#[test]
fn p36_tuple_rest_bundle_code_specific_spans_and_historical_override() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/tuple-rest-pattern-proposal-fixtures");
    for name in [
        "main.nova",
        "full_and_single.nova",
        "float_copy.nova",
        "guard_skipped.nova",
        "guard_abort.nova",
        "omitted_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    let historical_root = root.join("../struct-rest-pattern-proposal-fixtures");
    unit(
        &load(
            &historical_root.join("tuple_rest.nova"),
            Some(&historical_root),
        )
        .unwrap(),
    );
    for (name, code, expected) in [
        ("incomplete.nova", "N3101", (12, 17)),
        ("duplicate.nova", "N3102", (35, 41)),
        ("too_many.nova", "N2201", (15, 23)),
        ("missing_no_rest.nova", "N2201", (15, 19)),
        ("unit_wrong.nova", "N2101", (15, 19)),
        ("scalar_wrong.nova", "N2101", (20, 24)),
        ("component_type.nova", "N2101", (27, 31)),
        ("too_large.nova", "N2102", (35, 38)),
        ("immutable.nova", "N3004", (31, 32)),
        ("let_refutable.nova", "N3103", (15, 21)),
        ("const_refutable.nova", "N3103", (17, 23)),
        ("global_refutable.nova", "N3103", (5, 11)),
        ("const_omitted_failure.nova", "N3201", (18, 21)),
        ("const_cycle.nova", "N3202", (35, 36)),
        ("duplicate_binder.nova", "N2002", (18, 19)),
        ("initializer_scope.nova", "N2001", (23, 24)),
        ("middle_rest.nova", "N1102", (21, 22)),
        ("double_rest.nova", "N1102", (19, 20)),
        ("spaced_dots.nova", "N1102", (16, 17)),
        ("enum_rest.nova", "N1102", (56, 57)),
        ("tuple_label.nova", "N1102", (15, 17)),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        // P37 preserves this P36 rejection fixture and records its current success separately.
        if name == "enum_rest.nova" {
            unit(&l);
            continue;
        }
        // P38 preserves the P36 middle-rest rejection bytes and records current success separately.
        if name == "middle_rest.nova" {
            unit(&l);
            continue;
        }
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p36_tuple_rest_imported_alias_global_prefix_exports_and_source_origins() {
    let l = fixture(&[
        (
            "main.nova",
            "use values::Data;use values::N;func main(){let(n,..):Data=(N,true,'🙂')}",
        ),
        (
            "values.nova",
            "public type Data=(int8,bool,char);public const(N,..):Data=(8,true,'가')",
        ),
    ]);
    unit(&l);
    let (h, _, c) = {
        let (r, c) = analyzed(&l);
        (l.module.as_ref().unwrap(), r, c)
    };
    assert!(!c.has_errors());
    for n in h.nodes() {
        l.sources.slice(n.span).unwrap();
    }
}

#[test]
fn p37_payload_rest_bundle_code_specific_spans_and_history() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/payload-rest-pattern-proposal-fixtures");
    for name in [
        "main.nova",
        "full_and_nullary.nova",
        "float_copy.nova",
        "guard_skipped.nova",
        "guard_abort.nova",
        "omitted_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    let historical_root = root.join("../tuple-rest-pattern-proposal-fixtures");
    unit(
        &load(
            &historical_root.join("enum_rest.nova"),
            Some(&historical_root),
        )
        .unwrap(),
    );
    for (name, code, expected) in [
        ("incomplete.nova", "N3101", (36, 41)),
        ("duplicate.nova", "N3102", (64, 74)),
        ("too_many.nova", "N2201", (36, 50)),
        ("missing_no_rest.nova", "N2201", (36, 43)),
        ("nullary_rest.nova", "N2201", (28, 38)),
        ("nullary_parens.nova", "N2201", (28, 36)),
        ("none_rest.nova", "N2201", (36, 52)),
        ("component_type.nova", "N2101", (56, 60)),
        ("too_large.nova", "N2102", (57, 60)),
        ("immutable.nova", "N3004", (60, 61)),
        ("let_refutable.nova", "N3103", (36, 46)),
        ("const_refutable.nova", "N3103", (38, 48)),
        ("global_refutable.nova", "N3103", (26, 36)),
        ("const_omitted_failure.nova", "N3201", (47, 50)),
        ("const_cycle.nova", "N3202", (64, 65)),
        ("duplicate_binder.nova", "N2002", (47, 48)),
        ("initializer_scope.nova", "N2001", (52, 53)),
        ("middle_rest.nova", "N1102", (46, 47)),
        ("double_rest.nova", "N1102", (44, 45)),
        ("spaced_dots.nova", "N1102", (41, 42)),
        ("payload_label.nova", "N1102", (41, 42)),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        // P39 keeps the frozen rejection source/metadata and validates current success.
        if name == "middle_rest.nova" {
            unit(&l);
            continue;
        }
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p37_payload_rest_imported_alias_global_exports_visibility_and_source() {
    let l=fixture(&[("main.nova","use values::Data;use values::S as Imported;use values::N;func main(){let Imported::V(n,..):Data=Imported::V(N,true,'🙂')}"),("values.nova","public enum S{V(int8,bool,char)}public type Data=S;public const S::V(N,..):Data=S::V(8,true,'가')")]);
    unit(&l);
    let (_, c) = analyzed(&l);
    assert!(!c.has_errors());
    for n in l.module.as_ref().unwrap().nodes() {
        l.sources.slice(n.span).unwrap();
    }
    let bad = fixture(&[
        ("main.nova", "use values::S;func main(){}"),
        ("values.nova", "private enum S{V(int,bool)}"),
    ]);
    let (_, c) = analyzed(&bad);
    assert!(codes(&c).contains(&"N2004".to_string()));
}

#[test]
fn p38_tuple_middle_rest_bundle_code_specific_spans_and_history() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/tuple-middle-rest-proposal-fixtures");
    for name in [
        "main.nova",
        "full_and_single.nova",
        "float_copy.nova",
        "guard_skipped.nova",
        "guard_abort.nova",
        "omitted_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    let historical_root = root.join("../tuple-rest-pattern-proposal-fixtures");
    unit(
        &load(
            &historical_root.join("middle_rest.nova"),
            Some(&historical_root),
        )
        .unwrap(),
    );
    for (name, code, expected) in [
        ("incomplete.nova", "N3101", (12, 17)),
        ("duplicate.nova", "N3102", (41, 51)),
        ("too_many.nova", "N2201", (15, 25)),
        ("missing_no_rest.nova", "N2201", (15, 19)),
        ("unit_wrong.nova", "N2101", (15, 21)),
        ("scalar_wrong.nova", "N2101", (20, 26)),
        ("component_type.nova", "N2101", (36, 37)),
        ("too_large.nova", "N2102", (40, 43)),
        ("immutable.nova", "N3004", (31, 32)),
        ("let_refutable.nova", "N3103", (15, 21)),
        ("const_refutable.nova", "N3103", (17, 23)),
        ("global_refutable.nova", "N3103", (5, 11)),
        ("const_omitted_failure.nova", "N3201", (20, 23)),
        ("const_cycle.nova", "N3202", (35, 36)),
        ("duplicate_binder.nova", "N2002", (21, 22)),
        ("initializer_scope.nova", "N2001", (28, 29)),
        ("double_rest.nova", "N1102", (21, 22)),
        ("spaced_dots.nova", "N1102", (16, 17)),
        ("struct_middle.nova", "N1102", (54, 55)),
        ("payload_middle.nova", "N1102", (46, 47)),
        ("tuple_label.nova", "N1102", (19, 20)),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        // P39 keeps the frozen rejection source/metadata and validates current success.
        if name == "payload_middle.nova" {
            unit(&l);
            continue;
        }
        // P40 preserves the historical rejection bytes and checks current acceptance.
        if name == "struct_middle.nova" {
            unit(&l);
            continue;
        }
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p38_tuple_middle_rest_imported_alias_global_suffix_exports_and_source() {
    let l = fixture(&[
        (
            "main.nova",
            "use values::Data;use values::C;func main(){let(..,c):Data=(8,true,C)}",
        ),
        (
            "values.nova",
            "public type Data=(int8,bool,char);public const(N,..,C):Data=(8,true,'가')",
        ),
    ]);
    unit(&l);
    let (_, c) = analyzed(&l);
    assert!(!c.has_errors());
    for n in l.module.as_ref().unwrap().nodes() {
        l.sources.slice(n.span).unwrap();
    }
}

#[test]
fn p39_payload_middle_rest_bundle_code_specific_spans_and_two_histories() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/payload-middle-rest-proposal-fixtures");
    for name in [
        "main.nova",
        "full_and_nullary.nova",
        "float_copy.nova",
        "guard_skipped.nova",
        "guard_abort.nova",
        "omitted_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    for (folder, name) in [
        ("payload-rest-pattern-proposal-fixtures", "middle_rest.nova"),
        ("tuple-middle-rest-proposal-fixtures", "payload_middle.nova"),
    ] {
        let old = root.join("..").join(folder);
        unit(&load(&old.join(name), Some(&old)).unwrap());
    }
    for (name, code, expected) in [
        ("incomplete.nova", "N3101", (41, 46)),
        ("duplicate.nova", "N3102", (75, 89)),
        ("too_many.nova", "N2201", (36, 50)),
        ("missing_no_rest.nova", "N2201", (36, 43)),
        ("nullary_rest.nova", "N2201", (28, 38)),
        ("nullary_parens.nova", "N2201", (28, 36)),
        ("none_rest.nova", "N2201", (36, 52)),
        ("component_type.nova", "N2101", (70, 71)),
        ("too_large.nova", "N2102", (71, 74)),
        ("immutable.nova", "N3004", (60, 61)),
        ("let_refutable.nova", "N3103", (36, 46)),
        ("const_refutable.nova", "N3103", (37, 47)),
        ("global_refutable.nova", "N3103", (25, 35)),
        ("const_omitted_failure.nova", "N3201", (53, 56)),
        ("const_cycle.nova", "N3202", (64, 65)),
        ("duplicate_binder.nova", "N2002", (50, 51)),
        ("initializer_scope.nova", "N2001", (57, 58)),
        ("double_rest.nova", "N1102", (46, 47)),
        ("spaced_dots.nova", "N1102", (41, 42)),
        ("struct_middle.nova", "N1102", (54, 55)),
        ("payload_label.nova", "N1102", (44, 45)),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        // P40 preserves the historical rejection bytes and checks current acceptance.
        if name == "struct_middle.nova" {
            unit(&l);
            continue;
        }
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p39_payload_middle_rest_imported_alias_suffix_exports_visibility_and_source() {
    let l=fixture(&[("main.nova","use values::Data;use values::S as Imported;use values::C;func main(){let Imported::V(..,c):Data=Imported::V(8,true,C)}"),("values.nova","public enum S{V(int8,bool,char)}public type Data=S;public const S::V(N,..,C):Data=S::V(8,true,'가')")]);
    unit(&l);
    let (_, c) = analyzed(&l);
    assert!(!c.has_errors());
    for n in l.module.as_ref().unwrap().nodes() {
        l.sources.slice(n.span).unwrap();
    }
    let bad = fixture(&[
        ("main.nova", "use values::S;func main(){}"),
        ("values.nova", "private enum S{V(int,bool,char)}"),
    ]);
    let (_, c) = analyzed(&bad);
    assert!(codes(&c).contains(&"N2004".to_string()));
}

#[test]
fn p40_struct_middle_rest_bundle_code_specific_spans_and_three_histories() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development-v0.1/struct-middle-rest-proposal-fixtures");
    for name in [
        "main.nova",
        "full_and_empty.nova",
        "float_copy.nova",
        "guard_skipped.nova",
        "guard_abort.nova",
        "omitted_abort.nova",
    ] {
        unit(&load(&root.join(name), Some(&root)).unwrap());
    }
    for (folder, name) in [
        ("struct-rest-pattern-proposal-fixtures", "middle_rest.nova"),
        ("tuple-middle-rest-proposal-fixtures", "struct_middle.nova"),
        (
            "payload-middle-rest-proposal-fixtures",
            "struct_middle.nova",
        ),
    ] {
        let old = root.join("..").join(folder);
        unit(&load(&old.join(name), Some(&old)).unwrap());
    }
    for (name, code, expected) in [
        ("incomplete.nova", "N3101", (55, 60)),
        ("duplicate.nova", "N3102", (86, 96)),
        ("missing_no_rest.nova", "N2201", (59, 63)),
        ("unknown_label.nova", "N2201", (64, 71)),
        ("duplicate_label.nova", "N2201", (68, 69)),
        ("after_named.nova", "N2201", (68, 69)),
        ("overlap.nova", "N2201", (68, 69)),
        ("suffix_overlap.nova", "N2201", (66, 67)),
        ("excessive.nova", "N2201", (68, 69)),
        ("component_type.nova", "N2101", (82, 83)),
        ("annotation_range.nova", "N2102", (84, 87)),
        ("immutable.nova", "N3004", (83, 84)),
        ("let_refutable.nova", "N3103", (59, 69)),
        ("const_refutable.nova", "N3103", (61, 71)),
        ("global_refutable.nova", "N3103", (49, 59)),
        ("const_omitted_failure.nova", "N3201", (67, 70)),
        ("const_cycle.nova", "N3202", (83, 84)),
        ("duplicate_binder.nova", "N2002", (66, 67)),
        ("initializer_scope.nova", "N2001", (76, 77)),
        ("double_rest.nova", "N1102", (66, 67)),
        ("spaced_dots.nova", "N1102", (61, 62)),
        ("rest_value.nova", "N1102", (63, 64)),
        ("alias_head.nova", "N1102", (68, 69)),
        ("private.nova", "N2004", (50, 64)),
        ("rest_binder.nova", "N1102", (63, 67)),
    ] {
        let l = load(&root.join(name), Some(&root)).unwrap();
        let mut diagnostics = l.diagnostics.clone();
        if l.module.is_some() {
            let (_, c) = analyzed(&l);
            diagnostics.extend(c.diagnostics);
        }
        assert!(
            diagnostics.iter().any(|d| d.code.to_string() == code
                && (d.primary.span.start(), d.primary.span.end()) == expected),
            "{name}: {diagnostics:?}"
        );
        if matches!(name, "const_refutable.nova" | "global_refutable.nova") {
            assert!(!diagnostics
                .iter()
                .any(|d| matches!(d.code.to_string().as_str(), "N3201" | "N3202")));
        }
        for d in diagnostics {
            l.sources.slice(d.primary.span).unwrap();
            for label in d.secondary {
                l.sources.slice(label.span).unwrap();
            }
        }
    }
}

#[test]
fn p40_struct_middle_rest_imported_alias_exports_private_and_sources() {
    let l=fixture(&[("main.nova","use values::Data;use values::S as Imported;use values::C;func main(){let Imported(..,c):Data=Imported(8,true,C)}"),("values.nova","public struct S{public let n:int8;public let b:bool;public let c:char}public type Data=S;public const S(c:C,..,n:N):Data=S(8,true,'가')")]);
    unit(&l);
    let (_, c) = analyzed(&l);
    assert!(!c.has_errors());
    for n in l.module.as_ref().unwrap().nodes() {
        l.sources.slice(n.span).unwrap();
    }
    let bad = fixture(&[
        ("main.nova", "use values::S;func f(s:S){let S(..,n:n)=s}"),
        (
            "values.nova",
            "public struct S{public let n:int;private let hidden:bool}",
        ),
    ]);
    let (_, c) = analyzed(&bad);
    assert_eq!(codes(&c), ["N2004"]);
}
