extern crate smush;

use smush::{decode_data, encode_data, Encoding};

const TEST_DATA: &'static [u8] = include_bytes!("../src/ipsum.txt");

fn print_delta(identity: f32, codec: f32, name: &str) {
    let delta = (identity - codec) / identity * 100f32;
    if delta > 0f32 {
        println!("{} is {}% smaller than Identity", name, delta);
    } else {
        println!("{} is {}% larger than Identity", name, delta.abs());
    }
}

fn main() {
    let identity = encode_data(&TEST_DATA, Encoding::Identity).unwrap();
    let deflate = encode_data(&TEST_DATA, Encoding::Deflate).unwrap();
    let gzip = encode_data(&TEST_DATA, Encoding::Gzip).unwrap();
    let brotli = encode_data(&TEST_DATA, Encoding::Brotli).unwrap();
    let zlib = encode_data(&TEST_DATA, Encoding::Zlib).unwrap();
    let zstd = encode_data(&TEST_DATA, Encoding::Zstd).unwrap();
    let lz4 = encode_data(&TEST_DATA, Encoding::Lz4).unwrap();
    let lzma = encode_data(&TEST_DATA, Encoding::Lzma).unwrap();
    let lzma2 = encode_data(&TEST_DATA, Encoding::Lzma2).unwrap();
    let bincode = encode_data(&TEST_DATA, Encoding::BinCode).unwrap();
    let base58 = encode_data(&TEST_DATA, Encoding::Base58).unwrap();

    assert_eq!(&TEST_DATA, &identity.as_slice());
    assert_ne!(&TEST_DATA, &deflate.as_slice());
    assert_ne!(&TEST_DATA, &gzip.as_slice());
    assert_ne!(&TEST_DATA, &brotli.as_slice());
    assert_ne!(&TEST_DATA, &zlib.as_slice());
    assert_ne!(&TEST_DATA, &zstd.as_slice());
    assert_ne!(&TEST_DATA, &lz4.as_slice());
    assert_ne!(&TEST_DATA, &lzma.as_slice());
    assert_ne!(&TEST_DATA, &lzma2.as_slice());
    assert_ne!(&TEST_DATA, &bincode.as_slice());
    assert_ne!(&TEST_DATA, &base58.as_slice());

    let identity_prime = decode_data(&identity, Encoding::Identity).unwrap();
    let deflate_prime = decode_data(&deflate, Encoding::Deflate).unwrap();
    let gzip_prime = decode_data(&gzip, Encoding::Gzip).unwrap();
    let brotli_prime = decode_data(&brotli, Encoding::Brotli).unwrap();
    let zlib_prime = decode_data(&zlib, Encoding::Zlib).unwrap();
    let zstd_prime = decode_data(&zstd, Encoding::Zstd).unwrap();
    let lz4_prime = decode_data(&lz4, Encoding::Lz4).unwrap();
    let lzma_prime = decode_data(&lzma, Encoding::Lzma).unwrap();
    let lzma2_prime = decode_data(&lzma2, Encoding::Lzma2).unwrap();
    let bincode_prime = decode_data(&bincode, Encoding::BinCode).unwrap();
    let base58_prime = decode_data(&base58, Encoding::Base58).unwrap();

    let identity_len = identity.len() as f32;
    let deflate_len = deflate.len() as f32;
    let gzip_len = gzip.len() as f32;
    let brotli_len = brotli.len() as f32;
    let zlib_len = zlib.len() as f32;
    let zstd_len = zstd.len() as f32;
    let lz4_len = lz4.len() as f32;
    let lzma_len = lzma.len() as f32;
    let lzma2_len = lzma2.len() as f32;
    let bincode_len = bincode.len() as f32;
    let base58_len = base58.len() as f32;

    print_delta(identity_len, deflate_len, "Deflate");
    print_delta(identity_len, gzip_len, "Gzip");
    print_delta(identity_len, brotli_len, "Brotli");
    print_delta(identity_len, zlib_len, "Zlib");
    print_delta(identity_len, zstd_len, "Zstd");
    print_delta(identity_len, lz4_len, "Lz4");
    print_delta(identity_len, lzma_len, "Lzma");
    print_delta(identity_len, lzma2_len, "Lzma2");
    print_delta(identity_len, bincode_len, "BinCode");
    print_delta(identity_len, base58_len, "Base58");

    assert_eq!(&TEST_DATA, &identity_prime.as_slice());
    assert_eq!(&TEST_DATA, &deflate_prime.as_slice());
    assert_eq!(&TEST_DATA, &gzip_prime.as_slice());
    assert_eq!(&TEST_DATA, &brotli_prime.as_slice());
    assert_eq!(&TEST_DATA, &zlib_prime.as_slice());
    assert_eq!(&TEST_DATA, &zstd_prime.as_slice());
    assert_eq!(&TEST_DATA, &lz4_prime.as_slice());
    assert_eq!(&TEST_DATA, &lzma_prime.as_slice());
    assert_eq!(&TEST_DATA, &lzma2_prime.as_slice());
    assert_eq!(&TEST_DATA, &bincode_prime.as_slice());
    assert_eq!(&TEST_DATA, &base58_prime.as_slice());
}
