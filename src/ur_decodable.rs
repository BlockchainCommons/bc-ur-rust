use dcbor::prelude::*;

use crate::UR;

/// A type that can be decoded from a UR.
pub trait URDecodable: CBORTaggedDecodable {
    fn from_ur(ur: impl AsRef<UR>) -> dcbor::Result<Self> where Self: Sized {
        ur.as_ref().check_type(Self::cbor_tags()[0].name().as_ref().unwrap().clone())?;
        Self::from_untagged_cbor(ur.as_ref().clone().into())
    }

    fn from_ur_string(ur_string: impl Into<String>) -> dcbor::Result<Self> where Self: Sized {
        Self::from_ur(UR::from_ur_string(ur_string)?)
    }
}

impl<T> URDecodable for T where T: CBORTaggedDecodable {}
