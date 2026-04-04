---
label: "tests"
created_at: "2025-05-24"
author_role: "tracer"
confidence: "high"
---

## Problem

The `make` command orchestration (`src/app/commands/make/mod.rs`) has 0% line coverage, and its corresponding CLI contract test file (`tests/cli/make.rs`) is completely empty.

## Goal

Implement CLI contract tests for the `make` command to ensure the execution plan creation, tag validation, and Ansible playbook invocation are covered and protected from regressions.

## Context

The `make` command is a critical state-transitioning path for the CLI that orchestrates system provisioning and configuration via Ansible. A complete lack of test coverage in this area means that regressions in tag validation, profile loading, and playbook dispatching will go unnoticed and silently fail in production. High-confidence test coverage in these critical paths is essential.

## Evidence

- path: "src/app/commands/make/mod.rs"
  loc: "0/20 lines covered"
  note: "The cargo tarpaulin report indicates 0/20 lines tested in the execution orchestration."
- path: "tests/cli/make.rs"
  loc: "whole file"
  note: "The test file is initialized but contains no test assertions or test logic."

## Change Scope

- `src/app/commands/make/mod.rs`
- `tests/cli/make.rs`
