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

    /// The 'base58' encoding.
    Base58,

    /// The `identity` encoding.
    Identity,

    /// Some other encoding that is less common, can be any String.
    EncodingExt(String),

    #[doc(hidden)]
    __Nonexhaustive, // Hack so we don't get "unreachable pattern" warnings when features are enabled
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
            Encoding::Base58 => "base58",
            Encoding::Identity => "identity",
            Encoding::EncodingExt(ref custom) => custom.as_ref(),
            Encoding::__Nonexhaustive => unreachable!(),
        })
    }
}

impl str::FromStr for Encoding {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            "base58" => Ok(Encoding::Base58),
            "identity" => Ok(Encoding::Identity),
            _ => Ok(Encoding::EncodingExt(s.to_owned())),
        }
    }
}

#[allow(unused_imports)]
pub fn encode_data(data: &[u8], encoding: Encoding) -> io::Result<Vec<u8>> {
    use std::io::prelude::*;

    let mut buf = Vec::new();
    match encoding {
        #[cfg(feature = "gzip")]
        Encoding::Gzip => {
            io::BufReader::new(flate2::read::GzEncoder::new(
                data,
                flate2::Compression::default(),
            ))
            .read_to_end(&mut buf)?;
        }
        #[cfg(feature = "deflate")]
        Encoding::Deflate => {
            io::BufReader::new(flate2::read::DeflateEncoder::new(
                data,
                flate2::Compression::default(),
            ))
            .read_to_end(&mut buf)?;
        }
        #[cfg(feature = "zlib")]
        Encoding::Zlib => {
            io::BufReader::new(flate2::read::ZlibEncoder::new(
                data,
                flate2::Compression::default(),
            ))
            .read_to_end(&mut buf)?;
        }
        #[cfg(feature = "zstd")]
        Encoding::Zstd => {
            let mut writer = io::Cursor::new(&mut buf);
            let mut encoder = zstd::stream::Encoder::new(&mut writer, 0 /* level */)?;
            io::copy(&mut io::Cursor::new(data), &mut encoder)?;
            encoder.finish().map_err(|err| {
                std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to encode with zstd - details: {:?}", err),
                )
            })?;
        }
        #[cfg(feature = "brotli")]
        Encoding::Brotli => {
            io::BufReader::new(brotli::CompressorReader::new(data, 4096, 6, 20))
                .read_to_end(&mut buf)?;
        }
        #[cfg(feature = "lz4")]
        Encoding::Lz4 => {
            let mut writer = io::Cursor::new(&mut buf);
            let mut encoder = lz4::EncoderBuilder::new().level(4).build(&mut writer)?;
            io::copy(&mut io::Cursor::new(data), &mut encoder)?;
            let (_, result) = encoder.finish();
            match result {
                Err(err) => {
                    return Err(std::io::Error::new(
                        io::ErrorKind::Other,
                        format!("failed to encode with lz4 - details: {:?}", err),
                    ));
                }
                Ok(_) => {
                    drop(writer);
                }
            }
        }
        #[cfg(feature = "lzma")]
        Encoding::Lzma => {
            lzma_rs::lzma_compress(&mut io::Cursor::new(data), &mut buf).map_err(|err| {
                std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to encode with lzma - details: {:?}", err),
                )
            })?;
        }
        #[cfg(feature = "lzma")]
        Encoding::Lzma2 => {
            lzma_rs::lzma2_compress(&mut io::Cursor::new(data), &mut buf).map_err(|err| {
                std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to encode with lzma2 - details: {:?}", err),
                )
            })?;
        }
        #[cfg(feature = "bincode")]
        Encoding::BinCode => match bincode::serialize(&data[..]) {
            Err(err) => {
                return Err(std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to encode with bincode - details: {:?}", err),
                ));
            }
            Ok(buff) => buf = buff,
        },
        #[cfg(feature = "base58")]
        Encoding::Base58 => {
            let base58_str = bs58::encode(data).into_string();
            buf = Vec::from(base58_str);
        }
        Encoding::Identity => {
            buf.reserve_exact(data.len() - buf.len());
            // We've reserved the amount of data we need, but
            // copy_from_slice requires that the sizes are the same,
            // this way we ignore the cost of initializing the bytes
            // inside the vec since they are just going to be overwritten
            // anyways
            unsafe { buf.set_len(data.len()) }
            buf.copy_from_slice(data)
        }
        Encoding::EncodingExt(ref custom) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("`{}` custom encoding is unsupported", custom),
            ));
        }
        disabled => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("compression algorithm `{}` was not enabled", disabled),
            ));
        }
    }

    Ok(buf)
}

