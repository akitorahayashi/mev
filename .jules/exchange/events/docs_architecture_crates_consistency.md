---
label: "docs"
created_at: "2024-03-14"
author_role: "consistency"
confidence: "high"
---

## Problem

The architectural documentation (`docs/architecture.md`) claims that `crates/mev-internal/` contains internal command implementations for `(shell, vcs)`. However, there is no `shell` implementation inside `mev-internal`, only `git` and `gh` (which belong to VCS).

## Goal

Correct the `docs/architecture.md` file to reflect the actual implementations available inside `crates/mev-internal/`.

## Context

The architecture documentation map is important for discovering internal dependencies. Specifying non-existent modules leads to confusion when a developer searches for a `shell` internal command implementation that doesn't exist.

## Evidence

- path: "docs/architecture.md"
  loc: "35"
  note: "Documents `└── mev-internal/          # Internal command implementations (shell, vcs)`"
- path: "crates/mev-internal/src/app/cli/"
  loc: "N/A"
  note: "Directory contains `gh.rs` and `git.rs`. There is no `shell` implementation."

## Change Scope

- `docs/architecture.md`
