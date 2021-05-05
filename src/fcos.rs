//! APIs for interacting specifically with Fedora CoreOS

use strum_macros::{Display, EnumString};

/// Base URL to Fedora CoreOS streams metadata.
pub const STREAM_BASE_URL: &str = "https://builds.coreos.fedoraproject.org/streams/";

/// Well-known streams for Fedora CoreOS.
///
/// For more information, see https://docs.fedoraproject.org/en-US/fedora-coreos/update-streams/
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum StreamID {
    /// The stable stream.
    Stable,
    /// The testing stream.
    Testing,
    /// The next stream.
    Next,
}

impl StreamID {
    /// Return the URL for this stream.
    pub fn url(&self) -> String {
        format!("{}{}.json", STREAM_BASE_URL, self)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_rhcos_streamid() {
        assert_eq!(StreamID::Stable.to_string(), "stable");
        assert_eq!(StreamID::from_str("testing").unwrap(), StreamID::Testing);
        assert!(StreamID::from_str("foo").is_err());
    }
}
