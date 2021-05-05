//! APIs for interacting specifically with RHEL CoreOS

use strum_macros::{Display, EnumString};

const INSTALLER_GIT: &str = "https://raw.githubusercontent.com/openshift/installer/";
const PATH: &str = "/data/data/rhcos-stream.json";

/// Well-known streams for RHEL CoreOS.
///
/// These map to OpenShift versions.  Only 4.8 at this moment has stream data.
#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumString, Display)]
pub enum StreamID {
    /// 4.8
    #[strum(serialize = "rhcos-4.8")]
    FourEight,
}

impl StreamID {
    /// Return the URL for this stream.
    pub fn url(&self) -> String {
        let branchname = match self {
            StreamID::FourEight => "master",
        };
        format!("{}{}{}", INSTALLER_GIT, branchname, PATH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_rhcos_streamid() {
        assert_eq!(StreamID::FourEight.to_string(), "rhcos-4.8");
        assert_eq!(
            StreamID::from_str("rhcos-4.8").unwrap(),
            StreamID::FourEight
        );
        assert!(StreamID::from_str("foo").is_err());
    }
}
