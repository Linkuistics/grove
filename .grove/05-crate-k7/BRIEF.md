# crate-k7 — brief

## Goal

Increment 1 of the root brief: `ordinal-fs-tree` and its CLI, standing alone.
A crate that implements the design in `docs/ordinal-fs-tree/ARCHITECTURE.md`,
whose behaviour follows the two models rather than the reverse, and a CLI that
drives a conforming tree. No grove changes anywhere in this subtree — the flip
is increment 2 and its regression cover is grove's own ~130 CLI-contract tests,
which are not this increment's business.

## Done when

- The crate builds and tests green at `crates/ordinal-fs-tree/`, as a workspace
  member of this repo, with `grove` still building and testing green beside it.
- Every operation in `ARCHITECTURE.md`'s *Operations* tables exists, with its
  stated refusals, and each test names the model claim it discharges.
- A test — not a convention — enforces that the algebra cannot reach `std::fs`.
- A conforming domain can check its own name type against the five trait
  obligations without reading the architecture document.
- The CLI drives a conforming tree end to end.
- `docs/formalism-findings.md` carries an entry from every leaf here, including
  the leaves that found nothing.

## Read first, in this order

Every leaf in this subtree reads these; no leaf body repeats them.

1. `docs/ordinal-fs-tree/ARCHITECTURE.md` — the specification of record for
   everything the models do not cover, and the explanation for everything they
   do. Read the whole document once; a later leaf may read only its own
   operation's rows plus *How an operation runs*, *Refusals* and *Invariants*.
2. `docs/ordinal-fs-tree/CONTEXT.md` — this context's glossary. Its *leaf* and
   *node* are **not** grove's, and the collision is why the library is a
   separate bounded context. grove's words must not appear in the crate.
3. `docs/ordinal-fs-tree/models/operations.qnt`, its closing `HANDOFF` block
   first, then the claims your leaf's operation names; and `structure.als` for
   anything about a single tree. `run-alloy.sh` and `run-quint.sh` report
   pass/fail per claim.
4. `docs/formalism-findings.md`, entries 001–003 — **the misses before the
   hits**. They name what the models did not establish and therefore what an
   implementation has to get right unaided.
5. `docs/adr/entry-name-is-the-only-seam.md` and
   `docs/adr/entries-are-never-removed.md` — the two decisions the design
   earned, carrying the rejected alternatives the architecture document does
   not.

## Decomposition

Dependency-ordered. Each leaf leaves the crate compiling and its tests green,
and each can be verified without waiting on a sibling.

1. `seam` — the crate skeleton, the name types, the `EntryName` trait, the
   reference domain, the obligation conformance kit, the no-`std::fs` test.
2. `reading` — the lock, the snapshot and its parse trichotomy, and the five
   reading operations. The first leaf that touches a filesystem.
3. `interpreter` — plan and effect types, the interpreter with its exclusive
   create and its rollback, and the `append` family that makes both observable.
4. `insert` — the sibling shift, the highest-first ordering rule, and the
   sequential destination check the ordering rule depends on.
5. `promote` — the leaf-to-node change, its unavoidable transient invariant
   break, and the one path by which the library can damage a tree.
6. `rewrite` — attribute change in place, and the species refusal.
7. `h3-probe` — the pre-registered model-versus-prose experiment. Runs after
   every mutation exists so it can fall back to a second probe.
8. `cli-shape` — what the CLI is, given a library generic over a name type.
9. `cli` — build it.

## What binds every leaf here

**The model leads, and it leads in a specific direction.** Where a model and a
test disagree, the model wins and the test changes. Where a model and the
implementation disagree, change the **model first**, re-run its runner, and only
then the code — `operations.qnt`'s handoff block states this and states what
adding an operation to the model obliges. Every such disagreement is a finding
for `docs/formalism-findings.md`, not something to fix in passing. The working
implementation in `src/tree_*` is **not** the reference; it may be read for
prior art and never cited as authority.

**Each test names the claim it discharges.** A test derived from a model claim
carries that claim's name in a comment. A test with no claim behind it says so.
This is what lets a later reader tell a checked property from an arranged one,
and it is the H3 probe's measure — so it is not optional and not cosmetic.

**Re-run a model's suite when you touch that model; run both once, early.**
`seam` establishes that both toolchains work on this machine before anything
depends on them. Entry 003's third instrument failure is the reason: a suite of
must-hold claims cannot detect that it did not run, and the must-be-reached
witnesses are the positive control. Every witness failing at once is a broken
toolchain, never a design defect.

**Append to `docs/formalism-findings.md` before retiring.** Six fields; short is
fine, silent is not. An uneventful episode — the model and the code agreed, at
this cost — is H2 evidence, and a log that records only disagreements is a
survivorship sample.

**The shared test seams.** One reference domain implementation of `EntryName`,
the course-syllabus domain `ARCHITECTURE.md` uses for its examples, shared by
every test and by the CLI — so the document's examples and the fixtures cannot
drift apart. One internal seam for making an effect fail, which must not appear
in the public API: a second public seam contradicts
`docs/adr/entry-name-is-the-only-seam.md`, and if it cannot be kept private that
is a finding that reworks the record rather than quietly widening it.

