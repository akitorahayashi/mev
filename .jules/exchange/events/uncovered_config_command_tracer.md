---
label: "tests"
created_at: "2025-05-24"
author_role: "tracer"
confidence: "high"
---

## Problem

The `config` command CLI contract tests are entirely missing, and the command logic (`src/app/commands/config/mod.rs`) only has 16/33 lines covered.

## Goal

Add CLI tests for the `config` command to verify that `mev config deploy` properly triggers the domain logic.

## Context

The `config` command orchestration interacts heavily with `AnsiblePort` and `FsPort` to orchestrate file system modifications (moving role config directories into `.config`). A failure in this path would silently fail to configure essential development tools on new environments. The unit test `test_deploy_config_success` checks a happy path, but there are multiple paths in `deploy_internal` (e.g. invalid role, no roles, existing target without overwrite) and zero CLI contract tests in `tests/cli/config.rs`.

## Evidence

- path: "src/app/commands/config/mod.rs"
  loc: "16/33 lines tested"
  note: "Only the primary success path is unit-tested. Error paths like invalid role are uncovered."
- path: "tests/cli/config.rs"
  loc: "whole file"
  note: "File exists but contains no test code."

## Change Scope

- `src/app/commands/config/mod.rs`
- `tests/cli/config.rs`
