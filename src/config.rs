use crate::{
    catalog::{Module, relative_path},
    paths::UserPaths,
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct DesiredFile {
    pub path: PathBuf,
    pub content: Vec<u8>,
    pub local_only: bool,
}

pub fn check_parent(path: &Path) -> Result<(), String> {
    for ancestor in path
        .parent()
        .ok_or("destination has no parent")?
        .ancestors()
    {
        match fs::symlink_metadata(ancestor) {
            Ok(metadata)
                if metadata.file_type().is_symlink()
                    && !matches!(ancestor, p if p == Path::new("/var") || p == Path::new("/tmp"))
                    || !metadata.is_dir() =>
            {
                return Err(format!(
                    "unsafe destination ancestor: {}",
                    ancestor.display()
                ));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(())
}

pub fn compose(
    module: &Module,
    os: &str,
    paths: &UserPaths,
    values: &BTreeMap<String, String>,
) -> Result<Vec<DesiredFile>, String> {
    if !["ubuntu", "macos"].contains(&os) {
        return Err("unsupported configuration platform".into());
    }
    let root = module
        .root
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let mut result = Vec::new();
    for mapping in &module.files {
        if !relative_path(&mapping.source) || !relative_path(&mapping.target) {
            return Err("invalid mapping path".into());
        }
        let base = match mapping.base.as_str() {
            "home" => &paths.home,
            "config" => &paths.config,
            _ => return Err("invalid destination base".into()),
        };
        let path = base.join(&mapping.target);
        check_parent(&path)?;
        let override_path = root.join(os).join(&mapping.source);
        let source = if fs::symlink_metadata(&override_path).is_ok() {
            override_path
        } else {
            root.join("common").join(&mapping.source)
        };
        let source = source.canonicalize().map_err(|error| error.to_string())?;
        if !source.starts_with(&root) {
            return Err("configuration source escapes module".into());
        }
        let mut content = fs::read(source).map_err(|error| error.to_string())?;
        if let Some(format) = &mapping.template {
            let input = String::from_utf8(content).map_err(|_| "template must be UTF-8")?;
            content = render(&input, format, &module.variables, values)?.into_bytes();
        }
        if result.iter().any(|file: &DesiredFile| file.path == path) {
            return Err("duplicate configuration destination".into());
        }
        result.push(DesiredFile {
            path,
            content,
            local_only: mapping.local_only,
        });
    }
    Ok(result)
}

fn render(
    input: &str,
    format: &str,
    declarations: &BTreeMap<String, String>,
    values: &BTreeMap<String, String>,
) -> Result<String, String> {
    let mut remaining = input;
    let mut output = String::new();
    while let Some((literal, variable)) = remaining.split_once("{{") {
        output.push_str(literal);
        let (name, tail) = variable
            .split_once("}}")
            .ok_or("unclosed template variable")?;
        let resolver = declarations
            .get(name)
            .ok_or_else(|| format!("undeclared variable: {name}"))?;
        let value = values
            .get(resolver)
            .ok_or_else(|| format!("unresolved variable: {name}"))?;
        let encoded = match format {
            "text" => value.to_owned(),
            "shell" => format!("'{}'", value.replace('\'', "'\"'\"'")),
            "json" => serde_json::to_string(value).map_err(|error| error.to_string())?,
            _ => return Err("unknown template format".into()),
        };
        output.push_str(&encoded);
        remaining = tail;
    }
    output.push_str(remaining);
    Ok(output)
}
