//! The conformance kit: hand it sample names and sample triples, and learn
//! which of the trait's obligations your implementation violates.
//!
//! The seven obligations under [`EntryName`] are the **consumer's**, and the
//! library can check only the last of them from inside an operation — a design
//! missing any one of the other six admits a tree the library will quietly
//! corrupt, and it corrupts it silently, in a tree someone is using. This module
//! is where a domain finds that out instead, from a test, before there is a
//! tree. The seventh is here too, because meeting it as an
//! [`Error::NameIsNotOneComponent`] in an operation is worse than meeting it in
//! a test, even though it is not silent.
//!
//! [`Error::NameIsNotOneComponent`]: crate::Error::NameIsNotOneComponent
//!
//! ```no_run
//! # use ordinal_fs_tree::{conformance, reference::{SyllabusName, Parts, Status, Label}, Found, Ordinal, Key};
//! let report = conformance::check::<SyllabusName>(
//!     &[("01-draft-vectors-i1.md", Found::File), ("README.md", Found::File)],
//!     &[(Ordinal::new(1), Key::new(1), Parts::lesson(Status::Draft, Label::new("vectors").unwrap()))],
//! );
//! report.assert_conforming();
//! ```
//!
//! # Two kinds of finding, and why the second one exists
//!
//! A kit that only reports violations reads exactly the same when it is handed
//! nothing to check: no samples, no violations, conforming. That is the failure
//! this workstream has already met three times — a model, a tool and a runner
//! each reporting *found nothing* and *succeeded* with the same bytes
//! (`docs/formalism-findings.md`, entry 003). So the kit also reports which
//! obligations were never **exercised**, and [`Report::is_conforming`] is false
//! while any of them is. A suite of must-hold claims cannot detect that it did
//! not run; one that also says what it reached can.

use core::fmt;

use crate::{EntryName, EntryNameExt, Found, NameView, Species, Verdict};

/// One of the obligations this kit checks.
///
/// Five, not seven. Rust constrains the visible shape of the other two — *a
/// name is positioned or distinguished, never neither* and *the species
/// follows from the parts* — and those constraints are listed in
/// [`TYPE_SHAPE_CONSTRAINTS`]. Their stability across calls remains a semantic
/// law: neither Rust nor this sample-based kit proves the absence of hidden
/// mutable state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Obligation {
    /// `compose(o, k, p)` yields a name whose triple is `Some` and equal to
    /// `(o, k, p)`.
    ComposePlacesWhatItIsGiven,
    /// Distinct filenames never parse to the same name: `format(parse(f)) == f`,
    /// and every name the consumer can produce parses back to itself.
    TheGrammarIsCanonical,
    /// `parse` yields species `Distinguished` for `distinguished()` and for
    /// nothing else, and that name carries no triple.
    DistinguishedNamesTheOnlyEntryOfItsSpecies,
    /// A name declaring a species the listing contradicts is `Malformed`, never
    /// `Entry`.
    ParseRefusesWhatFoundContradicts,
    /// Every name the domain renders is exactly one filename: not empty, not
    /// `.` or `..`, and holding no path separator.
    ///
    /// The one obligation the library also enforces, so a domain that skips
    /// this check meets it as an `Error` rather than as a corrupted tree. It is
    /// checked here anyway, because a test is a cheaper place to meet it than
    /// an operation.
    ANameRendersAsOnePathComponent,
}

impl Obligation {
    /// Every obligation this kit checks.
    pub const ALL: [Self; 5] = [
        Self::ComposePlacesWhatItIsGiven,
        Self::TheGrammarIsCanonical,
        Self::DistinguishedNamesTheOnlyEntryOfItsSpecies,
        Self::ParseRefusesWhatFoundContradicts,
        Self::ANameRendersAsOnePathComponent,
    ];

