use gabriel::process::{Invocation, ProcessOutput, Runner, SystemRunner, brew_prefix};
use std::{cell::RefCell, path::PathBuf};

#[test]
fn passes_arguments_literally_without_shell_evaluation() {
    let output = SystemRunner
        .run(&Invocation {
            program: "/usr/bin/printf".into(),
            arguments: vec!["%s".into(), "$(touch /tmp/should-not-exist); spaces".into()],
        })
        .unwrap();
    assert!(output.success);
    assert_eq!(output.stdout, "$(touch /tmp/should-not-exist); spaces");
}

#[test]
fn does_not_expose_arguments_in_spawn_errors() {
    let error = SystemRunner
        .run(&Invocation {
            program: "/nonexistent/gabriel-test".into(),
            arguments: vec!["private-credential".into()],
        })
        .err()
        .unwrap();
    assert!(!error.contains("private-credential"));
}

struct BrewProbe {
    calls: RefCell<Vec<Invocation>>,
    prefix: &'static str,
}

impl Runner for BrewProbe {
    fn run(&self, invocation: &Invocation) -> Result<ProcessOutput, String> {
        self.calls.borrow_mut().push(invocation.clone());
        if invocation.program == std::path::Path::new("/missing/brew") {
            return Err("not found".into());
        }
        Ok(ProcessOutput {
            success: true,
            stdout: self.prefix.into(),
        })
    }
}

#[test]
fn queries_actual_brew_prefix_after_missing_path_candidate() {
    let runner = BrewProbe {
        calls: RefCell::new(vec![]),
        prefix: "/custom brew\n",
    };
    let prefix = brew_prefix(&runner, &["/missing/brew".into(), "/known/brew".into()]).unwrap();
    assert_eq!(prefix, Some(PathBuf::from("/custom brew")));
    assert_eq!(runner.calls.borrow()[1].arguments, ["--prefix"]);
}

#[test]
fn rejects_invalid_prefix_reported_by_brew() {
    let runner = BrewProbe {
        calls: RefCell::new(vec![]),
        prefix: "relative",
    };
    assert!(brew_prefix(&runner, &["/known/brew".into()]).is_err());
}
