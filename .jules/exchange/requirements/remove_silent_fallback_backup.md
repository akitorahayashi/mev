---
label: "refacts"
implementation_ready: false
---

## Goal

Remove silent `unwrap_or_else` during JSON serialization in `src/app/commands/backup/system.rs` and properly surface serialization failures.

## Problem

Silent fallback `unwrap_or_else` is used during JSON serialization of system settings backup in `src/app/commands/backup/system.rs`.

## Context

The rule "Silent fallbacks are prohibited; any fallback is explicit, opt-in, and surfaced as a failure or a clearly logged, reviewed decision" is broken here. If `serde_json::to_string` fails (which can happen, e.g. with invalid strings, though `Cow<str>` is generally safe, falling back silently to raw string value masks potential issues and violates explicit error handling).

## Evidence

- path: "src/app/commands/backup/system.rs"
  loc: "line 117"
  note: "`serde_json::to_string(&value).unwrap_or_else(|_| value.into_owned())`"
- path: "src/app/commands/backup/system.rs"
  loc: "line 177"
  note: "`serde_json::to_string(&value).unwrap_or_else(|_| value.into_owned())`"

## Change Scope

- `src/app/commands/backup/system.rs`

## Constraints

- Serialization failures must be surfaced as an explicit error.

## Acceptance Criteria

- No silent fallbacks are used for JSON serialization in `system.rs`.
