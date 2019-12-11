use elapsed::measure_time;
use smush::{
    decode, encode, is_codec_enabled,
    Codec::{self, *},
    Quality,
};

const TEST_DATA: &[u8] = include_bytes!("../src/ipsum.txt");

fn print_delta(identity: f32, encoded: f32, codec: Codec, quality: Quality, timings: &str) {
    let delta = (identity - encoded) / identity * 100f32;
    if delta > 0f32 {
        println!(
            "[{}] - {} is {:.2}% smaller than identity - {}",
            quality, codec, delta, timings
        );
    } else {
        println!(
            "[{}] - {} is {:.2}% larger than identity - {}",
            quality,
            codec,
            delta.abs(),
            timings
        );
    }
}

fn run_test(encoding: Codec, quality: Quality) {
    if is_codec_enabled(encoding) {
        let (encode_elapsed, encoded) =
            measure_time(|| encode(&TEST_DATA, encoding, quality).unwrap());
        assert_ne!(&TEST_DATA, &encoded.as_slice());

        let (decode_elapsed, decoded) = measure_time(|| decode(&encoded, encoding).unwrap());
        assert_eq!(&TEST_DATA, &decoded.as_slice());

        let encoded_len = encoded.len() as f32;
        print_delta(
            TEST_DATA.len() as f32,
            encoded_len,
            encoding,
            quality,
            &format!("encode: {}, decode: {}", encode_elapsed, decode_elapsed),
        );
    } else {
        println!("[{}] - {} not enabled", quality, encoding);
    }
}

fn run_tests(quality: Quality) {
    run_test(Deflate, quality);
    run_test(Gzip, quality);
    run_test(Brotli, quality);
    run_test(Zlib, quality);
    run_test(Zstd, quality);
    run_test(Lz4, quality);
    run_test(Xz, quality);
    run_test(BinCode, quality);
    run_test(Base58, quality);
}

fn main() {
    println!("*********************");
    println!("Level 1 Quality");
    println!("*********************");
    run_tests(Quality::Level1);

    println!();
    println!("*********************");
    println!("Default Quality");
    println!("*********************");
    run_tests(Quality::Default);

    println!();
    println!("*********************");
    println!("Maximum Quality");
    println!("*********************");
    run_tests(Quality::Maximum);
}
