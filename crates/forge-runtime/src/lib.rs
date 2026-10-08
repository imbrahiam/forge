//! MSYS2 shell launch planning. Plans describe Windows processes; they do not execute them.
use std::{
    collections::BTreeMap,
    ffi::OsString,
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
}

/// A native process description. Keep arguments separate when passing it to a PTY.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchPlan {
    pub executable: PathBuf,
    pub args: Vec<OsString>,
    pub cwd: PathBuf,
    /// Overrides applied to the child only; all other variables are inherited.
    pub env: BTreeMap<OsString, OsString>,
}

#[derive(Debug, Clone)]
pub struct Runtime {
    root: PathBuf,
}

impl Runtime {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn plan_shell(&self, shell: Shell, cwd: impl AsRef<Path>) -> io::Result<LaunchPlan> {
        let root = std::path::absolute(&self.root)?;
        let cwd = std::path::absolute(cwd)?;
        let filename = match shell {
            Shell::Bash => "bash.exe",
            Shell::Zsh => "zsh.exe",
        };
        let executable = root.join("usr/bin").join(filename);
        if !executable.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Missing shell executable: {}. Check the MSYS2 runtime root.",
                    executable.display()
                ),
            ));
        }
        let dll = root.join("usr/bin/msys-2.0.dll");
        if !dll.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Missing POSIX runtime: {}. Use a complete MSYS2 installation.",
                    dll.display()
                ),
            ));
        }
        let profile = root.join("etc/profile");
        if !profile.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Missing login profile: {}. Use a complete MSYS2 installation.",
                    profile.display()
                ),
            ));
        }
        if shell == Shell::Zsh && !root.join("etc/zprofile").is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Missing Zsh login bridge: {}. Install the complete MSYS2 zsh package.",
                    root.join("etc/zprofile").display()
                ),
            ));
        }
        if !cwd.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Invalid working directory: {}. Choose an existing directory.",
                    cwd.display()
                ),
            ));
        }
        Ok(LaunchPlan {
            executable,
            args: vec!["--login".into(), "-i".into()],
            cwd,
            env: BTreeMap::from([
                ("MSYSTEM".into(), "UCRT64".into()),
                ("CHERE_INVOKING".into(), "1".into()),
                ("MSYS2_PATH_TYPE".into(), "inherit".into()),
            ]),
        })
    }
}
