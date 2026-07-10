# task-kinds-impl-k9

**Kind:** work

## Goal

Implement the five-kind taxonomy and the five-var model scheme decided in
`task-kinds-model-selection-k6`, and bring every doc that describes the old
binary into line — so `TASK-FORMAT.md` never promises a kind the binary rejects.

## Context

Decisions are already recorded, not open: ADRs `task-kind-taxonomy` (closed set;
gate-on-write / degrade-on-read; `leaf-decompose` inherits the parent's kind) and
`model-per-task-kind` (one env var per kind; **no fallback chain**; the corrected
`/model`-persistence caveat), plus the `Task kind` / `HITL` / `Per-kind model
selection` entries in `CONTEXT.md`. Read those three, not the grilling.

Grounded inventory (from `grep`, not memory):

| File | Change |
|---|---|
| `src/leaf.rs` | `Kind` → five variants; `parse`/`label` round-trip; error lists all five |
| `src/tree_read.rs` | `read_kind`: unknown token ⇒ **warn to stderr + return `Kind::Work`**, never `Err` |
| `src/loop_driver.rs` | `select_model`: five vars, no chain; keep the zero-subprocess fast path (skip the `grove-llm kind` peek when none of the five is set) |
| `src/tree_lifecycle.rs` | `leaf_decompose`: first child defaults to the **parent leaf's** kind, not `work` |
| `src/llm_cli.rs` | `--kind` help text for the three grow verbs; `leaf-decompose`'s `--kind` becomes an *override* of the inherited default |
| `src/cli.rs:9` | `--help` env-var block: five vars |
| `content/TASK-FORMAT.md` | "The two kinds" → the five; per-kind discipline; the **HITL/AFK** marking |
| `README.md:40-51` | env-var table (five rows); **fix the over-broad `/model` claim** (see below) |
| `docs/grove.md:23` | "one of two kinds" → the five |
| `tests/loop_driver.rs`, `tests/kind.rs` | extend at the existing seams |

`docs/concepts.md` never mentions kinds — leave it to `concepts-adr-refresh-k8`.

**The README defect** (present today, independent of this change): it claims an
in-session `/model` switch "does not persist across relaunch." Verified against
code.claude.com/docs — interactive `/model` *saves as the user's default for new
sessions*. It fails to persist only because grove passes `--model`, which outranks
settings. So it persists for any kind whose env var is **unset**. State both cases.

## Done when

- `cargo test` green; `cargo clippy` clean.
- `grove-llm leaf-add . x --kind review` succeeds; `--kind reserch` errors listing
  the five; a leaf hand-edited to `**Kind:** reserch` makes `grove-llm kind` print
  `work` with a warning on stderr and **exit 0**.
- `leaf-decompose` on a `research` leaf gives its first child `**Kind:** research`.
- `10-review-provider-research-k10.md` is **re-kinded to `research`** (its
  `**Kind:**` line was written as `work` because the enum did not yet accept it).
- No doc still says grove has two kinds.

## Test seams

Agreed in `task-kinds-model-selection-k6` — **zero new seams**, all four behaviours
land at seams that already exist:

- `select_model` stays private, exercised through `tests/loop_driver.rs`'s
  fake-harness argv recorder (`loop_selects_model_by_kind`).
- `Kind::parse`/`label`, `read_kind`'s degrade, and `leaf_decompose`'s inheritance
  each extend the inline `#[cfg(test)]` module in their own file.

## Notes

Order matters: this leaf sits ahead of `decomposition-craft-k7` so that leaf's
prose rewrite lands on the settled taxonomy. Both later leaves cite by key, so the
renumber this insert caused rewrote nothing.
