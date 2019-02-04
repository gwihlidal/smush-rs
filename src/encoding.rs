use crate::error::{Error, Result};
use brotli;
use flate2::read::{DeflateEncoder, GzEncoder, ZlibEncoder};
use flate2::write::{DeflateDecoder, GzDecoder, ZlibDecoder};
use flate2::Compression;
use std::{fmt, io, str};

// http://tools.ietf.org/html/rfc7231#section-5.3.4
// https://tools.ietf.org/html/rfc7932
// https://blogs.dropbox.com/tech/2016/06/lossless-compression-with-brotli/
// https://hacks.mozilla.org/2015/11/better-than-gzip-compression-with-brotli/

/// A value to represent an encoding
#[derive(Clone, PartialEq, Debug)]
pub enum Encoding {
    /// The `gzip` encoding.
    Gzip,

    /// The `deflate` encoding.
    Deflate,

    /// The `zlib` encoding.
    Zlib,

    /// The `zstd` encoding.
    Zstd,

    /// The `br` encoding.
    Brotli,

    /// The 'lz4' encoding.
    Lz4,

    /// The 'lzma' encoding.
    Lzma,

    /// The 'lzma2' encoding.
    Lzma2,

    /// The 'bincode' encoding.
    BinCode,

    /// The `identity` encoding.
    Identity,

    /// Some other encoding that is less common, can be any String.
    EncodingExt(String),
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Encoding::Gzip => "gzip",
            Encoding::Deflate => "deflate",
            Encoding::Zlib => "zlib",
            Encoding::Zstd => "zstd",
            Encoding::Brotli => "br",
            Encoding::Lz4 => "lz4",
            Encoding::Lzma => "lzma",
            Encoding::Lzma2 => "lzma2",
            Encoding::BinCode => "bincode",
            Encoding::Identity => "identity",
            Encoding::EncodingExt(ref custom) => custom.as_ref(),
        })
    }
}

impl str::FromStr for Encoding {
    type Err = Error;
    fn from_str(s: &str) -> Result<Encoding> {
        match s {
            "deflate" => Ok(Encoding::Deflate),
            "gzip" => Ok(Encoding::Gzip),
            "zlib" => Ok(Encoding::Zlib),
            "zstd" => Ok(Encoding::Zstd),
            "br" => Ok(Encoding::Brotli),
            "lz4" => Ok(Encoding::Lz4),
            "lzma" => Ok(Encoding::Lzma),
            "lzma2" => Ok(Encoding::Lzma2),
            "bincode" => Ok(Encoding::BinCode),
            "identity" => Ok(Encoding::Identity),
            _ => Ok(Encoding::EncodingExt(s.to_owned())),
        }
    }
}

pub fn encode_data(data: &[u8], encoding: &Encoding) -> io::Result<Vec<u8>> {
    use std::io::prelude::*;
    let mut buf = Vec::new();
    #[allow(unreachable_patterns)]
    match *encoding {
        Encoding::Gzip => {
            io::BufReader::new(GzEncoder::new(data, Compression::default()))
                .read_to_end(&mut buf)?;
            Ok(buf)
        }
        Encoding::Deflate => {
            io::BufReader::new(DeflateEncoder::new(data, Compression::default()))
                .read_to_end(&mut buf)?;
            Ok(buf)
        }
        Encoding::Zlib => {
            io::BufReader::new(ZlibEncoder::new(data, Compression::default()))
                .read_to_end(&mut buf)?;
            Ok(buf)
        }
        Encoding::Zstd => {
            let mut writer = io::Cursor::new(&mut buf);
            let mut encoder = zstd::stream::Encoder::new(&mut writer, 0 /* level */)?;
            io::copy(&mut io::Cursor::new(data), &mut encoder)?;
            match encoder.finish() {
                Err(err) => Err(std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to encode with zstd - details: {:?}", err),
                )),
                Ok(_) => Ok(buf),
            }
        }
        Encoding::Brotli => {
            io::BufReader::new(brotli::CompressorReader::new(data, 4096, 6, 20))
                .read_to_end(&mut buf)?;
            Ok(buf)
        }
        Encoding::Lz4 => {
            let mut writer = io::Cursor::new(&mut buf);
            let mut encoder = lz4::EncoderBuilder::new().level(4).build(&mut writer)?;
            io::copy(&mut io::Cursor::new(data), &mut encoder)?;
            let (_, result) = encoder.finish();
            match result {
                Err(err) => Err(std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to encode with lz4 - details: {:?}", err),
                )),
                Ok(_) => {
                    drop(writer);
                    Ok(buf)
                }
            }
        }
        Encoding::Lzma => match lzma_rs::lzma_compress(&mut io::Cursor::new(data), &mut buf) {
            Err(err) => Err(std::io::Error::new(
                io::ErrorKind::Other,
                format!("failed to encode with lzma - details: {:?}", err),
            )),
            _ => Ok(buf),
        },
        Encoding::Lzma2 => match lzma_rs::lzma2_compress(&mut io::Cursor::new(data), &mut buf) {
            Err(err) => Err(std::io::Error::new(
                io::ErrorKind::Other,
                format!("failed to encode with lzma2 - details: {:?}", err),
            )),
            _ => Ok(buf),
        },
        Encoding::BinCode => match bincode::serialize(&data[..]) {
            Err(err) => Err(std::io::Error::new(
                io::ErrorKind::Other,
                format!("failed to encode with bincode - details: {:?}", err),
            )),
            Ok(buf) => Ok(buf),
        },
        Encoding::Identity => Ok(data.to_vec()),
        Encoding::EncodingExt(ref custom) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{}` custom encoding is unsupported", custom),
        )),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{}` encoding is unsupported", encoding.to_string()),
        )),
    }
}

