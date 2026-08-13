//! The embedded methodology, addressable by unit.
//!
//! `content/` is compiled into **both** binaries — `grove` to provision and to
//! compose mandates, `grove-llm` to serve unit bytes to a session that followed
//! a `defers=` id. This module owns the embed, the two readers over it
//! ([`parse`] and [`whole_embed`], both shared with `build.rs`), the
//! [`compose`] projection a mandate is built from, and the build's [`identity`].
//!
//! [`compose`] lives **here rather than in a third submodule**, and the
//! asymmetry is deliberate: the two readers are files because they are
//! `#[path]`-included into the build script, so a third file beside them would
//! imply a sharing that does not exist. The build gate needs no composer — it
//! decides whether the embed is well-formed, not what any session receives.
//!
//! The identity is a hash of the embed rather than a compile-time constant, and
//! that is a consequence of `grove-llm` linking `content/` at all. The constant
//! existed precisely so that *naming* the identity did not link the embed; once
//! the agent-facing binary links it anyway, both binaries hash it directly,
//! which removes a build-script traversal and the equality test that existed
//! only to keep two traversals in step
//! (`docs/adr/one-build-owns-a-session.md`).

mod parse;
mod whole_embed;

// Neither reader's error type is re-exported. They are what `build.rs` prints
// and what each module's own tests match on, and no function out here returns
// one — `units` wraps both in `anyhow` — so exporting them would be public
// surface nothing outside the crate could reach a value of (`src/lib.rs` states
// the rule).
pub use parse::{Class, Scope, Unit};

use crate::leaf::Kind;

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
/// That order is the one the whole-embed check reports failures in, and it is
/// the order `build.rs` assembles too, so a malformation is named identically by
/// the gate and by a verb.
///
/// The build gate has already parsed and checked the same corpus through the
/// same two readers, so a failure here is a bug rather than a contributor's
/// mistake — it is still returned rather than panicked, because the callers are
/// a CLI verb and a library function, neither of which should abort a session
/// over it. Re-running the whole-embed check costs a walk of a handful of units
/// and makes the invariant true **where a consumer reads it**: the gate ran over
/// `content/` on disk, and this is the linked embed, which is a different
/// traversal of what ought to be the same tree.
pub fn units() -> Result<Vec<Unit>> {
    let mut units = Vec::new();
    for (path, text) in markdown_files()? {
        units.extend(parse::parse_units(&path, text).with_context(|| {
            format!("the embedded methodology no longer parses; content/{path} is malformed")
        })?);
    }
    whole_embed::check(&units)
        .map_err(|error| anyhow::anyhow!("content/{error}"))
        .context("the embedded methodology is internally inconsistent")?;
    Ok(units)
}

