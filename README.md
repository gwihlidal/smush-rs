# smush

[![meshopt on travis-ci.com](https://travis-ci.com/gwihlidal/smush-rs.svg?branch=master)](https://travis-ci.com/gwihlidal/smush-rs)
[![Latest version](https://img.shields.io/crates/v/smush.svg)](https://crates.io/crates/smush)
[![Documentation](https://docs.rs/smush/badge.svg)](https://docs.rs/smush)
[![LoC](https://tokei.rs/b1/github/gwihlidal/smush-rs)](https://github.com/gwihlidal/smush-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![APACHE2](https://img.shields.io/badge/license-APACHE2-blue.svg)

Common rust abstraction around a variety of encoding and compression codecs.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
smush = "0.1.4"
```

Example:

```rust
extern crate elapsed;
extern crate smush;

use elapsed::measure_time;
use smush::{decode, enabled_encoding, encode, Encoding, Quality};

const TEST_DATA: &'static [u8] = include_bytes!("../src/ipsum.txt");

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
        println!(
            "[{}] - {} not enabled",
            &format!("{}", quality),
            &encoding
        );
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
```

## Example

```shell
$ cargo run --release --example main

*********************
Level 1 Quality
*********************
[level1] - deflate is 53.08% smaller than identity - encode: 174.30 μs, decode: 64.40 μs
[level1] - gzip is 52.76% smaller than identity - encode: 138.50 μs, decode: 53.60 μs
[level1] - brotli is 53.33% smaller than identity - encode: 165.70 μs, decode: 172.00 μs
[level1] - zlib is 52.97% smaller than identity - encode: 127.50 μs, decode: 57.40 μs
[level1] - zstd is 59.80% smaller than identity - encode: 145.80 μs, decode: 59.70 μs
[level1] - lz4 is 40.96% smaller than identity - encode: 100.00 μs, decode: 49.10 μs
[level1] - xz is 60.63% smaller than identity - encode: 1.58 ms, decode: 205.90 μs
[level1] - bincode is 0.14% larger than identity - encode: 6.10 μs, decode: 9.20 μs
[level1] - base58 is 36.57% larger than identity - encode: 43.33 ms, decode: 14.23 ms
*********************
Default Quality
*********************
[default] - deflate is 63.12% smaller than identity - encode: 217.50 μs, decode: 47.60 μs
[default] - gzip is 62.80% smaller than identity - encode: 201.00 μs, decode: 61.20 μs
[default] - brotli is 63.32% smaller than identity - encode: 1.37 ms, decode: 92.30 μs
[default] - zlib is 63.01% smaller than identity - encode: 170.00 μs, decode: 57.80 μs
[default] - zstd is 62.30% smaller than identity - encode: 566.00 μs, decode: 80.10 μs
[default] - lz4 is 46.67% smaller than identity - encode: 267.80 μs, decode: 33.60 μs
[default] - xz is 62.06% smaller than identity - encode: 4.31 ms, decode: 258.40 μs
[default] - bincode is 0.14% larger than identity - encode: 3.70 μs, decode: 9.10 μs
[default] - base58 is 36.57% larger than identity - encode: 42.93 ms, decode: 13.79 ms
*********************
Maximum Quality
*********************
[maximum] - deflate is 63.12% smaller than identity - encode: 187.40 μs, decode: 66.40 μs
[maximum] - gzip is 62.80% smaller than identity - encode: 172.00 μs, decode: 55.90 μs
[maximum] - brotli is 65.12% smaller than identity - encode: 8.82 ms, decode: 170.70 μs
[maximum] - zlib is 63.01% smaller than identity - encode: 166.70 μs, decode: 46.50 μs
[maximum] - zstd is 63.12% smaller than identity - encode: 12.99 ms, decode: 124.50 μs
[maximum] - lz4 is 46.67% smaller than identity - encode: 350.10 μs, decode: 39.70 μs
[maximum] - xz is 62.06% smaller than identity - encode: 10.91 ms, decode: 895.00 μs
[maximum] - bincode is 0.14% larger than identity - encode: 6.60 μs, decode: 8.60 μs
[maximum] - base58 is 36.57% larger than identity - encode: 43.50 ms, decode: 13.85 ms
```
By default, all codecs are enabled. It may be desirable to only enable the codecs that you want.

You can specify `--no-default-features` / `default-features = false` to disable all codecs, and then opt in to the feature names for the codecs you want.

Available codec feature names:

- bincode_support
- brotli_support
- base58_support
- deflate_support
- gzip_support
- xz_support
- lz4_support
- zlib_support
- zstd_support

As an example, the following shows support for only `brotli`, `lz4`, and `zstd`:

```shell
$ cargo run --release --example main --no-default-features --features=brotli_support,lz4_support,zstd_support

*********************
Level 1 Quality
*********************
[level1] - deflate not enabled
[level1] - gzip not enabled
[level1] - brotli is 53.33% smaller than identity - encode: 302.30 μs, decode: 209.20 μs
[level1] - zlib not enabled
[level1] - zstd is 59.80% smaller than identity - encode: 205.30 μs, decode: 96.90 μs
[level1] - lz4 is 40.96% smaller than identity - encode: 119.40 μs, decode: 53.40 μs
[level1] - xz not enabled
[level1] - bincode not enabled
[level1] - base58 not enabled
*********************
Default Quality
*********************
[default] - deflate not enabled
[default] - gzip not enabled
[default] - brotli is 63.32% smaller than identity - encode: 1.57 ms, decode: 80.20 μs
[default] - zlib not enabled
[default] - zstd is 62.30% smaller than identity - encode: 446.50 μs, decode: 73.50 μs
[default] - lz4 is 46.67% smaller than identity - encode: 241.80 μs, decode: 74.70 μs
[default] - xz not enabled
[default] - bincode not enabled
[default] - base58 not enabled
*********************
Maximum Quality
*********************
[maximum] - deflate not enabled
[maximum] - gzip not enabled
[maximum] - brotli is 65.12% smaller than identity - encode: 8.84 ms, decode: 111.80 μs
[maximum] - zlib not enabled
[maximum] - zstd is 63.12% smaller than identity - encode: 12.53 ms, decode: 110.60 μs
[maximum] - lz4 is 46.67% smaller than identity - encode: 225.90 μs, decode: 78.30 μs
[maximum] - xz not enabled
[maximum] - bincode not enabled
[maximum] - base58 not enabled
```
