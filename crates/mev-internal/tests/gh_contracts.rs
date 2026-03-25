use serial_test::serial;
use std::env;
use std::fs;
use mev_internal::app::commands::gh::{labels_deploy, labels_reset};
use tempfile::TempDir;

pub struct PathGuard {
    original_path: String,
}

impl Drop for PathGuard {
    fn drop(&mut self) {
        unsafe {
            env::set_var("PATH", &self.original_path);
        }
    }
}

pub fn create_mock_bin(name: &str, temp_dir: &TempDir, script_content: &str) -> PathGuard {
    let bin_path = temp_dir.path().join(name);
    fs::write(&bin_path, script_content).unwrap();

    let mut perms = fs::metadata(&bin_path).unwrap().permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    fs::set_permissions(&bin_path, perms).unwrap();

    let original_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{}:{}", temp_dir.path().display(), original_path);
    unsafe {
        env::set_var("PATH", new_path);
    }

    PathGuard { original_path }
}

#[test]
#[serial(env_path)]
fn test_gh_labels_deploy() {
    let temp_dir = tempfile::tempdir().unwrap();
    let gh_log = temp_dir.path().join("gh_log.txt");

    let mock_script = format!(
        "#!/bin/sh\necho \"$@\" >> \"{}\"\nif [ \"$1\" = \"label\" ] && [ \"$2\" = \"list\" ]; then\n    echo \"bugs\"\nelse\n    exit 0\nfi",
        gh_log.display()
    );
    let mock_script = mock_script.replace("$\n", "$\\\n");

    let _path_guard = create_mock_bin("gh", &temp_dir, &mock_script);

    let args = labels_deploy::LabelsDeployArgs {
        repo: Some("owner/repo".to_string()),
    };

    labels_deploy::run(args).expect("deploy should succeed");

    let log_content = fs::read_to_string(gh_log).unwrap();
    assert!(log_content.contains("label list"));
    assert!(log_content.contains("label delete bugs"));
    assert!(log_content.contains("label create bugs"));
}

#[test]
#[serial(env_path)]
fn test_gh_labels_reset() {
    let temp_dir = tempfile::tempdir().unwrap();
    let gh_log = temp_dir.path().join("gh_log.txt");

    let mock_script = format!(
        "#!/bin/sh\necho \"$@\" >> \"{}\"\nif [ \"$1\" = \"label\" ] && [ \"$2\" = \"list\" ]; then\n    echo \"bugs\"\nelse\n    exit 0\nfi",
        gh_log.display()
    );
    let mock_script = mock_script.replace("$\n", "$\\\n");

    let _path_guard = create_mock_bin("gh", &temp_dir, &mock_script);

    let args = labels_reset::LabelsResetArgs {
        repo: Some("owner/repo".to_string()),
    };

    labels_reset::run(args).expect("reset should succeed");

    let log_content = fs::read_to_string(gh_log).unwrap();
    assert!(log_content.contains("label delete bugs"));
}
