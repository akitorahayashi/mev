---
label: "refacts"
created_at: "2023-10-27"
author_role: "consistency"
confidence: "high"
---

## Problem

`src/domain/profile.rs`, `src/domain/tag.rs`, and `src/domain/vcs_identity.rs` contain CLI-specific string input parsing logic and aliases (`aliases()`, `SWITCH_IDENTITY_ALIASES`, `tag_groups()`, `resolve_tags()`). The architecture rules mandate: "Domain Input Parsing: Core domain models must not contain CLI-specific string input parsing logic or aliases. Such validation and transport/UI mapping must be exclusively handled by the adapter or application CLI layer."

## Goal

Document the architectural boundary violation so that the CLI string parsing and alias resolution can be moved out of the domain layer and into the CLI layer (`src/app/cli/`).

## Context

According to memory: "Architecture Rule (Domain Input Parsing): Core domain models must not contain CLI-specific string input parsing logic or aliases. Such validation and transport/UI mapping must be exclusively handled by the adapter or application CLI layer." Currently, multiple files in the `src/domain/` directory violate this rule by owning string parsing and CLI shorthand aliases (`mbk`, `mmn`, `cmn`, `p`, `w`, and shorthand tags like `rust` -> `rust-platform, rust-tools`). Note: the rule specifies moving this logic out of the domain layer, which is an architectural boundary issue.

## Evidence

- path: "src/domain/profile.rs"
  loc: "aliases"
  note: "Contains string aliases `mbk`, `mmn`, `cmn` which are CLI/UI mapping concerns."

- path: "src/domain/tag.rs"
  loc: "tag_groups"
  note: "Contains string mapping rules for expanding CLI tag groups."

- path: "src/domain/vcs_identity.rs"
  loc: "SWITCH_IDENTITY_ALIASES"
  note: "Contains string aliases `p`, `w` which are CLI/UI mapping concerns."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/tag.rs`
- `src/domain/vcs_identity.rs`
- `src/app/cli/make.rs`
- `src/app/cli/create.rs`
- `src/app/cli/switch.rs`
- `src/app/commands/list/mod.rs`
