#[cfg(feature = "base58_support")]
pub mod base58;

#[cfg(feature = "bincode_support")]
pub mod bincode;

#[cfg(feature = "brotli_support")]
pub mod brotli;

#[cfg(feature = "deflate_support")]
pub mod deflate;

#[cfg(feature = "gzip_support")]
pub mod gzip;

#[cfg(feature = "lz4_support")]
pub mod lz4;

#[cfg(feature = "lzma_support")]
pub mod lzma;

#[cfg(feature = "lzma2_support")]
pub mod lzma2;

#[cfg(feature = "zlib_support")]
pub mod zlib;

#[cfg(feature = "zstd_support")]
pub mod zstd;