/// The methodology a session of `kind` is handed: every triggering unit whose
/// scope admits that kind, byte-exact, in mandate order, joined by blank lines.
///
/// **This returns the composed slices and nothing else.** The two facts a driver
/// resolves at runtime — the selected leaf's stable handle and the stated
/// version control — are appended by the driver, because they are not
/// expressible in `content/` and therefore have no unit to be a slice of. That
/// is also what the 64 KiB classification alarm counts: the composer's bytes,
/// which are deterministic, rather than a session's, which do not exist yet.
///
/// Three rules, each of which is a decision rather than an implementation
/// detail (`docs/specs/mandate-delivered-methodology.md`, *The driver authors
/// mandate prose only for facts it resolves at runtime*):
///
/// * **Selection is the scope and nothing else.** Triggering units whose scope
///   admits `kind`; no procedural unit, whatever it defers from. A procedural
///   body reaches a session through `grove-llm methodology` or not at all.
/// * **Order is `(file position, offset within the file)`.** The framing unit
///   needs no rule of its own — it leads because `content/MANDATE.md` is ordered
///   first — which is what keeps the composer free of any knowledge about which
///   unit is which. The sort is performed here rather than assumed of the
///   caller: [`units`] is assembled in *path* order, which is what the two
///   readers report failures in, and the two orders are unrelated on purpose.
/// * **Slices are joined by a blank line and nothing else.** Every unit's source
///   ends in a newline — the build rejects a file that does not, so that a
///   multi-unit fetch keeps each marker on its own line — so one `\n` between
///   adjacent sources is exactly one blank line. Any framing sentence here would
///   be methodology living in Rust, which is the drift a byte-exact projection
///   exists to make impossible.
pub fn compose(units: &[Unit], kind: Kind) -> String {
    let mut selected: Vec<&Unit> = units
        .iter()
        .filter(|unit| unit.class == Class::Triggering)
        .filter(|unit| unit.scope.as_ref().is_some_and(|scope| scope.admits(kind)))
        .collect();
    selected.sort_by_key(|unit| (unit.file_order, unit.offset));
    selected
        .iter()
        .map(|unit| unit.source.as_str())
        .collect::<Vec<_>>()
        .join("\n")
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
/// Markdown is the whole corpus the unit grammar governs; the embed's other
/// files are the licence notices under `LICENSES/`, which carry no methodology.
/// The sort is what makes [`units`] deterministic, and it is the same order
/// `build.rs`'s filesystem walk produces, so the two traversals name a
/// whole-embed malformation at the same unit.
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

    /// Composition's fixtures come from the parser's output, which is why the
    /// design rejected a separate composer seam: a boundary here would have to
    /// hand-build [`Unit`] values, and a fixture that skipped the reader could
    /// assert a shape the grammar cannot produce.
    fn embed(files: &[(&str, u32, &str)]) -> Vec<Unit> {
        files
            .iter()
            .flat_map(|(name, order, text)| {
                let ordered = format!("<!-- file: order={order} -->\n{text}");
                parse::parse_units(name, &ordered).expect("fixture must parse")
            })
            .collect()
    }

    /// *Requirement: Triggering units reach every kind they are scoped to* — all
    /// three scenarios, and the middle one is the half that is easy to lose: a
    /// narrowed unit must be **absent** from the other eighteen, not merely
    /// present in its own.
    #[test]
    fn selection_is_the_scope_and_nothing_else() {
        let units = embed(&[(
            "A.md",
            1,
            "<!-- unit: wide kinds=* class=triggering -->\nevery kind\n\
             <!-- unit: narrow kinds=impl class=triggering -->\nimpl only\n\
             <!-- unit: body class=procedural -->\nfetched, never composed\n",
        )]);

        for kind in Kind::ALL {
            let mandate = compose(&units, kind);
            assert!(
                mandate.contains("every kind"),
                "a `kinds=*` unit must reach {}",
                kind.label()
            );
            assert_eq!(
                mandate.contains("impl only"),
                kind == Kind::Impl,
                "`kinds=impl` must reach `impl` and no other kind; {} disagreed",
                kind.label()
            );
            assert!(
                !mandate.contains("fetched, never composed"),
                "a procedural unit ships to no mandate, and {} received one",
                kind.label()
            );
        }
    }

    /// *Requirement: Slices are byte-exact.* The whole mandate is asserted
    /// against a hand-written expectation rather than by substring, because the
    /// claim is about what is **not** there — no introduction, no separator, no
    /// re-wrapping — which no `contains` can see.
    #[test]
    fn slices_are_verbatim_and_joined_by_one_blank_line() {
        let units = embed(&[(
            "A.md",
            1,
            "<!-- unit: one kinds=* class=triggering -->\nfirst body\n\
             <!-- unit: two kinds=* class=triggering -->\nsecond body\n",
        )]);

        assert_eq!(
            compose(&units, Kind::Design),
            "<!-- unit: one kinds=* class=triggering -->\n\
             first body\n\
             \n\
             <!-- unit: two kinds=* class=triggering -->\n\
             second body\n",
            "each unit's source bytes, marker line included, with exactly one \
             blank line between adjacent slices and nothing else"
        );
    }

    /// Mandate order is the **file directive** then position within the file —
    /// deliberately not the path order [`units`] is assembled in, which is what
    /// this fixture separates: `A.md` sorts first by path and last by position.
    #[test]
    fn order_is_the_file_directive_then_position_within_the_file() {
        let units = embed(&[
            (
                "A.md",
                2,
                "<!-- unit: second-file-first kinds=* class=triggering -->\nc\n\
                 <!-- unit: second-file-second kinds=* class=triggering -->\nd\n",
            ),
            (
                "B.md",
                1,
                "<!-- unit: first-file-first kinds=* class=triggering -->\na\n\
                 <!-- unit: first-file-second kinds=* class=triggering -->\nb\n",
            ),
        ]);

        let mandate = compose(&units, Kind::Finish);
        let ids: Vec<&str> = mandate
            .lines()
            .filter(|line| line.starts_with("<!-- unit:"))
            .map(|line| line.split_whitespace().nth(2).unwrap())
            .collect();
        assert_eq!(
            ids,
            [
                "first-file-first",
                "first-file-second",
                "second-file-first",
                "second-file-second"
            ],
            "the file's declared position outranks its path, and units keep their \
             order within a file"
        );
    }

    /// A kind no unit is scoped to composes to nothing rather than to a
    /// driver-authored placeholder. Empty is the honest projection of an empty
    /// selection, and it is the shape the size alarm and the ending guard both
    /// have to be able to see.
    #[test]
    fn a_selection_of_nothing_composes_to_nothing() {
        let units = embed(&[(
            "A.md",
            1,
            "<!-- unit: only kinds=finish class=triggering -->\nbody\n",
        )]);
        assert_eq!(compose(&units, Kind::Impl), "");
        assert!(!compose(&units, Kind::Finish).is_empty());
    }

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
