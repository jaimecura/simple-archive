use crate::{prelude::*, Metadata};

use std::{
    ffi::CString,
    fs::File,
    io::{Read, Write},
    ptr::null_mut,
};

use libc::c_void;

use crate::{
    carchive::{
        self, archive, archive_entry_free, archive_entry_new, archive_entry_set_atime,
        archive_entry_set_ctime, archive_entry_set_gid, archive_entry_set_mode,
        archive_entry_set_mtime, archive_entry_set_pathname, archive_entry_set_perm,
        archive_entry_set_size, archive_entry_set_uid, archive_write_data, archive_write_free,
        archive_write_header,
    },
    ARCHIVE_FILTER_BZIP2, ARCHIVE_FILTER_GZIP, ARCHIVE_FILTER_LRZIP, ARCHIVE_FILTER_LZ4,
    ARCHIVE_FILTER_LZIP, ARCHIVE_FILTER_LZMA, ARCHIVE_FILTER_LZOP, ARCHIVE_FILTER_NONE,
    ARCHIVE_FILTER_XZ, ARCHIVE_FILTER_ZSTD, ARCHIVE_FORMAT_7ZIP, ARCHIVE_FORMAT_TAR,
    ARCHIVE_FORMAT_XAR, ARCHIVE_FORMAT_ZIP,
};
use std::os::raw::c_int;

const BUFFER_SIZE: usize = 16384;

pub struct ArchiveWriter<W: Write> {
    archive_writer: *mut archive,
    fileref: Box<FileWriter<W>>,
    file_format: c_int,
    file_filter: c_int,
}

struct FileWriter<W: Write> {
    obj: W,
}

