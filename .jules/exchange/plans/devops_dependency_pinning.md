---
label: "refacts"
---

## Goal

Enhance CI/CD pipeline determinism and security by explicitly pinning external dependencies and implementing verifiable artifact provenance for builds.

## Current State

GitHub workflow actions install dynamic, unpinned dependency versions for tools (`pipx`, `ansible`, `shellcheck`, `shfmt`), creating risks for toolchain drift and non-deterministic execution. Furthermore, release build artifacts omit code signing or provenance attestation, lacking proof of origin.
- `.github/actions/setup-base/action.yml`: Installs `pipx` and `ansible` via `pip` and `pipx` without version specifiers.
- `.github/workflows/run-linters.yml`: Installs `shellcheck` and `shfmt` via `brew` without specifying versions.
- `.github/workflows/build.yml`: Creates binary and SHA256 hashes, but omits code signing or provenance generation.

## Plan

1. Modify `.github/actions/setup-base/action.yml` to specify explicit versions for `pipx` (e.g., `pipx==1.8.0`) and `ansible` (e.g., `ansible==11.0.0`) matching the versions in `pyproject.toml` and the latest releases.
2. Modify `.github/workflows/run-linters.yml` to install `shellcheck` and `shfmt` with explicit pinned versions (e.g., `shellcheck@0.11.0`, `shfmt@3.13.0`), for example by using `mise`, direct binary downloads, or pinning via `brew` syntax if available, instead of unpinned `brew install`.
3. Modify `.github/workflows/build.yml` to generate artifact attestations for the release binaries using an appropriate GitHub Action for build provenance and adding necessary workflow permissions.

## Acceptance Criteria

- `ansible`, `pipx`, `shellcheck`, and `shfmt` are pinned to explicit versions in workflows.
- Release artifacts are generated with cryptographic provenance attestations.

## Risks

- Explicit pinning might cause failures if a specified version becomes unavailable or introduces breaking changes.
- The workflow `build.yml` might fail if required permissions or OIDC configurations for artifact provenance attestation are missing.
