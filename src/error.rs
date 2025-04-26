use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("UR decoder error ({0})")] UR(ur::ur::Error),

    #[error("Bytewords error ({0})")] Bytewords(#[from] ur::bytewords::Error),

    #[error("CBOR error ({0})")] Cbor(#[from] dcbor::Error),

    #[error("invalid UR scheme")]
    InvalidScheme,

    #[error("no UR type specified")]
    TypeUnspecified,

    #[error("invalid UR type")]
    InvalidType,

    #[error("UR is not a single-part")]
    NotSinglePart,

    #[error("expected UR type {0}, but found {1}")] UnexpectedType(String, String),
}

impl From<ur::ur::Error> for Error {
    fn from(err: ur::ur::Error) -> Self {
        Error::UR(err)
    }
}

impl From<Error> for dcbor::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::Cbor(err) => err,
            _ => dcbor::Error::Custom(err.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
