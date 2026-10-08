# Domain vocabulary

- **Terminal**: screen state, escape sequence interpretation and input handling.
- **Renderer**: turns terminal state into visible pixels.
- **PTY**: bidirectional terminal session connected to a child process.
- **Shell**: Bash or Zsh process interpreting commands.
- **Runtime**: managed MSYS2 distribution and its POSIX compatibility library.
- **Native package**: Windows executable, potentially linked to Windows runtime libraries.
- **POSIX package**: Windows executable using the MSYS2 compatibility runtime.
- **Catalog**: explicit mapping of Forge names to supported package identities.
- **Install plan**: deterministic, validated package selections before mutation.
- **Seam**: location of a module's public interface used by callers and tests.
