mod carchive;
mod error;
mod prelude;
pub mod reader;
pub mod writer;

use std::fs::Metadata as FSMeta;
use std::os::unix::fs::MetadataExt;

/// Metadata information for an archive entry.
#[derive(Clone)]
pub struct Metadata {
    filepath: String,
    size: i64,
    nodetype: u32,
    perm: mode_t,
    ctime: i64,
    ctime_nano: i64,
    atime: i64,
    atime_nano: i64,
    mtime: i64,
    mtime_nano: i64,
    owner: libc::uid_t,
    group: libc::gid_t,
}

fn into_nodetype(source: &FSMeta) -> u32 {
    if source.is_dir() {
        AE_IFDIR
    } else if source.is_file() {
        AE_IFREG
    } else if source.is_symlink() {
        AE_IFLNK
    } else {
        0
    }
}

impl From<FSMeta> for Metadata {
    fn from(meta: FSMeta) -> Self {
        Metadata {
            filepath: "".to_owned(),
            size: meta.size() as i64, // Iḿ assuming anything above 2**63 is way to much for
            // a single file
            nodetype: into_nodetype(&meta),
            perm: meta.mode(),
            ctime: meta.ctime(),
            ctime_nano: meta.ctime_nsec(),
            atime: meta.atime(),
            atime_nano: meta.atime_nsec(),
            mtime: meta.mtime(),
            mtime_nano: meta.mtime_nsec(),
            owner: meta.uid(),
            group: meta.gid(),
        }
    }
}

impl Metadata {
    /// Create metadata for in-memory entries without filesystem dependency.
    pub fn from_fields(
        size: i64,
        nodetype: u32,
        perm: mode_t,
        mtime: i64,
        mtime_nano: i64,
    ) -> Self {
        Self {
            filepath: String::new(),
            size,
            nodetype,
            perm,
            ctime: 0,
            ctime_nano: 0,
            atime: 0,
            atime_nano: 0,
            mtime,
            mtime_nano,
            owner: 0,
            group: 0,
        }
    }

    /// Returns the filepath of the entry.
    pub fn filepath(&self) -> &str {
        &self.filepath
    }

    /// Returns the size of the entry in bytes.
    pub fn size(&self) -> i64 {
        self.size
    }

    /// Returns the node type (file, directory, etc.)
    pub fn nodetype(&self) -> u32 {
        self.nodetype
    }

    /// Returns the permissions mode.
    pub fn perm(&self) -> mode_t {
        self.perm
    }

    /// Returns the creation time in seconds.
    pub fn ctime(&self) -> i64 {
        self.ctime
    }

    /// Returns the creation time nanoseconds part.
    pub fn ctime_nano(&self) -> i64 {
        self.ctime_nano
    }

    /// Returns the last access time in seconds.
    pub fn atime(&self) -> i64 {
        self.atime
    }

    /// Returns the last access time nanoseconds part.
    pub fn atime_nano(&self) -> i64 {
        self.atime_nano
    }

    /// Returns the last modification time in seconds.
    pub fn mtime(&self) -> i64 {
        self.mtime
    }

    /// Returns the last modification time nanoseconds part.
    pub fn mtime_nano(&self) -> i64 {
        self.mtime_nano
    }

    /// Returns the owner UID.
    pub fn owner(&self) -> libc::uid_t {
        self.owner
    }

    /// Returns the group GID.
    pub fn group(&self) -> libc::gid_t {
        self.group
    }

    /// Returns true if the entry is a directory.
    pub fn is_dir(&self) -> bool {
        self.nodetype == AE_IFDIR
    }

    /// Returns true if the entry is a regular file.
    pub fn is_file(&self) -> bool {
        self.nodetype == AE_IFREG
    }

    /// Returns true if the entry is a symbolic link.
    pub fn is_symlink(&self) -> bool {
        self.nodetype == AE_IFLNK
    }

    /// Returns true if the entry is a block device.
    pub fn is_block_device(&self) -> bool {
        self.nodetype == AE_IFBLK
    }

    /// Returns true if the entry is a character device.
    pub fn is_char_device(&self) -> bool {
        self.nodetype == AE_IFCHR
    }

    /// Returns true if the entry is a FIFO (named pipe).
    pub fn is_fifo(&self) -> bool {
        self.nodetype == AE_IFIFO
    }

    /// Returns true if the entry is a socket.
    pub fn is_socket(&self) -> bool {
        self.nodetype == AE_IFSOCK
    }
}

