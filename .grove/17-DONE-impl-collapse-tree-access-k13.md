# collapse-tree-access-k13

## Goal

Delete grove's **second lock layer**. All three of the recorded reasons for it
have dissolved, and this is the leaf that collects the dividend.

## Context

`src/tree_lifecycle.rs:127` records why grove holds its own `flock` on top of the
library's: *"Classification needs grove's guard, since the tree it classifies may
be absent, legacy or mid transaction and the library can read none of those"* —
and the two **deadlock**, *"two open file descriptions on one directory do not
share a lock"*.

Each clause is now false, and only now:

| clause | dissolved by |
|---|---|
| *may be absent* | `open-shape-k25` — `Reading`/`Writing` answer it as a shape |
| *may be legacy* | `delete-migration-k6` — there is no legacy tree shape |
| *mid transaction* | `delete-finish-transaction-k8` — there is no transaction |

`minimalism-k1`'s `## Deletion list`, *Reconciled* row 3: `src/tree_access.rs`
(315 lines), **37 call sites**, of which 6 are in modules already deleted by this
point and 24 are in `tree_lifecycle`, itself heavily reduced. **Seven sites in
surviving code need rework, not deletion**: `llm_cli` 4, `task_name` 2,
`task_tree` 1. The two-phase `Classification`/`settle` dance and `write_scaffold`
go with it.

## Done when

- `src/tree_access.rs` is gone. Grove acquires no lock of its own; every tree
  operation opens through the store's `read` / `write` and gets its guard from
  there.
- The `Classification` / `settle` two-phase dance and `write_scaffold` are gone.
- **All seven surviving sites are reworked and each is named in the commit
  message** — this is the awkward part of the whole deletion list and the reason
  it is its own leaf. Re-derive the seven rather than trusting the 2026-08-28
  count.
- `src/tree_lifecycle.rs` (2,853 lines today) shrinks to whatever survives; its
  remaining responsibility is stated in one sentence at the top of the file.
- `tests/tree_access.rs` and `tests/lifecycle_invariants.rs` are reconciled — the
  invariants that still exist are asserted against the store's guards.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**`complete_partial_root_unlocked` is this leaf's, and `delete-finish-transaction-k8`
left it deliberately.** It is the last of `minimalism-k1`'s roughly twenty-five
auto-repair functions still standing (it is the one k1 lists as
`recover_partial_root_init_unlocked`), and k8's done-when named it. k8 left it
because the anomaly it repairs — a root holding its charter and no keyed entry —
is one *grove itself creates*, in the very two-phase `Classification`/`settle`
dance this leaf deletes. Turning it into a refusal one leaf early would make
`bare grove` stop on a state its own `root_init` produced, which is a regression
rather than a deletion. Delete it here, with the window it exists to close.

**Lands green**, and it is the second of the two **forced orderings** the
planning leaf identified. It cannot move earlier: all three reasons dissolve at
once, and any earlier attempt reintroduces the deadlock the second layer was
papering over.

**The deadlock is the failure mode to test for.** Two open file descriptions on
one directory do not share a lock, so the bug this leaf removes is *grove waiting
on itself*. A test that opens the tree twice in one process is the direct
assertion; write it before the deletion, not after.

**This is grove-side work, deliberately outside the `store-operations-k12`
node.** The node's leaves speak the store's vocabulary; this one speaks grove's.
