use std::fs;
use crate::diff::diff as do_diff;
use crate::diff::patch as do_patch;
use crate::plan::Plan;
use std::io::{Read, Write};
use std::fs::File;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

struct FileAdd {
    path: String,
    data: Vec<u8>,
}

impl FileAdd {

    fn new(base: &String, path: String) -> FileAdd {
        let data = fs::read(format!("{}/{}", &base, &path)).unwrap();
        FileAdd {
            path,
            data,
        }
    }

    fn write_to_file(&self, file: &mut File) {
        // Write path size and then path
        file.write_u64::<LittleEndian>(self.path.len() as u64).unwrap();
        file.write(self.path.as_bytes()).unwrap();
        // Write data size and then data
        file.write_u64::<LittleEndian>(self.data.len() as u64).unwrap();
        file.write(&self.data).unwrap();
    }

    fn read_from_file(file: &mut File) -> FileAdd {
        // Read file path
        let path_size = file.read_u64::<LittleEndian>().unwrap();
        let mut path = String::new();
        file.take(path_size).read_to_string(&mut path).unwrap();

        // Read file contents
        let data_size = file.read_u64::<LittleEndian>().unwrap();
        let mut data = vec![0u8; data_size as usize];
        file.read_exact(&mut data).unwrap();
       
        FileAdd {
            path,
            data,
        }
    }
}

struct FileDel {
    path: String,
}

impl FileDel {

    fn new(path: String) -> FileDel {
        FileDel {
            path
        }
    }

    fn write_to_file(&self, file: &mut File) {
        // Write path size and then path
        file.write_u64::<LittleEndian>(self.path.len() as u64).unwrap();
        file.write(self.path.as_bytes()).unwrap();
    }

    fn read_from_file(file: &mut File) -> FileDel {
        // Read path size and then path
        let path_size = file.read_u64::<LittleEndian>().unwrap();
        let mut path = String::new();
        file.take(path_size).read_to_string(&mut path).unwrap();

        FileDel {
            path
        }
    }
}

struct FilePatch {
    file: String,
    patch: Vec<u8>,
    size: usize,
}

impl FilePatch {

    fn calculate(file: String, source: &String, target: &String) -> FilePatch { 
        // Read source and target file contents as bytes
        let source_contents = fs::read(format!("{}/{}", source, file)).unwrap();
        let target_contents = fs::read(format!("{}/{}", target, file)).unwrap();
        // Calculate the patch diff
        let (patch, size) = do_diff(source_contents, target_contents);
        // Return patch
        FilePatch {
            file,
            patch,
            size
        }
    }

    fn write_to_file(&self, file: &mut File) {

        // Calculate some sizes to allow reading from file 
        let file_size = self.file.len();
        let patch_size = self.patch.len();
        let decom_patch_size = self.size;

        // Write the file name size
        file.write_u64::<LittleEndian>(file_size as u64).unwrap();
        // Write the file name
        file.write(self.file.as_bytes()).unwrap();

        // Write the patch size
        file.write_u64::<LittleEndian>(patch_size as u64).unwrap();
        // Write the decompressed patch size
        file.write_u64::<LittleEndian>(decom_patch_size as u64).unwrap();
        // Write the patch data
        file.write(&self.patch).unwrap();
    }

    fn from_file(file: &mut File) -> FilePatch {
        // Read file path size
        let file_path_size = file.read_u64::<LittleEndian>().unwrap();
        // Read file path
        let mut file_path = String::new();
        file.take(file_path_size).read_to_string(&mut file_path).unwrap();
        
        // Read patch size
        let patch_size = file.read_u64::<LittleEndian>().unwrap();
        // Read decompressed patch size
        let decom_patch_size = file.read_u64::<LittleEndian>().unwrap();
        // Read patch bytes
        let mut patch_bytes = vec![0u8; patch_size as usize];
        file.read_exact(&mut patch_bytes).unwrap();

        FilePatch {
            file: file_path,
            patch: patch_bytes,
            size: decom_patch_size as usize,
        }
    }
}

pub struct Patch {
    patches: Vec<FilePatch>,
    additions: Vec<FileAdd>,
    deletions: Vec<FileDel>,
}

impl Patch {

    // Create a new directory patch
    fn new() -> Patch {
        let patches = Vec::new();
        let additions = Vec::new();
        let deletions = Vec::new();
        Patch {
            patches,
            additions,
            deletions,
        }
    }

    // Add a file patch into the directory patch
    fn add(&mut self, patch: FilePatch) {
        self.patches.push(patch);
    }

    // Create a directory patch from a plan
    pub fn from_plan(plan: Plan) -> Patch {
        let mut patch = Patch::new();
        
        // Add the patch for each file in the directory
        for file in plan.mutuals {
            let file_patch = FilePatch::calculate(file.path, &plan.source, &plan.target);            
            patch.add(file_patch);
        }

        for deletion in plan.deletions {
            patch.deletions.push(FileDel::new(deletion.path));
        }

        for addition in plan.additions {
            patch.additions.push(FileAdd::new(&plan.target, addition.path));
        }

        // Return patch
        patch
    }

    fn apply_patches(&self, dir: &String) {
        for patch in &self.patches {
            let path = format!("{}/{}", dir, patch.file);
            let source = fs::read(&path).unwrap();
            let patched = do_patch(source, &patch.patch, patch.size);
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
        self.apply_patches(dir);
        self.apply_deletions(dir);
        self.apply_additions(dir);
    }
    
    pub fn write_to_file(&self, name: &String) {
        
        // Create a new file to write patch to
        let mut file = File::create(name).unwrap();
        
        // Write the number of patches to the patch file
        file.write_u64::<LittleEndian>(self.patches.len() as u64).unwrap();
        
        // Write each file patch to the patch file 
        for file_patch in &self.patches {

            file_patch.write_to_file(&mut file);
            
        }
    }

    fn read_patches(&mut self, file: &mut File) {
        let patches = file.read_u64::<LittleEndian>().unwrap();
        for _patch in 0..patches {
            self.patches.push(FilePatch::from_file(file));
        }
    }

    fn read_additions(&mut self, file: &mut File) {
        let additions = file.read_u64::<LittleEndian>().unwrap();
        for _addition in 0..additions {
            self.additions.push(FileAdd::read_from_file(file));
        }
    }

    fn read_deletions(&mut self, file: &mut File) {
        let deletions = file.read_u64::<LittleEndian>().unwrap();
        for _deletion in 0..deletions {
            self.deletions.push(FileDel::read_from_file(file));
        }
    }

    pub fn from_file(file: String) -> Patch {
        let mut patch = Patch::new();
        let mut file = File::open(file).unwrap(); 
        patch.read_patches(&mut file);
        patch.read_additions(&mut file);
        patch.read_deletions(&mut file);
        patch
    }

    fn write_patches(&self, file: &mut File) {
        file.write_u64::<LittleEndian>(self.patches.len() as u64).unwrap();
        for patch in &self.patches {
            patch.write_to_file(file);
        }
    }

    fn write_additions(&self, file: &mut File) {
        file.write_u64::<LittleEndian>(self.additions.len() as u64).unwrap();
        for addition in &self.additions {
            addition.write_to_file(file);
        }
    }

    fn write_deletions(&self, file: &mut File) {
        file.write_u64::<LittleEndian>(self.deletions.len() as u64).unwrap();
        for deletion in &self.deletions {
            deletion.write_to_file(file);
        }
    }

    pub fn write_file(&self, file: &String) {
        let mut file = File::create(file).unwrap();
        self.write_patches(&mut file);
        self.write_additions(&mut file);
        self.write_deletions(&mut file);
    }

}
