pub fn encode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma2_compress(&mut std::io::Cursor::new(data), &mut buf).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to encode with lzma2 - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn decode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma2_decompress(&mut std::io::Cursor::new(data), &mut buf).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to decode with lzma2 - details: {:?}", err),
        )
    })?;
    Ok(buf)
}
