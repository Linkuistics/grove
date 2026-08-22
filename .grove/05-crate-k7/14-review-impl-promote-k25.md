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
