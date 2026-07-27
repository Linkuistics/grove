# bootstrap-leaf-kind-k19

**Kind:** impl

## Goal

Settle which kind `root-init` mints for a fresh grove's first leaf, now that
`planning` no longer grills — and make the code, the ADR and the start-path
routing agree on the answer.

## Context

Surfaced by `config-sweep-k16` while relabelling the doc surface, and
externalized rather than absorbed: it is a code + ADR change, not a doc sweep.

The inconsistency, as it stands after `01`–`06`:

- `root-init` writes `**Kind:** planning` (`src/tree_lifecycle.rs`, and the test
  `root_init_first_leaf_is_a_planning_task`), and *fresh-grove-start-contract*
  records that as a contract.
- The `start` path routes as `KindPeek::Leaf(Kind::Planning, None)`
  unconditionally, because there is no leaf on disk to peek
  (`src/loop_driver.rs`) — so a brand-new grove's model comes from the
  `PLANNING` suffix.
- `content/prompts/start.md` has that session **grill** on the goal. But
  `docs/specs/task-kind-taxonomy.md` now defines `planning` as AFK and *not*
  interrogating: grilling moved to `requirements`.

So a fresh grove's first session does `requirements` work under a `planning`
label, and draws a `planning` model for it.

## Done when

- One of the two readings is chosen and recorded: either `root-init` mints
  `requirements` (and the start path routes as `requirements`), or `planning`
  stays and the spec/prompt say why the bootstrap leaf is the one place the two
  disciplines are deliberately fused.
- *fresh-grove-start-contract* is reworked **in place** to state the chosen
  answer, and *model-per-task-kind*'s "start-path is planning by construction"
  bullet is reconciled with it.
- `content/prompts/start.md` and `content/TASK-FORMAT.md` agree with the code.
- The renamed test and the start-path launch test assert the chosen kind.

## Notes

Not merely cosmetic: the start path is the **one** launch grove routes without
peeking a file, so whichever suffix it names is the one var a brand-new grove
cannot run without — and under `required-model-vars-k18` an unset one is now a
hard failure on the very first `grove do`. Whichever way this lands, say plainly
in `README.md` which single var a fresh grove needs.

The cheap answer is probably `requirements`, since that is where the grilling the
prompt actually performs now lives — but it moves the *first-run* var, so it is a
user-visible config change, not a rename.