**Consider review, do not schedule it.** A review chain is cut lazily, as the
last act of the session whose artifact needs one. `seam` is the likeliest
candidate — every later leaf is built on the trait's shape — but that is the
seam session's call, not this brief's.

## Pointers

- ADRs: `docs/adr/entry-name-is-the-only-seam.md`,
  `docs/adr/entries-are-never-removed.md`.
- Glossary: `docs/ordinal-fs-tree/CONTEXT.md`. Terms in play throughout —
  entry, leaf, node, root, distinguished child, ordinal, key, parts, species,
  verdict, snapshot, algebra, decision, plan, effect, interpreter, report,
  refusal, shift, promotion.
- Placement: the crate is `crates/ordinal-fs-tree/`, a member of a workspace
  whose root package stays `grove`. `docs/ordinal-fs-tree/` does not move while
  the crate lives in this repo — the two library ADRs, `CONTEXT-MAP.md` and
  `docs/formalism-findings.md` all link into it, and `docs/adr/` is flat and
  repo-wide because grove occupies the repo root.
- Build floor: `rust-version = "1.85"` and a clippy baseline of zero, both
  stated with their evidence in the root `Cargo.toml`. A new crate inherits
  both; `cargo clippy --all-targets` before committing.

## Settled since this brief was written

**The seam changed shape at `seam-k18`, after `seam-k17` found the discharge
claim half-true.** The eight remaining leaves here were cut against the old
surface, so three facts they need:

- **`EntryName` has `view()` and `positioned_species()` where it had `triple()`
  and `species()`.** Those two survive as readings on `EntryNameExt`, which is
  blanket-implemented and sealed — import it to call them, and note that no
  domain can override them. That is the point: *a name is positioned or
  distinguished, never neither* and *the species follows from the parts* are now
  discharged by the seam's shape, which is why the stated obligations number six
  and the conformance kit still checks four.
- **A recognised name over a contradicting listing must be `Malformed`**, not
  merely refused. `Foreign` is skipped silently — with its whole subtree when it
  is a directory — so the kit now rejects a domain that answers it, and grove's
  own domain in increment 2 inherits the constraint.
- **`ARCHITECTURE.md` and `structure.als` were reconciled again**, and
  `docs/formalism-findings.md` entry 005 carries why the first reconciliation
  overstated what Rust guarantees. Read it before trusting a *discharged by the
  type system* note in either.

**The reading layer was reviewed and its findings integrated at
`reading-k20`.** Three facts the seven remaining leaves need, because each
changes how a leaf writes its tests or its code:

- **`Builder` and `Place` are crate-private, so a test that builds a tree by
  hand is a unit test.** The pure algebra tests live in `src/snapshot/tests.rs`,
  not in `tests/`. A later leaf testing a decision or a plan against a hand-built
  snapshot puts those tests beside the module they test and reaches the builder
  through `super`; `tests/` keeps the tests that go through the public surface —
  which means through a real directory. A `Place` also carries which builder
  handed it out and panics deterministically if it came from another.
- **The no-filesystem guard is a token scan, not a text scan.** It lexes every
  source outside `src/fs/` with `proc-macro2` and refuses the identifier `fs`;
  the only exemption is the token sequence `mod fs ;`. A new module is still
  inside the algebra by default, and prose about the filesystem is now free —
  but a *binding* named `fs` is a false positive that fails loudly.
- **The lock names the tree, not a spelling of it.** The directory locked is
  `<root>/..`, resolved by the kernel, so `..` routes and a symbolic link naming
  the root all reach one inode while every reported path stays in the caller's
  own spelling. `ARCHITECTURE.md` states the property; do not reintroduce a
  lexical parent, and do not canonicalise to fix anything.

**The plan interpreter was reviewed and its findings integrated at
`interpreter-k22`.** Five facts the six remaining leaves need, because each
changes what a leaf may assume or must test:

- **`EntryName` has a seventh obligation, and it is the first one the library
  enforces.** *A name renders as one path component* — not empty, not `.` or
  `..`, no separator. It is checked at both boundaries where a name becomes a
  path: every name a snapshot admits, and every name a plan will place, the
  latter **before the first effect** so a plan carrying one changes nothing. A
  violation is `Error::NameIsNotOneComponent`. The conformance kit now checks
  five obligations and names two as discharged; a leaf citing *four checks
  against six* is quoting a superseded count. Nothing a new operation writes
  needs to repeat the check — it lives in `fs::apply` and `fs::read` — but an
  operation that reached the filesystem by any other route would bypass it.
- **A same-path `MoveTo` is a no-op that succeeds**, claims nothing and
  registers no undo. `rewrite-k13` depends on this directly:
  `wit_rewriteToSameParts` requires a rewrite to the parts an entry already
  carries to succeed, and the interpreter used to refuse it. The undo half is the
  subtle one — an `Undo::Restore` for a no-op renames onto its own occupied path,
  so registering one would turn a clean rollback into
  `FailedPartiallyRolledBack`.
