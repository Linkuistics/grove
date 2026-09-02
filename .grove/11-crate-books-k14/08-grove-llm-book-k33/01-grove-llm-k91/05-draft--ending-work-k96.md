# ending-work-k96

## Goal

Draft chapter 5 of the `grove-llm` book: slice `two-steps-remain`,
`05-ending-work.md`, owning `verbs-ending` (`cli.rs` 217–247), `args-ending`
(400–411) and `handlers-ending` (768–825).

## Context

- Draft stage, child 5 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *5 · Ending work* section. Thesis: retire and prune are the
  terminal-marking pair and the **last grove verbs a session runs** — Retire
  precedes Commit, and the commit is jj's — so their stderr names the two steps
  that remain, at the moment of decision; stdout stays data. Cover
  `eprint_next_steps` and its plural; retire's refusals as the verb's and its
  byte-identical promise; prune's **HITL** rule as help the code cannot enforce;
  prune's node case; and the reminder printed last and only when something was
  marked.
- Link the glossary at `task-commit-boundary` beside the reminder.
- The required example anchor is `worked-retire`: `leaf-retire` of
  `/work/atlas/.grove/01-impl--rate-limit-k3.md` — the renamed path on stdout
  and the two remaining steps on stderr; `leaf-prune` of a node shown once, with
  its untouched `DONE` leaf reported. Measure both against the built binary.
- Evidence: `leaf_ops.rs` and `reviewed_producer_lifecycle.rs`. Mark the
  terminal-mark handlers row `explained`.

## Done when

- The fragments for the three blocks are defined on the page, the defers are
  replaced, the ownership rows read `resolved`, the fragment index has their
  rows, and navigation, contents and the concept index are updated.
- `book-check --through two-steps-remain --check all` is valid: 917 resolved
  lines, 100 deferred. The repository Markdown sweep passes. `scripts/check.sh`
  stays red on `book-check` alone, by design.
