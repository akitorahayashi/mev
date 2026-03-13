---
label: "docs"
created_at: "2026-03-14"
author_role: "devops"
confidence: "high"
---

## Problem

The repository lacks a centralized automation topology contract (`docs/automation_topology.md`), and execution surfaces do not reference it.

## Goal

Establish a single source of truth for automation policies and enforce its reference across all execution surfaces to ensure consistency and prevent source-of-truth drift.

## Context

According to the memory rules, the repository uses `docs/automation_topology.md` as the single source of truth for automation policies. Execution surfaces (like `justfile` and `.github/workflows/*.yml`) must include the inline header `# Automation Topology Source: docs/automation_topology.md`. Currently, the documentation file is missing, and the execution surfaces do not contain the required header.

## Evidence

- path: "docs/automation_topology.md"
  loc: "file"
  note: "File does not exist in the repository."

- path: "justfile"
  loc: "line 1"
  note: "Missing `# Automation Topology Source: docs/automation_topology.md` header."

- path: ".github/workflows/ci-workflows.yml"
  loc: "line 1"
  note: "Missing `# Automation Topology Source: docs/automation_topology.md` header."

## Change Scope

- `docs/automation_topology.md`
- `justfile`
- `.github/workflows/*.yml`
