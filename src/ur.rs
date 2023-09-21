use dcbor::{CBOR, CBOREncodable};
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
    pub fn new<T: Into<String>, C: CBOREncodable>(ur_type: T, cbor: &C) -> Result<UR, URError> {
        let ur_type = ur_type.into();
        if !ur_type.is_ur_type() {
            return Err(URError::InvalidType);
        }
        let cbor = cbor.cbor();
        Ok(UR { ur_type, cbor })
    }

    /// Creates a new UR from the provided UR string.
    pub fn from_ur_string<T: Into<String>>(ur_string: T) -> Result<UR, URError> {
        let ur_string = ur_string.into();
        let strip_scheme = ur_string.strip_prefix("ur:").ok_or(URError::InvalidScheme)?;
        let (ur_type, _) = strip_scheme.split_once('/').ok_or(URError::TypeUnspecified)?;
        if !ur_type.is_ur_type() {
            return Err(URError::InvalidType);
        }
        let a = decode(&ur_string);
        let (kind, data) = a.map_err(URError::UR)?;
        if kind != ur::ur::Kind::SinglePart {
            return Err(URError::NotSinglePart);
        }
        let cbor = CBOR::from_data(&data).map_err(URError::Cbor)?;
        Ok(UR { ur_type: ur_type.to_string(), cbor })
    }

    /// Returns the String representation of the UR.
    pub fn string(&self) -> String {
        let data = self.cbor.cbor_data();
        ur::encode(&data, &self.ur_type)
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

    /// Returns the CBOR.
    pub fn cbor(&self) -> &CBOR {
        &self.cbor
    }
}

impl std::fmt::Display for UR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ur() {
        let cbor = vec![1, 2, 3].cbor();
        let ur = UR::new("test", &cbor).unwrap();
        let ur_string = ur.string();
        assert_eq!(ur_string, "ur:test/lsadaoaxjygonesw");
        let ur = UR::from_ur_string(ur_string).unwrap();
        assert_eq!(ur.ur_type, "test");
        assert_eq!(&ur.cbor, &cbor);
    }
}
