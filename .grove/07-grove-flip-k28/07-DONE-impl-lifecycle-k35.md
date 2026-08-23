# lifecycle-k35

## Goal

Move the lifecycle verbs that *surround* the tree onto the library's write path:
`root-init`, `materialize-finish`, `transition-to-current` and `finish-commit`.
None of these is tree algebra — that is why they come after the algebra is
flipped and why they survive the sweep — but each of them writes entries, and
after this leaf they write them through the library.

## Context

- `src/tree_lifecycle.rs` — `root_init`, `recover_partial_root_init_unlocked`,
  `materialize_finish`, `finish_commit`, `transition_to_current` and
  `transition_driver_to_current`.
- `src/tree_format.rs` — `require_current` and `write_current_last`, and the
  ordering rule they encode: `.grove/FORMAT` is written **last** and printed as
  the third path, so a partially-created root is recognisable as partial.
- `src/tree_access.rs::write_for_lifecycle`, the lifecycle guard.
- `src/finish_transaction.rs` and `src/repo/migration_commit.rs`, the two callers
  outside the tree modules.
- `docs/adr/task-tree-transactions-fail-closed.md`.
- Suites: `root_init`, `finish_lifecycle`, `lifecycle_cutover`, `complete`,
  `loop_driver`, `driver_lease`.

## Done when

- `root-init` scaffolds through the library: `.grove/`, the root `BRIEF.md` as
  the root's distinguished child, the first `requirements` leaf via `append`, and
  `.grove/FORMAT` written last and printed third — unchanged, because that
  ordering is what makes a partial root recoverable.
- `materialize-finish` appends the driver-owned `finish` sentinel through the
  library.
- `transition-to-current` and `finish-commit` work against the library's guards,
  and the pending-transaction refusals still fail closed.
- `recover_partial_root_init` still recovers, and the case it recovers from still
  exists — check this rather than assume it, because the library creating the
  root changes what *partial* can look like.
- The whole suite passes; changed tests recorded in the node brief.

## Notes

**The root's own creation is the awkward case, and it is worth thinking about
before writing.** `fs::read` and `fs::write` lock the directory *containing* the
tree root, which is deliberate and general — the containing directory exists
before the root is created and persists after it is deleted. That is exactly what
`root-init` needs, so this should work; the check is that grove's existing
refusal (`.grove/` already exists) still happens where it does now and does not
become a library error with different words.

**`FORMAT` is Foreign to the grammar and load-bearing to grove.** `domain-k29`
classifies it as Foreign so a healthy tree reads; this leaf is where the *other*
half lives — `require_current` still has to refuse a tree without it, and that is
grove's check and not the library's. Do not be tempted to make `FORMAT` Reserved
so the library enforces it; Reserved halts, and a healthy tree would stop
working.

**The finish sentinel is driver-reserved and the grow verbs refuse to create
one.** `materialize_finish` is the exception, and it stays one — the library will
happily `append` anything grove's `Parts` can express, so whatever prevents an
ordinary `leaf-add` from producing a `finish` kind has to keep doing it on
grove's side.

**Two callers live outside the tree modules** — `finish_transaction` and
`repo/migration_commit` — and both reach in for lifecycle concerns rather than
algebra. Expect them to keep working with narrower imports rather than to change.

## Decisions (running log)

**The scaffold is two phases with the guard released between them, and it could
not be one.** `ordinal_fs_tree::fs` locks the directory *containing* the root, so
the lock spans the root's creation — but it still has to reach the root to
snapshot it, so it cannot create one, and a `BRIEF.md` arrives only through
`promote`. Both are grove's, under grove's guard; the first leaf is the
library's, under its. Nesting them deadlocks (one directory, two open file
descriptions), so phase one releases before phase two takes. This is what forced
`transition-to-current` into the same shape: it called `root_init_unlocked` while
holding the lifecycle guard, so classification and scaffolding are now separated
by the same release.

**The window that release opens is the one `FORMAT` already made legible.** A
root without its format witness is *partial*: every ordinary verb refuses it and
bare `grove` completes it through `recover_partial_root_init_unlocked`. So the
tree exposed mid-scaffold is exactly the tree recovery existed for — previously
reachable only by a process death. Recorded as a behaviour change rather than
hidden: a concurrent `pick` in that window is told to migrate where it used to
block. Held by `phase_one_leaves_the_partial_root_recovery_completes`.

**`complete_scaffold` is idempotent, and that is load-bearing rather than
defensive.** Another process can complete the partial root first, so the append
happens only when the snapshot holds no positioned entry. Appending
unconditionally would give the tree two first leaves and *no refusal* — the
second lands at ordinal 2, key 2, quite legally.
`a_scaffold_completed_by_a_recovery_leaves_the_original_nothing_to_add` sequences
the exact race.

**`recover_partial_root_init_unlocked` does not go through the library, and that
is not an omission.** It runs inside the session-kind migration transaction,
which holds grove's exclusive guard — reaching for the library's would be the
nesting above — and it allocates nothing: ordinal, key, slug and bytes are all
fixed, and every file is byte-compared before anything is written. It completes a
scaffold; it does not grow a tree. What it *did* drop is `tree_id`: it composes
and recognises the scaffold name through `TaskName` now, which is what orphaned
that module's last lifecycle caller.

**`task_tree::write_scaffold` is a third acquisition and the only writer exempt
from `require_current`.** `FORMAT` is written last precisely so a partial root is
recognisable, so the writer that installs it cannot demand it first. It also
takes no waiting diagnostic, for `reopen_write`'s reason — the command announced
its wait when it took grove's guard.

**`finish-commit`'s guard changed what it refuses, deliberately.** On the
library's guard a `FINISHING-*` / `PREPARING-FINISH-*` name halts it at the guard
in the domain's own words, rather than reaching `preflight_root`'s *reserved
finish transaction path*. One condition, one wording (clause 3). The preflight
check stays as defence against a writer that ignored the lock — it re-reads the
root through its own `O_NOFOLLOW` descriptor — and the verb still classifies
`.grove` itself first, because a symlink to a directory elsewhere is a root the
library follows and a no-follow teardown must refuse.

**No formalism was reached for and none is owed.** The one modelled fact this
leaf leans on — that `append` composes `max + 1` from the snapshot, which is what
makes the idempotency check sufficient — is entry 003's, already cited by
`growing-k33`. Nothing new was modelled and no model was consulted beyond its
recorded misses.
