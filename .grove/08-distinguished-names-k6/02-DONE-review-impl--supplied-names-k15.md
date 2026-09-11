# supplied-names-k15

**Reviews:** supplied-names-k14

## Goal

Adversarially review the supplied distinguished-name API boundary before the
following enforcement child builds on it.

## Context

The producer commit is identified by the `supplied-names` child handle above.
Read its diff, the parent brief, `docs/ordinal-fs-tree/ARCHITECTURE.md`, the
entry-name seam ADR and formalism finding 049. This is a scoped API boundary,
not completion of the parent contract: reader/planner level enforcement and
expected-level conformance samples are explicitly assigned to the next child.

## Done when

- Check that caller-supplied initialization/promotion names determine the actual
  destination, preserve bytes, stay confined to one component, and reject
  positioned destinations before effects without weakening guard or rollback
  behavior. Inspect every adapted caller, including Grove and coarse-parts
  equality fixtures, rather than trusting successful compilation alone.
- Check canonical-rendered distinguished identity and retained positioned
  view/species identity, with meaningful regressions and explicit conformance
  samples. Distinct distinguished values are lawful; at-most-one at a level is
  the remaining enforcement child's contract, not a singleton name law.
- Check that source and books describe this boundary consistently. In
  particular, reject a claim of already-enforced level validation: the trait
  method exists, but its reader/planner invocation is the next increment.
- Classify actionable findings with precise source evidence and expected
  behavior. If findings require integration, insert that leaf before the
  enforcement sibling so it consumes a reviewed API.

## Notes

No in-session reviewer was used by the producer. Review only; do not implement
fixes or treat the explicitly queued enforcement work as an unplanned omission.
The installed driver, plugin and live-tree grammar remain untouched.

## Findings

Reviewed: commit `kyymvozo` (`9d09fb67`), *supplied-names-k14: accept
caller-supplied distinguished names*, read against the working tree at that
commit (the working copy was empty on top of it). The artifact is the
`ordinal-fs-tree` API boundary and its callers, tests and two books;
coordinates are `path:line` in that tree. Inspection only: no build, test,
check or model runner was executed, and the recorded evidence in the producer
leaf was read, not reproduced. Ordered by severity.

### F1 — The grove-loop book still teaches the removed `TaskName::distinguished` in four places (actionable, medium)

`docs/walkthroughs/grove-loop/02-the-tokens.md:244-246` keeps a glossary bullet
naming `TaskName::distinguished` as "the associated function by which the domain
advertises the name of a node's distinguished child … chapter 4 owns its
definition", in a chapter this commit edited. `07-the-walk.md:657-659` says
"chapter 4 read `TaskName::distinguished`, the associated function through which
grove hands that name to the library". `08-kind-and-briefs.md:216-218` says the
identification is "made in chapter 2 where `TaskName::distinguished` advertises
the name". The symbol ledger `source-index.md:685` carries a row for
`TaskName::distinguished`, status `explained`, pointing at a chapter-2 anchor
that no longer explains it. The function was removed at
`crates/grove-loop/src/task_name.rs` (the diff drops `fn distinguished()`), and
grove now passes `TaskName::Brief` at `tree_lifecycle.rs:390` and `:578`.

`scripts/check.sh` stayed green because the validator compares fragments and
ledger ranges by bytes, never a prose claim about a symbol. Consequence: the
book contradicts the source it reconstructs, and the producer's own Done when
("changed source roots land with book fragments, ledgers, indexes and prose")
is not met for this root. Correction: rewrite the three prose sites to say the
name is a caller-supplied value at the two lifecycle call sites, and drop or
retarget the ledger row.

### F2 — No changelog entry for a public library API break (actionable, medium)

`CHANGELOG.md:52` `## Unreleased` is empty after this commit removed
`EntryName::distinguished`, added `validate_distinguished`, changed the
signatures of `Vacancy::initialize` and `WriteGuard::promote`, renamed
`Refusal::NoDistinguishedChild` to `SuppliedNameNotDistinguished`, and gave
`conformance::check` a third argument. The file's rule at `:11-13` is "a
session logs its change when it makes it", and the previous change to this
same surface was logged (`CHANGELOG.md:931-944`, the `initialize` bytes and
the `NoDistinguishedChild` rename). Correction: one `ordinal-fs-tree` entry
under `## Unreleased`, naming `docs/adr/entry-name-is-the-only-seam.md` as
the file's convention asks.

