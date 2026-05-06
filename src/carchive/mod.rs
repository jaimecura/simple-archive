mod generated;

use std::ffi::CStr;

pub use generated::*;

use crate::Metadata;

fn entry_pathname(entry: *mut archive_entry) -> String {
    let pathname = unsafe { archive_entry_pathname(entry) };
    if pathname.is_null() {
        return "".to_owned();
    }

    String::from_utf8_lossy(unsafe { CStr::from_ptr(pathname) }.to_bytes()).into()
}

fn entry_ctime(entry: *mut archive_entry) -> i64 {
    unsafe { archive_entry_ctime(entry) }
}

fn entry_ctime_nano(entry: *mut archive_entry) -> i64 {
    unsafe { archive_entry_ctime_nsec(entry) }
}

fn entry_atime(entry: *mut archive_entry) -> i64 {
    unsafe { archive_entry_atime(entry) }
}

fn entry_atime_nano(entry: *mut archive_entry) -> i64 {
    unsafe { archive_entry_atime_nsec(entry) }
}

fn entry_mtime(entry: *mut archive_entry) -> i64 {
    unsafe { archive_entry_mtime(entry) }
}

fn entry_mtime_nano(entry: *mut archive_entry) -> i64 {
    unsafe { archive_entry_mtime_nsec(entry) }
}

fn entry_owner(entry: *mut archive_entry) -> libc::uid_t {
    unsafe { archive_entry_uid(entry) }
}

fn entry_group(entry: *mut archive_entry) -> libc::gid_t {
    unsafe { archive_entry_gid(entry) }
}

pub fn entry_nodetype(entry: *mut archive_entry) -> u32 {
    unsafe { archive_entry_filetype(entry) }
}

pub fn entry_header(archive: *mut archive) -> crate::prelude::Result<*mut archive_entry> {
    unsafe {
        let mut entry = std::mem::MaybeUninit::<*mut archive_entry>::uninit();
        match archive_read_next_header(archive, entry.as_mut_ptr()) {
            ARCHIVE_EOF => Err(crate::Error::Unknown("EOF".to_owned())),
            ARCHIVE_OK | ARCHIVE_WARN => {
                Ok(entry.assume_init())
            }
            _ => Err(crate::Error::from(archive)),
        }
    }
}

pub fn extract(archive: *mut archive, entry: *mut archive_entry, flags: i32) -> crate::prelude::Result<()> {
    unsafe {
        match archive_read_extract(archive, entry, flags) {
            ARCHIVE_OK | ARCHIVE_WARN => Ok(()),
            _ => Err(crate::Error::from(archive)),
        }
    }
}

fn entry_size(entry: *mut archive_entry) -> i64 {
    unsafe { archive_entry_size(entry) }
}

fn entry_perm(entry: *mut archive_entry) -> mode_t {
    unsafe { archive_entry_perm(entry) }
}

impl From<*mut archive_entry> for Metadata {
    fn from(input: *mut archive_entry) -> Self {
        Metadata {
            filepath: entry_pathname(input),
            size: entry_size(input),
            nodetype: entry_nodetype(input),
            perm: entry_perm(input),
            ctime: entry_ctime(input),
            ctime_nano: entry_ctime_nano(input),
            atime: entry_atime(input),
            atime_nano: entry_atime_nano(input),
            mtime: entry_mtime(input),
            mtime_nano: entry_mtime_nano(input),
            owner: entry_owner(input),
            group: entry_group(input),
        }
    }
}
