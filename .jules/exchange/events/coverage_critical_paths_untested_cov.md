---
label: "tests"
created_at: "2024-03-12"
author_role: "cov"
confidence: "high"
---

## Problem

Several critical domains and commands are completely uncovered. This introduces risk of silent breakages for core functionality like configuring environments or running backup operations.

## Goal

Write test coverage focused on the business logic inside the command modules. Ensure domain types are adequately covered.

## Context

Using line coverage from `cargo-tarpaulin`. Commands like `app/commands/config/mod.rs` (0/40), `app/commands/create/mod.rs` (0/36), `app/commands/deploy_configs.rs` (0/34) and domain elements like `domain/repo_target.rs` (0/7) are critical logic segments and entirely uncovered.

## Evidence

- path: "src/app/commands/config/mod.rs"
  loc: "0/40"
  note: "0% test coverage for configuration settings command"
- path: "src/app/commands/create/mod.rs"
  loc: "0/36"
  note: "0% coverage for creating environments"
- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "0/48"
  note: "domain model defining critical references has 0% coverage"

## Change Scope

- `src/app/commands/config/mod.rs`
- `src/app/commands/create/mod.rs`
- `crates/mev-internal/src/domain/repository_ref.rs`