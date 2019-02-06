pub fn encode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use flate2::{read::GzEncoder, Compression};
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::BufReader::new(GzEncoder::new(data, Compression::default())).read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn decode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use flate2::write::GzDecoder;
    use std::io::Write;
    let mut buf = Vec::new();
    let mut decoder = GzDecoder::new(buf);
    decoder.write_all(&data[..])?;
    buf = decoder.finish()?;
    Ok(buf)
}
