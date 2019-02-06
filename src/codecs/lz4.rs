pub fn encode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    let mut encoder = lz4::EncoderBuilder::new().level(4).build(&mut writer)?;
    std::io::copy(&mut std::io::Cursor::new(data), &mut encoder)?;
    let (_, result) = encoder.finish();
    match result {
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to encode with lz4 - details: {:?}", err),
        )),
        Ok(_) => {
            drop(writer);
            Ok(buf)
        }
    }
}

pub fn decode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    let mut decoder = lz4::Decoder::new(std::io::Cursor::new(data))?;
    std::io::copy(&mut decoder, &mut writer)?;
    drop(writer);
    Ok(buf)
}
