# mev - macOS Environment Setup Project

## Overview

Rust-first CLI for macOS dev environment setup using bundled Ansible playbooks.
Installable as a standalone Rust binary via `install.sh`.

## Architecture

See [docs/architecture.md](docs/architecture.md) for architectural details and package structure.

## CLI Commands

See [README.md](README.md) for the list of available commands and usage instructions.

## Python Surface

Python ownership is limited to development tooling (`ansible-lint`) managed by `pyproject.toml`.
Runtime command ownership belongs to the Rust implementation.

## Development
- `just run <args>`: Run mev in dev mode
- `just check`: Format and lint
- `just test`: Run all Rust tests
- `v*` tag push: `.github/workflows/release.yml` delegates to `.github/workflows/build.yml`, and the build job attaches `mev-darwin-aarch64` plus its SHA256 file directly to GitHub Releases
