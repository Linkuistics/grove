# plan-k2

**Reviews:** plan-k1

## Goal

An adversarial read of the root brief and `plan-k1`'s decision log, before any
of it is built on. Findings only; fix nothing.

## Context

Eighteen decisions, four of them reversed mid-interview, settled in one sitting
and executed thereafter by a long chain of sessions with no human present. The
reversals were the interview working rather than a fault — but they mean the
final set was never read whole by anyone who had not just argued their way to
it.

## Done when

Every decision has been read against the repository it claims about, and the
findings are recorded. Cut an integration only if there is something worth
acting on.

## Notes

Read for these in particular.

- **Decisions that reversed.** 1 (depth), 9 (frozen corpus), 13 (loop in
  grove), and the scope widening in 4. Each reversal cascaded, and a
  consequence may have been missed where it was recorded rather than traced.
- **Whether the frozen corpus survives contact.** Decision 9 forbids inline
  fixes across a campaign of many sessions over 14,525 lines. Ask what happens
  the first time a book cannot be written truthfully without a code change.
- **Whether the architecture move is safe as stated.** Decision 5 moves three
  sections out of a document whose anchors are cited from Rust source and
  asserted at compile time. The claim is that red tests make this mechanical.
  Check that claim rather than accepting it.
- **Whether the set is right.** Decision 6 folds `crates/grove` into the
  overview *and* decision 1 requires complete source coverage — so the overview
  owns production source as fragments. Ask whether that is coherent.
- **Whether four products belong in one grove at all**, and whether the
  ordering in the brief's *Decomposition* actually keeps the documentation
  unblocked, given `pick` is a walk and not a scheduler.
- **What the brief assumes and never states.** The interview settled what was
  asked; a fresh reader is the only one who can see what was not.

## Findings

### F1 · High — “Every crate” and the accepted set disagree

The root goal requires complete source-exact walkthroughs “for every crate”
(`.grove/BRIEF.md:5-9`), while decision 6 explicitly excludes
`book-validation` (`.grove/01-DONE-requirements--plan-k1.md:66-70`). That is a
real workspace crate (`Cargo.toml:39-48`), not an external tool. The acceptance
list then names the other runtime crates and never states a narrower meaning of
“every” (`.grove/BRIEF.md:24-27`). A successful implementation can therefore
both satisfy the list and violate the goal. The requirements must either include
`book-validation` or qualify the goal and acceptance boundary as the Grove
runtime crates, recording why the validator is outside it.

### F2 · High — The tree order cannot deliver the promised loop-driven scale-out without blocking documentation

Decision 12 requires the pilot first, then extraction of “the kinds and the
loop”, and only then the remaining books through the shipped pipeline
(`.grove/01-DONE-requirements--plan-k1.md:98-101`). The root brief simultaneously
requires every child of `walkthroughs-k3` — whose scope includes scale-out — to
be inserted ahead of the research pair, while `loop-construct-k7` remains after
that pair (`.grove/BRIEF.md:43-57`). `pick` is a depth-first pre-order walk:
`selected` takes the first live non-finish entry in `Snapshot::walk` order
(`crates/grove-loop/src/task_tree.rs:606-636`; `crates/ordinal-fs-tree/src/snapshot.rs:599-606,663-673`). Thus either scale-out runs before the loop it must use, or it is placed after `loop-construct-k7` and waits behind the deep research pair, breaking decision 17’s “documentation is never blocked.” The requirements need one executable ordering that puts pilot → pipeline extraction → loop design → scale-out together ahead of unrelated research, or must relax one of those promises.

### F3 · High — The claimed mechanical safety net does not cover Rust anchor citations

Decision 5 says the architecture move is guarded because Rust-source and test
citations will be re-pointed by turning red tests green
(`.grove/01-DONE-requirements--plan-k1.md:58-64`; `.grove/BRIEF.md:87-90`). The
repository-wide resolver scans only `.md` files
(`crates/grove/tests/reference_navigation.rs:357-402`). The only `.rs` scan is
`every_adr_citation_names_a_decision_record`, which recognizes the special
`ADR <slug>` form and checks record kind, not raw
`docs/ARCHITECTURE.md#<anchor>` links
(`crates/grove/tests/reference_navigation.rs:487-548,551-583`). Current Rust
source contains many such raw references, including
`crates/grove-loop/src/task_tree.rs:205,326,420,484,533,850`. Moving an anchor can
therefore leave source citations dangling while the named suites stay green.
The move needs an acceptance test that enumerates and resolves architecture
anchors in both Markdown and Rust, or the brief must stop claiming the move is
mechanical.

