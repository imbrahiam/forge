# Forge

Windows-first Unix developer environment: a fast terminal, Bash/Zsh, familiar
commands, and coherent package management without WSL. Forge is a working name.

**Status: first CLI milestone implemented.** Forge can plan curated packages and
check the files needed for a Bash/Zsh launch from an existing MSYS2 root. It does
not yet execute shells, install packages, bootstrap MSYS2 or render a terminal.

## Try it

```sh
cargo run -p forge-cli -- --help
cargo run -p forge-cli -- plan git node python ripgrep zsh
cargo run -p forge-cli -- doctor --runtime-root /path/to/msys64 --shell zsh
```

On Windows, use your actual runtime path, for example:

```powershell
cargo run -p forge-cli -- doctor --runtime-root 'C:\msys64' --shell bash
```

`doctor` checks file presence only. It does not validate executable contents,
all transitive DLL dependencies, package database health, or actual Windows
execution. Launch planning preserves native arguments and sets MSYSTEM=UCRT64;
MSYS2's login profile initializes its tool paths. The Windows PATH is inherited
to allow calling installed Windows tools, which may introduce tool conflicts.

The curated catalog supports **bash, git, node, python, ripgrep, zsh**. Plans
preserve first-request order, deduplicate targets and reject unsupported names.
They do not resolve transitive dependencies, pin versions, or perform mutations.
See [catalog provenance](docs/catalog.md).

## Development

Install Rust stable, then run:

```sh
cargo build --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Tests exercise the CLI executable, runtime launch planning and package planning.
The first milestone has 17 behavior tests, implemented in individual red/green
cycles. Runtime fixtures contain placeholder files, not executable Windows binaries. GitHub Actions
is configured for Windows, Linux and macOS; remote CI has not been run yet.

## Layout

- `apps/forge-cli`: command parsing, user-facing output, process exit codes.
- `crates/forge-runtime`: managed environment and shell launch implementation.
- `crates/forge-packages`: curated package catalog and installation planning.
- `docs/architecture.md`: dependency rules, runtime and terminal design.
- `docs/adr`: recorded decisions with upstream references and tradeoffs.
- `docs/roadmap.md`: acceptance criteria in implementation order.

The desktop app and terminal engine will be added when a real PTY-to-screen slice
requires them. Empty renderer, networking and storage abstractions are avoided.

See [architecture](docs/architecture.md) and [contributing](CONTRIBUTING.md).
