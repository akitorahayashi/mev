---
label: "refacts"
created_at: "2026-03-14"
author_role: "devops"
confidence: "high"
---

## Problem

External tool dependencies, specifically Homebrew packages (`shellcheck`, `shfmt`, `mise`) and Python pipx packages (`ansible`), are not pinned to specific versions in GitHub workflow and action definitions.

## Goal

Pin all external system dependencies in execution paths to guarantee determinism, isolate test signals from upstream supply chain mutations, and improve reproducibility.

## Context

Implicit trust of mutable external artifacts breaks deterministic execution. Unpinned tools like linters (`shellcheck`, `shfmt`) or runners (`mise`, `ansible`) can introduce breaking changes silently, creating CI flakiness and masking actual codebase errors. Trust boundaries must be explicit, and dependencies must be pinned to enforce operational integrity.

## Evidence

- path: ".github/workflows/run-linters.yml"
  loc: "line 29"
  note: "Unpinned Homebrew installation: `brew install shellcheck shfmt`."

- path: ".github/workflows/collect-coverage.yml"
  loc: "line 28"
  note: "Unpinned Homebrew installation: `brew install mise`."

- path: ".github/actions/setup-base/action.yml"
  loc: "line 20"
  note: "Unpinned pipx installation: `python -m pipx install --force ansible`."

## Change Scope

- `.github/workflows/run-linters.yml`
- `.github/workflows/collect-coverage.yml`
- `.github/actions/setup-base/action.yml`