#[allow(unused_imports)]
pub fn decode_data(data: &[u8], encoding: Encoding) -> io::Result<Vec<u8>> {
    use std::io::prelude::*;

    let mut buf = Vec::new();
    match encoding {
        #[cfg(feature = "gzip")]
        Encoding::Gzip => {
            let mut decoder = flate2::write::GzDecoder::new(buf);
            decoder.write_all(&data[..])?;
            buf = decoder.finish()?;
        }
        #[cfg(feature = "deflate")]
        Encoding::Deflate => {
            let mut decoder = flate2::write::DeflateDecoder::new(buf);
            decoder.write_all(&data[..])?;
            buf = decoder.finish()?;
        }
        #[cfg(feature = "zlib")]
        Encoding::Zlib => {
            let mut decoder = flate2::write::ZlibDecoder::new(buf);
            decoder.write_all(&data[..])?;
            buf = decoder.finish()?;
        }
        #[cfg(feature = "zstd")]
        Encoding::Zstd => {
            let mut reader = io::Cursor::new(data);
            let mut writer = io::Cursor::new(&mut buf);
            zstd::stream::copy_decode(&mut reader, &mut writer).map_err(|err| {
                std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to decode with zstd - details: {:?}", err),
                )
            })?;
        }
        #[cfg(feature = "brotli")]
        Encoding::Brotli => {
            let mut writer = io::Cursor::new(&mut buf);
            let mut decoder = brotli::DecompressorWriter::new(&mut writer, 4096);
            decoder.write_all(&data[..])?;
            drop(decoder);
            drop(writer);
        }
        #[cfg(feature = "lz4")]
        Encoding::Lz4 => {
            let mut writer = io::Cursor::new(&mut buf);
            let mut decoder = lz4::Decoder::new(io::Cursor::new(data))?;
            io::copy(&mut decoder, &mut writer)?;
            drop(writer);
        }
        #[cfg(feature = "lzma")]
        Encoding::Lzma => {
            lzma_rs::lzma_decompress(&mut io::Cursor::new(data), &mut buf).map_err(|err| {
                std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to decode with lzma - details: {:?}", err),
                )
            })?;
        }
        #[cfg(feature = "lzma")]
        Encoding::Lzma2 => {
            lzma_rs::lzma2_decompress(&mut io::Cursor::new(data), &mut buf).map_err(|err| {
                std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to decode with lzma2 - details: {:?}", err),
                )
            })?;
        }
        #[cfg(feature = "bincode")]
        Encoding::BinCode => match bincode::deserialize(&data[..]) {
            Err(err) => {
                return Err(std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to decode with bincode - details: {:?}", err),
                ));
            }
            Ok(buff) => buf = buff,
        },
        #[cfg(feature = "base58")]
        Encoding::Base58 => match str::from_utf8(data) {
            Err(err) => {
                return Err(std::io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to decode with base58 - details: {:?}", err),
                ));
            }
            Ok(ref base58_str) => match bs58::decode(base58_str.trim()).into_vec() {
                Err(err) => {
                    return Err(std::io::Error::new(
                        io::ErrorKind::Other,
                        format!("failed to decode with base58 - details: {:?}", err),
                    ));
                }
                Ok(buff) => buf = buff,
            },
        },
        Encoding::Identity => {
            buf.reserve_exact(data.len() - buf.len());
            // We've reserved the amount of data we need, but
            // copy_from_slice requires that the sizes are the same,
            // this way we ignore the cost of initializing the bytes
            // inside the vec since they are just going to be overwritten
            // anyways
            unsafe { buf.set_len(data.len()) }
            buf.copy_from_slice(data)
        }
        Encoding::EncodingExt(ref custom) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("`{}` custom decoding is unsupported", custom),
            ));
        }
        disabled => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("compression algorithm `{}` was not enabled", disabled),
            ));
        }
    }

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_DATA: &'static [u8] = include_bytes!("ipsum.txt");

    #[test]
    fn encode_identity() {
        let encoded = encode_data(&TEST_DATA, Encoding::Identity).unwrap();
        assert_eq!(&TEST_DATA, &encoded.as_slice());
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn encode_gzip() {
        encode_data(&TEST_DATA, Encoding::Gzip).unwrap();
    }

    #[cfg(feature = "deflate")]
    #[test]
    fn encode_deflate() {
        encode_data(&TEST_DATA, Encoding::Deflate).unwrap();
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn encode_zlib() {
        encode_data(&TEST_DATA, Encoding::Zlib).unwrap();
    }

    #[cfg(feature = "zstd")]
    #[test]
    fn encode_zstd() {
        encode_data(&TEST_DATA, Encoding::Zstd).unwrap();
    }

    #[cfg(feature = "brotli")]
    #[test]
    fn encode_brotli() {
        encode_data(&TEST_DATA, Encoding::Brotli).unwrap();
    }

    #[cfg(feature = "lz4")]
    #[test]
    fn encode_lz4() {
        encode_data(&TEST_DATA, Encoding::Lz4).unwrap();
    }

    #[cfg(feature = "lzma")]
    #[test]
    fn encode_lzma() {
        encode_data(&TEST_DATA, Encoding::Lzma).unwrap();
    }

    #[cfg(feature = "lzma")]
    #[test]
    fn encode_lzma2() {
        encode_data(&TEST_DATA, Encoding::Lzma2).unwrap();
    }

    #[cfg(feature = "bincode")]
    #[test]
    fn encode_bincode() {
        encode_data(&TEST_DATA, Encoding::BinCode).unwrap();
    }

    #[cfg(feature = "base58")]
    #[test]
    fn encode_base58() {
        encode_data(&TEST_DATA, Encoding::Base58).unwrap();
    }

    #[test]
    fn decode_identity() {
        let encoded = encode_data(&TEST_DATA, Encoding::Identity).unwrap();
        assert_eq!(&encoded, &TEST_DATA);
        let decoded = decode_data(&encoded, Encoding::Identity).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "gzip")]
    #[test]
    fn decode_gzip() {
        let encoded = encode_data(&TEST_DATA, Encoding::Gzip).unwrap();
        let decoded = decode_data(&encoded, Encoding::Gzip).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "deflate")]
    #[test]
    fn decode_deflate() {
        let encoded = encode_data(&TEST_DATA, Encoding::Deflate).unwrap();
        let decoded = decode_data(&encoded, Encoding::Deflate).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "zlib")]
    #[test]
    fn decode_zlib() {
        let encoded = encode_data(&TEST_DATA, Encoding::Zlib).unwrap();
        let decoded = decode_data(&encoded, Encoding::Zlib).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "zstd")]
    #[test]
    fn decode_zstd() {
        let encoded = encode_data(&TEST_DATA, Encoding::Zstd).unwrap();
        let decoded = decode_data(&encoded, Encoding::Zstd).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "brotli")]
    #[test]
    fn decode_brotli() {
        let encoded = encode_data(&TEST_DATA, Encoding::Brotli).unwrap();
        let decoded = decode_data(&encoded, Encoding::Brotli).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "lz4")]
    #[test]
    fn decode_lz4() {
        let encoded = encode_data(&TEST_DATA, Encoding::Lz4).unwrap();
        let decoded = decode_data(&encoded, Encoding::Lz4).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "lzma")]
    #[test]
    fn decode_lzma() {
        let encoded = encode_data(&TEST_DATA, Encoding::Lzma).unwrap();
        let decoded = decode_data(&encoded, Encoding::Lzma).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "lzma")]
    #[test]
    fn decode_lzma2() {
        let encoded = encode_data(&TEST_DATA, Encoding::Lzma2).unwrap();
        let decoded = decode_data(&encoded, Encoding::Lzma2).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "bincode")]
    #[test]
    fn decode_bincode() {
        let encoded = encode_data(&TEST_DATA, Encoding::BinCode).unwrap();
        let decoded = decode_data(&encoded, Encoding::BinCode).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "base58")]
    #[test]
    fn decode_base58() {
        let encoded = encode_data(&TEST_DATA, Encoding::Base58).unwrap();
        let decoded = decode_data(&encoded, Encoding::Base58).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }
}
