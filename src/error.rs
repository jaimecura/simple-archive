use crate::carchive;
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

impl From<*mut carchive::archive> for Error {
    fn from(input: *mut carchive::archive) -> Self {
        let errno;
        unsafe {
            let error_string = carchive::archive_error_string(input);
            if !error_string.is_null() {
                return Error::Extraction(
                    CStr::from_ptr(error_string).to_string_lossy().to_string(),
                );
            }

            errno = carchive::archive_errno(input);
        }

        if errno != 0 {
            return io::Error::from_raw_os_error(errno).into();
        }

        Error::Unknown("Cannot read error string from archive".to_owned())
    }
}

impl From<Error> for io::Error {
    fn from(value: Error) -> Self {
        io::Error::new(io::ErrorKind::Other, value)
    }
}
