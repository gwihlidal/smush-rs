pub fn encode(data: &[u8], _quality: crate::Quality) -> std::io::Result<Vec<u8>> {
    use flate2::{read::DeflateEncoder, Compression};
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::BufReader::new(DeflateEncoder::new(data, Compression::default()))
        .read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use flate2::write::DeflateDecoder;
    use std::io::Write;
    let mut buf = Vec::new();
    let mut decoder = DeflateDecoder::new(buf);
    decoder.write_all(&data[..])?;
    buf = decoder.finish()?;
    Ok(buf)
}
