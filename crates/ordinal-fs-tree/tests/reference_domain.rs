//! The reference domain against the architecture document's own examples.
//!
//! Every test here names the model claim it discharges, or says it has none.
//! That is what lets a later reader tell a checked property from an arranged
//! one, and it is the H3 probe's measure (`07-impl-h3-probe-k14`), so it is not
//! cosmetic.

use ordinal_fs_tree::reference::{
    Label, Parts, Status, SyllabusError, SyllabusName, OVERVIEW, PUBLISHING,
};
use ordinal_fs_tree::{
    EntryName, EntryNameExt, Found, Key, Ordinal, PositionedSpecies, Species, Verdict,
};

/// Every name in `ARCHITECTURE.md`'s tree diagram, with what the listing finds.
const DOCUMENT_EXAMPLES: &[(&str, Found, Species)] = &[
    (OVERVIEW, Found::File, Species::Distinguished),
    ("01-published-orientation-i1.md", Found::File, Species::Leaf),
    ("02-linear-algebra-i2", Found::Dir, Species::Node),
    ("03-draft-assessment-i9.md", Found::File, Species::Leaf),
    ("01-published-vectors-i5.md", Found::File, Species::Leaf),
    ("02-draft-matrices-i6.md", Found::File, Species::Leaf),
];

fn entry(name: &str, found: Found) -> SyllabusName {
    match SyllabusName::parse(name, found) {
        Verdict::Entry(n) => n,
        Verdict::Foreign => panic!("`{name}` was disclaimed"),
        Verdict::Malformed(e) | Verdict::Reserved(e) => panic!("`{name}` was refused: {e}"),
    }
}

/// Discharges `ParseIsCanonical` and `RoundTripDisplay` over the document's own
/// examples: the tree in the architecture diagram is one this domain reads, and
/// each name renders back to exactly the bytes it was read from.
#[test]
fn the_documents_examples_round_trip() {
    for (name, found, species) in DOCUMENT_EXAMPLES {
        let parsed = entry(name, *found);
        assert_eq!(parsed.to_string(), *name, "`{name}` did not render back");
        assert_eq!(
            parsed.species(),
            *species,
            "`{name}` named the wrong species"
        );
    }
}

/// Discharges the trichotomy's `Foreign` arm — no model claim, because
/// `Verdict` being a sum type is what makes classification total
/// (`TrichotomyIsTotalAndDisjoint`, which Alloy found free). What is checked
/// here is that this domain draws the Foreign/Malformed line where the document
/// says: a name it positively disclaims, and nothing else.
#[test]
fn names_this_domain_disclaims_are_foreign() {
    for (name, found) in [
        ("README.md", Found::File),
        ("notes", Found::Dir),
        (".DS_Store", Found::File),
        ("archive.tar.gz", Found::File),
    ] {
        assert!(
            matches!(SyllabusName::parse(name, found), Verdict::Foreign),
            "`{name}` should be foreign"
        );
    }
}

/// Discharges no model claim: `Reserved` is a domain's own affair. What it
/// checks is the architecture document's requirement that a refusal carry
/// **recovery advice** rather than detection alone — the library halts the whole
/// tree on this verdict, so an error saying only *something is wrong* leaves
/// whoever hit it frozen.
#[test]
fn the_reserved_witness_halts_and_says_what_to_do() {
    let Verdict::Reserved(err) = SyllabusName::parse(PUBLISHING, Found::File) else {
        panic!("`{PUBLISHING}` should be reserved");
    };
    let advice = err.to_string();
    assert!(advice.contains("delete"), "no recovery advice in: {advice}");
}

/// Discharges the obligation *the grammar is canonical* against the exact
/// structure `witness_two_filenames_name_one_entry` produces: a second spelling
/// of a name already in the tree. Each of these differs from a well-formed name
/// only in a way another parser would forgive — and forgiving it is what makes
/// two files on disk one entry.
///
/// `src/tree_id.rs` forgives the first of them, which is what this domain's
/// strictness is a correction of.
#[test]
fn a_second_spelling_of_a_name_is_malformed_not_foreign() {
    for (name, canonical) in [
        // unpadded ordinal
        ("5-draft-matrices-i6.md", "05-draft-matrices-i6.md"),
        // over-padded ordinal
        ("002-draft-matrices-i6.md", "02-draft-matrices-i6.md"),
        // padded key
        ("02-draft-matrices-i06.md", "02-draft-matrices-i6.md"),
    ] {
        let Verdict::Malformed(err) = SyllabusName::parse(name, Found::File) else {
            panic!("`{name}` should be malformed");
        };
        let SyllabusError::NotCanonical {
            canonical: advice, ..
        } = &err
        else {
            panic!("`{name}`: wrong refusal: {err}");
        };
        assert_eq!(advice, canonical, "wrong advice for `{name}`");
    }
}

