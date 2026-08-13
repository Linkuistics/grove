//! The half of the build gate that no single file can decide — **one
//! implementation, compiled twice**, exactly like the reader beside it.
//!
//! [`super::parse`] answers everything a single `(path, text)` settles. Five
//! malformations are left over, and all five need the *assembled* unit set:
//!
//! * **Two files claiming one mandate position.** The parser makes every file
//!   *carry* a position; only the set can say the positions differ, and a
//!   composition order that is not total is the one property the directive
//!   exists to supply.
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
//! * **A chain of deferrals that returns to a unit it already passed through.**
//!   Reachability does *not* subsume this, which is the correction the check
//!   below records: a ring nothing enters is unreachable, but a ring a trigger
//!   enters is reached like any other chain, and a session walking it out of
//!   its mandate never arrives anywhere.
//!
//! All five are a **contributor's** mistake, visible to the build that produced
//! the embed, so all five fail `cargo build`
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
    /// Two files claiming one position, named as a pair for the same reason a
    /// duplicate id is: the repair is to renumber one of them, and a contributor
    /// cannot choose which without seeing both.
    DuplicateFileOrder {
        order: u32,
        first_file: String,
    },
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
    /// The ring, in the order the walk entered it: `ids[0]` is the unit the
    /// closing deferral names, and the last is the unit that holds that
    /// deferral — which is where the error is located, so the two are never
    /// carried separately and cannot disagree. One id is a self-deferral, and
    /// a ring of one is still a ring.
    ProceduralCycle {
        ids: Vec<String>,
    },
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::DuplicateFileOrder { order, first_file } => write!(
                f,
                "mandate position {order} is already claimed by {first_file}; the composition \
                 order must be total, so no two embedded files share one. Renumber the \
                 `<!-- file: order=<n> -->` at the top of this file's body — the coordinate \
                 above is its first unit, since a position belongs to a file rather than to \
                 any unit in it."
            ),
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
            // Rendered with the entry id repeated at the end, so the ring reads
            // as the walk a session would actually perform rather than as a set
            // it has to close itself.
            Fault::ProceduralCycle { ids } => write!(
                f,
                "the deferral chain {} returns to a unit it has already passed through; a \
                 session following `defers=` out of a mandate is sent round it forever, because \
                 no unit on the ring ends the chain. Reachability cannot catch this one — a \
                 triggering unit enters the ring — so one unit on it must stop deferring back \
                 into it.",
                ids.iter()
                    .chain(ids.first())
                    .map(|id| format!("`{id}`"))
                    .collect::<Vec<_>>()
                    .join(" -> ")
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
/// The last four run in **dependency order**, and the order is not cosmetic:
/// `defers=` resolution is meaningless while two units share an id, reachability
/// is meaningless while a deferral may not resolve, and a ring nothing enters is
/// reported as the orphan it also is. Reporting the *first* failure therefore
/// reports the one a contributor can act on.
///
/// [`check_file_order`] shares no dependency with that chain — it is a claim
/// about *files* rather than about the unit graph — so its position is free, and
/// it is first because it is the one a contributor can repair without reading
/// anything else.
///
/// **Determinism comes from the caller.** Both callers assemble `units` by
/// sorted `content/`-relative path, units in file order — `build.rs` from the
/// filesystem walk, [`super::units`] from the embed — so the reported failure is
/// the same one on every machine.
pub fn check(units: &[Unit]) -> Result<(), EmbedError> {
    check_file_order(units)?;
    let declared = declared_ids(units)?;
    check_deferrals(units, &declared)?;
    check_reachability(units, &declared)?;
    check_acyclic(units, &declared)
}

fn error_at(unit: &Unit, fault: Fault) -> EmbedError {
    EmbedError {
        file: unit.file.clone(),
        line: unit.line,
        offset: unit.offset,
        fault,
    }
}

/// No two files claim one mandate position.
///
/// The units of one file all carry that file's position, so the claim is read
/// off the **first unit of each file** and a file is only ever compared with
/// other files. That the position is *present* is the parser's business, decided
/// per file; that positions *differ* needs the set, and is this one.
///
/// It settles totality rather than density: gaps are legal. A position is an
/// ordering key, and requiring contiguity would make inserting a file renumber
/// every later one for no gain — the composer sorts, it does not index.
fn check_file_order(units: &[Unit]) -> Result<(), EmbedError> {
    let mut claimed: BTreeMap<u32, &str> = BTreeMap::new();
    for unit in units {
        // Every unit of a file repeats its position, so only the first one of
        // each file is a fresh claim; the rest would collide with themselves.
        if claimed.get(&unit.file_order) == Some(&unit.file.as_str()) {
            continue;
        }
        if let Some(first_file) = claimed.get(&unit.file_order) {
            return Err(error_at(
                unit,
                Fault::DuplicateFileOrder {
                    order: unit.file_order,
                    first_file: (*first_file).to_string(),
                },
            ));
        }
        claimed.insert(unit.file_order, unit.file.as_str());
    }
    Ok(())
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
/// This disposes of a ring **no root enters** — it is unreachable, so it fails
/// here as a group — and of nothing else. A ring a triggering unit *does* enter
/// is reached like any other chain, and the second arrival is discarded by the
/// very guard that makes this walk terminate; [`check_acyclic`] is what sees
/// that one.
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

/// No chain of `defers=` returns to a unit it has already passed through.
///
/// Reachability asks *can a mandate get here*; this asks *can a session that
/// follows the deferrals stop*. Two questions, two kinds of bookkeeping — one
/// global visited set, order-insensitive, against a per-path stack popped on
/// the way out — which is why the walk above cannot answer this one. Its
/// `reached` set doubles as the guard that makes it terminate, so a second
/// arrival at an already-reached unit is discarded rather than recognised as a
/// return.
///
/// It runs **after** reachability, and the order carries meaning rather than
/// convenience. An unrooted ring is unreachable *and* cyclic, and its first
/// fault is that nothing enters it — that is the repair a contributor makes,
/// and the ring is dead prose until they make it. So every cycle reported here
/// is a **rooted** one by construction, which is exactly the class reachability
/// lets through.
///
/// Only procedural units can lie on a ring: [`check_deferrals`] ran first, so
/// every `defers=` target is procedural and no edge points at a triggering
/// unit. They are therefore the only roots worth walking from.
fn check_acyclic<'u>(
    units: &'u [Unit],
    declared: &BTreeMap<&'u str, &'u Unit>,
) -> Result<(), EmbedError> {
    let mut settled: BTreeSet<&str> = BTreeSet::new();
    let mut path: Vec<&str> = Vec::new();
    for unit in units.iter().filter(|unit| unit.class == Class::Procedural) {
        if settled.contains(unit.id.as_str()) {
            continue;
        }
        walk(unit, declared, &mut settled, &mut path)?;
    }
    Ok(())
}

/// Depth-first from one procedural unit, reporting the first deferral naming a
/// unit already on the path.
///
/// What keeps a **diamond** from reading as a ring is that `path` is *popped*:
/// only a unit on the chain currently being walked counts as a return, so a
/// procedure two chains both reach is arrived at twice and is on the path
/// neither time. Being *seen before* and being *on the current path* are
/// different facts, and conflating them is the naive form of this check — the
/// one that would reuse [`check_reachability`]'s `reached` set and reject every
/// diamond. `settled` is the memo that keeps the walk linear rather than
/// re-descending a shared tail once per chain into it; it decides nothing.
/// `path` doubles as the diagnostic, so naming the ring costs no second
/// traversal.
fn walk<'u>(
    unit: &'u Unit,
    declared: &BTreeMap<&'u str, &'u Unit>,
    settled: &mut BTreeSet<&'u str>,
    path: &mut Vec<&'u str>,
) -> Result<(), EmbedError> {
    path.push(unit.id.as_str());
    for target in &unit.defers {
        let target = target.as_str();
        if let Some(entry) = path.iter().position(|seen| *seen == target) {
            // Located at `unit`, whose marker holds the deferral that closes
            // the ring — the line a contributor edits to break it.
            return Err(error_at(
                unit,
                Fault::ProceduralCycle {
                    ids: path[entry..].iter().map(|id| (*id).to_string()).collect(),
                },
            ));
        }
        if settled.contains(target) {
            continue;
        }
        // `check_deferrals` ran first, so every target here is declared.
        if let Some(&next) = declared.get(target) {
            walk(next, declared, settled, path)?;
        }
    }
    path.pop();
    settled.insert(unit.id.as_str());
    Ok(())
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
    ///
    /// Each file is given its position from its place in that sorted order, so
    /// every fixture below is well-ordered by construction and each test
    /// provokes only the fault it names. The one test that is *about* ordering
    /// writes its own directives instead.
    fn embed(files: &[(&str, &str)]) -> Vec<Unit> {
        let mut sorted = files.to_vec();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        sorted
            .iter()
            .enumerate()
            .flat_map(|(index, (name, text))| {
                let ordered = format!("<!-- file: order={} -->\n{text}", index + 1);
                parse_units(name, &ordered).expect("fixture must parse")
            })
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

    /// **The ordering key's cross-file half.** The parser makes every file carry
    /// a position and can say nothing about the others; two files claiming one
    /// is only visible here, and it costs the composition order the totality it
    /// exists to supply.
    ///
    /// Written with explicit directives rather than through [`embed`], whose
    /// whole job is to number fixtures apart.
    #[test]
    fn two_files_claiming_one_mandate_position_are_rejected() {
        let ordered = |name: &'static str, order: u32, id: &str| {
            parse_units(
                name,
                &format!(
                    "<!-- file: order={order} -->\n\
                     <!-- unit: {id} kinds=* class=triggering -->\nbody\n"
                ),
            )
            .expect("fixture must parse")
        };

        let mut units = ordered("A.md", 2, "alpha");
        units.extend(ordered("B.md", 2, "beta"));
        assert_eq!(
            check(&units).expect_err("the embed must be rejected").fault,
            Fault::DuplicateFileOrder {
                order: 2,
                first_file: "A.md".to_string(),
            },
            "the second claimant is the error and the first is named with it"
        );

        let mut apart = ordered("A.md", 2, "alpha");
        apart.extend(ordered("B.md", 7, "beta"));
        assert_eq!(
            check(&apart),
            Ok(()),
            "gaps are legal — the composer sorts by the key, it does not index by it"
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
                first_line: 2,
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
                target_line: 2,
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

    /// An **unrooted** ring, and reachability alone is enough for it: nothing
    /// enters it, so it fails as a group before [`check_acyclic`] is consulted.
    /// That ordering is deliberate — its first fault is that no mandate can get
    /// to it, and the ring only becomes a live defect once one can. What this
    /// fixture does *not* prove is that reachability disposes of cycles
    /// generally; the two tests below are the ones it does not cover.
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

    /// **The rooted ring**, which reachability cannot see: both procedures are
    /// entered from a trigger, so both are reached, and the revisit that closes
    /// the ring is discarded as "already reached". A session walking `defers=`
    /// out of the mandate is sent round it forever.
    #[test]
    fn a_rooted_cycle_of_procedural_units_is_rejected() {
        assert_eq!(
            fault(&[
                (
                    "A.md",
                    "<!-- unit: alpha kinds=* class=triggering defers=first -->\ncondition\n",
                ),
                (
                    "B.md",
                    "<!-- unit: first class=procedural defers=second -->\nbody\n\
                     <!-- unit: second class=procedural defers=first -->\nbody\n",
                ),
            ]),
            Fault::ProceduralCycle {
                ids: vec!["first".to_string(), "second".to_string()],
            },
            "the ring is named in the order a session would walk it"
        );
    }

    /// The one-unit ring. It needs no second unit to be a ring, and a check
    /// written as "a chain must not revisit" gets it for free — where a check
    /// written as "two units must not defer to each other" would not.
    #[test]
    fn a_rooted_self_cycle_is_rejected() {
        assert_eq!(
            fault(&[(
                "A.md",
                "<!-- unit: alpha kinds=* class=triggering defers=loop-back -->\ncondition\n\
                 <!-- unit: loop-back class=procedural defers=loop-back -->\nbody\n",
            )]),
            Fault::ProceduralCycle {
                ids: vec!["loop-back".to_string()],
            }
        );
    }

    /// The error is located at the marker a contributor edits to break the
    /// ring — the unit holding the deferral that closes it, not the one it
    /// closes onto.
    #[test]
    fn a_cycle_is_reported_at_the_deferral_that_closes_it() {
        let rendered = check(&embed(&[
            (
                "A.md",
                "<!-- unit: alpha kinds=* class=triggering defers=first -->\ncondition\n",
            ),
            (
                "B.md",
                "<!-- unit: first class=procedural defers=second -->\nbody\n\
                 <!-- unit: second class=procedural defers=first -->\nbody\n",
            ),
        ]))
        .expect_err("the fixture must be rejected")
        .to_string();

        assert!(
            rendered.starts_with("B.md:4:"),
            "the closing deferral is `second`'s, on B.md line 4 past the directive: {rendered}"
        );
        assert!(
            rendered.contains("`first` -> `second` -> `first`"),
            "the ring is spelled out in walk order: {rendered}"
        );
    }

    /// **The false positive to avoid.** Two chains meeting at one procedure is a
    /// diamond, not a ring: `join` is visited twice, but never twice on one
    /// path. A cycle check that memoised nothing but "seen" would reject this.
    #[test]
    fn a_procedure_two_chains_both_reach_is_not_a_cycle() {
        let units = embed(&[(
            "A.md",
            "<!-- unit: alpha kinds=* class=triggering defers=\"left right\" -->\ncondition\n\
             <!-- unit: left class=procedural defers=join -->\nbody\n\
             <!-- unit: right class=procedural defers=join -->\nbody\n\
             <!-- unit: join class=procedural -->\nbody\n",
        )]);

        assert_eq!(check(&units), Ok(()));
    }

    /// **The claim checked against an independent method rather than asserted.**
    /// The fixtures either side of this one are shapes a contributor is likely
    /// to author; this is every shape there is. Over all 512 digraphs on three
    /// procedural units — self-loops included, and every unit rooted so
    /// reachability always passes — the walk's verdict must agree with a
    /// transitive closure computed the other way round, by saturation rather
    /// than by descent.
    ///
    /// It earns its place because the rule it guards shipped once already as a
    /// claim about graph behaviour that plausible fixtures did not actually
    /// cover. A hand-written suite proves the shapes its author thought of,
    /// which is exactly the thing in question.
    #[test]
    fn the_cycle_check_agrees_with_transitive_closure_on_every_three_unit_graph() {
        const NODES: [&str; 3] = ["a", "b", "c"];

        for edges in 0u16..512 {
            let has = |from: usize, to: usize| edges & (1u16 << (from * 3 + to)) != 0;

            // `closure[i][j]` saturates to "a chain of one or more deferrals
            // runs from i to j", so a cycle is any unit reaching itself.
            let mut closure = [[false; 3]; 3];
            for (from, row) in closure.iter_mut().enumerate() {
                for (to, cell) in row.iter_mut().enumerate() {
                    *cell = has(from, to);
                }
            }
            for through in 0..3 {
                for from in 0..3 {
                    for to in 0..3 {
                        if closure[from][through] && closure[through][to] {
                            closure[from][to] = true;
                        }
                    }
                }
            }
            let cyclic = (0..3).any(|node| closure[node][node]);

            let mut text =
                String::from("<!-- unit: root kinds=* class=triggering defers=\"a b c\" -->\nc\n");
            for (from, id) in NODES.iter().enumerate() {
                let targets: Vec<&str> = (0..3)
                    .filter(|to| has(from, *to))
                    .map(|to| NODES[to])
                    .collect();
                let defers = match targets.len() {
                    0 => String::new(),
                    1 => format!(" defers={}", targets[0]),
                    _ => format!(" defers=\"{}\"", targets.join(" ")),
                };
                text.push_str(&format!(
                    "<!-- unit: {id} class=procedural{defers} -->\nbody\n"
                ));
            }

            let verdict = check(&embed(&[("A.md", text.as_str())]));
            assert_eq!(
                verdict.is_err(),
                cyclic,
                "graph {edges:#011b} is {}cyclic by closure, and the walk disagreed: {verdict:?}",
                if cyclic { "" } else { "a" }
            );
            if let Err(error) = verdict {
                assert!(
                    matches!(error.fault, Fault::ProceduralCycle { .. }),
                    "every unit here is rooted, so a cycle is the only fault available: {error:?}"
                );
            }
        }
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
            rendered.starts_with("SKILL.md:4:74: "),
            "a build error must be openable at the referring unit: {rendered}"
        );
    }
}
