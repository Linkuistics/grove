# library-k6

## Goal

Decompose increment 1 — `ordinal-fs-tree` and its CLI, standing alone — into
leaves, now that the design has landed and the operation set is fixed.

## Context

`architecture-k2` closed with the operation set settled and both models green.
The root brief's horizon item *"Implementing the operations, each modelled
first. Cannot be leafed until the design lands"* is therefore unblocked, and
this is the leaf that discharges it. `operations-model-k4` did not cut those
leaves itself: a `design` session cutting `impl` leaves has drifted into
planning's job.

Read first, in this order:

- `docs/ordinal-fs-tree/ARCHITECTURE.md` — the specification of record for
  everything the models do not cover, and the explanation for everything they
  do.
- `docs/ordinal-fs-tree/models/structure.als` and `operations.qnt`, with
  `run-alloy.sh` and `run-quint.sh`. **The models lead**: where a model and a
  test disagree, the model wins and the test changes. `operations.qnt`'s
  closing handoff block states what it does and does not cover, and what
  changing it obliges.
- `docs/formalism-findings.md`, entries 001–003 — in particular the misses,
  which name what the models did *not* establish and therefore what the
  implementation still has to get right unaided.

## What the decomposition has to account for

Not a decomposition — inputs to one. The root brief's constraint is that the
algebra stays free of `std::fs`, enforced by a test rather than by convention.

- **The seam and the name type** come before anything that uses them, and the
  five trait obligations are the consumer's, unchecked by the library.
- **The five mutations** — `append`, `append_many`, `insert`, `promote`,
  `rewrite` — each already have a modelled plan, a modelled refusal set and a
  two-state property. That is what "one leaf per operation" was waiting for.
- **The plan interpreter** is one leaf's worth on its own, and it is where the
  atomicity and rollback claims live. Note that `operations.qnt` shows a failed
  *rollback* is the one path by which the library damages a tree, and the
  document now states the recovery.
- **The reading operations**, the lock, and the crate skeleton.
- **The CLI's own shape** is a root-brief horizon item still, and whether it is
  in this increment's decomposition or its own is part of what this leaf
  decides.
- **H3 is untested and needs a deliberate test, not an impression.** Whether a
  checked model actually drives an implementation better than prose is the
  least certain of the three hypotheses. The decomposition is the only place
  that test can be designed in — deciding it afterwards means the evidence is
  already contaminated.

## Done when

- Increment 1's work is cut as leaves under the grove root, each a vertical
  slice that can be verified on its own.
- The root brief's horizon is reconciled: items this decomposition graduates
  are removed from it, and anything still too dim to phrase stays.
- How H3 gets tested is decided and written into whichever leaves carry it.

## Notes

The ~130 existing CLI-contract tests are regression cover for the *flip*
increment, not assurance for this one — this increment has no consumer to
constrain it, which is why the models are load-bearing. Do not plan around
adapting them here.

## Decisions (running log)

**The CLI belongs in this increment, as two leaves at the end.** The root
brief's `Done when` already puts "a CLI that drives any conforming tree" in
increment 1, and this is where that placement is confirmed rather than
reopened. The reason to keep it here rather than give it its own increment is
that it is the *only* end-to-end consumer the library gets before the flip —
increment 1 otherwise has no consumer at all, which is exactly why the models
are load-bearing. A CLI exercises the seam from outside: it forces a real
domain implementation, a real `Display`, and real error text through the same
surface a library test can fake. Deferring it to its own grove would push that
validation past a grove boundary and buy a second `root-init` for two leaves
that share every pointer with the library's. Its *shape* is genuinely
undesigned, so it gets a `design` leaf ahead of its `impl` leaf.

**Increment 1's leaves go under one node, not flat at the grove root.** They
share a large body of context — the read-first order, the models-lead rule, the
findings-log obligation, the shared test seams — that would otherwise be
repeated in nine leaf bodies or appended to the root brief, where every
increment-2 session would read it needlessly. A node is a leaf that proved
bigger than one session, which is precisely what increment 1 is;
`02-architecture-k2` set the same precedent for the design work.