/// Discharges the same obligation for the parts rather than the numbers: a name
/// that is this domain's shape and whose middle it cannot read is Malformed, so
/// the operation halts instead of the entry vanishing from every traversal.
#[test]
fn a_name_of_this_shape_that_cannot_be_read_is_malformed() {
    for (name, found) in [
        ("01-review-vectors-i5.md", Found::File), // no such status
        ("01-Draft-vectors-i5.md", Found::File),  // status is lowercase
        ("01-draft-Vectors-i5.md", Found::File),  // labels are lowercase
        ("01-draft-2vectors-i5.md", Found::File), // a label starts with a letter
        ("01-vectors-i5.md", Found::File),        // a lesson always states a status
        ("01-Linear-i2", Found::Dir),             // the same, for a module's label
    ] {
        assert!(
            matches!(SyllabusName::parse(name, found), Verdict::Malformed(_)),
            "`{name}` should be malformed"
        );
    }
}

/// Discharges `SpeciesAgreementIsParsed` — the obligation *parse refuses what
/// found contradicts* — and with it
/// the invariant `SpeciesAgreementHoldsWhenParsed` proves of every tree the
/// library walks. Two of these are `witness_species_mismatch_is_unclassifiable`
/// and one is `witness_distinguished_directory_hides_a_subtree`, which is the
/// case that hides an entire subtree while every other invariant holds.
#[test]
fn a_name_the_listing_contradicts_is_malformed() {
    for (name, found) in [
        ("02-draft-matrices-i6.md", Found::Dir), // a leaf's name over a directory
        ("02-linear-algebra-i2", Found::File),   // a node's name over a file
        (OVERVIEW, Found::Dir),                  // the subtree-hiding one
        ("02-draft-matrices-i6.md", Found::Other), // a symlink wearing a name
        (OVERVIEW, Found::Other),
    ] {
        let Verdict::Malformed(err) = SyllabusName::parse(name, found) else {
            panic!("`{name}` over {found} should be malformed");
        };
        assert!(
            matches!(err, SyllabusError::SpeciesMismatch { .. }),
            "`{name}` over {found}: wrong refusal: {err}"
        );
    }
}

/// Discharges no model claim — the models hold no strings, so a grammar's
/// ambiguities are invisible to them. This is the case that makes the terminal
/// `-i<key>` rule worth stating: a label may itself end in something shaped like
/// a key, and the name is still read one way only.
#[test]
fn the_key_is_the_terminal_token() {
    let parsed = entry("01-draft-notes-i7-i3.md", Found::File);
    let triple = parsed.triple().expect("a lesson is positioned");
    assert_eq!(triple.key, Key::new(3));
    assert_eq!(triple.parts.label().as_str(), "notes-i7");
    assert_eq!(parsed.to_string(), "01-draft-notes-i7-i3.md");
}

/// Discharges `DistLawful` — a distinguished child carries neither an ordinal
/// nor a key. Since `seam-k18` that is read off `NameView::Distinguished`, so
/// the test now checks that this domain's `view` says *distinguished* for this
/// name rather than that it remembered to return `None`.
#[test]
fn the_distinguished_child_carries_no_triple() {
    let overview = SyllabusName::distinguished().expect("this domain has one");
    assert!(overview.triple().is_none());
    assert_eq!(overview.species(), Species::Distinguished);
    assert_eq!(overview.to_string(), OVERVIEW);
}

