---
label: "docs"
created_at: "2026-03-23"
author_role: "consistency"
confidence: "high"
---

## Problem

The CI environment setup for release build artifacts does not use `actions/attest-build-provenance@v2` for code signing and build provenance, despite this being a required CI environment setup rule. The documentation incorrectly describes the release process using manual SHA256 hashes instead of build provenance.

## Goal

Update the CI release workflow to use `actions/attest-build-provenance@v2` with the necessary permissions, and update `docs/configuration.md` to accurately reflect this process.

## Context

The repository rules require that release build artifacts use `actions/attest-build-provenance@v2` for code signing and build provenance. However, `.github/workflows/build.yml` uses a manual `shasum` command and does not use the provenance action, and `docs/configuration.md` documents this manual process.

## Evidence

- path: ".github/workflows/build.yml"
  loc: "lines 25-30"
  note: "Uses manual `shasum -a 256` instead of `actions/attest-build-provenance@v2`."
- path: "docs/configuration.md"
  loc: "line 15"
  note: "Documents that 'the build job attaches mev-darwin-aarch64 plus its SHA256 file directly to GitHub Releases', ignoring build provenance requirements."

## Change Scope

- `.github/workflows/build.yml`
- `docs/configuration.md`
