# ravel-history-snapshot-k19


## Goal
Deliver an explicit operator instrument that snapshots legacy history into a
separate Ravel store and safely imports later rows, without personal migration.



## Context
The legacy record schema is defined by `crates/harness-dispatch/src/store.rs`
in this Grove workspace. The catalog fixture carries opaque historical fields.
Ravel's runner is committed at ee5e64b1; its retired bookmark remains at
035edfa30f3bf40eb870a525f49ac98a3b5abbb1 locally and at origin.

## Done when
- Ravel has a documented, explicit source/destination snapshot command using
  SQLite's backup API, with read-only legacy access and a separate application ID.
- New snapshots preserve row IDs, timestamps and document bytes. Existing Ravel
  destinations receive only absent rows in one transaction; identical rows are
  idempotent, conflicts refuse with no committed changes. Late supervision and
  correction-chain observations can be caught up without replacing a destination.
- Tests exercise real temporary SQLite stores, WAL snapshots, schema 1/2,
  legacy/foreign destination refusal, missing/corrupt inputs, conflicts and
  rollback, path aliases and bounded locks. No personal stores are opened.
- Root Taskfile checks the instrument with pinned development tools and the
  existing runner. Document its operator-only scope and remaining launcher work.
- Commit Ravel through jj, record the commit and evidence here, retire this
  child and seal the coordination change. k20 and all ancestor runtime nodes
  remain live; k16 performs the eventual actual migration.

## Notes
No installed Ravel CLI, publication, release preparation, personal configuration
or personal history migration in this child. Preserve the live old command.

## Decisions (running log)

**Implementation plan.** Use a Python 3.11+ standard-library operator script,
not a new shipped executable or public runtime protocol. Write real-store
regressions first; use a private backup and validate the known immutable schema;
publish a new destination without replacement or append to an existing Ravel
store transactionally; add help, JSON result/error, dry-run and bounded waits;
document and check it through Taskfile; seal Ravel and coordination commits.
The existing requirements and approved extraction architecture supply the design;
there is no new human scope decision to re-interview.

**Store identity.** Ravel uses SQLite application ID 0x52564c53 (RVLS), with
the existing schema versions 1 and 2. The legacy HDRS identity is accepted only
as the script's read-only source. k20 must use RVLS and refuse the legacy ID
in every runtime store path. Launch and observation documents stay byte-identical.

**Complete old-tree retirement.** On the human's correction, checked the full
tracked tree and on-disk remnants. k17 had already removed every tracked old
implementation/doc/state path in ee5e64b1. Moved ignored Elixir dependencies,
build products, editor state and empty old source/state/website directories to
the sibling Ravel-retired-local-artifacts-2026-10-12 archive before further
implementation. The durable retired bookmark remains unchanged.

**Human rejection.** Historical continuity is not useful. The human further
removed the persistent store, all record/lookup APIs, extra CLI verbs and prompt
injection. This task is abandoned; its prototype and Python toolchain were
removed from Ravel. No personal history was opened or migrated. The current
root/k18 contracts and Ravel docs/session-cli.md replace this task's proposed
store identity, schema/import design and acceptance conditions.

**Testing workflow.** The human selected cargo nextest for efficient test
execution and its reporting UI. The replacement Taskfile/check entry points
keep that UI visible, with saved formatting/lint diagnostics. The rejected
prototype's Python toolchain and generated environments/caches were removed.
