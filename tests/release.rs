use gabriel::{
    download::sha256,
    release::{commit, prepare},
};
use std::fs;

#[test]
fn stages_only_checksum_verified_binary_and_commits_atomically() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("bin/gabriel");
    let binary = b"new executable";
    let temporary = prepare(binary, &sha256(binary), &destination).unwrap();
    assert!(!destination.exists());
    commit(&temporary, &destination).unwrap();
    assert_eq!(fs::read(destination).unwrap(), binary);
}

#[test]
fn failed_update_preserves_existing_binary_and_rejects_unsafe_paths() {
    let root = tempfile::tempdir().unwrap();
    let destination = root.path().join("gabriel");
    fs::write(&destination, "old").unwrap();
    assert!(prepare(b"new", "bad", &destination).is_err());
    assert_eq!(fs::read(&destination).unwrap(), b"old");
    assert!(prepare(b"new", &sha256(b"new"), std::path::Path::new("relative")).is_err());
    let link = root.path().join("link");
    std::os::unix::fs::symlink(&destination, &link).unwrap();
    assert!(prepare(b"new", &sha256(b"new"), &link).is_err());
}