    /// The obligation as the architecture document states it.
    #[must_use]
    pub const fn statement(self) -> &'static str {
        match self {
            Self::ComposePlacesWhatItIsGiven => "compose places what it is given",
            Self::TheGrammarIsCanonical => "the grammar is canonical",
            Self::DistinguishedNamesTheOnlyEntryOfItsSpecies => {
                "distinguished() names the only entry of its species"
            }
            Self::ParseRefusesWhatFoundContradicts => "parse refuses what found contradicts",
            Self::ANameRendersAsOnePathComponent => "a name renders as one path component",
        }
    }

    /// What a tree looks like when this obligation does not hold.
    ///
    /// Every one of these but the last is a structure
    /// `docs/ordinal-fs-tree/models/structure.als` produces on demand, under the
    /// named `witness_…` command. The last has no witness and can have none:
    /// both models hold no strings by design, so a rendering that is not a
    /// filename is not a thing either can say — which is why it is the one
    /// obligation the library enforces instead of assuming.
    #[must_use]
    pub const fn what_it_admits(self) -> &'static str {
        match self {
            Self::ComposePlacesWhatItIsGiven => {
                "a sibling shift that moves one entry's key onto another's position, \
                 while every stated invariant still holds (witness_shift_corrupts_identity)"
            }
            Self::TheGrammarIsCanonical => {
                "two files on disk that are one entry, sharing a key and an ordinal \
                 (witness_two_filenames_name_one_entry)"
            }
            Self::DistinguishedNamesTheOnlyEntryOfItsSpecies => {
                "a node holding two distinguished children (witness_two_distinguished_children)"
            }
            Self::ParseRefusesWhatFoundContradicts => {
                "a directory wearing a leaf's name, or a distinguished child that is a \
                 directory — either way an entire subtree invisible to every traversal \
                 (witness_species_mismatch_is_unclassifiable, \
                 witness_distinguished_directory_hides_a_subtree)"
            }
            Self::ANameRendersAsOnePathComponent => {
                "a create, a rename, a rollback removal and a reported path addressing \
                 outside the tree whose containing directory is the only thing locked \
                 — the library joins this rendering to a level's directory, while the \
                 algebra compares views and sees a perfectly canonical name (no model \
                 witness: both models hold no strings)"
            }
        }
    }
}

impl fmt::Display for Obligation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.statement())
    }
}

/// The part of an obligation constrained by Rust's type shape.
pub struct TypeShapeConstraint {
    /// The obligation, as the architecture document states it.
    pub statement: &'static str,
    /// What the type shape enforces for each call.
    pub enforced: &'static str,
    /// The semantic law that the type shape does not enforce.
    pub assumed: &'static str,
}

/// The structural constraints this kit does **not** sample because each return
/// value already has the required Rust shape.
///
/// Reporting them is the point: a consumer reading five checks where the
/// document states seven needs to know that the other two were not forgotten.
/// This table deliberately does not call either obligation discharged: trait
/// methods may consult interior or global mutable state, so identical explicit
/// inputs can produce different well-shaped answers across calls. A finite
/// conformance sample could expose an implementation that changes during that
/// sample, but could not prove deterministic behavior in general.
pub const TYPE_SHAPE_CONSTRAINTS: &[TypeShapeConstraint] = &[
    TypeShapeConstraint {
        statement: "a name is positioned or distinguished, never neither",
        enforced: "Each `EntryName::view` call returns one `NameView`: either a `Triple` — \
                   the ordinal, key and parts together — or the distinguished child, which \
                   has none of them. A single returned value cannot carry only some triple \
                   fields or carry a triple while claiming the distinguished species.",
        assumed: "Repeated `EntryName::view` calls with the same receiver and no \
                  caller-visible mutation are deterministic; hidden mutable state does not \
                  influence the variant or triple.",
    },
    TypeShapeConstraint {
        statement: "the species follows from the parts",
        enforced: "`EntryName::positioned_species` is an associated function whose only \
                   explicit input is `&Parts`: it receives no `self`, ordinal or key.",
        assumed: "`EntryName::positioned_species` is deterministic from the parts value \
                  across calls and does not derive its answer from hidden mutable state. A \
                  `Parts` equality coarser than the species remains lawful; occupancy uses \
                  `EntryNameExt::same_name`, which compares both view and species.",
    },
];

