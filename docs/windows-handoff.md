# Windows handoff

## Resume here

Build a Windows-first Unix developer environment with a fast native terminal,
Bash/Zsh, familiar utilities, and one coherent package interface, without WSL.
Forge is the working name. The user authorized implementation, Matt Pocock TDD,
and pushing this repository to `git@github.com:imbrahiam/forge.git`.

Start by reading AGENTS.md, GLOSSARY.md, docs/architecture.md and both ADRs.
Continue the existing implementation; do not recreate the scaffold.

## Current state

Baseline implementation commit: `9565744`.

- Rust workspace: forge-cli, forge-runtime, forge-packages.
- CLI: help/version, `plan <packages...>`, `doctor --runtime-root <path>
  --shell bash|zsh` (Bash is the default).
- Runtime: `Runtime::plan_shell` returns executable, native arguments, cwd and
  child environment overrides. It resolves relative paths and checks shell,
  MSYS DLL and login profile presence, plus the Zsh profile bridge when selected.
- Packages: curated identities for bash, git, node, python, ripgrep and zsh.
  Plans validate names, deduplicate and preserve first-request order.
- 17 integration-style behavior tests: 4 CLI, 8 runtime, 5 package.
- GitHub Actions configured on Windows/Linux/macOS. Check the actual run after
  this push; no remote CI result was available when the handoff was written.

Only macOS was exercised locally. Formatting, Clippy with warnings denied, all
17 tests and CLI smoke checks passed there. The declared Rust 1.85 minimum has
not been independently tested. Filesystem fixtures contain placeholder files,
not real Windows binaries. No PTY, actual shell execution, GUI, runtime bootstrap,
package installation or project lockfile exists yet.

## Windows setup and baseline

Use Windows x86-64 first, Rust stable with the MSVC toolchain, the corresponding
Visual Studio C++ build tools/Windows SDK, Git, and an existing initialized MSYS2
installation. Use a dedicated development runtime if possible. Forge does not
install any of these yet. Follow MSYS2's own setup/update instructions and install
its Zsh package in that environment if needed; runtime maintenance is external
to Forge for now. Do not run blanket system upgrades as an automated test step.

From PowerShell:

```powershell
git clone git@github.com:imbrahiam/forge.git
Set-Location forge
rustup show
cargo build --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo run -p forge-cli -- --help
cargo run -p forge-cli -- plan git node python ripgrep zsh
cargo run -p forge-cli -- doctor --runtime-root 'C:\msys64' --shell bash
cargo run -p forge-cli -- doctor --runtime-root 'C:\msys64' --shell zsh
```

Replace `C:\msys64` with the actual MSYS2 root. A successful doctor command only
establishes file presence and a plan. It does not establish executability,
architecture, DLL dependency health, profile correctness or a working shell.

## TDD skill and prior agreement

Matt Pocock's TDD skill was installed on the Mac under `~/.codex/skills/tdd`.
It is not included in Git and must be installed on the Windows Codex machine.
Use the official `mattpocock/skills` repository's `skills/engineering/tdd` skill;
read its current SKILL.md and referenced documents. Provenance is recorded in
`docs/skill-provenance.md`. Do not use the historical Mac absolute path on Windows.

The user confirmed these public seams by replying “Continue”:

1. CLI executable: arguments, output and exit codes.
2. Runtime launch interface: shell selection, paths and environment.
3. Package-plan interface: selection, unsupported names and plan behavior.

Preserve this agreement; do not re-ask for the same scope. Follow the skill's
confirmation requirement if introducing a genuinely new test seam, such as
terminal state/rendering. Implement one behavior test, observe its expected red,
implement the minimum behavior, observe green, then move to the next slice.
Keep tests at the public interface; use actual processes/files rather than
mocking internal modules. Record Windows evidence separately from macOS evidence.

## Next task: real Windows PTY session

Reuse WezTerm's `portable-pty` in forge-runtime; inspect the current upstream API
and license before adding it. It provides ConPTY on Windows. Keep platform details
in runtime and leave package logic and rendering out of the process lifecycle.
Do not add empty renderer crates or a custom VT parser for this milestone.

Implement these vertical slices in order:

1. Launch a real MSYS2 Bash process from the existing launch plan through a PTY,
   send a deterministic marker and exit command, observe marker/output and exit.
   Exercise the runtime's public launch/session interface on Windows. Design the
   narrow session interface before coding; no global environment mutation.
2. Repeat with Zsh and verify its login bridge initializes UCRT64. Verify
   MSYSTEM, PATH precedence, cwd and Windows executable interoperability from
   the real child, not just from LaunchPlan fields.
3. Verify a runtime path and project cwd containing spaces and Unicode. Ensure
   executable/cwd paths stay native PathBuf values and arguments stay separate.
4. Add resize, control input, EOF and child-exit handling. Make tests bounded by
   timeouts with child cleanup on failure; do not allow CI to hang on an open PTY.
5. Verify teardown kills/reaps the child, reader/writer ownership is explicit,
   and output reading cannot block the eventual UI thread.
6. Expose an honest CLI shell command only once interactive input forwarding,
   resize and restoration of the host terminal on errors/exit are implemented.
   A child connected to a PTY alone is not a working interactive CLI terminal.

Keep the current pure tests portable. Real MSYS2/ConPTY tests belong in an explicit
Windows integration target with documented runtime prerequisites. Ordinary tests
must not install system packages. Provision a Windows CI runtime deliberately;
missing required integration prerequisites should fail clearly in that job,
not silently count as successful coverage.

Upstream references are linked in docs/architecture.md and docs/catalog.md.
Record any deviation from direct shell launch after testing the supported MSYS2
launcher behavior. PATH inheritance is intentional for Windows-tool access but
can create conflicts; HOME currently follows inherited/MSYS2 defaults and is not
yet a managed per-user Forge home.

## After PTY execution

- Terminal state plus a minimal GPU window: evaluate Alacritty's terminal library
  independently of the native UI/renderer. Confirm this new seam before tests.
- Managed per-user MSYS2 bootstrap with integrity, recovery and license notices.
- Actual package operations using Pacman transactions/dependency solving.
- Tabs/splits, IME, DPI, clipboard, accessibility, performance and distribution.

Do not reimplement Pacman, assume Homebrew bottles run on Windows, or promise
unmodified Linux/macOS executable compatibility. MSYS2 is rolling-release;
reproducible version locks require retained exact artifacts and digests. UCRT64
Python is the MinGW build, so python.org Windows wheel compatibility is not
assumed. No redistribution license has been chosen for Forge yet.

## Suggested continuation prompt

“Read docs/windows-handoff.md and AGENTS.md. Verify the existing 17 tests on
Windows, install/read Matt Pocock's TDD skill if missing, then implement the
first real Bash PTY launch slice through the agreed runtime seam. Use an actual
MSYS2 runtime and bounded integration tests. Continue one red/green slice at a
time, document Windows evidence, and keep unsupported capabilities explicit.”
