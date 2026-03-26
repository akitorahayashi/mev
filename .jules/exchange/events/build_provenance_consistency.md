---
label: "bugs"
created_at: "2024-03-26"
author_role: "consistency"
confidence: "high"
---

## Problem

The CI environment documentation specifies that release build artifacts use `actions/attest-build-provenance@v2` for code signing and build provenance, which requires adding `id-token: write` and `attestations: write` permissions to the relevant workflow job. However, `.github/workflows/build.yml` does not implement `actions/attest-build-provenance@v2` and lacks the necessary permissions.

## Goal

Add `actions/attest-build-provenance@v2` to `.github/workflows/build.yml` when uploading release assets, along with the required `id-token: write` and `attestations: write` permissions.

## Context

According to the provided architecture rule for CI Environment Setup (Provenance), release build artifacts must use `actions/attest-build-provenance@v2` for code signing and build provenance, which requires adding `id-token: write` and `attestations: write` permissions to the relevant workflow job. The actual `build.yml` currently builds and uploads the artifact with its sha256 checksum but skips the provenance attestation step.

## Evidence

- path: ".github/workflows/build.yml"
  loc: "7-42"
  note: "The `build-darwin-aarch64` job has `permissions: contents: read` only and does not call `actions/attest-build-provenance@v2` after building the release artifact."

## Change Scope

- `.github/workflows/build.yml`
