---
label: "bugs"
created_at: "2026-03-11"
author_role: "devops"
confidence: "high"
---

## Problem

The release and artifact provenance paths lack robust verification layers. Artifacts rely strictly on direct downloads and unpinned basic checksum validation instead of explicit, robust verification.

## Goal

Strengthen the verification gating and signature lineage of release artifacts from compilation through to the installer script, removing implicit trust bounds.

## Context

The build pipeline compiles artifacts natively but the promotion model (via `release.yml` and `install.sh`) simply uploads binaries and runs `shasum`. There is no rigorous cryptographic verification, rollback readiness, or explicit trust handoff policy.

## Evidence

- path: "install.sh"
  loc: "lines 45-76"
  note: "Performs bare-minimum `shasum` checking over `curl` without signature checking or rigorous artifact lineage verification."
- path: ".github/workflows/release.yml"
  loc: "lines 1-13"
  note: "Pushes out releases directly by relying on `build.yml` with no intermediate signed gates or supply chain verification."

## Change Scope

- `install.sh`
- `.github/workflows/release.yml`
- `.github/workflows/build.yml`
