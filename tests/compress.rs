use std::fs::File;

use simple_archive::writer::ArchiveWriter;

#[test]
fn compress_archive_7z() {
    let dest = File::create("tests/fixtures_out/compressed.7z").unwrap();
    let mut a = ArchiveWriter::new(dest).unwrap();
    a.set_output_7zlzma2().unwrap();
    a.set_compression_high().unwrap();
    a.open().unwrap();
    a.add_file("tests/fixtures/random.txt", "E/output.xz")
        .unwrap();
}

#[test]
fn compress_archive_zip() {
    let dest = File::create("tests/fixtures_out/compressed.zip").unwrap();
    let mut a = ArchiveWriter::new(dest).unwrap();
    a.set_output_zip().unwrap();
    a.set_compression_high().unwrap();
    a.open().unwrap();
    a.add_file("tests/fixtures/random.txt", "E/output.xz")
        .unwrap();
}

#[test]
fn compress_archive_targz() {
    let dest = File::create("tests/fixtures_out/compressed.tar.gz").unwrap();
    let mut a = ArchiveWriter::new(dest).unwrap();
    a.set_output_targz().unwrap();
    a.set_compression_high().unwrap();
    a.open().unwrap();
    a.add_file("tests/fixtures/random.txt", "E/output.xz")
        .unwrap();
}

#[test]
fn compress_archive_tarxz() {
    let dest = File::create("tests/fixtures_out/compressed.tar.xz").unwrap();
    let mut a = ArchiveWriter::new(dest).unwrap();
    a.set_output_tarxz().unwrap();
    a.set_compression_high().unwrap();
    a.open().unwrap();
    a.add_file("tests/fixtures/random.txt", "E/output.xz")
        .unwrap();
}

#[test]
fn compress_archive_tarzst() {
    let dest = File::create("tests/fixtures_out/compressed.tar.zst").unwrap();
    let mut a = ArchiveWriter::new(dest).unwrap();
    a.set_output_tarzst().unwrap();
    a.set_compression_high().unwrap();
    a.open().unwrap();
    a.add_file("tests/fixtures/random.txt", "E/output.xz")
        .unwrap();
}
