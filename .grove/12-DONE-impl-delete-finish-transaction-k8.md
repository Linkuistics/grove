# delete-finish-transaction-k8

## Goal

Delete the hand-built finish transaction. The version control system owns safety,
history and transactionality: jj snapshots the working copy before every command
and its operation log is the transaction record, so grove **takes** a commit and
implements no witness, manifest, rollback proof, index image, quarantine or
recovery path.

## Context

- `docs/specs/module-decomposition.md`, decision 8, and the requirement
  *"no module implements a version-control guarantee"* with its scenario: a
  failed teardown commit names the operation-log command that restores the
  working copy, and **no grove-authored recovery runs**.
- `minimalism-k1`'s `## The VCS already does the finish transaction (measured)` —
  the jj 0.44.0 spike, colocated: `rm -rf .grove/` with no jj command run then
  `jj restore .grove` returned all five files; a **partial** deletion then
  `jj undo` reported *"Added 2 files"*, exactly the missing ones. That
  measurement is the warrant and is not to be re-gathered.
- `minimalism-k1`'s `## Deletion list`, *Contained* rows 1 and 4:
  `finish_transaction`, `finish_cleanup/*`, `repo/finish_commit` — **10,366
  lines, no surviving caller**; plus `driver_lease`'s
  `ensure_supported_workspace_layout` and `control_directory_device` and
  `repo::measured_device` (~200), which exist only to guarantee the quarantine
  rename is same-device.

## Done when

- `src/finish_transaction.rs`, `src/finish_cleanup.rs` and `src/finish_cleanup/`
  are gone. `src/repo/finish_commit.rs` is replaced by a plain path-scoped jj
  commit — take it, or refuse naming the operation-log command.
- The same-device gate and its device measurement are gone, along with every
  auto-repair function in the roughly twenty-five `minimalism-k1` enumerates
  (`recover_pending`, `rollback`, `recover_finish`, `recover_jj_finish`,
  `recover_jj_index_backup`, `recover_auxiliary`,
  `recover_partial_root_init_unlocked`, `reap_orphaned`, `restore_git_index`, …).
  Each becomes a message that names what is wrong and how to fix it, or becomes
  nothing at all where the anomaly it unwound can no longer occur.
- `grove-llm finish-commit` still revalidates the live driver-owned finish leaf
  and the absence of ordinary work under the exclusive tree lock, and still
  deletes and commits only `.grove/`. **The teardown's tree and VCS facts are not
  what is being deleted** — the transaction around them is.
- `docs/adr/task-tree-transactions-fail-closed.md` is **retired**, and the
  retirement says so on the record's own terms: it is superseded not by the
  reopen condition it names (a durable finish receipt) but because the version
  control system owns the transaction.
- `docs/adr/supported-workspace-layouts.md` is **retired** — its whole subject is
  the same-device rename the quarantine needed.
- `tests/finish_lifecycle.rs` (4,144) and `tests/workspace_layout.rs` (721) go;
  what survives of finish behaviour is asserted against the plain commit.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is the largest single deletion in the tree — about 10,400
non-test lines and roughly 4,900 test lines. It is contained: `minimalism-k1`
measured **no caller outside the deleted set**. Re-derive that before deleting,
and note that `drop-git-lane-k7` has already removed the auxiliary family, so the
count you measure will be smaller than the one recorded.

**The two retirements are the point of the leaf, not paperwork after it.**
`ADR-FORMAT.md` governs how a record is retired and how its citations are chased;
four artifacts link into this area, so chase them.

**Nothing here reaches for a replacement guarantee.** The temptation under a
10,000-line deletion is to keep "just the witness" or "just the rollback proof."
Principle 1 is settled: the VCS owns it. If this session finds itself arguing
against that, it raises it rather than quietly designing around it.
