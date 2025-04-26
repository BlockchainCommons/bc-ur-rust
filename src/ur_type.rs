use crate::{ URTypeString, Error, Result };

#[derive(Debug, Clone, PartialEq)]
pub struct URType(String);

impl URType {
    /// Creates a new URType from the provided type.
    pub fn new(ur_type: impl Into<String>) -> Result<URType> {
        let ur_type = ur_type.into();
        if !ur_type.is_ur_type() {
            return Err(Error::InvalidType);
        }
        Ok(URType(ur_type))
    }

    /// Returns the String representation of the URType.
    pub fn string(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for URType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        URType::new(value)
    }
}

impl TryFrom<&str> for URType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        URType::new(value)
    }
}
