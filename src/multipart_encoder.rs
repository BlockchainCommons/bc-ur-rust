use ur::Encoder;
use anyhow::Result;

use crate::UR;

pub struct MultipartEncoder<'a> {
    encoder: Encoder<'a>,
}

impl<'a> MultipartEncoder<'a> {
    pub fn new(ur: &'a UR, max_fragment_len: usize) -> Result<Self> {
        Ok(Self {
            encoder: Encoder::new(&ur.cbor().to_cbor_data(), max_fragment_len, ur.ur_type_str())
                .map_err(|e| anyhow::Error::msg(e.to_string()))?,
        })
    }

    pub fn next_part(&mut self) -> Result<String> {
        self.encoder.next_part()
            .map_err(|e| anyhow::Error::msg(e.to_string()))
    }

    pub fn current_index(&self) -> usize {
        self.encoder.current_index()
    }

    pub fn parts_count(&self) -> usize {
        self.encoder.fragment_count()
    }
}
