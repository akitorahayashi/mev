#!/bin/bash
# Remove all backup_system_failure_no_definitions implementations
sed -i '/fn backup_system_failure_no_definitions/,/^}/d' tests/cli/backup.rs
sed -i '/#\[test\]/d' tests/cli/backup.rs # oops this will remove all #[test]

git checkout tests/cli/backup.rs
