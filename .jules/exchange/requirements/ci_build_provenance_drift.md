---
label: "docs"
implementation_ready: true
---

## Goal

Update the CI release workflow to use `actions/attest-build-provenance@v2` with necessary permissions, and update `docs/configuration.md` to accurately reflect this process.

## Problem

The CI environment setup for release build artifacts does not use `actions/attest-build-provenance@v2` for code signing and build provenance, despite this being a required CI environment setup rule. The documentation incorrectly describes the release process using manual SHA256 hashes instead of build provenance.

## Evidence

- source_event: "build_provenance_drift_consistency.md"
  path: ".github/workflows/build.yml"
  loc: "lines 25-30"
  note: "Uses manual `shasum -a 256` instead of `actions/attest-build-provenance@v2`."
- source_event: "build_provenance_drift_consistency.md"
  path: "docs/configuration.md"
  loc: "line 15"
  note: "Documents that 'the build job attaches mev-darwin-aarch64 plus its SHA256 file directly to GitHub Releases', ignoring build provenance requirements."

## Change Scope

- `.github/workflows/build.yml`
- `docs/configuration.md`

## Constraints

- Build artifacts must be signed and provenance attested using `actions/attest-build-provenance@v2`.
- `id-token: write` and `attestations: write` permissions must be configured for the relevant job.
- Declarative updates must preserve existing content without duplication or complete replacement.

## Acceptance Criteria

- The `.github/workflows/build.yml` file uses `actions/attest-build-provenance@v2`.
- Workflow job permissions include `id-token: write` and `attestations: write`.
- `docs/configuration.md` accurately describes the build provenance process.