---
label: "bugs"
implementation_ready: true
---

## Goal

Add `actions/attest-build-provenance@v2` with appropriate permissions to `.github/workflows/build.yml` for release builds.

## Problem

The CI environment documentation specifies that release build artifacts must use `actions/attest-build-provenance@v2` for code signing, requiring `id-token: write` and `attestations: write`. However, the current workflow lacks both the action call and the permissions.

## Evidence

- source_event: "build_provenance_consistency.md"
  path: ".github/workflows/build.yml"
  loc: "7-42"
  note: "The `build-darwin-aarch64` job has `permissions: contents: read` only and does not call `actions/attest-build-provenance@v2`."

## Change Scope

- `.github/workflows/build.yml`

## Constraints

- Ensure the permissions allow for provenance generation without compromising security.

## Acceptance Criteria

- The build workflow correctly defines the required permissions and runs the attestation step.