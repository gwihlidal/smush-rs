// http://tools.ietf.org/html/rfc7231#section-5.3.4
// https://tools.ietf.org/html/rfc7932
// https://blogs.dropbox.com/tech/2016/06/lossless-compression-with-brotli/
// https://hacks.mozilla.org/2015/11/better-than-gzip-compression-with-brotli/

fn quality_to_codec(quality: crate::Quality) -> i32 {
    match quality {
        crate::Quality::Default => 6,
        crate::Quality::Level1 => 1,
        crate::Quality::Level2 => 2,
        crate::Quality::Level3 => 3,
        crate::Quality::Level4 => 4,
        crate::Quality::Level5 => 5,
        crate::Quality::Level6 => 6,
        crate::Quality::Level7 => 7,
        crate::Quality::Level8 => 8,
        crate::Quality::Level9 => 9,
        crate::Quality::Maximum => 11,
    }
}

pub fn encode(data: &[u8], quality: crate::Quality) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    let mut buf = Vec::new();
    let buffer_size = 4096;
    let params = brotli::enc::BrotliEncoderParams {
        quality: quality_to_codec(quality),
        lgwin: 20,
        ..Default::default()
    };
    std::io::BufReader::new(brotli::CompressorReader::with_params(
        data,
        buffer_size,
        &params,
    ))
    .read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use std::io::Write;
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    let mut decoder = brotli::DecompressorWriter::new(&mut writer, 4096);
    decoder.write_all(&data[..])?;
    drop(decoder);
    drop(writer);
    Ok(buf)
}
