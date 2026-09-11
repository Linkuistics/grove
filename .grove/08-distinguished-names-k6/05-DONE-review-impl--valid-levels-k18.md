# valid-levels-k18

**Reviews:** valid-levels-k16

## Goal

Adversarially review the completed distinguished-level enforcement boundary
before `node-files-k7` builds Grove's required node-file grammar on it.

## Context

Read the producer's commit and this parent's charter. The contracts are
`docs/ordinal-fs-tree/ARCHITECTURE.md`, the entry-name seam ADR, and formalism
finding 049. The supplied-name review and its integration at
`supplied-names-k17` establish the preceding API boundary.

The changed implementation includes full snapshot reads, final plan projection,
pre-effect checks, path-bearing domain and cardinality errors, the explicit
`LevelSample` conformance API, API consumers, regression fixtures, and the
ordinal-fs-tree/grove-loop books and validator inventories.

## Done when

- Inspect whether projected names and parent identities preserve every reachable
  level through initialization, append/batch/insert, promotion with its optional
  child, rewrites and sibling shifts. Look for successful plans leaving a level
  unreadable, skipped levels, incorrect projected diagnostic paths or an effect
  occurring before rejection. Distinguish legal plans from impossible internal
  effect sequences.
- Check complete-listing validation, full-tree validation before searches under
  both guards, domain-error precedence and independent competing-name rejection.
  Check the unchanged reported-error rollback and process-interruption limits.
- Examine whether expected conformance verdicts are independent, whether sampling
  claims match actual coverage, and whether missing samples remain untested.
  Check the different root/node names and node-parts fixtures separately from
  the conformance kit and the bounded model evidence.
- Review affected source fragments, explanatory prose, ledgers and validator
  corruption fixtures for semantic drift beyond byte reconstruction. Grove's
  current filename grammar must remain consistent at this library boundary.
- Record findings without fixing them. If integration is earned, place it in
  this node after the review and before the grammar consumer can run.

## Notes

No in-session reviewer was used by the producer. The installed binary, plugin
and live-tree grammar remain outside this review's mutation scope.

## Findings

Reviewed: commit `yrquyqyp` (`bbd60935`), *valid-levels-k16: enforce
distinguished levels on reads and final plans*, read against the working tree
at that commit (the working copy was empty on top of it). The artifact is the
enforcement boundary in `ordinal-fs-tree`, its consumers, tests and the two
books; coordinates are `path:line` in that tree. Inspection only: no build,
test, check or model runner was executed, and the producer's recorded
verification was read, not reproduced. Ordered by severity.

### F1 — The trait doc now contradicts itself about whether the level rule is invoked (actionable, medium)

`crates/ordinal-fs-tree/src/name.rs:460-466` is the contract every consumer
reads for `validate_distinguished`. Lines 461-462 say *the reader and planner
invoke this before exposing a level or applying effects*; line 466, kept from
the previous boundary, still says *declaring this policy does not invoke it*.
Both sentences cannot be true, and the second was written to mean *nothing
calls it yet* — the earlier review cited it as the correct statement of that
boundary. Line 465 also restates line 463's order-independence sentence. The
book reproduces the same paragraph byte for byte at
`docs/walkthroughs/ordinal-fs-tree/02-name-seam.md:655-660`. Correction: drop
the stale clause (or reword it to *the default is permissive*), collapse the
two order-independence sentences, and resync the fragment.

### F2 — The syllabus CLI's exit-code contract and the "half the surface" claims were not extended (actionable, low–medium)

`crates/ordinal-fs-tree/bin/syllabus.rs:698-703` maps `InvalidLevel` and
`CompetingDistinguished` to exit `5`, but the table that is the CLI's stated
contract, `docs/ordinal-fs-tree/CLI.md:442`, still enumerates only `Malformed`,
`Reserved`, `NonUtf8Name`, `NameIsNotOneComponent`, `RootIsNotATree` for that
row. `CLI.md:627` and `docs/ARCHITECTURE.md:421` both argue from *`Malformed`
and `Reserved` carry `EntryName::Err`*; `error.rs:20-23` now names three such
variants. The sentences are not false, but the argument they carry — which half
of the error surface speaks the domain's words — is now understated by one
variant on the domain's side. Correction: add the two variants to the exit-5
row and mention `InvalidLevel` in the two "carries the domain's error"
sentences.

### F3 — The grove-loop book's file total for `task_name.rs` is stale in seven places, and its prose block ranges in three chapters (actionable, low)

