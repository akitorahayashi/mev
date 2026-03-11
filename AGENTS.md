# mev - macOS Environment Setup Project

## Overview

Rust-first CLI for macOS dev environment setup using bundled Ansible playbooks.
Installable as a standalone Rust binary via `install.sh`.

## Documentation

See [docs/README.md](docs/README.md) for detailed documentation and architecture.

## Architecture Principles

### Directory Naming
- No ambiguous names: `core/`, `utils/`, `helpers/` are forbidden
- Every file must belong to a clear, specific category

## Design Rules

### Path Resolution
- CLI passes `profile`, `config_dir_abs_path`, `repo_root_path`, `local_config_root` as Ansible extra vars
- `local_config_root` points to `~/.config/mev/roles` for externalized configs
- Roles handle fallback logic (profile-specific → common)

### Profile Design
- Common profile by default: most roles use `common` profile
- Profile-specific configs: `brew` role supports profile-specific configs (macbook/mac-mini)
- Roles store configs in `config/common/` (all roles) and `config/profiles/` (brew only)

### Config Deployment Strategy
Two-stage config deployment:
1. Package → `~/.config/mev/roles/{role}/`: Copy via `mev config create` or auto-deploy on `mev make`
2. `~/.config/mev/roles/{role}/` → Local destinations: Symbolic links

### Development
- `just run <args>`: Run mev in dev mode
- `just check`: Format and lint
- `just test`: Run all Rust tests
- `v*` tag push: `.github/workflows/release.yml` delegates to `.github/workflows/build.yml`, and the build job attaches `mev-darwin-aarch64` plus its SHA256 file directly to GitHub Releases
