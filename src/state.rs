use crate::config::{DesiredFile, check_parent};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::Write,
    os::unix::fs::{OpenOptionsExt, PermissionsExt, symlink},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

type Baseline = BTreeMap<PathBuf, Vec<u8>>;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    schema_version: u32,
    modules: BTreeMap<String, Baseline>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Snapshot {
    Absent,
    File { content: Vec<u8>, mode: u32 },
    Link { target: PathBuf },
}

impl Snapshot {
    fn read(path: &Path) -> Result<Self, String> {
        check_parent(path)?;
        match fs::symlink_metadata(path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::Absent),
            Err(error) => Err(error.to_string()),
            Ok(metadata) if metadata.file_type().is_symlink() => Ok(Self::Link {
                target: fs::read_link(path).map_err(|error| error.to_string())?,
            }),
            Ok(metadata) if metadata.is_file() => Ok(Self::File {
                content: fs::read(path).map_err(|error| error.to_string())?,
                mode: metadata.permissions().mode() & 0o777,
            }),
            Ok(_) => Err(format!(
                "destination is not a regular file or link: {}",
                path.display()
            )),
        }
    }

    fn bytes(&self) -> Option<&[u8]> {
        match self {
            Self::File { content, .. } => Some(content),
            _ => None,
        }
    }

    fn write(&self, path: &Path) -> Result<(), String> {
        check_parent(path)?;
        let parent = path.parent().ok_or("missing parent")?;
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        match self {
            Self::Absent => match fs::remove_file(path) {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.to_string()),
            },
            Self::File { content, mode } => atomic_write(path, content, *mode)?,
            Self::Link { target } => {
                let staging = parent.join(format!(".gabriel-{}", identifier()));
                symlink(target, &staging).map_err(|error| error.to_string())?;
                if let Err(error) = fs::rename(&staging, path) {
                    let _ = fs::remove_file(staging);
                    return Err(error.to_string());
                }
            }
        }
        sync_directory(parent)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Change {
    path: PathBuf,
    before: Snapshot,
    after: Snapshot,
}

#[derive(Serialize, Deserialize)]
struct Transaction {
    schema_version: u32,
    id: String,
    module: String,
    status: String,
    before: Option<Baseline>,
    after: Baseline,
    changes: Vec<Change>,
}

pub struct Store {
    root: PathBuf,
    _lock: File,
    state: State,
}

impl Store {
    pub fn open(root: &Path) -> Result<Self, String> {
        check_parent(&root.join("state.json"))?;
        let state = if root.join("state.json").exists() {
            read_json::<State>(&root.join("state.json"))?
        } else {
            State {
                schema_version: 1,
                modules: BTreeMap::new(),
            }
        };
        if state.schema_version != 1 {
            return Err("unsupported state schema".into());
        }
        fs::create_dir_all(root).map_err(|error| error.to_string())?;
        fs::set_permissions(root, fs::Permissions::from_mode(0o700))
            .map_err(|error| error.to_string())?;
        let lock_path = root.join("lock");
        reject_link(&lock_path)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .open(lock_path)
            .map_err(|error| error.to_string())?;
        lock.try_lock_exclusive()
            .map_err(|_| "another operation owns the state lock")?;
        let state = if root.join("state.json").exists() {
            read_json::<State>(&root.join("state.json"))?
        } else {
            state
        };
        if state.schema_version != 1 {
            return Err("unsupported state schema".into());
        }
        let mut store = Self {
            root: root.to_owned(),
            _lock: lock,
            state,
        };
        fs::create_dir_all(store.root.join("transactions")).map_err(|error| error.to_string())?;
        store.recover()?;
        Ok(store)
    }

    pub fn apply(&mut self, module: &str, files: &[DesiredFile]) -> Result<Option<String>, String> {
        self.apply_with_policy(module, files, false)
    }

