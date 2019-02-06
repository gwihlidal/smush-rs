// http://tools.ietf.org/html/rfc7231#section-5.3.4
// https://tools.ietf.org/html/rfc7932
// https://blogs.dropbox.com/tech/2016/06/lossless-compression-with-brotli/
// https://hacks.mozilla.org/2015/11/better-than-gzip-compression-with-brotli/

pub fn encode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::BufReader::new(brotli::CompressorReader::new(data, 4096, 6, 20))
        .read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn decode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use std::io::Write;
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    let mut decoder = brotli::DecompressorWriter::new(&mut writer, 4096);
    decoder.write_all(&data[..])?;
    drop(decoder);
    drop(writer);
    Ok(buf)
}
