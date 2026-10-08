# TDD skill provenance

Installed using the Codex skill-installer helper from the official repository:
https://github.com/mattpocock/skills/tree/main/skills/engineering/tdd

Local installation: `/Users/brahiam/.codex/skills/tdd`.
Upstream main observed on 2026-10-08: `b0618bc436ad893b3c5e84e55fba86586d34a404`.
The helper downloaded main; this observed revision is context, not a cryptographic
attestation of the downloaded archive. Skill content is not vendored into Forge.

Read SKILL.md, tests.md and mocking.md. Also consulted the upstream codebase-design
skill as required by TDD when selecting interfaces. Apply one public-interface
behavior per red/green cycle, without internal mocks or implementation snapshots.
The installed TDD version requires explicit confirmation of testing seams.
