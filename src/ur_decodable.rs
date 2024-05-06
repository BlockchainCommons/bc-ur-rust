use crate::ur::UR;

use dcbor::CBORTaggedDecodable;

/// A type that can be decoded from a UR.
pub trait URDecodable: CBORTaggedDecodable {
    fn from_ur(ur: impl AsRef<UR>) -> anyhow::Result<Self> where Self: Sized {
        ur.as_ref().check_type(Self::cbor_tags()[0].name().as_ref().unwrap())?;
        Self::from_untagged_cbor(ur.as_ref().clone().into())
    }

    fn from_ur_string(ur_string: impl Into<String>) -> anyhow::Result<Self> where Self: Sized {
        Self::from_ur(UR::from_ur_string(ur_string)?)
    }
}

impl<T> URDecodable for T where T: CBORTaggedDecodable { }