/// Legacy description of a type-shape constraint.
#[deprecated(note = "use TypeShapeConstraint and TYPE_SHAPE_CONSTRAINTS")]
pub struct Discharged {
    /// The obligation, as the architecture document states it.
    pub statement: &'static str,
    /// What Rust constrains and which deterministic behavior remains assumed.
    pub how: &'static str,
}

/// Compatibility view of [`TYPE_SHAPE_CONSTRAINTS`] under its original name.
///
/// The name is retained for source compatibility, not as a claim that Rust
/// proves call stability. New code should use [`TYPE_SHAPE_CONSTRAINTS`].
#[allow(deprecated)]
#[deprecated(note = "use TYPE_SHAPE_CONSTRAINTS; call stability is a semantic law")]
pub const DISCHARGED_BY_THE_TYPE_SYSTEM: &[Discharged] = &[
    Discharged {
        statement: "a name is positioned or distinguished, never neither",
        how: "Each call returns one complete `NameView`; repeated calls with the same \
              receiver are assumed deterministic because hidden state is not excluded.",
    },
    Discharged {
        statement: "the species follows from the parts",
        how: "The only explicit input is `&Parts`; the answer is assumed deterministic from \
              that input because global mutable state is not excluded.",
    },
];

/// What the kit learned about one obligation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Finding {
    /// The implementation breaks this obligation. `detail` says how.
    Violated {
        /// The broken obligation.
        obligation: Obligation,
        /// The concrete case that broke it.
        detail: String,
    },
    /// The samples never put this obligation to the test, so a pass would mean
    /// nothing. `detail` says what sample was missing.
    NotExercised {
        /// The untested obligation.
        obligation: Obligation,
        /// What the samples would need to contain.
        detail: String,
    },
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Violated { obligation, detail } => {
                write!(f, "VIOLATED   {obligation}\n           {detail}")
            }
            Self::NotExercised { obligation, detail } => {
                write!(f, "UNTESTED   {obligation}\n           {detail}")
            }
        }
    }
}

/// What [`check`] found.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    findings: Vec<Finding>,
}

impl Report {
    /// Every finding, violations and untested obligations alike.
    #[must_use]
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    /// Only the obligations the implementation actually breaks.
    pub fn violations(&self) -> impl Iterator<Item = &Finding> {
        self.findings
            .iter()
            .filter(|f| matches!(f, Finding::Violated { .. }))
    }

    /// Only the obligations the samples never reached.
    pub fn unexercised(&self) -> impl Iterator<Item = &Finding> {
        self.findings
            .iter()
            .filter(|f| matches!(f, Finding::NotExercised { .. }))
    }

    /// True when every obligation was exercised and none was violated.
    ///
    /// Untested counts against it deliberately: see this module's header.
    #[must_use]
    pub fn is_conforming(&self) -> bool {
        self.findings.is_empty()
    }

    /// Panic with the whole report unless [`is_conforming`](Report::is_conforming).
    ///
    /// The one line a consumer's own test needs.
    #[track_caller]
    pub fn assert_conforming(&self) {
        assert!(self.is_conforming(), "{self}");
    }

    fn violate(&mut self, obligation: Obligation, detail: String) {
        self.findings.push(Finding::Violated { obligation, detail });
    }

    fn untested(&mut self, obligation: Obligation, detail: &str) {
        self.findings.push(Finding::NotExercised {
            obligation,
            detail: detail.to_string(),
        });
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.findings.is_empty() {
            return write!(
                f,
                "conforming: all {} checked obligations exercised and held",
                Obligation::ALL.len()
            );
        }
        writeln!(f, "{} finding(s):", self.findings.len())?;
        for finding in &self.findings {
            writeln!(f, "{finding}")?;
        }
        Ok(())
    }
}

