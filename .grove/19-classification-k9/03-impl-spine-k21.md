# spine-k21

## Goal

Classify **`content/SKILL.md` lines 5–166** (12,024 bytes): the title and mermaid
diagram, `## The spine — seven constraints`, and the loop's opening run — the
working-tree paragraph, *One configuration, no other launch policy*, the
session-name paragraph, *Starting a new grove*, *Pick*, *Do not pick again*, and
*Bootstrap*.

This is batch 1 of 12. It also **establishes the id conventions** the other eleven
inherit — see *Context*.

## Context

Read the node brief's *The batching contract* first; it carries the two lemmas,
the inbound sweep, the `pending-` convention and the full batch table. Do not
restate them here.

### Region and residual

- Carve `content/SKILL.md` **L5–L166**. L1–4 is the YAML preamble — the parser
  skips a leading `---` block uninterpreted, so **do not touch it**.
- The seed unit `skill` (currently L5–760) is **consumed**. Mint exactly one
  residual, **`pending-skill-loop`**, covering **L167–760**, as
  `class=triggering kinds=*` with no `defers=`.

### Cross-file deferral: none

This region references no other embedded file, so it writes **zero** cross-file
`defers=`. Every procedural unit it creates must be reached from a triggering
unit **inside the same region** — check (R) before you build, not after.

Candidate procedural bodies in here, to weigh rather than to accept: the
`${session_name}` derivation recipe, the `pick` pre-order walk's mechanics, and
*Bootstrap*'s read-order. Each sits behind a condition in the same region
(*the driver offers a session name*, *the driver has already picked*, *you have a
mandate*), so all three are self-rootable.

### What this batch fixes for the other eleven

1. **Id prefixes are file-scoped** — `skill-` here; `task-`, `driving-`,
   `grilling-`, `spec-`, `brief-`, `context-`, `adr-`, `continue-` elsewhere.
   Record the convention in your commit message.
2. **Residual ids are `pending-<file>-<next-region>`.**
3. Whatever id-naming grain you settle on (one id per bold-led block? per
   sub-clause?), **state it in your leaf body before retiring** — eleven sessions
   will follow it, and a convention discovered independently eleven times will
   not agree with itself.

### Two doubts already visible in this region

- **Constraint 2's parenthesis** (L63–68, "Keeping this skill in step with the
  `grove-llm` it instructs…") is prose about the build boundary, not a condition
  and not a procedure. The node brief's *Notes* says to flag such prose as a
  finding about the design rather than force it into a class.
- **The mermaid diagram** (L14–50) is a fenced block. The parser tracks fence
  state, so a marker cannot land inside it — but decide deliberately whether the
  diagram belongs to the title unit or to a unit of its own, because the choice
  moves bytes into or out of every mandate.

## Done when

- `content/SKILL.md` L5–166 is subdivided into real units; `pending-skill-loop`
  covers L167–760 and nothing else.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` in `tests/methodology.rs` is updated in the **same commit**:
  `skill` removed, the new `skill-*` ids added, `pending-skill-loop` added — each
  named deliberately.
- `grove-llm methodology` (rebuilt) lists the new units, and spot-fetching one
  triggering unit returns its bytes with its marker line intact.
- The id-naming convention is written into this leaf's body for the eleven
  batches that follow.

## Notes

- Nothing in this region ships to a mandate yet — no composer exists in this
  grove. A residual that is temporarily coarse costs nothing here; the 64 KiB
  per-kind alarm belongs to the successor grove.
- Doubts to carry forward: record, by id, the units you were least sure about.
  `finish-cycle-k32` assembles them into the aggregate `review-impl` handoff.