    pub fn apply_with_policy(
        &mut self,
        module: &str,
        files: &[DesiredFile],
        overwrite_conflicts: bool,
    ) -> Result<Option<String>, String> {
        let baseline = self.state.modules.get(module).cloned();
        let mut after = BTreeMap::new();
        let mut changes = Vec::new();
        let mut destinations = BTreeMap::new();
        for file in files {
            if destinations.insert(file.path.clone(), ()).is_some() {
                return Err("duplicate destination".into());
            }
            for (owner, managed) in &self.state.modules {
                if owner != module && managed.contains_key(&file.path) {
                    return Err(format!("destination owned by {owner}"));
                }
            }
            let current = Snapshot::read(&file.path)?;
            if file.local_only && current != Snapshot::Absent {
                continue;
            }
            let base = baseline.as_ref().and_then(|files| files.get(&file.path));
            let decision = if baseline.is_none() || base.is_none() && current == Snapshot::Absent {
                Decision::Incoming
            } else {
                decide(
                    base.map(Vec::as_slice),
                    current.bytes(),
                    Some(&file.content),
                )
            };
            if decision == Decision::Conflict && !overwrite_conflicts {
                return Err(format!(
                    "configuration conflict: {module}: {}",
                    file.path.display()
                ));
            }
            after.insert(file.path.clone(), file.content.clone());
            if (decision == Decision::Incoming || overwrite_conflicts)
                && current.bytes() != Some(&file.content)
            {
                let mode = match &current {
                    Snapshot::File { mode, .. } => *mode,
                    _ => 0o600,
                };
                changes.push(Change {
                    path: file.path.clone(),
                    before: current,
                    after: Snapshot::File {
                        content: file.content.clone(),
                        mode,
                    },
                });
            }
        }
        if let Some(baseline) = &baseline {
            for (path, content) in baseline {
                if destinations.contains_key(path) {
                    continue;
                }
                let current = Snapshot::read(path)?;
                if decide(Some(content), current.bytes(), None) == Decision::Conflict
                    && !overwrite_conflicts
                {
                    return Err(format!(
                        "configuration conflict on removed file: {}",
                        path.display()
                    ));
                }
                if current != Snapshot::Absent {
                    changes.push(Change {
                        path: path.clone(),
                        before: current,
                        after: Snapshot::Absent,
                    });
                }
            }
        }
        if changes.is_empty()
            && (baseline.as_ref() == Some(&after) || baseline.is_none() && after.is_empty())
        {
            return Ok(None);
        }
        let mut transaction = Transaction {
            schema_version: 1,
            id: identifier(),
            module: module.into(),
            status: "pending".into(),
            before: baseline,
            after,
            changes,
        };
        self.save_transaction(&transaction)?;
        for change in &transaction.changes {
            if Snapshot::read(&change.path)? != change.before {
                self.rollback(&mut transaction)?;
                return Err("configuration changed during apply".into());
            }
            if let Err(error) = change.after.write(&change.path) {
                self.rollback(&mut transaction)?;
                return Err(error);
            }
        }
        self.state
            .modules
            .insert(module.into(), transaction.after.clone());
        self.save_state()?;
        transaction.status = "committed".into();
        self.save_transaction(&transaction)?;
        Ok(Some(transaction.id))
    }

    pub fn restore(&mut self, transaction: &str) -> Result<(), String> {
        if transaction.is_empty()
            || !transaction
                .bytes()
                .all(|byte| byte.is_ascii_digit() || byte == b'-')
        {
            return Err("invalid transaction id".into());
        }
        let mut original: Transaction = read_json(
            &self
                .root
                .join("transactions")
                .join(format!("{transaction}.json")),
        )?;
        if original.schema_version != 1 || original.status != "committed" {
            return Err("transaction is not restorable".into());
        }
        if self.state.modules.get(&original.module) != Some(&original.after) {
            return Err("restoration conflicts with a newer module revision".into());
        }
        for change in &original.changes {
            if Snapshot::read(&change.path)? != change.after {
                return Err(format!("restoration conflict: {}", change.path.display()));
            }
        }
        let mut reversal = Transaction {
            schema_version: 1,
            id: identifier(),
            module: original.module.clone(),
            status: "pending".into(),
            before: Some(original.after.clone()),
            after: original.before.clone().unwrap_or_default(),
            changes: original
                .changes
                .iter()
                .rev()
                .map(|change| Change {
                    path: change.path.clone(),
                    before: change.after.clone(),
                    after: change.before.clone(),
                })
                .collect(),
        };
        self.save_transaction(&reversal)?;
        for change in &reversal.changes {
            if let Err(error) = change.after.write(&change.path) {
                self.rollback(&mut reversal)?;
                return Err(error);
            }
        }
        match &original.before {
            Some(before) => {
                self.state
                    .modules
                    .insert(original.module.clone(), before.clone());
            }
            None => {
                self.state.modules.remove(&original.module);
            }
        }
        self.save_state()?;
        reversal.status = "committed".into();
        self.save_transaction(&reversal)?;
        original.status = "restored".into();
        self.save_transaction(&original)
    }

