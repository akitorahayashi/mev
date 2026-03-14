---
label: "refacts"
implementation_ready: false
---

## Goal
Enhance CI/CD pipeline determinism and security by explicitly pinning external dependencies and implementing verifiable artifact provenance for builds.

## Problem
GitHub workflow actions install dynamic, unpinned dependency versions for tools (`pipx`, `ansible`, `shellcheck`, `shfmt`), creating risks for toolchain drift and non-deterministic execution. Furthermore, release build artifacts omit code signing or provenance attestation, lacking proof of origin.

## Context
This requirement aggregates observer events related to the problem statement above.

## Evidence
- source_event: "unpinned_dependencies_devops.md"
  path: ".github/actions/setup-base/action.yml"
  loc: "23-24"
  note: "`pipx` and `ansible` are installed via `pip` and `pipx` respectively without version specifiers."
- source_event: "unpinned_homebrew_dependencies_devops.md"
  path: ".github/workflows/run-linters.yml"
  loc: "31"
  note: "`brew install shellcheck shfmt` does not specify versions."
- source_event: "unverified_artifact_provenance_devops.md"
  path: ".github/workflows/build.yml"
  loc: "23-35"
  note: "The workflow creates binary and SHA256 hashes, but omits code signing or provenance generation."

## Change Scope
- `.github/actions/setup-base/action.yml`
- `.github/workflows/run-linters.yml`
- `.github/workflows/build.yml`

## Constraints
- Dependency versions should be strictly declared in the GitHub workflows matching local configuration files where possible.

## Acceptance Criteria
- `ansible`, `pipx`, `shellcheck`, and `shfmt` are pinned to explicit versions in workflows.
- Release artifacts are generated with cryptographic provenance attestations.
