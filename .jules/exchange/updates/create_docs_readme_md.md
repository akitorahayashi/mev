---
created_at: "2026-03-11"
---

## Claim

The central documentation index `docs/README.md` is missing. The Canonical Documentation Structure constraint dictates that it must be an entry point that owns the table of contents and routes to all specific areas within the `docs/` directory.

## Points

- Create a `docs/README.md` file.
- Implement a clear Table of Contents (ToC) that routes to `docs/usage.md` and `docs/architecture.md`.
- Remove any redundant ToC content from the main `README.md` or `AGENTS.md`, and point them to `docs/README.md`.

## Evidence

- `docs/README.md` does not exist.
- Main `README.md` currently lists explicit links to `docs/usage.md` instead of routing through a central index.
