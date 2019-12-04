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

    /// The 'xz' encoding (also known as `lzma`).
    Xz,

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
            Encoding::Xz => "xz",
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
            "xz" => Ok(Encoding::Xz),
            "bincode" => Ok(Encoding::BinCode),
            "base58" => Ok(Encoding::Base58),
            "identity" => Ok(Encoding::Identity),
            _ => Ok(Encoding::EncodingExt(s.to_owned())),
        }
    }
}

/// A value to represent an encoding quality.
#[derive(Clone, PartialEq, Debug)]
pub enum Quality {
    Default,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
    Maximum,
}

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Quality::Default => "default",
            Quality::Level1 => "level1",
            Quality::Level2 => "level2",
            Quality::Level3 => "level3",
            Quality::Level4 => "level4",
            Quality::Level5 => "level5",
            Quality::Level6 => "level6",
            Quality::Level7 => "level7",
            Quality::Level8 => "level8",
            Quality::Level9 => "level9",
            Quality::Maximum => "maximum",
        })
    }
}

impl str::FromStr for Quality {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Quality::Default),
            "level1" => Ok(Quality::Level1),
            "level2" => Ok(Quality::Level2),
            "level3" => Ok(Quality::Level3),
            "level4" => Ok(Quality::Level4),
            "level5" => Ok(Quality::Level5),
            "level6" => Ok(Quality::Level6),
            "level7" => Ok(Quality::Level7),
            "level8" => Ok(Quality::Level8),
            "level9" => Ok(Quality::Level9),
            "maximum" => Ok(Quality::Maximum),
            _ => unreachable!(),
        }
    }
}

