use anyhow::Result;

const STREAM_DATA: &[u8] = include_bytes!("fixtures/fcos-stream.json");

fn main() -> Result<()> {
    let st: stream_metadata::Stream = serde_json::from_slice(STREAM_DATA)?;
    assert_eq!(st.stream, "stable");
    assert_eq!(
        st.architectures
            .get("x86_64")
            .unwrap()
            .artifacts
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
    Ok(())
}
