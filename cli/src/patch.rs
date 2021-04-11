use std::fs;
use crate::diff::diff;
use crate::diff::patch;
use crate::plan::Plan;
use std::io::{Read, Write};
use std::fs::File;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

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
        let (patch, size) = diff(source_contents, target_contents);
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
        file.write_u64::<BigEndian>(file_size as u64).unwrap();
        // Write the file name
        file.write(self.file.as_bytes()).unwrap();

        // Write the patch size
        file.write_u64::<BigEndian>(patch_size as u64).unwrap();
        // Write the decompressed patch size
        file.write_u64::<BigEndian>(decom_patch_size as u64).unwrap();
        // Write the patch data
        file.write(&self.patch).unwrap();
    }

    fn from_file(file: &mut File) -> FilePatch {
        // Read file path size
        let file_path_size = file.read_u64::<BigEndian>().unwrap();
        // Read file path
        let mut file_path = String::new();
        file.take(file_path_size).read_to_string(&mut file_path).unwrap();
        
        // Read patch size
        let patch_size = file.read_u64::<BigEndian>().unwrap();
        // Read decompressed patch size
        let decom_patch_size = file.read_u64::<BigEndian>().unwrap();
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
    patches: Vec<FilePatch>
}

impl Patch {

    // Create a new directory patch
    fn new() -> Patch {
        let patches = Vec::new();
        Patch {
            patches,
        }
    }

    // Add a file patch into the directory patch
    fn add(&mut self, patch: FilePatch) {
        self.patches.push(patch);
    }

    // Create a directory patch from a plan
    pub fn from_plan(plan: Plan) -> Patch {
        // Create a new directory patch
        let mut patch = Patch::new();
        // Add the patch for each file in the directory
        for file in plan.mutuals {
            let file_patch = FilePatch::calculate(file.path, &plan.source, &plan.target);            
            patch.add(file_patch);
        }
        // Return patch
        patch
    }

    pub fn apply(&self, dir: &String) {
        for file_patch in &self.patches {
            let path = format!("{}/{}", dir, file_patch.file);
            let source = fs::read(&path).unwrap();
            let patched = patch(source, &file_patch.patch, file_patch.size);
            let mut new_file = File::create(&path).unwrap();
            new_file.write_all(&patched).unwrap();
        }
    }
    
    pub fn write_to_file(&self, name: &String) {
        
        // Create a new file to write patch to
        let mut file = File::create(name).unwrap();
        
        // Write the number of patches to the patch file
        file.write_u64::<BigEndian>(self.patches.len() as u64).unwrap();
        
        // Write each file patch to the patch file 
        for file_patch in &self.patches {

            file_patch.write_to_file(&mut file);
            
        }
    }

    pub fn from_file(file: String) -> Patch {
        let mut patch = Patch::new();
        let mut file = File::open(file).unwrap(); 

        let patches = file.read_u64::<BigEndian>().unwrap();
        println!("patches = {}", patches);

        for x in 0..patches {
            let file_patch = FilePatch::from_file(&mut file);
            println!("fp = {}", file_patch.file);
            patch.add(file_patch);
        }
        
        patch
    }
}
