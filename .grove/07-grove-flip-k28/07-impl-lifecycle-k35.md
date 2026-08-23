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
