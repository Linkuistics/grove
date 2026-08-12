# shapes-k23

## Goal

Classify **`content/TASK-FORMAT.md` lines 193–472** (15,904 bytes):
`## Composing the kinds — the two shapes`, `### The review chain — each session
cuts the next step`, `### The vendor pair — one eager call`, `### What the shapes
are not`, `## Suggested shape`, and `## A leaf never names a harness`.

This is batch 3 of 12, and the largest single region in the plan.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve **L193–L472**, consuming `pending-task-shapes` in full. `TASK-FORMAT.md`
  is finished after this batch — mint **no** residual.
- L473–501 was already carved by `kinds-k22`; leave it alone.

### Cross-file deferral

- L204 and L285-ish point at `driving.md`; L?? points at `SPEC-FORMAT.md` and
  `BRIEF-FORMAT.md`. Run the inbound sweep for *this* file's own references and
  write `defers=` **only** where the target already exists — at this point that is
  nothing outside `TASK-FORMAT.md` itself, so expect to write **no** cross-file
  edge here either.
- Conversely, `shape-cutting-k30` will sweep back into your units when it carves
  `SKILL.md`'s *bare stem* and *grammar is five fields* paragraphs, which cite
  `TASK-FORMAT.md` for "the full reasoning". Name the unit that holds that
  reasoning so the sweep finds it.

### The judgement this batch exists for

The two shapes are **built in opposite ways**, and that asymmetry is the design.
The condition a session must not miss is *"more than one leaf serves one
artifact"* plus the two follow-ons — *"a producer's last act is to decide whether
review is required"* and *"a pair is cut eagerly, whole"*. Those are triggering:
a session that is never told them cuts neither shape and never knows it had a
choice. That is the unasked question the whole design exists to prevent.

The **bodies** — the exact `leaf-add` invocations, the `leaf-insert` targeting
rule for an integration, the discriminator filenames a pair writes — are
procedural.

`### What the shapes are not` (7,946 bytes) is the section to think hardest
about. It is mostly *rejected alternatives* and *why the grammar infers no
relationship*. Rejected-alternative prose is neither a condition nor a
procedure — the node brief says to say so rather than force it into a class. But
"the grammar infers no relationship between leaves" **is** a condition: a session
that assumes an `X` requires a `review-X` after it will cut leaves it does not
need. Split that section rather than classifying it whole.

### Traps specific to this region

- **Splitting mid-fence.** L257–258 and the `leaf-add-pair` example are indented
  code blocks and fenced blocks; the parser forbids a marker inside a fence and
  will say so, but the authoring rule behind it is the one no build checks — a
  unit must read correctly standing alone.
- `## Suggested shape` contains a fenced markdown template with `#` headings
  inside it. Do not let a heading scan mistake those for section boundaries.

## Done when

- `content/TASK-FORMAT.md` L193–472 is subdivided into real units;
  `pending-task-shapes` is gone and no `pending-task-*` unit remains.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The unit holding the step-suffix reasoning is named in this leaf's body, for
  `shape-cutting-k30`'s inbound sweep.

## Notes

- If any prose in `### What the shapes are not` is genuinely narrative — there to
  make the document readable and neither condition nor procedure — record that as
  a **finding about the design** in this leaf's body. Do not force it into a
  class, and do not silently leave it in a triggering unit to make the build pass.
- Doubts to carry forward, by id.
