# First milestone specification

Status: implemented and verified on macOS. Real Windows execution is milestone 2.

## CLI

`forge --help` prints available commands and succeeds. Invalid commands fail with
an actionable message on stderr. `forge doctor --runtime-root <path> --shell bash|zsh` diagnoses
an explicitly provided MSYS2 root without changing the machine. `forge plan
<packages...>` prints a validated package plan and performs no installation.
Do not advertise `install` or a working graphical terminal before they exist.

## Runtime interface

A caller supplies a runtime root, a shell choice (Bash or Zsh) and a working
directory. Runtime returns a launch description with executable, distinct native
arguments, cwd and environment overrides, or a specific diagnostic. Validate
required executable presence. Do not modify the caller's environment globally.
A launch description is deliberately not evidence of successful execution.

Use real filesystem fixtures for planning tests. Add real Windows execution and
ConPTY integration only in the next milestone. Launcher environment details
must follow MSYS2's documented rules and be verified experimentally.

## Package interface

A caller supplies Forge package names. The module validates against an explicit
catalog, reports unknown names and produces stable, deduplicated backend targets.
Represent MSYS versus UCRT64 provenance explicitly. Pacman will resolve transitive
repository dependencies; Forge should not recreate or guess that graph.
A curated catalog's own prerequisite declarations may be planned here, but only
if an actual initial package requires them. Catalog mappings require validation
against upstream repositories before claiming packages are installable.

## Implementation order

1. CLI help through the built executable: observe red, implement, observe green.
2. Bash launch planning through runtime's public interface: red then green.
3. One catalog name through packages' public interface: red then green.
4. CLI doctor and plan connect the actual modules; tests use their real behavior.
5. Add errors and Zsh as separate red/green slices.

No blanket up-front test suite or unimplemented function stubs are needed.
