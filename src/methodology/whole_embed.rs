//! The half of the build gate that no single file can decide — **one
//! implementation, compiled twice**, exactly like the reader beside it.
//!
//! [`super::parse`] answers everything a single `(path, text)` settles. Three
//! malformations are left over, and all three need the *assembled* unit set:
//!
//! * **A duplicate id anywhere in the embed.** An id is the verb's only address
//!   (`grove-llm methodology <id>`), so a collision between two files makes both
//!   units unaddressable rather than merely untidy — one answers and the other
//!   silently never does.
//! * **A `defers=` member that names no declared unit, or one whose unit is
//!   triggering.** A dangling target promises a body that does not exist; a
//!   triggering target delivers a *condition* where a procedure was promised.
//! * **A procedural unit no chain of deferrals reaches.** This is partition seen
//!   from the other end. Partition makes unclassified prose impossible;
//!   reachability makes an undiscoverable procedure impossible, and the two
//!   together say every byte of the methodology is either in a mandate or
//!   reachable from one.
//!
//! All three are a **contributor's** mistake, visible to the build that produced
//! the embed, so all three fail `cargo build`
//! (`docs/specs/mandate-delivered-methodology.md`, *A malformed embed fails the
//! build*). The unknown id a *caller* passes to `grove-llm methodology` is the
//! other species and stays a runtime error: it is visible only when the call is
//! made.
//!
//! This module is `#[path]`-included into the build script alongside the parser,
//! which is why it reaches its sibling as `super::parse` rather than by an
//! absolute path — `super` is `methodology` in the crate and the build script's
//! own root in `build.rs`, and the one spelling resolves in both.

use super::parse::{Class, Unit};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// What is wrong with the set, without where it is wrong — the same split
/// [`super::parse::Fault`] makes, so a test can name the class it provoked
/// without matching on a rendered string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    /// Both sites are named: the error is located at the second occurrence, and
    /// carries the first, because a contributor has to see the pair to know
    /// which one to rename.
    DuplicateId {
        id: String,
        first_file: String,
        first_line: usize,
    },
    UnknownDeferral {
        unit: String,
        target: String,
    },
    DeferralToTriggering {
        unit: String,
        target: String,
        target_file: String,
        target_line: usize,
    },
    /// Every unreachable procedure at once, rather than the first. They are
    /// almost always one authoring mistake — a renamed target, a chain detached
    /// at its root — and reporting them one build at a time would make a
    /// contributor rediscover that fact once per unit.
    UnreachableProcedures {
        ids: Vec<String>,
    },
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::DuplicateId {
                id,
                first_file,
                first_line,
            } => write!(
                f,
                "unit id `{id}` is already declared at {first_file}:{first_line}; ids are the \
                 only address `grove-llm methodology` has, so they are unique across the whole \
                 embed"
            ),
            Fault::UnknownDeferral { unit, target } => write!(
                f,
                "unit `{unit}` defers to `{target}`, which no unit declares; a deferral names \
                 the procedural unit whose body completes this one"
            ),
            Fault::DeferralToTriggering {
                unit,
                target,
                target_file,
                target_line,
            } => write!(
                f,
                "unit `{unit}` defers to `{target}` ({target_file}:{target_line}), which is \
                 `class=triggering`; a deferral must name a `class=procedural` unit, or the \
                 session is handed a condition where a procedure was promised"
            ),
            Fault::UnreachableProcedures { ids } => write!(
                f,
                "no chain of `defers=` from any triggering unit reaches {}; a procedural unit \
                 no mandate can reach is deleted from the methodology as surely as prose no \
                 parser can see. Name it in some unit's `defers=`, or delete it.",
                ids.iter()
                    .map(|id| format!("`{id}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

/// A [`Fault`] located in a file, rendered exactly as a per-file failure is:
/// `file:line:offset`. A contributor opens a build error the same way whichever
/// half of the gate produced it, which is why [`Unit`] carries the offset a
/// whole-embed check would otherwise have no way to recover.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbedError {
    pub file: String,
    pub line: usize,
    pub offset: usize,
    pub fault: Fault,
}

impl fmt::Display for EmbedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {}",
            self.file, self.line, self.offset, self.fault
        )
    }
}

