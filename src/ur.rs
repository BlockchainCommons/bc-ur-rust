use dcbor::prelude::*;
use ur::decode;

use crate::{Error, Result, URType};

/// A Uniform Resource (UR) is a URI-encoded CBOR object.
#[derive(Debug, Clone, PartialEq)]
pub struct UR {
    ur_type: URType,
    cbor: CBOR,
}

impl UR {
    /// Creates a new UR from the provided type and CBOR.
    pub fn new(
        ur_type: impl TryInto<URType, Error = Error>,
        cbor: impl Into<CBOR>,
    ) -> Result<UR> {
        let ur_type = ur_type.try_into()?;
        let cbor = cbor.into();
        Ok(UR { ur_type, cbor })
    }

    /// Creates a new UR from the provided UR string.
    pub fn from_ur_string(ur_string: impl Into<String>) -> Result<UR> {
        let ur_string = ur_string.into().to_lowercase();
        let strip_scheme =
            ur_string.strip_prefix("ur:").ok_or(Error::InvalidScheme)?;
        let (ur_type, _) =
            strip_scheme.split_once('/').ok_or(Error::TypeUnspecified)?;
        let ur_type = URType::new(ur_type)?;
        let a = decode(&ur_string);
        let (kind, data) = a.map_err(Error::UR)?;
        if kind != ur::ur::Kind::SinglePart {
            return Err(Error::NotSinglePart);
        }
        let cbor = CBOR::try_from_data(data)?;
        Ok(UR { ur_type, cbor })
    }

    /// Returns the String representation of the UR.
    pub fn string(&self) -> String {
        let data = self.cbor.to_cbor_data();
        ur::encode(&data, &ur::Type::Custom(self.ur_type.string()))
    }

    /// Returns the String representation of the UR in uppercase,
    /// most-efficient for QR codes.
    pub fn qr_string(&self) -> String { self.string().to_uppercase() }

    /// Returns the data representation of the UR in uppercase,
    /// most-efficient for QR codes.
    pub fn qr_data(&self) -> Vec<u8> { self.qr_string().as_bytes().to_vec() }

    /// Checks the UR type against the provided type.
    pub fn check_type(
        &self,
        other_type: impl TryInto<URType, Error = Error>,
    ) -> Result<()> {
        let other_type = other_type.try_into()?;
        if self.ur_type != other_type {
            Err(Error::UnexpectedType(
                other_type.string().to_string(),
                self.ur_type.string().to_string(),
            ))?;
        }
        Ok(())
    }

    pub fn ur_type(&self) -> &URType { &self.ur_type }

    /// Returns the UR type.
    pub fn ur_type_str(&self) -> &str { self.ur_type.string() }

    pub fn cbor(&self) -> CBOR { self.cbor.clone() }
}

impl From<UR> for CBOR {
    fn from(ur: UR) -> Self { ur.cbor }
}

impl From<UR> for String {
    fn from(ur: UR) -> Self { ur.string() }
}

impl TryFrom<String> for UR {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> { UR::from_ur_string(value) }
}

impl std::fmt::Display for UR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}

impl AsRef<UR> for UR {
    fn as_ref(&self) -> &UR { self }
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
        assert_eq!(ur.ur_type_str(), "test");
        assert_eq!(&ur.cbor, &cbor);

        let caps_ur_string = "UR:TEST/LSADAOAXJYGONESW";
        let ur = UR::from_ur_string(caps_ur_string).unwrap();
        assert_eq!(ur.ur_type_str(), "test");
        assert_eq!(&ur.cbor, &cbor);
    }
}
