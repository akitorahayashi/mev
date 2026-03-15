---
label: "refacts"
---

## Goal

Relocate CLI-specific string input parsing and alias resolution out of core domain models and into the application CLI layer to maintain Boundary Sovereignty. Domain models will instantiate only through fallible constructors or explicit conversion traits invoked post-parsing.

## Current State

Core domain models are currently responsible for parsing raw CLI strings and resolving command-line aliases, injecting UI/CLI concerns directly into the business logic layer.
- `src/domain/vcs_identity.rs`: Defines hardcoded CLI aliases `SWITCH_IDENTITY_ALIASES` and `resolve_switch_identity` within the domain model.
- `src/domain/profile.rs`: Contains CLI aliases `PROFILE_ALIASES`, `resolve_profile`, `validate_machine_profile`, and `validate_profile` for string-based mapping rules.
- `src/domain/backup_target.rs`: Implements string-based parsing and alias resolution directly on the domain type using `from_input`.
- `src/app/cli/switch.rs`: Uses `vcs_identity::resolve_switch_identity` to parse CLI input string directly.
- `src/app/cli/create.rs`: Uses `profile::validate_machine_profile` to parse CLI input string directly.
- `src/app/cli/make.rs`: Uses `profile::validate_profile` to parse CLI input string directly.
- `src/app/cli/backup.rs`: Takes a `target: Option<String>` and passes it to `api::backup`.
- `src/app/api.rs`: `backup` accepts a `&str` for target and delegates to `commands::backup::execute`.
- `src/app/commands/backup/mod.rs`: Inside `execute`, it parses `BackupTarget::from_input`.

## Plan

1. Migrate string parsing logic for `SwitchIdentity` out of `src/domain/vcs_identity.rs` into `src/app/cli/switch.rs`. Remove `SWITCH_IDENTITY_ALIASES` and `resolve_switch_identity` from `src/domain/vcs_identity.rs`. Update `src/app/cli/switch.rs` to handle parsing strings like "p", "personal" to `SwitchIdentity::Personal`. Fix affected tests.
2. Migrate string parsing logic for `Profile` out of `src/domain/profile.rs` into `src/app/cli/` adapter layer. Remove `PROFILE_ALIASES`, `resolve_profile`, `validate_machine_profile`, and `validate_profile` from `src/domain/profile.rs`. Implement fallible constructors on `Profile` or explicit conversions. Update `src/app/cli/create.rs` and `src/app/cli/make.rs` to handle string to `Profile` conversion. Fix affected tests.
3. Migrate string parsing logic for `BackupTarget` out of `src/domain/backup_target.rs`. Remove `from_input` from `BackupTarget` in `src/domain/backup_target.rs`. Update `src/app/api.rs` and `src/app/commands/backup/mod.rs` to accept `BackupTarget` instead of `&str`. Update `src/app/cli/backup.rs` to handle mapping aliases to `BackupTarget`. Fix affected tests.
4. Run tests and clippy to verify no regressions.

## Acceptance Criteria

- CLI aliases are moved out of domain models `VcsIdentity`, `Profile`, and `BackupTarget`.
- Domain models instantiate only through fallible constructors or explicit conversion traits invoked post-parsing.
- `src/app/cli/` adapter layer handles input validation and alias resolution.
- Observable behavior does not change.
- All tests pass.

## Risks

- Removing string parsing from domain models might break internal usages that rely on these parsing methods.
- Moving test coverage to the CLI layer could lead to slightly lower domain coverage, necessitating CLI test additions.
