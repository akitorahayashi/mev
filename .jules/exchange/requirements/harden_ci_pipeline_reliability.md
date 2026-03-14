---
label: "bugs"
implementation_ready: false
---

## Goal

Remove `continue-on-error: true` from the coverage generation step to ensure failures during coverage collection are explicitly surfaced and block the pipeline. Pin all external tool dependencies to specific versions to guarantee deterministic execution, eliminate environmental drift, and mitigate supply chain risks.

## Problem

The test coverage generation step is configured to unconditionally succeed even if the underlying command fails, masking deterministic failures and hiding test signal quality. External tool dependencies (e.g., pipx, Ansible, and Homebrew packages) are installed without explicit version pinning across multiple CI workflows, violating dependency pinning requirements and introducing supply-chain risks.

## Context

Verification gates must produce fast and trustworthy signals for merge decisions. Using `continue-on-error: true` represents a silent fallback pattern that masks underlying execution failures, making it impossible to trust whether the test coverage metric represents a successful execution of the test suite or a failed command that was silently ignored. According to the DevOps Rule (Dependency Pinning), external tool dependencies (e.g., Homebrew packages, pipx packages) must be explicitly pinned to specific versions in execution paths and GitHub workflows. Unpinned dependencies can lead to non-deterministic failures, silent behavioral changes in CI pipelines, and potential security vulnerabilities if an upstream package is compromised.

## Evidence

- source_event: "masked_coverage_failures_devops.md"
  path: ".github/workflows/collect-coverage.yml"
  loc: "Generate coverage"
  note: "The step includes `continue-on-error: true` when running `just coverage`, masking any potential failures in coverage tool execution."
- source_event: "unpinned_tool_dependencies_devops.md"
  path: ".github/actions/setup-base/action.yml"
  loc: "Install ansible with pipx and export binary path"
  note: "pipx and ansible are installed without version pinning."
- source_event: "unpinned_tool_dependencies_devops.md"
  path: ".github/workflows/run-linters.yml"
  loc: "Install shell tools"
  note: "Homebrew packages shellcheck and shfmt are installed without version constraints."
- source_event: "unpinned_tool_dependencies_devops.md"
  path: ".github/workflows/collect-coverage.yml"
  loc: "Ensure mise is available"
  note: "Homebrew package mise is installed without version constraints."

## Change Scope

- `.github/workflows/collect-coverage.yml`
- `.github/actions/setup-base/action.yml`
- `.github/workflows/run-linters.yml`

## Constraints

- Ensure all changes align with architecture and design rules.
- Maintain tests for all new logic.

## Acceptance Criteria

- The problem is fully resolved.
- Pre-commit checks and tests pass.
