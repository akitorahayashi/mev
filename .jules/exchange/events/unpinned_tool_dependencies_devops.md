---
label: "refacts"
created_at: "2025-03-14"
author_role: "devops"
confidence: "high"
---

## Problem

External tool dependencies (e.g., pipx, Ansible, and Homebrew packages) are installed without explicit version pinning across multiple CI workflows, violating dependency pinning requirements and introducing supply-chain risks.

## Goal

Pin all external tool dependencies to specific versions to guarantee deterministic execution, eliminate environmental drift, and mitigate supply chain risks.

## Context

According to the DevOps Rule (Dependency Pinning), external tool dependencies (e.g., Homebrew packages, pipx packages) must be explicitly pinned to specific versions in execution paths and GitHub workflows. Unpinned dependencies can lead to non-deterministic failures, silent behavioral changes in CI pipelines, and potential security vulnerabilities if an upstream package is compromised.

## Evidence

- path: ".github/actions/setup-base/action.yml"
  loc: "Install ansible with pipx and export binary path"
  note: "pipx and ansible are installed without version pinning."

- path: ".github/workflows/run-linters.yml"
  loc: "Install shell tools"
  note: "Homebrew packages shellcheck and shfmt are installed without version constraints."

- path: ".github/workflows/collect-coverage.yml"
  loc: "Ensure mise is available"
  note: "Homebrew package mise is installed without version constraints."

## Change Scope

- `.github/actions/setup-base/action.yml`
- `.github/workflows/run-linters.yml`
- `.github/workflows/collect-coverage.yml`
