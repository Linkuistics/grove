//! The embedded methodology, addressable by unit.
//!
//! `content/` is compiled into **both** binaries — `grove` to provision and
//! (later) to compose mandates, `grove-llm` to serve unit bytes to a session
//! that followed a `defers=` id. This module owns the embed, the reader over it
//! ([`parse`], shared with `build.rs`), and the build's [`identity`].
//!
//! The identity is a hash of the embed rather than a compile-time constant, and
//! that is a consequence of `grove-llm` linking `content/` at all. The constant
//! existed precisely so that *naming* the identity did not link the embed; once
//! the agent-facing binary links it anyway, both binaries hash it directly,
//! which removes a build-script traversal and the equality test that existed
//! only to keep two traversals in step
//! (`docs/adr/one-build-owns-a-session.md`).

mod parse;

// The reader's error type is deliberately **not** re-exported. It is what
// `build.rs` prints and what the parser's own tests match on, and no function
// out here returns it — `units` wraps it in `anyhow` — so exporting it would be
// public surface nothing outside the crate could reach a value of (`src/lib.rs`
// states the rule).
pub use parse::{Class, Scope, Unit};

use anyhow::{Context, Result};
use include_dir::{include_dir, Dir, DirEntry};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::OnceLock;

/// grove's full retained methodology, embedded at compile time. `build.rs` emits
/// `rerun-if-changed` for the tree so an edit to any file re-embeds.
static CONTENT: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/content");

/// The embed itself, for the one caller that needs whole files rather than
/// units: provisioning, which extracts the tree verbatim.
pub fn embed() -> &'static Dir<'static> {
    &CONTENT
}

/// Every unit in the embed, in a deterministic order: files by `content/`-relative
/// path, units by position within their file.
///
/// The build gate has already parsed the same corpus through the same reader, so
/// a failure here is a bug rather than a contributor's mistake — it is still
/// returned rather than panicked, because the callers are a CLI verb and a
/// library function, neither of which should abort a session over it.
pub fn units() -> Result<Vec<Unit>> {
    let mut files: Vec<(String, &'static str)> = Vec::new();
    collect_markdown(&CONTENT, &mut files)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let mut units = Vec::new();
    for (path, text) in &files {
        units.extend(parse::parse_units(path, text).with_context(|| {
            format!("the embedded methodology no longer parses; content/{path} is malformed")
        })?);
    }
    Ok(units)
}

/// This build's **methodology identity** — the content hash of its embedded
/// `content/`.
///
/// It covers the embedded **file payload** and not the embedded directory
/// structure. `include_dir` embeds a `DirEntry::Dir` for every directory, an
/// empty one included, and `docs/adr/one-build-owns-a-session.md` deliberately
/// excludes those: a directory with no files carries no methodology. The
/// accepted consequence is that an empty directory is not part of a build's
/// identity.
///
/// Computed once. Every caller wants the same string for the life of the
/// process, and the value is a pure function of a `static`.
pub fn identity() -> &'static str {
    static IDENTITY: OnceLock<String> = OnceLock::new();
    IDENTITY.get_or_init(|| content_hash(&CONTENT))
}

/// Collect `(content/-relative path, text)` for every embedded markdown file.
///
/// Markdown is the whole corpus the unit grammar governs; the embed's other
/// files are the licence notices under `LICENSES/`, which carry no methodology.
fn collect_markdown(dir: &'static Dir, out: &mut Vec<(String, &'static str)>) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(child) => collect_markdown(child, out)?,
            DirEntry::File(file) => {
                let path = file.path();
                if path.extension().is_some_and(|extension| extension == "md") {
                    let text = std::str::from_utf8(file.contents()).with_context(|| {
                        format!("embedded content/{} is not UTF-8", path.display())
                    })?;
                    out.push((path.to_string_lossy().into_owned(), text));
                }
            }
        }
    }
    Ok(())
}

/// A deterministic hash over the embedded tree — every file's relative path and
/// bytes, in sorted path order. Changes whenever any file is added, removed,
/// moved, or edited, so it faithfully names which embed a binary carries.
fn content_hash(dir: &Dir) -> String {
    let mut files: Vec<(&Path, &[u8])> = Vec::new();
    collect_files(dir, &mut files);
    files.sort_by(|a, b| a.0.cmp(b.0));
    hash_files(&files)
}

fn collect_files<'a>(dir: &'a Dir, out: &mut Vec<(&'a Path, &'a [u8])>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(d) => collect_files(d, out),
            DirEntry::File(f) => out.push((f.path(), f.contents())),
        }
    }
}

/// Hash a pre-sorted (path, bytes) list. Each field is length-prefixed so no
/// reshuffling of bytes across the path/content boundary can collide.
fn hash_files(files: &[(&Path, &[u8])]) -> String {
    let mut hasher = Sha256::new();
    for (path, bytes) in files {
        let p = path.to_string_lossy();
        hasher.update((p.len() as u64).to_le_bytes());
        hasher.update(p.as_bytes());
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_identity_is_stable_and_nonempty() {
        assert!(!identity().is_empty());
        assert_eq!(identity(), identity(), "hashing is deterministic");
    }

    #[test]
    fn hash_changes_when_a_file_is_edited() {
        let a: [(&Path, &[u8]); 1] = [(Path::new("a.md"), b"one")];
        let b: [(&Path, &[u8]); 1] = [(Path::new("a.md"), b"two")];
        assert_ne!(hash_files(&a), hash_files(&b));
    }

    #[test]
    fn hash_changes_when_a_file_is_added() {
        let a: [(&Path, &[u8]); 1] = [(Path::new("a.md"), b"x")];
        let b: [(&Path, &[u8]); 2] = [(Path::new("a.md"), b"x"), (Path::new("b.md"), b"y")];
        assert_ne!(hash_files(&a), hash_files(&b));
    }

    #[test]
    fn hash_is_not_confused_by_the_path_content_boundary() {
        // "ab" + "c" vs "a" + "bc": length-prefixing must keep these distinct.
        let a: [(&Path, &[u8]); 1] = [(Path::new("ab"), b"c")];
        let b: [(&Path, &[u8]); 1] = [(Path::new("a"), b"bc")];
        assert_ne!(hash_files(&a), hash_files(&b));
    }
}