### F4 · High — “Defect becomes a leaf” does not define a safe source-change transition

The frozen-corpus rule says a discovered defect becomes its own leaf, but says
nothing about what that leaf may commit or how already-completed books are
revalidated (`.grove/BRIEF.md:109-112`;
`.grove/01-DONE-requirements--plan-k1.md:81-85`). A code-fix leaf necessarily
changes the authoritative bytes and invalidates line ranges below the edit — the
same failure the freeze exists to prevent. The existing book contract supplies
the missing invariant: an accepted source change updates affected ownership
ranges and fragments and then runs final validation against the new bytes
(`docs/specs/ordinal-fs-tree-book.md:10-15`). The campaign requirements need the
equivalent cross-book transition: whether the fix waits until the campaign ends,
or lands together with every affected ledger/page update and a full revalidation.
Without it, the first real defect makes “fix nothing inline” and “finished pages
stay proved” mutually incompatible.

### F5 · High — P4 has no root acceptance criterion and its research question is already asserted as fact

P4 is called a product and allegedly exists because P1 cannot be reached without
it (`.grove/BRIEF.md:11-20`), yet the root `Done when` contains no specification-
capture deliverable at all (`.grove/BRIEF.md:22-39`), and the same brief says the
manual form needed by every book is already available
(`.grove/BRIEF.md:122-126`). The research can therefore return any result — or no
durable result — without changing whether the grove is done. Worse, decision 16
and the root note assert that capture and formalism are two layers and that a
single notation fails, while the commissioned research is supposed to test that
hypothesis (`.grove/01-DONE-requirements--plan-k1.md:119-124`;
`.grove/BRIEF.md:128-131`). Either make P4 an independent research workstream, or
state the decision its evidence must enable and let the two-tier form remain a
falsifiable hypothesis until then.

### F6 · Medium — “Complete user guide” is not falsifiable

The only user-guide acceptance condition is that `docs/USAGE.md` be “complete”
and retain its ownership row (`.grove/BRIEF.md:28-29`). Decision 2 settles the
file, not the missing content (`.grove/01-DONE-requirements--plan-k1.md:39-42`),
and the document already exists with start/resume/finish, task-tree, session,
review, and teardown sections (`docs/USAGE.md:14,96,148,227,318`). Nothing says
which current gap this work must close, which user journeys or observable
commands it must cover, or what check/review distinguishes the present guide
from the completed one. A no-op can satisfy the requirement by assertion. The
requirements need a bounded coverage inventory and review seam for the guide.

### F7 · Medium — The claimed eight-field walkthrough intake is incomplete at the source boundary

The brief says the `writing-code-walkthroughs` intake is fully answered and must
not be re-elicited (`.grove/BRIEF.md:61-63`), but the method requires an exact
manifest of authoritative production files plus an explicit classification of
tests, fixtures, models, generated files, examples and dependencies
(`plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md:24-54,56-61`).
The decision log supplies only component names and aggregate counts. Those
counts reconcile to 33 roots / 14,525 lines only by including `crates/grove` in
the overview corpus and excluding the 1,680-line
`crates/grove-loop/src/task_grow/tests.rs`; neither boundary is recorded. A
future author can choose a different plausible corpus and still claim the same
decision was followed. Record the exact per-deliverable source/evidence/excluded
inventory or explicitly assign its production as unfinished requirements work.

### F8 · Medium — The pilot cannot falsify or derive the pipeline under the stated measure

The pipeline must be “derived from what the pilot measured”
(`.grove/BRIEF.md:36-37`), and the brief correctly observes that `book-check`
cannot tell whether six editorial stages beat two
(`.grove/BRIEF.md:114-120`). But “record what each stage changed” is activity
provenance, not a quality measure: there is no judged outcome, baseline,
acceptance threshold, attribution rule, or decision rule for retaining,
combining, or dropping a stage. Consequently any six-stage pipeline can be
described after the fact as derived from the run. The requirements must make
“paid for itself” falsifiable before the pilot — using the cited judging
protocol, an explicit alternative, and a rule mapping evidence to the extracted
kinds.

### F9 · Low — The review premise miscounts its own decision set

The root brief says this review covers seventeen decisions
(`.grove/BRIEF.md:45-48`), while the producer log contains decisions 1 through
18, with decision 18 being the decision to commission this review
(`.grove/01-DONE-requirements--plan-k1.md:130-133`). The review task itself uses
the correct count. This does not alter scope, but it is exactly the sort of stale
summary-layer residue the fresh read was commissioned to catch.
