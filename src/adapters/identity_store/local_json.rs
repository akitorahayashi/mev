//! Configuration file store using JSON on disk.

use std::path::{Path, PathBuf};

use crate::domain::error::AppError;
use crate::domain::ports::identity_store::{IdentityState, IdentityStore};
use crate::domain::vcs_identity::{SwitchIdentity, VcsIdentity};

#[derive(serde::Serialize, serde::Deserialize)]
struct VcsIdentityDto {
    name: String,
    email: String,
}

impl From<VcsIdentityDto> for VcsIdentity {
    fn from(dto: VcsIdentityDto) -> Self {
        Self {
            name: dto.name,
            email: dto.email,
        }
    }
}

impl From<&VcsIdentity> for VcsIdentityDto {
    fn from(identity: &VcsIdentity) -> Self {
        Self {
            name: identity.name.clone(),
            email: identity.email.clone(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct IdentityStateDto {
    personal: VcsIdentityDto,
    work: VcsIdentityDto,
}

impl From<IdentityStateDto> for IdentityState {
    fn from(dto: IdentityStateDto) -> Self {
        Self {
            personal: dto.personal.into(),
            work: dto.work.into(),
        }
    }
}

impl From<&IdentityState> for IdentityStateDto {
    fn from(state: &IdentityState) -> Self {
        Self {
            personal: (&state.personal).into(),
            work: (&state.work).into(),
        }
    }
}

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
            let dto: IdentityStateDto = serde_json::from_str(&content)
                .map_err(|e| AppError::Config(format!("failed to parse identity config: {e}")))?;
            return Ok(dto.into());
        }

        if self.legacy_config_path().exists() {
            let content = std::fs::read_to_string(self.legacy_config_path())?;
            let dto: IdentityStateDto = serde_json::from_str(&content).map_err(|e| {
                AppError::Config(format!("failed to parse legacy identity config: {e}"))
            })?;
            let state: IdentityState = dto.into();

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

        let dto: IdentityStateDto = state.into();
        let content = serde_json::to_string_pretty(&dto)
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

    fn get_identity(&self, identity: SwitchIdentity) -> Result<Option<VcsIdentity>, AppError> {
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
