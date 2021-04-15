use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct Addition {
    pub path: String,
    pub data: Vec<u8>,
}

impl Addition {
    pub fn new(base: &String, path: String) -> Addition {
        let data = fs::read(format!("{}/{}", &base, &path)).unwrap();
        Addition { path, data }
    }

    pub fn write_to_file(&self, file: &mut File) {
        // Write path size and then path
        file.write_u64::<LittleEndian>(self.path.len() as u64)
            .unwrap();
        file.write(self.path.as_bytes()).unwrap();
        
        // Write data size and then data
        file.write_u64::<LittleEndian>(self.data.len() as u64)
            .unwrap();
        file.write(&self.data).unwrap();
    }

    pub fn read_from_file(file: &mut File) -> Addition {
        // Read file path
        let path_size = file.read_u64::<LittleEndian>().unwrap();
        let mut path = String::new();
        file.take(path_size).read_to_string(&mut path).unwrap();

        // Read file contents
        let data_size = file.read_u64::<LittleEndian>().unwrap();
        let mut data = vec![0u8; data_size as usize];
        file.read_exact(&mut data).unwrap();

        Addition { path, data }
    }
}
