extern crate elapsed;
extern crate smush;

use elapsed::measure_time;
use smush::{decode, enabled_encoding, encode, Encoding, Quality};

const TEST_DATA: &[u8] = include_bytes!("../src/ipsum.txt");

fn print_delta(identity: f32, codec: f32, encoding: &str, quality: &str, timings: &str) {
    let delta = (identity - codec) / identity * 100f32;
    if delta > 0f32 {
        println!(
            "[{}] - {} is {:.2}% smaller than identity - {}",
            quality, encoding, delta, timings
        );
    } else {
        println!(
            "[{}] - {} is {:.2}% larger than identity - {}",
            quality,
            encoding,
            delta.abs(),
            timings
        );
    }
}

fn run_test(encoding: Encoding, quality: Quality) {
    if enabled_encoding(encoding.clone()) {
        let (encode_elapsed, encoded) =
            measure_time(|| encode(&TEST_DATA, encoding.clone(), quality.clone()).unwrap());
        assert_ne!(&TEST_DATA, &encoded.as_slice());

        let (decode_elapsed, decoded) =
            measure_time(|| decode(&encoded, encoding.clone()).unwrap());
        assert_eq!(&TEST_DATA, &decoded.as_slice());

        let encoded_len = encoded.len() as f32;
        print_delta(
            TEST_DATA.len() as f32,
            encoded_len,
            &format!("{}", encoding),
            &format!("{}", quality),
            &format!("encode: {}, decode: {}", encode_elapsed, decode_elapsed),
        );
    } else {
        println!("[{}] - {} not enabled", &format!("{}", quality), &encoding);
    }
}

fn run_tests(quality: Quality) {
    run_test(Encoding::Deflate, quality.clone());
    run_test(Encoding::Gzip, quality.clone());
    run_test(Encoding::Brotli, quality.clone());
    run_test(Encoding::Zlib, quality.clone());
    run_test(Encoding::Zstd, quality.clone());
    run_test(Encoding::Lz4, quality.clone());
    run_test(Encoding::Xz, quality.clone());
    run_test(Encoding::BinCode, quality.clone());
    run_test(Encoding::Base58, quality.clone());
}

fn main() {
    println!("*********************");
    println!("Level 1 Quality");
    println!("*********************");
    run_tests(Quality::Level1);

    println!("*********************");
    println!("Default Quality");
    println!("*********************");
    run_tests(Quality::Default);

    println!("*********************");
    println!("Maximum Quality");
    println!("*********************");
    run_tests(Quality::Maximum);
}
