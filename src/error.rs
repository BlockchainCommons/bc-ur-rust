use thiserror::Error;

#[derive(Debug, Error)]
pub enum URError {
    #[error("UR decoder error ({0})")]
    UR(ur::ur::Error),
    #[error("CBOR error ({0})")]
    Cbor(dcbor::CBORError),
    #[error("invalid UR scheme")]
    InvalidScheme,
    #[error("no UR type specified")]
    TypeUnspecified,
    #[error("invalid UR type")]
    InvalidType,
    #[error("UR is not a single-part")]
    NotSinglePart,
    #[error("expected UR type {0}, but found {1}")]
    UnexpectedType(String, String),
}

impl From<ur::ur::Error> for URError {
    fn from(err: ur::ur::Error) -> Self {
        URError::UR(err)
    }
}

impl From<dcbor::CBORError> for URError {
    fn from(err: dcbor::CBORError) -> Self {
        URError::Cbor(err)
    }
}
