use clap::{Parser, Subcommand};
use dialoguer::MultiSelect;
use gabriel::{
    catalog::Catalog,
    config::compose,
    paths::{UserPaths, brew_candidates},
    plan::{self, ActionKind},
    platform::{OperatingSystem, Platform},
    process::{Invocation, Runner, SystemRunner, brew_prefix},
    provider,
    state::Store,
};
use std::{
    collections::BTreeMap,
    env, fs,
    io::{self, IsTerminal, Write},
    path::PathBuf,
    process::ExitCode,
};

#[derive(Parser)]
#[command(
    version,
    about = "Install and maintain your personal Ubuntu and macOS environment"
)]
struct Cli {
    #[arg(long, global = true)]
    source: Option<PathBuf>,
    #[arg(long, global = true)]
    dry_run: bool,
    #[arg(long, global = true)]
    yes: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Setup {
        #[arg(long)]
        all: bool,
        #[arg(long)]
        profile: Option<String>,
    },
    Install {
        #[arg(required = true)]
        modules: Vec<String>,
    },
    Status,
    Doctor,
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    Upgrade {
        #[arg(required = true)]
        modules: Vec<String>,
    },
    #[command(name = "self")]
    Executable {
        #[command(subcommand)]
        command: ExecutableCommand,
    },
}

#[derive(Subcommand)]
enum ConfigCommand {
    Update,
    Restore {
        transaction: String,
    },
    Resolve {
        module: String,
        #[arg(long)]
        file: PathBuf,
        #[arg(long, value_parser = ["local", "incoming"])]
        r#use: String,
    },
}

#[derive(Subcommand)]
enum ExecutableCommand {
    Update,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            eprintln!("{message}");
            ExitCode::from(code)
        }
    }
}

