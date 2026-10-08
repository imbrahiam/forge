# First milestone implementation evidence

Host: macOS. Tests use actual CLI child processes and actual temporary directories.
No internal modules are mocked. The runtime fixture files are placeholders.

Each slice was tested before implementation and after it:

| Command filter | Observed red | Result after implementation |
| --- | --- | --- |
| CLI help_describes | placeholder exits 1 | help succeeds, lists doctor and plan |
| Runtime bash_launch | missing public interface | native paths/arguments and UCRT64 overrides |
| Packages ripgrep_maps | missing catalog interface | correct UCRT64 target |
| Runtime missing_bash | erroneously returns a plan | actionable missing-executable error |
| Packages unknown_packages | silently ignores unknown name | whole request rejected |
| CLI plan_prints | rejects package arguments | prints curated target without installing |
| Packages developer_tools | missing MSYS provenance | six supported identities |
| Packages repeated_packages | duplicate targets | stable deduplication |
| Runtime a_shell_without | accepts missing DLL | missing-runtime diagnostic |
| Runtime zsh_is_selected | missing shell choice | Zsh launch plan |
| Runtime login_launch | accepts missing profile | login-profile diagnostic |
| Runtime a_non_directory | accepts file as cwd | invalid-directory diagnostic |
| CLI doctor_reports | rejects runtime-root argument | file-check diagnostic |
| Runtime zsh_requires | accepts absent zprofile | Zsh bridge diagnostic |
| CLI doctor_can | rejects shell argument | Zsh selection through CLI |
| Runtime relative_runtime | relative executable | absolute executable/cwd |
| Packages empty_requests | accepts empty plan | explicit empty-request error |

Initial help-test compilation was corrected before observing its expected
behavior failure. Subsequent red results include missing-interface compile
failures and behavior failures as recorded above.

Review centralized catalog identities and derived supported-name diagnostics from
the same catalog to prevent drift. Package errors are structured for CLI/desktop
callers. No shell command strings or global environment mutation are used.

Validation: workspace tests (17), formatting and Clippy with warnings denied on
macOS. Windows/Linux CI is configured but has not been remotely run. Stable Rust
is used; the declared minimum Rust version is not separately verified yet.
