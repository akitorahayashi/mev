---
label: "tests"
created_at: "2024-04-17"
author_role: "auditor"
confidence: "high"
---

## Problem

Tests in `tests/cli/backup.rs` and `tests/cli/switch.rs` use overly broad textual assertions (`assert!(content.contains(...))`) rather than explicitly parsing the output structures (e.g. YAML or JSON) and validating their structured properties, which increases the risk of brittle tests or ambiguous failure diagnoses if the formatting changes slightly.

## Goal

Improve test failure diagnosability by converting broad substring string assertions into precise structure-based assertions that validate output semantics over textual syntax.

## Context

Using `assert!(content.contains("AppleShowAllFiles"));` ensures a substring exists but does not verify the data shape, nesting, or correctness of the broader config file, meaning failures will be hard to isolate to a specific logical data corruption. Parsing the structured output to assert on values isolates logic from simple string representation details.

## Evidence

- path: "tests/cli/backup.rs"
  loc: "28"
  note: "Asserts content contains a string instead of parsing generated YAML"
- path: "tests/cli/backup.rs"
  loc: "51"
  note: "Asserts content contains a string instead of parsing generated JSON"
- path: "tests/cli/switch.rs"
  loc: "29"
  note: "Asserts log_content contains a string instead of explicitly matching command invocations"

## Change Scope

- `tests/cli/backup.rs`
- `tests/cli/switch.rs`
