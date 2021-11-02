//! APIs for interacting specifically with RHEL CoreOS

use strum_macros::{Display, EnumString};

const INSTALLER_GIT: &str = "https://raw.githubusercontent.com/openshift/installer/";
const LEGACY_PATH: &str = "/data/data/rhcos-stream.json";
const PATH: &str = "/data/data/coreos/rhcos.json";

/// Well-known streams for RHEL CoreOS.
///
/// These map to OpenShift versions.  Only >= 4.8 has stream metadata.
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, Display)]
pub enum StreamID {
    /// 4.8
    #[strum(serialize = "4.8")]
    FourEight,
    /// 4.9
    #[strum(serialize = "4.9")]
    FourNine,
    /// 4.10
    #[strum(serialize = "4.10")]
    FourTen,
}

impl StreamID {
    /// Return the URL for this stream.
    pub fn url(&self) -> String {
        let (path, branchname) = match self {
            StreamID::FourEight => (LEGACY_PATH, "release-4.8"),
            StreamID::FourNine => (LEGACY_PATH, "release-4.9"),
            StreamID::FourTen => (PATH, "release-4.10"),
        };
        format!("{}{}{}", INSTALLER_GIT, branchname, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_rhcos_streamid() {
        assert_eq!(StreamID::FourEight.to_string(), "4.8");
        assert_eq!(StreamID::FourNine.to_string(), "4.9");
        assert_eq!(StreamID::FourTen.to_string(), "4.10");

        assert_eq!(StreamID::from_str("4.8").unwrap(), StreamID::FourEight);
        assert_eq!(StreamID::from_str("4.9").unwrap(), StreamID::FourNine);
        assert_eq!(StreamID::from_str("4.10").unwrap(), StreamID::FourTen);
        assert!(StreamID::from_str("foo").is_err());

        assert_eq!(StreamID::FourEight.url(), "https://raw.githubusercontent.com/openshift/installer/release-4.8/data/data/rhcos-stream.json");
        assert_eq!(StreamID::FourNine.url(), "https://raw.githubusercontent.com/openshift/installer/release-4.9/data/data/rhcos-stream.json");
        assert_eq!(StreamID::FourTen.url(), "https://raw.githubusercontent.com/openshift/installer/release-4.10/data/data/coreos/rhcos.json");
    }
}
