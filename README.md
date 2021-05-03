# Rust library for interacting with CoreOS Stream metadata

See the [Fedora CoreOS documentation](https://docs.fedoraproject.org/en-US/fedora-coreos/getting-started/)
for basic information about streams.

This is a Rust library which defines standard structs that `#[derive(Deserialize)]`
for use with serde.

# Example usage

```
use anyhow::Result;
use coreos_stream_metadata::Stream;

#[tokio::main]
fn main() -> Result<()> {
  let streamid = coreos_stream_metadata::fcos::StreamId::Stable;
  let arch = "x86_64";
  let region = "us-east-1";
  let buf = reqwest::get(streamid.url())
    .await?
    .bytes()
    .await?;
  let stream: Stream = serde_json::from_slice(&buf)?;
  let ami = stream.architectures.get(arch).unwrap().images.get("aws").unwrap().regions(region).unwrap();
  println!("The AMI for FCOS {} {} is {} (version {})", streamid, region, ami.image, ami.release);
  Ok(())
}
```
