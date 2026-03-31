---
label: "refacts"
---

## Goal

Remove silent `unwrap_or_else` during JSON serialization in `src/app/commands/backup/system.rs` and properly surface serialization failures.

## Current State

Silent fallback `unwrap_or_else` is used during JSON serialization of system settings backup in `src/app/commands/backup/system.rs`. This masks potential serialization issues and violates explicit error handling principles.
- `src/app/commands/backup/system.rs`: Lines 117 and 177 use `serde_json::to_string(&value).unwrap_or_else(|_| value.into_owned())`.

## Plan

1. Change the signature of `format_string` in `src/app/commands/backup/system.rs` to `fn format_string(raw_value: &str, key: &str, default: &serde_yaml::Value) -> Result<String, AppError>`.
2. Replace `serde_json::to_string(&value).unwrap_or_else(|_| value.into_owned())` in `format_string` with `serde_json::to_string(&value).map_err(|e| AppError::Backup(format!("failed to serialize string value: {e}")))`.
3. Change the signature of `format_value` in `src/app/commands/backup/system.rs` to `fn format_value(def: &SettingDefinition, raw_value: &str) -> Result<String, AppError>`.
4. Update the `match` arms in `format_value` to return `Ok(...)` for `bool`, `int`, and `float` types. Update the `string` arm to propagate the error from `format_string(raw_value, &def.key, &def.default)?`. Update the `_` arm to map the error from `serde_json::to_string(&value)` to an `AppError::Backup(format!("failed to serialize value: {e}"))`.
5. In `src/app/commands/backup/system.rs` update `execute` function to handle the `Result` from `format_value`: `let formatted = format_value(def, &raw_value)?;`
6. Run tests using `cargo test` to verify no regressions.

## Acceptance Criteria

- No silent fallbacks are used for JSON serialization in `system.rs`.
- Serialization failures are surfaced as an explicit `AppError::Backup`.

## Risks

- Failing to properly bubble up errors from `format_value` could lead to unexpected behavior during backup execution.
