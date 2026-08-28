# delete-migration-k6

## Goal

Delete the migration path and the legacy tree format outright. No legacy tree
needs either, and a genuinely legacy tree is meant to fail on its **names** —
through `TaskNameError`, which already carries what is on disk and what it
should be — rather than be repaired.

## Context

- `docs/specs/module-decomposition.md`, `## Out of scope` — *"Migration.
  Deleted rather than preserved."*
- `decomposition-k2`'s `## Decisions (running log)`, the paragraph beginning
  **"`.grove/FORMAT` is deleted with migration"** — the warrant, including why
  the discriminator is not load-bearing on anything that survives.
- `minimalism-k1`'s `## Deletion list`, *Contained* row 2: `tree_migrate`,
  `tree_migration_transaction`, `repo/migration_commit`, **3,373 non-test
  lines**, with **no caller outside the deleted set** — 4 + 4 sites, all in
  `tree_lifecycle` and in each other; `migration_commit` has one caller,
  `repo.rs`.

Suites: `tests/migration_commit.rs` (663), `tests/migration_transition.rs`, and
`tests/lifecycle_cutover.rs` (1,946) — `minimalism-k1` names the last as a suite
for deleted machinery; confirm that from its contents rather than from the name
before removing it.

## Done when

- `src/tree_migrate.rs`, `src/tree_migration_transaction.rs` and
  `src/repo/migration_commit.rs` are gone, with every `tree_lifecycle` and
  `repo.rs` call site reconciled.
- Nothing writes, reads or requires `.grove/FORMAT`. `src/tree_format.rs` goes
  with it or shrinks to whatever survives the discriminator's removal — see the
  open question below.
- ~~**The new binaries are installed before this tree loses its witness, and this
  session ends without signalling.**~~ **Withdrawn — this is not a cutover leaf.**
  Deleting the file from *this* tree is what forced the install, and the file does
  not have to go now: nothing reads it, and a stray `FORMAT` is an ordinary
  foreign entry every reader ignores. Leaving it keeps the tree readable by the
  installed 19.3.0, which is what this grove and three others are still driven by.
  The file's deletion rides with the first release that ships this change; see
  `## Decisions` and the root brief's standing notes, both corrected.
- The migration suites are deleted; `cargo test` and
  `cargo clippy --all-targets` are clean.
- Any auto-repair function that existed only to unwind a half-run migration goes
  with it, becoming nothing (principle 2: a message, not machinery — and here
  there is no longer even an anomaly to report).
- `CHANGELOG.md`'s `## Unreleased` records the removal.

## Notes

**Lands green.** This is a contained deletion: the caller counts in
`minimalism-k1` were taken against `src/` on 2026-08-28 and exclude the deleted
set's own subtree. Re-derive them rather than trusting them — the count is a
year-zero measurement, not a contract.

**One open question the design log settles against the requirements.**
`minimalism-k1`'s *"Not deleted, and worth saying so"* lists `tree_format` as
surviving; `decomposition-k2` then decided `.grove/FORMAT` is deleted with
migration. The later decision governs the *file*. Whether the *module* has
anything left to do once `require_current` and the discriminator go is this
session's call — if it does not, delete it and say so; the requirements sentence
was written before the FORMAT decision existed and is superseded, not
contradicted.

**~~This is a cutover leaf~~ — it is not, and the hazard it names is real but
avoidable.** Deleting the witness from a live tree under the *old* installed build
does two things, both checked in the source rather than assumed: every `grove-llm`
verb that reads the tree fails, because `tree_format::require_current` bails on a
missing witness with *"this is a legacy tree and must be migrated"*; and the
running driver's next iteration reaches `tree_migration_transaction::run_unlocked`
through `transition_driver_to_current`, which on an absent witness plans a
migration of this tree. Both were true. The conclusion drawn from them — that the
session must install new binaries — did not follow, because **not deleting the
file avoids the hazard entirely and costs nothing.** See `## Decisions`.

**Do not touch the finish transaction here.** It is `delete-finish-transaction-k8`'s,
and the two share a shape (`*_transaction.rs`, quarantine, rollback) that makes
it easy to over-reach. This leaf's blast radius is migration and FORMAT only.

## Decisions (running log)

