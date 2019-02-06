pub fn encode(data: &[u8], _quality: crate::Quality) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    let mut encoder = zstd::stream::Encoder::new(&mut writer, 0 /* level */)?;
    std::io::copy(&mut std::io::Cursor::new(data), &mut encoder)?;
    encoder.finish().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to encode with zstd - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut reader = std::io::Cursor::new(data);
    let mut writer = std::io::Cursor::new(&mut buf);
    zstd::stream::copy_decode(&mut reader, &mut writer).map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to decode with zstd - details: {:?}", err),
        )
    })?;
    Ok(buf)
}
