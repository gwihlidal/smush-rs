use std::io;

pub fn encode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma_compress(&mut io::Cursor::new(data), &mut buf).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("failed to encode with lzma - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn decode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma_decompress(&mut io::Cursor::new(data), &mut buf).map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("failed to decode with lzma - details: {:?}", err),
        )
    })?;
    Ok(buf)
}
