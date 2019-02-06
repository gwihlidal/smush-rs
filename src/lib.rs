use std::{fmt, io, str};

mod codecs;

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

    /// The `brotli` encoding.
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
    // Silence "unreachable pattern" warnings when features are enabled.
    __Nonexhaustive,
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Encoding::Gzip => "gzip",
            Encoding::Deflate => "deflate",
            Encoding::Zlib => "zlib",
            Encoding::Zstd => "zstd",
            Encoding::Brotli => "brotli",
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
            "brotli" => Ok(Encoding::Brotli),
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

pub fn encode_data(data: &[u8], encoding: Encoding) -> io::Result<Vec<u8>> {
    match encoding {
        Encoding::Identity => Ok(data.to_vec()),

        #[cfg(feature = "gzip_support")]
        Encoding::Gzip => codecs::gzip::encode_data(data),

        #[cfg(feature = "deflate_support")]
        Encoding::Deflate => codecs::deflate::encode_data(data),

        #[cfg(feature = "zlib_support")]
        Encoding::Zlib => codecs::zlib::encode_data(data),

        #[cfg(feature = "zstd_support")]
        Encoding::Zstd => codecs::zstd::encode_data(data),

        #[cfg(feature = "brotli_support")]
        Encoding::Brotli => codecs::brotli::encode_data(data),

        #[cfg(feature = "lz4_support")]
        Encoding::Lz4 => codecs::lz4::encode_data(data),

        #[cfg(feature = "lzma_support")]
        Encoding::Lzma => codecs::lzma::encode_data(data),

        #[cfg(feature = "lzma2_support")]
        Encoding::Lzma2 => codecs::lzma2::encode_data(data),

        #[cfg(feature = "bincode_support")]
        Encoding::BinCode => codecs::bincode::encode_data(data),

        #[cfg(feature = "base58_support")]
        Encoding::Base58 => codecs::base58::encode_data(data),

        #[cfg(feature = "custom_support")]
        Encoding::EncodingExt(ref custom) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{}` custom encoding is currently unsupported", custom),
        )),

        disabled => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("encoding algorithm `{}` was not enabled", disabled),
        )),
    }
}

pub fn decode_data(data: &[u8], encoding: Encoding) -> io::Result<Vec<u8>> {
    match encoding {
        Encoding::Identity => Ok(data.to_vec()),

        #[cfg(feature = "gzip_support")]
        Encoding::Gzip => codecs::gzip::decode_data(data),

        #[cfg(feature = "deflate_support")]
        Encoding::Deflate => codecs::deflate::decode_data(data),

        #[cfg(feature = "zlib_support")]
        Encoding::Zlib => codecs::zlib::decode_data(data),

        #[cfg(feature = "zstd_support")]
        Encoding::Zstd => codecs::zstd::decode_data(data),

        #[cfg(feature = "brotli_support")]
        Encoding::Brotli => codecs::brotli::decode_data(data),

        #[cfg(feature = "lz4_support")]
        Encoding::Lz4 => codecs::lz4::decode_data(data),

        #[cfg(feature = "lzma_support")]
        Encoding::Lzma => codecs::lzma::decode_data(data),

        #[cfg(feature = "lzma2_support")]
        Encoding::Lzma2 => codecs::lzma2::decode_data(data),

        #[cfg(feature = "bincode_support")]
        Encoding::BinCode => codecs::bincode::decode_data(data),

        #[cfg(feature = "base58_support")]
        Encoding::Base58 => codecs::base58::decode_data(data),

        Encoding::EncodingExt(ref custom) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("`{}` custom decoding is currently unsupported", custom),
        )),

        disabled => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("encoding algorithm `{}` was not enabled", disabled),
        )),
    }
}

pub fn enabled_encoding(encoding: Encoding) -> bool {
    match encoding {
        Encoding::Gzip => cfg!(feature = "gzip_support"),
        Encoding::Deflate => cfg!(feature = "deflate_support"),
        Encoding::Zlib => cfg!(feature = "zlib_support"),
        Encoding::Zstd => cfg!(feature = "zstd_support"),
        Encoding::Brotli => cfg!(feature = "brotli_support"),
        Encoding::Lz4 => cfg!(feature = "lz4_support"),
        Encoding::Lzma => cfg!(feature = "lzma_support"),
        Encoding::Lzma2 => cfg!(feature = "lzma2_support"),
        Encoding::BinCode => cfg!(feature = "bincode_support"),
        Encoding::Base58 => cfg!(feature = "base58_support"),
        Encoding::Identity => true,
        Encoding::EncodingExt(_) => false,
        _disabled => false,
    }
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

    #[cfg(feature = "gzip_support")]
    #[test]
    fn encode_gzip() {
        encode_data(&TEST_DATA, Encoding::Gzip).unwrap();
    }

    #[cfg(feature = "deflate_support")]
    #[test]
    fn encode_deflate() {
        encode_data(&TEST_DATA, Encoding::Deflate).unwrap();
    }

    #[cfg(feature = "zlib_support")]
    #[test]
    fn encode_zlib() {
        encode_data(&TEST_DATA, Encoding::Zlib).unwrap();
    }

    #[cfg(feature = "zstd_support")]
    #[test]
    fn encode_zstd() {
        encode_data(&TEST_DATA, Encoding::Zstd).unwrap();
    }

    #[cfg(feature = "brotli_support")]
    #[test]
    fn encode_brotli() {
        encode_data(&TEST_DATA, Encoding::Brotli).unwrap();
    }

    #[cfg(feature = "lz4_support")]
    #[test]
    fn encode_lz4() {
        encode_data(&TEST_DATA, Encoding::Lz4).unwrap();
    }

    #[cfg(feature = "lzma_support")]
    #[test]
    fn encode_lzma() {
        encode_data(&TEST_DATA, Encoding::Lzma).unwrap();
    }

    #[cfg(feature = "lzma2_support")]
    #[test]
    fn encode_lzma2() {
        encode_data(&TEST_DATA, Encoding::Lzma2).unwrap();
    }

    #[cfg(feature = "bincode_support")]
    #[test]
    fn encode_bincode() {
        encode_data(&TEST_DATA, Encoding::BinCode).unwrap();
    }

    #[cfg(feature = "base58_support")]
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

    #[cfg(feature = "gzip_support")]
    #[test]
    fn decode_gzip() {
        let encoded = encode_data(&TEST_DATA, Encoding::Gzip).unwrap();
        let decoded = decode_data(&encoded, Encoding::Gzip).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "deflate_support")]
    #[test]
    fn decode_deflate() {
        let encoded = encode_data(&TEST_DATA, Encoding::Deflate).unwrap();
        let decoded = decode_data(&encoded, Encoding::Deflate).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "zlib_support")]
    #[test]
    fn decode_zlib() {
        let encoded = encode_data(&TEST_DATA, Encoding::Zlib).unwrap();
        let decoded = decode_data(&encoded, Encoding::Zlib).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "zstd_support")]
    #[test]
    fn decode_zstd() {
        let encoded = encode_data(&TEST_DATA, Encoding::Zstd).unwrap();
        let decoded = decode_data(&encoded, Encoding::Zstd).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "brotli_support")]
    #[test]
    fn decode_brotli() {
        let encoded = encode_data(&TEST_DATA, Encoding::Brotli).unwrap();
        let decoded = decode_data(&encoded, Encoding::Brotli).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "lz4_support")]
    #[test]
    fn decode_lz4() {
        let encoded = encode_data(&TEST_DATA, Encoding::Lz4).unwrap();
        let decoded = decode_data(&encoded, Encoding::Lz4).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "lzma_support")]
    #[test]
    fn decode_lzma() {
        let encoded = encode_data(&TEST_DATA, Encoding::Lzma).unwrap();
        let decoded = decode_data(&encoded, Encoding::Lzma).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "lzma2_support")]
    #[test]
    fn decode_lzma2() {
        let encoded = encode_data(&TEST_DATA, Encoding::Lzma2).unwrap();
        let decoded = decode_data(&encoded, Encoding::Lzma2).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "bincode_support")]
    #[test]
    fn decode_bincode() {
        let encoded = encode_data(&TEST_DATA, Encoding::BinCode).unwrap();
        let decoded = decode_data(&encoded, Encoding::BinCode).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "base58_support")]
    #[test]
    fn decode_base58() {
        let encoded = encode_data(&TEST_DATA, Encoding::Base58).unwrap();
        let decoded = decode_data(&encoded, Encoding::Base58).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }
}
