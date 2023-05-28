#[derive(Debug)]
pub enum Error {
    // UR Decoder Error
    UR(ur::ur::Error),
    // CBOR Error
    Cbor(dcbor::Error),
    /// Invalid scheme.
    InvalidScheme,
    /// No type specified.
    TypeUnspecified,
    /// Invalid UR type.
    InvalidType,
    /// Not single-part.
    NotSinglePart,
    /// Unexpected UR type.
    UnexpectedType,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::UR(err) => format!("UR Decoder Error: {}", err),
            Error::Cbor(err) => format!("CBOR Error: {}", err),
            Error::InvalidScheme => "Invalid scheme".to_string(),
            Error::TypeUnspecified => "No type specified".to_string(),
            Error::InvalidType => "Invalid UR type".to_string(),
            Error::NotSinglePart => "Not single-part".to_string(),
            Error::UnexpectedType => "Unexpected UR type".to_string(),
        };
        f.write_str(&s)
    }
}

impl std::error::Error for Error {}
