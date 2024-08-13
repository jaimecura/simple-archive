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
    a.reader_seek_obj("random.txt").unwrap();

    let mut buf = [0u8; 40960];
    let amount = a.read(&mut buf).unwrap();
    let hash = sha256::digest(&buf[..amount]);
    let disk_data = std::fs::read("tests/fixtures/random.txt").unwrap();
    let hash2 = sha256::digest(disk_data);
    if hash != hash2 {
        panic!("wrong uncompressed data")
    }
}

#[test]
fn iter_obj() {
    let source = File::open("tests/fixtures/single_file.tar.bz2").unwrap();
    let mut a = ArchiveReader::new(source).unwrap();
    let r = a.next();
    if r.is_none() {
        panic!("no file was returned");
    }
    let r = r.unwrap();

    let mut v = vec![];
    a.read_to_end(&mut v).unwrap();
    let hash = sha256::digest(v);
    let name = r.filepath();
    let disk_data = std::fs::read(format!("tests/fixtures/{name}")).unwrap();
    let hash2 = sha256::digest(disk_data);
    println!("F: tests/fixtures/{name}");
    if hash != hash2 {
        panic!("wrong uncompressed data")
    }
}
