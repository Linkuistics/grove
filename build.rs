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
//! The gate has two halves, built as two modules and run in that order. The
//! **per-file** half is everything a single `(path, text)` decides. The
//! **whole-embed** half — id uniqueness, `defers=` resolution and its class
//! check, procedural reachability — needs the assembled set, so it runs once
//! every file has parsed. Both are `#[path]`-included from `src/`, so neither
//! is a second implementation of anything.

// `Kind` alone is what the parser needs (a `kinds=` member is validated against
// the closed nineteen), but a module is the unit of inclusion; the rest of
// `leaf` is dead in a build script by construction.
#[allow(dead_code)]
#[path = "src/leaf.rs"]
mod leaf;
#[allow(dead_code)]
#[path = "src/methodology/parse.rs"]
mod parse;
// Reaches the parser as `super::parse`, which resolves to this script's root
// here and to `crate::methodology::parse` in the crate — one spelling, two
// module trees.
#[allow(dead_code)]
#[path = "src/methodology/whole_embed.rs"]
mod whole_embed;

use std::path::{Path, PathBuf};

/// What a contributor is told after a per-file failure: the grammar of the line
/// the error points at.
const MARKER_HELP: &[&str] = &[
    "The marker grammar is: <!-- unit: <id> kinds=<scope> class=<class> defers=<target> -->",
    "`kinds` is required on a triggering unit and forbidden on a procedural one,",
    "attributes are written in that fixed order, and every body byte of every file",
    "belongs to exactly one unit.",
];

/// ...and after a whole-embed failure, where the line the error points at is
/// well-formed and it is the *set* that is not. Repeating the marker grammar
/// there would send a contributor to look for a typo that is not present.
const EMBED_HELP: &[&str] = &[
    "Every unit id is unique across the whole embed, every `defers=` names a declared",
    "`class=procedural` unit, and every procedural unit is reached by following",
    "`defers=` from some triggering unit.",
];

fn main() {
    let content = Path::new("content");
    emit_rerun(content);
    // `#[path]`-included sources are invisible to Cargo's change tracking, and
    // emitting any `rerun-if-changed` at all opts out of the default
    // rerun-on-any-change. Without these, editing the parser would leave the
    // gate running the version compiled into the last build script.
    println!("cargo:rerun-if-changed=src/leaf.rs");
    println!("cargo:rerun-if-changed=src/methodology/parse.rs");
    println!("cargo:rerun-if-changed=src/methodology/whole_embed.rs");

    if let Err((failure, help)) = gate(content) {
        eprintln!("grove: the embedded methodology is malformed and will not be embedded.");
        eprintln!("  {failure}");
        for line in help {
            eprintln!("  {line}");
        }
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

/// Parse every embedded markdown file, then check the assembled set, reporting
/// the first malformation with the guidance that fits its half of the gate.
///
/// Sorted so the reported failure is the same one on every machine: a build
/// error that moves with directory iteration order is a build error two
/// contributors describe differently. The same sort is what makes this walk's
/// unit order match the embed walk's in `methodology::units`, so the two
/// traversals report a whole-embed failure at the same unit.
///
/// Every file is parsed before any of them is checked as a set, because a
/// whole-embed complaint about a file that does not parse would be a complaint
/// about a unit set the contributor has not written yet.
fn gate(root: &Path) -> Result<(), (String, &'static [&'static str])> {
    let mut files = Vec::new();
    collect_markdown(root, root, &mut files);
    files.sort();
    let mut units = Vec::new();
    for (relative, path) in files {
        let text = std::fs::read_to_string(&path)
            .map_err(|error| (format!("reading {}: {error}", path.display()), MARKER_HELP))?;
        units.extend(
            parse::parse_units(&relative, &text)
                .map_err(|error| (format!("{}/{error}", root.display()), MARKER_HELP))?,
        );
    }
    whole_embed::check(&units).map_err(|error| (format!("{}/{error}", root.display()), EMBED_HELP))
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
