use crate::download::verify;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn prepare(binary: &[u8], checksum: &str, destination: &Path) -> Result<PathBuf, String> {
    verify(binary, checksum)?;
    if destination.as_os_str().is_empty() || !destination.is_absolute() {
        return Err("binary destination must be absolute".into());
    }
    let parent = destination
        .parent()
        .ok_or("binary destination has no parent")?;
    if fs::symlink_metadata(destination).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err("binary destination cannot be a symlink".into());
    }
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let temporary = parent.join(format!(".gabriel-update-{}", std::process::id()));
    fs::write(&temporary, binary).map_err(|error| error.to_string())?;
    Ok(temporary)
}

pub fn commit(temporary: &Path, destination: &Path) -> Result<(), String> {
    if !temporary.is_absolute()
        || !temporary.starts_with(
            destination
                .parent()
                .ok_or("binary destination has no parent")?,
        )
    {
        return Err("invalid staged binary".into());
    }
    fs::rename(temporary, destination).map_err(|error| error.to_string())
}
