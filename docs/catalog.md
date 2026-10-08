# Curated catalog

Verified package identities against the official MSYS2 index on 2026-10-08.
These links establish repository identity, not a successful Forge installation.
Package versions are deliberately not hardcoded: MSYS2 repositories roll forward.

| Forge name | Repository | Upstream package |
| --- | --- | --- |
| bash | msys | [bash](https://packages.msys2.org/packages/bash) |
| git | msys | [git](https://packages.msys2.org/packages/git) |
| zsh | msys | [zsh](https://packages.msys2.org/packages/zsh) |
| node | ucrt64 | [mingw-w64-ucrt-x86_64-nodejs](https://packages.msys2.org/packages/mingw-w64-ucrt-x86_64-nodejs) |
| python | ucrt64 | [mingw-w64-ucrt-x86_64-python](https://packages.msys2.org/packages/mingw-w64-ucrt-x86_64-python) |
| ripgrep | ucrt64 | [mingw-w64-ucrt-x86_64-ripgrep](https://packages.msys2.org/packages/mingw-w64-ucrt-x86_64-ripgrep) |

Node's identity was also verified against its official [build recipe](https://github.com/msys2/MINGW-packages/blob/master/mingw-w64-nodejs/PKGBUILD).
The UCRT64 Python is the MSYS2/MinGW build: it is not the python.org Windows
distribution, and compatibility with that distribution's wheels must not be assumed.

Pacman will resolve dependencies from its actual repository database. Forge's
current plan only selects explicit targets; it is not an install transaction or
an environment lockfile. Online package-index checks are not in behavior tests.

## Launch initialization references

- [MSYS2 launcher](https://github.com/msys2/MSYS2-packages/blob/master/filesystem/msys2_shell.cmd)
- [Login profile](https://github.com/msys2/MSYS2-packages/blob/master/filesystem/profile)
- [Zsh profile bridge](https://github.com/msys2/MSYS2-packages/blob/master/zsh/zprofile)
- [Environment selection](https://www.msys2.org/docs/environments/)

The login shell, MSYSTEM=UCRT64, CHERE_INVOKING and MSYS2_PATH_TYPE settings follow
these references. Forge checks selected executable, msys-2.0.dll, /etc/profile,
and /etc/zprofile for Zsh. Actual process behavior remains to be tested on Windows.
