use pipe::pipe;
use std::thread;
use std::io::{Cursor};
use std::io::{copy};

pub fn diff(source: Vec<u8>, target: Vec<u8>) -> (Vec<u8>, usize) {
    
    // Calculate patch
    let (patch_reader, mut patch_writer) = pipe();
    thread::spawn(move || {
        bidiff::simple_diff(&source[..], &target[..], &mut patch_writer)
            .unwrap();
    });
   
    // Compress patch
    let patch = zstd::stream::encode_all(patch_reader, 0).unwrap();
    let patch_size = patch.len();
    
    (patch, patch_size)
}

pub fn patch(source: Vec<u8>, patch: &Vec<u8>, patch_size: usize) -> Vec<u8> {

    // Decompress patch
    let mut zstd = zstd::block::Decompressor::new();
    let patch = zstd.decompress(&patch, patch_size).unwrap();

    let patch = Cursor::new(patch);
    let source = Cursor::new(source);

    let mut patched_reader = bipatch::Reader::new(patch, source).unwrap();

    let mut patched_writer: Vec<u8> = vec![];

    copy(&mut patched_reader, &mut patched_writer).unwrap(); 

    patched_writer
}
