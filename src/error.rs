use crate::carchive;
use crate::prelude::*;
use std::{ffi::CStr, io, str::Utf8Error};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Extraction error: '{0}'")]
    Extraction(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Error converting to UTF8")]
    Utf8(#[from] Utf8Error),

    #[error("Unknown error: {0}")]
    Unknown(String),

    #[error("Archive has been allocated but no filter nor format has been defined")]
    IncompleteInitialization,

    #[error("Unknown filter")]
    UnknownFilter,

    #[error("Unknown format")]
    UnknownFormat,

    #[error("Error to create the archive struct, is null")]
    NullArchive,
}

// this part is almost identical to
pub(crate) fn archive_result(value: i32, archive: *mut carchive::archive) -> Result<()> {
    match value {
        carchive::ARCHIVE_OK | carchive::ARCHIVE_WARN => Ok(()),
        _ => Err(Error::from(archive)),
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
impl From<*mut carchive::archive> for Error {
    fn from(input: *mut carchive::archive) -> Self {
        let mut errno;
        let mut error_string;
        unsafe {
            error_string =  CStr::from_ptr(carchive::archive_error_string(input));
        }

        if !error_string.is_null() {
            return Error::Extraction(error_string.to_string_lossy().to_string());
        }

        unsafe{
        errno = carchive::archive_errno(input);
        }
        if errno != 0 {
            return io::Error::from_raw_os_error(errno).into();
        }

        Error::Unknown("Cannot read error string from archive".to_owned())
    }
}