    pub fn resolve(
        &mut self,
        module: &str,
        path: &Path,
        incoming: &[u8],
        use_incoming: bool,
    ) -> Result<(), String> {
        let baseline = self
            .state
            .modules
            .get(module)
            .and_then(|files| files.get(path))
            .cloned()
            .ok_or("file is not managed by module")?;
        let current = Snapshot::read(path)?;
        if decide(Some(&baseline), current.bytes(), Some(incoming)) != Decision::Conflict {
            return Err("file does not have a conflict".into());
        }
        if use_incoming {
            let mode = match current {
                Snapshot::File { mode, .. } => mode,
                _ => 0o600,
            };
            Snapshot::File {
                content: incoming.to_vec(),
                mode,
            }
            .write(path)?;
        }
        if let Some(files) = self.state.modules.get_mut(module) {
            files.insert(path.to_owned(), incoming.to_vec());
        }
        self.save_state()
    }

    fn recover(&mut self) -> Result<(), String> {
        let entries =
            fs::read_dir(self.root.join("transactions")).map_err(|error| error.to_string())?;
        for entry in entries {
            let path = entry.map_err(|error| error.to_string())?.path();
            if path
                .extension()
                .is_some_and(|extension| extension == "json")
            {
                let mut transaction: Transaction = read_json(&path)?;
                if transaction.schema_version != 1 {
                    return Err("unsupported transaction schema".into());
                }
                if transaction.status == "pending" {
                    self.rollback(&mut transaction)?;
                }
            }
        }
        Ok(())
    }

    fn rollback(&mut self, transaction: &mut Transaction) -> Result<(), String> {
        for change in transaction.changes.iter().rev() {
            let current = Snapshot::read(&change.path)?;
            if current != change.before && current != change.after {
                return Err(format!("recovery conflict: {}", change.path.display()));
            }
        }
        for change in transaction.changes.iter().rev() {
            change.before.write(&change.path)?;
        }
        match &transaction.before {
            Some(before) => {
                self.state
                    .modules
                    .insert(transaction.module.clone(), before.clone());
            }
            None => {
                self.state.modules.remove(&transaction.module);
            }
        }
        self.save_state()?;
        transaction.status = "rolled_back".into();
        self.save_transaction(transaction)
    }

    fn save_transaction(&self, transaction: &Transaction) -> Result<(), String> {
        write_json(
            &self
                .root
                .join("transactions")
                .join(format!("{}.json", transaction.id)),
            transaction,
        )
    }

    fn save_state(&self) -> Result<(), String> {
        write_json(&self.root.join("state.json"), &self.state)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    Incoming,
    Local,
    Conflict,
}

pub fn decide(base: Option<&[u8]>, local: Option<&[u8]>, incoming: Option<&[u8]>) -> Decision {
    if local == incoming || local == base {
        Decision::Incoming
    } else if incoming == base {
        Decision::Local
    } else {
        Decision::Conflict
    }
}

fn identifier() -> String {
    format!(
        "{}-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
        std::process::id()
    )
}

fn reject_link(path: &Path) -> Result<(), String> {
    if fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err("state file cannot be a symbolic link".into());
    }
    Ok(())
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    reject_link(path)?;
    serde_json::from_slice(&fs::read(path).map_err(|error| error.to_string())?)
        .map_err(|_| format!("invalid state document: {}", path.display()))
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    let content = serde_json::to_vec(value).map_err(|error| error.to_string())?;
    atomic_write(path, &content, 0o600)
}

fn atomic_write(path: &Path, content: &[u8], mode: u32) -> Result<(), String> {
    check_parent(path)?;
    let parent = path.parent().ok_or("missing parent")?;
    let temporary = parent.join(format!(".gabriel-{}", identifier()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .open(&temporary)
        .map_err(|error| error.to_string())?;
    let result = (|| {
        file.write_all(content).map_err(|error| error.to_string())?;
        file.sync_all().map_err(|error| error.to_string())?;
        fs::rename(&temporary, path).map_err(|error| error.to_string())?;
        sync_directory(parent)
    })();
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}

fn sync_directory(path: &Path) -> Result<(), String> {
    File::open(path)
        .and_then(|file| file.sync_all())
        .map_err(|error| error.to_string())
}