impl std::error::Error for EmbedError {}

/// Check the assembled unit set, reporting the first malformation.
///
/// The checks run in dependency order, and the order is not cosmetic: `defers=`
/// resolution is meaningless while two units share an id, and reachability is
/// meaningless while a deferral may not resolve. Reporting the *first* failure
/// therefore reports the one a contributor can act on.
///
/// **Determinism comes from the caller.** Both callers assemble `units` by
/// sorted `content/`-relative path, units in file order — `build.rs` from the
/// filesystem walk, [`super::units`] from the embed — so the reported failure is
/// the same one on every machine.
pub fn check(units: &[Unit]) -> Result<(), EmbedError> {
    let declared = declared_ids(units)?;
    check_deferrals(units, &declared)?;
    check_reachability(units, &declared)
}

fn error_at(unit: &Unit, fault: Fault) -> EmbedError {
    EmbedError {
        file: unit.file.clone(),
        line: unit.line,
        offset: unit.offset,
        fault,
    }
}

/// Index the units by id, rejecting the first collision.
///
/// Ordered rather than hashed, so the *set* iterates deterministically wherever
/// a later check walks it — reachability's residue is one such walk, and a
/// diagnostic listing ids in hash order is a diagnostic two contributors quote
/// differently.
fn declared_ids(units: &[Unit]) -> Result<BTreeMap<&str, &Unit>, EmbedError> {
    let mut declared: BTreeMap<&str, &Unit> = BTreeMap::new();
    for unit in units {
        if let Some(first) = declared.get(unit.id.as_str()) {
            return Err(error_at(
                unit,
                Fault::DuplicateId {
                    id: unit.id.clone(),
                    first_file: first.file.clone(),
                    first_line: first.line,
                },
            ));
        }
        declared.insert(unit.id.as_str(), unit);
    }
    Ok(declared)
}

/// Every `defers=` member names a declared unit, and that unit is procedural.
fn check_deferrals(units: &[Unit], declared: &BTreeMap<&str, &Unit>) -> Result<(), EmbedError> {
    for unit in units {
        for target in &unit.defers {
            let Some(found) = declared.get(target.as_str()) else {
                return Err(error_at(
                    unit,
                    Fault::UnknownDeferral {
                        unit: unit.id.clone(),
                        target: target.clone(),
                    },
                ));
            };
            if found.class != Class::Procedural {
                return Err(error_at(
                    unit,
                    Fault::DeferralToTriggering {
                        unit: unit.id.clone(),
                        target: target.clone(),
                        target_file: found.file.clone(),
                        target_line: found.line,
                    },
                ));
            }
        }
    }
    Ok(())
}

/// Every procedural unit is reached by following `defers=` from some triggering
/// unit.
///
/// The roots are *all* triggering units, and that is the same claim as
/// "reachable from some kind's mandate": a triggering unit's scope is `*` or a
/// non-empty kind list, both of which the parser enforces, so every triggering
/// unit reaches at least one kind's mandate. Reachability is per kind in what it
/// *means* — a procedure reached only from a `kinds=impl` condition is
/// discoverable from the `impl` mandate and no other — but the union is what the
/// build has to decide, and the union does not need the kinds enumerated.
///
/// Cycles need no rule of their own: a ring of procedural units deferring only
/// to each other is entered by no root, so it fails here as a group.
fn check_reachability(units: &[Unit], declared: &BTreeMap<&str, &Unit>) -> Result<(), EmbedError> {
    let mut reached: BTreeSet<&str> = BTreeSet::new();
    let mut pending: Vec<&str> = units
        .iter()
        .filter(|unit| unit.class == Class::Triggering)
        .flat_map(|unit| unit.defers.iter().map(String::as_str))
        .collect();
    while let Some(id) = pending.pop() {
        if !reached.insert(id) {
            continue;
        }
        // `check_deferrals` ran first, so every id here is declared; a unit that
        // deferred to nothing simply contributes nothing.
        if let Some(unit) = declared.get(id) {
            pending.extend(unit.defers.iter().map(String::as_str));
        }
    }

    let mut unreachable = units
        .iter()
        .filter(|unit| unit.class == Class::Procedural && !reached.contains(unit.id.as_str()));
    let Some(first) = unreachable.next() else {
        return Ok(());
    };
    let ids = std::iter::once(first.id.clone())
        .chain(unreachable.map(|unit| unit.id.clone()))
        .collect();
    Err(error_at(first, Fault::UnreachableProcedures { ids }))
}

