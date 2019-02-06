use std::io;

pub fn encode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut writer = io::Cursor::new(&mut buf);
    let mut encoder = zstd::stream::Encoder::new(&mut writer, 0 /* level */)?;
    io::copy(&mut io::Cursor::new(data), &mut encoder)?;
    encoder.finish().map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("failed to encode with zstd - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn decode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut reader = io::Cursor::new(data);
    let mut writer = io::Cursor::new(&mut buf);
    zstd::stream::copy_decode(&mut reader, &mut writer).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("failed to decode with zstd - details: {:?}", err),
        )
    })?;
    Ok(buf)
}
