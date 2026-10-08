//! User-facing command composition. Runtime and package behavior live in their modules.
use clap::{Parser, Subcommand, ValueEnum};
use forge_packages::{Catalog, Repository};
use forge_runtime::{Runtime, Shell};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "forge",
    version,
    about = "Unix developer tools for Windows without WSL"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inspect an existing MSYS2 runtime without changing it
    Doctor {
        #[arg(long)]
        runtime_root: PathBuf,
        #[arg(long, value_enum, default_value_t = ShellChoice::Bash)]
        shell: ShellChoice,
    },
    /// Plan curated MSYS2 packages without installing them
    Plan {
        #[arg(required = true, num_args = 1..)]
        packages: Vec<String>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum ShellChoice {
    Bash,
    Zsh,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Plan { packages } => {
            match Catalog::curated().plan(packages.iter().map(String::as_str)) {
                Ok(plan) => {
                    println!("Package plan (Windows x86-64 / UCRT64):");
                    for package in plan.packages {
                        let repository = match package.repository {
                            Repository::Ucrt64 => "ucrt64",
                            Repository::Msys => "msys",
                        };
                        println!("  {} -> {}/{}", package.name, repository, package.target);
                    }
                    println!(
                        "No packages installed. Pacman will resolve transitive dependencies during installation."
                    );
                }
                Err(error) => {
                    eprintln!("error: {error}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Doctor {
            runtime_root,
            shell,
        } => {
            let result = std::env::current_dir().and_then(|cwd| {
                Runtime::new(runtime_root).plan_shell(
                    match shell {
                        ShellChoice::Bash => Shell::Bash,
                        ShellChoice::Zsh => Shell::Zsh,
                    },
                    cwd,
                )
            });
            match result {
                Ok(plan) => {
                    println!("Runtime file checks passed for the selected shell / UCRT64.");
                    println!("Shell: {}", plan.executable.display());
                    println!(
                        "Windows binaries not executed; this is a file-presence and launch-plan check."
                    );
                }
                Err(error) => {
                    eprintln!("error: {error}");
                    std::process::exit(1);
                }
            }
        }
    }
}
