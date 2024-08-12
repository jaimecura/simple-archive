use std::fs::File;

use simple_archive::reader::ArchiveReader;

#[test]
fn seek_to_obj() {
    let source = File::open("tests/fixtures/single_file.tar.bz2").unwrap();
    let mut a = ArchiveReader::new(source).unwrap();
    a.reader_seek_obj("test2.txt").unwrap();
    a.reader_seek_obj("random.txt").unwrap();
}

