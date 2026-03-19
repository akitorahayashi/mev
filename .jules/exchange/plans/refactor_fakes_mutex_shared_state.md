---
label: "refacts"
---

## Goal

Replace coarse-grained `std::sync::Mutex` shared mutable state in `FakeFsPort` and `FakeAnsiblePort` with `std::cell::RefCell` to utilize isolated, single-owner structures aligned with synchronous test execution.

## Current State

- `src/testing/fakes.rs`: `FakeFsPort` and `FakeAnsiblePort` currently use `std::sync::Mutex` to wrap mock internal state (`files`, `dirs`, `events`). This unnecessary thread-safe synchronization is an anti-pattern for component tests and should be replaced with `std::cell::RefCell`.

## Plan

1. Edit `src/testing/fakes.rs` to replace `std::sync::Mutex` with `std::cell::RefCell`.
   - Update `FakeFsPort` struct fields `files`, `dirs`, and `events` to use `RefCell`.
   - Update `FakeAnsiblePort` struct field `events` to use `RefCell`.
   - Update the `.new()` implementations to use `RefCell::new(...)`.
   - Replace all instances of `.lock().unwrap()` with `.borrow()` or `.borrow_mut()` across all trait method implementations (`exists`, `read_to_string`, `read_dir`, `write`, `create_dir_all`, `remove_dir_all`, `copy`, `rename`, `is_dir`, `run_playbook`).
2. Run `cargo check --tests` and `cargo test` to verify that all synchronous component tests continue to compile and pass with the new `RefCell` structures, preserving externally observable behavior.
3. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
4. Submit the implementation changes.

## Acceptance Criteria

- All instances of `Mutex` used to wrap mock internal state in `src/testing/fakes.rs` are removed.
- `FakeFsPort` and `FakeAnsiblePort` utilize `RefCell` for internal state mutation.
- All test suites compile and pass successfully, demonstrating that multithreading synchronization was not strictly required and that external behavior is preserved.
- Old structures (`std::sync::Mutex` imports and usage in fakes) and terms are removed.
- Behavior invariants (the logical operations of the fakes) remain unchanged.

## Risks

- If any tests involving these fakes are multithreaded and share the fake across threads, `RefCell` will cause a compilation error (`RefCell` is `!Sync`). In that case, the tests or adapters themselves should be refactored to not rely on shared mutable test fakes across threads.