### F3 — Confinement of a caller-supplied distinguished name has no regression (actionable, low–medium)

The mechanism holds by reading: `crates/ordinal-fs-tree/src/fs/apply.rs:89-104`
renders `effect.name()` for every effect, which covers the root `Create` at
`ops.rs:157-161` and the promotion's `MoveTo` carrying the supplied name at
`ops.rs:365-370`; `fs/mod.rs:465` runs that check before the root is created
at `:470`. But `tests/names_are_confined.rs` pins only a parse escape (`:118`)
and a compose escape (`:199`). Before this commit every distinguished name
came from the domain; now the caller is a third input surface, and the seam
this repo agreed on for the rendering obligation is that file's adversarial
domains ("two adversarial domains, one per boundary",
`ARCHITECTURE.md:392-394`). Correction: a domain whose supplied distinguished
value renders as `../OVERVIEW.md`, driven through `initialize` (asserting no
root directory appears) and through `promote` (asserting the listing is
unchanged), each expecting `Error::NameIsNotOneComponent`.

### F4 — The refusal's shape diverges from the model, and the enforcement child must extend it (decision to settle, low)

`Refusal` variants mirror `operations.qnt` outcomes by name — `TargetMissing`
↔ `RefusedTargetMissing`, `PromotePartsNotNode` ↔ `RefusedPromotePartsNotNode`.
The model has one `RefusedInvalidDistinguished`, "reached from promotion or
initialization when the supplied name or absence is incompatible with the
level's policy" (`docs/ordinal-fs-tree/models/operations.qnt:1211-1213`,
`:1364-1368`), and `ARCHITECTURE.md:892-896` lists wrong species, unacceptable
name for its level, and missing required content as one refusal before
effects. `plan.rs:353-357` introduces `SuppliedNameNotDistinguished` for the
species half alone. `valid-levels-k16` must add the level-policy half, which
carries the domain's own error value; whether that is a second variant or a
widening of this one is an API choice its consumers see: `CLI.md:539`,
`12-leaf-to-node.md:1597`, `13-outcomes.md`, `tree_lifecycle.rs:2190`. The
new tests also cite no model witness where their predecessors did:
`ops/tests.rs:1238-1240` replaced a comment naming
`wit_refusedNoDistinguishedChild` with "a supplied positioned destination
refuses before effects", and `tests/conformance_kit.rs:697` discharges
`inv_initializeUsesSuppliedName` (`operations.qnt:1000-1008`) without saying
so. Correction: settle the variant shape before k16 builds on it, and restore
the witness citations this crate's tests carry by convention.

### F5 — The kit can no longer say a domain with no distinguished names conforms (visible trade-off or decision, low)

`conformance.rs:533-537` reports `DistinguishedNamesAreCanonical` untested
whenever the sample slice is empty, and a domain that admits no distinguished
value has nothing to put in it. The architecture keeps such a domain
(`ARCHITECTURE.md:1175`, the `no_distinguished` instance at
`operations.qnt:1463`). Before this commit `distinguished() == None` was a
positive statement the kit could read; now "no samples" and "none admitted"
are the same bytes, so a `Contentless`-shaped domain is non-conforming forever
and a caller who forgot the argument is indistinguishable from one who has
none. k16's expected-level samples are the natural place to carry "this domain
admits none"; if that is the intent, say so at the untested message and in
`02-name-seam.md`. Otherwise it is a trade-off to record.

### F6 — Chapter 4's ownership count was not updated with its file total (actionable, low)

`docs/walkthroughs/grove-loop/04-the-name.md:86` says "the chapter owns 700 of
the file's 1,712 lines"; the ledger row `source-index.md:740` and the closing
arithmetic at `21-what-could-not-move.md:446` both say 698. The file total was
corrected in the same sentence, the chapter's own count was not.

### F7 — Two comments describe the old mechanism (trivia)

`crates/grove-loop/src/tree_lifecycle.rs:2190` says the refusal "is discharged
by the domain itself"; it is discharged by the call site passing
`TaskName::Brief`, which the assertion beneath checks. `conformance.rs:520-523`
reports a positioned sample as "not a canonical distinguished name" rather
than as the wrong species. Wording only.

