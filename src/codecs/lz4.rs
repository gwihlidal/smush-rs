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
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    let mut encoder = lz4::EncoderBuilder::new()
        .level(quality_to_codec(quality))
        .build(&mut writer)?;
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

pub fn decode(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buf);
    let mut decoder = lz4::Decoder::new(std::io::Cursor::new(data))?;
    std::io::copy(&mut decoder, &mut writer)?;
    drop(writer);
    Ok(buf)
}
