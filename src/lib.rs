mod carchive;
mod prelude;
mod error;
pub mod reader;
pub mod writer;

use std::fs::Metadata as FSMeta;
use std::os::unix::fs::MetadataExt;

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
    owner: __uid_t,
    group: __gid_t,
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
    pub fn filepath(&self)-> &str{
        &self.filepath
    }

    pub fn size(&self) -> i64{
        self.size
    }

    pub fn nodetype(&self) -> u32{
        self.nodetype
    }

    pub fn perm(&self) -> mode_t {
        self.perm
    }

    pub fn ctime(&self) -> i64{
        self.ctime
    }

    pub fn ctime_nano(&self) -> i64{
        self.ctime_nano
    }

    pub fn atime(&self) -> i64{
        self.atime
    }

    pub fn atime_nano(&self) -> i64{
        self.atime_nano
    }

    pub fn mtime(&self) -> i64{
        self.mtime
    }

    pub fn mtime_nano(&self) -> i64{
        self.mtime_nano
    }

    pub fn owner(&self) -> __uid_t {
        self.owner
    }

    pub fn group(&self) -> __gid_t {
        self.group
    }
}

use carchive::{__gid_t, __uid_t, mode_t};

pub use error::Error;

// these are definition vars needed
// when the raw libarchive is used
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
