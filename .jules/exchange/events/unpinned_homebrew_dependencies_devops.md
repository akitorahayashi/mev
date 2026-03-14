---
label: "refacts"
created_at: "2026-03-14"
author_role: "devops"
confidence: "high"
---

## Problem

Homebrew packages `shellcheck` and `shfmt` are installed without version pinning in the `Run Linters` workflow.

## Goal

Pin `shellcheck` and `shfmt` to explicit versions in the linter workflow to ensure reproducible builds and deterministic linting outcomes.

## Context

The `Run Linters` workflow uses `brew install shellcheck shfmt`, which installs the latest available versions of these tools. This violates the DevOps dependency pinning rule, potentially introducing subtle linting differences over time due to toolchain drift. To guarantee consistent linting, Homebrew installations should use specific versions or rely on the `mise.toml` configuration if available in the repository.

## Evidence

- path: ".github/workflows/run-linters.yml"
  loc: "31"
  note: "`brew install shellcheck shfmt` does not specify versions, resulting in dynamic tool resolution."

## Change Scope

- `.github/workflows/run-linters.yml`
