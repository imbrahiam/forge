# ADR 0001: Rust workspace with feature-oriented crates

Status: accepted for the initial scaffold.

Rust supports Windows process integration and existing terminal ecosystem
libraries. Use a workspace with CLI, runtime and packages crates. A crate earns
its place through distinct responsibilities and dependencies, not organizational
symmetry. Keep rendering and terminal state separate when implemented.

Evidence: [Alacritty workspace](https://github.com/alacritty/alacritty/blob/master/Cargo.toml),
[WezTerm PTY](https://github.com/wezterm/wezterm/tree/main/pty).

Tradeoffs: Rust requires more explicit ownership and platform handling than a web
stack. Native UI and renderer technology remain open; Rust alone does not supply
native Windows UI or guarantee terminal performance.
