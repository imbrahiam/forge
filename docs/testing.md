# TDD seams

Approval status: confirmed by the user’s “Continue” after the proposed seams.
Implemented: 4 CLI tests, 8 runtime tests and 5 package tests.

| Seam | Catches | Does not establish |
| --- | --- | --- |
| CLI executable | argument handling, stdout/stderr, exit codes | GUI/rendering correctness |
| Runtime launch interface | shell selection, paths, launch environment | complete POSIX compatibility |
| Package-plan interface | catalog selection, dependencies, unsupported requests | downloads or installation correctness |

Initial vertical slices: CLI help; runtime Bash launch description from a fixture
root; a curated install plan. Each test must first fail for the expected missing
behavior. Follow with Zsh, missing executables, spaces in paths, unknown packages,
duplicate selections and dependency ordering only as those behaviors are added.

Windows integration tests must eventually launch actual MSYS2 processes over
ConPTY, including resize, control input and shutdown. Host-independent launch
planning is useful but cannot establish that the Windows launch works.
