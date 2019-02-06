pub fn encode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use flate2::{read::ZlibEncoder, Compression};
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::BufReader::new(ZlibEncoder::new(data, Compression::default()))
        .read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn decode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use flate2::write::ZlibDecoder;
    use std::io::Write;
    let mut buf = Vec::new();
    let mut decoder = ZlibDecoder::new(buf);
    decoder.write_all(&data[..])?;
    buf = decoder.finish()?;
    Ok(buf)
}
