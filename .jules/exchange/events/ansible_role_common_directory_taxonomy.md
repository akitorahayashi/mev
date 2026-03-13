---
label: "refacts"
created_at: "2026-03-13"
author_role: "taxonomy"
confidence: "high"
---

## Problem

The `common` directory name is extensively used within `src/assets/ansible/roles/*/config/common`, violating the naming rule prohibiting ambiguous names like `common`.

## Goal

Rename the `common` directories to a specific domain term reflecting their single responsibility, such as `default` or `shared_config`.

## Context

Ambiguous names like `common` are prohibited as they restate package/directory scope and hide true responsibilities.

## Evidence

- path: "src/assets/ansible/roles/"
  loc: "config/common"
  note: "Multiple roles use 'common' as the subdirectory for configuration (e.g. system, shell, nodejs, etc)."

## Change Scope

- `src/assets/ansible/roles/*/config/common`
