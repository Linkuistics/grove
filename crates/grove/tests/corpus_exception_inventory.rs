//! Every book's corpus exceptions, against the specification's inventory.
//!
//! **This is the successor to `compiled_corpus_copy_matches_the_book_ledger_tables`,
//! and it is why that bridge could be deleted.** The bridge compared the
//! validator's compiled corpus against the book's own ledger — two statements
//! the same author wrote, so it caught drift between them and could not catch a
//! file left out of both. `validator-fragments-k22` replaced the compiled corpus
//! with filesystem derivation, which is a genuinely external witness for
//! *roots*: a production file added to a crate and forgotten by its book is a
//! finding rather than a silence.
//!
//! Derivation cannot witness an **exception**. A `reason` is prose, and one
//! manifest edit would otherwise move both the asserted rule and the roots
//! checked against it — an author who wanted a file out of the corpus could put
//! it out, in the same commit, with no second party disagreeing. So every
//! exception is declared twice: in the manifest, and in the normative inventory
//! in `docs/specs/walkthrough-books.md`. An addition or exclusion is then an
//! agreement between the book and the specification rather than an assertion
//! the book makes about itself.
//!
//! **`[book].subject` is declared twice for the same reason, and this file
//! carries that agreement too.** The exception inventory closes the corpus rule
//! at the top — what a book may add or drop — and the base patterns derived
//! from `subject` close it at the bottom. But `subject` itself was the book's
//! to pick: a manifest naming a subdirectory of its crate narrows both base
//! patterns, so every root outside that subdirectory is reached by no pattern,
//! is dropped from `[[root]]` with the pages that reconstructed it, and the
//! book validates green over a fraction of the crate it claims to be about.
//! Derivation cannot see that, because derivation compares the declared roots
//! against the tree the *patterns* reach. The subject inventory is the second
//! party the book has to agree with.
//!
//! It is a **repository** test rather than a `book-check` diagnostic on
//! purpose. `book-check` runs against one book directory and would be reading
//! the book's own account of itself again; this comparison is with a document
//! the book does not own.

use std::collections::BTreeSet;
use std::fs;

mod support;

use support::repo_root;

const SPECIFICATION: &str = "docs/specs/walkthrough-books.md";
const BOOKS: &str = "docs/walkthroughs";
const HEADING: &str = "**The corpus exception inventory.**";
const SUBJECT_HEADING: &str = "**The book subject inventory.**";

/// One row of the agreement, in the form both sides are reduced to:
/// `(book id, "add" | "exclude", path, class)`.
type Entry = (String, String, String, String);

/// The inventory table under *The corpus rule and its witness*.
fn inventory() -> BTreeSet<Entry> {
    let text = fs::read_to_string(repo_root().join(SPECIFICATION)).expect("the specification");
    let after = text
        .split_once(HEADING)
        .unwrap_or_else(|| panic!("{SPECIFICATION} carries the paragraph `{HEADING}`"))
        .1;
    let table = after
        .split("\n\n")
        .find(|block| block.starts_with("| Book |"))
        .expect("the paragraph is followed by the inventory table");
    let rows: BTreeSet<Entry> = table
        .lines()
        .skip(2)
        .map(|row| {
            let cells: Vec<String> = row
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().trim_matches('`').to_owned())
                .collect();
            assert_eq!(cells.len(), 4, "inventory row has four columns: {row}");
            (
                cells[0].clone(),
                cells[1].clone(),
                cells[2].clone(),
                cells[3].clone(),
            )
        })
        .collect();
    assert!(
        !rows.is_empty(),
        "the inventory table is empty; a comparison against nothing passes for any set of books"
    );
    rows
}

/// The same rows, read from every manifest under `docs/walkthroughs/`.
fn declared() -> BTreeSet<Entry> {
    let mut entries = BTreeSet::new();
    let mut books = 0;
    for book in book_roots() {
        books += 1;
        let manifest = manifest_of(&book, |text| text.to_owned());
        let corpus = manifest.corpus();
        for (kind, exceptions) in [("add", corpus.add()), ("exclude", corpus.exclude())] {
            for exception in exceptions {
                entries.insert((
                    manifest.book_id().to_owned(),
                    kind.to_owned(),
                    exception.path().to_owned(),
                    exception.class().as_str().to_owned(),
                ));
            }
        }
    }
    assert!(
        books > 0,
        "no book roots found under {BOOKS}; an empty walk agrees with any inventory"
    );
    entries
}

