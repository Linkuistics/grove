# grove-confinement-docs-removal-k153

## Goal

Make Grove's documents, skill references, books and changelog agree with the
removal of confinement.

## Context

- The "Documents" list of the handoff in `docs/confinement-withdrawal-evidence.md`.
- The dispositions of the acceptance criteria in the same record.

## Done when

- Each document item of the handoff is done, except the runner sections of the
  acceptance record and the floor documents of the earlier siblings.
- The prose of the walkthrough books stops naming confinement, and the final
  book check passes.
- `CHANGELOG.md` has an entry for the withdrawal that states the weakened
  guarantees.
- The technical-communication checker passes on each changed prose document.
- The change is a described commit.

## Notes

Convert a `pending` region or a decision record to controlled English before
the change, as the handoff states.

## Decisions (running log)

- `plugins/grove/skills/grove/references/execute.md` converts only its
  section "Visible delegated work", and a `pending` region covers the rest.
  Tests pin many of its sentences, and Grove's skill corpus is not yet in
  controlled English. A whole conversion is methodology work outside this leaf.
- The standalone section of `docs/USAGE.md` was already in controlled English.
  The section "Release notes" of `docs/RELEASING.md` and its section "4. Check
  the installation" were `pending`. The change rewrites or removes most of
  their prose, so their conversion and their change are one rewrite in the
  task commit. The preparatory commit converts each document whose change is a
  small edit. These are the decision record, the evidence records, the
  coverage table, the book chapters and the skill section.
- "Runtime access" goes, and its replacement "The harness sandbox" states the
  helper's sandbox, its sign-in check and the inert
  `GROVE_RELEASE_RUNTIME_READ`. No link named the old anchor.
- "4. Verify the installation" became "4. Check the installation". The checker
  reads "verify" and "check" as one action, and no link named the anchor.
- The `off` region of released history in `CHANGELOG.md` now starts at
  `## v22.8.0`. It started at `## v22.5.0`, so the checker read the released
  entries of v22.6.0 to v22.8.0 and found three problems there.
- In the coverage table, G6 was separated from its table by a blank line, so
  it rendered as a paragraph. The conversion joined it to the table. The
  coverage test reads row identifiers line by line, so it is not affected.
- One label in the grove-loop concept index quotes the refusal "must be
  corrected by hand" of `task_grow.rs`. It carries a `synonym-rotation`
  departure.
- The handoff's `docs/ravel-target-inputs.md` needed no change. The floor
  leaf had already removed its confined route. In
  `docs/grove-delivery-contraction.md`, only the dated k49 and k52 sentence
  kept confinement. It now points at the withdrawal record, because its
  "accepted below" named nothing.
- `source-index.md` of the overview book stays unconverted. Its one finding is
  a cell that must equal the `statement` of `walkthrough.toml`, and this leaf
  does not change that file for confinement.
- The policy reference loses its recommendation of the directory route for
  `grove run`. That recommendation rested on the sandbox that hid native
  skills. Both routes now work for a standalone run.
- The design README gets a new "Current changes" section for the withdrawal.
  The earlier "Current changes" becomes "Earlier recovery changes". No link
  named either anchor.
- Dated records keep their confinement text. These are the stage record, the
  extraction and skill-delivery evidence, the caller-surface audit and
  inventory, the formalism findings and the research notes. Other matches use "confined" or
  "runtime read" in an unrelated sense.
- `bash scripts/check.sh` with the 1.1 keg passed all 19 checks, with mise's
  Bun 1.4.3 first on PATH. The run with the 2.0 build stays in `k154`.