- **`Report::paths()` is now the plan's own landing order**, while `created()`
  and `renamed()` keep each species' own order. `insert` and `promote` build the
  first mixed plans an operation produces — shifts then a create, and create,
  move, create — so they are where this becomes observable through the public
  surface, and a test of it belongs with them.
- **There is a third internal failure point: `Faults::at_content(i)`**, which
  fires after a leaf's destination is claimed exclusively and before its bytes
  are written. It exists because the undo registration sits in that interval and
  `at_effect` cannot reach it. Still internal, still `cfg(test)` for its
  constructors, and still not a second public seam.
- **The claim account is forty-eight tests here, thirty naming a model claim and
  eighteen saying they have none** — and re-reading the forty-two found three
  that named *neither*, which a count of two kinds cannot see. When a leaf states
  its own account, count the tests too, not only the two labels.

**`promote` was reviewed and its finding integrated at `promote-k26`.** Two
facts the five remaining leaves need:

- **Name identity is the view *and* the species, and it has a name:
  `EntryNameExt::same_name`.** A `Parts` equality coarser than the domain's own
  species is lawful — nothing in the seam forbids it — so `view() == view()` is
  not *the same name*, and a promotion, whose new name deliberately reuses the
  leaf's ordinal and key, was refused as `DestinationOccupied` in exactly such a
  domain. Any later code asking whether two names collide uses `same_name`; a
  leaf citing an occupancy that compares views is quoting a superseded rule. The
  obligation count is unchanged at seven, five checked: the congruence was
  deliberately **not** made an eighth, because no sample of parts can exercise
  one and the kit reports an unexercised obligation as a finding.
- **Both models are silent here by construction, not by omission.**
  `structure.als` compares `Parts` atoms and `operations.qnt` compares ints, so
  *equal parts imply equal species* is free in both. `docs/formalism-findings.md`
  entry 015 generalises it: an opaque sort in a model is compared by identity,
  and the target language compares it by whatever bound the interface states.
  Worth applying to `Key` and to the parts a `rewrite` compares before trusting
  either.

**`rewrite` landed at `rewrite-k13`, and with it the last mutation.** Three
facts the three remaining leaves need:

- **Every operation in `ARCHITECTURE.md`'s *Operations* tables now exists**,
  which is the precondition `h3-probe-k14` was waiting on. Neither model changed
  and neither contradicted the code — the third leaf running — so the probe runs
  against a complete operation set with both suites green (Alloy 20/20, Quint
  every claim across all eight instances).
- **State a claim account by counting, never by adding up the per-leaf
  numbers.** `rewrite` contributed fourteen tests, ten naming a claim, four
  saying they have none, and none naming neither — but `interpreter-k22` found
  three tests naming *neither* only by re-reading the forty-two, which no sum of
  two labels can see. The probe's measure is the crate-wide account, so it is
  recomputed from the suite rather than accumulated from these notes.
- **`Refusal::RewriteSpeciesChange` carries the entry's species and not the
  supplied one**, because two positioned species make the second derivable and
  two fields that restate each other can disagree. It is also the one refusal
  whose message is *not* a function of its payload alone: the advice branches on
  direction, since a leaf can become a node by `promote` and nothing turns a
  node into a leaf. A CLI rendering refusals must not assume one message per
  variant.

**The CLI's shape is settled at `cli-shape-k15`, in
`docs/ordinal-fs-tree/CLI.md`.** That document is `cli-k16`'s specification and
this brief does not restate it. Three facts that are about the **crate** rather
than about the CLI, and that the node close needs:

- **The crate gains a `cli` feature, on by default, and a `[[bin]]` whose source
  is at `bin/syllabus.rs` — outside `src/`.** Measured, not assumed: a probe at
  `src/bin/` calling `ordinal_fs_tree::fs::read` fails
  `the_algebra_cannot_reach_the_filesystem`, because the guard lexes every source
  under `src/` and refuses the identifier `fs`. The feature is default-on so the
  binary and its contract tests are inside a plain `cargo test`; an external
  consumer takes the bare library with `default-features = false`.
- **No ADR was earned, and the clause that failed is hard-to-reverse.** The
  rejected alternatives — a generic command factory, and one parameterised by a
  parts-parser — therefore live in `CLI.md`, which is the durable artifact.
  `.grove/` is deleted at the finish, so do not close this node expecting a
  record that was deliberately not written.
- **`docs/formalism-findings.md` entry 018 is this leaf's**, and it is a
  no-formalism entry by design: the routing rule from entry 009 predicted zero
  coverage before the leaf started and the prediction held. The node's *Done
  when* is met by it.

## On the horizon

Nothing. `cli-k16` closed this node, and the one item that stood here — whether
the crate wants its own `CHANGELOG`, version and release lane — was promoted to
the root brief, which is the chain every increment-2 session reads.

## Notes

The library's Unix-only build is deliberate and invisible in the interface —
locking is entirely internal, so gaining a platform later changes no signature
and no caller. Do not thread a platform parameter through anything to prepare
for it.
