use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Component, Path, PathBuf},
};

pub fn sha256(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    let digest = digest.finalize();
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn verify(bytes: &[u8], expected: &str) -> Result<(), String> {
    if expected.len() != 64 || !expected.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("invalid SHA-256 checksum".into());
    }
    if !sha256(bytes).eq_ignore_ascii_case(expected) {
        return Err("checksum mismatch".into());
    }
    Ok(())
}

pub fn extract_destination(root: &Path, member: &Path) -> Result<PathBuf, String> {
    if !member
        .components()
        .all(|component| matches!(component, Component::Normal(_)))
    {
        return Err("archive member escapes destination".into());
    }
    let destination = root.join(member);
    if !destination.starts_with(root) {
        return Err("archive member escapes destination".into());
    }
    if fs::symlink_metadata(root).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err("extraction root cannot be a symlink".into());
    }
    Ok(destination)
}
