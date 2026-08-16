# task-format-conditions-k14

## Goal

Rewrite `content/TASK-FORMAT.md`'s **14 universal triggering units** into
condition lines plus deferred remainders, and settle where the task-file grammar
lives in `content/references/`.

## What this covers

The universal half of `TASK-FORMAT.md`, after `per-kind-references-k12` has taken
its **14 narrowed** units into the per-kind files — which is exactly half of that
file's 28 triggering units, so this child works on a materially smaller file than
the one on disk today.

What remains universal: the leaf filename grammar, the kind-in-the-filename rule,
the nineteen kinds table, the HITL/AFK mark, the in-session doubt budget, the
too-big-is-planning rule, the two composing shapes, declaring the relationship in
the body, what the shapes are not, no node for a shape, the five-field grammar,
nothing-in-a-body-is-metadata, the three design kinds, and the
deliverable-split-is-not-a-gate rule.

Its 10 procedural units move to `content/references/` alongside.

## Two things not to lose in the compression

Both are units whose *point* is a distinction that dies if the condition line is
written carelessly:

- **`task-grammar-is-five-fields`** turns on the difference between what is
  *parsed* (position, outcome infix, kind, slug, key) and what is *convention*
  (the shared stem, relative ordering, the two declaration lines). A condition line
  that says "a leaf name is five fields" without the convention half loses the
  argument that decided the bare-stem rule.
- **`task-in-session-doubt-budget`** is a table keyed by the picked session's kind.
  A table is not a condition sentence, and squeezing it into one would misstate it.
  The condition is *the session has run Bootstrap and adopted the mandate, and
  doubt has arisen*; the table is the procedure and belongs in the reference file.

## Done when

- `TASK-FORMAT.md`'s 14 universal units are condition lines in `SKILL.md` with
  deferred remainders in `content/references/`.
- The composed-mandates golden shows exactly the expected shrink.
- The build gate passes.
- `cargo test` is green.

## Notes

`tests/session_kind_guidance.rs` (84 kB) is the suite file most exposed to this
child. It generates its claims from `Kind::ALL` and the real `grove-llm` command
model rather than enumerating spellings — which is why it is worth reconciling
carefully rather than trimming: it is the check that fails the day a *twentieth*
kind ships undocumented, and that property must survive the rewrite. Where it
asserts a kind's guidance exists, the guidance now lives in that kind's reference
file; point it there rather than weakening the claim.

The nineteen-kind table and the two-shapes section are the material
`skill-opening-k16`'s routing table draws on. Leave them in a shape that child can
route to without restating.
