use crate::ur::UR;

use dcbor::CBORTaggedEncodable;

pub trait UREncodable: CBORTaggedEncodable {
    /// Returns the UR representation of the object.
    fn ur(&self) -> UR {
        UR::new(Self::CBOR_TAG.name().as_ref().unwrap(), &self.untagged_cbor()).unwrap()
    }

    /// Returns the UR string representation of the object.
    fn ur_string(&self) -> String {
        self.ur().string()
    }
}
