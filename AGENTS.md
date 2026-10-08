# Forge engineering instructions

Read README.md, GLOSSARY.md and the relevant ADR before modifying a module.
For Windows continuation, read docs/windows-handoff.md. The existing CLI, runtime
launch and package-plan seams are already confirmed; preserve that agreement.
Use Matt Pocock's installed TDD skill (`~/.codex/skills/tdd/SKILL.md`). Confirm
public test seams with the user before writing tests. Implement one failing
behavior test, observe the expected failure, implement the minimum behavior,
and rerun before moving to the next slice. Never manufacture tests that simply
mirror implementation or silently replace unavailable Windows tests with mocks.

Keep CLI composition separate from runtime and package implementations. Do not
add a shared `core`, `utils`, speculative traits, or one crate per type. Keep
platform paths as PathBuf/OsString and process arguments separate; never build
shell command strings from user inputs. Keep imports acyclic. Add dependencies
only for the slice being implemented and commit Cargo.lock changes with them.

Run format, Clippy with warnings denied, and relevant tests. State which host
was actually tested. Windows behavior requires Windows execution, not just
successful compilation on macOS. Do not claim package installation, GUI or Unix
binary compatibility until verified. Use local docs for planning; do not create
remote issues, push, or publish unless requested.