unsafe extern "C" fn archivewriter_write<W: Write>(
    archive: *mut carchive::archive,
    client_data: *mut c_void,
    buffer: *const c_void,
    size: usize,
) -> carchive::la_ssize_t {
    let writer = (client_data as *mut FileWriter<W>).as_mut().unwrap();
    let writable = std::slice::from_raw_parts(buffer as *const u8, size);

    match writer.obj.write(writable) {
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

impl<W: Write> ArchiveWriter<W> {
    pub fn new(dest: W) -> Result<ArchiveWriter<W>>
    where
        W: Write,
    {
        let fref = Box::new(FileWriter { obj: dest });
        unsafe {
            let archive_writer = carchive::archive_write_new();

            if archive_writer.is_null() {
                return Err(Error::NullArchive);
            }

            match carchive::archive_write_set_bytes_in_last_block(archive_writer, 1) {
                carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
                _ => return Err(Error::from(archive_writer)),
            };

            Ok(ArchiveWriter {
                archive_writer,
                fileref: fref,
                file_format: -1,
                file_filter: -1,
            })
        }
    }

    // Raw Archive API
    pub fn set_output_format(&mut self, format: c_int) -> Result<()> {
        match unsafe { carchive::archive_write_set_format(self.archive_writer, format) } {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(self.archive_writer)),
        }

        self.file_format = format;
        Ok(())
    }

    pub fn set_output_filter(&mut self, filter: c_int) -> Result<()> {
        match unsafe { carchive::archive_write_add_filter(self.archive_writer, filter) } {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(self.archive_writer)),
        }
        self.file_filter = filter;
        Ok(())
    }

    pub fn add_filter_option(&mut self, name: &str, value: &str) -> Result<()> {
        let n = CString::new(name.to_string()).unwrap();
        let v = CString::new(value.to_string()).unwrap();
        match unsafe {
            carchive::archive_write_set_filter_option(
                self.archive_writer,
                null_mut(),
                n.as_ptr(),
                v.as_ptr(),
            )
        } {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(self.archive_writer)),
        }
        Ok(())
    }

    pub fn add_format_option(&mut self, name: &str, value: &str) -> Result<()> {
        let n = CString::new(name.to_string()).unwrap();
        let v = CString::new(value.to_string()).unwrap();
        match unsafe {
            carchive::archive_write_set_format_option(
                self.archive_writer,
                null_mut(),
                n.as_ptr(),
                v.as_ptr(),
            )
        } {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(self.archive_writer)),
        }
        Ok(())
    }

    pub fn open(&mut self) -> Result<()> {
        if self.file_format < 0 || self.file_filter < 0 {
            return Err(Error::IncompleteInitialization);
        }
        match unsafe {
            carchive::archive_write_open(
                self.archive_writer,
                std::ptr::addr_of_mut!(*self.fileref) as *mut c_void,
                None,
                Some(archivewriter_write::<W>),
                None,
            )
        } {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
            _ => return Err(Error::from(self.archive_writer)),
        }
        Ok(())
    }

    // this free is not meant to called directly. Only by borrow system
    fn free(&mut self) -> Result<()> {
        match unsafe { archive_write_free(self.archive_writer) } {
            carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => Ok(()),
            _ => Err(Error::from(self.archive_writer)),
        }
    }

    // this is only for output write filter.
    pub fn set_compression_high(&mut self) -> Result<()> {
        let mut max_compression_level = match self.file_filter {
            ARCHIVE_FILTER_BZIP2 => 9,
            ARCHIVE_FILTER_GZIP => 9,
            ARCHIVE_FILTER_LRZIP => 9,
            ARCHIVE_FILTER_XZ => 9,
            ARCHIVE_FILTER_LZ4 => 9,
            ARCHIVE_FILTER_LZIP => 9,
            ARCHIVE_FILTER_ZSTD => 22,
            ARCHIVE_FILTER_LZMA => 9,
            ARCHIVE_FILTER_LZOP => 9,
            _ => -1,
        };

        if max_compression_level > 0 {
            self.add_filter_option("compression-level", &max_compression_level.to_string())?;
        }

        max_compression_level = match self.file_format {
            ARCHIVE_FORMAT_7ZIP => 9,
            ARCHIVE_FORMAT_XAR => 9,
            ARCHIVE_FORMAT_ZIP => 9,
            _ => -1,
        };

        if max_compression_level > 0 {
            self.add_format_option("compression-level", &max_compression_level.to_string())?;
        }

        Ok(())
    }

    pub fn set_compression_mid(&mut self) -> Result<()> {
        let mut max_compression_level = match self.file_filter {
            ARCHIVE_FILTER_BZIP2 => 9,
            ARCHIVE_FILTER_GZIP => 9,
            ARCHIVE_FILTER_LRZIP => 9,
            ARCHIVE_FILTER_XZ => 9,
            ARCHIVE_FILTER_LZ4 => 9,
            ARCHIVE_FILTER_LZIP => 9,
            ARCHIVE_FILTER_ZSTD => 22,
            ARCHIVE_FILTER_LZMA => 9,
            ARCHIVE_FILTER_LZOP => 9,
            _ => -1,
        };

        if max_compression_level > 0 {
            self.add_filter_option(
                "compression-level",
                &(max_compression_level / 2).to_string(),
            )?;
        }

        max_compression_level = match self.file_format {
            ARCHIVE_FORMAT_7ZIP => 9,
            ARCHIVE_FORMAT_XAR => 9,
            ARCHIVE_FORMAT_ZIP => 9,
            _ => -1,
        };

        if max_compression_level > 0 {
            self.add_format_option(
                "compression-level",
                &(max_compression_level / 2).to_string(),
            )?;
        }

        Ok(())
    }

    pub fn set_compression_low(&mut self) -> Result<()> {
        self.add_format_option("compression-level", "0")?;
        self.add_filter_option("compression-level", "0")
    }

    // Simple Rust API. Nothing else but call new and then set format and add objects
    // it cannot be simpler
    pub fn set_output_targz(&mut self) -> Result<()> {
        self.set_output_format(ARCHIVE_FORMAT_TAR)?;
        self.set_output_filter(ARCHIVE_FILTER_GZIP)
    }

    pub fn set_output_tarxz(&mut self) -> Result<()> {
        self.set_output_format(ARCHIVE_FORMAT_TAR)?;
        self.set_output_filter(ARCHIVE_FILTER_XZ)
    }

    pub fn set_output_tarzst(&mut self) -> Result<()> {
        self.set_output_format(ARCHIVE_FORMAT_TAR)?;
        self.set_output_filter(ARCHIVE_FILTER_ZSTD)
    }

    pub fn set_output_7zlzma2(&mut self) -> Result<()> {
        self.set_output_format(ARCHIVE_FORMAT_7ZIP)?;
        self.set_output_filter(ARCHIVE_FILTER_NONE)?;
        self.add_format_option("compression", "lzma2")
    }

    pub fn set_output_zip(&mut self) -> Result<()> {
        self.set_output_format(ARCHIVE_FORMAT_ZIP)?;
        self.set_output_filter(ARCHIVE_FILTER_NONE)?;
        self.add_format_option("compression", "deflate")
    }

    pub fn add_obj_from_reader<S: Read>(
        &mut self,
        mut source: S,
        archivepath: &str,
        objmeta: &Metadata,
    ) -> Result<()> {
        let mut buffer: [u8; BUFFER_SIZE] = [0u8; BUFFER_SIZE];
        let p = CString::new(archivepath.to_string()).unwrap();

        unsafe {
            let entry = archive_entry_new();

            archive_entry_set_size(entry, objmeta.size()); // quick way to get the size?
                                                         // archive_entry_set_perm(entry, 0o777);
                                                         // archive_entry_set_filetype(entry, AE_IFREG);
            archive_entry_set_mode(entry, objmeta.nodetype() | objmeta.perm());
            archive_entry_set_perm(entry, objmeta.perm());
            archive_entry_set_ctime(entry, objmeta.ctime(), objmeta.ctime_nano());
            archive_entry_set_mtime(entry, objmeta.mtime(), objmeta.mtime_nano());
            archive_entry_set_atime(entry, objmeta.atime(), objmeta.atime_nano());
            archive_entry_set_pathname(entry, p.as_ptr());
            archive_entry_set_uid(entry, objmeta.owner());
            archive_entry_set_gid(entry, objmeta.group());

            match archive_write_header(self.archive_writer, entry) {
                carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => (),
                _ => return Err(Error::from(self.archive_writer)),
            }

            loop {
                let readed = source.read(&mut buffer)?;
                if readed == 0 {
                    break;
                }

                if archive_write_data(
                    self.archive_writer,
                    buffer.as_ptr() as *const c_void,
                    readed,
                ) != readed as isize
                {
                    return Err(Error::from(self.archive_writer));
                }
            }

            //write file
            archive_entry_free(entry);
        }
        Ok(())
    }

    pub fn add_file(&mut self, localpath: &str, archivepath: &str) -> Result<()> {
        let source = File::open(localpath)?;
        let meta = source.metadata()?;
        self.add_obj_from_reader(source, archivepath, &meta.into())
    }
}

impl<W: Write> Drop for ArchiveWriter<W> {
    fn drop(&mut self) {
        drop(self.free());
    }
}