#[cfg(test)]
mod tests {
    use super::super::parse::parse_units;
    use super::*;

    /// Assemble a synthetic embed the way both real callers do: parse each file
    /// through the shared reader, in sorted path order, and concatenate.
    ///
    /// Going through `parse_units` rather than hand-building [`Unit`] values is
    /// deliberate — a fixture that skipped the reader could assert a shape the
    /// grammar cannot actually produce.
    fn embed(files: &[(&str, &str)]) -> Vec<Unit> {
        let mut sorted = files.to_vec();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        sorted
            .iter()
            .flat_map(|(name, text)| parse_units(name, text).expect("fixture must parse"))
            .collect()
    }

    fn fault(files: &[(&str, &str)]) -> Fault {
        check(&embed(files))
            .expect_err("the embed must be rejected")
            .fault
    }

    /// **The positive control**, and it is the same shape as every rejection
    /// below: two files, a deferral crossing between them, and a chain. A
    /// well-formed pattern matching nothing reads exactly like a clean
    /// repository, so the accepting case is asserted beside the refusing ones.
    #[test]
    fn a_well_formed_embed_with_a_cross_file_chain_is_accepted() {
        let units = embed(&[
            (
                "A.md",
                "<!-- unit: alpha kinds=* class=triggering defers=beta -->\ncondition\n\
                 <!-- unit: gamma class=procedural -->\nthe end of the chain\n",
            ),
            (
                "B.md",
                "<!-- unit: beta class=procedural defers=gamma -->\nprocedure\n",
            ),
        ]);

        assert_eq!(
            check(&units),
            Ok(()),
            "a deferral resolves across files, and a chain may run on from there"
        );
    }

    /// **The cross-file case.** Neither file's parse could have caught this: each
    /// declares one well-formed unit, and only the assembled set knows the id is
    /// taken.
    #[test]
    fn a_duplicate_id_across_two_files_is_rejected_naming_both() {
        assert_eq!(
            fault(&[
                (
                    "A.md",
                    "<!-- unit: alpha kinds=* class=triggering -->\none\n"
                ),
                (
                    "B.md",
                    "<!-- unit: alpha kinds=* class=triggering -->\ntwo\n"
                ),
            ]),
            Fault::DuplicateId {
                id: "alpha".to_string(),
                first_file: "A.md".to_string(),
                first_line: 1,
            },
            "the second occurrence is the error and the first is named with it"
        );
    }

    #[test]
    fn a_duplicate_id_within_one_file_is_rejected_too() {
        assert!(matches!(
            fault(&[(
                "A.md",
                "<!-- unit: alpha kinds=* class=triggering -->\none\n\
                 <!-- unit: alpha kinds=* class=triggering -->\ntwo\n",
            )]),
            Fault::DuplicateId { .. }
        ));
    }

    #[test]
    fn a_deferral_naming_no_declared_unit_is_rejected() {
        assert_eq!(
            fault(&[(
                "A.md",
                "<!-- unit: alpha kinds=* class=triggering defers=missing -->\nbody\n",
            )]),
            Fault::UnknownDeferral {
                unit: "alpha".to_string(),
                target: "missing".to_string(),
            }
        );
    }

