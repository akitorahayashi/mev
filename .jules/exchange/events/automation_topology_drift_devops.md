---
label: "bugs"
created_at: "2026-03-11"
author_role: "devops"
confidence: "high"
---

## Problem

The repository lacks an explicit, authoritative source-of-truth mapping for its automation control points, and there is an implicit trust boundary between the build tools and the workflow executions.

## Goal

Establish a single source-of-truth generator for workflow automation assets to prevent drift, map out the automation topology explicitly, and clarify the contract between execution surfaces and generation sources.

## Context

Currently, multiple workflow files are defined statically under `.github/workflows/` (e.g., `build.yml`, `release.yml`, setup actions), and logic like `justfile` commands serve as execution points without clear generator contracts or unified policy architecture.

## Evidence

- path: ".github/workflows/"
  loc: "directory"
  note: "Multiple static workflow files exist with no explicit generator documented, violating source-of-truth integrity."
- path: "justfile"
  loc: "lines 1-122"
  note: "Serves as an execution entry point without explicit linkage to a master automation control plane."

## Change Scope

- `.github/workflows/`
- `justfile`
