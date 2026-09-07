use crate::{
    catalog::Provider,
    process::{Invocation, Runner},
};
use std::path::Path;

pub fn commands(
    provider: &Provider,
    upgrade: bool,
    brew: Option<&Path>,
) -> Result<Vec<Invocation>, String> {
    let mut result = Vec::new();
    match provider {
        Provider::Apt { package } => {
            if !upgrade {
                result.push(Invocation {
                    program: "sudo".into(),
                    arguments: vec!["apt-get".into(), "update".into()],
                });
            }
            let mut arguments = vec!["apt-get".into(), "install".into(), "--yes".into()];
            if upgrade {
                arguments.push("--only-upgrade".into());
            }
            arguments.push(package.clone());
            result.push(Invocation {
                program: "sudo".into(),
                arguments,
            });
        }
        Provider::Brew { package } | Provider::Cask { package } => {
            let prefix = brew.ok_or("Homebrew is required but not installed")?;
            let mut arguments = vec![if upgrade {
                "upgrade".into()
            } else {
                "install".into()
            }];
            if matches!(provider, Provider::Cask { .. }) {
                arguments.push("--cask".into());
            }
            arguments.push(package.clone());
            result.push(Invocation {
                program: prefix.join("bin/brew"),
                arguments,
            });
        }
    }
    Ok(result)
}

pub fn install(
    runner: &dyn Runner,
    provider: &Provider,
    upgrade: bool,
    brew: Option<&Path>,
) -> Result<(), String> {
    for command in commands(provider, upgrade, brew)? {
        if !runner.run(&command)?.success {
            return Err("package installation failed".into());
        }
    }
    Ok(())
}
