fn quality_to_codec(quality: crate::Quality) -> u32 {
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
        crate::Quality::Maximum => 9,
    }
}

pub fn encode(data: &[u8], quality: crate::Quality) -> std::io::Result<Vec<u8>> {
    use flate2::{read::ZlibEncoder, Compression};
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::BufReader::new(ZlibEncoder::new(
        data,
        Compression::new(quality_to_codec(quality)),
    ))
    .read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use flate2::write::ZlibDecoder;
    use std::io::Write;
    let mut buf = Vec::new();
    let mut decoder = ZlibDecoder::new(buf);
    decoder.write_all(&data[..])?;
    buf = decoder.finish()?;
    Ok(buf)
}
