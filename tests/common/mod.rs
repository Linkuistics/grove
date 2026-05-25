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
