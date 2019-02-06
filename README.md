# smush

[![meshopt on travis-ci.com](https://travis-ci.com/gwihlidal/smush-rs.svg?branch=master)](https://travis-ci.com/gwihlidal/smush-rs)
[![Latest version](https://img.shields.io/crates/v/smush.svg)](https://crates.io/crates/smush)
[![Documentation](https://docs.rs/smush/badge.svg)](https://docs.rs/smush)
[![LoC](https://tokei.rs/b1/github/gwihlidal/smush)](https://github.com/gwihlidal/smush)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![APACHE2](https://img.shields.io/badge/license-APACHE2-blue.svg)

Common rust abstraction around a variety of encoding and compression codecs.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
smush = "0.1.2"
```

Example:

```rust
extern crate smush;

use smush::{decode, enabled_encoding, encode, Encoding, Quality};

const TEST_DATA: &'static [u8] = include_bytes!("../src/ipsum.txt");

fn print_delta(identity: f32, codec: f32, encoding: &str, quality: &str) {
    let delta = (identity - codec) / identity * 100f32;
    if delta > 0f32 {
        println!(
            "[{}] - {} is {}% smaller than identity",
            quality, encoding, delta
        );
    } else {
        println!(
            "[{}] - {} is {}% larger than identity",
            quality,
            encoding,
            delta.abs()
        );
    }
}

fn run_test(encoding: Encoding, quality: Quality) {
    let enabled = enabled_encoding(encoding.clone());
    if enabled {
        let encoded = encode(&TEST_DATA, encoding.clone(), quality.clone()).unwrap();
        assert_ne!(&TEST_DATA, &encoded.as_slice());

        let decoded = decode(&encoded, encoding.clone()).unwrap();
        assert_eq!(&TEST_DATA, &decoded.as_slice());

        let encoded_len = encoded.len() as f32;
        print_delta(
            TEST_DATA.len() as f32,
            encoded_len,
            &format!("{}", encoding),
            &format!("{}", quality),
        );
    } else {
        println!("Encoding '{}': not enabled", &encoding);
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
[level1] - deflate is 53.080654% smaller than identity
[level1] - gzip is 52.757324% smaller than identity
[level1] - brotli is 53.332138% smaller than identity
[level1] - zlib is 52.972878% smaller than identity
[level1] - zstd is 59.798813% smaller than identity
[level1] - lz4 is 40.95563% smaller than identity
[level1] - xz is 60.62511% smaller than identity
[level1] - bincode is 0.14370397% larger than identity
[level1] - base58 is 36.57266% larger than identity
*********************
Default Quality
*********************
[default] - deflate is 63.121967% smaller than identity
[default] - gzip is 62.798637% smaller than identity
[default] - brotli is 63.319565% smaller than identity
[default] - zlib is 63.01419% smaller than identity
[default] - zstd is 62.29567% smaller than identity
[default] - lz4 is 46.667866% smaller than identity
[default] - xz is 62.06215% smaller than identity
[default] - bincode is 0.14370397% larger than identity
[default] - base58 is 36.57266% larger than identity
*********************
Maximum Quality
*********************
[maximum] - deflate is 63.121967% smaller than identity
[maximum] - gzip is 62.798637% smaller than identity
[maximum] - brotli is 65.11586% smaller than identity
[maximum] - zlib is 63.01419% smaller than identity
[maximum] - zstd is 64.199745% smaller than identity
[maximum] - lz4 is 46.667866% smaller than identity
[maximum] - xz is 62.06215% smaller than identity
[maximum] - bincode is 0.14370397% larger than identity
[maximum] - base58 is 36.57266% larger than identity
```