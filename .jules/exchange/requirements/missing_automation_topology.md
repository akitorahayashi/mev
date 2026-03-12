---
label: "bugs"
implementation_ready: false
---

## Goal

Create the missing `docs/automation_topology.md` file and inject the required inline header into all automation entrypoints.

## Problem

The canonical automation document `docs/automation_topology.md` is entirely missing, and automation files lack the mandatory `# Automation Topology Source: docs/automation_topology.md` header, breaking documentation architecture rules.

## Evidence

- source_event: "missing_automation_topology_devops.md"
  path: "docs/automation_topology.md"
  loc: ""
  note: "File is missing, representing a broken contract for the source of truth."
- source_event: "missing_automation_topology_devops.md"
  path: "justfile"
  loc: "line 1"
  note: "Missing the '# Automation Topology Source: docs/automation_topology.md' header."
- source_event: "missing_automation_topology_devops.md"
  path: ".github/workflows/build.yml"
  loc: "line 1"
  note: "Missing the '# Automation Topology Source: docs/automation_topology.md' header."

## Change Scope

- `docs/automation_topology.md`
- `justfile`
- `.github/workflows/build.yml`

## Constraints

- The new file must clearly document the basic automation boundaries as a source of truth.

## Acceptance Criteria

- `docs/automation_topology.md` is authored.
- The required inline header is correctly placed at the top of the `justfile` and relevant GitHub actions workflows.
