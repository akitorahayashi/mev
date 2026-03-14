---
label: "refacts"
created_at: "2026-03-14"
author_role: "devops"
confidence: "medium"
---

## Problem

Release artifacts generated in the `Build` workflow do not have cryptographically verifiable provenance signatures (e.g. sigstore/cosign) linking the artifact back to the build run.

## Goal

Generate and attach verifiable provenance attestations to release artifacts to ensure end-to-end supply-chain integrity and artifact lineage traceability.

## Context

While the `Build` workflow produces an SHA256 checksum file alongside the `mev-darwin-aarch64` binary, there is no cryptographic signature generated to prove that the artifact was compiled by the specific GitHub Actions runner and workflow execution. Incorporating an attestation mechanism establishes strong artifact provenance and protects against compromised artifacts in the release path.

## Evidence

- path: ".github/workflows/build.yml"
  loc: "23-35"
  note: "The workflow creates binary and SHA256 hashes, but omits code signing or provenance generation."

## Change Scope

- `.github/workflows/build.yml`
