---
label: "refacts"
implementation_ready: false
---

## Goal

Decouple I/O concerns from the application and domain layers by replacing `std::path` references with primitive string types and routing filesystem mutations through the abstract `FsPort`.

## Problem

The `app/commands` layer directly uses `std::fs` and `std::path` to mutate files, bypassing the `FsPort`. Simultaneously, domain ports (`FsPort`, `IdentityStore`) couple themselves to host filesystem paths by using `std::path::Path` and `PathBuf` instead of abstract strings. This tightly couples the logic to host environments and breaks testability.

## Evidence

- source_event: "app_fs_violation_structural_arch.md"
  path: "src/app/commands/config/mod.rs"
  loc: "Lines 42-73"
  note: "Directly calls standard filesystem routines like `remove_dir_all`, `create_dir_all`, and `copy`."
- source_event: "app_fs_violation_structural_arch.md"
  path: "src/app/commands/deploy_configs.rs"
  loc: "Lines 41-76"
  note: "Directly calls standard filesystem routines."
- source_event: "domain_path_violation_structural_arch.md"
  path: "src/domain/ports/fs.rs"
  loc: "Lines 3, 10-20"
  note: "Uses `Path` and `PathBuf` in all port method signatures instead of primitive types."
- source_event: "domain_path_violation_structural_arch.md"
  path: "src/domain/ports/identity_store.rs"
  loc: "Lines 3, 21"
  note: "Uses `PathBuf` as the return type for `identity_path`."

## Change Scope

- `src/app/commands/config/mod.rs`
- `src/app/commands/deploy_configs.rs`
- `src/domain/ports/fs.rs`
- `src/domain/ports/identity_store.rs`
- `src/adapters/fs/std_fs.rs`
- `src/adapters/identity_store/local_json.rs`
- `src/app/commands/*`

## Constraints

- All filesystem mutations must use the `FsPort` dependency rather than `std::fs`.
- `FsPort` and `IdentityStore` interfaces must only accept and return primitive string types.

## Acceptance Criteria

- No direct usages of `std::fs` for file manipulation exist in the `app/commands/` module.
- Domain ports use `&str` or `String` instead of `std::path` types.
- The implementations in `src/adapters/` successfully translate strings to path operations internally.
