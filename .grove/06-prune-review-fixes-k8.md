# prune-review-fixes-k8

**Kind:** work

## Goal

Fix the concrete code defects `review-k6` found in the pruning change
(`prune-verb-k2` + `methodology-k3`), before `release-k5` ships the binary that
embeds them.

## Context

- `review-k6`'s running notes (this leaf's immediate predecessor) — read it for
  the full adversarial trail; the two probe scripts below are reproduced from
  there.
- `src/tree_read.rs::resolve` (and `Resolution` / `AmbiguousMatch`).
- `src/tree_lifecycle.rs::leaf_prune` / `prune_subtree` / `cmd_leaf_prune` in
  `src/llm_cli.rs`.
- ADR *pruning* — the design intent these defects undercut.

## Done when

- **`resolve` distinguishes abandoned from live, not just from done.**
  `Resolution::Found.retired` (and `AmbiguousMatch.retired`) is computed as
  `e.is_done()`, which is `false` for an `Outcome::Abandoned` entry — so
  `grove-llm resolve` on an abandoned leaf prints the path with **no** stderr
  annotation at all, identical to resolving a genuinely live leaf. Repro:
  ```
  grove-llm root-init plan && git add -A && git commit -qm init
  grove-llm leaf-add . spike && git add -A && git commit -qm add
  grove-llm leaf-prune .grove/02-spike-k2.md && git add -A && git commit -qm prune
  grove-llm resolve "[2]"    # prints the path, no note — looks live
  ```
  Compare a `DONE` leaf resolved the same way: it gets
  `note: referenced task is retired (DONE): …` on stderr. An abandoned leaf
  should get an equivalent note (its own wording, not "DONE"). This is the
  exact failure mode ADR *pruning* exists to prevent — "a tree that hides its
  dead ends lies" — except here it's `resolve`, not the tree, doing the hiding.
  Fix by widening `retired: bool` to carry the real three-way outcome (or add a
  parallel `abandoned: bool`), threading it through `render_resolution`, and add
  a test asserting the stderr note appears for a resolved `ABANDONED` entry
  (the existing `resolve_finds_a_pruned_leaf_by_key` test only asserts the
  *path*, not `retired`/the note — it would not have caught this).
- **`leaf-prune <node>` is atomic, or its partial state is reported.**
  `leaf_prune`'s node branch calls `prune_subtree(&target_abs, &mut result)?` —
  on a `git mv` failure partway through the subtree walk (e.g. an untracked
  sibling leaf added earlier in the same uncommitted session — the same class
  of gotcha already filed as issue #3 for `leaf-insert`), every leaf processed
  *before* the failing one is already renamed to `ABANDONED` in the working
  tree and git's index, but the `?` discards the accumulated `PruneResult`, so
  `cmd_leaf_prune` prints nothing — the operator sees only the `git mv` error
  and has no idea part of the subtree was just mutated. Repro:
  ```
  grove-llm root-init plan && git add -A && git commit -qm init
  grove-llm leaf-decompose .grove/01-plan-k1.md design
  git add -A && git commit -qm decompose
  grove-llm leaf-add .grove/01-plan-k1 tracked-second
  git add -A && git commit -qm "add tracked-second"
  grove-llm leaf-add .grove/01-plan-k1 untracked-third   # deliberately NOT git-added
  grove-llm leaf-prune .grove/01-plan-k1
  git status --short   # design + tracked-second already renamed ABANDONED; nothing was printed
  ```
  Either make the walk two-phase (validate every leaf in scope is prunable —
  tracked, live/done — before mutating any of them) so a failure is a clean
  no-op, or report `result.marked`/`result.left_done` accumulated so far
  even when returning the error (print-then-bail, matching `leaf-insert`'s own
  "report what happened, error on top" style). Either fix needs a test that
  asserts on the partial-failure case, not just the clean-success cases the
  current suite covers.
- **Three `--help` doc comments in `src/llm_cli.rs` are stale** (all pre-date
  this branch's `Outcome` split and were never extended to the third state):
  - `Command::Pick`'s comment says it skips "briefs and retired (`DONE`)
    leaves" — omit-fix to mention `ABANDONED` too (the implementation, in
    `tree_read.rs`, already does the right thing).
  - `Command::Resolve`'s comment says it searches "live **and** retired
    (`DONE`) entries" — same omission (and once the `resolve` fix above lands,
    this is the moment to describe the abandoned-note behavior too).
  - `Command::LeafRetire`'s comment says it "Refuses a brief and an
    already-retired leaf" — it now also refuses an already-*abandoned* leaf
    (`tree_lifecycle::leaf_retire`'s new `Outcome::Abandoned` arm, shipped in
    `prune-verb-k2`); the enumeration should say so.
- `cargo test` still green; the new tests above included.

## Notes

None of these are design-level disagreements with ADR *pruning* — the mark, the
HITL gate, the bulk node arity, and the key-monotonicity property it exists to
protect all held up under adversarial probing (see `review-k6`'s notes for the
cases that were tried and passed: key reuse after pruning the highest key,
`pick` on all-abandoned / mixed DONE+ABANDONED trees, `leaf-prune` refusing
`DONE` leaves / the grove root / a brief, `validate_slug` reserving
`ABANDONED`). These are implementation gaps in code this same change shipped,
not push-back on the decision.
