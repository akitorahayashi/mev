---
label: "tests"
---

## Goal
Consolidate basic CLI shape checks into a central boundary test (`tests/cli/help_and_version.rs`) and provide foundational test coverage for untrusted/uncovered boundary interfaces and CLI logic in the main app and internal crates.

## Current State
- `tests/cli/help_and_version.rs`: Only tests primary subcommands help and version.
- `tests/cli/make.rs`: Duplicate boundary test coverage for CLI argument checks exists across subcommand test files instead of centralizing global arguments and shape tests.
- `tests/cli/create.rs`: Contains shape tests for `--help`
- `tests/cli/backup.rs`: Contains shape tests for `--help` and `list`
- `tests/cli/switch.rs`: Contains shape tests for `--help`
- `tests/cli/config.rs`: Contains shape tests for `--help`
- `tests/cli/identity.rs`: Contains shape tests for `--help`
- `tests/cli/list.rs`: Contains shape tests for `--help`
- `src/app/cli/mod.rs`: The main app's CLI layer lacks test coverage.
- `crates/mev-internal/src/app/cli/gh.rs`: Only tests exist for subcommand shape but zero line coverage recorded in tarpaulin output.
- `crates/mev-internal/tests/`: Lacks shape tests for subcommands.

## Plan

1. Move `make_help_shows_overwrite_flag`, `make_help_shows_verbose_flag`, `make_help_shows_profile_flag` from `tests/cli/make.rs` to `tests/cli/help_and_version.rs`. Remove these tests from `tests/cli/make.rs`.
2. Verify changes to `tests/cli/make.rs` and `tests/cli/help_and_version.rs` by reading the files.
3. Move `create_help_shows_overwrite_flag`, `create_help_shows_verbose_flag` from `tests/cli/create.rs` to `tests/cli/help_and_version.rs`. Remove these tests from `tests/cli/create.rs`.
4. Verify changes to `tests/cli/create.rs` by reading the file.
5. Move `backup_help_shows_target_argument`, `backup_alias_bk_is_accepted`, `backup_list_shows_targets`, `backup_short_list_flag_shows_targets`, `backup_unknown_target_fails`, `backup_help_visible_in_main_help` from `tests/cli/backup.rs` to `tests/cli/help_and_version.rs`. Remove these tests from `tests/cli/backup.rs`.
6. Verify changes to `tests/cli/backup.rs` by reading the file.
7. Move `switch_help_shows_identity_argument`, `switch_alias_sw_is_accepted`, `switch_requires_identity_argument`, `switch_without_config_fails_gracefully`, `switch_help_visible_in_main_help` from `tests/cli/switch.rs` to `tests/cli/help_and_version.rs`. Remove these tests from `tests/cli/switch.rs`.
8. Verify changes to `tests/cli/switch.rs` by reading the file.
9. Move `config_deploy_help` from `tests/cli/config.rs` to `tests/cli/help_and_version.rs`. Remove this test from `tests/cli/config.rs`.
10. Verify changes to `tests/cli/config.rs` by reading the file.
11. Move `identity_show_help`, `identity_set_help` from `tests/cli/identity.rs` to `tests/cli/help_and_version.rs`. Remove these tests from `tests/cli/identity.rs`.
12. Verify changes to `tests/cli/identity.rs` by reading the file.
13. Move `list_help_shows_description`, `list_alias_ls_is_accepted`, `list_visible_in_main_help`, `list_shows_expected_sections` from `tests/cli/list.rs` to `tests/cli/help_and_version.rs`. Remove these tests from `tests/cli/list.rs`.
14. Verify changes to `tests/cli/list.rs` by reading the file.
15. Add a test module in `crates/mev-internal/src/app/cli/mod.rs` to verify the CLI shape using `clap::CommandFactory`. Add `use clap::CommandFactory;` to the module. Implement tests to parse `gh labels deploy --help`, `gh labels reset --help` and `git delete-submodule --help` checking that they parse or error out showing help output using `Cli::command().try_get_matches_from(...)`.
16. Verify changes to `crates/mev-internal/src/app/cli/mod.rs` by reading the file.
17. Run tests by executing `cargo test` and `cd crates/mev-internal && cargo test` to ensure changes didn't break functionality.
18. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
19. Submit changes.

## Acceptance Criteria
- Basic CLI shape checks are consolidated into a single central test file (`tests/cli/help_and_version.rs`).
- Coverage boundaries for missing CLI/adapter areas are established.

## Risks
- Tests might fail due to missing dependencies in the test environment. (Mitigated by using isolated test harnesses or mocks).
- Moving tests might break existing test coverage if not done correctly. (Mitigated by explicitly writing tests that cover the same code paths).
