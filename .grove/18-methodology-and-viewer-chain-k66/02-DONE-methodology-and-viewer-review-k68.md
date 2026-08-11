# methodology-and-viewer-review-k68

**Kind:** review-impl
**Reviews:** methodology-and-viewer-k48
**Producer launch:** {"producer":"methodology-and-viewer-k48","session":"jj-task-commit-sealing-k162","generation":"k162","harness":"claude","model":"opus"}

## Goal

Adversarially review `methodology-and-viewer-k48` and record concrete findings for its integration step.

## Context

- This review is inspection-only. Inspect the producer's committed diff,
  source, specifications, and recorded verification evidence. Do not run test,
  build, lint, or format commands, edit production or test code, or redo the
  implementation.
- Record findings only. `methodology-and-viewer-integrate-k69` owns every fix
  and all post-fix verification.

## Done when

- Findings are recorded here with severity and concrete source or diff evidence,
  or an explicit no-finding result.
- The review relies on the producer's recorded verification evidence; no test,
  build, lint, or format command is run.
- No production or test code is changed.

## Notes

## Findings

### F1 — High — the documented task-commit boundary occurs before the operations it says the commit must contain

`content/SKILL.md:38-46` draws `Commit` before `Retire`, and the prose preserves
that order: the Commit step at `content/SKILL.md:357-377` says the one focused
commit includes the `DONE` rename and, in jj, must be sealed with `jj new`; only
the following Retire step at `content/SKILL.md:379-392` tells the session to run
`grove-llm leaf-retire`. The same later step can still add missing leaves,
promote brief/ADR/glossary material, and requires a closing node's stable handle
in the commit message (`content/SKILL.md:418-455`). All of those facts are
learned after the documented commit has already been made.

Following the page in order therefore cannot satisfy the new
`jj-task-commit-sealing-k162` contract. In jj, sealing at the Commit step opens a
new `@`, so the later `DONE` rename and any close-time edits land in the next
task's change — the exact cross-task contamination this slice exists to stop. In
Git, the same order leaves retirement/cascade edits uncommitted or forces a
second commit. Move leaf retirement, the parent-close walk, and durable-record
reconciliation before the task commit/seal boundary (or split and reorder the
loop steps) so the diagram, procedural headings, commit contents, and close-time
commit message can all be followed literally.

### F2 — Medium — the methodology falsely says Grove parses only the filename kind

`content/SKILL.md:290-295` says the filename is parsed “for exactly one thing”
and that Grove reads no ordering or position from it.
`content/TASK-FORMAT.md:266-275` repeats that the kind is the only parsed part and calls the position
convention rather than grammar. The shipped parser does the opposite:
`src/tree_id.rs:236-263` parses position, terminal outcome, routed kind, slug,
and permanent key into `Entry`; selection order, terminal filtering, stable
resolution, and key allocation depend on those fields.

This is not merely loose wording around relationships: it tells task authors
that mutable position and stable key are outside the grammar while the binary
uses both structurally. Restrict the claim to what is true — the kind is the
only routing token parsed out of the routed slug, while step suffixes encode no
relationship — and explicitly retain position, outcome, slug, and key as the
filename grammar.

### F3 — Medium — “every grow verb takes `--kind`” contradicts the fixed research-pair CLI, and the new test cannot catch that class of mismatch

`content/SKILL.md:341-350` states that every grow verb takes `--kind`, defaulting
to `impl` except for `leaf-add-chain`. But `leaf-add-pair` is one of the shape-
emitting grow verbs described on the same page and deliberately accepts only
`parent` and `stem` (`src/llm_cli.rs:397-406`); the existing executable contract
positively rejects `leaf-add-pair ... --kind design`
(`tests/leaf_chain.rs:215-253`). The earlier pair example correctly omits the
flag, so the canonical skill contradicts itself.

The producer's new guard overstates its coverage. `real_long_flags` flattens
flags from every subcommand into one global set
(`tests/session_kind_guidance.rs:564-586`), and
`every_documented_grove_llm_flag_exists_on_the_real_verb` checks a flag against
that global set (`tests/session_kind_guidance.rs:627-655`). A documented
`leaf-add-pair --kind` would therefore pass because some *other* verb owns
`--kind`. Scope the prose to the verbs that actually accept the selector, and
index the test's accepted flags by the specific subcommand named on the line.

### F4 — Medium — the producer recorded no verification evidence for any of its four completed slices

The completed producer tasks `lifecycle-methodology-k79`,
`session-kind-methodology-k86`, `review-methodology-k87`, and
`jj-task-commit-sealing-k162` each require focused guidance checks,
`cargo fmt --check`, and `cargo test --locked`, but all four task files end with
their original Notes and contain no `## Verification evidence` section or
command result. Their focused commits (`5d0800367760`, `2e2e1f9daec0`,
`2d51a7e0464e`, and `7909bf718ded`) record the diffs, not execution results.

That leaves this inspection-only review unable to satisfy its own instruction
to rely on the producer's recorded verification evidence without rerunning the
forbidden commands. The integration step must run and durably record the full
post-fix gate; it should not treat the retired leaves' unchecked “pass” clauses
as evidence that the aggregate was green.

### F5 — Medium — the filename-example guard accepts names the production parser rejects

The new test advertises that every documented filename example is classified
against the shipped grammar, but `classify_example` only finds a kind prefix and
an `-k` separator, then checks that the slug is non-empty
(`tests/session_kind_guidance.rs:249-285`). It ignores the key entirely and never
applies the production slug validator. Consequently examples such as
`01-impl-extract-knope.md` or `01-impl-bad_slug-k7.md` are classified `Kinded`
and keep the guard green, while `tree_id::parse` rejects them through
`parse_parts` / `validate_slug` (`src/tree_id.rs:242-246`).

This leaves the producer's filename-grammar examples protected only against the
old missing-kind spelling, not against the grammar it claims to enumerate. Use
the real parser for concrete examples and an explicit placeholder path for
grammar sketches, or make the classifier validate the numeric key and the same
slug rules with negative controls for both failures.

## Review coverage

- Inspected the aggregate committed producer diff from `bb956e9af6f3` through
  `7909bf718ded`, the four focused slice diffs, current methodology/doubt
  sources, the binding ADRs and specs, the relevant parser/CLI implementations,
  and the added guidance tests.
- Confirmed the single-launcher deletion, nineteen-kind enumeration,
  mandate-based review ownership, receipt/diversity removal, and jj sealing
  prose are otherwise represented in the producer diff.
- No producer verification results were recorded, which is F4; per this leaf's
  inspection-only mandate I did not substitute a test, build, lint, or format
  run of my own.
- No production or test code was changed.
