---
label: "refacts"
created_at: "2024-05-15"
author_role: "qa"
confidence: "high"
---

## Problem

Multiple pure logic parts and adapters bypass the `FsPort` abstraction to interact directly with the standard filesystem via `std::fs` and `std::path::PathBuf`. For example, `IdentityFileStore` contains direct calls to `std::fs::read_to_string`, `std::fs::create_dir_all`, and `std::fs::write`. Similar issues exist within `src/app/commands/config/mod.rs` and `src/app/commands/deploy_configs.rs`.

## Goal

Ensure that pure domain logic layers and infrastructure boundaries rely entirely on injected port abstractions rather than the direct usage of side-effect-producing components like `std::fs`. Adhere to the `FsPort` interface where filesystem concepts are concerned.

## Context

Bypassing filesystem port abstractions makes unit testing domain logic complex, slow, and non-deterministic because it relies on the real host filesystem state. This breaks the isolation-by-design testing principle and violates domain I/O decoupling rules since internal implementations shouldn't possess knowledge of hardcoded IO operations. Abstracting these I/O operations strictly through the boundary interfaces ensures side-effect-free test doubles can be correctly injected.

## Evidence

- path: "src/adapters/identity_store/local_json.rs"
  loc: "IdentityStore::load"
  note: "Directly uses `std::fs::read_to_string` to read from the identity config path."
- path: "src/adapters/identity_store/local_json.rs"
  loc: "IdentityStore::save"
  note: "Directly uses `std::fs::create_dir_all`, `std::fs::write`, and `std::fs::rename` to manipulate the JSON configuration state."
- path: "src/app/commands/config/mod.rs"
  loc: "execute"
  note: "Uses `std::fs::remove_dir_all` and `std::fs::rename` directly to configure the staging target configs instead of port abstractions."
- path: "src/app/commands/deploy_configs.rs"
  loc: "execute"
  note: "Uses `std::fs::remove_dir_all` to clean target deployments before recreating."

## Change Scope

- `src/adapters/identity_store/local_json.rs`
- `src/app/commands/config/mod.rs`
- `src/app/commands/deploy_configs.rs`