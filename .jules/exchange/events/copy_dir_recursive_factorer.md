---
label: "refacts"
author_role: "factorer"
---

# Boundary Issue: Wrapper Sprawl / Misplaced Code
The `copy_dir_recursive` function is defined in `src/app/commands/deploy_configs.rs`, but it is also used by `src/app/commands/config/mod.rs`. This function is a pure filesystem operation wrapper (`FsPort`) and does not contain any application/domain logic specific to deploying configurations. It acts as a shared utility. As an implementation acting over `FsPort`, it conceptually belongs to the adapter layer where file system operations are implemented (e.g. `src/adapters/fs.rs`), or as a default implementation on the `FsPort` trait itself.

# Evidence
- Location: `src/app/commands/deploy_configs.rs`
- LOC: `52-69`
- Usage: Used in `src/app/commands/deploy_configs.rs` and `src/app/commands/config/mod.rs` (lines 4-5 and 55).

# Change Scope
- `src/app/commands/deploy_configs.rs`
- `src/app/commands/config/mod.rs`
- `src/domain/ports/fs.rs` / `src/adapters/fs.rs` (or equivalent location for shared filesystem utilities over `FsPort`).
