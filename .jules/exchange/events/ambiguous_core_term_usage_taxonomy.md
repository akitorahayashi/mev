---
label: "refacts"
created_at: "2026-03-12"
author_role: "taxonomy"
confidence: "medium"
---

## Problem

The term "core" is used in the `svo-cli-design` skill document which violates the anti-pattern rule prohibiting "core" as a vague identifier.

## Goal

Replace occurrences of "core" with precise terms when not referring to specific packages or configuration parameters.

## Context

The rule is primarily for code, but using "core" loosely in prompt guidelines like `SKILL.md` creates an inconsistency with the repository-wide vocabulary guidelines.

## Evidence

- path: "src/assets/ansible/roles/nodejs/config/common/coder/skills/svo-cli-design/SKILL.md"
  loc: "line 8, 10"
  note: "Uses 'Core Objective' and 'core required inputs'. These can be replaced with 'Primary Objective' and 'fundamental required inputs'."
- path: "src/assets/ansible/roles/nodejs/config/common/coder/skills/effective-prompting/SKILL.md"
  loc: "line 8"
  note: "Uses 'Core Objective' as a section header."

## Change Scope

- `src/assets/ansible/roles/nodejs/config/common/coder/skills/svo-cli-design/SKILL.md`
- `src/assets/ansible/roles/nodejs/config/common/coder/skills/effective-prompting/SKILL.md`
