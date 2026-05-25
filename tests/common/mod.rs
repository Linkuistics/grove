use flate2::write::GzEncoder;
use flate2::Compression;
use tar::{Builder, Header};

/// Build a tarball that mirrors GitHub's archive layout:
/// grove-<tag>/content/<file> + grove-<tag>/Cargo.toml etc.
pub fn fixture_tarball(tag: &str, files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut gz = GzEncoder::new(Vec::new(), Compression::default());
    {
        let mut builder = Builder::new(&mut gz);
        for (path, content) in files {
            let full = format!("grove-{}/{}", tag, path);
            let mut header = Header::new_gnu();
            header.set_path(&full).unwrap();
            header.set_size(content.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            builder.append(&header, *content).unwrap();
        }
        builder.finish().unwrap();
    }
    gz.finish().unwrap()
}

use grove::fetch::Fetcher;

#[allow(dead_code)]
pub struct StubFetcher {
    pub latest: String,
    pub tarball: Vec<u8>,
}

impl Fetcher for StubFetcher {
    fn latest_version(&self) -> anyhow::Result<String> {
        Ok(self.latest.clone())
    }
    fn fetch_tarball(&self, _tag: &str) -> anyhow::Result<Vec<u8>> {
        Ok(self.tarball.clone())
    }
}
