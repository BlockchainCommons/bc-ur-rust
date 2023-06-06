use crate::{UREncodable, URDecodable};

pub trait URCodable: UREncodable + URDecodable {}

#[cfg(test)]
mod tests {
    use dcbor::{CBOR, Tag, CBOREncodable, CBORTaggedEncodable, CBORDecodable, Error as CBORError, CBORTaggedDecodable, CBORTagged};

    use super::*;

    struct Test {
        s: String,
    }

    impl Test {
        fn new(s: &str) -> Self {
            Self { s: s.to_string() }
        }
    }

    impl CBORTagged for Test {
        const CBOR_TAG: Tag = Tag::new_with_static_name(24, "leaf");
    }

    impl CBOREncodable for Test {
        // This ensures that asking for the CBOR for this type will always
        // return a tagged CBOR value.
        fn cbor(&self) -> CBOR {
            self.tagged_cbor()
        }
    }

    impl CBORTaggedEncodable for Test {
        // This is the core of the CBOR encoding for this type. It is the
        // untagged CBOR encoding.
        fn untagged_cbor(&self) -> CBOR {
            self.s.cbor()
        }
    }

    impl UREncodable for Test {}

    impl CBORDecodable for Test {
        // This ensures that asking for the CBOR for this type will always
        // expect a tagged CBOR value.
        fn from_cbor(cbor: &CBOR) -> Result<Self, CBORError> {
            Self::from_tagged_cbor(cbor)
        }
    }

    impl CBORTaggedDecodable for Test {
        // This is the core of the CBOR decoding for this type. It is the
        // untagged CBOR decoding.
        fn from_untagged_cbor(cbor: &CBOR) -> Result<Self, CBORError> {
            Ok(Self::new(&String::from_cbor(cbor)?))
        }
    }

    impl URDecodable for Test {}

    impl URCodable for Test {}

    #[test]
    fn test_ur_codable() {
        let test = Test::new("test");
        let ur = test.ur();
        let ur_string = ur.string();
        assert_eq!(ur_string, "ur:leaf/iejyihjkjygupyltla");
        let test2 = Test::from_ur_string(ur_string).unwrap();
        assert_eq!(test.s, test2.s);
    }
}
