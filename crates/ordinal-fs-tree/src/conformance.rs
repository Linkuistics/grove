//! The conformance kit: hand it sample names and sample triples, and learn
//! which of the trait's obligations your implementation violates.
//!
//! The five obligations under [`EntryName`] are the **consumer's**, and the
//! library cannot check any of them from inside an operation — a design missing
//! any one of them admits a tree the library will quietly corrupt, and it
//! corrupts it silently, in a tree someone is using. This module is where a
//! domain finds that out instead, from a test, before there is a tree.
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

use crate::{EntryName, Found, Species, Verdict};

/// One of the obligations this kit checks.
///
/// Four, not five. The fifth — *a name is positioned or distinguished, never
/// neither* — is discharged by the type system and is listed in
/// [`DISCHARGED_BY_THE_TYPE_SYSTEM`] rather than checked here.
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
}

impl Obligation {
    /// Every obligation this kit checks.
    pub const ALL: [Self; 4] = [
        Self::ComposePlacesWhatItIsGiven,
        Self::TheGrammarIsCanonical,
        Self::DistinguishedNamesTheOnlyEntryOfItsSpecies,
        Self::ParseRefusesWhatFoundContradicts,
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
        }
    }

    /// What a tree looks like when this obligation does not hold. Every one of
    /// these is a structure `docs/ordinal-fs-tree/models/structure.als`
    /// produces on demand, under the named `witness_…` command.
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
        }
    }
}

impl fmt::Display for Obligation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.statement())
    }
}

/// An obligation the target language already makes unrepresentable, and how.
pub struct Discharged {
    /// The obligation, as the architecture document states it.
    pub statement: &'static str,
    /// What makes it free.
    pub how: &'static str,
}

/// The obligations this kit does **not** check because Rust does not admit a
/// violation of them.
///
/// Reporting them is the point: a consumer reading four checks where the
/// document states five needs to know that the fifth was not forgotten. The
/// finding that produced this list is `docs/formalism-findings.md` entry 002 —
/// *before modelling a structural property, ask whether the target language
/// already forbids it*.
pub const DISCHARGED_BY_THE_TYPE_SYSTEM: &[Discharged] = &[Discharged {
    statement: "a name is positioned or distinguished, never neither",
    how: "`EntryName::triple` returns one `Option` over the ordinal, the key and the \
          parts together, so a name carrying some of the three and not the others cannot \
          be written. The document states the obligation of three separate `Option` \
          accessors, where it is a real thing to get wrong \
          (witness_leaf_name_without_an_ordinal).",
}];

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
        self.findings
            .push(Finding::Violated { obligation, detail });
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
        match name.triple() {
            None => report.violate(
                Obligation::ComposePlacesWhatItIsGiven,
                format!(
                    "compose(ordinal {ordinal}, key {key}, …) produced `{name}`, whose \
                     triple() is None. A composed name is positioned by construction."
                ),
            ),
            Some(t) => {
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

    // --- the grammar is canonical -----------------------------------------
    //
    // Both directions, because *isomorphic* means both: a filename that parses
    // must render back to itself, and a composed name must parse back to
    // itself. Alloy: `ParseIsCanonical` and `RoundTripDisplay`.
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
    for name in &composed {
        let rendered = name.to_string();
        match N::parse(&rendered, name.species().requires()) {
            Verdict::Entry(reparsed) => {
                let again = reparsed.to_string();
                if again != rendered {
                    report.violate(
                        Obligation::TheGrammarIsCanonical,
                        format!("composed `{rendered}` parses to a name rendering as `{again}`."),
                    );
                }
            }
            _ => report.violate(
                Obligation::TheGrammarIsCanonical,
                format!(
                    "composed `{rendered}` does not parse back to an entry. Every name \
                     the consumer can produce must be one the consumer can read."
                ),
            ),
        }
    }

    // --- distinguished() names the only entry of its species ---------------
    //
    // Alloy: `OneDistinguishedName` and `DistLawful`, checked as
    // `DistinguishedIsUniquePerNode`. This is also where the half of
    // *positioned or distinguished* that the type system does not cover lives —
    // a name that is positioned *and* claims the distinguished species.
    let distinguished = N::distinguished();
    let distinguished_name = distinguished.as_ref().map(ToString::to_string);
    if let Some(d) = &distinguished {
        let rendered = d.to_string();
        if d.species() != Species::Distinguished {
            report.violate(
                Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                format!("distinguished() returned `{rendered}`, whose species is {}.", d.species()),
            );
        }
        if d.triple().is_some() {
            report.violate(
                Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                format!(
                    "distinguished() returned `{rendered}`, which carries a triple. A \
                     distinguished child has neither an ordinal nor a key."
                ),
            );
        }
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
    for name in &composed {
        if name.species() == Species::Distinguished {
            report.violate(
                Obligation::DistinguishedNamesTheOnlyEntryOfItsSpecies,
                format!(
                    "compose produced `{name}`, whose species is the distinguished one. A \
                     composed name carries an ordinal and a key; a distinguished child \
                     carries neither."
                ),
            );
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
        let mut entries = 0;
        for found in every_found {
            if let Verdict::Entry(name) = N::parse(filename, found) {
                entries += 1;
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
        }
        if entries > 0 && entries < every_found.len() && index < supplied {
            refused = true;
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
            "every sample name that parsed did so under all three `Found` values, so no \
             contradiction was ever refused. Supply a name of a species that requires a \
             file, or one that requires a directory.",
        );
    }

    report
}
