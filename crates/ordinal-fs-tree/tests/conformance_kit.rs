//! The conformance kit, and the four deliberately broken domains that prove it
//! is not reading clean while broken.
//!
//! Each broken domain is one of `structure.als`'s witnesses written in Rust: a
//! shape the model produces on demand, here as an `EntryName` implementation the
//! kit must reject. A kit that passes the reference domain and nothing else has
//! shown only that it can say yes.

use core::fmt;

use ordinal_fs_tree::conformance::{self, Finding, Obligation, DISCHARGED_BY_THE_TYPE_SYSTEM};
use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusError, SyllabusName};
use ordinal_fs_tree::{EntryName, Found, Key, Ordinal, Species, Triple, Verdict};

/// A listing the reference domain should be entirely at home in: both species,
/// the distinguished child, a foreign name, and the reserved witness.
fn listings() -> Vec<(&'static str, Found)> {
    vec![
        ("OVERVIEW.md", Found::File),
        ("01-published-orientation-i1.md", Found::File),
        ("02-linear-algebra-i2", Found::Dir),
        ("03-draft-assessment-i9.md", Found::File),
        ("README.md", Found::File),
        ("PUBLISHING", Found::File),
    ]
}

fn triples() -> Vec<(Ordinal, Key, Parts)> {
    vec![
        (
            Ordinal::new(1),
            Key::new(1),
            Parts::lesson(Status::Published, Label::new("orientation").unwrap()),
        ),
        (
            Ordinal::new(2),
            Key::new(2),
            Parts::module(Label::new("linear-algebra").unwrap()),
        ),
    ]
}

fn violated(report: &conformance::Report) -> Vec<Obligation> {
    let mut found: Vec<Obligation> = report
        .violations()
        .map(|f| match f {
            Finding::Violated { obligation, .. } => *obligation,
            Finding::NotExercised { .. } => unreachable!(),
        })
        .collect();
    found.sort_by_key(|o| format!("{o}"));
    found.dedup();
    found
}

/// Discharges every trait obligation Alloy states — `ComposeLawful`,
/// `ParseIsCanonical`, `RoundTripDisplay`, `OneDistinguishedName`, `DistLawful`
/// and `SpeciesAgreementIsParsed` — for the reference domain, through the kit
/// that every other domain will use for the same purpose.
#[test]
fn the_reference_domain_conforms() {
    let report = conformance::check::<SyllabusName>(&listings(), &triples());
    report.assert_conforming();
}

/// The kit's own positive control, and the reason it reports two kinds of
/// finding. Handed nothing, a violations-only kit says *conforming* — the exact
/// shape of `docs/formalism-findings.md` entry 003's three instruments, each of
/// which reported *found nothing* and *succeeded* with the same bytes.
#[test]
fn a_kit_handed_nothing_reports_that_it_checked_nothing() {
    let report = conformance::check::<SyllabusName>(&[], &[]);
    assert!(!report.is_conforming(), "an empty run must not pass");
    assert_eq!(report.violations().count(), 0, "nothing was violated");
    assert_eq!(
        report.unexercised().count(),
        Obligation::ALL.len(),
        "every obligation should be reported untested:\n{report}"
    );
}

/// Samples can be thin in one direction only, too: names but no triples leaves
/// `compose` uncalled.
#[test]
fn samples_that_reach_only_half_the_seam_say_so() {
    let report = conformance::check::<SyllabusName>(&listings(), &[]);
    assert!(!report.is_conforming());
    assert!(report
        .unexercised()
        .any(|f| matches!(f, Finding::NotExercised { obligation, .. }
                          if *obligation == Obligation::ComposePlacesWhatItIsGiven)));
}

/// The obligation Rust discharges is named rather than dropped: a consumer
/// counting four checks against the document's five needs to see that the fifth
/// was not forgotten.
#[test]
fn the_obligation_the_type_system_discharges_is_reported() {
    assert_eq!(DISCHARGED_BY_THE_TYPE_SYSTEM.len(), 1);
    assert_eq!(
        Obligation::ALL.len() + DISCHARGED_BY_THE_TYPE_SYSTEM.len(),
        5,
        "the architecture document states five obligations"
    );
    assert!(DISCHARGED_BY_THE_TYPE_SYSTEM[0]
        .statement
        .contains("positioned or distinguished"));
}

// ===========================================================================
// The broken domains. Each wraps the reference domain and breaks exactly one
// obligation, so the kit's verdict has one thing in it.
// ===========================================================================

macro_rules! delegate {
    ($t:ty) => {
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

/// `witness_shift_corrupts_identity`: a `compose` that does not place what it
/// is given. Nothing the architecture document said before the model ran ruled
/// this out, and under it a sibling shift moves one entry's key onto another's
/// position while every stated invariant still holds.
#[derive(Clone)]
struct Forgetful(SyllabusName);
delegate!(Forgetful);

impl EntryName for Forgetful {
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
        // The defect: the ordinal it was handed is not the one it writes.
        Self(SyllabusName::compose(
            Ordinal::new(ordinal.get() + 1),
            key,
            parts,
        ))
    }

    fn distinguished() -> Option<Self> {
        SyllabusName::distinguished().map(Self)
    }

    fn triple(&self) -> Option<Triple<'_, Self::Parts>> {
        self.0.triple()
    }

    fn species(&self) -> Species {
        self.0.species()
    }
}

