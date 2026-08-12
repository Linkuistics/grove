# spine-k21

## Goal

Classify **the front of `content/SKILL.md`**, from the file's body start down to
the line before `**Execute.**` (baseline L5–166, 12,024 bytes): the title and
mermaid diagram, `## The spine — seven constraints`, and the loop's opening run —
the working-tree paragraph, *One configuration, no other launch policy*, the
session-name paragraph, *Starting a new grove*, *Pick*, *Do not pick again*, and
*Bootstrap*.

This is batch 1 of 12. It also **establishes the id conventions** the other eleven
inherit — see *Context*.

## Context

Read the node brief's *The batching contract* first; it carries the boundary-anchor
rule, the marker-placement convention, the narrowed greenness lemma and the local
per-marker obligations, edge ownership and the two sweeps, the `pending-`
convention, the five pre-decided repeated-rule families, the edge inventory and the
full batch table. Do not restate them here.

### Region and residual

- **The anchor is authoritative, not the line range.** Carve from
  `content/SKILL.md`'s body start to the line **before** `**Execute.**`. Baseline
  L5–166 is orientation only; you are the first batch, so it happens to be accurate
  here — but state the anchor in your commit message, because the eleven batches
  after you cannot rely on their ranges.
- L1–4 is the leading `---` block — the parser skips it uninterpreted, so **do not
  touch it** and do not place a marker inside it.
- The seed unit `skill` is **consumed**. Mint exactly one residual,
  **`pending-skill-loop`**, covering `**Execute.**` to end of file, as
  `class=triggering kinds=*` **with no `defers=`** — a residual is a coverage
  placeholder and never an edge ledger.
- **Marker placement:** a marker goes immediately above the first prose line of its
  unit, so the blank line above a marker belongs to the preceding unit. Baseline
  L166 is that blank line, and it is **inside your region** — your last unit runs
  through it, and `execute-k29`'s first marker sits above L167 (`**Execute.**`). The
  corpus arithmetic in the node brief assumes exactly this, at all four `SKILL.md`
  boundaries.

### Edge inventory rows owned: none

This region references no other embedded file, so it owns no inventory row and
writes **zero** cross-file `defers=`.

Every procedural unit this batch creates must be reached from a triggering unit
**inside the same region** — check (R) before you build, not after. And the root
must be a condition the body actually answers; reachability is satisfiable by any
inbound edge, which is what makes an artificial root easy and dishonest.

Candidate procedural bodies in here, to weigh rather than to accept: the
`${session_name}` derivation recipe, the `pick` pre-order walk's mechanics, and
*Bootstrap*'s read-order. Each sits behind a condition in the same region
(*the driver offers a session name*, *the driver has already picked*, *you have a
mandate*), so all three are self-rootable.

### What this batch fixes for the other eleven

1. **Id prefixes are file-scoped** — `skill-` here; `task-`, `driving-`,
   `grilling-`, `spec-`, `brief-`, `context-`, `adr-`, `continue-` elsewhere.
   Record the convention in your commit message. This is what makes embed-wide id
   uniqueness hold without any coordination between batches.
2. **Residual ids are `pending-<file>-<next-region>`,** always
   `class=triggering kinds=*` and always without `defers=`.
3. Whatever id-naming grain you settle on (one id per bold-led block? per
   sub-clause?), **state it in your leaf body before retiring** — eleven sessions
   will follow it, and a convention discovered independently eleven times will
   not agree with itself.
4. **Name every unit an anchor can find.** Later batches reach back into your
   markers to add `defers=` members (rows 24–25 of the inventory may target
   `**Execute.**`'s neighbours, and #9 reads your unit list before carving). An id
   whose prose you can locate by `grep -n` on a heading or bold-lead costs the next
   eleven sessions nothing; one that needs a line number costs each of them a
   re-derivation.

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

- `content/SKILL.md` from its body start to the line before `**Execute.**` is
  subdivided into real units; `pending-skill-loop` covers `**Execute.**` to end of
  file and nothing else, and carries no `defers=`.
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
