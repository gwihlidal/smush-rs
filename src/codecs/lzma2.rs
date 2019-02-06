use std::io;

pub fn encode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma2_compress(&mut io::Cursor::new(data), &mut buf).map_err(|err| {
        std::io::Error::new(
            io::ErrorKind::Other,
            format!("failed to encode with lzma2 - details: {:?}", err),
        )
    })?;
    Ok(buf)
}

pub fn decode_data(data: &[u8]) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    lzma_rs::lzma2_decompress(&mut io::Cursor::new(data), &mut buf).map_err(|err| {
        std::io::Error::new(
            io::ErrorKind::Other,
            format!("failed to decode with lzma2 - details: {:?}", err),
        )
    })?;
    Ok(buf)
}
