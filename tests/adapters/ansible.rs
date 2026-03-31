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
