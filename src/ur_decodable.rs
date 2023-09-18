use crate::ur::UR;

use dcbor::CBORTaggedDecodable;

/// A type that can be decoded from a UR.
pub trait URDecodable: CBORTaggedDecodable {
    fn from_ur(ur: &UR) -> anyhow::Result<Self> where Self: Sized {
        ur.check_type(Self::CBOR_TAG.name().as_ref().unwrap())?;
        Self::from_untagged_cbor(&ur.cbor)
    }

    fn from_ur_string<T: Into<String>>(ur_string: T) -> anyhow::Result<Self> where Self: Sized {
        Self::from_ur(&UR::from_ur_string(ur_string)?)
    }
}
