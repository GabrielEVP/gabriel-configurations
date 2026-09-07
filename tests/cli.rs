use std::{fs, process::Command};

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_gabriel"))
}

#[test]
fn exposes_help_and_version_and_rejects_invalid_arguments() {
    let help = command().arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("setup"));
    let version = command().arg("--version").output().unwrap();
    assert!(String::from_utf8_lossy(&version.stdout).contains("0.1.0"));
    assert_eq!(
        command().arg("unknown-command").status().unwrap().code(),
        Some(2)
    );
}

#[test]
fn dry_run_renders_plan_without_creating_home_or_state() {
    let root = tempfile::tempdir().unwrap();
    let module = root.path().join("modules/example");
    fs::create_dir_all(module.join("common")).unwrap();
    fs::write(module.join("common/config"), "preference").unwrap();
    fs::write(module.join("module.toml"), "schema_version=1\nid=\"example\"\ndescription=\"Example\"\n[platforms.ubuntu]\nminimum_os=\"24.04\"\narchitectures=[\"x86_64\",\"aarch64\"]\n[platforms.macos]\nminimum_os=\"14.0\"\narchitectures=[\"x86_64\",\"aarch64\"]\n[[files]]\nsource=\"config\"\nbase=\"config\"\ntarget=\"example/config\"\n").unwrap();
    let home = root.path().join("home");
    let result = command()
        .env("HOME", &home)
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("XDG_STATE_HOME")
        .args(["setup", "--all", "--dry-run", "--source"])
        .arg(root.path())
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("example"));
    assert!(!home.exists());
}

#[test]
fn noninteractive_setup_requires_selection() {
    let output = command()
        .arg("setup")
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn installs_configuration_only_module_and_applies_selected_defaults_on_second_setup() {
    let root = tempfile::tempdir().unwrap();
    let module = root.path().join("modules/example");
    fs::create_dir_all(module.join("common")).unwrap();
    fs::write(module.join("common/config"), "preference").unwrap();
    fs::write(module.join("module.toml"), "schema_version=1\nid=\"example\"\ndescription=\"Example\"\n[platforms.ubuntu]\nminimum_os=\"24.04\"\narchitectures=[\"x86_64\",\"aarch64\"]\n[platforms.macos]\nminimum_os=\"14.0\"\narchitectures=[\"x86_64\",\"aarch64\"]\n[[files]]\nsource=\"config\"\nbase=\"config\"\ntarget=\"example/config\"\n").unwrap();
    let home = root.path().join("home");
    let run = || {
        command()
            .env("HOME", &home)
            .env_remove("XDG_CONFIG_HOME")
            .env_remove("XDG_STATE_HOME")
            .args(["install", "example", "--yes", "--source"])
            .arg(root.path())
            .output()
            .unwrap()
    };
    let output = run();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let destination = home.join(".config/example/config");
    fs::write(&destination, "local").unwrap();
    assert!(run().status.success());
    assert_eq!(fs::read(destination).unwrap(), b"preference");
}
