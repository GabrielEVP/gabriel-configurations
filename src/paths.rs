use crate::platform::OperatingSystem;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct UserPaths {
    pub home: PathBuf,
    pub config: PathBuf,
    pub state: PathBuf,
}

impl UserPaths {
    pub fn resolve(
        os: &OperatingSystem,
        home: &Path,
        config: Option<&Path>,
        state: Option<&Path>,
    ) -> Result<Self, String> {
        if !home.is_absolute() {
            return Err("HOME must be absolute".into());
        }
        let config = config
            .filter(|path| path.is_absolute())
            .map(Path::to_path_buf)
            .unwrap_or_else(|| home.join(".config"));
        let state = state
            .filter(|path| path.is_absolute())
            .map(Path::to_path_buf)
            .unwrap_or_else(|| match os {
                OperatingSystem::Ubuntu => home.join(".local/state"),
                OperatingSystem::Macos => home.join("Library/Application Support"),
            })
            .join("gabriel");
        Ok(Self {
            home: home.to_path_buf(),
            config,
            state,
        })
    }
}

pub fn brew_candidates(os: &OperatingSystem, path: &[PathBuf]) -> Vec<PathBuf> {
    let known = match os {
        OperatingSystem::Ubuntu => vec!["/home/linuxbrew/.linuxbrew/bin"],
        OperatingSystem::Macos => vec!["/opt/homebrew/bin", "/usr/local/bin"],
    };
    let mut candidates = Vec::new();
    for directory in path
        .iter()
        .cloned()
        .chain(known.into_iter().map(PathBuf::from))
    {
        let candidate = directory.join("brew");
        if directory.is_absolute() && !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    candidates
}
