---
created_at: "2026-03-11"
---

## Claim

`AGENTS.md` is currently overloaded with detailed architectural components, package structure, and design rules, violating the Canonical Documentation Structure constraint which mandates that it be a concise repository description with basic routing. These detailed sections must be removed and relocated to the appropriate architectural documentation surfaces.

## Points

- Remove the "Architecture", "Package Structure", "Architecture Principles", and "Design Rules" sections from `AGENTS.md`.
- Keep the "Overview" and "CLI Commands" sections.
- Ensure `AGENTS.md` explicitly routes agents to `CONTRIBUTING.md` and `docs/` as their primary sources of truth for detailed work.

## Evidence

- `AGENTS.md` contains 40+ lines of detailed tables outlining layers, paths, and responsibilities, which belong in `docs/architecture.md`.
- `AGENTS.md` contains detailed package structure directory trees and internal dependency mapping.
- The `AGENTS.md Policy` in memory specifies that documentation for LLMs must be concise for token efficiency, focusing only on essential information.