`task_name.rs` grew from 1,712 to 1,743 lines in this commit; the manifest
(`walkthrough.toml:214`), the ledger (`source-index.md:13`) and the crate total
(10,562 → 10,593, `README.md`, `21-what-could-not-move.md:447`) were updated.
The prose was not: *the file's 1,712 lines* stands at `02-the-tokens.md:33`,
`03-kind-slug-handle.md:60` and `04-the-name.md:86`, and
`21-what-could-not-move.md:87`, `:112`, `:285` and `:448` (*the five parts are
1,712, 2,556, 2,732, 516 and 2,601* — that sum plus chapter 1's 445 is 10,562,
not the 10,593 the same paragraph now states). This is the mirror image of the
previous review's F6, which the integration fixed one leaf ago.

Inherited and widened, not introduced: the block-range sentences at
`02-the-tokens.md:35-36` (*1,178 to 1,199*), `03-kind-slug-handle.md:62`
(*1,522 to 1,550 and 1,551 to 1,712*) and `04-the-name.md:87-88` (*1,021 to
1,177 … 1,200 to 1,312*) already disagreed with the manifest by two lines
before this commit and now disagree by up to 31 (manifest: `1207-1228`,
`1342-1550`, `1551-1579`, `1580-1743`, `1017-1206`, `1229-1341`). The
validator compares fragments and ledger ranges by bytes and never a prose
number, which is why `scripts/check.sh` stayed green. Correction: one pass over
the three chapters and chapter 21 against the manifest.

### F4 — The obligation arithmetic is now told three ways (actionable, low)

`Obligation::ALL` has six entries (`conformance.rs:82-89`) and the kit test
asserts six plus two equals eight, *the name obligations plus deterministic
level validation* (`tests/conformance_kit.rs:143-147`). The surrounding prose
still counts the old way: `name.rs:355-362` says *seven obligations … the
conformance kit samples five semantic obligations*; the `TYPE_SHAPE_CONSTRAINTS`
doc at `conformance.rs:166-167` says *sampled checks where the document states
seven*; the kit test's own doc at `conformance_kit.rs:138-141` says *five
sampled checks … seven obligations*; and the book's section heading
`03-reference-domain.md:741-762` says *seven obligations … five sampled checks
… the kit samples the other five*, then adds the level-sample paragraph beneath
without reconciling the count. The library architecture lists eight bold laws
with level validation as *a separate reader and planner obligation*
(`ARCHITECTURE.md:243-244`, `:327`). Correction: pick one statement — seven
name laws, of which five are sampled and two are shape-constrained, plus one
separately sampled level rule — and say it identically at the four sites.

### F5 — The kit's untested signal covers the context axis only, so a fixture missing a shape reads as full coverage (decision to settle, low)

`conformance.rs:344-353` reports `LevelValidationMatchesSamples` untested only
when root or node context is absent. The architecture's claim for the kit is
*root and node contexts, empty, singleton, competing and permuted samples*
(`ARCHITECTURE.md:357-358`); the empty, singleton and competing shapes are
supplied by convention and never checked for, and no sample with
`accepted: false` is required either. A required-name domain handing the kit
two singleton samples therefore passes this obligation with no refusal ever
sampled, whereas `DistinguishedNamesAreCanonical` reports untested on an empty
slice (`:617-621`). Relatedly, the "competing" samples in the reference domain
(`tests/conformance_kit.rs:54-75`), the `Blind` domain
(`tests/promoting_on_disk.rs:552-567`) and Grove's `TaskName`
(`task_name.rs:1099-1123`, and the book prose *missing, single and competing
brief values* at `04-the-name.md`) present one name twice, a set no listing can
produce; only the `Required` fixtures in `tests/valid_levels.rs:341-388` present
two distinct names. Two ways to settle it: report untested when any of the
three shapes, or a refusing verdict, is absent — matching the canonical-name
check's discipline — or state at the untested message and in the architecture
that the kit polices contexts only and the shapes are the fixture author's.

### F6 — A planning-time `InvalidLevel` names a directory that does not exist (visible trade-off, trivia)

`error.rs:57` says the path is *including a projected destination when
planning*, and `tests/valid_levels.rs:334-337` pins that an insert refusal
reports `02-ordinal-i1` — the shifted node's destination — while the operator's
tree still holds `01-ordinal-i1`. The `Display` at `error.rs:399-401` is the
same sentence for a read-time path that exists and a plan-time path that does
not. This is a coherent choice and the ADR does not decide it; record it, or
add the on-disk name for moved levels, but do not treat it as a defect.
Observation in the same class: initialization runs `validate_snapshot` twice on
the same projection (`fs/mod.rs:464` and again inside `apply.rs:58`); harmless
and O(n).

### What was checked and holds

