use std::io;

pub fn encode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = io::Cursor::new(&mut buf);
    let mut encoder = lz4::EncoderBuilder::new().level(4).build(&mut writer)?;
    io::copy(&mut io::Cursor::new(data), &mut encoder)?;
    let (_, result) = encoder.finish();
    match result {
        Err(err) => Err(std::io::Error::new(
            io::ErrorKind::Other,
            format!("failed to encode with lz4 - details: {:?}", err),
        )),
        Ok(_) => {
            drop(writer);
            Ok(buf)
        }
    }
}

pub fn decode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = io::Cursor::new(&mut buf);
    let mut decoder = lz4::Decoder::new(io::Cursor::new(data))?;
    io::copy(&mut decoder, &mut writer)?;
    drop(writer);
    Ok(buf)
}
