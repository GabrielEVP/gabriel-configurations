use gabriel::{catalog::Module, config::compose, paths::UserPaths, platform::OperatingSystem};
use std::{collections::BTreeMap, fs};

fn fixture() -> (tempfile::TempDir, Module, UserPaths) {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("common")).unwrap();
    fs::create_dir(root.path().join("macos")).unwrap();
    fs::write(root.path().join("common/config"), "shared").unwrap();
    fs::write(root.path().join("macos/config"), "apple").unwrap();
    let mut module: Module = toml::from_str("schema_version=1\nid=\"test\"\ndescription=\"test\"\n[platforms.ubuntu]\nminimum_os=\"24.04\"\narchitectures=[\"x86_64\"]\n[[files]]\nsource=\"config\"\nbase=\"config\"\ntarget=\"test/config\"\n").unwrap();
    module.root = root.path().to_owned();
    let paths = UserPaths::resolve(
        &OperatingSystem::Ubuntu,
        &root.path().join("home"),
        None,
        None,
    )
    .unwrap();
    (root, module, paths)
}

#[test]
fn overrides_files_by_os_and_does_not_create_destinations() {
    let (_root, module, paths) = fixture();
    assert_eq!(
        compose(&module, "macos", &paths, &BTreeMap::new()).unwrap()[0].content,
        b"apple"
    );
    assert_eq!(
        compose(&module, "ubuntu", &paths, &BTreeMap::new()).unwrap()[0].content,
        b"shared"
    );
    assert!(!paths.home.exists());
}

#[test]
fn shell_template_quotes_detected_values_and_leaves_unmarked_files_literal() {
    let (root, mut module, paths) = fixture();
    fs::write(
        root.path().join("common/config"),
        "export PREFIX={{prefix}}\n",
    )
    .unwrap();
    module
        .variables
        .insert("prefix".into(), "brew_prefix".into());
    let values = BTreeMap::from([("brew_prefix".into(), "/a path/it's $(dangerous)".into())]);
    assert_eq!(
        compose(&module, "ubuntu", &paths, &values).unwrap()[0].content,
        b"export PREFIX={{prefix}}\n"
    );
    module.files[0].template = Some("shell".into());
    assert_eq!(
        String::from_utf8(
            compose(&module, "ubuntu", &paths, &values).unwrap()[0]
                .content
                .clone()
        )
        .unwrap(),
        "export PREFIX='/a path/it'\"'\"'s $(dangerous)'\n"
    );
}

#[test]
fn unknown_and_unresolved_variables_block_composition() {
    let (root, mut module, paths) = fixture();
    fs::write(root.path().join("common/config"), "{{missing}}").unwrap();
    module.files[0].template = Some("text".into());
    assert!(compose(&module, "ubuntu", &paths, &BTreeMap::new()).is_err());
    module.variables.insert("missing".into(), "node".into());
    assert!(compose(&module, "ubuntu", &paths, &BTreeMap::new()).is_err());
}

#[test]
fn refuses_destination_parent_symlinks_and_source_escapes() {
    let (root, module, paths) = fixture();
    fs::create_dir_all(&paths.config).unwrap();
    std::os::unix::fs::symlink(root.path(), paths.config.join("test")).unwrap();
    assert!(compose(&module, "ubuntu", &paths, &BTreeMap::new()).is_err());
    fs::remove_file(paths.config.join("test")).unwrap();
    fs::remove_file(root.path().join("common/config")).unwrap();
    std::os::unix::fs::symlink("/etc/passwd", root.path().join("common/config")).unwrap();
    assert!(compose(&module, "ubuntu", &paths, &BTreeMap::new()).is_err());
}

#[test]
fn allows_broken_leaf_link_for_backed_up_adoption() {
    let (_root, module, paths) = fixture();
    fs::create_dir_all(paths.config.join("test")).unwrap();
    std::os::unix::fs::symlink("/nonexistent/legacy", paths.config.join("test/config")).unwrap();
    assert!(compose(&module, "ubuntu", &paths, &BTreeMap::new()).is_ok());
}