- **Every reachable level is preserved by the projection.** `Plan::projected`
  (`plan.rs:159-196`) keeps snapshot entries and plan-created nodes in disjoint
  identity spaces — `Level::Entry(i)` and `Level::Created(effect_index)`, the
  same convention the interpreter's `Run.landed` uses (`apply.rs:112-114`) and
  that `promote` builds with `Level::Created(0)` (`ops.rs:366-370`, `:388-392`)
  — and re-parents by identity, so a shifted or rewritten node keeps its
  children under its new name and an ancestor rename cannot detach a subtree.
  `slot()` is dense over `snapshot.len() + effects.len() + 1`, and `len()`
  counts distinguished children (`snapshot.rs:633-636`), so no legal plan
  indexes out of range. Walked by hand for initialization, append, batch,
  insert (highest-first shifts), promotion with and without a child, and a
  rewrite; each final level has exactly the distinguished set the operation
  leaves on disk, and intermediate promotion states are never checked, as the
  architecture requires (`ARCHITECTURE.md:352`).
- **Nothing lands before rejection.** `apply.rs:57-58` orders the component
  check before the projection (so a `../` rendering is never joined into a
  diagnostic path), and both precede `Run`; `fs/mod.rs:463-464` runs both before
  `create_dir` of the root. `Run`, `Undo` and the initialization unwind are
  unchanged, so rollback and process-interruption limits are as before.
- **Complete listing, whole tree, both guards, domain first.** `read.rs:23-86`
  parses the full direct listing, then `validate_level` at `:74`, and the
  snapshot is returned only after every level; `acquire` (`fs/mod.rs:168-187`)
  is the one path for `read` and `write`, so no search can run on an
  unvalidated tree. `validate_level` (`read.rs:89-105`) calls the domain, then
  the cardinality rule, so a strict domain's canonical form wins and a
  permissive one still cannot expose two (`tests/valid_levels.rs:156-203`).
- **Legal plans versus impossible sequences.** The planner-path
  `CompetingDistinguished` witness is a private self-colliding plan
  (`fs/apply/tests.rs:37-67`) that `guarded` would refuse as occupied; no public
  operation can produce two distinguished names in one level from a validated
  snapshot, so its absence from the public suite is correct, not a gap. Every
  public constructor and the rewrite have a refusal-without-effects fixture
  (`tests/valid_levels.rs:222-260`) and a success fixture with different root
  and node names (`:262-299`); the node-parts and shifted-ordinal policies the
  bounded models do not cover are exercised at `:87-96`, `:246-247`, `:322-339`.
- **Conformance verdicts are independent and finite.** `check_levels`
  (`conformance.rs:342-390`) reads `accepted` from the fixture only, covers all
  permutations for sets of three or fewer, and says so; the deliberately wrong
  fixture (`tests/valid_levels.rs:301-320`) and the order-dependent domain
  (`:390-417`) are the positive controls. The empty-slice limit settled in the
  previous integration is unchanged (`:617-621`).
- **Grove's boundary is unchanged.** `TaskName` keeps the permissive default,
  the kit runs over it with both contexts (`task_name.rs:1099-1132`), and Grove
  has one distinguished spelling, so neither new error is reachable from a
  `.grove/` today; `task_tree.rs:271-287` lets both fall to the generic arm,
  whose `Display` already carries the path. The models under
  `docs/ordinal-fs-tree/models/` were not touched, and the code cites
  `inv_successHasValidLevels` and `RefusedInvalidDistinguished` where finding
  049 places them.
- **Books and fixtures.** Every changed root's fragments, ledgers and manifest
  ranges moved together; the book-validation inventory, expected totals and
  corruption fixtures (`crates/book-validation/tests/*`) were re-tuned to the
  new line counts rather than weakened; the new prose in chapters 3, 4, 5, 6
  and 8 of the library book states the contract as the code implements it.
  `CHANGELOG.md:54-57` records the change.

## Decisions (running log)

**1. Six findings; one integration leaf.** F1–F4 are actionable against the
producer's own Done when (*every affected book, ledger, index and explanatory
paragraph*) and against documents that state the library's contract; F5 is a
kit-coverage decision the grammar consumer would otherwise inherit; F6 is a
recorded trade-off. None is a defect in enforcement, so no redesign is asked
for. The integration is appended with `leaf-add` under this node with the bare
slug `valid-levels`: no later sibling entry in this directory holds live work,
and the walk finishes this directory before `node-files-k7`, which is the
placement the task requires. Its body names this review's handle and not the
findings.

**2. Inspection only, no in-session reviewer.** Nothing was built, tested or
run; every claim above is read off the tree at `yrquyqyp`. The parent stays
live until the integration settles.
