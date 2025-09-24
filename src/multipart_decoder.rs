use dcbor::prelude::*;

use crate::{Error, Result, UR, URType};

pub struct MultipartDecoder {
    ur_type: Option<URType>,
    decoder: ur::Decoder,
}

impl MultipartDecoder {
    pub fn new() -> Self {
        Self { ur_type: None, decoder: ur::Decoder::default() }
    }
}

impl Default for MultipartDecoder {
    fn default() -> Self { Self::new() }
}

impl MultipartDecoder {
    pub fn receive(&mut self, value: &str) -> Result<()> {
        let decoded_type = Self::decode_type(value)?;
        if let Some(ur_type) = &self.ur_type {
            if ur_type != &decoded_type {
                return Err(Error::UnexpectedType(
                    ur_type.string().to_string(),
                    decoded_type.string().to_string(),
                ));
            }
        } else {
            self.ur_type = Some(decoded_type);
        }
        Ok(self.decoder.receive(value)?)
    }

    pub fn is_complete(&self) -> bool { self.decoder.complete() }

    pub fn message(&self) -> Result<Option<UR>> {
        let message_data = self.decoder.message()?;
        if let Some(data) = message_data {
            let cbor = CBOR::try_from_data(data)?;
            let ur_type = self.ur_type.as_ref().unwrap();
            let ur_type_string = ur_type.string();
            let ur = UR::new(ur_type_string, cbor)?;
            Ok(Some(ur))
        } else {
            Ok(None)
        }
    }

    fn decode_type(ur_string: &str) -> Result<URType> {
        let without_scheme =
            ur_string.strip_prefix("ur:").ok_or(Error::InvalidScheme)?;
        let first_component =
            without_scheme.split('/').next().ok_or(Error::InvalidType)?;
        URType::new(first_component)
    }
}
