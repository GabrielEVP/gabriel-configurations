use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
pub struct Catalog {
    pub modules: BTreeMap<String, Module>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Module {
    pub schema_version: u32,
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub platforms: BTreeMap<String, Compatibility>,
    pub detect: Option<Detection>,
    #[serde(default)]
    pub files: Vec<FileMapping>,
    #[serde(default)]
    pub variables: BTreeMap<String, String>,
    #[serde(skip)]
    pub root: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Compatibility {
    pub minimum_os: String,
    pub architectures: Vec<String>,
    #[serde(default)]
    pub graphical: bool,
    pub provider: Option<Provider>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum Provider {
    Apt { package: String },
    Brew { package: String },
    Cask { package: String },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Detection {
    pub program: String,
    pub args: Vec<String>,
    pub minimum_version: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileMapping {
    pub source: PathBuf,
    pub base: String,
    pub target: PathBuf,
    #[serde(default)]
    pub template: Option<String>,
    #[serde(default)]
    pub local_only: bool,
}

pub fn relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && path
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

impl Catalog {
    pub fn load(root: &Path) -> Result<Self, String> {
        let mut modules = BTreeMap::new();
        let entries =
            fs::read_dir(root.join("modules")).map_err(|error| format!("catalog: {error}"))?;
        for entry in entries {
            let entry = entry.map_err(|error| error.to_string())?;
            if !entry
                .file_type()
                .map_err(|error| error.to_string())?
                .is_dir()
            {
                continue;
            }
            let manifest = entry.path().join("module.toml");
            if !manifest.exists() {
                continue;
            }
            if fs::symlink_metadata(&manifest)
                .map_err(|error| error.to_string())?
                .file_type()
                .is_symlink()
            {
                return Err(format!("manifest symlink: {}", manifest.display()));
            }
            let content = fs::read_to_string(&manifest).map_err(|error| error.to_string())?;
            let mut module: Module = toml::from_str(&content)
                .map_err(|error| format!("{}: {error}", manifest.display()))?;
            module.root = entry.path();
            module.validate()?;
            let id = module.id.clone();
            if modules.insert(id.clone(), module).is_some() {
                return Err(format!("duplicate module: {id}"));
            }
        }
        let catalog = Self { modules };
        catalog.ordered(&catalog.modules.keys().cloned().collect::<Vec<_>>())?;
        let mut destinations: BTreeMap<(String, PathBuf), String> = BTreeMap::new();
        for module in catalog.modules.values() {
            for file in &module.files {
                let key = (file.base.clone(), file.target.clone());
                for ((base, target), owner) in &destinations {
                    if base == &key.0 && (target.starts_with(&key.1) || key.1.starts_with(target)) {
                        return Err(format!(
                            "destination collision: {owner} and {}: {}",
                            module.id,
                            file.target.display()
                        ));
                    }
                }
                destinations.insert(key, module.id.clone());
            }
        }
        Ok(catalog)
    }

    pub fn ordered(&self, selected: &[String]) -> Result<Vec<String>, String> {
        let mut visiting = BTreeSet::new();
        let mut done = BTreeSet::new();
        let mut result = Vec::new();
        for id in selected.iter().collect::<BTreeSet<_>>() {
            self.visit(id, &mut visiting, &mut done, &mut result)?;
        }
        Ok(result)
    }

    fn visit(
        &self,
        id: &str,
        visiting: &mut BTreeSet<String>,
        done: &mut BTreeSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), String> {
        if done.contains(id) {
            return Ok(());
        }
        let module = self
            .modules
            .get(id)
            .ok_or_else(|| format!("missing dependency or module: {id}"))?;
        if !visiting.insert(id.to_owned()) {
            return Err(format!("dependency cycle at {id}"));
        }
        for dependency in module.dependencies.iter().collect::<BTreeSet<_>>() {
            self.visit(dependency, visiting, done, result)?;
        }
        visiting.remove(id);
        done.insert(id.to_owned());
        result.push(id.to_owned());
        Ok(())
    }
}

impl Module {
    fn validate(&self) -> Result<(), String> {
        let error = |message: &str| format!("{}: {message}", self.id);
        if self.schema_version != 1 {
            return Err(error("unsupported schema_version"));
        }
        if self.id.is_empty()
            || !self
                .id
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(error("invalid identifier"));
        }
        if self.description.trim().is_empty() || self.platforms.is_empty() {
            return Err(error("description and platforms required"));
        }
        for (os, compatibility) in &self.platforms {
            if !["ubuntu", "macos"].contains(&os.as_str()) {
                return Err(error("unsupported platform"));
            }
            validate_version(&compatibility.minimum_os).map_err(|_| error("invalid minimum_os"))?;
            if compatibility.architectures.is_empty()
                || compatibility
                    .architectures
                    .iter()
                    .any(|arch| !["x86_64", "aarch64"].contains(&arch.as_str()))
            {
                return Err(error("unsupported architecture"));
            }
            if let Some(provider) = &compatibility.provider {
                if self.detect.is_none() {
                    return Err(error("provider requires detection"));
                }
                let package = match provider {
                    Provider::Apt { package } if os == "ubuntu" => package,
                    Provider::Brew { package } => package,
                    Provider::Cask { package } if os == "macos" => package,
                    _ => return Err(error("provider incompatible with platform")),
                };
                if package.is_empty()
                    || package.starts_with('-')
                    || !package
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || b"-+._/@".contains(&byte))
                {
                    return Err(error("invalid package"));
                }
            }
        }
        if let Some(detect) = &self.detect {
            if detect.program.is_empty() || detect.program.starts_with('-') {
                return Err(error("invalid detection program"));
            }
            validate_version(&detect.minimum_version)
                .map_err(|_| error("invalid minimum_version"))?;
        }
        for value in self.variables.values() {
            if !["home", "config", "brew_prefix", "node", "clipboard"].contains(&value.as_str()) {
                return Err(error("unknown variable resolver"));
            }
        }
        for file in &self.files {
            if !relative_path(&file.source)
                || !relative_path(&file.target)
                || !["home", "config"].contains(&file.base.as_str())
            {
                return Err(error("invalid source or destination path"));
            }
            if file
                .template
                .as_ref()
                .is_some_and(|kind| !["text", "shell", "json"].contains(&kind.as_str()))
            {
                return Err(error("unknown template format"));
            }
            let mut found = false;
            for directory in ["common", "ubuntu", "macos"] {
                let source = self.root.join(directory).join(&file.source);
                if fs::symlink_metadata(&source).is_ok() {
                    let canonical = source
                        .canonicalize()
                        .map_err(|err| error(&err.to_string()))?;
                    if !canonical
                        .starts_with(self.root.canonicalize().map_err(|err| err.to_string())?)
                        || !canonical.is_file()
                    {
                        return Err(error("source escapes module or is not a file"));
                    }
                    found = true;
                }
            }
            if !found {
                return Err(error("source file missing"));
            }
        }
        Ok(())
    }
}

pub fn validate_version(value: &str) -> Result<Vec<u32>, String> {
    value
        .split('.')
        .map(|part| {
            if part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(format!("invalid version: {value}"));
            }
            part.parse()
                .map_err(|_| format!("invalid version: {value}"))
        })
        .collect()
}
