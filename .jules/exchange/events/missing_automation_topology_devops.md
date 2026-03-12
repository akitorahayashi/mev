---
label: "bugs"
created_at: "2024-03-12"
author_role: "devops"
confidence: "high"
---

## Problem

The repository is missing the canonical `docs/automation_topology.md` file, which serves as the single source of truth for automation policies. Execution surfaces like `justfile` and `.github/workflows/*.yml` lack the required `# Automation Topology Source: docs/automation_topology.md` inline header.

## Goal

Create the `docs/automation_topology.md` file to document the automation graph, and inject the `# Automation Topology Source: docs/automation_topology.md` header into all automation entrypoints (e.g. `justfile`, `.github/workflows/*.yml`).

## Context

The system architecture defines `docs/automation_topology.md` as the authoritative source for workflow execution surfaces. Drifts between the documented and executed control plane introduce configuration ambiguity and violate the required source-of-truth integrity.

## Evidence

- path: "docs/automation_topology.md"
  loc: ""
  note: "File is missing, representing a broken contract for the source of truth."
- path: "justfile"
  loc: "line 1"
  note: "Missing the '# Automation Topology Source: docs/automation_topology.md' header."
- path: ".github/workflows/build.yml"
  loc: "line 1"
  note: "Missing the '# Automation Topology Source: docs/automation_topology.md' header."

## Change Scope

- `docs/automation_topology.md`
- `justfile`
- `.github/workflows/build.yml`
