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

/// Alias for backward compatibility
pub type AwsImages = ReplicatedImage;

/// Alias for backward compatibility
pub type AwsRegionImage = SingleImage;

/// An image in all regions of an AWS-like cloud
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReplicatedImage {
    /// Mapping from region name to image
    pub regions: HashMap<String, SingleImage>,
}

/// An globally-accessible image or an image in a single region of an
/// AWS-like cloud
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct SingleImage {
    /// The release version of FCOS.
    pub release: String,
    /// Image reference
    pub image: String,
}

/// A tagged container image
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct ContainerImage {
    /// The release version of FCOS.
    pub release: String,
    /// Preferred way to reference the image, which might be by tag or digest
    pub image: String,
    /// Image reference by digest
    pub digest_ref: String,
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
    /// Images for Aliyun
    pub aliyun: Option<ReplicatedImage>,
    /// Images for AWS.
    pub aws: Option<ReplicatedImage>,
    /// Images for GCP.
    pub gcp: Option<GcpImage>,
    /// Objects for IBMCloud
    pub ibmcloud: Option<ReplicatedObject>,
    /// ContainerDisk for KubeVirt
    pub kubevirt: Option<ContainerImage>,
    /// Objects for PowerVS
    pub powervs: Option<ReplicatedObject>,
}

impl Stream {
    /// Returns the data for the CPU architecture matching the running process.
    pub fn this_architecture(&self) -> Option<&Arch> {
        self.architectures.get(this_architecture())
    }

    /// Find a `disk` artifact.
    pub fn query_disk(&self, arch: &str, artifact: &str, format_name: &str) -> Option<&Artifact> {
        self.architectures
            .get(arch)
            .and_then(|a| a.artifacts.get(artifact))
            .and_then(|p| p.formats.get(format_name))
            .and_then(|p| p.get("disk"))
    }

    /// Find the single `disk` image for this architecture of the given type.  Only use this
    /// for images which don't have multiple format.s
    pub fn query_thisarch_single(&self, artifact: &str) -> Option<&Artifact> {
        self.this_architecture()
            .and_then(|a| a.artifacts.get(artifact))
            .and_then(|p| p.formats.iter().next())
            .and_then(|(_fmt, v)| v.get("disk"))
    }
}

/// Return the RPM/GNU architecture identifier for the current binary.
///
/// See also https://github.com/coreos/stream-metadata-go/blob/c5fe1b98ac1b1e6ab62a606b7580dc1f30703f83/arch/arch.go
pub fn this_architecture() -> &'static str {
    match std::env::consts::ARCH {
        // Funny enough, PowerPC is so far the only weird case here.
        // For everything else, the Rust architecture is the same as RPM/GNU/Linux.
        "powerpc64" if cfg!(target_endian = "big") => "ppc64",
        "powerpc64" if cfg!(target_endian = "little") => "ppc64le",
        o => o,
    }
}
