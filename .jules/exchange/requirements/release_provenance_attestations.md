---
label: "feats"
implementation_ready: false
---

## Goal

Secure the artifact release lineage by attaching SLSA-compliant provenance attestations and signatures during the release workflow.

## Problem

The current release process only generates simple SHA256 checksums, which does not securely verify lineage or prevent tampering. Provenance attestations are essential for securing the supply chain and establishing deterministic deployment identity.

## Evidence

- source_event: "release_provenance_gap_devops.md"
  path: ".github/workflows/build.yml"
  loc: "Prepare artifact step"
  note: "Calculates a simple SHA256 checksum but lacks secure signing or provenance metadata generation."

## Change Scope

- `.github/workflows/build.yml`
- `.github/workflows/release.yml`

## Constraints

- Must integrate into the existing GitHub Actions workflow gracefully.

## Acceptance Criteria

- Workflows are updated to generate provenance metadata for the released artifacts.
