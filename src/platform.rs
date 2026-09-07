#[derive(Debug, PartialEq, Eq)]
pub enum OperatingSystem {
    Ubuntu,
    Macos,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    Aarch64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Platform {
    pub os: OperatingSystem,
    pub architecture: Architecture,
    pub version: Vec<u32>,
}

impl Platform {
    pub fn parse(kernel: &str, architecture: &str, release: &str) -> Result<Self, String> {
        let architecture = match architecture {
            "x86_64" => Architecture::X86_64,
            "arm64" | "aarch64" => Architecture::Aarch64,
            value => return Err(format!("unsupported architecture: {value}")),
        };
        let (os, version) = match kernel {
            "Darwin" => (OperatingSystem::Macos, release.trim().to_owned()),
            "Linux" => {
                let id = release_field(release, "ID")?;
                if id != "ubuntu" {
                    return Err(format!("unsupported distribution: {id}"));
                }
                (
                    OperatingSystem::Ubuntu,
                    release_field(release, "VERSION_ID")?,
                )
            }
            value => return Err(format!("unsupported operating system: {value}")),
        };
        let version = version
            .split('.')
            .map(|part| {
                if part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()) {
                    return Err("invalid operating system version".to_owned());
                }
                part.parse::<u32>()
                    .map_err(|_| "invalid operating system version".to_owned())
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            os,
            architecture,
            version,
        })
    }
}

fn release_field(release: &str, key: &str) -> Result<String, String> {
    let values: Vec<_> = release
        .lines()
        .filter_map(|line| line.trim().split_once('='))
        .filter(|(name, _)| *name == key)
        .map(|(_, value)| value)
        .collect();
    if values.len() != 1 {
        return Err(format!("expected one {key} field"));
    }
    let value = values[0];
    if let Some(value) = value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
    {
        Ok(value.to_owned())
    } else if let Some(value) = value
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
    {
        Ok(value.to_owned())
    } else {
        Ok(value.to_owned())
    }
}
