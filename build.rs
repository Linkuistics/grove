//! Re-embed `content/` whenever any file under it changes, and refuse to embed a
//! malformed one.
//!
//! `src/methodology.rs` pulls `content/` into both binaries with `include_dir!`,
//! but on stable Rust that macro does not register its file reads with Cargo's
//! change tracker — so editing a content file would otherwise leave a stale
//! embed baked into the binary. A bare `rerun-if-changed=content` only catches
//! adds/removes (the directory mtime), so we walk the tree and emit a hint per
//! file: any edit then forces a rebuild, which is what makes the binary's
//! embedded methodology faithfully match the working `content/`.
//!
//! The same walk **gates** the embed on the unit-marker grammar. A malformed
//! embed fails `cargo build`, because grove's own compile-time artifact is fully
//! observable by the build that produced it: a hard failure inconveniences a
//! contributor here, with the file and offset in hand, while a soft one ships a
//! binary that silently drops a triggering unit from every session it launches.
//! Constraint 5 — grove guides and does not gate — governs the *human's task
//! tree*, not this (`docs/specs/mandate-delivered-methodology.md`, *A malformed
//! embed fails the build*).
//!
//! **The gate reads through the crate's own parser, not a copy of it.** The two
//! modules below are `#[path]`-included rather than reimplemented, so a rule
//! learned in `src/` cannot be half-learned here. This script used to hash
//! `content/` as well, in a traversal written twice and held together by a
//! standing equality test; that duplication is exactly what is not repeated.
//! The hash is gone with the compile-time identity constant — `grove-llm` links
//! the embed now, so both binaries hash it directly.
//!
//! This gate is **per file**: everything a single `(path, text)` decides.
//! Whole-embed checks — id uniqueness, `defers=` resolution, procedural
//! reachability — need the assembled set and are built separately.

// `Kind` alone is what the parser needs (a `kinds=` member is validated against
// the closed nineteen), but a module is the unit of inclusion; the rest of
// `leaf` is dead in a build script by construction.
#[allow(dead_code)]
#[path = "src/leaf.rs"]
mod leaf;
#[allow(dead_code)]
#[path = "src/methodology/parse.rs"]
mod parse;

use std::path::{Path, PathBuf};

fn main() {
    let content = Path::new("content");
    emit_rerun(content);
    // `#[path]`-included sources are invisible to Cargo's change tracking, and
    // emitting any `rerun-if-changed` at all opts out of the default
    // rerun-on-any-change. Without these, editing the parser would leave the
    // gate running the version compiled into the last build script.
    println!("cargo:rerun-if-changed=src/leaf.rs");
    println!("cargo:rerun-if-changed=src/methodology/parse.rs");

    if let Err(failure) = gate(content) {
        eprintln!("grove: the embedded methodology is malformed and will not be embedded.");
        eprintln!("  {failure}");
        eprintln!(
            "  The marker grammar is: <!-- unit: <id> kinds=<scope> class=<class> defers=<target> -->"
        );
        eprintln!("  `kinds` is required on a triggering unit and forbidden on a procedural one,");
        eprintln!(
            "  attributes are written in that fixed order, and every body byte of every file"
        );
        eprintln!("  belongs to exactly one unit.");
        std::process::exit(1);
    }
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

/// Parse every embedded markdown file, reporting the first malformation.
///
/// Sorted so the reported failure is the same one on every machine: a build
/// error that moves with directory iteration order is a build error two
/// contributors describe differently.
fn gate(root: &Path) -> Result<(), String> {
    let mut files = Vec::new();
    collect_markdown(root, root, &mut files);
    files.sort();
    for (relative, path) in files {
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("reading {}: {error}", path.display()))?;
        parse::parse_units(&relative, &text)
            .map_err(|error| format!("{}/{error}", root.display()))?;
    }
    Ok(())
}

/// Collect `(path relative to `root`, absolute path)` for every markdown file
/// beneath `dir`, in the same shape `include_dir` reports paths — relative to
/// the embedded root — so a build error and a listing row name one file the same
/// way.
fn collect_markdown(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            collect_markdown(root, &path, out);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            out.push((relative.to_string_lossy().into_owned(), path));
        }
    }
}