**The crate lands at `crates/ordinal-fs-tree/`, as a workspace member, and the
docs stay where they are.** The root `Cargo.toml` gains a `[workspace]` table
and keeps `grove` as the root package. `docs/ordinal-fs-tree/` — glossary,
architecture, both models and their runners — does *not* move into the crate
while the crate lives in this repo: `docs/adr/` is flat and repo-wide
(`ADR-FORMAT.md`'s split rule, because grove occupies the repo root), and the
two library ADRs, `CONTEXT-MAP.md` and `docs/formalism-findings.md` all link
into `docs/ordinal-fs-tree/`. Moving it buys nothing today and breaks four
cross-links; `CONTEXT-MAP.md`'s promise that the glossary "moves with the crate"
is about extraction to its own repository, which is not this increment. The
decision fails the ADR test on its first clause — a `git mv` and one manifest
line reverse it — so it is recorded here and in `CONTEXT-MAP.md`, not in
`docs/adr/`.

**The plan interpreter ships with the `append` family, not alone.** The task
file is right that the interpreter is a leaf's worth of work, but a leaf has to
be verifiable on its own and an interpreter with no plan producer is not —
atomicity and rollback are only observable across a *multi-effect* plan, and
`append` is a single create. `append_many` is the smallest thing that makes the
interpreter's own claims checkable, and `append` is its degenerate case. So one
leaf carries the plan types, the interpreter, its rollback, its failure seam,
and both append operations. The other three mutations get a leaf each, as the
task file anticipated.

**`promote` and `rewrite` stay separate despite a shared check.** They share the
parts-imply-species test with opposite verdicts, which is a real cohesion
argument for merging, and `rewrite` alone is a thin session. They stay apart
anyway: `promote` is the only operation that breaks the invariant list
transiently and the only path by which the library can damage a tree it was
handed, and its findings-log entry should be about `promote` and nothing else.
Grove's bar is *fits this session*, not *fills it*.

**Every test cites the model claim it discharges.** A test derived from
`inv_subtreePreservedUnderShift` names that claim in a comment; a test with no
claim behind it is a test of the implementation's behaviour rather than of the
design, and says so. This is the mechanism that makes three separate things
work: the models-lead rule becomes enforceable (a failing test points at a
claim, and the claim is what wins), the H3 probe gets a measure that was not
tuned to either arm, and a later reader can tell a checked property from an
arranged one — the miss entry 003 warns about.

**Every implementation leaf appends a findings entry, including an uneventful
one.** The root brief obliges every session that reaches for a formalism to
append to `docs/formalism-findings.md`. An implementation leaf that derives its
tests from a model has reached for one. Entries may be short — one line for
*Situation* and *Formalism*, the weight on *Caught / missed*, *Cost* and
*Counterfactual* — but "the model and the code agreed, at this cost" must be
recorded, because H2 is a claim about the *ratio* of disagreements to episodes
and a log that only records drama is a survivorship sample.

**H3's probe is `insert`, pre-registered now, run as its own leaf after all five
mutations exist.** Two arms: arm A is the ordinary model-led `insert` leaf, arm
B is a fresh context given only the reconciled prose and the public signature —
no `.qnt`, no tests, no arm-A code. Both are scored by the same model-claim-
citing suite. `insert` is the probe because its ordering rule's payoff is an
*intermediate-state* property: prose can state it (and now does), but acting on
it requires the reader to notice that the order of two renames is a design
decision. `promote` is the named fallback if the first probe is degenerate. The
prediction and the falsification condition are written into the leaf body
*before* any of it runs, because deciding the measure afterwards is what
contaminates the evidence — and arm A's leaf is not told it is an arm.

**A shared reference domain, and one internal failure seam.** Every leaf's tests
and the CLI use one `EntryName` implementation — the course-syllabus domain
`ARCHITECTURE.md` already uses for its examples — so the document's examples and
the test fixtures cannot drift, and grove's vocabulary stays out of the library
by construction. The interpreter needs effects to fail on demand; that seam is
internal and must not surface in the public API, because a public second seam
would contradict `docs/adr/entry-name-is-the-only-seam.md`. If it cannot be kept
private, that is a finding and it reworks the record rather than quietly
widening it.

**The seam leaf ships a conformance kit for the five trait obligations.** They
are the consumer's and the library cannot check them, and a design missing any
one of them admits a tree the library will quietly corrupt (entry 002). An
unchecked obligation stated only in prose is exactly the artifact this
workstream exists to stop trusting, so the library exports a checker a domain
runs against its own name type. The flip needs it too: grove's domain
implementation is the second consumer, and it should not be the first thing to
discover an obligation by corrupting a task tree.

**Two claims in the root brief are stale against the architecture, and are
corrected in place.** The `Surface` line still lists "the version-control-aware
move primitive", and the `Pointers` list still says "lock scope is therefore
itself a domain decision". `ARCHITECTURE.md` settled both the other way —
a rename is `rename(2)` and the library detects no repository; locking is the
library's own rule and consumers never mention it. Grove's artifacts hold the
present and the VCS holds the past, so the lines are edited rather than
annotated. The dropped VC-awareness has a consequence increment 2 inherits, and
it is recorded there rather than left to be discovered: `src/tree_rename.rs`
dispatches on trackedness and uses `git mv` for a tracked entry, so a grove
flipped onto the library renames tracked entries with a plain
`fs::rename` under git. The two commit byte-identical trees — git infers renames
at diff time — but the operator's `git status` between the rename and the commit
shows a delete plus an untracked file rather than a rename, and a commit naming
only the old path would record the deletion alone.

**No ADR from this session.** Every decision above fails the first clause of
`ADR-FORMAT.md`'s AND test: crate placement is a `git mv`, the leaf cut is a
tree the next session can reshape, the reference domain is a test fixture. The
two records the design earned are already filed.
