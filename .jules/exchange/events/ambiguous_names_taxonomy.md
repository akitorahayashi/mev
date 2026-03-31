---
label: "refacts"
created_at: "2026-03-31"
author_role: "taxonomy"
confidence: "high"
---

## Problem

Files and variables are named using generic terms like `utils`, `helper`, `base`, `common`, or `core`, which violates the specific design anti-patterns.

## Goal

Identify and rename all instances of `utils`, `helper`, `base`, `common`, and `core` to names that describe their specific responsibilities, eliminating ambiguous module and variable names.

## Context

The rule "Class and file must not have ambiguous names or responsibilities such as base, common, core, utils, or helpers" explicitly prohibits these terms because they lead to dumping grounds for unrelated code and do not specify the problem domain. Several files and variables in the codebase violate this rule.
For example, `src/adapters/identity_store/paths.rs` contains `fn config_base()`. There are references to "Common development commands" and "Helper function". Several Cargo.lock entries also show usage of `-core` crates (though we might not be able to change external dependency names, we should avoid them in our code).

## Evidence

- path: "src/adapters/identity_store/paths.rs"
  loc: "fn config_base()"
  note: "Violates the naming rule by using the suffix `_base`."

- path: "src/assets/ansible/roles/shell/config/global/alias/dev/dev.sh"
  loc: "line 13: # Helper function..."
  note: "Violates the naming rule in a comment describing a function as a helper."

- path: "src/assets/ansible/roles/shell/config/global/alias/dev/dev.sh"
  loc: "line 19: # Common development commands"
  note: "Uses the term 'Common'."

- path: "src/assets/ansible/roles/nodejs/config/global/coder/skills/svo-cli-design/SKILL.md"
  loc: "line 8: ## Core Objective"
  note: "Uses the term 'Core'."

## Change Scope

- `src/adapters/identity_store/paths.rs`
- `src/assets/ansible/roles/shell/config/global/alias/dev/dev.sh`
- `src/assets/ansible/roles/nodejs/config/global/coder/skills/svo-cli-design/SKILL.md`
- `src/assets/ansible/roles/nodejs/config/global/coder/skills/effective-prompting/SKILL.md`
