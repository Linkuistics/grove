//! The embedded methodology — the embed itself, and the build's methodology
//! identity.
//!
//! `content/` is compiled into **both** binaries — `grove` to provision it into
//! every installed harness's skill directory, `grove-llm` to hash it for the
//! build's [`identity`]. This module owns the embed and that identity, and
//! nothing else: the corpus is **plain markdown**, so there is no reader over
//! it and no grain finer than a file.
//!
//! **Nothing here composes a session's prompt.** The methodology reaches a
//! session as a *provisioned skill*, and `${prompt}` is the short guaranteed
//! core [`crate::prompt`] assembles — which depends on this module for the embed
//! rather than the other way round, so provisioning's supplier never sits behind
//! a prompt-composition seam (`docs/adr/skill-delivers-the-methodology.md`).
//!
//! The identity is a hash of the embed rather than a compile-time constant, and
//! that is a consequence of `grove-llm` linking `content/` at all. The constant
//! existed precisely so that *naming* the identity did not link the embed; once
//! the agent-facing binary links it anyway, both binaries hash it directly,
//! which removes a build-script traversal and the equality test that existed
//! only to keep two traversals in step
//! (`docs/adr/one-build-owns-a-session.md`).

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

/// Every embedded markdown file as `(content/-relative path, text)`, sorted by
/// path.
///
/// Markdown is the whole of the methodology a session reads; the embed's other
/// files are the licence notices under `LICENSES/`, which carry none. The sort
/// makes the walk deterministic, so a failure a caller reports names the same
/// file on every machine.
///
/// **Public as a seam**, on the exemption `src/lib.rs` names: the suite asserts
/// claims about the embed's prose — that it instructs no `grove-llm` verb the
/// CLI lacks — and production's own door onto that corpus is `include_dir`,
/// which a test cannot open without making a runtime dependency a dev one as
/// well. Handing the corpus out is cheaper than that, and strictly cheaper than
/// a third copy of this walk living in `tests/`.
pub fn markdown_files() -> Result<Vec<(String, &'static str)>> {
    let mut files: Vec<(String, &'static str)> = Vec::new();
    collect_markdown(&CONTENT, &mut files)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}

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