**`tree_format` goes entirely; the module had nothing left.** The open question
above is answered: once `require_current` and `write_current_last` go, the module
is empty. `minimalism-k1`'s *"Not deleted, and worth saying so"* row is superseded
by `decomposition-k2`'s FORMAT decision, exactly as the leaf body anticipated.

**`tests/lifecycle_cutover.rs` is not a migration suite and was not deleted.**
`minimalism-k1` named it as one; its contents say otherwise. Of its ~22 tests,
six concerned migration (two jj legacy adoptions, the legacy tree adoption, the
kindless-v2 adoption, the withdrawn-layout refusal, and the config reload after a
legacy transition). The rest — build pairing, spawn failure, the finish sentinel,
skill restoration, config validation, worktree anchoring — survive untouched. The
leaf body said to confirm from contents rather than from the name, and that was
the right instruction.

**`recover_partial_root_init_unlocked`'s byte-exact content match is deleted with
migration, and the classification is now one question of the listing.** The
120-line match existed to tell a *partial root-init scaffold* apart from a
*legacy tree*, because a witnessless root was both. With legacy gone the
discrimination has no job. `tree_lifecycle::root_shape` now returns one of three
answers from the root's own listing: `PartialScaffold` (nothing but the charter,
if that), `ATree` (at least one name the grammar owns — an entry, or one it
refuses), `Unrecognised` (names grove disclaims, and nothing else). It rests on
`entries-are-never-removed`: a tree that has ever held a leaf still holds one, so
"no keyed entry" can only mean root-init did not finish.

**The `Unrecognised` arm is new work this leaf added deliberately, and it is not
migration.** Deleting migration would otherwise have introduced a regression the
leaf body did not anticipate: the withdrawn `NNN-slug/` + `done/` and v1-flat
layouts are positioned but *unkeyed*, so every one of their names is `Foreign` —
invisible to the reader rather than refused by it. Such a tree would have read as
an empty grove and taken the driver's finish sentinel. The refusal is one
classification over the listing, no repair and no per-layout matcher: what died
with migration is the recognition of *which* withdrawn layout this is, which the
operator does not need in order to act (principle 2). `minimalism-k1`'s premise —
"no legacy tree needs it" — holds for the *conversion*; it does not extend to
silently writing into one.

**Two whole-verb sweeps were retargeted rather than deleted.**
`every_tree_verb_refuses_a_pending_migration_before_format_validation` carried a
property that outlives its vehicle — every agent-side verb refuses a tree held by
a transaction witness — and there was no equivalent sweep for the *finish*
witness. It is now `every_tree_verb_refuses_a_pending_finish_transaction`.
Likewise `driver_lease`'s two migration-vehicle tests were re-aimed at the one
tree mutation the driver still performs: completing a partial scaffold.

**One test's property lost its only injection point and was deleted.**
`config_is_reloaded_after_a_completed_legacy_transition_before_launch` proved the
driver reloads configuration *after* the lifecycle transition, by corrupting the
config from a `git` stand-in during migration's commit. The transition now takes
no commit, so grove drives no process in that window and the claim has no
black-box vehicle. The pre-transition load is still covered (an invalid config
leaves current, empty and partial trees byte-identical); whether the second load
still earns its place is `loop-crate-driver-k22`'s to judge.

**The recorded deletion count was roughly double the truth.** `minimalism-k1`
said 3,373 *non-test* lines for `tree_migrate` + `tree_migration_transaction` +
`migration_commit`. Re-derived against the `#[cfg(test)]` boundaries, those three
plus `tree_format` are 1,804 non-test lines and 1,639 test lines. Net across the
whole change: 6,096 deletions against 627 insertions. The leaf body was right to
say re-derive rather than trust.

**No framework or library version decision arose.** This is a deletion within
grove's own code; nothing here depends on the version of a dependency, so
`references/impl.md`'s source-citation rule had nothing to bind to.

**Rebased onto `main`, which had moved a long way.** The branch's eight commits
now sit on `main`'s tip. Four documents conflicted — `CONTEXT-MAP.md`,
`CHANGELOG.md`, `CONTEXT.md` and, at the earliest commit, a spec list — all of
them two sides appending to one list; resolving the earliest propagated to six of
the seven conflicted commits automatically. No `src/` file conflicted, so the
deletions rebased untouched.