/// The book roots, by discovery. A sixth book joins this check by existing.
fn book_roots() -> Vec<String> {
    let mut roots: Vec<String> = fs::read_dir(repo_root().join(BOOKS))
        .expect("the book root directory")
        .map(|entry| entry.expect("a readable book entry"))
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    roots.sort();
    roots
}

#[test]
fn every_books_corpus_exceptions_are_exactly_the_specifications_inventory() {
    // Per book, and only for books that exist. The inventory deliberately
    // carries rows for deliverables this campaign has not written yet — those
    // are forward commitments, checked by the test below, and a row nobody can
    // claim is not a disagreement. Once a book's directory exists, every row
    // naming it binds in both directions.
    let books: BTreeSet<String> = book_roots().into_iter().collect();
    let inventory: BTreeSet<Entry> = inventory()
        .into_iter()
        .filter(|(book, ..)| books.contains(book))
        .collect();
    let declared = declared();

    let unlisted: Vec<&Entry> = declared.difference(&inventory).collect();
    let unclaimed: Vec<&Entry> = inventory.difference(&declared).collect();

    assert!(
        unlisted.is_empty(),
        "these corpus exceptions are declared by a book and appear in no row of {SPECIFICATION}: {unlisted:#?}"
    );
    assert!(
        unclaimed.is_empty(),
        "these rows of {SPECIFICATION} are claimed by no book's manifest: {unclaimed:#?}"
    );
}

/// The inventory names books that do not exist yet, and that is the point: it
/// is the campaign's forward commitment, so a book landing later cannot quietly
/// widen its own corpus.
///
/// Without this the test above would still pass for a table whose future rows
/// had been silently deleted along with the book that was going to carry them.
#[test]
fn the_inventory_covers_books_that_have_not_been_written_yet() {
    let books: BTreeSet<String> = book_roots().into_iter().collect();
    let promised: BTreeSet<String> = inventory()
        .into_iter()
        .map(|(book, ..)| book)
        .filter(|book| !books.contains(book))
        .collect();

    assert!(
        !promised.is_empty(),
        "every inventory row names an existing book, so the table records no forward commitment"
    );
}

/// One row of the subject agreement: `(book id, subject directory)`.
type Subject = (String, String);

/// The subject table under *The corpus rule and its witness*.
fn subject_inventory() -> BTreeSet<Subject> {
    let text = fs::read_to_string(repo_root().join(SPECIFICATION)).expect("the specification");
    let after = text
        .split_once(SUBJECT_HEADING)
        .unwrap_or_else(|| panic!("{SPECIFICATION} carries the paragraph `{SUBJECT_HEADING}`"))
        .1;
    let table = after
        .split("\n\n")
        .find(|block| block.starts_with("| Book |"))
        .expect("the paragraph is followed by the subject table");
    let rows: BTreeSet<Subject> = table
        .lines()
        .skip(2)
        .map(|row| {
            let cells: Vec<String> = row
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().trim_matches('`').to_owned())
                .collect();
            assert_eq!(cells.len(), 2, "subject row has two columns: {row}");
            (cells[0].clone(), cells[1].clone())
        })
        .collect();
    assert!(
        !rows.is_empty(),
        "the subject table is empty; a comparison against nothing passes for any set of books"
    );
    rows
}

/// The same rows, read from every manifest under `docs/walkthroughs/`.
fn declared_subjects() -> BTreeSet<Subject> {
    let mut entries = BTreeSet::new();
    let mut books = 0;
    for book in book_roots() {
        books += 1;
        let manifest = manifest_of(&book, |text| text.to_owned());
        entries.insert((manifest.book_id().to_owned(), manifest.subject().to_owned()));
    }
    assert!(
        books > 0,
        "no book roots found under {BOOKS}; an empty walk agrees with any inventory"
    );
    entries
}

