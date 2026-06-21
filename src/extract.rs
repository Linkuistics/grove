use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs;
use std::io::{Cursor, Read};
use std::path::Path;
use tar::Archive;

/// Extract `<tarball>/<root>/content/*` (where `<root>` is the single top-level
/// dir GitHub names after the tag) into `dest`.
pub fn extract_content(tarball: &[u8], dest: &Path) -> Result<()> {
    let gz = GzDecoder::new(Cursor::new(tarball));
    let mut archive = Archive::new(gz);

    fs::create_dir_all(dest).with_context(|| format!("creating dest dir {}", dest.display()))?;

    for entry in archive.entries().context("reading tar entries")? {
        let mut entry = entry.context("reading tar entry")?;
        let entry_path = entry.path().context("decoding entry path")?.to_path_buf();
        // entry_path looks like: grove-0.1.0/content/SKILL.md
        // Strip the first two components to get the path relative to content/.
        let mut comps = entry_path.components();
        comps.next(); // strip "grove-<tag>"
        let kind = comps.next();
        if kind.map(|c| c.as_os_str() != "content").unwrap_or(true) {
            continue; // not under content/
        }
        let rel: std::path::PathBuf = comps.collect();
        if rel.as_os_str().is_empty() {
            continue; // the content/ directory itself
        }
        let out_path = dest.join(&rel);
        if entry.header().entry_type().is_dir() {
            fs::create_dir_all(&out_path).ok();
            continue;
        }
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let mut bytes = Vec::new();
        entry
            .read_to_end(&mut bytes)
            .context("reading entry bytes")?;
        fs::write(&out_path, bytes).with_context(|| format!("writing {}", out_path.display()))?;
    }
    Ok(())
}
