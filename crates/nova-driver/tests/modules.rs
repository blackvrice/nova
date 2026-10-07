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
