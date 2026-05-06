use crate::{
    carchive::{self, archive_entry, archive_read_free},
    prelude::*,
};

use libc::{c_int, c_void};

use crate::carchive::archive;
use std::{
    ffi::CString,
    io::{Error as IOError, ErrorKind, Read, Seek, SeekFrom},
    mem::MaybeUninit,
};

const BUFFER_SIZE: usize = 16384;

/// A reader for compressed archives.
///
/// `ArchiveReader` allows reading from any source that implements `Read + Seek`.
/// It provides an iterator over the entries in the archive and implements `Read`
/// to extract the content of the current entry.
pub struct ArchiveReader<R: Read + Seek> {
    archive_reader: Option<*mut archive>,
    #[allow(dead_code)]
    fileref: Box<SourceReader<R>>,
    current_entry: Option<Metadata>,
}

struct SourceReader<R: Read + Seek> {
    obj: R,
    buffer: Box<[u8]>,
}

unsafe extern "C" fn archivereader_read<R: Read + Seek>(
    archive: *mut carchive::archive,
    client_data: *mut c_void,
    buffer: *mut *const c_void,
) -> carchive::la_ssize_t {
    let reader = (client_data as *mut SourceReader<R>).as_mut().unwrap();
    *buffer = reader.buffer.as_ptr() as *const c_void;

    match reader.obj.read(reader.buffer.as_mut()) {
        Ok(size) => size as carchive::la_ssize_t,
        Err(e) => {
            let description = CString::new(e.to_string()).unwrap();

            carchive::archive_set_error(
                archive,
                e.raw_os_error().unwrap_or(0),
                description.as_ptr(),
            );

            -1
        }
    }
}

unsafe extern "C" fn archivereader_seek<R: Read + Seek>(
    _: *mut carchive::archive,
    client_data: *mut c_void,
    offset: carchive::la_int64_t,
    whence: c_int,
) -> i64 {
    let seeker = (client_data as *mut SourceReader<R>).as_mut().unwrap();
    let whence = match whence {
        0 => SeekFrom::Start(offset as u64),
        1 => SeekFrom::Current(offset),
        2 => SeekFrom::End(offset),
        _ => return -1,
    };

    match seeker.obj.seek(whence) {
        Ok(offset) => offset as i64,
        Err(_) => -1,
    }
}

impl<R: Read + Seek> ArchiveReader<R> {
    /// Creates a new `ArchiveReader` from a source that implements `Read + Seek`.
    pub fn new(source: R) -> Result<Self> {
        let buffer = [0; BUFFER_SIZE];
        let mut fref = Box::new(SourceReader {
            obj: source,
            buffer: Box::new(buffer),
        });

        unsafe {
            Ok(ArchiveReader {
                archive_reader: Some(ArchiveReader::start(&mut fref)?),
                fileref: fref,
                current_entry: Option::None,
            })
        }
    }

    unsafe fn start(fref: &mut Box<SourceReader<R>>) -> Result<*mut archive> {
        let archive_reader = carchive::archive_read_new();

        if archive_reader.is_null() {
            return Err(Error::NullArchive);
        }

        match carchive::archive_read_support_filter_all(archive_reader) {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(archive_reader)),
        };

