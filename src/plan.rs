use crate::{
    catalog::{Catalog, Module, validate_version},
    platform::{Architecture, OperatingSystem, Platform},
};
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum ActionKind {
    Install,
    Upgrade,
    Configure,
}

#[derive(Debug)]
pub struct Action {
    pub module: String,
    pub kind: ActionKind,
}

#[derive(Debug)]
pub struct Plan {
    pub actions: Vec<Action>,
    pub excluded: BTreeMap<String, String>,
}

pub fn os_name(platform: &Platform) -> &'static str {
    match platform.os {
        OperatingSystem::Ubuntu => "ubuntu",
        OperatingSystem::Macos => "macos",
    }
}

pub fn version_at_least(actual: &[u32], minimum: &[u32]) -> bool {
    for index in 0..actual.len().max(minimum.len()) {
        match actual
            .get(index)
            .unwrap_or(&0)
            .cmp(minimum.get(index).unwrap_or(&0))
        {
            std::cmp::Ordering::Greater => return true,
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Equal => {}
        }
    }
    true
}

pub fn compatible(module: &Module, platform: &Platform, graphical: bool) -> Result<(), String> {
    let compatibility = module
        .platforms
        .get(os_name(platform))
        .ok_or("platform is not supported")?;
    let architecture = match platform.architecture {
        Architecture::X86_64 => "x86_64",
        Architecture::Aarch64 => "aarch64",
    };
    if !compatibility
        .architectures
        .iter()
        .any(|value| value == architecture)
    {
        return Err("architecture is not supported".into());
    }
    if !version_at_least(
        &platform.version,
        &validate_version(&compatibility.minimum_os)?,
    ) {
        return Err(format!("requires OS {} or newer", compatibility.minimum_os));
    }
    if compatibility.graphical && !graphical {
        return Err("requires a graphical session".into());
    }
    Ok(())
}

pub fn build(
    catalog: &Catalog,
    platform: &Platform,
    selected: &[String],
    all: bool,
    graphical: bool,
    installed: &BTreeMap<String, Option<Vec<u32>>>,
) -> Result<Plan, String> {
    let selected = if all {
        catalog.modules.keys().cloned().collect::<Vec<_>>()
    } else {
        selected.to_vec()
    };
    let mut plan = Plan {
        actions: Vec::new(),
        excluded: BTreeMap::new(),
    };
    for id in catalog.ordered(&selected)? {
        let module = &catalog.modules[&id];
        let incompatibility = compatible(module, platform, graphical).err().or_else(|| {
            module
                .dependencies
                .iter()
                .find(|dependency| plan.excluded.contains_key(*dependency))
                .map(|dependency| format!("dependency {dependency} is incompatible"))
        });
        if let Some(reason) = incompatibility {
            if !all {
                return Err(format!("{id}: {reason}"));
            }
            plan.excluded.insert(id, reason);
            continue;
        }
        if let Some(detect) = &module.detect {
            let kind = match installed.get(&id) {
                None => Some(ActionKind::Install),
                Some(None) => return Err(format!("{id}: cannot determine installed version")),
                Some(Some(version))
                    if !version_at_least(version, &validate_version(&detect.minimum_version)?) =>
                {
                    Some(ActionKind::Upgrade)
                }
                _ => None,
            };
            if let Some(kind) = kind {
                if module.platforms[os_name(platform)].provider.is_none() {
                    return Err(format!("{id}: no installation provider"));
                }
                plan.actions.push(Action {
                    module: id.clone(),
                    kind,
                });
            }
        }
        if !module.files.is_empty() {
            plan.actions.push(Action {
                module: id,
                kind: ActionKind::Configure,
            });
        }
    }
    Ok(plan)
}

pub fn build_upgrade(
    catalog: &Catalog,
    platform: &Platform,
    selected: &[String],
    graphical: bool,
    installed: &BTreeMap<String, Option<Vec<u32>>>,
) -> Result<Plan, String> {
    let mut plan = build(catalog, platform, selected, false, graphical, installed)?;
    for id in selected {
        let module = catalog
            .modules
            .get(id)
            .ok_or_else(|| format!("unknown module: {id}"))?;
        if module.detect.is_some() {
            match installed.get(id) {
                Some(Some(_)) => {
                    if !plan
                        .actions
                        .iter()
                        .any(|action| action.module == *id && action.kind == ActionKind::Upgrade)
                    {
                        let index = plan
                            .actions
                            .iter()
                            .position(|action| action.module == *id)
                            .unwrap_or(plan.actions.len());
                        plan.actions.insert(
                            index,
                            Action {
                                module: id.clone(),
                                kind: ActionKind::Upgrade,
                            },
                        );
                    }
                }
                Some(None) => return Err(format!("{id}: cannot determine installed version")),
                None => return Err(format!("{id}: is not installed; use install first")),
            }
        }
    }
    Ok(plan)
}
