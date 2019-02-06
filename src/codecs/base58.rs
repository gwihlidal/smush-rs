pub fn encode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let base58_str = bs58::encode(data).into_string();
    Ok(Vec::from(base58_str))
}

pub fn decode_data(data: &[u8]) -> std::io::Result<Vec<u8>> {
    match std::str::from_utf8(data) {
        Err(err) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to decode with base58 - details: {:?}", err),
        )),
        Ok(ref base58_str) => match bs58::decode(base58_str.trim()).into_vec() {
            Err(err) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("failed to decode with base58 - details: {:?}", err),
            )),
            Ok(buf) => Ok(buf),
        },
    }
}
