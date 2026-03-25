sed -i 's/^pub fn create_mock_bin/use tempfile::TempDir;\n\npub fn create_mock_bin/' crates/mev-internal/src/testing/env_mock.rs
