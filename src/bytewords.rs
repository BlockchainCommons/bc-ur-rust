pub use ur::bytewords::Style;

pub fn encode(data: impl AsRef<[u8]>, style: Style) -> String {
    ur::bytewords::encode(data.as_ref(), style)
}

pub fn decode(data: &str, style: Style) -> Result<Vec<u8>, anyhow::Error> {
    ur::bytewords::decode(data, style).map_err(|e| anyhow::anyhow!(e))
}
