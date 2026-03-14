---
label: "tests"
created_at: "2026-03-14"
author_role: "cov"
confidence: "high"
---

## Problem

The GitHub repository reference domain model in `crates/mev-internal/src/domain/repository_ref.rs` has 0/48 lines covered during the workspace coverage run.

## Goal

Ensure the core domain parsing logic for repository references is fully covered by tests and properly executed in the CI coverage gate.

## Context

`RepositoryRef` handles critical domain logic: parsing SSH, HTTPS, and SCP-like Git remotes. Invariants in parsing must be robust to ensure correct repository operations downstream. A regression in parsing would silently break internal Git and GitHub adapter flows. Although there is a `#[cfg(test)]` module in the file, it is not being detected or executed by `cargo-tarpaulin` (0% coverage), indicating a gap in either test structure, coverage tool configuration for the workspace, or missing integration test execution.

## Evidence

- path: "crates/mev-internal/src/domain/repository_ref.rs"
  loc: "3-83"
  note: "Line coverage is at 0/48 lines. The unexecuted code includes core invariant checks like `parse_scp_like_remote` and `parse_ssh_remote` which process external state."

## Change Scope

- `crates/mev-internal/src/domain/repository_ref.rs`
