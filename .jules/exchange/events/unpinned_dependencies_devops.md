---
label: "refacts"
created_at: "2026-03-14"
author_role: "devops"
confidence: "high"
---

## Problem

External dependencies `pipx` and `ansible` are installed without explicit version pinning in the base setup action.

## Goal

Pin `pipx` and `ansible` to explicit versions to ensure deterministic execution, prevent unexpected regressions due to upstream updates, and mitigate supply chain risks.

## Context

The `setup-base` action currently installs `pipx` and `ansible` using `pip install --user pipx` and `pipx install --force ansible` respectively. This results in the latest available versions being installed, introducing a risk of breaking changes or supply chain attacks in upstream packages. Per DevOps rules, external dependencies must be pinned to specific versions to guarantee deterministic execution.

## Evidence

- path: ".github/actions/setup-base/action.yml"
  loc: "23-24"
  note: "`pipx` and `ansible` are installed via `pip` and `pipx` respectively without version specifiers, pulling the latest available versions."

## Change Scope

- `.github/actions/setup-base/action.yml`
