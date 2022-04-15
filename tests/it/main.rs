use coreos_stream_metadata::Stream;
use coreos_stream_metadata::{fcos, ContainerImage, SingleImage};

const STREAM_DATA: &[u8] = include_bytes!("fixtures/fcos-stream.json");

#[test]
fn test_basic() {
    assert_eq!(
        fcos::StreamID::Stable.url(),
        "https://builds.coreos.fedoraproject.org/streams/stable.json"
    );

    let un = nix::sys::utsname::uname();
    let myarch = un.machine();

    let st: Stream = serde_json::from_slice(STREAM_DATA).unwrap();
    assert_eq!(st.stream, "stable");
    let a = st.architectures.get("x86_64").unwrap();
    if myarch == "x86_64" {
        assert!(st.this_architecture().is_some());
        assert_eq!(
            st.query_thisarch_single("qemu").unwrap().sha256,
            "a7e93e32665086d4a07a14dbe6c125177402f04603fc5bb575035028701afa5b"
        );
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
        st.query_disk("x86_64", "metal", "raw.xz").unwrap().sha256,
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
        &SingleImage {
            image: "ami-037a0ba6d14ca2e05".to_string(),
            release: "33.20201201.3.0".to_string(),
        }
    );

    assert_eq!(
        a.images
            .as_ref()
            .unwrap()
            .kubevirt
            .as_ref()
            .unwrap(),
        &ContainerImage {
            image: "quay.io/openshift-release-dev/rhcos:stable".to_string(),
            digest_ref: "quay.io/openshift-release-dev/rhcos@sha256:67a81539946ec0397196c145394553b8e0241acf27b14ae9de43bc56e167f773".to_string(),
            release: "33.20201201.3.0".to_string(),
        }
    );

    assert_eq!(
        a.images
            .as_ref()
            .unwrap()
            .aliyun
            .as_ref()
            .unwrap()
            .regions
            .get("us-east-1")
            .unwrap(),
        &SingleImage {
            image: "m-0xi29kf08acv9dps47zs".to_string(),
            release: "33.20201201.3.0".to_string(),
        }
    );
}
