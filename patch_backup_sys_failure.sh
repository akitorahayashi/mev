#!/bin/bash
sed -i '$d' tests/cli/backup.rs
sed -i '$d' tests/cli/backup.rs
sed -i '$d' tests/cli/backup.rs
sed -i '$d' tests/cli/backup.rs
sed -i '$d' tests/cli/backup.rs
sed -i '$d' tests/cli/backup.rs
sed -i '$d' tests/cli/backup.rs
sed -i '$d' tests/cli/backup.rs
sed -i '$d' tests/cli/backup.rs
cat << 'INNEREOF' >> tests/cli/backup.rs
#[test]
fn backup_system_failure_no_definitions() {
    let ctx = TestContext::new();

    // To hit the "definitions directory not found" error without falling back to package defaults,
    // we must create an empty definitions directory so that resolution picks Local, but then
    // when load_definitions is called, it finds no items or the directory doesn't exist?
    // Wait, resolve_definitions_dir returns DefinitionsDirResolution::Local if local_definitions.exists().
    // If not, it returns DefinitionsDirResolution::PackageDefault.
    // If PackageDefault is returned, but package defaults directory is missing, it will hit the "definitions directory not found" error.
    // However, the test harness has package defaults (the ansible assets).
    // Therefore, if we want to hit the error, we need `definitions_dir` (resolved) to not exist.
    // Actually, `resolve_definitions_dir` returns the package defaults directory even if it doesn't exist.
    // If we override the local ansible configuration by mocking `MEV_ANSIBLE_ROLES_PATH`, we could break the fallback.
    // Alternatively, if we create the local `definitions` directory, it resolves to Local, and then it is checked for existence (it exists).
    // Then `load_definitions` is called, which finds no files and returns empty definitions.
    // So it returns "no setting definitions found in ...".
    ctx.cli()
        .env("MEV_ANSIBLE_ROLES_PATH", ctx.work_dir().join("missing-roles"))
        .args(["backup", "system"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("definitions directory not found"));
}
INNEREOF