**`main` had landed a decision about this leaf's own subject, and it is reworked
in place here.** `docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md`
records a three-way classification of a witnessless root, and names *the removal
of migration* as its own reopen condition — this leaf. Its principle survives
intact: `root_shape`'s three branches are the same ordered fail-closed test,
reached from the tree's shape rather than from a byte comparison. Two things in it
changed. The `Exact`/`Ambiguous` class parameter is retired, there being no byte
comparison left to split on. And the record's stated *cost* — that removing
migration would tell an operator to migrate by a command that no longer exists —
is answered rather than realised: the window it feared is now the branch that
completes, so there is no diagnostic to get wrong. Arriving at the same three-way
test from the code and from the model, independently, is what makes it worth
keeping.

**The formal-methods experiment is externalised, not absorbed.** The rebase
surfaced ~38,000 lines of modelling material on `main` whose subject this leaf
demolishes. The human settled both the boundary and the placement: the apparatus
and `docs/specs/semantic-contract.md` go, the campaign records the `linkuistics`
skills cite for their authority stay, and it lands as its own leaf rather than
inside this one — `delete-formal-models-k29`, inserted at position 09. This leaf
fixed only what its own change broke: one dangling reference into
`src/tree_migration_transaction.rs` from the semantic contract, and the two
glossary entries that linked to the deleted `[[Tree format witness]]` term.

**The cutover is withdrawn, and this leaf is not a cutover leaf.** Recorded at
the moment the human settled it: *"We should not install over 19.3.0 — if we need
to do a local install (and I'm not sure why we do) we should cut a minor release,
publish and update via homebrew. We have other groves currently running, so we
need to be aware of them as well."* Both halves were right. The premise that a
local install was needed at all did not survive being asked about — the forcing
step was deleting `.grove/FORMAT` from this tree, and that step is optional. And
three other live drivers share the installed binaries, which the plan had not
counted. The evidence is in `## Why this leaf does not install anything`.

## Why this leaf does not install anything

The cutover was planned on one premise: that this tree must lose `.grove/FORMAT`
in this session. It does not, and the four-cell matrix is the whole argument —
measured, not reasoned, against both binaries:

| | `FORMAT` present | `FORMAT` absent |
|---|---|---|
| installed 19.3.0 | reads the tree | **refuses**: *"legacy tree … must be migrated"* |
| this build | reads the tree | reads the tree |

One cell is broken and deleting the file is the only thing that reaches it. So the
file stays, every reader ignores it as a foreign entry, and the leaf's actual
subject — *nothing writes, reads or requires it* — is satisfied by the code.

**The install would also have been actively harmful, which the plan did not
weigh.** Three other grove drivers are live on this machine against the same
installed binaries: `grove.gh-issue-12` (`blinded-read-k27`),
`grove.code-walkthrough-for-ordinal-fs-tree` (`walkthrough-k2`), and the default
workspace. Overwriting the Cellar swaps the binary under all three mid-flight,
and each one's `provision::reverify_installed` restores *its own* embedded
methodology every iteration, so three old drivers would have fought the new
install over the shared skill directories. The root brief's own warning — *"a
reinstall under a live loop halts nothing and can be undone by the old process
between iterations"* — applies once per live loop, and the plan counted only this
one.

**Deployment is a release, not a file copy.** Overwriting
`/opt/homebrew/Cellar/grove/19.3.0/bin/` makes `grove --version` lie and desyncs
Homebrew's manifest from the bytes on disk. When a leaf genuinely needs the new
build deployed, the route is: cut a minor release, publish, update through
Homebrew. Checked before writing this: the new build reads both other live groves'
trees and picks the same leaf each driver is on, so such a release does not strand
them.

**Which leaf actually forces a release** is `grammar-separator-k15`: it renames
this tree onto a grammar 19.3.0 cannot parse, and there is no *don't do the
tree-visible half* option there, because the rename **is** the deliverable. That
is where `.grove/FORMAT` should be deleted too — one deployment, one tree-visible
cutover, both after a published release. The root brief and k15's own leaf now say
so. Whether `prompt-names-the-kind-k18`, `delete-provisioning-k19` and
`open-kind-k20` genuinely need deployment, or merely assumed it as k6 did, is
re-derivable the same way and is left to those leaves.
