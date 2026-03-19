---
label: "refacts"
implementation_ready: false
---

## Goal

Replace coarse-grained `Mutex` shared mutable state with more isolated or single-owner structures. If tests are synchronous, `RefCell` is more appropriate. Alternatively, tests should be structured so that the `App` or the `Adapter` can run without needing heavily synchronized shared state.

## Problem

`FakeAnsiblePort` and other fakes in `src/testing/fakes.rs` utilize `std::sync::Mutex` to wrap mock internal state (like `files`, `dirs`, `events`). Defaulting to `Arc<Mutex<T>>` as a universal escape hatch is an anti-pattern. Test fakes shouldn't need thread-safe locking unless they are actively being used across multiple threads, which shouldn't be the case for simple component tests. If the adapter tests are indeed multithreaded, the lock scope and state management should be explicit and bounded.

## Evidence

- source_event: "mutex_shared_state_testing_rustacean.md"
  path: "src/testing/fakes.rs"
  loc: "11, 13, 15, 21-23"
  note: "`pub files: Mutex<HashMap<PathBuf, String>>` and other state wrapped in Mutex inside `FakeAnsiblePort`."

## Change Scope

- `src/testing/fakes.rs`

## Constraints

- Favor isolated or single-owner structures instead of `std::sync::Mutex` where multithreading is not strictly required.
- RefCell may be considered if tests are synchronous.

## Acceptance Criteria

- All instances of `Mutex` used to wrap mock internal state in `src/testing/fakes.rs` are replaced with isolated, single-owner, or appropriate single-threaded wrappers (like `RefCell`).
