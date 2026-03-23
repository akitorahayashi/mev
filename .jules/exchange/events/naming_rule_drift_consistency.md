---
label: "docs"
created_at: "2026-03-23"
author_role: "consistency"
confidence: "high"
---

## Problem

The documentation explicitly forbids ambiguous directory names such as `core/`, but a `core/` directory exists in the shell alias configurations.

## Goal

Ensure the directory naming rule in the architecture documentation is followed by renaming the `core/` directory, or clarify in the documentation if this rule applies only to Rust source code and not to Ansible configuration assets.

## Context

`docs/architecture.md` defines an Architecture Principle that "Ambiguous names such as `core/`, `utils/`, `helpers/` are forbidden". However, the directory `src/assets/ansible/roles/shell/config/common/alias/core/` exists.

## Evidence

- path: "docs/architecture.md"
  loc: "line 67"
  note: "Documentation states 'Ambiguous names such as `core/`, `utils/`, `helpers/` are forbidden'."
- path: "src/assets/ansible/roles/shell/config/common/alias/core"
  loc: "Directory exists"
  note: "A directory named `core` exists in the codebase, violating the documented principle."

## Change Scope

- `docs/architecture.md`
- `src/assets/ansible/roles/shell/config/common/alias/core/`