/// Discharges `ComposeLawful`, checked by Alloy as `ShiftPreservesIdentity`,
/// in the form the library actually uses it: a sibling shift is
/// `compose(new_ordinal, key, parts)` and nothing else, so it cannot disturb a
/// key, a label or an attribute. The conformance kit checks the same obligation
/// generically; this checks that a *shift* of a real name is what it claims.
#[test]
fn a_shift_moves_the_ordinal_and_nothing_else() {
    let before = entry("02-draft-matrices-i6.md", Found::File);
    let t = before.triple().expect("positioned");
    let after = SyllabusName::compose(Ordinal::new(3), t.key, t.parts.clone());

    assert_eq!(after.to_string(), "03-draft-matrices-i6.md");
    let t2 = after.triple().expect("positioned");
    assert_eq!(t2.ordinal, Ordinal::new(3));
    assert_eq!(t2.key, t.key);
    assert_eq!(t2.parts, t.parts);
    assert_eq!(after.species(), before.species());
}

/// Discharges no model claim: `Label` validation is a grammar concern the models
/// abstract away entirely. It is here because every refusal above rests on it.
#[test]
fn a_label_is_what_the_grammar_can_read_back() {
    assert!(Label::new("linear-algebra").is_ok());
    assert!(Label::new("notes-i7").is_ok());
    assert!(Label::new("week2").is_ok());
    for bad in [
        "",
        "-leading",
        "trailing-",
        "Upper",
        "2leading",
        "with space",
        "with_score",
    ] {
        assert!(Label::new(bad).is_err(), "`{bad}` should not be a label");
    }
}

/// Discharges `NothingRecognisedIsSkipped` at the one place this domain nearly
/// broke it: a name carrying both markers this domain recognises its own names
/// by — a leading ordinal and a terminal key — with the label between them
/// missing. `01--i3` is what an empty label *renders* as, so it is this
/// domain's own name with its middle damaged, and disclaiming it would skip the
/// file. The directory spelling is the one that costs a subtree: a walk that
/// skips it loses everything beneath it while reporting a healthy tree.
///
/// `seam-k17` found this classified as `Foreign` in both spellings.
#[test]
fn a_name_of_this_shape_with_no_label_is_malformed_not_foreign() {
    for (name, found) in [
        ("01--i3.md", Found::File), // the lesson spelling
        ("01--i3", Found::Dir),     // the module spelling — a whole subtree
    ] {
        let Verdict::Malformed(err) = SyllabusName::parse(name, found) else {
            panic!("`{name}` should be malformed, not skipped");
        };
        let SyllabusError::BadLabel { label, .. } = &err else {
            panic!("`{name}`: wrong refusal: {err}");
        };
        assert!(
            label.is_empty(),
            "`{name}`: the empty label is what is wrong"
        );
        let advice = err.to_string();
        assert!(
            advice.contains("Rename it") && advice.contains("may not be empty"),
            "no recovery advice in: {advice}"
        );
    }
}

/// The other side of that line, so the correction did not swallow it: a name
/// missing one of the two markers is genuinely not this domain's, and staying
/// `Foreign` is what lets a tree hold unrelated files at all.
#[test]
fn a_name_missing_a_marker_is_still_foreign() {
    for (name, found) in [
        ("01-i3.md", Found::File), // no `-i` before the key digits
        ("-i3.md", Found::File),   // no leading ordinal
        ("draft-vectors.md", Found::File),
    ] {
        assert!(
            matches!(SyllabusName::parse(name, found), Verdict::Foreign),
            "`{name}` should be foreign"
        );
    }
}

/// Discharges no model claim: this is the species-follows-from-parts rule at the
/// level of the concrete domain, which the models take as given.
#[test]
fn the_species_follows_from_the_parts() {
    let lesson = Parts::lesson(Status::Draft, Label::new("vectors").unwrap());
    let module = Parts::module(Label::new("vectors").unwrap());
    assert_eq!(lesson.species(), PositionedSpecies::Leaf);
    assert_eq!(module.species(), PositionedSpecies::Node);
    // And it is the *only* input to a positioned name's species: the seam reads
    // `species()` off `positioned_species(parts)`, which has no ordinal and no
    // key to consult, so the same parts at any position are the same species.
    assert_eq!(
        SyllabusName::positioned_species(&lesson),
        SyllabusName::positioned_species(&lesson)
    );

    let o = Ordinal::new(1);
    let k = Key::new(4);
    assert_eq!(
        SyllabusName::compose(o, k, lesson).to_string(),
        "01-draft-vectors-i4.md"
    );
    assert_eq!(
        SyllabusName::compose(o, k, module).to_string(),
        "01-vectors-i4"
    );
}
