use crate::file::FileInfo;
use std::collections::HashSet;

pub struct Plan {
    pub source: String,
    pub target: String,
    pub mutuals: HashSet<FileInfo>,
    pub additions: HashSet<FileInfo>,
    pub deletions: HashSet<FileInfo>,
}

impl Plan {

    // Calculate the intersection of source files and target files in place, moving the intersection into a new hashset
    fn mutual_files(source_files: &mut HashSet<FileInfo>, target_files: &mut HashSet<FileInfo>) -> HashSet<FileInfo> {
        let intersection: HashSet<FileInfo> = source_files
            .iter()
            .filter_map(|file| target_files.take(file))
            .collect();
        source_files.retain(|file| !intersection.contains(&file));
        intersection 
    }

    pub fn new(source: String, target: String) -> Plan {

        // Find all files in the source and target directories
        let mut source_files = FileInfo::from_dir(&source);
        let mut target_files = FileInfo::from_dir(&target);

        // Find all mutual files between source and target directories
        let mutual_files = Plan::mutual_files(&mut source_files, &mut target_files);

        Plan {
            source,
            target,
            mutuals: mutual_files,
            additions: target_files,
            deletions: source_files,
        }
    }

    pub fn print(&self) {
        println!("source: {}", &self.source);
        println!("target: {}", &self.target);
        for file in &self.mutuals {
            println!("~ : {}", file.path);
        }
        for file in &self.additions {
            println!("+ : {}", file.path);
        }
        for file in &self.deletions {
            println!("- : {}", file.path);
        }
    }
}
