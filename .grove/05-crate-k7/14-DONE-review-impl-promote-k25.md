# promote-k25

**Reviews:** promote-k12

## Goal

Read `promote-k12`'s artifact adversarially and produce findings. The artifact is
`ops::promote` and its `level_of` helper, the three new `Refusal` variants and
their `Display` arms, `WriteGuard::promote`, the thirteen algebra tests appended
to `src/ops/tests.rs`, the two interpreter tests appended to
`src/fs/apply/tests.rs`, the seven in `tests/promoting_on_disk.rs`, the
`Contentless` domain in `src/fixtures.rs`, two corrected passages in
`ARCHITECTURE.md` (*Operations*' preamble and the `promote` refusal bullet) and
entry 014 of `docs/formalism-findings.md`.

This is the operation with the most that can go wrong, and the only one by which
the library can damage a tree it was handed. That is why it had a leaf to itself,
and it is why it has this.

## Context

Beyond the brief chain and the *Read first* list in `05-crate-k7/BRIEF.md`:

- `promote-k12`'s own task file, whose `## Decisions (running log)` states eight
  calls this session made. Each is a place to push.
- `operations.qnt`'s `planPromote`, `inv_promoteKeepsIdentity`,
  `inv_freshKeysAreFresh` **and its explanatory comment**,
  `inv_ordinalsDistinctThroughout`'s `midPromote` exemption, and the six promote
  witnesses — `wit_refusedPromoteNotLeaf`, `wit_refusedPromoteNoDistinguished`,
  `wit_refusedPromotePartsNotNode`, `wit_promoteWithChild`,
  `wit_promoteTransientlyDuplicatesAKey`, `wit_promoteTransientlyDuplicatesAnOrdinal`
  — plus `wit_partialRollbackLeavesADuplicateKey` and the `no_distinguished` and
  `rollback_fails` instances. Both suites were re-run green and **unchanged** by
  this leaf: Alloy 20/20, Quint every claim across all eight instances.
- `docs/formalism-findings.md` entry 014, including its two recorded misses, and
  entry 003, whose 0.07% figure entry 014 argues is misleading about test cost.

## Done when

Findings exist, or the session concludes there are none and retires without
cutting an integrate leaf.

## Notes

Six specific doubts this session could not settle on itself, written down so the
reviewer does not have to find them first. They are a floor, not a ceiling.

**The transient-duplicate test may check a weaker property than the claim it
names.** `a_promotion_passes_through_a_state_where_the_leaf_and_the_node_share_an_ordinal_and_a_key`
names `wit_promoteTransientlyDuplicatesAKey` and
`wit_promoteTransientlyDuplicatesAnOrdinal`, and what it actually asserts is that
the created node's name carries the leaf's ordinal and key, and that the second
effect's level is `Level::Created(0)`. It never constructs the intermediate
directory state and never asserts that two entries are simultaneously present at
one ordinal. `insert`'s equivalent replays the landings over the level with a
hand-written fold; this one does not. Entry 005's shape — *a test that names a
claim and checks a weaker property, drifting toward the property that was easier
to observe* — is exactly what to look for here. Is the structural assertion
sufficient, given that the leaf's index is only *implied* to be still in place
after effect one, or does this need the fold?

**The first-effect create is guarded against a level the leaf still occupies, and
nothing states why that is safe.** `Plan::guarded` folds the plan through the
snapshot in order, so effect one's destination — `compose(leaf.ordinal,
leaf.key, node_parts)` at the leaf's own level — is checked while the leaf is
*still there*, unvacated. It does not collide only because the node's parts
differ from the leaf's, and they differ only because the species follows from the
parts. So the promotion's applicability rests transitively on the trait's
*species follows from parts* obligation, through `occupied`'s view comparison,
and neither `ops::promote` nor `ARCHITECTURE.md` says so. Is that reasoning
sound? Can a conforming domain make `compose(o, k, node_parts).view() ==
compose(o, k, leaf_parts).view()` — for instance one whose `Parts` `Eq` is
coarser than its species function — and if so, what does a promotion do on it?

**`level_of` is a second way of naming a level, beside `resolve`.**
`ops::resolve` already maps a target to a `Level`, and `promote` needed the
*container* of an entry instead, so `level_of` was added. It reads
`Container::entry()` and answers `Level::Root` for `None`. Is that the same
mapping `resolve` makes, on every level — including a leaf directly under the
root, a leaf inside a node, and a leaf inside a node inside a node? Should the
two be one function? And does `Level::Entry(node.index())` reach the right
directory through `Run::level_path` when the node is at depth two or more, where
`entry_path` walks `ancestors()`?

**The `Contentless` domain exists twice, in two files, and the brief says test
seams are shared.** `05-crate-k7/BRIEF.md` says *one reference domain
implementation of `EntryName` … shared by every test and by the CLI — so the
document's examples and the fixtures cannot drift apart*. `Contentless` is not
the reference domain, but it is now written out in full in both
`src/fixtures.rs` and `tests/promoting_on_disk.rs`, because the first is
crate-private and the second is an integration test. Two copies of one domain
can drift, and if they do, one of the two `no_distinguished` tests silently stops
testing what it says. Is the duplication justified, or should the integration
test be dropped, or should something be made visible for tests only?

**The refusal order is transcribed from the model, and the model's order may be
incidental.** `planPromote` is a chain of `if`s and Quint has no notion of
*first* — a refusal is an outcome, and the model never claims which of two true
refusals is reported. So `the_refusals_are_reported_in_the_models_own_order`
pins an order that nothing above it requires. Is pinning it right, is the order
the *useful* one for a consumer, and is a test that can only fail when someone
deliberately reorders two branches worth its line count?

**Two passages of `ARCHITECTURE.md` were rewritten on this leaf's own
judgement.** The `promote` refusal bullet now says the distinguished-child case
cannot be asked for, and the *Operations* preamble now says `promote` and
`rewrite` take a bare key while the level-taking operations take a target. Both
are claims about **every** operation, made from inside the leaf that implements
one of them, and `rewrite` has not been written yet. Is the preamble's claim
true of `rewrite` as the model has it? And does the refusal bullet's new wording
survive the case where a domain's `parse` is *wrong* — a tree already holding a
distinguished child at a name that also parses as positioned?

## Findings

### Medium — `Parts::Eq` can make every valid promotion look occupied

`crates/ordinal-fs-tree/src/ops.rs:370`–375 says the first destination cannot
collide because the node's parts differ from the leaf's, but the guard decides
that at `src/plan.rs:216` by comparing whole `NameView`s. Their ordinals and keys
are deliberately identical, so the decision reduces to `leaf_parts ==
node_parts`.

The trait requires only `Parts: Clone + Eq` (`src/name.rs:398`). Neither its
seven obligations nor the conformance kit requires equality to preserve
`positioned_species`. The signature of `positioned_species(&Parts)` prevents the
answer varying with `self`, ordinal or key, but it does not prevent two distinct
values in one `Eq` equivalence class from producing different species. A domain
can therefore have leaf and node parts with the same label compare equal while
`positioned_species` distinguishes their variants. That is a lawful equivalence
relation, `compose` can place the exact parts it was given, and all current
conformance checks pass — the kit compares composed parts with the same `Eq` at
`src/conformance.rs:338` and reports species-following-from-parts as discharged.

For that conforming domain, promoting a leaf with same-label node parts builds a
node view equal to the still-present leaf view. `Plan::guarded` returns
`Refusal::DestinationOccupied` before any effect, so the public operation cannot
perform the promotion its contract promises. Either name identity must include
the positioned species, or the seam needs an explicit congruence obligation
(`a == b` implies equal positioned species) with an adversarial conformance and
promotion control. The current reference domain's derived equality makes every
existing test miss the case.

## Doubt verdicts

1. **The transient-duplicate test is sufficient.** `Effect::Create` cannot
   vacate the leaf; the test establishes the same level, ordinal and key and
   then establishes that the move targets the level created by effect zero. A
   fold would replay facts already structural in the effect variants.
2. **The first-effect occupancy argument is not sound for every conforming
   domain.** This is the finding above.
3. **`level_of` is sound.** `Entry::container` is the immediate containing
   level; `None` is exactly the root and `Some(node)` is exactly
   `Level::Entry(node.index())`. `Run::level_path` delegates that entry to
   `entry_path`, whose root-first ancestor walk works at arbitrary depth. The
   helper is the container counterpart of `resolve`, not a competing target
   resolver.
4. **The two `Contentless` implementations are justified test scaffolding.**
   The integration test must cross the public crate boundary while the unit
   fixture deliberately remains crate-private. Making either test-only type
   public would widen the very seam under review; dropping the on-disk test
   would lose the public-surface control. It is not the shared reference domain
   the brief requires every ordinary example to use.
5. **The refusal order is sound.** Quint's nested `if` chain does have observable
   priority when two predicates are true, even though no separately named claim
   states it. Treating the checked model as the specification therefore supports
   transcription, and node-before-domain is the useful order for a call that
   could not work even in a domain with a distinguished child.
6. **Both architecture corrections are sound.** `TagRewrite` carries a bare key
   in the unchanged model, and `by_key` derives its answer from a triple, so a
   genuinely distinguished name cannot be supplied to `promote`. A broken
   `parse` that presents the spelling as positioned has made it a positioned
   entry from the library's perspective; that is a trait-obligation violation,
   not a reachable distinguished-child case the refusal should enumerate.

Codebase-memory had no project for this jj workspace. Indexing through both the
CLI and MCP failed before parsing source because active-daemon coordination
could not be verified; the only existing grove graph predates this crate and
returned no candidate symbols. The required current-workspace coverage call was
also unavailable under the harness's never-approve policy. This review therefore
uses producer commit `fd5a7567`, complete direct reads of every changed source
and test hunk, and the exact model predicates; no negative claim relies on the
empty graph result.
