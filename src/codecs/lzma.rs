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
