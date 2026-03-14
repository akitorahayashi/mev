---
label: "bugs"
created_at: "2025-03-14"
author_role: "devops"
confidence: "high"
---

## Problem

The test coverage generation step is configured to unconditionally succeed even if the underlying command fails, masking deterministic failures and hiding test signal quality.

## Goal

Remove `continue-on-error: true` from the coverage generation step to ensure failures during coverage collection are explicitly surfaced and block the pipeline.

## Context

Verification gates must produce fast and trustworthy signals for merge decisions. Using `continue-on-error: true` represents a silent fallback pattern that masks underlying execution failures, making it impossible to trust whether the test coverage metric represents a successful execution of the test suite or a failed command that was silently ignored.

## Evidence

- path: ".github/workflows/collect-coverage.yml"
  loc: "Generate coverage"
  note: "The step includes `continue-on-error: true` when running `just coverage`, masking any potential failures in coverage tool execution."

## Change Scope

- `.github/workflows/collect-coverage.yml`
