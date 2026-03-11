---
label: "bugs"
implementation_ready: false
---

## Goal

Strengthen the verification gating and signature lineage of release artifacts from compilation through to the installer script, removing implicit trust bounds.

## Problem

The release and artifact provenance paths lack robust verification layers. Artifacts rely strictly on direct downloads and unpinned basic checksum validation instead of explicit, robust verification. The build pipeline compiles artifacts natively but the promotion model (via `release.yml` and `install.sh`) simply uploads binaries and runs `shasum`. There is no rigorous cryptographic verification, rollback readiness, or explicit trust handoff policy.

## Evidence

- source_event: "release_path_provenance_gaps_devops.md"
  path: "install.sh"
  loc: "lines 45-76"
  note: "Performs bare-minimum `shasum` checking over `curl` without signature checking or rigorous artifact lineage verification."
- source_event: "release_path_provenance_gaps_devops.md"
  path: ".github/workflows/release.yml"
  loc: "lines 1-13"
  note: "Pushes out releases directly by relying on `build.yml` with no intermediate signed gates or supply chain verification."

## Change Scope

- `install.sh`
- `.github/workflows/release.yml`
- `.github/workflows/build.yml`

## Constraints

- Release strategy requires that artifacts must be strictly proven.
- Scripts pulling artifacts over the network should perform rigorous checks.

## Acceptance Criteria

- `install.sh` validates signatures or pinned cryptographic proofs of release artifacts.
- `.github/workflows/release.yml` signs or cryptographically verifies artifacts prior to publishing.
