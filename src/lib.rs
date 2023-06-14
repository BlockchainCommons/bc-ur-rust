#![doc(html_root_url = "https://docs.rs/bc-ur/0.1.0")]
#![warn(rust_2018_idioms)]

mod ur;
pub use crate::ur::UR;

mod error;
mod utils;

mod ur_encodable;
pub use ur_encodable::UREncodable;

mod ur_decodable;
pub use ur_decodable::URDecodable;

mod ur_codable;
pub use ur_codable::URCodable;

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
