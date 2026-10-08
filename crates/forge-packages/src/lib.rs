//! Curated package identity and planning. Pacman owns repository dependency resolution.
use std::{error::Error, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Repository {
    Msys,
    Ucrt64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Package {
    pub name: &'static str,
    pub target: &'static str,
    pub repository: Repository,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    pub packages: Vec<Package>,
}

/// Errors are structured so desktop and CLI callers can handle them independently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    EmptyRequest,
    UnknownPackage(String),
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRequest => f.write_str("Request at least one package."),
            Self::UnknownPackage(name) => {
                write!(f, "Unknown package '{name}'. Supported packages: ")?;
                for (index, package) in CURATED.iter().enumerate() {
                    if index != 0 {
                        f.write_str(", ")?;
                    }
                    f.write_str(package.name)?;
                }
                f.write_str(".")
            }
        }
    }
}
impl Error for PlanError {}

// Identities verified against the MSYS2 index; see docs/catalog.md.
const CURATED: &[Package] = &[
    Package {
        name: "bash",
        target: "bash",
        repository: Repository::Msys,
    },
    Package {
        name: "git",
        target: "git",
        repository: Repository::Msys,
    },
    Package {
        name: "node",
        target: "mingw-w64-ucrt-x86_64-nodejs",
        repository: Repository::Ucrt64,
    },
    Package {
        name: "python",
        target: "mingw-w64-ucrt-x86_64-python",
        repository: Repository::Ucrt64,
    },
    Package {
        name: "ripgrep",
        target: "mingw-w64-ucrt-x86_64-ripgrep",
        repository: Repository::Ucrt64,
    },
    Package {
        name: "zsh",
        target: "zsh",
        repository: Repository::Msys,
    },
];

/// A catalog of supported identities. No network or installation side effects.
pub struct Catalog {
    packages: &'static [Package],
}

impl Catalog {
    pub fn curated() -> Self {
        Self { packages: CURATED }
    }

    /// Validate the whole request and preserve first-request ordering.
    /// Pacman dependency solving and package versions are intentionally not part of this plan.
    pub fn plan<'a>(
        &self,
        names: impl IntoIterator<Item = &'a str>,
    ) -> Result<InstallPlan, PlanError> {
        let mut packages: Vec<Package> = Vec::new();
        for name in names {
            let package = self
                .packages
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| PlanError::UnknownPackage(name.to_owned()))?;
            if !packages.iter().any(|p| p.target == package.target) {
                packages.push(*package);
            }
        }
        if packages.is_empty() {
            return Err(PlanError::EmptyRequest);
        }
        Ok(InstallPlan { packages })
    }
}
