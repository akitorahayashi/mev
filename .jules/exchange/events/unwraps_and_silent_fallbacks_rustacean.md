---
label: "refacts"
created_at: "2026-03-11"
author_role: "rustacean"
confidence: "medium"
---

## Problem

Use of unwrap() and expect() scattered across tests and core paths. Specifically, `.unwrap_or()` with fallback values or generic fallback `-1` strings, and `.expect()` in domain models, masking failures or bypassing error boundaries. While some of these are in tests where panics are acceptable, relying on them as a general pattern undermines the error handling principles, especially outside test code.

## Goal

Ensure all `unwrap()`/`expect()` usage outside test modules are replaced with explicit error handling and propagation using robust domain error types.

## Context

According to the analysis points for errors: "Using `unwrap()` / `expect()` on production paths where failure is plausible" is an anti-pattern. We must also verify "Silent fallback behaviors that mask failures or drift configuration", which `unwrap_or(-1)` strongly suggests. First principle: "Errors are part of the contract: keep semantic meaning; attach context where the system meets the world."

## Evidence

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "lines 100, 107, 114, 121"
  note: "Tests heavily rely on `.expect()`. While acceptable in tests, it should be audited to ensure it doesn't leak into production paths."

- path: "crates/mev-internal/src/domain/repo_target.rs"
  loc: "lines 28, 35"
  note: "Tests rely on `.expect()`."

- path: "crates/mev-internal/src/domain/label_catalog.rs"
  loc: "line 24"
  note: "Test uses `.expect()`."

- path: "src/domain/profile.rs"
  loc: "lines 123, 124"
  note: "Test uses `.unwrap()`."

- path: "src/adapters/version_source/install_script.rs"
  loc: "line 54"
  note: "`status.code().unwrap_or(-1)` acts as a silent fallback that masks the actual signal or lack of an exit code."

- path: "src/adapters/ansible/executor.rs"
  loc: "line 152"
  note: "`code.unwrap_or(-1)` acts as a silent fallback."

## Change Scope

- `src/adapters/version_source/install_script.rs`
- `src/adapters/ansible/executor.rs`
- `crates/mev-internal/src/adapters/process.rs`
