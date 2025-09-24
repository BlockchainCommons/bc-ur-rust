use dcbor::prelude::*;

use crate::ur::UR;

/// A type that can be encoded to a UR.
pub trait UREncodable: CBORTaggedEncodable {
    /// Returns the UR representation of the object.
    fn ur(&self) -> UR {
        let tag = &Self::cbor_tags()[0];
        if let Some(name) = tag.name().as_ref() {
            UR::new(name.clone(), self.untagged_cbor()).unwrap()
        } else {
            panic!(
                "CBOR tag {} must have a name. Did you call `register_tags()`?",
                tag.value()
            );
        }
    }

    /// Returns the UR string representation of the object.
    fn ur_string(&self) -> String { self.ur().string() }
}

impl<T> UREncodable for T where T: CBORTaggedEncodable {}
