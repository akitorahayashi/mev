---
label: "refacts"
---

## Goal

Ensure tests are isolated, deterministic, and do not manipulate global process state (like `PATH`), instead relying on dependency injection or test fakes.

## Current State

- `crates/mev-internal/src/testing/env_mock.rs`: Modifies global `PATH` in `create_mock_bin` using `unsafe { env::set_var }`. This impacts concurrent test execution safety.
- `crates/mev-internal/src/adapters/git.rs` and `crates/mev-internal/src/adapters/gh.rs`: Require `#[serial(env_path)]` macro because tests modify global `PATH` via `env_mock::create_mock_bin`.
- `tests/adapters/git.rs` and `tests/adapters/jj.rs`: Blindly verify Git and Jujutsu logic assuming the host machine contains specific pre-existing configurations (e.g., `get_identity` succeeding implies `.gitconfig` has `user.name` and `user.email`). They are non-deterministic across sandboxes.

## Plan

1. Edit `crates/mev-internal/src/testing/env_mock.rs` to remove the `PathGuard` and `unsafe { env::set_var }` calls. Update `create_mock_bin` to simply create the executable script in the temp directory and return a `std::path::PathBuf` to the temporary directory path instead of modifying the global process `PATH`.
2. Verify changes by running `cd crates/mev-internal && cargo check`.
3. Edit `crates/mev-internal/src/adapters/git.rs` to encapsulate the free functions into a `pub struct GitAdapter` containing an optional `mock_env_path` property (`Option<String>`). Update `git_command` to be a private method on `GitAdapter` that applies the `PATH` environment variable override if `mock_env_path` is set. Update all adapter contract tests in `git.rs` to instantiate `GitAdapter` with the mock `PATH` and call its methods, rather than relying on global `PATH` manipulation. Remove the `#[serial(env_path)]` and `#[serial(env_dir)]` attributes. For `remove_submodule_module_dir`, add a `current_dir` override capability to `GitAdapter` or explicitly use the mock directory context instead of `DirGuard`.
4. Verify changes by running `cd crates/mev-internal && cargo check`.
5. Edit `crates/mev-internal/src/app/commands/git/delete_submodule.rs`, `crates/mev-internal/src/app/commands/gh/labels_deploy.rs`, and `crates/mev-internal/src/app/commands/gh/labels_reset.rs` to instantiate `GitAdapter::default()` and invoke its methods.
6. Verify changes by running `cd crates/mev-internal && cargo check`.
7. Edit `crates/mev-internal/src/adapters/gh.rs` to encapsulate the free functions into a `pub struct GhAdapter` containing an optional `mock_env_path` property. Update `build_gh_command` to apply the `PATH` environment override. Update tests in `gh.rs` to instantiate `GhAdapter` with the mock `PATH` and remove `#[serial(env_path)]`.
8. Verify changes by running `cd crates/mev-internal && cargo check`.
9. Edit `crates/mev-internal/src/app/commands/gh/labels_deploy.rs` and `crates/mev-internal/src/app/commands/gh/labels_reset.rs` to instantiate `GhAdapter::default()` and invoke its methods.
10. Verify changes by running `cd crates/mev-internal && cargo check`.
11. Edit `src/adapters/git/cli.rs` to add a `pub home_dir: Option<PathBuf>` to `GitCli` to allow environment injection. Update the `Command::new("git")` builder to apply `.env("HOME", ...)` if `home_dir` is present.
12. Verify changes by running `cargo check`.
13. Edit `src/adapters/jj/cli.rs` similarly to add environment injection (`pub home_dir: Option<PathBuf>`) to `JjCli` for isolated testing.
14. Verify changes by running `cargo check`.
15. Edit `tests/adapters/git.rs` to inject a mock `HOME` directory into `GitCli` initialized with a valid mock git configuration using `fs::write` or a test fixture, and verify determinism instead of blind assertions on unconstrained host configs.
16. Edit `tests/adapters/jj.rs` to use an isolated mock `HOME` directory with an injected configuration into `JjCli`, asserting a predictable output regardless of the host environment.
17. Run all relevant tests via `cargo test` and `cd crates/mev-internal && cargo test` to verify no regressions were introduced and that all tests run deterministically.
18. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
19. Submit the changes with a PR addressing the requirement.

## Acceptance Criteria

- Adapter tests run deterministically in any sandbox environment regardless of host Git/JJ config.
- No tests rely on `unsafe` global `PATH` manipulation.
- `cargo test` runs without intermittent flakiness or mandatory serialization over `env_path`.

## Risks

- Overriding HOME may inadvertently cause tools to look in unexpected paths, causing spurious CI failures if any other tool dependencies leak into test suites.
- Adding properties to `GitCli` or `JjCli` changes their public interface, which may impact other components creating these struct instances.
