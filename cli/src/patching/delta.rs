use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::patching::core::diff;

pub struct Delta {
    pub file: String,
    pub patch: Vec<u8>,
    pub size: usize,
}

impl Delta {
    
    fn idential(source: &Vec<u8>, target: &Vec<u8>) -> bool {
        
        // Check size of source and target is the same
        if source.len() != target.len() {
            return false
        }

        for i in 0..source.len() {
            if source[i] != target[i] {
                return false
            }
        }

        return true
    }

    pub fn calculate(file: String, source: &String, target: &String) -> Option<Delta> {
        // Read source and target file contents as bytes
        let source_contents = fs::read(format!("{}/{}", source, file)).unwrap();
        let target_contents = fs::read(format!("{}/{}", target, file)).unwrap();

        match Delta::idential(&source_contents, &target_contents) {
            true => None,
            false => {
                let (patch, size) = diff(source_contents, target_contents);
                Some(Delta { file, patch, size })
            }
        }
    }

    pub fn write_to_file(&self, file: &mut File) {
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
        file.write_u64::<LittleEndian>(decom_patch_size as u64)
            .unwrap();
        // Write the patch data
        file.write(&self.patch).unwrap();
    }

    pub fn from_file(file: &mut File) -> Delta {
        // Read file path size
        let file_path_size = file.read_u64::<LittleEndian>().unwrap();
        // Read file path
        let mut file_path = String::new();
        file.take(file_path_size)
            .read_to_string(&mut file_path)
            .unwrap();

        // Read patch size
        let patch_size = file.read_u64::<LittleEndian>().unwrap();
        // Read decompressed patch size
        let decom_patch_size = file.read_u64::<LittleEndian>().unwrap();
        // Read patch bytes
        let mut patch_bytes = vec![0u8; patch_size as usize];
        file.read_exact(&mut patch_bytes).unwrap();

        Delta {
            file: file_path,
            patch: patch_bytes,
            size: decom_patch_size as usize,
        }
    }
}
