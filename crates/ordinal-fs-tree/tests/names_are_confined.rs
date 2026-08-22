//! The seventh obligation, through the public surface: a name renders as one
//! path component, and the library **enforces** it.
//!
//! The other six obligations the library assumes; a domain that breaks one of
//! them gets a tree quietly corrupted, and `tests/conformance_kit.rs` is where
//! that is caught before there is a tree. This one is different in kind. The
//! rendering is what gets joined to a level's directory, so breaking it does not
//! corrupt the tree — it leaves it, and creates, renames, removes and reports
//! outside the directory whose lock is the only thing covering any of it. The
//! central proposition of the library is that one directory tree *is* the data
//! structure, and this is the one way a consumer could make that false.
//!
//! **Neither model can pose it.** `operations.qnt`'s handoff block and
//! `structure.als` both record that their state holds no strings, exactly as it
//! holds no bytes, so a rendering is not a thing either can say. `ARCHITECTURE.md`
//! owns it, in the same position as the refusals for content-for-a-node and a
//! non-UTF-8 filename: a case the library can see and no model can reach.
//!
//! Two boundaries, so two adversaries. Each satisfies everything the algebra
//! looks at — occupancy compares `view`s, and both views are the reference
//! domain's — and differs only in what it renders.

use core::fmt;
use std::fs;
use std::path::Path;

use tempfile::TempDir;

use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusError, SyllabusName};
use ordinal_fs_tree::{
    EntryName, Error, Found, Key, NameView, NewEntry, Ordinal, PositionedSpecies, Target, Verdict,
};

fn draft(label: &str) -> Parts {
    Parts::lesson(
        Status::Draft,
        Label::new(label).expect("a well-formed label"),
    )
}

/// A root inside a temporary directory, with a sibling beside it that nothing in
/// the tree may ever reach.
fn tree() -> (TempDir, std::path::PathBuf) {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary.path().join("syllabus");
    fs::create_dir(&root).expect("creating the tree root");
    (temporary, root)
}

/// Every path directly inside a directory, sorted.
fn names(directory: &Path) -> Vec<String> {
    let mut out: Vec<String> = fs::read_dir(directory)
        .expect("a readable directory")
        .map(|entry| {
            entry
                .expect("a readable entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    out.sort();
    out
}

// ---------------------------------------------------------------------------
// The reading boundary: a domain whose `parse` yields a name that renders
// somewhere else.
// ---------------------------------------------------------------------------

/// Reads the reference domain's filenames and renders each one a level up.
///
/// A snapshot name is what `entry_path` renders to find the entry a move starts
/// from, and what `level_path` renders to find a node a plan writes into — so a
/// snapshot holding one of these takes the interpreter outside the tree without
/// any plan naming anything unusual.
#[derive(Clone)]
struct ParseEscapes(SyllabusName);

impl fmt::Display for ParseEscapes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "../{}", self.0)
    }
}

impl EntryName for ParseEscapes {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        match SyllabusName::parse(name, found) {
            Verdict::Entry(n) => Verdict::Entry(Self(n)),
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self(SyllabusName::compose(ordinal, key, parts))
    }

    fn distinguished() -> Option<Self> {
        SyllabusName::distinguished().map(Self)
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

/// A snapshot never admits a name that is not one filename, so nothing built
/// from a snapshot's names can address outside the tree.
///
/// Halting at the read is what makes `entry_path` and `level_path` safe without
/// either of them repeating the check.
#[test]
fn a_snapshot_refuses_a_name_that_does_not_render_as_one_filename() {
    let (_temporary, root) = tree();
    fs::write(root.join("01-draft-first-i1.md"), "first").expect("a fixture");

    // A `let … else` rather than `expect_err`, because a `ReadGuard` is not
    // `Debug`: it holds a lock, and a guard that could be printed would tempt
    // exactly the logging the *locking is invisible* rule exists to prevent.
    let Err(failed) = ordinal_fs_tree::fs::read::<ParseEscapes>(&root) else {
        panic!("a name that leaves the tree cannot enter a snapshot");
    };
    let Error::NameIsNotOneComponent {
        rendered, reason, ..
    } = &failed
    else {
        panic!("this is the library's own refusal, not the domain's: {failed:?}");
    };
    assert_eq!(rendered, "../01-draft-first-i1.md");
    assert!(reason.contains("separator"));
    // The advice has to name the thing to fix, not merely the thing that broke.
    let message = failed.to_string();
    assert!(message.contains("one filename"), "{message}");
    assert!(message.contains("conformance::check"), "{message}");
}

// ---------------------------------------------------------------------------
// The planning boundary: a domain whose `compose` yields such a name.
// ---------------------------------------------------------------------------

/// The review's own adversary: a domain that satisfies **every other
/// obligation** — canonicity included, because its `parse` claims exactly the
/// spellings its `Display` produces — and composes names that leave the tree.
///
/// It is `tests/conformance_kit.rs`'s `Escaping`, which that file's
/// `a_name_that_renders_as_more_than_one_component_is_caught` puts through the
/// kit. This is the other half: what happens if a domain skips the kit.
#[derive(Clone)]
struct ComposeEscapes(SyllabusName);

impl fmt::Display for ComposeEscapes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "../{}", self.0)
    }
}

impl EntryName for ComposeEscapes {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        let Some(rest) = name.strip_prefix("../") else {
            return Verdict::Foreign;
        };
        match SyllabusName::parse(rest, found) {
            Verdict::Entry(n) => Verdict::Entry(Self(n)),
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self(SyllabusName::compose(ordinal, key, parts))
    }

    fn distinguished() -> Option<Self> {
        SyllabusName::distinguished().map(Self)
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

/// A mutation refuses a plan whose name is not one filename, and refuses it
/// before any effect runs — so nothing is created inside the tree either.
///
/// The plan the algebra built is impeccable: the composed name carries a fresh
/// key, so occupancy — which compares `view`s — finds the destination free, and
/// `Plan::guarded` proceeds. Only the rendering betrays it, which is why the
/// check is at the boundary where a rendering is used and nowhere earlier.
#[test]
fn a_mutation_refuses_a_composed_name_that_leaves_the_tree() {
    let (temporary, root) = tree();
    let beside = names(temporary.path());

    let guard = ordinal_fs_tree::fs::write::<ComposeEscapes>(&root).expect("an empty tree reads");
    let failed = guard
        .append(
            Target::Root,
            NewEntry::new(draft("escaped"), b"mine".to_vec()),
        )
        .expect_err("a name that leaves the tree cannot be placed");

    let Error::NameIsNotOneComponent { rendered, .. } = &failed else {
        panic!("this is not an I/O failure and not a refusal of the algebra's: {failed:?}");
    };
    assert_eq!(rendered, "../01-draft-escaped-i1.md");
    assert!(
        names(&root).is_empty(),
        "the tree is as it was found: nothing was created inside it"
    );
    assert_eq!(
        names(temporary.path()),
        beside,
        "and nothing was created beside it, which is where `../` would have put it"
    );
}
