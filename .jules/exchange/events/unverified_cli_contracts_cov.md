---
label: "tests"
created_at: "2024-05-24"
author_role: "cov"
confidence: "high"
---

## Problem

The main `mev` executable's CLI layer has dangerously low or non-existent coverage for its argument parsing and command dispatch. Specifically, `src/app/cli/*` and `src/main.rs` run without sufficient assertion logic that verifies external input.

## Goal

Add testing bounds around the main application command parsing to guarantee the shape of the CLI and its execution contracts remain stable. This focuses on providing basic help, version, and subcommand shape tests without relying heavily on internals.

## Context

Tarpaulin indicates the majority of the `src/app/cli/` modules are untested. This presents a high risk where renaming a flag or misconfiguring `clap` will break user-facing inputs without CI failure.

## Evidence

- path: "src/app/cli/make.rs"
  loc: "0/3 lines"
  note: "Command parsing entirely uncovered."
- path: "src/app/cli/config.rs"
  loc: "0/3 lines"
  note: "Config parsing uncovered."
- path: "src/app/cli/mod.rs"
  loc: "12/20 lines"
  note: "Core CLI dispatch is only partially covered, missing branches for specific commands."

## Change Scope

- `tests/cli/`
- `src/app/cli/mod.rs`
