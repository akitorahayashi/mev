use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::domain::error::AppError;
use crate::domain::ports::fs::FsPort;

pub struct FakeFsPort {
    // files maps a file path to its string content
    pub files: Mutex<HashMap<PathBuf, String>>,
    // dirs is a set of directory paths
    pub dirs: Mutex<HashSet<PathBuf>>,
    // events tracks method calls for assertions
    pub events: Mutex<Vec<String>>,
}

impl FakeFsPort {
    pub fn new() -> Self {
        Self {
            files: Mutex::new(HashMap::new()),
            dirs: Mutex::new(HashSet::new()),
            events: Mutex::new(Vec::new()),
        }
    }

    pub fn add_file(&self, path: &Path, content: &str) {
        self.files.lock().unwrap().insert(path.to_path_buf(), content.to_string());
        if let Some(parent) = path.parent() {
            self.add_dir(parent);
        }
    }

    pub fn add_dir(&self, path: &Path) {
        let mut dirs = self.dirs.lock().unwrap();
        Self::add_path_and_parents(&mut dirs, path);
    }

    fn add_path_and_parents(dirs: &mut HashSet<PathBuf>, path: &Path) {
        let mut current = path;
        while current != Path::new("") && current != Path::new("/") {
            dirs.insert(current.to_path_buf());
            if let Some(parent) = current.parent() {
                current = parent;
            } else {
                break;
            }
        }
        dirs.insert(path.to_path_buf());
    }
}

impl FsPort for FakeFsPort {
    fn exists(&self, path: &Path) -> bool {
        self.events.lock().unwrap().push(format!("exists: {}", path.display()));
        self.files.lock().unwrap().contains_key(path) || self.dirs.lock().unwrap().contains(path)
    }

    fn read_to_string(&self, path: &Path) -> Result<String, AppError> {
        self.events.lock().unwrap().push(format!("read_to_string: {}", path.display()));
        self.files.lock().unwrap().get(path).cloned().ok_or_else(|| {
            AppError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"))
        })
    }

    fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>, AppError> {
        self.events.lock().unwrap().push(format!("read_dir: {}", path.display()));
        if !self.dirs.lock().unwrap().contains(path) {
            return Err(AppError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "directory not found",
            )));
        }

        let mut entries = Vec::new();
        for file in self.files.lock().unwrap().keys() {
            if let Some(parent) = file.parent()
                && parent == path
                && !entries.contains(file)
            {
                entries.push(file.clone());
            }
        }
        for dir in self.dirs.lock().unwrap().iter() {
            if let Some(parent) = dir.parent()
                && parent == path
                && !entries.contains(dir)
            {
                entries.push(dir.clone());
            }
        }
        Ok(entries)
    }

    fn write(&self, path: &Path, content: &[u8]) -> Result<(), AppError> {
        self.events.lock().unwrap().push(format!("write: {}", path.display()));
        self.files
            .lock()
            .unwrap()
            .insert(path.to_path_buf(), String::from_utf8_lossy(content).to_string());
        Ok(())
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), AppError> {
        self.events.lock().unwrap().push(format!("create_dir_all: {}", path.display()));
        self.add_dir(path);
        Ok(())
    }

    fn remove_dir_all(&self, path: &Path) -> Result<(), AppError> {
        self.events.lock().unwrap().push(format!("remove_dir_all: {}", path.display()));
        let mut dirs = self.dirs.lock().unwrap();
        let mut files = self.files.lock().unwrap();

        let to_remove_dirs: Vec<PathBuf> =
            dirs.iter().filter(|p| p.starts_with(path)).cloned().collect();
        for p in to_remove_dirs {
            dirs.remove(&p);
        }

        let to_remove_files: Vec<PathBuf> =
            files.keys().filter(|p| p.starts_with(path)).cloned().collect();
        for p in to_remove_files {
            files.remove(&p);
        }

        Ok(())
    }

    fn copy(&self, from: &Path, to: &Path) -> Result<u64, AppError> {
        self.events.lock().unwrap().push(format!("copy: {} -> {}", from.display(), to.display()));
        let content = {
            self.files.lock().unwrap().get(from).cloned().ok_or_else(|| {
                AppError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"))
            })?
        };
        self.files.lock().unwrap().insert(to.to_path_buf(), content.clone());
        Ok(content.len() as u64)
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<(), AppError> {
        self.events.lock().unwrap().push(format!("rename: {} -> {}", from.display(), to.display()));
        let mut dirs = self.dirs.lock().unwrap();
        let mut files = self.files.lock().unwrap();

        let to_rename_dirs: Vec<PathBuf> =
            dirs.iter().filter(|p| p.starts_with(from)).cloned().collect();
        for p in to_rename_dirs {
            dirs.remove(&p);
            let rel = p.strip_prefix(from).unwrap();
            dirs.insert(to.join(rel));
        }

        let to_rename_files: Vec<(PathBuf, String)> = files
            .iter()
            .filter(|(p, _)| p.starts_with(from))
            .map(|(p, c)| (p.clone(), c.clone()))
            .collect();
        for (p, content) in to_rename_files {
            files.remove(&p);
            let rel = p.strip_prefix(from).unwrap();
            files.insert(to.join(rel), content);
            if let Some(parent) = to.join(rel).parent() {
                Self::add_path_and_parents(&mut dirs, parent);
            }
        }

        Ok(())
    }

    fn is_dir(&self, path: &Path) -> bool {
        self.dirs.lock().unwrap().contains(path)
    }
}
