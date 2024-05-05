use crate::ur::UR;

use dcbor::CBORTaggedEncodable;

/// A type that can be encoded to a UR.
pub trait UREncodable: CBORTaggedEncodable {
    /// Returns the UR representation of the object.
    fn ur(&self) -> UR {
        UR::new(Self::cbor_tags()[0].name().as_ref().unwrap(), self.untagged_cbor()).unwrap()
    }

    /// Returns the UR string representation of the object.
    fn ur_string(&self) -> String {
        self.ur().string()
    }
}

impl<T> UREncodable for T where T: CBORTaggedEncodable { }
