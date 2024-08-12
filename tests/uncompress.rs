use core::panic;
use std::{fs::File, io::Read};

use simple_archive::reader::ArchiveReader;

#[test]
fn seek_to_obj() {
    let source = File::open("tests/fixtures/single_file.tar.bz2").unwrap();
    let mut a = ArchiveReader::new(source).unwrap();
    a.reader_seek_obj("test2.txt").unwrap();
    a.reader_seek_obj("random.txt").unwrap();
}

#[test]
fn read_obj() {
    let source = File::open("tests/fixtures/single_file.tar.bz2").unwrap();
    let mut a = ArchiveReader::new(source).unwrap();
    a.reader_seek_obj("test2.txt").unwrap();

    let mut buf = [0u8; 1024];
    let amount = a.read(&mut buf).unwrap();
    let hash = sha256::digest(&buf[..amount]);
    let disk_data = std::fs::read("tests/fixtures/test2.txt").unwrap();
    let hash2 = sha256::digest(disk_data);
    if hash != hash2 {
        panic!("wrong uncompressed data")
    }
}
