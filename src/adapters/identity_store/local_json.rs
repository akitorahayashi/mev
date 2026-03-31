//! Configuration file store using JSON on disk.

use std::path::{Path, PathBuf};

use crate::domain::error::AppError;
use crate::domain::identity::{Identity, SwitchIdentity};
use crate::domain::ports::identity_store::{IdentityState, IdentityStore};

pub struct IdentityFileStore {
    identity_path: PathBuf,
}

impl IdentityFileStore {
    pub fn new(identity_path: PathBuf) -> Self {
        Self { identity_path }
    }

    fn legacy_config_path(&self) -> PathBuf {
        self.identity_path.with_file_name("config.json")
    }
}

impl IdentityStore for IdentityFileStore {
    fn exists(&self) -> bool {
        self.identity_path.exists() || self.legacy_config_path().exists()
    }

    fn load(&self) -> Result<IdentityState, AppError> {
        if self.identity_path.exists() {
            let content = std::fs::read_to_string(&self.identity_path)?;
            return serde_json::from_str(&content)
                .map_err(|e| AppError::Config(format!("failed to parse identity config: {e}")));
        }

        if self.legacy_config_path().exists() {
            let content = std::fs::read_to_string(self.legacy_config_path())?;
            let state: IdentityState = serde_json::from_str(&content).map_err(|e| {
                AppError::Config(format!("failed to parse legacy identity config: {e}"))
            })?;

            // Migrate automatically to the new path.
            if let Err(e) = self.save(&state) {
                eprintln!("Warning: failed to migrate identity config: {e}");
            }
            return Ok(state);
        }

        Err(AppError::Config("no identity configuration found".to_string()))
    }

    fn save(&self, state: &IdentityState) -> Result<(), AppError> {
        let parent = self
            .identity_path
            .parent()
            .ok_or_else(|| AppError::Config("identity path has no parent directory".to_string()))?;
        std::fs::create_dir_all(parent)?;

        let content = serde_json::to_string_pretty(state)
            .map_err(|e| AppError::Config(format!("failed to serialize identity config: {e}")))?;

        // Atomic write: write to temp file in same directory, then rename.
        let tmp_path = parent.join(".identity.json.tmp");
        std::fs::write(&tmp_path, &content)
            .map_err(|e| AppError::Config(format!("failed to write temp identity config: {e}")))?;
        std::fs::rename(&tmp_path, &self.identity_path).map_err(|e| {
            let _ = std::fs::remove_file(&tmp_path);
            AppError::Config(format!("failed to rename temp identity config: {e}"))
        })?;
        Ok(())
    }

    fn get_identity(&self, identity: SwitchIdentity) -> Result<Option<Identity>, AppError> {
        let state = self.load()?;
        match identity {
            SwitchIdentity::Personal => Ok(Some(state.personal)),
            SwitchIdentity::Work => Ok(Some(state.work)),
        }
    }

    fn identity_path(&self) -> &Path {
        self.identity_path.as_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_dummy_state() -> IdentityState {
        IdentityState {
            personal: Identity {
                name: "Personal Name".to_string(),
                email: "personal@example.com".to_string(),
            },
            work: Identity {
                name: "Work Name".to_string(),
                email: "work@example.com".to_string(),
            },
        }
    }

    #[test]
    fn exists_returns_false_when_neither_exist() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().join("identity.json");
        let store = IdentityFileStore::new(path);

        assert!(!store.exists());
        Ok(())
    }

    #[test]
    fn exists_returns_true_when_identity_exists() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().join("identity.json");
        std::fs::write(&path, "{}")?;

        let store = IdentityFileStore::new(path);
        assert!(store.exists());
        Ok(())
    }

    #[test]
    fn exists_returns_true_when_legacy_exists() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let legacy_path = dir.path().join("config.json");
        std::fs::write(&legacy_path, "{}")?;

        let path = dir.path().join("identity.json");
        let store = IdentityFileStore::new(path);
        assert!(store.exists());
        Ok(())
    }

    #[test]
    fn load_fails_when_neither_exists() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().join("identity.json");
        let store = IdentityFileStore::new(path);

        assert!(store.load().is_err());
        Ok(())
    }

    #[test]
    fn load_succeeds_from_new_path() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().join("identity.json");
        let state = create_dummy_state();
        let content = serde_json::to_string(&state)?;
        std::fs::write(&path, content)?;

        let store = IdentityFileStore::new(path);
        let loaded = store.load()?;
        assert_eq!(loaded.personal.email, "personal@example.com");
        Ok(())
    }

    #[test]
    fn load_succeeds_from_legacy_and_migrates() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let legacy_path = dir.path().join("config.json");
        let state = create_dummy_state();
        let content = serde_json::to_string(&state)?;
        std::fs::write(&legacy_path, content)?;

        let path = dir.path().join("identity.json");
        let store = IdentityFileStore::new(path.clone());

        assert!(!path.exists());

        let loaded = store.load()?;
        assert_eq!(loaded.personal.email, "personal@example.com");

        // Ensure it migrated
        assert!(path.exists());

        Ok(())
    }

    #[test]
    fn save_writes_atomically() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().join("nested").join("identity.json");
        let store = IdentityFileStore::new(path.clone());

        let state = create_dummy_state();
        store.save(&state)?;

        assert!(path.exists());

        let content = std::fs::read_to_string(&path)?;
        let loaded: IdentityState = serde_json::from_str(&content)?;
        assert_eq!(loaded.work.name, "Work Name");
        Ok(())
    }

    #[test]
    fn save_fails_without_parent() -> Result<(), Box<dyn std::error::Error>> {
        let path = PathBuf::from("");
        let store = IdentityFileStore::new(path);

        let state = create_dummy_state();
        assert!(store.save(&state).is_err());
        Ok(())
    }

    #[test]
    fn get_identity_returns_correct_variants() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().join("identity.json");
        let store = IdentityFileStore::new(path);

        let state = create_dummy_state();
        store.save(&state)?;

        let personal = store.get_identity(SwitchIdentity::Personal)?.ok_or("missing personal")?;
        assert_eq!(personal.name, "Personal Name");

        let work = store.get_identity(SwitchIdentity::Work)?.ok_or("missing work")?;
        assert_eq!(work.name, "Work Name");

        Ok(())
    }
}