/// Check an [`EntryName`] implementation against the obligations the library
/// assumes and cannot enforce.
///
/// `listings` are sample directory entries — a filename and what the listing
/// reports is under it — and should include the domain's own well-formed names,
/// its distinguished child, at least one foreign name, and any near-miss the
/// grammar is meant to refuse. `triples` are sample `(ordinal, key, parts)`
/// values, one per species the domain composes.
///
/// The samples do not have to be exhaustive and cannot be: this is a test kit,
/// not a proof. What it does guarantee is that it says so when they are too
/// thin to test something — see [`Report::is_conforming`].
#[must_use]
pub fn check<N: EntryName>(
    listings: &[(&str, Found)],
    triples: &[(crate::Ordinal, crate::Key, N::Parts)],
) -> Report {
    let mut report = Report::default();

    // --- compose places what it is given ----------------------------------
    //
    // Alloy: `ComposeLawful`, checked as `ShiftPreservesIdentity`. Without it a
    // shift is a corruption, because a shift is nothing but a compose.
    let composed: Vec<N> = triples
        .iter()
        .map(|(o, k, p)| N::compose(*o, *k, p.clone()))
        .collect();
    for ((ordinal, key, parts), name) in triples.iter().zip(&composed) {
        match name.view() {
            NameView::Distinguished => report.violate(
                Obligation::ComposePlacesWhatItIsGiven,
                format!(
                    "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, which is \
                     the distinguished child and carries no triple. A composed name is \
                     positioned by construction."
                ),
            ),
            NameView::Positioned(t) => {
                if t.ordinal != *ordinal {
                    report.violate(
                        Obligation::ComposePlacesWhatItIsGiven,
                        format!(
                            "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, \
                             which reports ordinal {}.",
                            t.ordinal
                        ),
                    );
                }
                if t.key != *key {
                    report.violate(
                        Obligation::ComposePlacesWhatItIsGiven,
                        format!(
                            "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, \
                             which reports key {}.",
                            t.key
                        ),
                    );
                }
                if t.parts != parts {
                    report.violate(
                        Obligation::ComposePlacesWhatItIsGiven,
                        format!(
                            "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, \
                             whose parts are not the ones it was given."
                        ),
                    );
                }
            }
        }
    }
    if triples.is_empty() {
        report.untested(
            Obligation::ComposePlacesWhatItIsGiven,
            "no sample triples were supplied, so `compose` was never called.",
        );
    }

    let distinguished = N::distinguished();
    let distinguished_name = distinguished.as_ref().map(ToString::to_string);

    // --- the grammar is canonical -----------------------------------------
    //
    // Both directions, because *isomorphic* means both: a filename that parses
    // must render back to itself, and a name the consumer can produce must
    // parse back to *that name*. Alloy: `ParseIsCanonical` and
    // `RoundTripDisplay`.
    let mut parsed_any_listing = false;
    for (filename, found) in listings {
        if let Verdict::Entry(name) = N::parse(filename, *found) {
            parsed_any_listing = true;
            let rendered = name.to_string();
            if rendered != *filename {
                report.violate(
                    Obligation::TheGrammarIsCanonical,
                    format!(
                        "`{filename}` parsed to a name that renders as `{rendered}`. Two \
                         spellings of one name means two files on disk are one entry."
                    ),
                );
            }
        }
    }
    if !parsed_any_listing {
        report.untested(
            Obligation::TheGrammarIsCanonical,
            "no sample listing parsed to an entry, so no filename was rendered back. \
             Supply at least one well-formed name of each species.",
        );
    }
    // `RoundTripDisplay` is `v.seen = n`: the name that comes back is *the same
    // name*, not merely one that renders the same way. Comparing renderings
    // alone lets a domain parse its own output into a different triple while
    // `Display` keeps saying what it said — the strings agree, and every
    // snapshot then reads an ordinal and a key that were never composed. The
    // distinguished spelling goes through the same check, where the thing to
    // come back is the absence of a triple.
    for name in composed.iter().chain(distinguished.as_ref()) {
        let rendered = name.to_string();
        match N::parse(&rendered, name.species().requires()) {
            Verdict::Entry(reparsed) => {
                let again = reparsed.to_string();
                if again != rendered {
                    report.violate(
                        Obligation::TheGrammarIsCanonical,
                        format!("`{rendered}` parses to a name rendering as `{again}`."),
                    );
                } else if reparsed.view() != name.view() {
                    report.violate(
                        Obligation::TheGrammarIsCanonical,
                        format!(
                            "`{rendered}` renders back to itself but parses to a different \
                             name — its ordinal, key or parts are not the ones that were \
                             composed. Every operation reads the triple, not the string."
                        ),
                    );
                } else if reparsed.species() != name.species() {
                    // Implied by the views agreeing, since the species is read
                    // off the view; stated because the obligation is stated of
                    // both and a later change to either derivation lands here.
                    report.violate(
                        Obligation::TheGrammarIsCanonical,
                        format!(
                            "`{rendered}` parses back to a {} where it was composed as a {}.",
                            reparsed.species(),
                            name.species()
                        ),
                    );
                }
            }
            _ => report.violate(
                Obligation::TheGrammarIsCanonical,
                format!(
                    "`{rendered}` does not parse back to an entry. Every name the consumer \
                     can produce must be one the consumer can read."
                ),
            ),
        }
    }

    // --- a name renders as one path component ------------------------------
    //
    // No Alloy claim, and there cannot be one: `structure.als` holds no strings,
    // so it cannot pose a rendering at all. This is the obligation the library
    // enforces at both boundaries where a name becomes a path, and the kit
    // checks it so that a domain meets it in a test rather than in an operation.
    //
    // Every name the domain can *produce* is a candidate: what it composes, what
    // it parses out of a listing, and its distinguished child.
    // `distinguished()` is checked like any other name but does not *count* as
    // coverage, for the reason the found-contradicts check gives: a domain that
    // supplies its own name would otherwise let a kit handed no samples at all
    // report this obligation as exercised.
    let mut rendered_any = !composed.is_empty();
    let render_check = |name: &N, report: &mut Report| {
        let rendered = name.to_string();
        if let Some(reason) = crate::name::not_one_component(&rendered) {
            report.violate(
                Obligation::ANameRendersAsOnePathComponent,
                format!(
                    "`{rendered}` {reason}, so it is not one filename. The library joins \
                     a rendering to a level's directory to reach the entry, so this one \
                     addresses outside the tree."
                ),
            );
        }
    };
    for name in composed.iter().chain(distinguished.as_ref()) {
        render_check(name, &mut report);
    }
    for (filename, found) in listings {
        if let Verdict::Entry(name) = N::parse(filename, *found) {
            rendered_any = true;
            render_check(&name, &mut report);
        }
    }
    if !rendered_any {
        report.untested(
            Obligation::ANameRendersAsOnePathComponent,
            "no sample yielded a name, so nothing was rendered. Supply a triple to \
             compose, or a listing this domain recognises.",
        );
    }

    // --- distinguished() names the only entry of its species ---------------
    //
    // Alloy: `OneDistinguishedName` and `DistLawful`, checked as
    // `DistinguishedIsUniquePerNode`. That `distinguished()` itself carries no
    // triple is no longer checkable — `NameView::Distinguished` holds none —
    // so what is left is the half about every *other* name.
    if let Some(d) = &distinguished {
        let rendered = d.to_string();
        match N::parse(&rendered, Found::File) {
            Verdict::Entry(n) if n.species() == Species::Distinguished => {}
            Verdict::Entry(n) => report.violate(
                Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                format!("`{rendered}` parses as {} rather than as the distinguished child.", n.species()),
            ),
            _ => report.violate(
                Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                format!("`{rendered}` is the name distinguished() returns and does not parse as an entry."),
            ),
        }
    }
    for (filename, found) in listings {
        if let Verdict::Entry(name) = N::parse(filename, *found) {
            if name.species() == Species::Distinguished {
                match &distinguished_name {
                    Some(d) if d == filename => {}
                    Some(d) => report.violate(
                        Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                        format!(
                            "`{filename}` parses as a distinguished child, but distinguished() \
                             returns `{d}`. A node could then hold both."
                        ),
                    ),
                    None => report.violate(
                        Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                        format!(
                            "`{filename}` parses as a distinguished child in a domain whose \
                             distinguished() is None."
                        ),
                    ),
                }
            }
        }
    }
    // Checking `distinguished()` against itself is half the obligation. The other
    // half — that *no other* name claims that species — needs names to look at,
    // and a domain whose own distinguished child is the only thing the kit saw
    // has not been asked the question at all.
    if !parsed_any_listing && composed.is_empty() {
        report.untested(
            Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
            "no supplied sample yielded a name, so nothing showed that no name other \
             than distinguished() claims that species.",
        );
    }

    // --- parse refuses what found contradicts ------------------------------
    //
    // Every sample name is offered under all three `Found` values, not only the
    // one it was paired with: the obligation is about what `parse` does when the
    // listing contradicts the name, and a sample that is only ever shown its own
    // truth never asks the question. Alloy: `SpeciesAgreementIsParsed`.
    //
    // The refusal has to be `Malformed`, and that is the whole of the
    // obligation rather than a detail of it: `Foreign` means *not my name*, and
    // a walk skips a foreign name silently — together with everything beneath
    // it when it is a directory. A domain that answers `Foreign` where its own
    // name contradicts the listing has hidden exactly the subtree this
    // obligation exists to expose, so the kit only accepts a halt that carries
    // the domain's own error.
    let mut agreed = false;
    let mut refused = false;
    let every_found = [Found::File, Found::Dir, Found::Other];
    let mut candidates: Vec<String> = listings.iter().map(|(f, _)| (*f).to_string()).collect();
    candidates.extend(composed.iter().map(ToString::to_string));
    // `distinguished()` is checked like any other name but does not *count* as
    // coverage: a domain that supplies its own name would otherwise let a kit
    // handed no samples at all report this obligation as exercised, which is
    // the failure mode the two kinds of finding exist to prevent.
    let supplied = candidates.len();
    if let Some(d) = &distinguished_name {
        candidates.push(d.clone());
    }
    for (index, filename) in candidates.iter().enumerate() {
        let mut recognised = false;
        let mut not_entry: Vec<(Found, &'static str)> = Vec::new();
        for found in every_found {
            match N::parse(filename, found) {
                Verdict::Entry(name) => {
                    recognised = true;
                    if name.species().agrees_with(found) {
                        agreed |= index < supplied;
                    } else {
                        report.violate(
                            Obligation::ParseRefusesWhatFoundContradicts,
                            format!(
                                "`{filename}` over {found} parsed as an entry of species {}, \
                                 which requires {}. That is how a subtree disappears from every \
                                 traversal while the tree reports itself healthy.",
                                name.species(),
                                name.species().requires()
                            ),
                        );
                    }
                }
                Verdict::Foreign => not_entry.push((found, "Foreign")),
                Verdict::Malformed(_) => not_entry.push((found, "Malformed")),
                Verdict::Reserved(_) => not_entry.push((found, "Reserved")),
            }
        }
        // A name this domain never accepts is its own affair: whether it is
        // foreign or reserved says nothing about this obligation. Only a name
        // the domain *does* recognise can contradict a listing.
        if !recognised {
            continue;
        }
        for (found, verdict) in not_entry {
            if verdict == "Malformed" {
                refused |= index < supplied;
            } else {
                report.violate(
                    Obligation::ParseRefusesWhatFoundContradicts,
                    format!(
                        "`{filename}` parses as an entry under another listing, so this \
                         domain owns the name; over {found} it answers {verdict}. A \
                         contradiction must be Malformed, carrying this domain's own error: \
                         Foreign is skipped silently, taking the whole subtree with it when \
                         the name is a directory, and Reserved says the name is deliberately \
                         not an entry, which this one is not."
                    ),
                );
            }
        }
    }
    if !agreed {
        report.untested(
            Obligation::ParseRefusesWhatFoundContradicts,
            "no sample name parsed as an entry under any `Found`, so the agreeing case \
             was never seen.",
        );
    } else if !refused {
        report.untested(
            Obligation::ParseRefusesWhatFoundContradicts,
            "no sample name that parsed was refused as Malformed under a contradicting \
             `Found`, so no contradiction was ever put to the domain. Supply a name of a \
             species that requires a file, or one that requires a directory.",
        );
    }

    report
}
