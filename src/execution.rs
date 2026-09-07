use crate::{catalog::Catalog, plan::Plan};
use std::collections::BTreeMap;

pub fn execute(
    catalog: &Catalog,
    plan: &Plan,
    mut action: impl FnMut(&crate::plan::Action) -> Result<(), String>,
) -> BTreeMap<String, String> {
    let mut failures = BTreeMap::new();
    for planned in &plan.actions {
        if failures.contains_key(&planned.module) {
            continue;
        }
        let module = &catalog.modules[&planned.module];
        if let Some(dependency) = module
            .dependencies
            .iter()
            .find(|dependency| failures.contains_key(*dependency))
        {
            failures.insert(
                planned.module.clone(),
                format!("blocked by dependency {dependency}"),
            );
            continue;
        }
        if let Err(error) = action(planned) {
            failures.insert(planned.module.clone(), error);
        }
    }
    failures
}
