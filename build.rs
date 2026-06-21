//! Re-embed `content/` whenever any file under it changes.
//!
//! `src/provision.rs` pulls `content/` into the binary with `include_dir!`, but
//! on stable Rust that macro does not register its file reads with Cargo's
//! change tracker — so editing a content file would otherwise leave a stale
//! embed baked into the binary. A bare `rerun-if-changed=content` only catches
//! adds/removes (the directory mtime), so we walk the tree and emit a hint per
//! file: any edit then forces a rebuild, which is what makes the binary's
//! embedded skill faithfully match the working `content/`.

use std::path::Path;

fn main() {
    emit_rerun(Path::new("content"));
}

fn emit_rerun(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                emit_rerun(&entry.path());
            }
        }
    }
}
