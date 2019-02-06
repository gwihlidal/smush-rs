extern crate smush;

use smush::{decode_data, enabled_encoding, encode_data, Encoding};

const TEST_DATA: &'static [u8] = include_bytes!("../src/ipsum.txt");

fn print_delta(identity: f32, codec: f32, name: &str) {
    let delta = (identity - codec) / identity * 100f32;
    if delta > 0f32 {
        println!("{} is {}% smaller than identity", name, delta);
    } else {
        println!("{} is {}% larger than identity", name, delta.abs());
    }
}

fn run_test(encoding: Encoding) {
    let enabled = enabled_encoding(encoding.clone());
    if enabled {
        let encoded = encode_data(&TEST_DATA, encoding.clone()).unwrap();
        assert_ne!(&TEST_DATA, &encoded.as_slice());

        let decoded = decode_data(&encoded, encoding.clone()).unwrap();
        assert_eq!(&TEST_DATA, &decoded.as_slice());

        let encoded_len = encoded.len() as f32;
        print_delta(
            TEST_DATA.len() as f32,
            encoded_len,
            &format!("{}", encoding),
        );
    } else {
        println!("Encoding '{}': not enabled", &encoding);
    }
}

fn main() {
    run_test(Encoding::Deflate);
    run_test(Encoding::Gzip);
    run_test(Encoding::Brotli);
    run_test(Encoding::Zlib);
    run_test(Encoding::Zstd);
    run_test(Encoding::Lz4);
    run_test(Encoding::Lzma);
    run_test(Encoding::Lzma2);
    run_test(Encoding::BinCode);
    run_test(Encoding::Base58);
}
