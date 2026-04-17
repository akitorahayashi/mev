---
label: "docs"
created_at: "2024-04-17"
author_role: "proofreader"
confidence: "high"
---

## Problem

The `docs/configuration.md` file does not reflect the current Python development tooling used in the repository.

## Goal

Update the documentation to accurately reflect the use of `uv` for Python dependency management.

## Context

The repository has migrated to using `uv` for Python development tooling (as seen in the presence of `uv.lock`, `.python-version`, and commands in `justfile`). However, `docs/configuration.md` still lists only `pyproject.toml` and omits `uv.lock` and `.python-version`. Furthermore, while `pipx` remains the correct tool for installing the runtime `ansible` prerequisite, the development workflows for `ansible-lint` are now managed by `uv`.

## Evidence

- path: "docs/configuration.md"
  loc: "5-13"
  note: "The Files table lists `pyproject.toml` but omits `uv.lock` and `.python-version`, which are now core configuration files for Python tooling."

## Change Scope

- `docs/configuration.md`
