use dcbor::CBOR;
use ur::decode;
use crate::{utils::URTypeString, error::URError};

/// A Uniform Resource (UR) is a URI-encoded CBOR object.
#[derive(Debug, Clone, PartialEq)]
pub struct UR {
    ur_type: String,
    cbor: CBOR,
}

impl UR {
    /// Creates a new UR from the provided type and CBOR.
    pub fn new<T: TryInto<CBOR>>(ur_type: impl Into<String>, cbor: T) -> anyhow::Result<UR>
    where <T as TryInto<CBOR>>::Error: std::error::Error + Send + Sync + 'static
    {
        let ur_type = ur_type.into();
        if !ur_type.is_ur_type() {
            return Err(URError::InvalidType.into());
        }
        let cbor = cbor.try_into()?;
        Ok(UR { ur_type, cbor })
    }

    /// Creates a new UR from the provided UR string.
    pub fn from_ur_string(ur_string: impl Into<String>) -> anyhow::Result<UR> {
        let ur_string = ur_string.into().to_lowercase();
        let strip_scheme = ur_string.strip_prefix("ur:").ok_or(URError::InvalidScheme)?;
        let (ur_type, _) = strip_scheme.split_once('/').ok_or(URError::TypeUnspecified)?;
        if !ur_type.is_ur_type() {
            return Err(URError::InvalidType.into());
        }
        let a = decode(&ur_string);
        let (kind, data) = a.map_err(URError::UR)?;
        if kind != ur::ur::Kind::SinglePart {
            return Err(URError::NotSinglePart.into());
        }
        let cbor = CBOR::from_data(data)?;
        Ok(UR { ur_type: ur_type.to_string(), cbor })
    }

    /// Returns the String representation of the UR.
    pub fn string(&self) -> String {
        let data = self.cbor.cbor_data();
        ur::encode(&data, &ur::Type::Custom(&self.ur_type))
    }

    /// Returns the String representation of the UR in uppercase,
    /// most-efficient for QR codes.
    pub fn qr_string(&self) -> String {
        self.string().to_uppercase()
    }

    /// Returns the data representation of the UR in uppercase,
    /// most-efficient for QR codes.
    pub fn qr_data(&self) -> Vec<u8> {
        self.qr_string().as_bytes().to_vec()
    }

    /// Checks the UR type against the provided type.
    pub fn check_type(&self, ur_type: &str) -> Result<(), URError> {
        if self.ur_type != ur_type {
            return Err(URError::UnexpectedType(ur_type.to_string(), self.ur_type.clone()));
        }
        Ok(())
    }

    /// Returns the UR type.
    pub fn ur_type(&self) -> &str {
        &self.ur_type
    }
}

impl From<UR> for CBOR {
    fn from(ur: UR) -> Self {
        ur.cbor
    }
}

impl From<UR> for String {
    fn from(ur: UR) -> Self {
        ur.string()
    }
}

impl TryFrom<String> for UR {
    type Error = anyhow::Error;

    fn try_from(value: String) -> anyhow::Result<Self> {
        UR::from_ur_string(value)
    }
}

impl std::fmt::Display for UR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}

impl AsRef<UR> for UR {
    fn as_ref(&self) -> &UR {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ur() {
        let cbor: CBOR = vec![1, 2, 3].into();
        let ur = UR::new("test", cbor.clone()).unwrap();
        let ur_string = ur.string();
        assert_eq!(ur_string, "ur:test/lsadaoaxjygonesw");
        let ur = UR::from_ur_string(ur_string).unwrap();
        assert_eq!(ur.ur_type, "test");
        assert_eq!(&ur.cbor, &cbor);

        let caps_ur_string = "UR:TEST/LSADAOAXJYGONESW";
        let ur = UR::from_ur_string(caps_ur_string).unwrap();
        assert_eq!(ur.ur_type, "test");
        assert_eq!(&ur.cbor, &cbor);
    }
}
