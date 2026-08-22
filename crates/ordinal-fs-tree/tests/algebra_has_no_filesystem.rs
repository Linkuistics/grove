//! The algebra cannot reach the filesystem.
//!
//! Every decision the library makes — which siblings move, in what order, what
//! each is renamed to — is a pure function of the snapshot, so it is testable
//! without a directory and modellable without an abstraction of one. Inside one
//! crate that is a convention, and a convention is not a seam: this test is what
//! makes it one, and it is what keeps a later split of the crate into
//! separately-modellable units mechanical.
//!
//! The rule it holds: **`src/fs/` may name the filesystem and no other module
//! may.** A new module is inside the algebra by default, which is the direction
//! that fails safe — a later leaf that adds a pure module gets the guarantee
//! without having to remember to ask for it.
//!
//! # This instrument reports clean when it is broken
//!
//! A scan that finds no files reports exactly what a scan that finds no
//! violations reports. Three separate instruments in this workstream have
//! already done that — a model, a tool and a runner each reporting *found
//! nothing* and *succeeded* with the same bytes (`docs/formalism-findings.md`,
//! entry 003) — so the guard here is not "it passes". It is:
//!
//! 1. [`the_detector_fires_on_a_deliberate_violation`] — the positive control,
//!    on synthetic sources, so the detector is known to be able to say no.
//! 2. [`the_scan_reaches_the_sources`] — the coverage control, so a scan that
//!    walked nothing fails rather than passes.
//! 3. A violation added by hand to a real module, watched failing, and removed.
//!    That was done when this test was written; redo it if the mechanism
//!    changes.

use std::fs;
use std::path::{Path, PathBuf};

/// The directory that is allowed to name the filesystem, relative to `src/`.
const FILESYSTEM_MODULE: &str = "fs";

/// What naming the filesystem looks like in source. `fs::` catches a call
/// through any import of the module; `std::fs` catches the import itself.
///
/// The scan reads text, not a module graph, so `use std::fs as f;` would slip
/// past it. That is a known limit and it fails in the safe direction: the guard
/// is against drift by a contributor who has not read this file, not against one
/// working to defeat it. A false *positive* — the tokens appearing in a string
/// literal — is possible and is also the safe direction, since it fails loudly.
const FILESYSTEM_TOKENS: &[&str] = &["fs::", "std::fs"];

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Every source file under `src/` that is not in the one module allowed to name
/// the filesystem.
fn algebra_sources(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let listing = fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
    for entry in listing {
        let path = entry.expect("a readable directory entry").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        if path.is_dir() {
            if name != FILESYSTEM_MODULE {
                found.extend(algebra_sources(&path));
            }
        } else if name.ends_with(".rs") && name != format!("{FILESYSTEM_MODULE}.rs") {
            found.push(path);
        }
    }
    found.sort();
    found
}

/// Strip line and block comments, so a doc comment that *discusses* the
/// filesystem — this crate's do — is not mistaken for one that reaches it.
/// Rust's block comments nest, so the depth is counted rather than flagged.
fn without_comments(source: &str) -> String {
    let bytes: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut depth = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        let two = (bytes.get(i), bytes.get(i + 1));
        match two {
            (Some('/'), Some('*')) => {
                depth += 1;
                i += 2;
            }
            (Some('*'), Some('/')) if depth > 0 => {
                depth -= 1;
                i += 2;
            }
            (Some('/'), Some('/')) if depth == 0 => {
                while i < bytes.len() && bytes[i] != '\n' {
                    i += 1;
                }
            }
            (Some(c), _) => {
                if depth == 0 {
                    out.push(*c);
                }
                i += 1;
            }
            (None, _) => break,
        }
    }
    out
}

/// The lines of `source` that name the filesystem, one-indexed.
fn violations(source: &str) -> Vec<(usize, String)> {
    without_comments(source)
        .lines()
        .enumerate()
        .filter(|(_, line)| FILESYSTEM_TOKENS.iter().any(|t| line.contains(t)))
        .map(|(n, line)| (n + 1, line.trim().to_string()))
        .collect()
}

/// The positive control. A detector that cannot say no has never said yes.
#[test]
fn the_detector_fires_on_a_deliberate_violation() {
    for source in [
        "use std::fs;\n",
        "use std::fs::read_dir;\n",
        "    let entries = std::fs::read_dir(root)?;\n",
        "    fs::rename(from, to)?;\n",
        "use std::os::unix::fs::symlink;\n",
    ] {
        assert!(
            !violations(source).is_empty(),
            "the detector missed: {source}"
        );
    }
}

/// The other half of the control: prose about the filesystem is not a use of
/// it, or every doc comment in this crate would be a violation and the test
/// would have to be weakened until it caught nothing.
#[test]
fn the_detector_ignores_a_mention_in_a_comment() {
    for source in [
        "// the algebra never calls std::fs\n",
        "/// See `std::fs` — which this module does not use.\n",
        "//! `fs::rename` lives one layer down.\n",
        "/* std::fs::read_dir /* nested */ */\nlet x = 1;\n",
    ] {
        assert!(
            violations(source).is_empty(),
            "the detector false-positived on: {source}"
        );
    }
}

/// The coverage control. A scan that reached nothing must fail, not pass — this
/// is the assertion that distinguishes a green suite from a dead one.
#[test]
fn the_scan_reaches_the_sources() {
    let sources = algebra_sources(&src_dir());
    assert!(
        sources.len() >= 4,
        "the scan found {} source file(s); the crate has more than that",
        sources.len()
    );
    for expected in ["lib.rs", "name.rs", "conformance.rs", "reference.rs"] {
        assert!(
            sources
                .iter()
                .any(|p| p.file_name().and_then(|n| n.to_str()) == Some(expected)),
            "the scan missed {expected}"
        );
    }
    let bytes: usize = sources
        .iter()
        .map(|p| fs::read_to_string(p).expect("a readable source file").len())
        .sum();
    assert!(bytes > 0, "the scan read no bytes");
}

/// The claim itself.
#[test]
fn the_algebra_cannot_reach_the_filesystem() {
    let mut offences = Vec::new();
    for path in algebra_sources(&src_dir()) {
        let source = fs::read_to_string(&path).expect("a readable source file");
        for (line, text) in violations(&source) {
            offences.push(format!("{}:{line}: {text}", path.display()));
        }
    }
    assert!(
        offences.is_empty(),
        "the algebra names the filesystem. Move this into `src/{FILESYSTEM_MODULE}/`:\n{}",
        offences.join("\n")
    );
}
