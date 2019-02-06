use std::io;

pub fn encode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    use flate2::{read::DeflateEncoder, Compression};
    use io::Read;
    let mut buf = Vec::new();
    io::BufReader::new(DeflateEncoder::new(data, Compression::default())).read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn decode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    use flate2::write::DeflateDecoder;
    use io::Write;
    let mut buf = Vec::new();
    let mut decoder = DeflateDecoder::new(buf);
    decoder.write_all(&data[..])?;
    buf = decoder.finish()?;
    Ok(buf)
}
