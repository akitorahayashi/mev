---
label: "refacts"
---

## Goal

Establish a single source-of-truth for workflow automation assets to prevent drift, explicitly map out the automation topology, and clarify the contract between execution surfaces and generation sources.

## Problem

The repository currently lacks an explicit, authoritative source-of-truth mapping for its automation control points. Multiple workflow files (`.github/workflows/*.yml`) and `justfile` commands are defined statically and act as execution points without clear generator contracts or a unified policy architecture, resulting in an implicit trust boundary and risk of drift.

## Affected Areas

### Automation Configuration Files

- `.github/workflows/`
- `justfile`
- `crates/mev-internal/justfile`

### Documentation

- `docs/automation_topology.md`

## Constraints

- There must be an explicit generator or a documented single source-of-truth mapping.
- The structure must eliminate ambiguity about where automations originate.
- Headers or documentation in automation files must reference this single source of truth.

## Risks

- Adding headers to workflows or justfiles might cause syntax issues if not formatted correctly as comments (`#`).
- Incorrect paths in the topology documentation might drift if not maintained.

## Acceptance Criteria

- A clearly documented standard policy logic (`docs/automation_topology.md`) is implemented to serve as the single source of truth for the automation topology.
- All files in `.github/workflows/` and `justfile`s trace their origin to this policy.
- Inline automation headers are added to all execution surfaces referencing this source of truth.

## Implementation Plan

1. Create a new documentation file `docs/automation_topology.md` detailing the standard policy logic for the repository's automation control points.
   - Run `write_file` to create `docs/automation_topology.md` containing the automation topology source of truth and mapping.
   - Run `cat docs/automation_topology.md` to verify its creation.
2. Add inline automation headers to `justfile` referencing the new documentation.
   - Use `run_in_bash_session` with `sed` or `replace_with_git_merge_diff` to add `# Automation Topology Source: docs/automation_topology.md` to the top of `justfile`.
   - Run `head -n 5 justfile` to verify.
3. Add inline automation headers to `crates/mev-internal/justfile`.
   - Use `run_in_bash_session` with `sed` or `replace_with_git_merge_diff` to add `# Automation Topology Source: docs/automation_topology.md` to the top of `crates/mev-internal/justfile`.
   - Run `head -n 5 crates/mev-internal/justfile` to verify.
4. Add inline automation headers to all `.yml` files in `.github/workflows/`.
   - Use `run_in_bash_session` with a loop and `sed` to prepend `# Automation Topology Source: docs/automation_topology.md` to every `.yml` file in the directory.
   - Run `head -n 5 .github/workflows/*.yml` to verify the headers were added correctly.
5. Run the verification and test suites using `run_in_bash_session` with commands `just check` and `just test` to ensure no regressions.
   - Use `run_in_bash_session` to run `just check` and `just test` to ensure that no regressions were introduced.
6. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
