use gabriel::{
    catalog::{Catalog, Module},
    plan::{ActionKind, build, build_upgrade},
    platform::Platform,
};
use std::collections::BTreeMap;

fn catalog() -> Catalog {
    let modules = ["git", "nvim"].into_iter().map(|id| {
        let mut module: Module = toml::from_str(&format!("schema_version=1\nid={id:?}\ndescription=\"example\"\n[platforms.ubuntu]\nminimum_os=\"24.04\"\narchitectures=[\"x86_64\"]\n[platforms.ubuntu.provider]\nkind=\"apt\"\npackage={id:?}\n[detect]\nprogram={id:?}\nargs=[\"--version\"]\nminimum_version=\"2.0\"\n")).unwrap();
        if id == "nvim" { module.dependencies = vec!["git".into()]; }
        (id.into(), module)
    }).collect();
    Catalog { modules }
}

fn platform() -> Platform {
    Platform::parse("Linux", "x86_64", "ID=ubuntu\nVERSION_ID=26.04").unwrap()
}

#[test]
fn installs_missing_upgrades_insufficient_and_reuses_compatible_tools() {
    let catalog = catalog();
    let plan = build(
        &catalog,
        &platform(),
        &["nvim".into()],
        false,
        false,
        &BTreeMap::from([("git".into(), Some(vec![1, 9]))]),
    )
    .unwrap();
    assert_eq!(
        plan.actions
            .iter()
            .map(|action| (action.module.as_str(), &action.kind))
            .collect::<Vec<_>>(),
        [
            ("git", &ActionKind::Upgrade),
            ("nvim", &ActionKind::Install)
        ]
    );
    let plan = build(
        &catalog,
        &platform(),
        &["nvim".into()],
        false,
        false,
        &BTreeMap::from([
            ("git".into(), Some(vec![2, 0])),
            ("nvim".into(), Some(vec![3])),
        ]),
    )
    .unwrap();
    assert!(plan.actions.is_empty());
}

#[test]
fn explicit_incompatibility_fails_before_actions() {
    let mut platform = platform();
    platform.version = vec![22, 4];
    assert!(
        build(
            &catalog(),
            &platform,
            &["nvim".into()],
            false,
            false,
            &BTreeMap::new()
        )
        .is_err()
    );
}

#[test]
fn all_reports_exclusions_and_excludes_dependents() {
    let mut catalog = catalog();
    catalog
        .modules
        .get_mut("git")
        .unwrap()
        .platforms
        .get_mut("ubuntu")
        .unwrap()
        .graphical = true;
    let plan = build(&catalog, &platform(), &[], true, false, &BTreeMap::new()).unwrap();
    assert!(plan.actions.is_empty());
    assert!(plan.excluded.contains_key("git"));
    assert!(plan.excluded.contains_key("nvim"));
}

#[test]
fn unparseable_existing_version_is_not_treated_as_missing() {
    let result = build(
        &catalog(),
        &platform(),
        &["git".into()],
        false,
        false,
        &BTreeMap::from([("git".into(), None)]),
    );
    assert!(result.unwrap_err().contains("version"));
}

#[test]
fn explicit_upgrade_includes_compatible_installed_tools() {
    let plan = build_upgrade(
        &catalog(),
        &platform(),
        &["git".into()],
        false,
        &BTreeMap::from([("git".into(), Some(vec![3, 0]))]),
    )
    .unwrap();
    assert_eq!(plan.actions[0].kind, ActionKind::Upgrade);
}
