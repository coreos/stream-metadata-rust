//! Library for interacting with CoreOS stream metadata, used
//! by Fedora CoreOS and RHEL CoreOS.
//!
//!
//! # Get the URL for FCOS stable stream:
//!
//! ```no_run
//! use coreos_stream_metadata::fcos;
//! let url = fcos::StreamID::Stable.url();
//! ```
//!
//! # Deserialize stream data and print URL for OpenStack image
//!
//! ```no_run
//! use coreos_stream_metadata::Stream;
//!
//! let stream: Stream = serde_json::from_reader(std::io::stdin())?;
//! let openstack = stream.query_thisarch_single("openstack").ok_or_else(|| anyhow::anyhow!("Missing openstack image"))?;
//! println!("OpenStack image URL: {}", openstack.location);
//! # Ok::<(), anyhow::Error>(())
//! ```

#![deny(unused_must_use)]
#![deny(unsafe_code)]
#![forbid(missing_docs)]

use serde::Deserialize;
use std::collections::HashMap;

pub mod fcos;
pub mod rhcos;

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
    /// The release version of FCOS.
    // Legacy metadata doesn't have this
    pub release: Option<String>,
    /// The project ID.
    pub project: String,
    /// The image family.
    pub family: Option<String>,
    /// The image name.
    pub name: String,
}

/// Objects in an object store for each region, such as on IBMCloud or PowerVS.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReplicatedObject {
    /// Mapping from region name to the object.
    pub regions: HashMap<String, RegionObject>,
}

/// ContainerDisk for KubeVirt
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct KubeVirtContainerDisk {
    /// image reference to the container disk in a container registry
    pub image: String,
}

/// Region-specific object in an object store, such as on IBMCloud or PowerVS.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RegionObject {
    /// The release version of FCOS.
    pub release: String,
    /// The name of the object in the object store.
    pub object: String,
    /// The bucket where the object resides.
    pub bucket: String,
    /// The url of the object.
    pub url: String,
}

/// Public cloud images.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Images {
    /// Images for AWS.
    pub aws: Option<AwsImages>,
    /// Images for GCP.
    pub gcp: Option<GcpImage>,
    /// Objects for IBMCloud
    pub ibmcloud: Option<ReplicatedObject>,
    /// Objects for PowerVS
    pub powervs: Option<ReplicatedObject>,
    /// ContainerDisk for KubeVirt
    pub kubevirt: Option<KubeVirtContainerDisk>,
}

impl Stream {
    /// Returns the data for the CPU architecture matching the running process.
    pub fn this_architecture(&self) -> Option<&Arch> {
        let un = nix::sys::utsname::uname();
        self.architectures.get(un.machine())
    }

    /// Find a `disk` artifact.
    pub fn query_disk(&self, arch: &str, artifact: &str, format_name: &str) -> Option<&Artifact> {
        self.architectures
            .get(arch)
            .map(|a| a.artifacts.get(artifact))
            .flatten()
            .map(|p| p.formats.get(format_name))
            .flatten()
            .map(|p| p.get("disk"))
            .flatten()
    }

    /// Find the single `disk` image for this architecture of the given type.  Only use this
    /// for images which don't have multiple format.s
    pub fn query_thisarch_single(&self, artifact: &str) -> Option<&Artifact> {
        self.this_architecture()
            .map(|a| a.artifacts.get(artifact))
            .flatten()
            .map(|p| p.formats.iter().next())
            .flatten()
            .map(|(_fmt, v)| v.get("disk"))
            .flatten()
    }
}