#[test]
fn a_compose_that_ignores_its_arguments_is_caught() {
    let report = conformance::check::<Forgetful>(&listings(), &triples());
    assert_eq!(
        violated(&report),
        vec![Obligation::ComposePlacesWhatItIsGiven],
        "{report}"
    );
}

/// `witness_two_filenames_name_one_entry`: a grammar that forgives an unpadded
/// ordinal, so `5-…` and `05-…` are one entry with one key at one ordinal —
/// and one of them is a file the tree cannot see twice. This is the shape
/// `src/tree_id.rs` has today.
#[derive(Clone)]
struct Lenient(SyllabusName);
delegate!(Lenient);

impl EntryName for Lenient {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        // The defect: pad a single-digit ordinal and carry on, so the name that
        // comes back is not the name that went in.
        let padded = match name.split_once('-') {
            Some((head, rest)) if head.len() == 1 && head.bytes().all(|b| b.is_ascii_digit()) => {
                format!("0{head}-{rest}")
            }
            _ => name.to_string(),
        };
        match SyllabusName::parse(&padded, found) {
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

    fn triple(&self) -> Option<Triple<'_, Self::Parts>> {
        self.0.triple()
    }

    fn species(&self) -> Species {
        self.0.species()
    }
}

#[test]
fn a_grammar_with_two_spellings_of_one_name_is_caught() {
    let mut names = listings();
    names.push(("5-draft-matrices-i6.md", Found::File));
    let report = conformance::check::<Lenient>(&names, &triples());
    assert_eq!(
        violated(&report),
        vec![Obligation::TheGrammarIsCanonical],
        "{report}"
    );
}

/// `witness_species_mismatch_is_unclassifiable`: a `parse` that ignores what
/// the listing found. Then a directory wearing a leaf's name is an `Entry`, a
/// walk never descends into it, and the whole subtree beneath it is invisible
/// while the tree reports itself healthy.
#[derive(Clone)]
struct Blind(SyllabusName);
delegate!(Blind);

impl EntryName for Blind {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        // The defect: whatever the listing says, try the species' own kind and
        // accept the name if it reads at all.
        for pretend in [Found::File, Found::Dir] {
            if let Verdict::Entry(n) = SyllabusName::parse(name, pretend) {
                return Verdict::Entry(Self(n));
            }
        }
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

    fn triple(&self) -> Option<Triple<'_, Self::Parts>> {
        self.0.triple()
    }

    fn species(&self) -> Species {
        self.0.species()
    }
}

#[test]
fn a_parse_that_ignores_the_listing_is_caught() {
    let report = conformance::check::<Blind>(&listings(), &triples());
    assert_eq!(
        violated(&report),
        vec![Obligation::ParseRefusesWhatFoundContradicts],
        "{report}"
    );
}

/// `witness_two_distinguished_children`: a second name of the distinguished
/// species. *At most one distinguished child per node* is a theorem given that
/// `distinguished()` names one thing and a directory cannot hold two entries of
/// one name; drop the first half and a node holds two, each hiding whatever the
/// other does not.
#[derive(Clone)]
enum TwoOverviews {
    Delegated(SyllabusName),
    /// The defect: a second name this domain calls a distinguished child.
    Index,
}

impl fmt::Display for TwoOverviews {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Delegated(n) => n.fmt(f),
            Self::Index => f.write_str("INDEX.md"),
        }
    }
}

impl EntryName for TwoOverviews {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        if name == "INDEX.md" && found == Found::File {
            return Verdict::Entry(Self::Index);
        }
        match SyllabusName::parse(name, found) {
            Verdict::Entry(n) => Verdict::Entry(Self::Delegated(n)),
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self::Delegated(SyllabusName::compose(ordinal, key, parts))
    }

    fn distinguished() -> Option<Self> {
        SyllabusName::distinguished().map(Self::Delegated)
    }

    fn triple(&self) -> Option<Triple<'_, Self::Parts>> {
        match self {
            Self::Delegated(n) => n.triple(),
            Self::Index => None,
        }
    }

    fn species(&self) -> Species {
        match self {
            Self::Delegated(n) => n.species(),
            Self::Index => Species::Distinguished,
        }
    }
}

#[test]
fn a_second_distinguished_name_is_caught() {
    let mut names = listings();
    names.push(("INDEX.md", Found::File));
    let report = conformance::check::<TwoOverviews>(&names, &triples());
    assert_eq!(
        violated(&report),
        vec![Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies],
        "{report}"
    );
}
