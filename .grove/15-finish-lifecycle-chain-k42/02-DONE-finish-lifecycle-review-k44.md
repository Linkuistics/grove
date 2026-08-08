# finish-lifecycle-review-k44

**Kind:** review-impl
**Reviews:** finish-lifecycle-k43
**Producer launch:** {"producer":"finish-lifecycle-k43","session":"finish-lifecycle-k43","generation":"k43","harness":"codex","model":"sol-xhigh"}

## Goal

Adversarially review `finish-lifecycle-k43` and record concrete findings for its integration step.

## Context

- Review `finish-lifecycle-k43` against the finish eligibility, confirmation
  boundary, universal lock, and scoped VCS commit contracts.
- Attack duplicate/hidden finish state, starvation, work appearing after
  launch, terminal-verb bypass, unrelated staged/working-copy consumption,
  unborn Git behavior, jj intermediate snapshots, and premature done signals.
- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `finish-lifecycle-integrate-k45` owns every fix and all
  post-fix verification.

## Done when

- Findings are recorded here with severity, exact tree/VCS reproducer, and the
  threatened contract, or an explicit no-finding result.
- The review cites inspected source, specifications, diff, and the producer's
  recorded post-launch insertion and unrelated-work evidence rather than
  re-running it.
- No production or test code is changed.

## Findings

Inspected: commit `679fd8ca` (`finish-lifecycle-k43`) in full;
`src/tree_lifecycle.rs` (`materialize_finish`, `finish_commit`),
`src/repo/finish_commit.rs`, `src/repo/migration_commit.rs`, `src/tree_read.rs`,
`src/tree_access.rs`, `src/tree_grow.rs`, `src/tree_id.rs`, `src/leaf.rs`,
`src/llm_cli.rs`, `src/loop_driver.rs`; `docs/specs/config-driven-sessions.md`
§§ "Finish leaf", "Fail-closed transaction and recovery", "Scoped Git and
Jujutsu commits"; and the producer's own evidence in `tests/finish_lifecycle.rs`,
`tests/lifecycle_cutover.rs`, `tests/driver_lease.rs`, `tests/session_kind_tree.rs`.
No production or test code was changed.

### F1 — Colocated-jj teardown restores a stale Git index that re-stages the deleted `.grove/` (high)

`src/repo/finish_commit.rs:59-92`. `commit_jj_finish` copies `.git/index`
before `jj commit`, then unconditionally renames the copy back — including on
success. In a colocated repo jj rewrites the index to match the new HEAD, so
restoring the *pre-teardown* snapshot re-introduces the `.grove/` blobs that the
finish commit just deleted.

Reproducer (colocated Git+jj worktree, jj 0.44, `.grove/` tracked in an earlier
commit, one unrelated `outside.txt` edit):

```text
after finish-commit, with the restore (current code):
  $ git status --short
  AD .grove/BRIEF.md
  AD .grove/FORMAT
   M outside.txt

with jj's own post-commit index (restore removed):
  $ git status --short
   M outside.txt
```

