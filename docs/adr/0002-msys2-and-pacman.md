# ADR 0002: MSYS2 runtime and Pacman package backend

Status: accepted direction; bootstrap and redistribution not implemented.

Use MSYS2 for Windows builds of Bash/Zsh and POSIX tools, plus UCRT64 native
packages. Forge supplies coherent UX and curated mappings. Pacman retains actual
dependency solving and transactions. Do not reimplement POSIX or package archives.

Evidence: [MSYS2 environments](https://www.msys2.org/docs/environments/),
[package management](https://www.msys2.org/docs/package-management/).

Tradeoffs: compatibility DLLs are required for MSYS tools, rolling packages do not
provide arbitrary historical versions, and licenses must be audited before
shipping. WSL and unmodified macOS/Linux binary execution are outside the scope.
