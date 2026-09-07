use gabriel::{
    catalog::Provider,
    process::{Invocation, ProcessOutput, Runner},
    provider::{commands, install},
};
use std::{cell::RefCell, path::Path};

#[test]
fn apt_uses_explicit_package_arguments_and_scoped_elevation() {
    let commands = commands(
        &Provider::Apt {
            package: "tmux".into(),
        },
        false,
        None,
    )
    .unwrap();
    assert_eq!(commands.last().unwrap().program, Path::new("sudo"));
    assert_eq!(
        commands.last().unwrap().arguments,
        ["apt-get", "install", "--yes", "tmux"]
    );
}

#[test]
fn brew_uses_detected_prefix_and_casks_only_when_declared() {
    let commands = commands(
        &Provider::Cask {
            package: "ghostty".into(),
        },
        false,
        Some(Path::new("/custom brew")),
    )
    .unwrap();
    assert_eq!(commands[0].program, Path::new("/custom brew/bin/brew"));
    assert_eq!(commands[0].arguments, ["install", "--cask", "ghostty"]);
    assert!(
        gabriel::provider::commands(
            &Provider::Brew {
                package: "tmux".into()
            },
            false,
            None
        )
        .is_err()
    );
}

#[test]
fn upgrades_only_the_named_package() {
    let commands = commands(
        &Provider::Brew {
            package: "neovim".into(),
        },
        true,
        Some(Path::new("/opt/homebrew")),
    )
    .unwrap();
    assert_eq!(commands[0].arguments, ["upgrade", "neovim"]);
}

struct Failure {
    calls: RefCell<Vec<Invocation>>,
}
impl Runner for Failure {
    fn run(&self, command: &Invocation) -> Result<ProcessOutput, String> {
        self.calls.borrow_mut().push(command.clone());
        Ok(ProcessOutput {
            success: false,
            stdout: "private detail".into(),
        })
    }
}

#[test]
fn stops_after_provider_failure_and_does_not_expose_output() {
    let runner = Failure {
        calls: RefCell::new(vec![]),
    };
    let error = install(
        &runner,
        &Provider::Apt {
            package: "tmux".into(),
        },
        false,
        None,
    )
    .unwrap_err();
    assert_eq!(runner.calls.borrow().len(), 1);
    assert!(!error.contains("private detail"));
}
