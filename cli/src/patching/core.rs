use pipe::pipe;
use std::io::copy;
use std::io::Cursor;
use std::io::Read;
use std::thread;

pub fn diff(source: Vec<u8>, target: Vec<u8>) -> (Vec<u8>, usize) {

    if source.len() == target.len() {
        println!("MAYBE SAME");
        for i in 0..source.len() {
            if source[i] != target[i] {
                println!("NOT SAME");
                break
            }
        }
    }

    // Calculate patch
    let (mut patch_reader, mut patch_writer) = pipe();
    thread::spawn(move || {
        bidiff::simple_diff(&source[..], &target[..], &mut patch_writer).unwrap();
    });

    let mut patch: Vec<u8> = Vec::new();
    patch_reader.read_to_end(&mut patch).unwrap();
    let patch_size = patch.len();

    // Compress patch
    let com_patch = zstd::stream::encode_all(patch.as_slice(), 0).unwrap();

    println!("raw={}, com={}", patch_size, com_patch.len());

    (com_patch, patch_size)
}

pub fn patch(source: Vec<u8>, patch: &Vec<u8>, patch_size: usize) -> Vec<u8> {
    // Create a new zstd decompressor
    let mut zstd = zstd::block::Decompressor::new();

    // Decompress patch
    let patch = zstd.decompress(&patch, patch_size).unwrap();

    let patch = Cursor::new(patch);
    let source = Cursor::new(source);

    let mut patched_reader = bipatch::Reader::new(patch, source).unwrap();
    let mut patched_writer: Vec<u8> = vec![];

    copy(&mut patched_reader, &mut patched_writer).unwrap();

    patched_writer
}