use carchive::mode_t;

pub use error::Error;

// Re-export common libarchive constants
pub use carchive::AE_IFBLK;
pub use carchive::AE_IFCHR;
pub use carchive::AE_IFDIR;
pub use carchive::AE_IFIFO;
pub use carchive::AE_IFLNK;
pub use carchive::AE_IFMT;
pub use carchive::AE_IFREG;
pub use carchive::AE_IFSOCK;

pub use carchive::ARCHIVE_FAILED;
pub use carchive::ARCHIVE_FATAL;
pub use carchive::ARCHIVE_OK;
pub use carchive::ARCHIVE_WARN;

pub use carchive::ARCHIVE_FORMAT_CPIO;
pub use carchive::ARCHIVE_FORMAT_CPIO_BIN_LE;
pub use carchive::ARCHIVE_FORMAT_CPIO_POSIX;
pub use carchive::ARCHIVE_FORMAT_CPIO_PWB;
pub use carchive::ARCHIVE_FORMAT_CPIO_SVR4_NOCRC;

pub use carchive::ARCHIVE_FORMAT_SHAR;
pub use carchive::ARCHIVE_FORMAT_SHAR_DUMP;

pub use carchive::ARCHIVE_FORMAT_TAR;
pub use carchive::ARCHIVE_FORMAT_TAR_GNUTAR;
pub use carchive::ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE;
pub use carchive::ARCHIVE_FORMAT_TAR_PAX_RESTRICTED;
pub use carchive::ARCHIVE_FORMAT_TAR_USTAR;

pub use carchive::ARCHIVE_FORMAT_EMPTY;
pub use carchive::ARCHIVE_FORMAT_ISO9660;
pub use carchive::ARCHIVE_FORMAT_ZIP;

pub use carchive::ARCHIVE_FORMAT_7ZIP;
pub use carchive::ARCHIVE_FORMAT_CAB;
pub use carchive::ARCHIVE_FORMAT_MTREE;
pub use carchive::ARCHIVE_FORMAT_RAR;
pub use carchive::ARCHIVE_FORMAT_RAW;
pub use carchive::ARCHIVE_FORMAT_WARC;
pub use carchive::ARCHIVE_FORMAT_XAR;

pub use carchive::ARCHIVE_FILTER_BZIP2;
pub use carchive::ARCHIVE_FILTER_COMPRESS;
pub use carchive::ARCHIVE_FILTER_GRZIP;
pub use carchive::ARCHIVE_FILTER_GZIP;
pub use carchive::ARCHIVE_FILTER_LRZIP;
pub use carchive::ARCHIVE_FILTER_LZ4;
pub use carchive::ARCHIVE_FILTER_LZIP;
pub use carchive::ARCHIVE_FILTER_LZMA;
pub use carchive::ARCHIVE_FILTER_LZOP;
pub use carchive::ARCHIVE_FILTER_NONE;
pub use carchive::ARCHIVE_FILTER_PROGRAM;
pub use carchive::ARCHIVE_FILTER_RPM;
pub use carchive::ARCHIVE_FILTER_UU;
pub use carchive::ARCHIVE_FILTER_XZ;
pub use carchive::ARCHIVE_FILTER_ZSTD;

pub use carchive::ARCHIVE_EXTRACT_ACL;
pub use carchive::ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS;
pub use carchive::ARCHIVE_EXTRACT_FFLAGS;
pub use carchive::ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED;
pub use carchive::ARCHIVE_EXTRACT_MAC_METADATA;
pub use carchive::ARCHIVE_EXTRACT_NO_AUTODIR;
pub use carchive::ARCHIVE_EXTRACT_NO_HFS_COMPRESSION;
pub use carchive::ARCHIVE_EXTRACT_NO_OVERWRITE;
pub use carchive::ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER;
pub use carchive::ARCHIVE_EXTRACT_OWNER;
pub use carchive::ARCHIVE_EXTRACT_PERM;
pub use carchive::ARCHIVE_EXTRACT_SAFE_WRITES;
pub use carchive::ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS;
pub use carchive::ARCHIVE_EXTRACT_SECURE_NODOTDOT;
pub use carchive::ARCHIVE_EXTRACT_SECURE_SYMLINKS;
pub use carchive::ARCHIVE_EXTRACT_TIME;
pub use carchive::ARCHIVE_EXTRACT_UNLINK;
pub use carchive::ARCHIVE_EXTRACT_XATTR;
