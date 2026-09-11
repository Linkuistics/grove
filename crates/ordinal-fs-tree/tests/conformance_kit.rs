//! The conformance kit, with deliberately broken domains that prove it
//! is not reading clean while broken.
//!
//! Each broken domain but the last is one of `structure.als`'s witnesses written
//! in Rust: a shape the model produces on demand, here as an `EntryName`
//! implementation the kit must reject. A kit that passes the reference domain
//! and nothing else has shown only that it can say yes.
//!
//! The last has no witness and can have none — both models hold no strings — so
//! it is written from the obligation's statement instead, and it is the one
//! obligation the library also enforces at run time. `tests/names_are_confined.rs`
//! is that half.

use core::{cell::Cell, fmt};
use std::sync::atomic::{AtomicBool, Ordering};

#[allow(deprecated)]
use ordinal_fs_tree::conformance::{
    self, Finding, Obligation, DISCHARGED_BY_THE_TYPE_SYSTEM, TYPE_SHAPE_CONSTRAINTS,
};
use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusError, SyllabusName};
use ordinal_fs_tree::{
    EntryName, EntryNameExt, Found, Key, NameView, Ordinal, PositionedSpecies, Species, Verdict,
};

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

fn accepting_levels<N: EntryName<Parts = Parts>>(own: N) -> Vec<conformance::LevelSample<N>> {
    [
        None,
        Some(N::compose(
            Ordinal::FIRST,
            Key::new(1),
            Parts::module(Label::new("topic").unwrap()),
        )),
    ]
    .into_iter()
    .flat_map(|node| {
        [vec![], vec![own.clone()], vec![own.clone(), own.clone()]]
            .into_iter()
            .map(move |distinguished| conformance::LevelSample {
                node: node.clone(),
                distinguished,
                accepted: true,
            })
    })
    .collect()
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
/// `ParseIsCanonical`, `RoundTripDisplay`, `DistLawful`
/// and `SpeciesAgreementIsParsed` — for the reference domain, through the kit
/// that every other domain will use for the same purpose.
#[test]
fn the_reference_domain_conforms() {
    let report = conformance::check::<SyllabusName>(
        &listings(),
        &triples(),
        &[SyllabusName::Overview],
        &accepting_levels(SyllabusName::Overview),
    );
    report.assert_conforming();
}

/// The kit's own positive control, and the reason it reports two kinds of
/// finding. Handed nothing, a violations-only kit says *conforming* — the exact
/// shape of `docs/formalism-findings.md` entry 003's three instruments, each of
/// which reported *found nothing* and *succeeded* with the same bytes.
#[test]
fn a_kit_handed_nothing_reports_that_it_checked_nothing() {
    let report = conformance::check::<SyllabusName>(&[], &[], &[], &[]);
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
    let report = conformance::check::<SyllabusName>(
        &listings(),
        &[],
        &[SyllabusName::Overview],
        &accepting_levels(SyllabusName::Overview),
    );
    assert!(!report.is_conforming());
    assert!(report
        .unexercised()
        .any(|f| matches!(f, Finding::NotExercised { obligation, .. }
                          if *obligation == Obligation::ComposePlacesWhatItIsGiven)));
}

/// Seven name laws have five sampled checks and two shape constraints.
/// A separate level rule brings the sampled checks to six. Each shape entry
/// also names the semantic stability that Rust cannot enforce; sampling
/// and shape constraints do not prove these eight obligations.
#[test]
fn the_type_shape_constraints_and_their_semantic_limit_are_published() {
    assert_eq!(TYPE_SHAPE_CONSTRAINTS.len(), 2);
    assert_eq!(
        Obligation::ALL.len() + TYPE_SHAPE_CONSTRAINTS.len(),
        8,
        "the name obligations plus deterministic level validation"
    );
    let constrained: Vec<&str> = TYPE_SHAPE_CONSTRAINTS
        .iter()
        .map(|constraint| constraint.statement)
        .collect();
    assert!(constrained
        .iter()
        .any(|s| s.contains("positioned or distinguished")));
    assert!(constrained
        .iter()
        .any(|s| s.contains("follows from the parts")));
    assert!(TYPE_SHAPE_CONSTRAINTS
        .iter()
        .all(|constraint| constraint.assumed.contains("determin")));
}

#[allow(deprecated)]
#[test]
fn the_legacy_discharge_table_preserves_its_public_shape_without_overclaiming() {
    assert_eq!(DISCHARGED_BY_THE_TYPE_SYSTEM.len(), 2);
    assert!(DISCHARGED_BY_THE_TYPE_SYSTEM
        .iter()
        .all(|discharged| discharged.how.contains("determin")));
}

static NEXT_POSITIONED_SPECIES_IS_NODE: AtomicBool = AtomicBool::new(false);

/// A legal Rust implementation that changes both structural readings while
/// their explicit inputs remain unchanged. `view` uses interior state and
/// `positioned_species` uses global state, the two channels the signatures do
/// not exclude.
#[derive(Clone)]
struct Stateful {
    inner: SyllabusName,
    next_view_is_distinguished: Cell<bool>,
}

impl fmt::Display for Stateful {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl EntryName for Stateful {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        match SyllabusName::parse(name, found) {
            Verdict::Entry(inner) => Verdict::Entry(Self {
                inner,
                next_view_is_distinguished: Cell::new(false),
            }),
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(error) => Verdict::Malformed(error),
            Verdict::Reserved(error) => Verdict::Reserved(error),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self {
            inner: SyllabusName::compose(ordinal, key, parts),
            next_view_is_distinguished: Cell::new(false),
        }
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        let distinguished = self.next_view_is_distinguished.get();
        self.next_view_is_distinguished.set(!distinguished);
        if distinguished {
            NameView::Distinguished
        } else {
            self.inner.view()
        }
    }

    fn positioned_species(_parts: &Self::Parts) -> PositionedSpecies {
        if NEXT_POSITIONED_SPECIES_IS_NODE.fetch_xor(true, Ordering::Relaxed) {
            PositionedSpecies::Node
        } else {
            PositionedSpecies::Leaf
        }
    }
}

#[test]
fn hidden_mutable_state_is_not_excluded_by_the_trait_shape() {
    let parts = Parts::lesson(Status::Published, Label::new("stateful").unwrap());
    let name = Stateful::compose(Ordinal::new(1), Key::new(1), parts.clone());

    assert!(matches!(name.view(), NameView::Positioned(_)));
    assert!(matches!(name.view(), NameView::Distinguished));

    NEXT_POSITIONED_SPECIES_IS_NODE.store(false, Ordering::Relaxed);
    assert_eq!(
        Stateful::positioned_species(&parts),
        PositionedSpecies::Leaf
    );
    assert_eq!(
        Stateful::positioned_species(&parts),
        PositionedSpecies::Node
    );
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

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

#[test]
fn a_compose_that_ignores_its_arguments_is_caught() {
    let report = conformance::check::<Forgetful>(
        &listings(),
        &triples(),
        &[Forgetful(SyllabusName::Overview)],
        &[],
    );
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

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

#[test]
fn a_grammar_with_two_spellings_of_one_name_is_caught() {
    let mut names = listings();
    names.push(("5-draft-matrices-i6.md", Found::File));
    let report =
        conformance::check::<Lenient>(&names, &triples(), &[Lenient(SyllabusName::Overview)], &[]);
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

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

#[test]
fn a_parse_that_ignores_the_listing_is_caught() {
    let report = conformance::check::<Blind>(
        &listings(),
        &triples(),
        &[Blind(SyllabusName::Overview)],
        &[],
    );
    assert_eq!(
        violated(&report),
        vec![Obligation::ParseRefusesWhatFoundContradicts],
        "{report}"
    );
}

/// A domain with two canonical distinguished names. Their coexistence as
/// values is lawful; cardinality is a separate level obligation.
#[derive(Clone)]
enum TwoOverviews {
    Delegated(SyllabusName),
    /// An independently supplied distinguished name.
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
        if name == "INDEX.md" {
            // Not part of the defect: a name this domain owns still refuses a
            // listing that contradicts it, so the kit's verdict names the one
            // obligation this domain actually breaks.
            return match found {
                Found::File => Verdict::Entry(Self::Index),
                _ => Verdict::Malformed(SyllabusError::SpeciesMismatch {
                    name: name.to_string(),
                    declares: Species::Distinguished,
                    found,
                }),
            };
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

    fn view(&self) -> NameView<'_, Self::Parts> {
        match self {
            Self::Delegated(n) => n.view(),
            Self::Index => NameView::Distinguished,
        }
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

#[test]
fn different_distinguished_names_conform() {
    let mut names = listings();
    names.push(("INDEX.md", Found::File));
    let report = conformance::check::<TwoOverviews>(
        &names,
        &triples(),
        &[
            TwoOverviews::Delegated(SyllabusName::Overview),
            TwoOverviews::Index,
        ],
        &accepting_levels(TwoOverviews::Index),
    );
    report.assert_conforming();
}

/// `seam-k17`'s third finding: a domain that *detects* the contradiction and
/// then answers `Foreign` rather than `Malformed`. Nothing is lost at the
/// boundary — the name is refused either way — and everything is lost in the
/// tree: `Foreign` means *not mine*, so a walk skips the entry silently, and
/// skips the whole subtree under it when the contradiction is a directory
/// wearing a leaf's name. That is exactly the tree
/// `witness_species_mismatch_is_unclassifiable` exhibits, reached through the
/// one verdict the old kit accepted.
#[derive(Clone)]
struct Evasive(SyllabusName);
delegate!(Evasive);

impl EntryName for Evasive {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        match SyllabusName::parse(name, found) {
            Verdict::Entry(n) => Verdict::Entry(Self(n)),
            Verdict::Foreign => Verdict::Foreign,
            // The defect: the one refusal that halts becomes the one that skips.
            Verdict::Malformed(SyllabusError::SpeciesMismatch { .. }) => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        Self(SyllabusName::compose(ordinal, key, parts))
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

#[test]
fn a_species_contradiction_disguised_as_foreign_is_caught() {
    let report = conformance::check::<Evasive>(
        &listings(),
        &triples(),
        &[Evasive(SyllabusName::Overview)],
        &[],
    );
    assert_eq!(
        violated(&report),
        vec![Obligation::ParseRefusesWhatFoundContradicts],
        "{report}"
    );
}

/// `seam-k17`'s fourth finding, and `witness_two_filenames_name_one_entry` from
/// the other end: a grammar whose round trip is exact on the *string* and wrong
/// on the *name*. It renders what it was composed with and parses that same
/// spelling into a different key, so nothing on disk looks amiss while every
/// snapshot reads an identity that was never written. `RoundTripDisplay` is
/// `v.seen = n` — the same name, not a name that spells the same.
#[derive(Clone)]
struct KeyDrift {
    shown: String,
    inner: SyllabusName,
}

impl fmt::Display for KeyDrift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.shown)
    }
}

impl KeyDrift {
    /// The defect: what was read is not what the string said.
    fn drifted(shown: &str, inner: SyllabusName) -> Self {
        let inner = match inner {
            SyllabusName::Positioned {
                ordinal,
                key,
                parts,
            } => SyllabusName::Positioned {
                ordinal,
                key: Key::new(key.get() + 1),
                parts,
            },
            other => other,
        };
        Self {
            shown: shown.to_string(),
            inner,
        }
    }
}

impl EntryName for KeyDrift {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        match SyllabusName::parse(name, found) {
            Verdict::Entry(n) => Verdict::Entry(Self::drifted(name, n)),
            Verdict::Foreign => Verdict::Foreign,
            Verdict::Malformed(e) => Verdict::Malformed(e),
            Verdict::Reserved(e) => Verdict::Reserved(e),
        }
    }

    fn compose(ordinal: Ordinal, key: Key, parts: Self::Parts) -> Self {
        let inner = SyllabusName::compose(ordinal, key, parts);
        Self {
            shown: inner.to_string(),
            inner,
        }
    }

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.inner.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

#[test]
fn a_parse_that_changes_the_key_behind_an_exact_display_is_caught() {
    let report = conformance::check::<KeyDrift>(
        &listings(),
        &triples(),
        &[KeyDrift {
            inner: SyllabusName::Overview,
            shown: "OVERVIEW.md".to_string(),
        }],
        &[],
    );
    assert_eq!(
        violated(&report),
        vec![Obligation::TheGrammarIsCanonical],
        "{report}"
    );
}

// ===========================================================================
// The seventh obligation: a name renders as one path component.
//
// The one obligation with no `structure.als` witness behind it — both models
// hold no strings, so neither can pose a rendering — and the one the library
// enforces rather than assumes. This domain is the review's own adversary:
// every other obligation holds, including canonicity, because its `parse`
// accepts exactly the spellings its `Display` produces.
// ===========================================================================

/// A domain whose grammar spells every name with a leading `../`.
#[derive(Clone)]
struct Escaping(SyllabusName);

impl fmt::Display for Escaping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "../{}", self.0)
    }
}

impl EntryName for Escaping {
    type Parts = Parts;
    type Err = SyllabusError;

    fn parse(name: &str, found: Found) -> Verdict<Self, Self::Err> {
        // Canonical: the only spellings this domain claims are the ones it
        // renders, so `format(parse(f)) == f` holds throughout.
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

    fn view(&self) -> NameView<'_, Self::Parts> {
        self.0.view()
    }

    fn positioned_species(parts: &Self::Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(parts)
    }
}

/// The samples this domain is at home in: its own spellings, which is what
/// makes every *other* obligation exercised and held, so the kit's verdict has
/// exactly one thing in it.
fn escaping_listings() -> Vec<(&'static str, Found)> {
    vec![
        ("../OVERVIEW.md", Found::File),
        ("../01-published-orientation-i1.md", Found::File),
        ("../02-linear-algebra-i2", Found::Dir),
        ("README.md", Found::File),
    ]
}

#[test]
fn a_name_that_renders_as_more_than_one_component_is_caught() {
    let report = conformance::check::<Escaping>(
        &escaping_listings(),
        &triples(),
        &[Escaping(SyllabusName::Overview)],
        &[],
    );
    assert_eq!(
        violated(&report),
        vec![Obligation::ANameRendersAsOnePathComponent],
        "every other obligation holds — canonicity included, which is what makes \
         this the escape the algebra cannot see: {report}"
    );
}

#[test]
fn distinguished_identity_compares_rendered_names() {
    let overview = TwoOverviews::Delegated(SyllabusName::Overview);
    assert!(!overview.same_name(&TwoOverviews::Index));
    assert!(TwoOverviews::Index.same_name(&TwoOverviews::Index));
}

// `operations.qnt`'s `inv_initializeUsesSuppliedName` pins the created name.
// A fixed OVERVIEW fails the INDEX checks; byte preservation is outside the model.
#[test]
fn supplied_names_initialize_and_promote_on_disk() {
    use ordinal_fs_tree::NewEntry;
    let temporary = tempfile::tempdir().unwrap();
    let root = temporary.path().join("syllabus");
    let parts = Parts::lesson(Status::Draft, Label::new("orientation").unwrap());
    ordinal_fs_tree::fs::write::<TwoOverviews>(&root)
        .unwrap()
        .expect_vacancy("absent root")
        .initialize(
            Some((TwoOverviews::Index, b"root".to_vec())),
            vec![NewEntry::new(parts, b"lesson".to_vec())],
        )
        .unwrap();
    assert_eq!(std::fs::read(root.join("INDEX.md")).unwrap(), b"root");
    ordinal_fs_tree::fs::write::<TwoOverviews>(&root)
        .unwrap()
        .expect_tree("initialized root")
        .promote(
            Key::new(1),
            Parts::module(Label::new("orientation").unwrap()),
            TwoOverviews::Delegated(SyllabusName::Overview),
            None,
        )
        .unwrap();
    assert_eq!(
        std::fs::read(root.join("01-orientation-i1/OVERVIEW.md")).unwrap(),
        b"lesson"
    );
    let tree = ordinal_fs_tree::fs::read::<TwoOverviews>(&root)
        .unwrap()
        .expect_tree("valid names");
    assert_eq!(
        tree.snapshot()
            .root()
            .distinguished()
            .unwrap()
            .name()
            .to_string(),
        "INDEX.md"
    );
}
