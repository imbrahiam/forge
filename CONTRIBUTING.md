# Contributing

Development uses Rust stable and a workspace committed with Cargo.lock.
Follow AGENTS.md and the TDD skill. Proposed seams and approval status are in
`docs/testing.md`. Add integration tests through public interfaces, with one
red-to-green behavior per cycle. Use real temporary directories and real child
processes where practical; mock external network/time boundaries only.

For Windows launch tests, use an explicitly configured MSYS2 root; missing
runtime dependencies must produce actionable failures rather than silent skips.
Do not install packages globally as a test side effect.

Run the commands in README.md before submitting changes. Document new behavior,
platform limitations, and deviations from ADRs. Record red and green commands
in the implementation notes. No contribution license has been selected yet;
this scaffold is not currently licensed for redistribution.
