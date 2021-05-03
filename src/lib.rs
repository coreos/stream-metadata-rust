//! Library for interacting with CoreOS stream metadata, used
//! by Fedora CoreOS and RHEL CoreOS.

#[deny(unused_must_use)]
#[deny(unsafe_code)]
#[forbid(missing_docs)]
use serde::Deserialize;
use std::collections::HashMap;

pub mod fcos;

/// Toplevel stream object.
#[derive(Debug, Deserialize)]
pub struct Stream {
    pub stream: String,
    pub architectures: HashMap<String, Arch>,
}

/// Artifacts for a particular architecture.
#[derive(Debug, Deserialize)]
pub struct Arch {
    pub artifacts: HashMap<String, Platform>,
    pub images: Option<Images>,
}

/// A specific platform (e.g. `aws`, `gcp`)
#[derive(Debug, Deserialize)]
pub struct Platform {
    pub formats: HashMap<String, HashMap<String, Artifact>>,
}

/// A downloadable artifact with a URL and detached signature.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Artifact {
    pub location: String,
    pub sha256: String,
    pub uncompressed_sha256: Option<String>,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AwsImages {
    pub regions: HashMap<String, AwsRegionImage>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct AwsRegionImage {
    pub release: String,
    pub image: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GcpImage {
    pub project: String,
    pub family: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Images {
    pub aws: Option<AwsImages>,
    pub gcp: Option<GcpImage>,
}