pub fn encode(data: &[u8], encoding: Encoding, quality: Quality) -> io::Result<Vec<u8>> {
    match encoding {
        Encoding::Identity => Ok(data.to_vec()),

        #[cfg(feature = "gzip_support")]
        Encoding::Gzip => codecs::gzip::encode(data, quality),

        #[cfg(feature = "deflate_support")]
        Encoding::Deflate => codecs::deflate::encode(data, quality),

        #[cfg(feature = "zlib_support")]
        Encoding::Zlib => codecs::zlib::encode(data, quality),

        #[cfg(feature = "zstd_support")]
        Encoding::Zstd => codecs::zstd::encode(data, quality),

        #[cfg(feature = "brotli_support")]
        Encoding::Brotli => codecs::brotli::encode(data, quality),

        #[cfg(feature = "lz4_support")]
        Encoding::Lz4 => codecs::lz4::encode(data, quality),

        #[cfg(feature = "xz_support")]
        Encoding::Xz => codecs::xz::encode(data, quality),

        #[cfg(feature = "bincode_support")]
        Encoding::BinCode => codecs::bincode::encode(data, quality),

        #[cfg(feature = "base58_support")]
        Encoding::Base58 => codecs::base58::encode(data, quality),

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

pub fn decode(data: &[u8], encoding: Encoding) -> io::Result<Vec<u8>> {
    match encoding {
        Encoding::Identity => Ok(data.to_vec()),

        #[cfg(feature = "gzip_support")]
        Encoding::Gzip => codecs::gzip::decode(data),

        #[cfg(feature = "deflate_support")]
        Encoding::Deflate => codecs::deflate::decode(data),

        #[cfg(feature = "zlib_support")]
        Encoding::Zlib => codecs::zlib::decode(data),

        #[cfg(feature = "zstd_support")]
        Encoding::Zstd => codecs::zstd::decode(data),

        #[cfg(feature = "brotli_support")]
        Encoding::Brotli => codecs::brotli::decode(data),

        #[cfg(feature = "lz4_support")]
        Encoding::Lz4 => codecs::lz4::decode(data),

        #[cfg(feature = "xz_support")]
        Encoding::Xz => codecs::xz::decode(data),

        #[cfg(feature = "bincode_support")]
        Encoding::BinCode => codecs::bincode::decode(data),

        #[cfg(feature = "base58_support")]
        Encoding::Base58 => codecs::base58::decode(data),

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

pub fn is_encoding_enabled(encoding: &Encoding) -> bool {
    match encoding {
        Encoding::Gzip => cfg!(feature = "gzip_support"),
        Encoding::Deflate => cfg!(feature = "deflate_support"),
        Encoding::Zlib => cfg!(feature = "zlib_support"),
        Encoding::Zstd => cfg!(feature = "zstd_support"),
        Encoding::Brotli => cfg!(feature = "brotli_support"),
        Encoding::Lz4 => cfg!(feature = "lz4_support"),
        Encoding::Xz => cfg!(feature = "xz_support"),
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

    const TEST_DATA: &[u8] = include_bytes!("ipsum.txt");

    #[test]
    fn encode_identity() {
        let encoded = encode(&TEST_DATA, Encoding::Identity, Quality::Default).unwrap();
        assert_eq!(&TEST_DATA, &encoded.as_slice());
    }

    #[cfg(feature = "gzip_support")]
    #[test]
    fn encode_gzip() {
        encode(&TEST_DATA, Encoding::Gzip, Quality::Default).unwrap();
    }

    #[cfg(feature = "deflate_support")]
    #[test]
    fn encode_deflate() {
        encode(&TEST_DATA, Encoding::Deflate, Quality::Default).unwrap();
    }

    #[cfg(feature = "zlib_support")]
    #[test]
    fn encode_zlib() {
        encode(&TEST_DATA, Encoding::Zlib, Quality::Default).unwrap();
    }

    #[cfg(feature = "zstd_support")]
    #[test]
    fn encode_zstd() {
        encode(&TEST_DATA, Encoding::Zstd, Quality::Default).unwrap();
    }

    #[cfg(feature = "brotli_support")]
    #[test]
    fn encode_brotli() {
        encode(&TEST_DATA, Encoding::Brotli, Quality::Default).unwrap();
    }

    #[cfg(feature = "lz4_support")]
    #[test]
    fn encode_lz4() {
        encode(&TEST_DATA, Encoding::Lz4, Quality::Default).unwrap();
    }

    #[cfg(feature = "xz_support")]
    #[test]
    fn encode_xz() {
        encode(&TEST_DATA, Encoding::Xz, Quality::Default).unwrap();
    }

    #[cfg(feature = "bincode_support")]
    #[test]
    fn encode_bincode() {
        encode(&TEST_DATA, Encoding::BinCode, Quality::Default).unwrap();
    }

    #[cfg(feature = "base58_support")]
    #[test]
    fn encode_base58() {
        encode(&TEST_DATA, Encoding::Base58, Quality::Default).unwrap();
    }

    #[test]
    fn decode_identity() {
        let encoded = encode(&TEST_DATA, Encoding::Identity, Quality::Default).unwrap();
        assert_eq!(&encoded, &TEST_DATA);
        let decoded = decode(&encoded, Encoding::Identity).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "gzip_support")]
    #[test]
    fn decode_gzip() {
        let encoded = encode(&TEST_DATA, Encoding::Gzip, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::Gzip).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "deflate_support")]
    #[test]
    fn decode_deflate() {
        let encoded = encode(&TEST_DATA, Encoding::Deflate, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::Deflate).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "zlib_support")]
    #[test]
    fn decode_zlib() {
        let encoded = encode(&TEST_DATA, Encoding::Zlib, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::Zlib).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "zstd_support")]
    #[test]
    fn decode_zstd() {
        let encoded = encode(&TEST_DATA, Encoding::Zstd, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::Zstd).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "brotli_support")]
    #[test]
    fn decode_brotli() {
        let encoded = encode(&TEST_DATA, Encoding::Brotli, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::Brotli).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "lz4_support")]
    #[test]
    fn decode_lz4() {
        let encoded = encode(&TEST_DATA, Encoding::Lz4, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::Lz4).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "xz_support")]
    #[test]
    fn decode_xz() {
        let encoded = encode(&TEST_DATA, Encoding::Xz, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::Xz).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "bincode_support")]
    #[test]
    fn decode_bincode() {
        let encoded = encode(&TEST_DATA, Encoding::BinCode, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::BinCode).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }

    #[cfg(feature = "base58_support")]
    #[test]
    fn decode_base58() {
        let encoded = encode(&TEST_DATA, Encoding::Base58, Quality::Default).unwrap();
        let decoded = decode(&encoded, Encoding::Base58).unwrap();
        assert_eq!(decoded, TEST_DATA);
    }
}
