pub fn compress_with_gzip(body: &[u8]) -> anyhow::Result<Vec<u8>> {
    use std::io::Write;

    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(body)?;
    let compressed = encoder.finish()?;

    Ok(compressed)
}

pub fn compress_with_deflate(body: &[u8]) -> anyhow::Result<Vec<u8>> {
    use std::io::Write;

    let mut encoder =
        flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(body)?;
    let compressed = encoder.finish()?;

    Ok(compressed)
}
