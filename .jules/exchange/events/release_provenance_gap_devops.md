---
label: "feats"
created_at: "2024-03-12"
author_role: "devops"
confidence: "high"
---

## Problem

Release lineage relies solely on `shasum -a 256 mev-darwin-aarch64` within the build action. There is no cryptographic signature, supply-chain attestation (SLSA), or SBOM to guarantee provenance and integrity from commit to the running environment.

## Goal

Implement robust artifact signing and attach SLSA-compliant provenance attestations during the release path to secure the artifact identity and establish a clear deployment lineage.

## Context

Determinism and provenance are baseline requirements. Trust boundaries must be explicit, and current implicit trust of the `.tar.gz`/binary hash leaves the release surface vulnerable to tampering, making the lineage untraceable.

## Evidence

- path: ".github/workflows/build.yml"
  loc: "Prepare artifact step"
  note: "Calculates a simple SHA256 checksum but lacks secure signing or provenance metadata generation."

## Change Scope

- `.github/workflows/build.yml`
- `.github/workflows/release.yml`
