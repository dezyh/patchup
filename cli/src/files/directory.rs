use std::collections::HashSet;
use crate::files::file::File;

pub struct DirectoryMap {
    pub source: String,
    pub target: String,
    pub mutuals: HashSet<File>,
    pub additions: HashSet<File>,
    pub deletions: HashSet<File>,
}

impl DirectoryMap {

    pub fn new(source: String, target: String) -> DirectoryMap {
        
        // Find all files in the source and target directories
        let mut source_files = File::from_dir(&source);
        let mut target_files = File::from_dir(&target);

        // Find all mutual files between source and target directories
        let mutual_files = DirectoryMap::mutual_files(&mut source_files, &mut target_files);

        DirectoryMap {
            source,
            target,
            mutuals: mutual_files,
            additions: target_files,
            deletions: source_files,
        }
    }

    // Calculate the intersection of source files and target files in place, moving the intersection into a new hashset
    fn mutual_files(
        source_files: &mut HashSet<File>,
        target_files: &mut HashSet<File>,
    ) -> HashSet<File> {
        let intersection: HashSet<File> = source_files
            .iter()
            .filter_map(|file| target_files.take(file))
            .collect();
        source_files.retain(|file| !intersection.contains(&file));
        intersection
    }
}
