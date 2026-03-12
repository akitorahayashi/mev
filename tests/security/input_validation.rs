//! Input validation security contracts.

use crate::harness::TestContext;
use predicates::prelude::*;

#[test]
fn create_rejects_invalid_profile() {
    let ctx = TestContext::new();

    ctx.cli()
        .args(["create", "nonexistent"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid profile: nonexistent"));
}

#[test]
fn switch_rejects_invalid_profile() {
    let ctx = TestContext::new();

    ctx.cli()
        .args(["switch", "badprofile"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid identity: invalid identity 'badprofile'"));
}
