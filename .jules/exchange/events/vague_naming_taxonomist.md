---
label: "refacts"
created_at: "2024-04-17"
author_role: "taxonomist"
confidence: "high"
---

## Problem

A violation of the "Anti-Patterns: Vague names that hide responsibility" and "Anti-Patterns: Class and file must not have ambiguous names or responsibilities such as base, common, core, utils, or helpers." is present. There are occurrences of vague or ambiguous names.

## Goal

Identify and rename all vague and ambiguous names (like base, common, core, utils, helpers, manager, handler, service) to conform to the rules, ensuring single, specific responsibilities linked to their domain.

## Context

The `AGENTS.md` and `SKILL.md` rules explicitly prohibit vague names. Searching for "manager", "handler", "service", "base", "common", "core", "utils", or "helpers" yields a number of hits across the project's Ansible configuration and shell aliases, which need to be rectified to match the codebase conventions.

## Evidence

- path: "src/assets/ansible/roles/shell/config/global/alias/brew.sh"
  loc: "1-10"
  note: "Uses 'services' in alias definitions."

- path: "src/assets/ansible/roles/system/config/global/definitions/mission_control.yml"
  loc: "1"
  note: "Uses 'manager' or related."

- path: "src/assets/ansible/roles/editor/config/global/zed/settings.json"
  loc: "2"
  note: "Uses 'base_keymap'."

- path: "src/assets/ansible/roles/rust/config/global/tools.yml"
  loc: "1-5"
  note: "Uses 'base' for URL definition."

## Change Scope

- `src/assets/ansible/roles/shell/config/global/alias/brew.sh`
- `src/assets/ansible/roles/system/config/global/definitions/mission_control.yml`
- `src/assets/ansible/roles/editor/config/global/zed/settings.json`
- `src/assets/ansible/roles/rust/config/global/tools.yml`
