use crate::ur::UR;

use anyhow::Result;

use dcbor::CBORTaggedDecodable;

/// A type that can be decoded from a UR.
pub trait URDecodable: CBORTaggedDecodable {
    fn from_ur(ur: impl AsRef<UR>) -> Result<Self> where Self: Sized {
        let tag = &Self::cbor_tags()[0];
        if let Some(name) = tag.name().as_ref() {
            ur.as_ref().check_type(name.clone())?;
            Self::from_untagged_cbor(ur.as_ref().clone().into())
        } else {
            panic!("CBOR tag {} must have a name. Did you call `register_tags()`?", tag.value());
        }
    }

    fn from_ur_string(ur_string: impl Into<String>) -> Result<Self> where Self: Sized {
        Self::from_ur(UR::from_ur_string(ur_string)?)
    }
}

impl<T> URDecodable for T where T: CBORTaggedDecodable { }
