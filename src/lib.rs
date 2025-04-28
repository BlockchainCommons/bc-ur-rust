#![doc(html_root_url = "https://docs.rs/bc-ur/0.8.0")]
#![warn(rust_2018_idioms)]

//! # Blockchain Commons Uniform Resources ("UR") for Rust
//!
//! [Uniform Resources
//! (URs)](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2020-005-ur.md)
//! are URI-encoded [CBOR](https://cbor.io) structures developed by [Blockchain
//! Commons](https://blockchaincommons.com). This crate is an opinionated
//! wrapper around the [ur](https://crates.io/crates/ur) crate by [Dominik
//! Spicher](https://github.com/dspicher), and is intended primarily for use in
//! higher-level Blockchain Commmons projects like [Gordian
//! Envelope](https://crates.io/crates/bc-envelope).
//!
//! It is a requirement of the UR specification that the CBOR encoded as URs
//! conform to Gordian dCBOR, which is a deterministic profile of CBOR currently
//! specified in [this IETF Internet
//! Draft](https://datatracker.ietf.org/doc/draft-mcnally-deterministic-cbor/).
//! The dependency `dcbor` crate can be used directly for that purpose. This
//! crate provides the traits `UREncodable`, `URDecodable`, and `URCodable` that
//! are built on traits from the `dcbor` crate such as `CBORTaggedEncodable` and
//! `CBORTaggedDecodable`. It is strongly recommended that adopters of URs
//! implement these traits for their types.
//!
//! This crate does not currenly provide opinionated affordances for multi-part
//! URs using fountain codes, but the dependency `ur` crate can be used directly
//! for that purpose.
//!
//! # Getting Started
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bc-ur = "0.8.0"
//! ```
//!
//! # Specification
//!
//! The primary specification for URs is [BCR-2020-005:
//! Uniform Resources](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2020-005-ur.md)
//! and the Swift implementation [URKit](https://github.com/BlockchainCommons/URKit).
//!
//! # Usage
//!
//! Encode a CBOR structure as a UR.
//!
//! ```
//! # fn main() {
//! # {
//! use dcbor::prelude::*;
//! use bc_ur::prelude::*;
//! let cbor: CBOR = vec![1, 2, 3].into();
//! let ur = UR::new("test", cbor).unwrap();
//! let ur_string = ur.string();
//! assert_eq!(ur_string, "ur:test/lsadaoaxjygonesw");
//! # }
//! # }
//! ```
//!
//! Decode a UR back to a CBOR structure.
//!
//! ```
//! # fn main() {
//! # {
//! use dcbor::prelude::*;
//! use bc_ur::prelude::*;
//! let ur_string = "ur:test/lsadaoaxjygonesw";
//! let ur = UR::from_ur_string(ur_string).unwrap();
//! assert_eq!(ur.ur_type_str(), "test");
//! let ur_cbor = ur.cbor();
//! let array_cbor: CBOR = vec![1, 2, 3].into();
//! assert_eq!(ur_cbor, array_cbor);
//! # }
//! # }
//! ```

mod ur;
pub use ur::UR;

pub mod bytewords;

mod ur_type;
pub use ur_type::URType;

mod error;
pub use error::{ Error, Result };

mod utils;
pub(crate) use utils::*;

mod ur_encodable;
pub use ur_encodable::UREncodable;

mod ur_decodable;
pub use ur_decodable::URDecodable;

mod ur_codable;
pub use ur_codable::URCodable;

mod multipart_decoder;
pub use multipart_decoder::MultipartDecoder;

mod multipart_encoder;
pub use multipart_encoder::MultipartEncoder;

pub mod prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn test_readme_deps() {
        version_sync::assert_markdown_deps_updated!("README.md");
    }

    #[test]
    fn test_html_root_url() {
        version_sync::assert_html_root_url_updated!("src/lib.rs");
    }
}

#[cfg(test)]
mod example_tests {
    use dcbor::prelude::*;
    use crate::*;

    #[test]
    fn encode() {
        let cbor: CBOR = vec![1, 2, 3].into();
        let ur = UR::new("test", cbor).unwrap();
        let ur_string = ur.string();
        assert_eq!(ur_string, "ur:test/lsadaoaxjygonesw");
    }

    #[test]
    fn decode() {
        let ur_string = "ur:test/lsadaoaxjygonesw";
        let ur = UR::from_ur_string(ur_string).unwrap();
        assert_eq!(ur.ur_type_str(), "test");
        assert_eq!(<UR as Into<CBOR>>::into(ur), <Vec<i32> as Into<CBOR>>::into(vec![1, 2, 3]));
    }

    fn run_fountain_test(start_part: usize) -> usize {
        let message = "The only thing we have to fear is fear itself.";
        let cbor = CBOR::to_byte_string(message.as_bytes());
        let ur = UR::new("bytes", cbor).unwrap();

        let mut encoder = MultipartEncoder::new(&ur, 10).unwrap();
        let mut decoder = MultipartDecoder::new();
        for _ in 0..1000 {
            let part = encoder.next_part().unwrap();
            if encoder.current_index() >= start_part {
                // println!("{}", part);
                decoder.receive(&part).unwrap();
            }
            if decoder.is_complete() {
                break;
            }
        }
        let received_ur = decoder.message().unwrap().unwrap();
        assert_eq!(received_ur, ur);
        encoder.current_index()
    }

    #[test]
    fn test_fountain() {
        assert_eq!(run_fountain_test(1), 5);
        assert_eq!(run_fountain_test(51), 61);
        assert_eq!(run_fountain_test(101), 110);
        assert_eq!(run_fountain_test(501), 507);
    }
}
