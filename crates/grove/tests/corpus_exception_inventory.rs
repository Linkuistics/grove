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
        let root = format!("{BOOKS}/{book}");
        let text = fs::read_to_string(repo_root().join(&root).join("walkthrough.toml"))
            .unwrap_or_else(|_| panic!("book `{book}` carries a walkthrough.toml"));
        let manifest = book_validation::Manifest::load(&root, &text)
            .unwrap_or_else(|error| panic!("book `{book}` manifest: {}", error.reason()));
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
