---
label: "refacts"
created_at: "2024-05-15"
author_role: "rustacean"
confidence: "high"
---

## Problem

The codebase uses `.unwrap()` or `.expect()` heavily in non-test paths (e.g., tests logic is mixed with production code, or commands unwrap directly without surfacing an error correctly to the user), such as parsing environment variables, setting permissions, or running commands. These usages present a risk of unhandled panics and silent failures.

## Goal

Ensure all production code uses explicit typed errors or safely falls back using established error mechanisms, rather than relying on `unwrap()` or `expect()`. Remove panics from critical execution paths (unless genuinely unreachable).

## Context

Using `unwrap()` and `expect()` obscures the source of errors and can crash the application abruptly. Instead of crashing, the application should handle failures gracefully or explicitly propagate them up the call stack to be diagnosed.

## Evidence

- path: "src/adapters/ansible/locator.rs"
  loc: "98, 99"
  note: "Locator functions use unwrap() to create directories or write files directly, bypassing error handling."
- path: "src/adapters/ansible/executor.rs"
  loc: "309, 311"
  note: "Executor logic unwraps directory accesses."
- path: "crates/mev-internal/src/adapters/gh.rs"
  loc: "71"
  note: "Unwrapping OS variables heavily or defaulting incorrectly."
- path: "crates/mev-internal/src/adapters/git.rs"
  loc: "36"
  note: "git base dir unwrap_or_else() uses current_dir().unwrap() without dealing with failure gracefully."

## Change Scope

- `src/adapters/ansible/locator.rs`
- `src/adapters/ansible/executor.rs`
- `crates/mev-internal/src/adapters/gh.rs`
- `crates/mev-internal/src/adapters/git.rs`
