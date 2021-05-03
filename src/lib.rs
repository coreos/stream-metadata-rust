//! Library for interacting with CoreOS stream metadata, used
//! by Fedora CoreOS and RHEL CoreOS.

#![deny(unused_must_use)]
#![deny(unsafe_code)]
#![forbid(missing_docs)]

use serde::Deserialize;
use std::collections::HashMap;

pub mod fcos;

/// Toplevel stream object.
#[derive(Debug, Deserialize)]
pub struct Stream {
    /// Name of the stream.
    pub stream: String,
    /// Architectures.
    pub architectures: HashMap<String, Arch>,
}

/// Artifacts for a particular architecture.
#[derive(Debug, Deserialize)]
pub struct Arch {
    /// Downloadable artifacts.
    pub artifacts: HashMap<String, Platform>,
    /// Images already uploaded to public clouds.
    pub images: Option<Images>,
}

/// A specific platform (e.g. `aws`, `gcp`)
#[derive(Debug, Deserialize)]
pub struct Platform {
    /// Specific formats.
    pub formats: HashMap<String, HashMap<String, Artifact>>,
}

/// A downloadable artifact with a URL and detached signature.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Artifact {
    /// The URL for this artifact.
    pub location: String,
    /// SHA-256 checksum.
    pub sha256: String,
    /// If the artifact is compressed, this is the uncompressed SHA-256.
    pub uncompressed_sha256: Option<String>,
    /// Detached GPG signature.
    pub signature: Option<String>,
}

/// Image for Amazon Web Services (EC2).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AwsImages {
    /// Mapping from region name to AMI.
    pub regions: HashMap<String, AwsRegionImage>,
}

/// A pair of an AWS image (AMI) and the release version.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct AwsRegionImage {
    /// The release version of FCOS.
    pub release: String,
    /// AMI (HVM).
    pub image: String,
}

/// Image stored in Google Compute Platform.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GcpImage {
    /// The project ID.
    pub project: String,
    /// The image family.
    pub family: String,
    /// The image name.
    pub name: String,
}

/// Public cloud images.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Images {
    /// Images for AWS.
    pub aws: Option<AwsImages>,
    /// Images for GCP.
    pub gcp: Option<GcpImage>,
}

impl Stream {
    /// Returns the data for the CPU architecture matching the running process.
    pub fn this_architecture(&self) -> Option<&Arch> {
        let un = nix::sys::utsname::uname();
        self.architectures.get(un.machine())
    }
}