    /// The category error: the target exists, so nothing dangles, but following
    /// it would hand the session a condition it has already read.
    #[test]
    fn a_deferral_naming_a_triggering_unit_is_rejected() {
        assert_eq!(
            fault(&[
                (
                    "A.md",
                    "<!-- unit: alpha kinds=* class=triggering defers=beta -->\nbody\n",
                ),
                (
                    "B.md",
                    "<!-- unit: beta kinds=* class=triggering -->\nbody\n"
                ),
            ]),
            Fault::DeferralToTriggering {
                unit: "alpha".to_string(),
                target: "beta".to_string(),
                target_file: "B.md".to_string(),
                target_line: 1,
            }
        );
    }

    #[test]
    fn a_procedural_unit_no_deferral_names_is_rejected() {
        assert_eq!(
            fault(&[
                (
                    "A.md",
                    "<!-- unit: alpha kinds=* class=triggering -->\nbody\n"
                ),
                ("B.md", "<!-- unit: orphan class=procedural -->\nbody\n"),
            ]),
            Fault::UnreachableProcedures {
                ids: vec!["orphan".to_string()],
            }
        );
    }

    /// Cycles are disposed of without a rule about them, which is the whole
    /// reason reachability is stated as reachability rather than as an acyclic
    /// check: a ring is entered by no root.
    #[test]
    fn a_ring_of_procedural_units_reachable_from_nothing_is_rejected() {
        assert_eq!(
            fault(&[
                (
                    "A.md",
                    "<!-- unit: alpha kinds=* class=triggering -->\nbody\n"
                ),
                (
                    "B.md",
                    "<!-- unit: ring-one class=procedural defers=ring-two -->\nbody\n\
                     <!-- unit: ring-two class=procedural defers=ring-one -->\nbody\n",
                ),
            ]),
            Fault::UnreachableProcedures {
                ids: vec!["ring-one".to_string(), "ring-two".to_string()],
            },
            "every unreachable id is reported, in embed order"
        );
    }

    /// A procedure named only by an unreachable one is itself unreachable — the
    /// spec's second half of that scenario, and the reason the walk starts from
    /// triggering roots rather than from "anything that is deferred to".
    #[test]
    fn a_procedure_reached_only_from_an_unreachable_one_is_rejected() {
        assert_eq!(
            fault(&[(
                "A.md",
                "<!-- unit: alpha kinds=* class=triggering -->\nbody\n\
                 <!-- unit: detached class=procedural defers=downstream -->\nbody\n\
                 <!-- unit: downstream class=procedural -->\nbody\n",
            )]),
            Fault::UnreachableProcedures {
                ids: vec!["detached".to_string(), "downstream".to_string()],
            }
        );
    }

    /// A triggering unit reachable from nothing is not an error. It is a
    /// *mandate root*: it ships because its scope says so, and requiring
    /// something to defer to it would invert the design.
    #[test]
    fn a_triggering_unit_needs_no_referrer() {
        let units = embed(&[(
            "A.md",
            "<!-- unit: alpha kinds=* class=triggering -->\none\n\
             <!-- unit: beta kinds=impl class=triggering -->\ntwo\n",
        )]);
        assert_eq!(check(&units), Ok(()));
    }

    /// A narrowly scoped condition still roots a chain. Reachability is per kind
    /// in meaning, and the build decides the union — so a procedure only the
    /// `finish` mandate can discover is reachable, not orphaned.
    #[test]
    fn a_narrowly_scoped_condition_still_reaches_its_procedure() {
        let units = embed(&[(
            "A.md",
            "<!-- unit: alpha kinds=finish class=triggering defers=teardown -->\nbody\n\
             <!-- unit: teardown class=procedural -->\nbody\n",
        )]);
        assert_eq!(check(&units), Ok(()));
    }

    #[test]
    fn an_error_names_the_file_the_line_and_the_offset() {
        let rendered = check(&embed(&[(
            "SKILL.md",
            "<!-- unit: alpha kinds=* class=triggering -->\nbody\n\
             <!-- unit: beta kinds=* class=triggering defers=missing -->\nbody\n",
        )]))
        .expect_err("the fixture must be rejected")
        .to_string();

        assert!(
            rendered.starts_with("SKILL.md:3:51: "),
            "a build error must be openable at the referring unit: {rendered}"
        );
    }
}
