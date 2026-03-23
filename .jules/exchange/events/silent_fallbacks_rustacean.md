---
label: "refacts"
created_at: "2024-05-24"
author_role: "rustacean"
confidence: "high"
---

## Problem

Multiple usage of `.unwrap_or_default()` when parsing configurations or standard paths, masking failure and providing silent fallbacks.

## Goal

Ensure silent fallbacks are explicit, opt-in, or clearly logged. If an operation fails, it should propagate an error rather than silently defaulting, or the default should be justified.

## Context

Silent fallback behaviors drift configuration and mask failures. Falling back to default strings without surfacing a failure makes debugging difficult, violating the "No silent fallback" principle.

## Evidence

- path: "src/adapters/git/cli.rs"
  loc: "read_config"
  note: "`read_config` uses `.unwrap_or_default()` if `git config` fails, silently suppressing config fetch errors."

- path: "src/adapters/ansible/locator.rs"
  loc: "locate_ansible_dir"
  note: "Locator logic defaults silently to returning an empty string/path when resolution fails."

## Change Scope

- `src/adapters/git/cli.rs`
- `src/adapters/ansible/locator.rs`