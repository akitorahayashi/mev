---
created_at: "2026-03-11"
---

## Claim

The repository lacks a canonical `docs/architecture.md` surface as mandated by the Canonical Documentation Structure constraint. The current architectural details, principles, and design rules are incorrectly located in `AGENTS.md`, and must be migrated here.

## Points

- Create `docs/architecture.md` to hold durable design aspects, boundaries, topology, and invariants.
- Migrate the following sections from `AGENTS.md` into this new file: "Architecture", "Package Structure", "Architecture Principles", and "Design Rules".
- Ensure that the migrated content does not contain runbooks.

## Evidence

- `docs/architecture.md` does not exist.
- The Canonical Documentation Structure constraint requires `docs/architecture.md` or `docs/architecture/` for durable design.
- `AGENTS.md` currently holds detailed architectural descriptions that belong here.
