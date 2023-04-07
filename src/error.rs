#[derive(Debug)]
pub enum Error {
    // UR Decoder Error
    UR(ur::ur::Error),
    // CBOR Error
    CBOR(dcbor::CBORError),
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
