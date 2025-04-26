use crate::{ UR, Result };

pub struct MultipartEncoder<'a> {
    encoder: ur::Encoder<'a>,
}

impl<'a> MultipartEncoder<'a> {
    pub fn new(ur: &'a UR, max_fragment_len: usize) -> Result<Self> {
        Ok(Self {
            encoder: ur::Encoder::new(
                &ur.cbor().to_cbor_data(),
                max_fragment_len,
                ur.ur_type_str()
            )?,
        })
    }

    pub fn next_part(&mut self) -> Result<String> {
        Ok(self.encoder.next_part()?)
    }

    pub fn current_index(&self) -> usize {
        self.encoder.current_index()
    }

    pub fn parts_count(&self) -> usize {
        self.encoder.fragment_count()
    }
}
