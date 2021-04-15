use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct File {
    pub base: String,
    pub path: String,
}

impl File {
    // Parses a single file
    pub fn from_file(base: &str, file: &PathBuf) -> File {
        File {
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
    pub fn from_dir(dir: &str) -> HashSet<File> {
        let files: HashSet<File> = WalkDir::new(&dir)
            .into_iter()
            .map(|entry| entry.unwrap().into_path())
            .filter(|path| path.is_file())
            .map(|file| File::from_file(&dir, &file))
            .collect();
        files
    }
}

impl Hash for File {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for File {}
