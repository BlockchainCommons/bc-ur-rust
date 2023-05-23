use std::rc::Rc;

use crate::{ur::UR, error::Error};

use dcbor::CBORTaggedDecodable;

pub trait URDecodable: CBORTaggedDecodable {
    fn from_ur(ur: &UR) -> Result<Rc<Self>, Error> {
        ur.check_type(&Self::CBOR_TAG.name().as_ref().unwrap())?;
        Ok(Self::from_untagged_cbor(&ur.cbor).map_err(Error::CBOR)?)
    }

    fn from_ur_string<T: Into<String>>(ur_string: T) -> Result<Rc<Self>, Error> {
        Self::from_ur(&UR::from_ur_string(ur_string)?)
    }
}
