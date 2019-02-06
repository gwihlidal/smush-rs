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
    use std::io::Write;
    let mut buf = Vec::new();
    let mut encoder = xz2::write::XzEncoder::new(buf, quality_to_codec(quality));
    encoder.write_all(&data[..])?;
    buf = encoder.finish()?;
    Ok(buf)
}

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    use std::io::Write;
    let mut buf = Vec::new();
    let mut decoder = xz2::write::XzDecoder::new(buf);
    decoder.write_all(&data[..])?;
    buf = decoder.finish()?;
    Ok(buf)
}

/*

// Old code from native rust lzma-rs (lzma2 doesn't seem to compress correctly, and neither variants expose compression level..)

pub fn encode(data: &[u8], _quality: crate::Quality) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma_compress(&mut std::io::Cursor::new(data), &mut buf).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to encode with lzma - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma_decompress(&mut std::io::Cursor::new(data), &mut buf).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to decode with lzma - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn encode(data: &[u8], _quality: crate::Quality) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma2_compress(&mut std::io::Cursor::new(data), &mut buf).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to encode with lzma2 - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma2_decompress(&mut std::io::Cursor::new(data), &mut buf).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to decode with lzma2 - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

*/
