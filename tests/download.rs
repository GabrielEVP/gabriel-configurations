use gabriel::download::{extract_destination, sha256, verify};
use std::fs;

#[test]
fn verifies_sha256_before_accepting_an_artifact() {
    let digest = sha256(b"artifact");
    assert_eq!(digest.len(), 64);
    assert!(verify(b"artifact", &digest).is_ok());
    assert!(verify(b"tampered", &digest).is_err());
    assert!(verify(b"artifact", "short").is_err());
}

#[test]
fn confines_archive_members_and_rejects_symlink_roots() {
    let root = tempfile::tempdir().unwrap();
    assert_eq!(
        extract_destination(root.path(), std::path::Path::new("bin/gabriel")).unwrap(),
        root.path().join("bin/gabriel")
    );
    for member in ["../secret", "/absolute", "bin/../../secret"] {
        assert!(extract_destination(root.path(), std::path::Path::new(member)).is_err());
    }
    let target = root.path().join("real");
    fs::create_dir(&target).unwrap();
    let link = root.path().join("link");
    std::os::unix::fs::symlink(target, &link).unwrap();
    assert!(extract_destination(&link, std::path::Path::new("bin")).is_err());
}