/// One book's manifest, with `rewrite` applied to its text first. The identity
/// rewrite is the ordinary load; the narrowing test below supplies an attack.
fn manifest_of(book: &str, rewrite: impl Fn(&str) -> String) -> book_validation::Manifest {
    let root = format!("{BOOKS}/{book}");
    let text = fs::read_to_string(repo_root().join(&root).join("walkthrough.toml"))
        .unwrap_or_else(|_| panic!("book `{book}` carries a walkthrough.toml"));
    book_validation::Manifest::load(&root, &rewrite(&text))
        .unwrap_or_else(|error| panic!("book `{book}` manifest: {}", error.reason()))
}

#[test]
fn every_books_subject_is_exactly_the_specifications_inventory() {
    // Per book, and only for books that exist — the same reading as the
    // exception inventory above, and for the same reason.
    let books: BTreeSet<String> = book_roots().into_iter().collect();
    let inventory: BTreeSet<Subject> = subject_inventory()
        .into_iter()
        .filter(|(book, _)| books.contains(book))
        .collect();
    let declared = declared_subjects();

    let unlisted: Vec<&Subject> = declared.difference(&inventory).collect();
    let unclaimed: Vec<&Subject> = inventory.difference(&declared).collect();

    assert!(
        unlisted.is_empty(),
        "these book subjects are declared by a manifest and appear in no row of {SPECIFICATION}: {unlisted:#?}"
    );
    assert!(
        unclaimed.is_empty(),
        "these rows of {SPECIFICATION} are claimed by no book's manifest: {unclaimed:#?}"
    );
}

/// The subject table carries the campaign's unwritten deliverables too, so a
/// book landing later is authored against a subject a second party already
/// fixed rather than choosing one and having the table follow it.
#[test]
fn the_subject_inventory_covers_books_that_have_not_been_written_yet() {
    let books: BTreeSet<String> = book_roots().into_iter().collect();
    let promised: BTreeSet<String> = subject_inventory()
        .into_iter()
        .map(|(book, _)| book)
        .filter(|book| !books.contains(book))
        .collect();

    assert!(
        !promised.is_empty(),
        "every subject row names an existing book, so the table records no forward commitment"
    );
}

/// The narrowing attack, run for real against every book in the repository.
///
/// Moving `subject` one directory down and moving the two base patterns with it
/// leaves a manifest the schema still accepts — the base-pattern rule only
/// requires the patterns to *match* the declared subject, and it has no opinion
/// about which subject that is. Under such a manifest the include patterns
/// reach a fraction of the crate, so the honest root set shrinks to match and
/// every check `book-check` runs stays internally consistent. What refuses it
/// is this file's comparison, and nothing else.
#[test]
fn a_manifest_that_narrows_its_subject_disagrees_with_the_inventory() {
    let inventory = subject_inventory();
    let mut attacked = 0;

    for book in book_roots() {
        let honest = manifest_of(&book, |text| text.to_owned());
        let subject = honest.subject().to_owned();
        let narrowed = format!("{subject}/src");
        let attack = manifest_of(&book, |text| {
            text.replace(
                &format!("subject = \"{subject}\""),
                &format!("subject = \"{narrowed}\""),
            )
            .replace(
                &format!("\"{subject}/Cargo.toml\""),
                &format!("\"{narrowed}/Cargo.toml\""),
            )
            .replace(
                &format!("\"{subject}/src/**/*.rs\""),
                &format!("\"{narrowed}/src/**/*.rs\""),
            )
        });
        assert_eq!(
            attack.subject(),
            narrowed,
            "the attack manifest for `{book}` did not take: the rewrite matched nothing"
        );

        let row = (attack.book_id().to_owned(), narrowed.clone());
        assert!(
            !inventory.contains(&row),
            "{SPECIFICATION} carries `{narrowed}` for book `{book}`, so the narrowing attack agrees with the inventory"
        );
        attacked += 1;
    }

    assert!(attacked > 0, "no book roots found under {BOOKS}");
}
