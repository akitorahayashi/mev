---
label: "refacts"
created_at: "2024-03-14"
author_role: "data_arch"
confidence: "high"
---

## Problem

Adapter implementations silently swallow external command failures by using `unwrap_or_default()`, masking errors instead of bubbling them up.

## Goal

Surface explicit errors to the caller using appropriate `Result` types instead of falling back to default values when external commands fail.

## Context

The design rules explicitly prohibit silent fallbacks; any fallback must be explicit, opt-in, and surfaced as a failure or logged. Additionally, the Rust Design Rule regarding error handling mandates that errors must preserve domain meaning without collapsing into defaults in production paths where failure is plausible.

## Evidence

- path: "src/adapters/git/cli.rs"
  loc: "read_config"
  note: "Uses `unwrap_or_default()` when the git command fails, silently returning an empty string instead of an error."
- path: "src/adapters/jj/cli.rs"
  loc: "read_config"
  note: "Uses `unwrap_or_default()` when the jj command fails, silently returning an empty string instead of an error."

## Change Scope

- `src/adapters/git/cli.rs`
- `src/adapters/jj/cli.rs`