`AD` is "added in the index, deleted in the working tree": the user's next plain
`git commit` re-adds the whole task tree to the repository. Threatened contract:
"Scoped Git and Jujutsu commits" (preserve unrelated work — this preserves
*related* work it must not) and "Finish leaf" ("the successful deletion commit
removes the whole tree"), plus the spine rule that integrated history never
carries `.grove/`.

The producer's own regression
`colocated_jj_finish_commit_preserves_unrelated_work_and_the_git_index`
(`tests/finish_lifecycle.rs:293-296`, body in the shared helper at `:247-287`)
asserts `git ls-files --stage` is byte-identical across the teardown — i.e. it
asserts the broken state as the invariant. The assertion must change with the
fix.

The migration rationale for the dance (protect the user's staged non-`.grove`
work from jj's export) still holds, so a blanket delete of the restore is the
simplest but not the only correct shape; scoping the restore to non-`.grove`
index entries preserves both properties. Integration owns the choice.

### F2 — `finish-commit` bypasses the shared tree-entry validations, then deletes the tree unconditionally (medium)

`src/tree_lifecycle.rs:132` + `src/tree_access.rs:92-97`. `finish_commit` guards
with `write_for_lifecycle`, which by design runs **no** `require_grove_root`, no
`refuse_pending`, and no `tree_format::require_current` — unlike
`tree_access::read`/`write` (`src/tree_access.rs:41-61`) and
`write_for_promotion` (`:65-74`). The spec is explicit that this verb "reacquires
the exclusive working-tree lock, **rejects any pending transaction**, re-resolves
the same live finish handle" (§ "Finish leaf"); the rejection is absent.

Reproducer (tree state, then one command):

```text
.grove/FORMAT                      session-kinds-v1
.grove/BRIEF.md
.grove/MIGRATING-session-kinds/    ← witness holding the untouched originals
.grove/01-DONE-impl-done-k1.md
.grove/02-finish-finish-k2.md      ← live finish leaf

$ grove-llm finish-commit finish-k2      # succeeds
```

`MIGRATING-session-kinds` and `PROMOTING-*` have no leading position digits, so
`parse_current` classifies them as foreign and returns `Ok(None)`
(`src/tree_id.rs:270-287`); `select_unlocked` skips them, and
`fs::remove_dir_all(&grove_root)` (`src/tree_lifecycle.rs:152`) destroys the
witness — the only rollback material — and commits the destruction. Every other
tree reader and mutator refuses in this state, which is precisely the guarantee
"Its presence alone makes every other tree reader and mutator refuse" (§
"Fail-closed transaction and recovery").

The missing `require_current` is the same gap one grain wider: a `.grove/FORMAT`
carrying an unknown (newer or foreign) marker is specified to "stop without
mutation", but `finish-commit` will tear the tree down if a finish-shaped live
leaf is present.

Honest reachability note: I could not construct a *naturally occurring* driver
sequence that reaches either state — `tree_read::select` in `loop_driver.rs:137`
takes `tree_access::read`, so the driver itself refuses a witnessed tree before
it can materialize or launch a finish. The finding is a stated-contract gap with
a demonstrable code path and a destructive outcome, reachable from a
hand-constructed or externally perturbed tree, not a live driver bug. The guard
is two lines.

### F3 — Retrying `finish-commit` after a successful teardown reports a raw IO error (low)

`src/tree_lifecycle.rs:135-136` → `src/tree_read.rs:517`. With `.grove/` already
gone, `select_unlocked` fails inside `fs::read_dir` and the session sees
`reading /…/.grove: No such file or directory` rather than a finished-grove
diagnostic. This is a designed path, not an exotic one: the Done-when explicitly
contemplates a crash between `finish-commit` and `complete --done`, and the
resumable finish contract invites the session to retry. A `require_grove_root`
style message ("this grove is already finished") is the whole fix.

### F4 — The jj child inherits Git worktree anchoring that `migration_commit` deliberately withholds (low)

`src/repo/finish_commit.rs:139-145` applies `anchor_git_worktree_environment`
(sets `GIT_WORK_TREE`, clears `GIT_DIR`/`GIT_COMMON_DIR`) to **every** child,
including `jj commit`. `src/repo/migration_commit.rs:313-321` gates the same call
on `if binary == "git"`. No behavioural difference is observable on jj 0.44 — the
colocated and native jj tests pass either way — but the two VCS seams now
disagree about a decision one of them made on purpose, and the repository-
selection-environment concern that motivated `workspace_control`'s
"deliberately does not invoke `git` or `jj`" comment (`src/repo.rs:69-72`) is the
same concern.

### F5 — Plain-Git teardown mutates the index with no failure backup (low)

`src/repo/finish_commit.rs:45-52`. `commit_git_finish` runs `git add -A -- .grove`
and then `git commit --only`. If the commit fails — a hook, a signing failure, a
bad `user.email` — `.grove/` is already deleted from disk (`tree_lifecycle.rs:152`
runs first) *and* its deletion is staged, with no restore.
`commit_git_migration` protects exactly this window with `INDEX_BACKUP_NAME`
(`src/repo/migration_commit.rs:64-105`), so the asymmetry is a deliberate-looking
omission rather than an oversight the reader can dismiss. The tests cannot
observe it: `init_git` sets `core.hooksPath=/dev/null`
(`tests/finish_lifecycle.rs:58-63`).

### Verified-correct (no finding)

Recorded so integration does not re-litigate them:

- **Allocation / reuse / preemption / starvation.** `materialize_finish`
  re-selects under the exclusive lock before allocating
  (`src/tree_lifecycle.rs:94-99`), so an interleaved insertion wins and an
  existing finish is reused with no allocation.
  `empty_current_tree_allocates_and_launches_one_resumable_finish_leaf`
  (`tests/lifecycle_cutover.rs:1054-1151`) drives allocate → preempt by
  `resumed-k3` → reuse of the same `finish-k2` handle, asserting exactly one
  `finish-finish` entry at each step. Preemption is ordering, not deletion:
  `select_unlocked` prefers any non-finish live leaf tree-wide
  (`src/tree_read.rs:90-97`), so finish cannot starve later work and later work
  cannot strand finish.
- **Duplicate / hidden finish state.** Two live finish leaves are rejected as
  malformed (`src/tree_read.rs:76-89`), covered at
  `tests/session_kind_tree.rs:123-133`.
- **Terminal-verb bypass.** `finish` is refused by `leaf-add`
  (`tree_grow.rs:47,58`), `leaf-insert` (`:366,377`), `leaf-decompose`
  (`tree_lifecycle.rs:442,446`), `leaf-retire` (`:498`), `leaf-prune`
  (`:614,757`), `leaf-add-chain` via `review_steps_or_refuse`
  (`leaf.rs:232-234`), and `leaf-promote-chain` (`tree_promotion.rs:630`);
  swept at `tests/session_kind_tree.rs:238-285`. Non-finish `leaf-insert` may
  still target the finish leaf and shift it — `refuse_finish_kind` gates the new
  leaf's kind, not the target — which is the specified asymmetry.
- **Work appearing after launch.** `finish_commit` re-selects under the lock and
  bails naming the new work before touching anything;
  `finish_commit_refuses_byte_identically_when_ordinary_work_appeared`
  (`tests/finish_lifecycle.rs:226-245`) asserts a byte-identical tree snapshot
  and an unchanged `HEAD`. `validate_finish_commit` is read-only and runs after
  those checks, so the refusal path writes nothing.
- **Unrelated staged / working-copy work.** Plain Git: staged `staged.txt` stays
  staged and out of the commit, unstaged work untouched
  (`tests/finish_lifecycle.rs:194-224`). Native jj: `outside.txt` lands in the
  successor working-copy commit, not the teardown commit
  (`:247-292`). (The colocated variant of this test carries F1.)
- **Unborn Git.** `validate_finish_commit` refuses before deletion when `.grove/`
  has no tracked state in `HEAD` (`src/repo/finish_commit.rs:7-29`), asserted
  tree-identical at `tests/finish_lifecycle.rs:299-320`.
- **Lock vs. deletion.** The tree lock is an `flock` on the *worktree directory*
  descriptor (`src/tree_access.rs:115-145`), and the loop's signal channel lives
  in the VCS administration area, so `remove_dir_all(.grove)` destroys neither.
  `configured_finish_target_commits_teardown_then_stops_the_loop_cleanly`
  (`tests/finish_lifecycle.rs:322-359`) proves the whole bare-`grove` path:
  ordinary mandate → `finish-commit` → `complete --done` → "grove finished".
- **No commit for allocation; message names the handle.** `materialize_finish`
  is working-tree only, and the only message is
  `"{finish_handle}: remove completed grove task tree"`
  (`src/repo/finish_commit.rs:34`).
- **Human confirmation not inferred.** `finish_commit` performs no
  confirmation-shaped check and its doc comment states the boundary
  (`src/tree_lifecycle.rs:129-131`), matching "not a security boundary or
  substitute for the HITL contract".
- **Epoch admission.** `finish-commit` flows through
  `driver_lease::admit_ambient_session` like every other verb
  (`src/llm_cli.rs:429`), and `tests/driver_lease.rs:1010-1046` exercises
  teardown followed by re-initialization with handle reuse.

### Coverage and evidence gaps (not defects)

- The producer recorded no `cargo fmt --check` / `cargo test --locked` result:
  the commit message is a bare subject line and the tree carries no evidence
  artifact. The Done-when required both. Integration re-runs them anyway, so
  this is a record gap, not an unverified claim to re-derive here.
- No test covers the handle-mismatch branch (`src/tree_lifecycle.rs:143-148`) —
  a live finish leaf whose handle differs from the argument. Every other
  `finish_commit` refusal has one.
- "Intermediate snapshots" is covered only in the direction where the finish leaf
  was never in a prior commit (`seed_jj_terminal_grove` writes it *after* the
  fixture commit). The reverse — jj having already snapshotted the live finish
  leaf into an earlier commit before teardown — is untested. Behaviour is
  benign by inspection (the deletion is simply recorded), so this is a coverage
  note only.

## Notes

The reviewer produces findings only; `finish-lifecycle-integrate-k45` owns
fixes. F1 changes an existing assertion, so integration must rewrite
`colocated_jj_finish_commit_preserves_unrelated_work_and_the_git_index` rather
than merely make it pass.

F1's reproducer was established in a throwaway scratch repository outside this
working tree, not by running the project's tests, building it, or editing any
production or test code.
