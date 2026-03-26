---
label: "tests"
---

## Goal

Provide rigorous test coverage ensuring `runtime_assets.rs` can safely extract embedded directories, thus increasing test coverage for `src/adapters/ansible/runtime_assets.rs` logic.

## Current State

- `src/adapters/ansible/runtime_assets.rs`: Contains the extraction logic in `materialize_embedded_ansible_dir` to write embedded assets to disk. Currently lacks adequate test coverage for extraction success and failure modes.
- `tests/adapters/ansible.rs` (does not exist): No integration or unit tests for the ansible assets module exists yet.
- `tests/adapters/mod.rs`: Needs to be updated to include the new `ansible` module.

## Plan

1. Write `tests/adapters/ansible.rs` with the following content:
   ```rust
   use std::fs;
   use mev::adapters::ansible::runtime_assets::{is_valid_ansible_dir, materialize_embedded_ansible_dir};

   #[test]
   fn test_materialize_embedded_ansible_dir() {
       let temp_dir = materialize_embedded_ansible_dir().expect("failed to materialize embedded ansible dir");

       let path = temp_dir.path();
       assert!(is_valid_ansible_dir(path));
       assert!(path.join("ansible.cfg").exists());
       assert!(path.join("hosts").exists());
       assert!(path.join("playbook.yml").exists());
       assert!(path.join("roles").is_dir());
   }

   #[test]
   fn test_is_valid_ansible_dir_invalid() {
       let temp_dir = tempfile::Builder::new().prefix("mev-test-").tempdir().unwrap();
       assert!(!is_valid_ansible_dir(temp_dir.path()));

       // Add playbook but no roles
       fs::write(temp_dir.path().join("playbook.yml"), "---").unwrap();
       assert!(!is_valid_ansible_dir(temp_dir.path()));

       // Add roles but no playbook
       fs::remove_file(temp_dir.path().join("playbook.yml")).unwrap();
       fs::create_dir(temp_dir.path().join("roles")).unwrap();
       assert!(!is_valid_ansible_dir(temp_dir.path()));
   }
   ```
2. Verify the contents of `tests/adapters/ansible.rs` by reading it.
3. Update `tests/adapters/mod.rs` to add `mod ansible;` to include the test in the test suite using git merge diff:
   ```rust
   <<<<<<< SEARCH
   mod git;
   mod jj;
   =======
   mod ansible;
   mod git;
   mod jj;
   >>>>>>> REPLACE
   ```
4. Verify the contents of `tests/adapters/mod.rs` by reading it.
5. Run `cargo test` to execute the full workspace test suite and ensure no regressions were introduced.

## Acceptance Criteria

- `runtime_assets.rs` logic is fully verified by isolated tests.
- `is_valid_ansible_dir` test coverage exists for successful and invalid scenarios.
- `cargo test` passes.
- No modifications are requested to non-test code since the implementation is already sound.

## Risks

- Flaky test if the extraction fails randomly or temp directory gets deleted before verification. (Mitigated by `tempfile` handling process lifetime scopes correctly).
