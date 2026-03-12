# Automation Topology

This document serves as the single source of truth for workflow automation assets within the repository. It explicitly maps out the automation topology and clarifies the contract between execution surfaces and generation sources to prevent drift and ambiguity.

## Purpose

The automation topology ensures that all automated control points have a clear origin, a documented purpose, and an explicit trace to a unified policy architecture. It removes implicit trust boundaries and minimizes the risk of structural drift.

## Execution Surfaces

The following execution surfaces are governed by this topology:

### Task Runners (`justfile`)
- `justfile` (root): Serves as the primary entry point for repository-level development and verification tasks.
- `crates/mev-internal/justfile`: Contains internal development tasks specific to the `mev-internal` crate.

### GitHub Actions Workflows (`.github/workflows/`)
All `.yml` files in `.github/workflows/` are managed as automation execution points. These files handle CI/CD, agent orchestration, testing, and verification.

## Generator Contract

To enforce the linkage between this source of truth and execution surfaces, every controlled file must include an inline header explicitly referencing this document:

`# Automation Topology Source: docs/automation_topology.md`

This ensures a traceable trust boundary from the execution surface back to this policy documentation. Any additions to task runners or CI workflows must strictly adhere to this topology mapping.