pub fn decode_data(data: &[u8], encoding: &Encoding) -> io::Result<Vec<u8>> {
    use std::io::prelude::*;
    let mut buf = Vec::new();
    #[allow(unreachable_patterns)]
    match *encoding {
        Encoding::Gzip => {
            let mut decoder = GzDecoder::new(buf);
            decoder.write_all(&data[..])?;
            buf = decoder.finish()?;
            Ok(buf)
        }
        Encoding::Deflate => {
            let mut decoder = DeflateDecoder::new(buf);
            decoder.write_all(&data[..])?;
            buf = decoder.finish()?;
            Ok(buf)
        }
        Encoding::Zlib => {
            let mut decoder = ZlibDecoder::new(buf);
            decoder.write_all(&data[..])?;
            buf = decoder.finish()?;
            Ok(buf)
        }
        Encoding::Zstd => {
            let mut reader = io::Cursor::new(data);
            let mut writer = io::Cursor::new(&mut buf);
            match zstd::stream::copy_decode(&mut reader, &mut writer) {
                Err(err) => Err(std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to decode with zstd - details: {:?}", err),
                )),
                _ => Ok(buf),
            }
        }
        Encoding::Brotli => {
            let mut writer = io::Cursor::new(&mut buf);
            let mut decoder = brotli::DecompressorWriter::new(&mut writer, 4096);
            decoder.write_all(&data[..])?;
            drop(decoder);
            drop(writer);
            Ok(buf)
        }
        Encoding::Lz4 => {
            let mut writer = io::Cursor::new(&mut buf);
            let mut decoder = lz4::Decoder::new(io::Cursor::new(data))?;
            io::copy(&mut decoder, &mut writer)?;
            drop(writer);
            Ok(buf)
        }
        Encoding::Lzma => match lzma_rs::lzma_decompress(&mut io::Cursor::new(data), &mut buf) {
            Err(err) => Err(std::io::Error::new(
                io::ErrorKind::Other,
                format!("failed to decode with lzma - details: {:?}", err),
            )),
            _ => Ok(buf),
        },
        Encoding::Lzma2 => match lzma_rs::lzma2_decompress(&mut io::Cursor::new(data), &mut buf) {
            Err(err) => Err(std::io::Error::new(
                io::ErrorKind::Other,
                format!("failed to decode with lzma2 - details: {:?}", err),
            )),
            _ => Ok(buf),
        },
        Encoding::BinCode => match bincode::deserialize(&data[..]) {
            Err(err) => Err(std::io::Error::new(
                io::ErrorKind::Other,
                format!("failed to decode with bincode - details: {:?}", err),
            )),
            Ok(buf) => Ok(buf),
        },
        Encoding::Identity => Ok(data.to_vec()),
        Encoding::EncodingExt(ref custom) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{}` custom decoding is unsupported", custom),
        )),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{}` decoding is unsupported", encoding.to_string()),
        )),
    }
}
