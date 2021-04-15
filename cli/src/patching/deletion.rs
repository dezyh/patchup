use std::fs::File;
use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct Deletion {
    pub path: String,
}

impl Deletion {
    pub fn new(path: String) -> Deletion {
        Deletion { path }
    }

    pub fn write_to_file(&self, file: &mut File) {
        // Write path size and then path
        file.write_u64::<LittleEndian>(self.path.len() as u64)
            .unwrap();
        file.write(self.path.as_bytes()).unwrap();
    }

    pub fn read_from_file(file: &mut File) -> Deletion {
        // Read path size and then path
        let path_size = file.read_u64::<LittleEndian>().unwrap();
        let mut path = String::new();
        file.take(path_size).read_to_string(&mut path).unwrap();

        Deletion { path }
    }
}
