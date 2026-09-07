use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invocation {
    pub program: PathBuf,
    pub arguments: Vec<String>,
}

pub struct ProcessOutput {
    pub success: bool,
    pub stdout: String,
}

pub trait Runner {
    fn run(&self, invocation: &Invocation) -> Result<ProcessOutput, String>;
}

pub struct SystemRunner;

impl Runner for SystemRunner {
    fn run(&self, invocation: &Invocation) -> Result<ProcessOutput, String> {
        let output = std::process::Command::new(&invocation.program)
            .args(&invocation.arguments)
            .output()
            .map_err(|error| format!("could not start process: {}", error.kind()))?;
        Ok(ProcessOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        })
    }
}

pub fn brew_prefix(runner: &dyn Runner, candidates: &[PathBuf]) -> Result<Option<PathBuf>, String> {
    for candidate in candidates {
        let output = runner.run(&Invocation {
            program: candidate.clone(),
            arguments: vec!["--prefix".into()],
        });
        if let Ok(output) = output
            && output.success
        {
            let prefix = PathBuf::from(output.stdout.trim());
            if !prefix.is_absolute() || output.stdout.trim().contains('\n') {
                return Err("Homebrew returned an invalid prefix".into());
            }
            return Ok(Some(prefix));
        }
    }
    Ok(None)
}
