use gabriel::catalog::Catalog;
use std::{fs, path::Path};

fn module(root: &Path, folder: &str, id: &str, dependencies: &[&str]) {
    let dir = root.join("modules").join(folder);
    fs::create_dir_all(dir.join("common")).unwrap();
    fs::write(dir.join("common/config"), "preference").unwrap();
    let dependencies = dependencies
        .iter()
        .map(|id| format!("{id:?}"))
        .collect::<Vec<_>>()
        .join(",");
    fs::write(dir.join("module.toml"), format!(
        "schema_version=1\nid={id:?}\ndescription=\"Example\"\ndependencies=[{dependencies}]\n[platforms.ubuntu]\nminimum_os=\"24.04\"\narchitectures=[\"x86_64\",\"aarch64\"]\n[platforms.ubuntu.provider]\nkind=\"apt\"\npackage={id:?}\n[detect]\nprogram={id:?}\nargs=[\"--version\"]\nminimum_version=\"1.0\"\n[[files]]\nsource=\"config\"\nbase=\"config\"\ntarget=\"{id}/config\"\n"
    )).unwrap();
}

#[test]
fn discovers_new_folder_without_binary_changes() {
    let root = tempfile::tempdir().unwrap();
    module(root.path(), "tmux", "tmux", &[]);
    assert_eq!(Catalog::load(root.path()).unwrap().modules.len(), 1);
    module(root.path(), "kitty", "kitty", &[]);
    let catalog = Catalog::load(root.path()).unwrap();
    assert_eq!(catalog.modules["kitty"].id, "kitty");
}

#[test]
fn dependency_order_is_stable_and_deduplicated() {
    let root = tempfile::tempdir().unwrap();
    module(root.path(), "git", "git", &[]);
    module(root.path(), "tmux", "tmux", &["git"]);
    module(root.path(), "nvim", "nvim", &["git"]);
    let catalog = Catalog::load(root.path()).unwrap();
    assert_eq!(
        catalog
            .ordered(&["tmux".into(), "nvim".into(), "tmux".into()])
            .unwrap(),
        ["git", "nvim", "tmux"]
    );
    assert!(catalog.ordered(&["unknown".into()]).is_err());
}

#[test]
fn rejects_duplicate_identifiers() {
    let root = tempfile::tempdir().unwrap();
    module(root.path(), "one", "tmux", &[]);
    module(root.path(), "two", "tmux", &[]);
    assert!(
        Catalog::load(root.path())
            .unwrap_err()
            .contains("duplicate")
    );
}

#[test]
fn rejects_missing_dependencies_and_cycles_before_execution() {
    let root = tempfile::tempdir().unwrap();
    module(root.path(), "one", "one", &["two"]);
    assert!(Catalog::load(root.path()).unwrap_err().contains("two"));
    module(root.path(), "two", "two", &["one"]);
    assert!(Catalog::load(root.path()).unwrap_err().contains("cycle"));
}

#[test]
fn rejects_invalid_schema_unknown_providers_and_escaping_paths() {
    for (old, new) in [
        ("schema_version=1", "schema_version=99"),
        ("kind=\"apt\"", "kind=\"shell\""),
        ("source=\"config\"", "source=\"../../secret\""),
        ("target=\"tmux/config\"", "target=\"../secret\""),
        (
            "architectures=[\"x86_64\",\"aarch64\"]",
            "architectures=[\"unknown\"]",
        ),
    ] {
        let root = tempfile::tempdir().unwrap();
        module(root.path(), "tmux", "tmux", &[]);
        let manifest = root.path().join("modules/tmux/module.toml");
        fs::write(
            &manifest,
            fs::read_to_string(&manifest).unwrap().replace(old, new),
        )
        .unwrap();
        assert!(Catalog::load(root.path()).is_err(), "{new}");
    }
}

#[test]
fn rejects_overlapping_module_destinations() {
    let root = tempfile::tempdir().unwrap();
    module(root.path(), "one", "one", &[]);
    module(root.path(), "two", "two", &[]);
    let manifest = root.path().join("modules/two/module.toml");
    fs::write(
        &manifest,
        fs::read_to_string(&manifest)
            .unwrap()
            .replace("two/config", "one/config"),
    )
    .unwrap();
    assert!(
        Catalog::load(root.path())
            .unwrap_err()
            .contains("destination")
    );
}

#[test]
fn rejects_sources_linked_outside_module() {
    let root = tempfile::tempdir().unwrap();
    module(root.path(), "tmux", "tmux", &[]);
    let source = root.path().join("modules/tmux/common/config");
    fs::remove_file(&source).unwrap();
    fs::write(root.path().join("secret"), "private").unwrap();
    std::os::unix::fs::symlink(root.path().join("secret"), source).unwrap();
    assert!(Catalog::load(root.path()).is_err());
}

#[test]
fn repository_catalog_contains_all_personal_configuration_modules() {
    let catalog = Catalog::load(Path::new(".")).unwrap();
    for id in [
        "nvim",
        "ghostty",
        "alacritty",
        "kitty",
        "zsh",
        "tmux",
        "zellij",
        "yazi",
        "zed",
        "opencode",
    ] {
        assert!(catalog.modules.contains_key(id), "missing module {id}");
    }
}