        match carchive::archive_read_support_format_all(archive_reader) {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(archive_reader)),
        };

        match carchive::archive_read_set_seek_callback(
            archive_reader,
            Some(archivereader_seek::<R>),
        ) {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(archive_reader)),
        }

        match carchive::archive_read_open(
            archive_reader,
            std::ptr::addr_of_mut!(**fref) as *mut c_void,
            None,
            Some(archivereader_read::<R>),
            None,
        ) {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(archive_reader)),
        };

        Ok(archive_reader)
    }

    /// Returns a list of all file metadata in the archive.
    ///
    /// Note: This will consume the reader as it iterates through the whole archive.
    pub fn list_files(self) -> Result<Vec<Metadata>> {
        let archive = self.get_archive()?;

        let mut outlist = Vec::<Metadata>::new();

        loop {
            unsafe {
                let mut entry = std::mem::MaybeUninit::<*mut archive_entry>::uninit();
                match carchive::archive_read_next_header(archive, entry.as_mut_ptr()) {
                    carchive::ARCHIVE_EOF => break,
                    carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => {
                        let entry = entry.assume_init();
                        outlist.push(entry.into());
                    }
                    _ => return Err(Error::from(archive)),
                };
            }
        }

        Ok(outlist)
    }

    /// Returns the metadata of the current entry being read.
    pub fn current_metadata(&self) -> Option<&Metadata> {
        self.current_entry.as_ref()
    }

    /// Extracts the whole archive to the specified destination directory.
    ///
    /// The `flags` parameter controls extraction behavior (e.g., `ARCHIVE_EXTRACT_TIME`, `ARCHIVE_EXTRACT_PERM`).
    pub fn reader_whole_archive(self, dest_path: &str, flags: i32) -> Result<()> {
        let archive = self.get_archive()?;
        let current_dir = std::env::current_dir()?;
        std::fs::create_dir_all(dest_path)?;
        std::env::set_current_dir(dest_path)?;

        loop {
            unsafe {
                let mut entry = std::mem::MaybeUninit::<*mut archive_entry>::uninit();
                match carchive::archive_read_next_header(archive, entry.as_mut_ptr()) {
                    carchive::ARCHIVE_EOF => break,
                    carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => {
                        let entry_ptr = entry.assume_init();
                        match carchive::archive_read_extract(archive, entry_ptr, flags) {
                            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
                            _ => {
                                let _ = std::env::set_current_dir(current_dir);
                                return Err(Error::from(archive));
                            }
                        }
                    }
                    _ => {
                        let _ = std::env::set_current_dir(current_dir);
                        return Err(Error::from(archive));
                    }
                };
            }
        }

        std::env::set_current_dir(current_dir)?;
        Ok(())
    }

    /// Seeks to a specific file within the archive by its name.
    ///
    /// This will reset the internal libarchive state and read from the beginning
    /// until the specified file is found.
    pub fn reader_seek_obj(&mut self, filename: &str) -> Result<()> {
        let archive = self.get_archive()?;

        unsafe {
            self.fileref.obj.seek(SeekFrom::Start(0))?;
            match archive_read_free(archive) {
                carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
                _ => return Err(archive.into()),
            };

            let archive = ArchiveReader::start(&mut self.fileref)?;
            self.archive_reader = Some(archive);

            loop {
                let mut entry = std::mem::MaybeUninit::<*mut archive_entry>::uninit();
                let hdr_result = carchive::archive_read_next_header(archive, entry.as_mut_ptr());
                let entry = entry.assume_init();
                match hdr_result {
                    carchive::ARCHIVE_EOF => {
                        return Err(IOError::new(
                            ErrorKind::NotFound,
                            format!("path {} doesn't exist inside archive", filename),
                        )
                        .into());
                    }
                    carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
                    _ => return Err(archive.into()),
                };

                let meta: Metadata = entry.into();
                if meta.filepath() == filename {
                    self.current_entry = Some(meta);
                    return Ok(());
                }
            }
        }
    }

    fn get_archive(&self) -> Result<*mut archive> {
        if let Some(a) = self.archive_reader {
            Ok(a)
        } else {
            Err(Error::NullArchive)
        }
    }

    // this free is not meant to called directly. Only by borrow system
    fn free(&mut self) -> Result<()> {
        let archive = self.get_archive()?;
        match unsafe { archive_read_free(archive) } {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => Ok(()),
            _ => Err(Error::from(archive)),
        }
    }
}

impl<R: Read + Seek> Read for ArchiveReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let archive = self.get_archive()?;
        let read_size = unsafe {
            carchive::archive_read_data(archive, buf.as_mut_ptr() as *mut c_void, buf.len())
        };

        if read_size >= 0 {
            Ok(read_size as usize)
        } else {
            Err(Error::from(archive).into())
        }
    }
}

impl<R: Read + Seek> Iterator for ArchiveReader<R> {
    type Item = Metadata;

    fn next(&mut self) -> Option<Self::Item> {
        let archive;
        let mut entry = MaybeUninit::<*mut archive_entry>::uninit();

        if let Ok(res) = self.get_archive() {
            archive = res;
        } else {
            return Option::None;
        }

        unsafe {
            match carchive::archive_read_next_header(archive, entry.as_mut_ptr()) {
                carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => {
                    let entry_ptr = entry.assume_init();
                    let meta: Metadata = entry_ptr.into();
                    self.current_entry = Some(meta.clone());
                    Some(meta)
                }
                _ => {
                    self.current_entry = None;
                    Option::None
                }
            }
        }
    }
}

impl<R: Read + Seek> Drop for ArchiveReader<R> {
    fn drop(&mut self) {
        drop(self.free());
    }
}

impl Drop for archive_entry {
    fn drop(&mut self) {
        unsafe { carchive::archive_entry_free(self) };
    }
}
