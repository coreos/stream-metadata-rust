//! APIs for interacting specifically with Fedora CoreOS

use std::fmt::Display;

/// Base URL to Fedora CoreOS streams metadata.
pub const STREAM_BASE_URL: &str = "https://builds.coreos.fedoraproject.org/streams/";

/// Well-known streams for Fedora CoreOS.
///
/// For more information, see https://docs.fedoraproject.org/en-US/fedora-coreos/update-streams/
pub enum Stream {
    Stable,
    Testing,
    Next,
}

impl Display for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Stream::Stable => "stable",
            Stream::Testing => "testing",
            Stream::Next => "next",
        })
    }
}

impl Stream {
    /// Return the URL for this stream.
    pub fn url(&self) -> String {
        format!("{}/{}.json", STREAM_BASE_URL, self)
    }
}