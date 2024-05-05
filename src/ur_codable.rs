use crate::{UREncodable, URDecodable};

/// A type that can be encoded to and decoded from a UR.
pub trait URCodable {}

impl<T> URCodable for T where T: UREncodable + URDecodable { }

#[cfg(test)]
mod tests {
    use dcbor::{CBOR, Tag, CBORTaggedEncodable, CBORTaggedDecodable, CBORTagged};

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
        fn cbor_tags() -> Vec<Tag> {
            vec![Tag::new_with_name(24, "leaf")]
        }
    }

    impl From<Test> for CBOR {
        // This ensures that asking for the CBOR for this type will always
        // return a tagged CBOR value.
        fn from(value: Test) -> Self {
            value.tagged_cbor()
        }
    }

    impl CBORTaggedEncodable for Test {
        // This is the core of the CBOR encoding for this type. It is the
        // untagged CBOR encoding.
        fn untagged_cbor(&self) -> CBOR {
            self.s.clone().into()
        }
    }

    impl TryFrom<CBOR> for Test {
        type Error = anyhow::Error;

        // This ensures that asking for the CBOR for this type will always
        // expect a tagged CBOR value.
        fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
            Self::from_tagged_cbor(cbor)
        }
    }

    impl CBORTaggedDecodable for Test {
        // This is the core of the CBOR decoding for this type. It is the
        // untagged CBOR decoding.
        fn from_untagged_cbor(cbor: CBOR) -> anyhow::Result<Self> {
            let s: String = cbor.try_into()?;
            Ok(Self::new(&s))
        }
    }

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
