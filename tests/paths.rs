use gabriel::{
    paths::{UserPaths, brew_candidates},
    platform::OperatingSystem,
};
use std::path::{Path, PathBuf};

#[test]
fn resolves_user_directories_without_creating_them() {
    let root = tempfile::tempdir().unwrap();
    let home = root.path().join("user with spaces");
    let paths = UserPaths::resolve(&OperatingSystem::Ubuntu, &home, None, None).unwrap();
    assert_eq!(paths.home, home);
    assert_eq!(paths.config, home.join(".config"));
    assert_eq!(paths.state, home.join(".local/state/gabriel"));
    assert!(!home.exists());
}

#[test]
fn respects_absolute_xdg_overrides() {
    let paths = UserPaths::resolve(
        &OperatingSystem::Macos,
        Path::new("/Users/example"),
        Some(Path::new("/Volumes/config")),
        Some(Path::new("/Volumes/state")),
    )
    .unwrap();
    assert_eq!(paths.config, Path::new("/Volumes/config"));
    assert_eq!(paths.state, Path::new("/Volumes/state/gabriel"));
}

#[test]
fn macos_state_uses_application_support_by_default() {
    let paths =
        UserPaths::resolve(&OperatingSystem::Macos, Path::new("/Users/me"), None, None).unwrap();
    assert_eq!(
        paths.state,
        Path::new("/Users/me/Library/Application Support/gabriel")
    );
}

#[test]
fn rejects_relative_home_and_ignores_relative_xdg_values() {
    assert!(
        UserPaths::resolve(&OperatingSystem::Ubuntu, Path::new("relative"), None, None).is_err()
    );
    let paths = UserPaths::resolve(
        &OperatingSystem::Ubuntu,
        Path::new("/home/me"),
        Some(Path::new("relative")),
        None,
    )
    .unwrap();
    assert_eq!(paths.config, Path::new("/home/me/.config"));
}

#[test]
fn finds_brew_candidates_in_path_then_known_prefixes_without_duplicates() {
    let candidates = brew_candidates(
        &OperatingSystem::Ubuntu,
        &[
            PathBuf::from("/custom path/bin"),
            PathBuf::from("/custom path/bin"),
        ],
    );
    assert_eq!(
        candidates,
        [
            PathBuf::from("/custom path/bin/brew"),
            PathBuf::from("/home/linuxbrew/.linuxbrew/bin/brew")
        ]
    );
    let candidates = brew_candidates(&OperatingSystem::Macos, &[]);
    assert!(candidates.contains(&PathBuf::from("/opt/homebrew/bin/brew")));
    assert!(candidates.contains(&PathBuf::from("/usr/local/bin/brew")));
}
