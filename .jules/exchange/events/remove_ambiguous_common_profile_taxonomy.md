---
label: "refacts"
created_at: "2024-03-19"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The term `common` is used extensively in the domain (e.g., `Profile::Common`, `src/domain/profile.rs`, config directories, CLI parameters). This violates the core design principle that prohibits ambiguous names such as `common`.

## Goal

Refactor the taxonomy to replace `common` with a precise, domain-aligned term (e.g., `baseline`, `shared`, or `default`). This includes updating the profile definitions, domain logic, command parameters, test cases, and configuration folder structures.

## Context

Using `common` is explicitly discouraged by the system architecture as an ambiguous classification that obscures a module or concept's precise responsibility. For `mev` profiles, a term like `baseline` defines the explicit role of configurations that apply universally before machine-specific overrides.

## Evidence

- path: "src/domain/profile.rs"
  loc: "line 12, 21, 56"
  note: "Defines the `Profile::Common` variant and its canonical mapping to `common`."
- path: "src/app/cli/make.rs"
  loc: "line 14, 15"
  note: "Defines the profile argument with the default value `common`."

## Change Scope

- `src/domain/profile.rs`
- `src/domain/backup_target.rs`
- `src/app/cli/make.rs`
- Config directory layouts under `src/assets/ansible/roles/**/config/common`