fn run(cli: Cli) -> Result<(), (u8, String)> {
    if matches!(
        cli.command,
        Commands::Setup {
            all: false,
            profile: None
        }
    ) && !io::stdin().is_terminal()
    {
        return Err((
            2,
            "noninteractive setup requires --all or --profile; use install for explicit modules"
                .into(),
        ));
    }
    let runner = SystemRunner;
    if matches!(
        cli.command,
        Commands::Executable {
            command: ExecutableCommand::Update
        }
    ) {
        let url = env::var("GABRIEL_RELEASE_URL")
            .map_err(|_| (2, "GABRIEL_RELEASE_URL is required for self update".into()))?;
        let checksum = env::var("GABRIEL_RELEASE_SHA256").map_err(|_| {
            (
                2,
                "GABRIEL_RELEASE_SHA256 is required for self update".into(),
            )
        })?;
        let destination = env::current_exe().map_err(|error| (1, error.to_string()))?;
        if cli.dry_run {
            println!("Would update {}", destination.display());
            return Ok(());
        }
        let download = env::temp_dir().join(format!("gabriel-release-{}", std::process::id()));
        let output = runner
            .run(&Invocation {
                program: "curl".into(),
                arguments: vec![
                    "--fail".into(),
                    "--silent".into(),
                    "--show-error".into(),
                    "-o".into(),
                    download.to_string_lossy().into_owned(),
                    url,
                ],
            })
            .map_err(|error| (1, error))?;
        if !output.success {
            return Err((1, "release download failed".into()));
        }
        let binary = fs::read(&download).map_err(|error| (1, error.to_string()))?;
        let _ = fs::remove_file(&download);
        let staged = gabriel::release::prepare(&binary, &checksum, &destination)
            .map_err(|error| (1, error))?;
        gabriel::release::commit(&staged, &destination).map_err(|error| (1, error))?;
        return Ok(());
    }
    let query = |program: &str, arguments: &[&str]| -> Result<String, (u8, String)> {
        let output = runner
            .run(&Invocation {
                program: program.into(),
                arguments: arguments.iter().map(|value| (*value).into()).collect(),
            })
            .map_err(|error| (1, error))?;
        if !output.success {
            return Err((1, format!("environment query failed: {program}")));
        }
        Ok(output.stdout.trim().into())
    };
    let kernel = query("uname", &["-s"])?;
    let architecture = query("uname", &["-m"])?;
    let release = if kernel == "Darwin" {
        query("sw_vers", &["-productVersion"])?
    } else {
        fs::read_to_string("/etc/os-release").map_err(|error| (2, error.to_string()))?
    };
    let platform = Platform::parse(&kernel, &architecture, &release).map_err(|error| (2, error))?;
    let home = env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or((2, "HOME is not set".into()))?;
    let config = env::var_os("XDG_CONFIG_HOME").map(PathBuf::from);
    let state = env::var_os("XDG_STATE_HOME").map(PathBuf::from);
    let paths = UserPaths::resolve(&platform.os, &home, config.as_deref(), state.as_deref())
        .map_err(|error| (2, error))?;
    if let Commands::Config {
        command: ConfigCommand::Restore { transaction },
    } = &cli.command
    {
        if cli.dry_run {
            println!("Restore transaction {transaction}");
            return Ok(());
        }
        confirm(cli.yes)?;
        return Store::open(&paths.state)
            .and_then(|mut store| store.restore(transaction))
            .map_err(|error| (1, error));
    }
    let source = cli
        .source
        .clone()
        .or_else(|| env::var_os("GABRIEL_SOURCE").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let source = if source.join("modules").is_dir() {
        source
    } else {
        paths.state.join("catalog")
    };
    let catalog = Catalog::load(&source).map_err(|error| (2, error))?;
    let path = env::var_os("PATH")
        .map(|path| env::split_paths(&path).collect::<Vec<_>>())
        .unwrap_or_default();
    let prefix =
        brew_prefix(&runner, &brew_candidates(&platform.os, &path)).map_err(|error| (1, error))?;
    let mut values = BTreeMap::from([
        ("home".into(), paths.home.to_string_lossy().into_owned()),
        ("config".into(), paths.config.to_string_lossy().into_owned()),
    ]);
    if let Some(prefix) = &prefix {
        values.insert("brew_prefix".into(), prefix.to_string_lossy().into_owned());
    }
    if let Commands::Config {
        command:
            ConfigCommand::Resolve {
                module,
                file,
                r#use,
            },
    } = &cli.command
    {
        let desired = compose(
            catalog
                .modules
                .get(module)
                .ok_or((2, format!("unknown module: {module}")))?,
            plan::os_name(&platform),
            &paths,
            &values,
        )
        .map_err(|error| (2, error))?
        .into_iter()
        .find(|candidate| candidate.path == *file || candidate.path.ends_with(file))
        .ok_or((
            2,
            format!("file is not managed by module: {}", file.display()),
        ))?;
        if cli.dry_run {
            println!("Resolve {} {}", module, desired.path.display());
            return Ok(());
        }
        confirm(cli.yes)?;
        return Store::open(&paths.state)
            .and_then(|mut store| {
                store.resolve(module, &desired.path, &desired.content, r#use == "incoming")
            })
            .map_err(|error| (1, error));
    }
    let graphical = platform.os == OperatingSystem::Macos
        || env::var_os("DISPLAY").is_some()
        || env::var_os("WAYLAND_DISPLAY").is_some();
    let mut installed = BTreeMap::new();
    for module in catalog.modules.values() {
        if let Some(detect) = &module.detect {
            let program = prefix
                .as_ref()
                .map(|prefix| prefix.join("bin").join(&detect.program))
                .filter(|candidate| candidate.is_file())
                .unwrap_or_else(|| PathBuf::from(&detect.program));
            if let Ok(output) = runner.run(&Invocation {
                program,
                arguments: detect.args.clone(),
            }) && output.success
            {
                let version = output.stdout.split_whitespace().find_map(|word| {
                    let value =
                        word.trim_start_matches(|character: char| !character.is_ascii_digit());
                    let value =
                        value.trim_end_matches(|character: char| !character.is_ascii_digit());
                    gabriel::catalog::validate_version(value).ok()
                });
                installed.insert(module.id.clone(), version);
            }
        }
    }
    let config_only = matches!(
        &cli.command,
        Commands::Config {
            command: ConfigCommand::Update
        }
    );
    let (selected, all) = match &cli.command {
        Commands::Install { modules } | Commands::Upgrade { modules } => (modules.clone(), false),
        Commands::Setup { all: true, .. } => (Vec::new(), true),
        Commands::Config {
            command: ConfigCommand::Update,
        } => (Vec::new(), true),
        Commands::Setup {
            profile: Some(profile),
            ..
        } => {
            if !profile
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            {
                return Err((2, "invalid profile identifier".into()));
            }
            #[derive(serde::Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Profile {
                modules: Vec<String>,
            }
            let profile: Profile = toml::from_str(
                &fs::read_to_string(source.join("profiles").join(format!("{profile}.toml")))
                    .map_err(|error| (2, error.to_string()))?,
            )
            .map_err(|error| (2, error.to_string()))?;
            (profile.modules, false)
        }
        Commands::Setup { .. } => {
            let modules = catalog.modules.values().collect::<Vec<_>>();
            let labels = modules
                .iter()
                .map(|module| format!("{} - {}", module.id, module.description))
                .collect::<Vec<_>>();
            let defaults = vec![true; labels.len()];
            let choices = MultiSelect::new()
                .with_prompt("Selecciona las configuraciones")
                .items(&labels)
                .defaults(&defaults)
                .interact()
                .map_err(|error| (1, error.to_string()))?;
            let selected = choices
                .into_iter()
                .map(|index| modules[index].id.clone())
                .collect::<Vec<_>>();
            if selected.is_empty() {
                return Err((2, "no modules selected".into()));
            }
            (selected, false)
        }
        Commands::Status | Commands::Doctor => {
            let mut failed = false;
            let pending = fs::read_dir(paths.state.join("transactions"))
                .ok()
                .map(|entries| {
                    entries
                        .flatten()
                        .filter_map(|entry| fs::read_to_string(entry.path()).ok())
                        .filter(|content| content.contains("\"status\":\"pending\""))
                        .count()
                })
                .unwrap_or(0);
            println!(
                "platform={} pending_transactions={pending}",
                plan::os_name(&platform)
            );
            failed |= pending > 0;
            for module in catalog.modules.values() {
                let compatibility = plan::compatible(module, &platform, graphical);
                if let Err(reason) = compatibility {
                    println!("{}: excluded: {reason}", module.id);
                    continue;
                }
                let ready = module.detect.is_none()
                    || installed.get(&module.id).is_some_and(Option::is_some);
                let files = compose(module, plan::os_name(&platform), &paths, &values);
                let configured = files
                    .as_ref()
                    .is_ok_and(|files| files.iter().all(|file| fs::read(&file.path).is_ok()));
                let version = installed
                    .get(&module.id)
                    .and_then(|value| value.as_ref())
                    .map(|value| {
                        value
                            .iter()
                            .map(u32::to_string)
                            .collect::<Vec<_>>()
                            .join(".")
                    })
                    .unwrap_or_else(|| "missing".into());
                println!(
                    "{}: installed={ready} version={version} configured={configured}",
                    module.id
                );
                failed |= !ready || !configured;
            }
            return if matches!(cli.command, Commands::Doctor) && failed {
                Err((
                    1,
                    "doctor found missing dependencies or configurations; run setup".into(),
                ))
            } else {
                Ok(())
            };
        }
        _ => return Err((1, "unsupported command".into())),
    };
    let upgrading = matches!(cli.command, Commands::Upgrade { .. });
    let mut plan = if upgrading {
        plan::build_upgrade(&catalog, &platform, &selected, graphical, &installed)
    } else {
        plan::build(&catalog, &platform, &selected, all, graphical, &installed)
    }
    .map_err(|error| (2, error))?;
    if config_only {
        plan.actions.retain(|action| {
            action.kind == ActionKind::Configure
                && catalog.modules[&action.module]
                    .detect
                    .as_ref()
                    .is_none_or(|detect| {
                        installed.get(&action.module).is_some_and(Option::is_some)
                            && gabriel::catalog::validate_version(&detect.minimum_version).is_ok()
                    })
        });
        plan.excluded.extend(
            catalog
                .modules
                .values()
                .filter(|module| {
                    module
                        .detect
                        .as_ref()
                        .is_some_and(|_| !installed.get(&module.id).is_some_and(Option::is_some))
                })
                .map(|module| {
                    (
                        module.id.clone(),
                        "dependency is not installed; run setup".into(),
                    )
                }),
        );
    }
    for (module, reason) in &plan.excluded {
        println!("Excluded {module}: {reason}");
    }
    let mut configurations = BTreeMap::new();
    for action in &plan.actions {
        println!("{:?} {}", action.kind, action.module);
        if action.kind == ActionKind::Configure {
            let files = compose(
                &catalog.modules[&action.module],
                plan::os_name(&platform),
                &paths,
                &values,
            )
            .map_err(|error| (2, error))?;
            configurations.insert(action.module.clone(), files);
        }
    }
    if cli.dry_run {
        return Ok(());
    }
    confirm(cli.yes)?;
    for action in &plan.actions {
        if action.kind == ActionKind::Configure {
            continue;
        }
        let module = &catalog.modules[&action.module];
        let compatibility = module
            .platforms
            .get(plan::os_name(&platform))
            .ok_or((2, format!("{} is incompatible", action.module)))?;
        let package_provider = compatibility
            .provider
            .as_ref()
            .ok_or((2, format!("{} has no provider", action.module)))?;
        provider::install(
            &runner,
            package_provider,
            action.kind == ActionKind::Upgrade,
            prefix.as_deref(),
        )
        .map_err(|error| (1, format!("{}: {error}", action.module)))?;
    }
    let mut store = Store::open(&paths.state).map_err(|error| (1, error))?;
    for (module, files) in configurations {
        let transaction = store.apply(&module, &files).map_err(|error| (1, error))?;
        println!(
            "{module}: {}",
            transaction.as_deref().unwrap_or("unchanged")
        );
    }
    Ok(())
}

fn confirm(yes: bool) -> Result<(), (u8, String)> {
    if yes {
        return Ok(());
    }
    if !io::stdin().is_terminal() {
        return Err((
            2,
            "use --yes to apply an explicit selection noninteractively".into(),
        ));
    }
    print!("Apply these changes? [y/N] ");
    io::stdout()
        .flush()
        .map_err(|error| (1, error.to_string()))?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|error| (1, error.to_string()))?;
    if input.trim().eq_ignore_ascii_case("y") {
        Ok(())
    } else {
        Err((1, "cancelled".into()))
    }
}
