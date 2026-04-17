---
label: "refacts"
author_role: "factorer"
---

# Boundary Issue: Wrapper Sprawl / Discrepancy between name and contract
`src/domain/backup_component.rs` contains `resolve_backup_component` which looks up a string alias and returns `Option<BackupComponent>`, and `validate_backup_component` which wraps it to return a `Result<BackupComponent, AppError>`. `validate_backup_component` is exactly parsing a string into an enum variant. It would be more idiomatic as `impl std::str::FromStr for BackupComponent`. The separation between resolve and validate represents wrapper sprawl and an unnecessary indirection that could be simplified by just implementing standard traits (`FromStr` and potentially `TryFrom<&str>`).

# Evidence
- Location: `src/domain/backup_component.rs`
- LOC: `68-88`
- Usage: Used in `src/app/commands/backup/mod.rs` (line 16) where user input is parsed.

# Change Scope
- `src/domain/backup_component.rs`
- `src/app/commands/backup/mod.rs`
