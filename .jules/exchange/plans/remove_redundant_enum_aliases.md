---
label: "refacts"
---

## Goal

Consolidate the string-to-variant mapping logic for domain enums to use a single source of truth by dynamically checking variant canonical names and explicitly defined aliases, removing hardcoded alias arrays.

## Current State

Domain enums duplicate the canonical string names and aliases in static alias arrays, which act as a redundant source of truth for string-to-variant mapping.
- `src/domain/profile.rs`: `PROFILE_ALIASES` hardcodes variants mapped to strings, duplicating the canonical names from `as_str()` and aliases from `aliases()`. `resolve_profile` iterates over `PROFILE_ALIASES`.
- `src/domain/identity.rs`: `SWITCH_IDENTITY_ALIASES` hardcodes variants and strings redundantly. `SwitchIdentity` lacks an `aliases()` method and `all()` method. `resolve_switch_identity` converts `input` to lowercase and iterates over `SWITCH_IDENTITY_ALIASES`.
- `src/domain/backup_target.rs`: `BACKUP_TARGET_ALIASES` hardcodes variants and strings redundantly, duplicating names from `name()`. `BackupTarget` lacks an `aliases()` method. `resolve_backup_target` uses `BACKUP_TARGET_ALIASES`.

## Plan

1. In `src/domain/profile.rs`:
   - Delete the `PROFILE_ALIASES` constant.
   - Update `resolve_profile` to iterate over `all_profiles()`, checking if `input == p.as_str()` or `p.aliases().contains(&input)`.
2. In `src/domain/identity.rs`:
   - Delete the `SWITCH_IDENTITY_ALIASES` constant.
   - Add `pub fn all() -> &'static [Self]` to `SwitchIdentity` returning `&[Self::Personal, Self::Work]`.
   - Add `pub fn aliases(&self) -> &'static [&'static str]` to `SwitchIdentity` returning `&["p"]` for `Personal` and `&["w"]` for `Work`.
   - Update `resolve_switch_identity` to convert `input` to lowercase and iterate over `SwitchIdentity::all()`, checking if the lowercased input matches `i.as_str()` or `i.aliases().contains(&lower.as_str())`.
3. In `src/domain/backup_target.rs`:
   - Delete the `BACKUP_TARGET_ALIASES` constant.
   - Add `pub fn aliases(self) -> &'static [&'static str]` to `BackupTarget` returning `&["vscode-extensions"]` for `Vscode` and `&[]` for `System`.
   - Update `resolve_backup_target` to iterate over `BackupTarget::all()`, checking if `input == t.name()` or `t.aliases().contains(&input)`.
4. Run `cargo test` to ensure changes are correct and have not introduced regressions, ensuring externally observable behaviour and mapping boundaries remain preserved.

## Acceptance Criteria

- Hardcoded static alias arrays (`PROFILE_ALIASES`, `SWITCH_IDENTITY_ALIASES`, `BACKUP_TARGET_ALIASES`) are removed.
- Parsing logic correctly interprets canonical names and explicitly defined aliases dynamically using a single source of truth.

## Risks

- Case-sensitivity or string comparison logic drift might break resolution if not handled exactly as the previous mappings (e.g., `resolve_switch_identity` uses `to_lowercase()`).
