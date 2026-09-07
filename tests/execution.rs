use gabriel::{
    catalog::{Catalog, Module},
    execution::execute,
    plan::{Action, ActionKind, Plan},
};
use std::collections::BTreeMap;

#[test]
fn blocks_failed_dependencies_and_continues_independent_modules() {
    let modules = ["dependency", "dependent", "independent"].into_iter().map(|id| {
        let mut module: Module = toml::from_str(&format!("schema_version=1\nid={id:?}\ndescription=\"test\"\n[platforms.ubuntu]\nminimum_os=\"24.04\"\narchitectures=[\"x86_64\"]")).unwrap();
        if id == "dependent" { module.dependencies.push("dependency".into()); }
        (id.into(), module)
    }).collect();
    let catalog = Catalog { modules };
    let plan = Plan {
        excluded: BTreeMap::new(),
        actions: vec![
            Action {
                module: "dependency".into(),
                kind: ActionKind::Install,
            },
            Action {
                module: "dependent".into(),
                kind: ActionKind::Configure,
            },
            Action {
                module: "independent".into(),
                kind: ActionKind::Install,
            },
        ],
    };
    let mut calls = Vec::new();
    let failures = execute(&catalog, &plan, |action| {
        calls.push(action.module.clone());
        if action.module == "dependency" {
            Err("installation verification failed".into())
        } else {
            Ok(())
        }
    });
    assert_eq!(calls, ["dependency", "independent"]);
    assert!(failures.contains_key("dependency"));
    assert!(failures["dependent"].contains("dependency"));
}
