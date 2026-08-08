# session-kind-migration-review-k28

**Kind:** review-impl
**Reviews:** session-kind-migration-k27
**Producer launch:** {"producer":"session-kind-migration-k27","session":"migration-transition-k97","generation":"k97","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `session-kind-migration-k27` and record concrete findings for its integration step.

## Context

- Review `session-kind-migration-k27` against the accepted-input table,
  current-format witness, transaction/recovery protocol, and scoped VCS commit
  contract in the spec.
- Attack interruption between every filesystem/index/commit phase, partial
  root misclassification, key or relationship loss, vendor-pair ambiguity,
  collision handling, witness visibility, and unrelated-work preservation.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `session-kind-migration-integrate-k29` owns every fix
  and all post-fix verification.

## Done when

- Findings are recorded here with severity, a deterministic reproducer or
  trace, and the threatened contract, or an explicit no-finding result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded Git, jj, and recovery evidence rather than re-running it.
- No production or test code is changed.

## Findings

Inspected: commits `nmponlxv`/`e43fa910` (k93), `usltolly`/`17e9846d` (k94),
`vouvqool`/`f7838c8f` (k95), `zytwyunm`/`bf645bbd` (k96), `klxyqutw`/`8e806737`
(k97) — the combined diff `slvwpylw..klxyqutw` (17 files, +3872/−54) — and
post-change source for `src/tree_migrate.rs`, `src/tree_migration_transaction.rs`,
`src/tree_lifecycle.rs`, `src/tree_access.rs`, `src/tree_format.rs`,
`src/tree_id.rs`, `src/leaf.rs`, `src/repo/migration_commit.rs`,
`src/tree_grow.rs`, `src/launch.rs`, `src/cli.rs`, `src/llm_cli.rs`,
`tests/migration_commit.rs`, and `tests/migration_transition.rs`;
`docs/specs/config-driven-sessions.md` sections "Session kinds live in
filenames" (216-274), "Fresh tree" (514-529), "Legacy migration" (581-692), and
"Module interfaces" (742-783); `docs/adr/promotion-transactions-fail-closed.md`.
**The producer recorded no verification evidence** (F7) — all five commit
messages are bare single-line subjects — so no conclusion below rests on a test
result. No code was run and no production or test file was changed.

### F1 — high — `land()` is not idempotent for a planned file whose source path equals its destination, so migrating any legacy **v2** tree with a node brief cannot resume across the commit window; it rolls back, and if the commit already landed the working tree is reverted underneath it

`src/tree_migrate.rs:502-516` plans a non-root legacy-v2 node `BRIEF.md` with
`to_rel` **equal to** `from_rel`:

```rust
files.push(PlannedFile {
    from_rel: from_rel.clone(),
    to_rel: from_rel,          // src/tree_migrate.rs:512-513
    body: body.into_bytes(),
});
```

This is deliberate and pinned by the planner's own test — `01-feature-k10/BRIEF.md`
→ `01-feature-k10/BRIEF.md` (`src/tree_migrate.rs:1393-1397`) — and
`validate_current_plan` explicitly tolerates it (`src/tree_migrate.rs:390-392`).
It is also the *normal* shape: v2 directory names are already current, so **every
non-root node brief in every v2 tree** is a from==to entry. This grove's own
`.grove/` has seven of them.

`land()` then reads progress per entry in two passes
(`src/tree_migration_transaction.rs:291-361`):

```rust
if source.exists() {
    verify_hash(&source, &file.source_sha256, "migration source")?;   // :301
    anyhow::ensure!(!moved.exists(),                                  // :302-306
        "migration source exists both in the tree and witness: {}", …);
```

For a from==to entry, once pass 2 has renamed `staged/<rel>` → `<rel>`
(`:335`), the *source* path exists again. On the next invocation, pass 1 sees
`source.exists() == true` **and** `moved/<rel>` still present, so it bails —
either at `verify_hash` (when the brief carried a `**Kind:**` line, so
`source_sha256 != destination_sha256`) or at the `ensure!` (when the brief
carried none and the bodies are byte-identical). Recovery never reaches the
already-landed state it is supposed to infer.

`finish_or_rollback` then checks `commit_completed || COMMITTED` exists
(`:206`); on a fresh process both are false, so it **rolls back** (`:213`).

Failure scenario, fully deterministic:

1. Bare `grove` migrates a v2 tree containing `01-feature-k10/BRIEF.md`.
2. `land()` completes; `FORMAT` is written (`:275`); `commit()` runs (`:278`).
3. The process is killed after `git commit`/`jj commit` returns but before
   `write_new(COMMITTED)` (`:281`) — a window that spans an entire subprocess.
4. Next bare `grove`: witness has `READY`, no `COMMITTED` → `finish_or_rollback`
   → `land` → bail → `rollback` → destinations moved back to `staged/`, sources
   restored from `moved/`, `FORMAT` removed, witness deleted.
5. The working tree is legacy again **while HEAD contains the migrated
   `.grove/`**. In jj the revert is snapshotted into the working-copy commit as a
   mass deletion of the migrated tree; in Git the whole task tree reads as
   deleted-and-re-added. The next invocation migrates again and adds a second
   migration commit on top.

Contract threatened, three ways:
- `docs/specs/config-driven-sessions.md:650-652` — "Landing and recovery infer
  progress from each entry's source, staged, and final location".
- `:659-661` — "A crash after the commit but before witness removal is recovered
  by verifying the committed final tree and removing the now-redundant witness."
  That path exists (`src/tree_migration_transaction.rs:126-134`) but is gated on
  `COMMITTED`, which is written *after* the commit; the gap belongs to
  `finish_or_rollback`, and for from==to entries it resolves to rollback.
- `session-kind-transaction-k94` Done-when: "Deterministic tests exercise every
  interruption boundary, successful retry, rollback…".

Why the suite does not catch it: both boundary tests build their fixture from
`old_nnn_tree()` (`src/tree_migration_transaction.rs:949`, `:963`, `:1016`,
`:1035`), where the root brief is excluded from the plan
(`src/tree_migrate.rs:504-505`) and every other entry genuinely moves. The third
fixture, `legacy_tree()` (`:636-647`), is flat. **No test drives `run()` over a
legacy-v2 node at all**, so the one shape that carries from==to entries is
untested end to end.

Suggested direction: make pass 1 recognise the landed state before treating the
path as an unmoved source — e.g. skip an entry whose `moved/<rel>` already
verifies, or key progress on `moved`/`staged` presence rather than on
`source.exists()`, which is ambiguous exactly when `from_rel == to_rel`. Add a
legacy-v2-with-node fixture to both boundary tests; it is the shape real trees
have.

### F2 — high — the partial-root-scaffold classifier fires on any legacy tree whose root `BRIEF.md` is still the untouched `root_init` stub, permanently refusing migration with an "ambiguous partial root scaffold" diagnostic

`recover_partial_root_init_unlocked` (`src/tree_lifecycle.rs:142-232`) runs
before the migration planner (`src/tree_migration_transaction.rs:149`), which is
the correct order per `docs/specs/config-driven-sessions.md:521-525`. Its
discriminator is not:

```rust
let has_exact_scaffold_file = expected
    .iter()
    .any(|(path, body)| path.is_file() && fs::read(path).is_ok_and(|found| found == *body));
if has_exact_scaffold_file {
    bail!("ambiguous partial root scaffold at {} …", …);   // :196-209
}
return Ok(false);                                          // :211
```

`any` over the three expected paths means **one** exact match is enough. The
root `BRIEF.md` is one of them, and its expected body is the bare section
scaffold `root_brief_body` writes (`src/tree_lifecycle.rs:760-764`) — headers
only, no prose, filled in later by the bootstrap session. A legacy tree whose
first requirements session never edited the root brief therefore matches
byte-for-byte, its real task leaves land in `unexpected`, and migration bails.

Reproducer (no timing, no interruption):

```sh
mkdir demo && cd demo && git init -q .
mkdir .grove
printf '# demo — brief\n\n## Goal\n\n## Done when\n\n## Decomposition\n\n## Pointers\n\n## Notes\n' \
  > .grove/BRIEF.md
printf '# plan-k1\n\n**Kind:** requirements\n\n## Goal\n\n\n## Context\n\n## Done when\n\n## Notes\n' \
  > .grove/01-plan-k1.md
# transition_to_current →
#   ambiguous partial root scaffold at …/.grove: exact fresh-tree content is
#   mixed with unexpected entries: …/.grove/01-plan-k1.md
```

This is a well-formed legacy v2 tree — exactly the input
`docs/specs/config-driven-sessions.md:590-592` says migration accepts — and it
can never migrate. Because bare `grove` is the only lifecycle action and the
diagnostic never names the root brief as the blocker, there is no
diagnostic-directed recovery; the operator has to guess that editing or deleting
`.grove/BRIEF.md` unblocks it.

Two further instances of the same discriminator:

- **Custom root-init slug.** `grove-llm root-init <slug>` still accepts an
  arbitrary slug (`src/llm_cli.rs:428-430` → `tree_lifecycle::root_init`), but
  recovery hardcodes `"plan"` for both the expected leaf name and its body
  (`src/tree_lifecycle.rs:153-164`). `root_init` writes `FORMAT` last
  (`:135`), so an interruption after the leaf leaves `BRIEF.md` matching
  exactly and `01-requirements-<slug>-k1.md` unexpected → the same permanent
  bail. Either recovery must accept any valid slug, or `root-init`'s slug
  argument must go with the rest of the removed surfaces.
- **Brief-only `.grove/` with a custom brief.** `unexpected` is empty, so the
  ambiguity branch is skipped and control reaches the exactness loop
  (`:214-216`), which bails "partial root scaffold file … differs from the
  deterministic fresh-tree content; refusing to overwrite it". The tree is not a
  scaffold at all; the planner's own "neither an exact partial fresh-tree
  scaffold nor a recognizable legacy tree" message
  (`src/tree_migration_transaction.rs:155-159`) is the one that should be
  reached. Cosmetic relative to the above, same root cause.

Contract threatened: `partial-root-recovery-k95` Done-when — "Any differing
body, extra task-shaped entry, collision, or foreign partial scaffold is refused
without overwrite and **routes clearly to migration** or an ambiguity
diagnostic". Here a legacy tree routes to ambiguity instead of migration.

Suggested direction: require the *leaf* to match (or every present expected file
to match) before claiming scaffold identity, and treat a task-shaped unexpected
entry as positive evidence of a legacy tree rather than of ambiguity. A root
brief alone cannot distinguish the two states — it is format-neutral, which
`detect()` already recognises (`src/tree_migrate.rs:702-703`).

### F3 — medium — `write_for_lifecycle` does not refuse a pending `MIGRATING-session-kinds` witness, so the surviving `grove migrate` and adoption-migrate paths can mutate a tree mid-transaction

`src/tree_access.rs:92-97` acquires the exclusive lock and returns, with no
`refuse_pending_migration` — unlike `read` (`:41-50`), `write` (`:52-61`) and
`write_for_promotion` (`:65-74`). Two of its three callers are safe by
construction: `transition_to_current` (`src/tree_lifecycle.rs:54`) owns witness
recovery, and `root_init` (`:104`) bails when `.grove/` already exists, which a
live witness implies. The third is not:

- `tree_migrate::migrate` (`src/tree_migrate.rs:133-136`) → `migrate_unlocked`
  (`:138-156`), which classifies via `detect()` and renames through `execute`
  (`:1131-1152`) with no witness check anywhere;
- `tree_migrate::migrate_on_adoption` (`:168-175`), same, plus a commit.

Both are live production entry points today: `src/cli.rs:170` (`grove migrate`)
and `src/launch.rs:37-38` (`grove do`'s adoption migrate). The witness directory
is invisible to `detect()` (`src/tree_migrate.rs:697-721` — not a node, not
`done`, no `NNN-` prefix), so a half-landed tree is classified and renamed as if
it were an ordinary legacy tree, destroying the transaction's source/staged/final
invariant.

Not reachable *yet*: nothing in production calls `transition_to_current`, so no
witness can exist. The window opens the moment `lifecycle-cutover-k39` wires it
in, and closes only at `legacy-command-surface-removal-k77`
(`.grove/16-legacy-launch-removal-k46/`) — two chains later. Recording it here
because the ordering makes it a real window rather than a hypothetical.

Contract threatened: `docs/adr/promotion-transactions-fail-closed.md:19-22` —
"the durable witness that every subsequent task-tree reader and mutator
refuses"; `docs/specs/config-driven-sessions.md:646-648` — "Its presence alone
makes every other tree reader and mutator refuse";
`session-kind-transaction-k94` Done-when — "Other readers and mutators refuse
while the witness exists".

Suggested direction: call `refuse_pending_migration` at the top of
`migrate_unlocked` (cheapest, and correct even after the cutover), or give
`write_for_lifecycle` a witness-policy parameter so the exemption is explicit
per caller rather than blanket.

### F4 — low — the witness path is a hand-written string literal in the commit scoping, decoupled from the constant that creates it

`src/repo/migration_commit.rs:7-8`:

```rust
const GIT_PATHS: [&str; 2] = [".grove", ":(exclude).grove/MIGRATING-session-kinds"];
const JJ_FILESET: &str = "root:.grove ~ root:.grove/MIGRATING-session-kinds";
```

The witness name itself is `tree_access::MIGRATION_TRANSACTION`
(`src/tree_access.rs:13`), which is what `run_observed_unlocked` creates and
`refuse_pending_migration` refuses. Renaming the constant compiles cleanly and
silently commits the live witness into the migration commit — the one thing
`docs/specs/config-driven-sessions.md:658-659` and `:673-676` require excluded.
Deriving both strings from the constant (via `format!`/`concat!`) removes the
coupling; a test asserting the literal contains
`tree_access::MIGRATION_TRANSACTION` would also do.

### F5 — low — `write_current_last` is symlink-unsafe, and its only guard sits in a classifier the transaction-recovery path skips

`src/tree_format.rs:35-42` uses `fs::write(&temporary_path, …)` (`:38`), which
follows a symlink at `.grove/.FORMAT.tmp` and truncates its target. The producer
knew and guarded it — but in the caller, not the callee
(`src/tree_lifecycle.rs:175-182`):

```rust
// `.FORMAT.tmp` is writer-owned transaction state, never legacy tree
// content. Validate it before deciding that the surrounding entries belong
// to a legacy tree; otherwise `write_current_last` could follow and truncate
// a near-match symlink during migration.
```

That validation runs only inside `recover_partial_root_init_unlocked`, reached
from `run_observed_unlocked:149` — i.e. only on the **first** planning pass.
The recovery path (`src/tree_migration_transaction.rs:135-142` →
`finish_transaction` → `write_current_last` at `:275`) never runs it, so a
`.FORMAT.tmp` symlink planted between a crash and the retry is followed. Narrow
and adversarial, but the fix is cheap and belongs in `write_current_last`:
`OpenOptions::new().write(true).create(true).truncate(true)` plus an
`O_NOFOLLOW`, or an unconditional `remove_file` of the temporary path first.
`write_new` (`src/tree_migration_transaction.rs:580-586`) already uses
`create_new(true)` and is not exposed this way; `write_current_last` is the
outlier.

### F6 — low — the body-marker rewrite matches only at column 0 and ignores fences, so an indented marker survives into a current tree and a fenced example is deleted

`src/tree_migrate.rs:647-672`. `content.strip_prefix("**Kind:**")` (`:652`) and
`content.starts_with("**Harness:**") || content.starts_with("**Producer launch:**")`
(`:664`) operate on the line with only its trailing newline trimmed. Two
consequences:

- `  **Kind:** design` (indented, e.g. inside a list item) is neither parsed nor
  removed. The kind silently degrades to `impl` (`:526`) *and* the marker
  survives into the migrated body — contradicting
  `docs/specs/config-driven-sessions.md:257-259` ("Task bodies no longer carry
  `**Kind:**`, `**Harness:**`, or `**Producer launch:**`") and, in this grove,
  the removal of read-side kind degradation that makes filenames authoritative.
- A `**Kind:** …` line inside a fenced code block — plausible in a task or brief
  that documents the old format, which several leaves in this grove do — is
  deleted, contradicting "while preserving all other bytes" (`:624-625`).

Both are narrow. The first is the one worth closing: either trim leading
whitespace before matching (and then remove the whole line), or refuse a body
whose non-column-0 text contains a marker, so a mis-formatted kind fails loudly
rather than degrading.

### F7 — low — the producer recorded no verification evidence, so the two `cargo` Done-when bullets are unverified and this review could not rest on them

All five commits are single-line subjects with empty bodies
(`nmponlxv`…`klxyqutw`), and none of the five leaf files carries a
`## Verification evidence` section. `session-kind-migration-k27`'s brief and
`migration-transition-k97` both claim "`cargo fmt --check` and `cargo test
--locked` pass"; nothing records that either ran.

The precedent exists in this grove — `session-kind-tree-k23` records exit codes
and a test count, and `tree-access-lock-review-k55` raised the identical gap as
its F3. It recurs. For this producer it is load-bearing twice over: the review
is inspection-only by its own Context, and F1 is precisely a case where the
suite passes while the property does not hold, so "the suite is green" would not
have been sufficient evidence even had it been recorded.

Action for `session-kind-migration-integrate-k29`: run both, record the result,
and treat those bullets as open until then.

### F8 — informational — restoring the pre-`jj` Git index in a colocated repo leaves the index inconsistent with the HEAD `jj` just advanced

`src/repo/migration_commit.rs:193-213` copies the colocated Git index aside,
runs `jj commit`, then renames the backup back. `tests/migration_commit.rs:421-457`
asserts `git ls-files --stage` is byte-identical before and after (`:453-456`),
which satisfies `migration-scoped-commit-k96`'s "do not mutate the colocated Git
index" exactly.

The side effect is that `jj`'s auto-export has moved Git's `HEAD` to the
migration commit while the index still describes the pre-migration tree, so
`git status` reports the just-committed migration as staged-in-reverse
(legacy paths added, migrated paths deleted) until the next `jj` snapshot
re-exports. No spec clause covers the index — `docs/specs/config-driven-sessions.md:687-689`
says only that Grove commits a `.grove/` fileset and leaves unrelated
working-copy changes in the successor — and under the repo's own jj-first policy
Git is read-only anyway, so this is transient noise rather than a defect.

Recording it so `session-kind-migration-integrate-k29` can decide whether the
index-preservation behaviour deserves a spec sentence (it is a real, tested
behaviour with no durable record) rather than rediscovering it later.

### Verified without finding

- **The two-pass ordering in `land()` is right, and prevents cross-entry
  aliasing.** All sources move into `moved/` (`src/tree_migration_transaction.rs:297-319`)
  before any destination lands from `staged/` (`:320-350`), so a destination that
  collides with a *different* entry's not-yet-moved source cannot occur. F1 is
  specifically the same-entry, same-path case, not an ordering defect.
- **Rollback is correct and byte-exact, including after F1's spurious bail.**
  `rollback` (`:409-498`) removes `FORMAT` only after `require_current`
  (`:416-425`), returns destinations to `staged/` in reverse order, restores each
  source from `moved/` or from the untouched `rollback/` copy (`:461-486`), and
  re-verifies every source hash before removing the witness (`:488-494`).
  Rollback failure leaves the witness and blocks the tree (`:213-220`), which
  `rollback_failure_leaves_the_tree_blocked_by_the_witness` (`:1264-1293`)
  proves end to end via a real `tree_access::read`.
- **The manifest is validated as a plan, not trusted.** `validate_manifest`
  (`:510-539`) pins the version, rejects duplicate sources and destinations,
  rejects any non-`Normal` path component (`:541-553` — no `..` escape from the
  grove root), and verifies every `rollback/` copy against its recorded hash
  before the plan is used.
- **Commit recovery by identity is sound in both VCSs.**
  `git_migration_already_committed` (`src/repo/migration_commit.rs:102-131`)
  requires *both* an exact HEAD-subject match and an empty
  `git diff --quiet HEAD -- .grove ':(exclude)…'`; the jj form (`:216-260`)
  requires an exact `@-` description and an empty scoped `jj diff -r @`. A
  re-invocation therefore reuses the commit instead of duplicating it
  (`tests/migration_commit.rs:272-299`, `:386-419`), which is what makes
  `finish_transaction`'s "commit then record" order safe for every entry that is
  not F1's from==to case.
- **Scoped commits preserve unrelated work, including the two hard cases the
  spec calls out.** Pre-existing staged and working-tree changes outside
  `.grove/` survive a plain-Git migration (`tests/migration_commit.rs:136-199`),
  tracked deletions under an absent directory are recorded (`:170-177`
  asserts `.grove/legacy.md` in the commit), and the unborn-`HEAD` case commits
  correctly while leaving an unrelated staged path staged (`:201-234`). Both
  facts `docs/specs/config-driven-sessions.md:678-681` promises tests for are
  tested.
- **The witness blocks every current-format reader and mutator, before the
  format check.** `read`/`write` call `refuse_pending` → `refuse_pending_migration`
  ahead of `require_current` (`src/tree_access.rs:41-61`, `:188-212`), the check
  uses `symlink_metadata` so a non-directory witness also refuses
  (`:203-208`, test `:272-287`), and the diagnostic names the witness and says
  "rerun bare `grove`" as `:646-648` requires. `write_for_promotion` refuses it
  too (`:65-74`, test `:300-314`). F3 is the one path that escapes.
- **The legacy and current filename grammars agree.** `parse_legacy_v2_leaf_parts`
  (`src/tree_migrate.rs:608-632`) re-implements the shape that
  `tree_id::parse_parts` (`src/tree_id.rs:294-339`) parses, using
  `rsplit_once("-k")` where the current parser takes the trailing digit run.
  I probed the divergence candidates — `01-foo-k1x`, `01-foo-k1-k2`,
  `01-foo-k1k2`, `01-k5`, `01-foo-k<overflow>`, reserved-word slugs — and both
  classify identically in every case, so no legacy leaf is silently left behind
  as "foreign" only to be rejected as `malformed Grove leaf` by
  `parse_current` (`src/tree_id.rs:270-287`) after migration. The duplication is
  a maintenance hazard rather than a present defect: the two must agree for the
  migration to produce a readable tree, and nothing pins that. A single test
  asserting agreement over a shared name corpus would make it structural.
- **Kind mapping matches the spec table exactly.** `parse_legacy_kind`
  (`src/tree_migrate.rs:674-682`) matches `"research"` *before* falling through
  to `Kind::parse_read`, which would otherwise collapse it to `ResearchA`
  (`src/leaf.rs:282-286`) and destroy vendor-pair detection; `work` /
  `review-work` / `integrate-review-work` map to the `impl` spellings; `finish`
  is rejected as unknown, keeping the driver-reserved kind out of migrated trees
  (`docs/specs/config-driven-sessions.md:264-266`). Empty and repeated markers
  bail with the source path before any mutation
  (`src/tree_migrate.rs:653-659`).
- **Vendor-pair classification matches `:606-612` clause by clause.** The
  candidate scan runs over position-sorted leaves (`src/tree_migrate.rs:531`,
  `:544-553`); `has_brief` disqualifies (`:554`); a child node or a fourth
  task-shaped child makes a candidate ambiguous rather than a pair (`:555-571`);
  terminal outcomes are ignored throughout; foreign files are skipped at
  `:517-519`. A standalone `research` maps to `research-a` (`:577`,
  `resolve_standalone_kind` at `:640-645`).
- **Planning is pure and pre-mutation.** `plan_current`
  (`src/tree_migrate.rs:290-311`) reads bodies but writes nothing;
  `require_unambiguous_legacy_layout` (`:313-335`) rejects mixed-format trees
  with all offending paths before anything is staged; `validate_current_plan`
  (`:379-409`) catches duplicate destinations and on-disk collisions, correctly
  exempting from==to entries from the collision probe. `prepare` failure
  discards the witness and leaves sources untouched (`:162-176`, test `:1144-1169`).
- **`FORMAT` is written last and atomically, and an unknown witness stops
  everything.** `write_current_last` renames a same-directory temporary
  (`src/tree_format.rs:35-42`), `finish_transaction` writes it only after
  `land` verifies the whole final tree (`src/tree_migration_transaction.rs:271-277`),
  and an unknown value refuses without mutation in all three repository kinds
  (`tests/migration_transition.rs:171-193`, `:210-223`). A dangling `FORMAT`
  symlink refuses without replacement (`src/tree_migration_transaction.rs:885-907`).
- **Relationships and keys survive.** `rewrite_legacy_metadata` removes only the
  three launch-policy markers and preserves `**Reviews:**` / `**Integrates:**`
  and all other bytes (`src/tree_migrate.rs:647-672`); v1-flat and v2 keys are
  preserved verbatim and only the keyless `NNN` format allocates fresh keys
  (`:25-28`, `:744-799`); the unkeyed root brief stays at `.grove/BRIEF.md` in
  every path (`:504-505`, `:702-703`, `:754-755`).
- **The deferral of the driver cutover is explicit, not an oversight.**
  `src/tree_migrate.rs:30-35` states that the old adoption/human entry points
  remain compatibility callers until `lifecycle-cutover-k39`, which is why
  `migrate_on_adoption` still has a production caller while
  `transition_to_current` has none. `migration-transition-k97`'s "existing
  standalone migrate/adoption callers are reconciled" is therefore satisfied at
  the documentation level only — acceptable given the leaf's own "do not
  implement the bare `grove` lifecycle cutover here", but F3 is the part of that
  deferral that carries risk.

## Notes

The reviewer produces findings only; `session-kind-migration-integrate-k29`
owns fixes.

F1 is the one that must not ship. It is not a rare race: from==to entries are
the normal shape of every legacy v2 tree — the format most real groves are in,
including this one — and the unresumable window spans the VCS commit
subprocess, the longest step in the transaction. Its worst outcome (working tree
rolled back while the migration commit persists) is exactly the "no partial
state is runnable" property `docs/adr/promotion-transactions-fail-closed.md`
exists to guarantee. Fix the progress inference *and* add the missing
legacy-v2-with-node fixture to both boundary tests; either alone leaves the
other half of the gap.

F2 and F1 share a theme worth naming for the integrate leaf: both are cases
where a state that is *identical on disk* to two different situations is
resolved by an ambiguous signal — `source.exists()` in F1, a single matching
expected file in F2. Both fixes are the same shape: pick a signal that
distinguishes, rather than one that merely usually correlates.

F3 needs no change under this leaf's scope if `lifecycle-cutover-k39` removes
the adoption caller and `legacy-command-surface-removal-k77` removes
`grove migrate` — but they land two chains apart, so the cheap guard belongs
here. F7 gates the `cargo` bullets. F4, F5, F6 and F8 are small and local.
