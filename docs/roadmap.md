# Implementation roadmap

1. **Implemented on macOS:** CLI, runtime launch planning and curated package planning through confirmed
   public seams. Acceptance: actionable output and errors; no package side effects.
2. Real Windows PTY session. Acceptance: Bash/Zsh launch, quoted paths, cwd,
   I/O, control input, resize, child exit, cleanup; Windows integration tests.
3. Terminal state and minimal GPU window. Acceptance: interactive editing,
   Unicode, scrolling and resize with bounded memory; measurable latency.
4. Managed runtime bootstrap. Acceptance: per-user installation, integrity,
   failure recovery, doctor command and license inventory.
5. Real package operations. Acceptance: search/install/remove/update, dependency
   failures, transaction reporting and runtime update coordination.
6. Product polish. Acceptance: tabs/splits, IME, clipboard, DPI, accessibility,
   configuration, startup/performance benchmarks and signed Windows distribution.

Keep each milestone shippable in small vertical slices. A lockfile/project
manifest is deferred until exact package artifacts can actually be retained.
