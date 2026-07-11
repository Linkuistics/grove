# prune-verb-k2

**Kind:** work

## Goal

Ship the `ABANDONED` infix and the `grove-llm leaf-prune` verb in the CLI, per ADR
*pruning*. TDD — the id-model and lifecycle modules are already test-dense; match
that.

## Context

Beyond the brief chain and ADR *pruning*:

- `src/tree_id.rs` — the v2 id model. `parse_stem` peels the optional `DONE-`
  infix; `Entry::Leaf { is_done }`, `name()`, `is_live_leaf()`, `next_key()` and
  `validate_slug()` all live here.
- `src/tree_lifecycle.rs` — `leaf_retire` is the direct template for `leaf_prune`
  (git mv, add the infix, refuse brief / node / already-marked).
- `src/tree_read.rs` — the walk; `pick` must skip `ABANDONED` exactly as it skips
  `DONE`.
- `src/llm_cli.rs`, `src/cli.rs` — verb wiring and `--help`.

## Done when

- **Id model.** A leaf's outcome is one of *live* / `DONE` / `ABANDONED`. The two
  marks are mutually exclusive; a node directory still refuses any infix.
- **`validate_slug` reserves `ABANDONED`**, exactly as it already reserves `DONE`
  and `BRIEF` — otherwise a leaf could be slugged into a state token. One line, and
  a real hole if missed.
- **`next_key` counts `ABANDONED` leaves.** It already maxes over *all* names, so
  this should need no code change — but **assert it in a test**, because it is the
  entire point of the ADR and a future refactor must not quietly regress it.
- **`pick` skips `ABANDONED`.** A grove whose only remaining leaves are abandoned
  reports "no live leaves; this grove is done" and enters the finish cycle — correct:
  the work is settled, however it settled.
- **`leaf-prune <leaf>`** marks in place, keeping position and key, leaving file
  contents byte-identical (the infix is filename-only). Refuses a brief, an
  already-`DONE` leaf, and an already-`ABANDONED` leaf.
- **`leaf-prune <node>`** marks every *live* leaf in the subtree, leaves `DONE` ones
  untouched, prints what it marked and what it left alone, and **refuses the grove
  root** (abandoning a whole workstream is a branch-delete, not a tree mark).
- **`resolve` still finds a pruned leaf by key.** An abandoned leaf's key must stay
  resolvable — durable cross-references to it are precisely what the ADR protects.
- Tests cover each of the above; `cargo test` green, `cargo clippy` clean.

## Notes

**Do not touch `content/` or `CONTEXT.md` here** — the methodology is `methodology-k3`'s
leaf. Keeping them apart keeps both sessions small.

**Do not fix `leaf-insert` here.** Its `git mv` fails on an untracked source
(issue #3) — a different verb, already filed, and absorbing it would swell this
session. If `leaf_prune` can avoid the same trap for free (`fs::rename` over
`git mv`), take that for `leaf_prune` only and say so in the commit.

Forward-compat is **not** a requirement: an older binary reading a tree that contains
an `ABANDONED` leaf is out of scope (a tree is driven by one grove's sessions, and
`.grove/` dies at finish). Add no migration machinery.
