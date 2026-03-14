---
label: "refacts"
created_at: "2026-03-14"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The codebase uses vague and overly generic names such as `helpers` and `common` which hide responsibility and create ambiguous boundaries. The role and rule constraints explicitly prohibit naming structures `base`, `common`, `core`, `utils`, or `helpers`.

## Goal

Remove anti-pattern naming (`common`, `helpers`) from the codebase, replacing them with precise terms that reflect their domain responsibility.

## Context

Using vague module and profile names like `common` or describing commands as `helpers` makes it difficult to understand the actual function of the code without inspecting it. `helpers` should be defined by the domain action they perform. The profile `common` should be referred to by a canonical term representing the global or default environment setting.

## Evidence

- path: "src/domain/profile.rs"
  loc: "Common,"
  note: "Defines a profile variant named `Common`, which maps to the string 'common'."
- path: "src/domain/backup_target.rs"
  loc: "common"
  note: "Returns a subpath 'common'."
- path: "src/app/commands/backup/mod.rs"
  loc: "// Shared helpers"
  note: "Uses 'helpers' to group functions in backup execution."
- path: "src/app/cli/mod.rs"
  loc: "/// Git helpers."
  note: "CLI help text uses the vague term 'helpers'."
- path: "crates/mev-internal/src/app/cli/mod.rs"
  loc: "/// GitHub CLI helpers."
  note: "CLI help text uses the vague term 'helpers'."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/commands/backup/mod.rs`
- `src/app/cli/mod.rs`
- `crates/mev-internal/src/app/cli/mod.rs`
