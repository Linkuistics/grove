//! Re-embed `content/` whenever any file under it changes, and name the embed.
//!
//! `src/provision.rs` pulls `content/` into the binary with `include_dir!`, but
//! on stable Rust that macro does not register its file reads with Cargo's
//! change tracker — so editing a content file would otherwise leave a stale
//! embed baked into the binary. A bare `rerun-if-changed=content` only catches
//! adds/removes (the directory mtime), so we walk the tree and emit a hint per
//! file: any edit then forces a rebuild, which is what makes the binary's
//! embedded skill faithfully match the working `content/`.
//!
//! The same walk emits the build's **methodology identity** as
//! `GROVE_CONTENT_HASH`, a compile-time constant every binary in the crate can
//! read (`provision::METHODOLOGY_IDENTITY`) without linking the embed — only
//! `grove` extracts content, so only `grove` should carry it
//! (`docs/adr/one-build-owns-a-session.md`).
//!
//! **The identity is the file payload, never the directory structure.**
//! `include_dir` also embeds a `DirEntry::Dir` for every directory, an empty one
//! included, and the identity deliberately excludes those: every directory
//! holding a file is already named by that file's path, and hashing typed
//! directory entries would oblige this script to reproduce `include_dir`'s
//! directory semantics on top of its file selection — doubling the surface the
//! one equality test exists to keep from drifting. So neither this traversal nor
//! `provision::content_hash` should start hashing directory entries; the
//! accepted consequence is that an empty directory is not part of a build's
//! identity.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

fn main() {
    let content = Path::new("content");
    emit_rerun(content);
    println!(
        "cargo:rustc-env=GROVE_CONTENT_HASH={}",
        content_hash(content)
    );
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

/// The hash `provision::content_hash` computes over the same tree once
/// `include_dir!` has embedded it: every file's path relative to `content/` and
/// its bytes, sorted by path, each field `u64`-LE length-prefixed, SHA-256,
/// lowercase hex. The two traversals must agree byte for byte, and an in-crate
/// test asserts exactly that against the linked embed.
fn content_hash(root: &Path) -> String {
    let mut files = Vec::new();
    collect_files(root, root, &mut files);
    files.sort();
    let mut hasher = Sha256::new();
    for (path, bytes) in &files {
        hasher.update((path.len() as u64).to_le_bytes());
        hasher.update(path.as_bytes());
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    format!("{:x}", hasher.finalize())
}

/// Collect `(path relative to `root`, bytes)` for every file beneath `dir`.
///
/// Paths are rendered exactly as `include_dir` reports them — relative to the
/// embedded root, with the platform separator — because they are hashed, not
/// merely compared here.
fn collect_files(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect_files(root, &path, out);
        } else if let Ok(bytes) = std::fs::read(&path) {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            out.push((relative.to_string_lossy().into_owned(), bytes));
        }
    }
}
