mod carchive;
mod error;
mod prelude;
pub mod writer;

// these are definition vars needed 
// when the raw libarchive is used
pub use carchive::AE_IFMT;
pub use carchive::AE_IFREG;
pub use carchive::AE_IFLNK;
pub use carchive::AE_IFSOCK;
pub use carchive::AE_IFCHR;
pub use carchive::AE_IFBLK;
pub use carchive::AE_IFDIR;
pub use carchive::AE_IFIFO;

pub use carchive::ARCHIVE_OK;
pub use carchive::ARCHIVE_WARN;
pub use carchive::ARCHIVE_FATAL;
pub use carchive::ARCHIVE_FAILED;


pub use  carchive::ARCHIVE_FORMAT_CPIO;
pub use  carchive::ARCHIVE_FORMAT_CPIO_POSIX;
pub use  carchive::ARCHIVE_FORMAT_CPIO_BIN_LE;
pub use  carchive::ARCHIVE_FORMAT_CPIO_SVR4_NOCRC;
pub use  carchive::ARCHIVE_FORMAT_CPIO_PWB;

pub use  carchive::ARCHIVE_FORMAT_SHAR;
pub use  carchive::ARCHIVE_FORMAT_SHAR_DUMP;

pub use  carchive::ARCHIVE_FORMAT_TAR;
pub use  carchive::ARCHIVE_FORMAT_TAR_USTAR;
pub use  carchive::ARCHIVE_FORMAT_TAR_PAX_INTERCHANGE;
pub use  carchive::ARCHIVE_FORMAT_TAR_PAX_RESTRICTED;
pub use  carchive::ARCHIVE_FORMAT_TAR_GNUTAR;

pub use  carchive::ARCHIVE_FORMAT_ISO9660;
pub use  carchive::ARCHIVE_FORMAT_ZIP;
pub use  carchive::ARCHIVE_FORMAT_EMPTY;

pub use  carchive::ARCHIVE_FORMAT_MTREE;
pub use  carchive::ARCHIVE_FORMAT_RAW;
pub use  carchive::ARCHIVE_FORMAT_XAR;
pub use  carchive::ARCHIVE_FORMAT_CAB;
pub use  carchive::ARCHIVE_FORMAT_RAR;
pub use  carchive::ARCHIVE_FORMAT_7ZIP;
pub use  carchive::ARCHIVE_FORMAT_WARC;

pub use  carchive::ARCHIVE_FILTER_NONE;
pub use  carchive::ARCHIVE_FILTER_GZIP;
pub use  carchive::ARCHIVE_FILTER_BZIP2;
pub use  carchive::ARCHIVE_FILTER_COMPRESS;
pub use  carchive::ARCHIVE_FILTER_PROGRAM;
pub use  carchive::ARCHIVE_FILTER_LZMA;
pub use  carchive::ARCHIVE_FILTER_XZ;
pub use  carchive::ARCHIVE_FILTER_UU;
pub use  carchive::ARCHIVE_FILTER_RPM;
pub use  carchive::ARCHIVE_FILTER_LZIP;
pub use  carchive::ARCHIVE_FILTER_LRZIP;
pub use  carchive::ARCHIVE_FILTER_LZOP;
pub use  carchive::ARCHIVE_FILTER_GRZIP;
pub use  carchive::ARCHIVE_FILTER_LZ4;
pub use  carchive::ARCHIVE_FILTER_ZSTD;
