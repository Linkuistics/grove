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
//! 3. [`the_exemption_is_the_promised_path_and_nothing_else`] — because a
//!    scan that skips a file reports what a scan that clears it reports, and
//!    the exemption is where files get skipped.
//! 4. A violation added by hand to a real module, watched failing, and removed.
//!    That was done when this test was written and again when `seam-k18`
//!    changed the detector; redo it if the mechanism changes again.

use std::fs;
use std::path::{Path, PathBuf};

/// The one path, relative to `src/`, that is allowed to name the filesystem:
/// the directory `src/fs/`, and the file `src/fs.rs` that would declare it.
const FILESYSTEM_MODULE: &str = "fs";

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Whether a source file, named relative to `src/`, is the module allowed to
/// name the filesystem.
///
/// By path, and not by file name. `seam-k17` found this written as *any
/// directory called `fs`, and any file called `fs.rs`*, which quietly exempts
/// `src/algebra/fs.rs` — a module in the middle of the algebra, free to hold
/// whatever it likes because of what it is called.
fn is_the_filesystem_module(relative: &Path) -> bool {
    let mut components = relative
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned());
    match components.next() {
        // `src/fs/…`: the directory, and everything under it.
        Some(first) if first == FILESYSTEM_MODULE => components.next().is_some(),
        // `src/fs.rs`: the file that declares it, and only at the top level.
        Some(first) => first == format!("{FILESYSTEM_MODULE}.rs") && components.next().is_none(),
        None => false,
    }
}

/// Every source file under `src/` that is not the one module allowed to name
/// the filesystem, as a path relative to `src/`.
fn algebra_sources(src: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, prefix: &Path, out: &mut Vec<PathBuf>) {
        let listing =
            fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
        for entry in listing {
            let path = entry.expect("a readable directory entry").path();
            let name = path.file_name().expect("a named entry").to_owned();
            let relative = prefix.join(&name);
            if path.is_dir() {
                walk(&path, &relative, out);
            } else if relative.extension().is_some_and(|e| e == "rs") {
                out.push(relative);
            }
        }
    }
    let mut found = Vec::new();
    walk(src, Path::new(""), &mut found);
    found.retain(|relative| !is_the_filesystem_module(relative));
    found.sort();
    found
}

/// What naming the filesystem looks like in source: the identifier **`fs`**, as
/// a whole word.
///
/// The word and not the two paths `fs::` and `std::fs`, which is what
/// `seam-k17` found this scanning for. Ordinary Rust writes the import a third
/// way — `use std::{fs, path::Path};` contains neither of those tokens, and
/// every `fs::…` call in the file then passes too, as does the aliased
/// `use std::{fs as f, …};`. A whole word catches the module however it is
/// spelled, at the cost of a false positive on a *binding* called `fs`, which
/// fails loudly and in the safe direction.
///
/// What still defeats it, stated accurately because a guard whose limits are
/// documented wrongly is worse than one with none: this reads text, not a
/// module graph. A path assembled by a macro, or reached through something
/// re-exported from outside `src/`, is invisible to it. It is a guard against
/// drift by a contributor who has not read this file, not against one working
/// to defeat it.
fn names_the_filesystem(text: &str) -> bool {
    let bytes = text.as_bytes();
    let word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut from = 0;
    while let Some(at) = text[from..].find(FILESYSTEM_MODULE) {
        let start = from + at;
        let end = start + FILESYSTEM_MODULE.len();
        let boundary_before = start == 0 || !word(bytes[start - 1]);
        let boundary_after = end >= bytes.len() || !word(bytes[end]);
        if boundary_before && boundary_after {
            return true;
        }
        from = end;
    }
    false
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
        .filter(|(_, line)| names_the_filesystem(line))
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
        // The grouped forms `seam-k17` found slipping past. The first is how
        // most Rust files actually import it; the second then makes every call
        // in the file read `f::…`, with the word `fs` nowhere near it.
        "use std::{fs, path::Path};\n",
        "use std::{fs as f, path::PathBuf};\n",
        "use std::{io::Write, fs::File};\n",
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

/// The exemption is a path and not a file name, which is the other way this
/// instrument can read clean while broken: a module called `fs` in the middle
/// of the algebra would be skipped by the scan and could then hold anything.
#[test]
fn the_exemption_is_the_promised_path_and_nothing_else() {
    for exempt in ["fs.rs", "fs/mod.rs", "fs/lock.rs", "fs/snapshot/read.rs"] {
        assert!(
            is_the_filesystem_module(Path::new(exempt)),
            "`src/{exempt}` is the filesystem module and should be exempt"
        );
    }
    for guarded in [
        "lib.rs",
        "name.rs",
        "algebra/fs.rs",
        "algebra/fs/plan.rs",
        "nested/deep/fs.rs",
    ] {
        assert!(
            !is_the_filesystem_module(Path::new(guarded)),
            "`src/{guarded}` is the algebra and must not be exempt"
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
            sources.iter().any(|p| p.as_os_str() == expected),
            "the scan missed {expected}"
        );
    }
    let bytes: usize = sources
        .iter()
        .map(|p| {
            fs::read_to_string(src_dir().join(p))
                .expect("a readable source file")
                .len()
        })
        .sum();
    assert!(bytes > 0, "the scan read no bytes");
}

/// The claim itself.
#[test]
fn the_algebra_cannot_reach_the_filesystem() {
    let mut offences = Vec::new();
    let src = src_dir();
    for path in algebra_sources(&src) {
        let source = fs::read_to_string(src.join(&path)).expect("a readable source file");
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
