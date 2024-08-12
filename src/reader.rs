use crate::{
    carchive::{self, archive_entry, archive_read_free},
    prelude::*,
    Metadata,
};

use libc::{c_int, c_void};

use crate::carchive::archive;
use std::{
    ffi::CString,
    io::{Error as IOError, ErrorKind, Read, Seek, SeekFrom},
};

const BUFFER_SIZE: usize = 16384;

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

    // match pipe.reader.read(pipe.buffer) {
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

    pub fn reader_whole_archive(self) -> Result<()> {
        Ok(())
    }

    pub fn reader_seek_obj(&mut self, filename: &str) -> Result<()> {
        let archive = self.get_archive()?;

        unsafe {
            self.fileref.obj.seek(SeekFrom::Start(0))?;
            match  archive_read_free(archive)  {
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
        match read_size {
            n if n >= 0 || n as usize <= buf.len() => Ok(n as usize),
            _ => Err(Error::from(archive).into()),
        }
    }
}

impl<R: Read + Seek> Drop for ArchiveReader<R> {
    fn drop(&mut self) {
        drop(self.free());
    }
}
