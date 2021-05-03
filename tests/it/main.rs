use anyhow::Result;
use coreos_stream_metadata::Stream;
use coreos_stream_metadata::{fcos, AwsRegionImage};

const STREAM_DATA: &[u8] = include_bytes!("fixtures/fcos-stream.json");

#[test]
fn test_basic() -> Result<()> {
    assert_eq!(
        fcos::StreamID::Stable.url(),
        "https://builds.coreos.fedoraproject.org/streams/stable.json"
    );

    let un = nix::sys::utsname::uname();
    let myarch = un.machine();

    let st: Stream = serde_json::from_slice(STREAM_DATA)?;
    assert_eq!(st.stream, "stable");
    let a = st.architectures.get("x86_64").unwrap();
    if myarch == "x86_64" {
        assert!(st.this_architecture().is_some());
    }

    assert_eq!(
        a.artifacts
            .get("metal")
            .unwrap()
            .formats
            .get("raw.xz")
            .unwrap()
            .get("disk")
            .unwrap()
            .sha256,
        "2848b111a6917455686f38a3ce64d2321c33809b9cf796c5f6804b1c02d79d9d"
    );

    assert_eq!(
        a.images
            .as_ref()
            .unwrap()
            .aws
            .as_ref()
            .unwrap()
            .regions
            .get("us-east-1")
            .unwrap(),
        &AwsRegionImage {
            image: "ami-037a0ba6d14ca2e05".to_string(),
            release: "33.20201201.3.0".to_string(),
        }
    );

    Ok(())
}
