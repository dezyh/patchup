use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct FileInfo {
    pub base: String,
    pub path: String,
}

impl FileInfo {
    // Parses a single file
    pub fn from_file(base: &str, file: &PathBuf) -> FileInfo {
        FileInfo {
            base: base.to_string(),
            path: file
                .strip_prefix(base)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }

    // Parses all files in a directory
    pub fn from_dir(dir: &str) -> HashSet<FileInfo> {
        let files: HashSet<FileInfo> = WalkDir::new(&dir)
            .into_iter()
            .map(|entry| entry.unwrap().into_path())
            .filter(|path| path.is_file())
            .map(|file| FileInfo::from_file(&dir, &file))
            .collect();
        files
    }
}

impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for FileInfo {}
