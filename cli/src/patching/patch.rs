use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::patching::core::patch;
use crate::files::DirectoryMap;
use crate::patching::{Addition, Deletion, Delta};

pub struct Patch {
    deltas: Vec<Delta>,
    additions: Vec<Addition>,
    deletions: Vec<Deletion>,
}

impl Patch {
    // Create a new directory patch
    fn new() -> Patch {
        let deltas = Vec::new();
        let additions = Vec::new();
        let deletions = Vec::new();
        Patch {
            deltas,
            additions,
            deletions,
        }
    }

    // Create a directory patch from a plan
    pub fn from_dirs(dirs: DirectoryMap) -> Patch {
        let mut patch = Patch::new();

        // Add the patch for each file in the directory
        for file in dirs.mutuals {
            let delta = Delta::calculate(file.path, &dirs.source, &dirs.target);
            match delta {
                Some(delta) => patch.deltas.push(delta),
                None => ()
            }
        }

        for deletion in dirs.deletions {
            patch.deletions.push(Deletion::new(deletion.path));
        }

        for addition in dirs.additions {
            patch.additions.push(Addition::new(&dirs.target, addition.path));
        }

        // Return patch
        patch
    }

    fn apply_deltas(&self, dir: &String) {
        for delta in &self.deltas {
            let path = format!("{}/{}", dir, delta.file);
            let source = fs::read(&path).unwrap();
            let patched = patch(source, &delta.patch, delta.size);
            let mut new_file = File::create(&path).unwrap();
            new_file.write_all(&patched).unwrap();
        }
    }

    fn apply_deletions(&self, dir: &String) {
        for deletion in &self.deletions {
            fs::remove_file(format!("{}/{}", dir, deletion.path)).unwrap();
        }
    }

    fn apply_additions(&self, dir: &String) {
        for addition in &self.additions {
            let mut file = File::create(format!("{}/{}", dir, addition.path)).unwrap();
            file.write_all(&addition.data).unwrap();
        }
    }

    pub fn apply(&self, dir: &String) {
        self.apply_deltas(dir);
        self.apply_deletions(dir);
        self.apply_additions(dir);
    }

    fn read_deltas(&mut self, file: &mut File) {
        let patches = file.read_u64::<LittleEndian>().unwrap();
        for _patch in 0..patches {
            self.deltas.push(Delta::from_file(file));
        }
    }

    fn read_additions(&mut self, file: &mut File) {
        let additions = file.read_u64::<LittleEndian>().unwrap();
        for _addition in 0..additions {
            self.additions.push(Addition::read_from_file(file));
        }
    }

    fn read_deletions(&mut self, file: &mut File) {
        let deletions = file.read_u64::<LittleEndian>().unwrap();
        for _deletion in 0..deletions {
            self.deletions.push(Deletion::read_from_file(file));
        }
    }

    pub fn from_file(file: String) -> Patch {
        let mut patch = Patch::new();
        let mut file = File::open(file).unwrap();
        patch.read_deltas(&mut file);
        patch.read_additions(&mut file);
        patch.read_deletions(&mut file);
        patch
    }

    fn write_deltas(&self, file: &mut File) {
        file.write_u64::<LittleEndian>(self.deltas.len() as u64)
            .unwrap();
        for patch in &self.deltas {
            patch.write_to_file(file);
        }
    }

    fn write_additions(&self, file: &mut File) {
        file.write_u64::<LittleEndian>(self.additions.len() as u64)
            .unwrap();
        for addition in &self.additions {
            addition.write_to_file(file);
        }
    }

    fn write_deletions(&self, file: &mut File) {
        file.write_u64::<LittleEndian>(self.deletions.len() as u64)
            .unwrap();
        for deletion in &self.deletions {
            deletion.write_to_file(file);
        }
    }

    pub fn write_file(&self, file: &String) {
        let mut file = File::create(file).unwrap();
        self.write_deltas(&mut file);
        self.write_additions(&mut file);
        self.write_deletions(&mut file);
    }
}