### What was checked and holds

- **Supplied names determine the destination and preserve bytes.**
  `ops.rs:147-165` places the root child as `Effect::Create` with the supplied
  name and bytes; `ops.rs:365-370` moves the leaf's file to the supplied name,
  so bytes travel by rename and are never read. `tests/conformance_kit.rs:697-737`
  initializes with `INDEX.md`, promotes into `OVERVIEW.md`, asserts both files'
  bytes on disk and reads the root's distinguished name back through the
  library; a fixed destination would fail it.
- **Positioned destinations refuse before effects.** `ops.rs:155` and `:341`
  return before `Plan::of(…).guarded`; `tests/initializing_on_disk.rs:385`,
  `tests/promoting_on_disk.rs:273` and `ops/tests.rs:1240` assert the refusal
  and an unchanged tree. Guard, lock and rollback code in `fs/mod.rs` and
  `fs/apply.rs` changed only by argument; `fs/apply/tests.rs` fault-injection
  tests were adapted by argument alone.
- **Every adapted caller supplies the name explicitly.** `bin/syllabus.rs:1358`
  and `:1442`; grove's `tree_lifecycle.rs:390` and `:578`; `fixtures.rs:38`;
  and the coarse-parts domain `Blind`/`LabelOnly` at
  `tests/promoting_on_disk.rs:548-565`, which still promotes and passes the
  kit with an explicit `Blind::Overview`.
- **Distinguished identity is canonical rendering; positioned identity keeps
  view and species.** `name.rs:665-673`. The regression at
  `tests/conformance_kit.rs:688-694` returns `false` for `OVERVIEW.md` against
  `INDEX.md`, which the previous `=> true` arm would have failed; the kit's
  pairwise check at `conformance.rs:524-531` samples the same law, and
  `different_distinguished_names_conform` (`:457`) shows two distinct values
  are lawful with no singleton law imposed. At-most-one per level is left to
  k16, as the task requires.
- **No source or book claims enforced level validation.** `name.rs:460-465`
  states "declaring this policy does not invoke it"; `snapshot.rs` now says
  cardinality belongs to level validation, not representation; the
  `ordinal-fs-tree` book's prose on validation (`01-orientation.md:379`,
  `06-filesystem-interpreter.md:597`, `07-syllabus-cli.md:135`) is about the
  rendering check only. `ARCHITECTURE.md` states the reader and planner
  invocation as the parent's target contract, which both the task and the
  producer leaf scope to `valid-levels-k16`; that is not a finding.
- **Glossaries and ADR agree with the boundary.** `docs/ordinal-fs-tree/CONTEXT.md:61-67`
  and `:104-108`, `CONTEXT-MAP.md:156`, and `docs/adr/entry-name-is-the-only-seam.md`
  already state supplied names and rendered identity; no naming history was
  added anywhere.
- **Stale-token sweep.** `distinguished()`, `NoDistinguishedChild`,
  `DistinguishedNamesTheOnlyEntryOfItsSpecies`, `OneDistinguishedName`,
  `DistinguishedIsUniquePerNode`, `HAS_DISTINGUISHED` across the whole tree
  with `--hidden`: the only live hits are the book sites in F1, the
  `Container::distinguished()` accessor (a different function), and history in
  `CHANGELOG.md`, `docs/formalism-findings.md` and terminal `.grove/` leaves.
  Positive control: `SuppliedNameNotDistinguished` returns 23 lines with the
  same flags.

Observation, not a finding: `crates/book-validation/tests/support/mod.rs`
mirrors real source line counts, so every source edit re-tunes that suite. The
parent's running log (decision 3) already records it.

## Decisions (running log)

**1. Seven findings; one integration leaf.** F1, F2, F3 and F6 are actionable
against the producer's own Done when and this repo's conventions; F4 and F5
are API decisions the enforcement child would otherwise inherit unsettled,
which is the reason this review sits before it. `integrate-review-impl` is
inserted with the bare slug `supplied-names` at `valid-levels-k16`, the first
following live sibling entry in this directory; its body names this review's
handle and not the findings.

**2. Inspection only, no in-session reviewer.** Nothing was built, tested or
run; every claim above is read off the tree at `kyymvozo`, and the sweep's
positive control is stated with it.
