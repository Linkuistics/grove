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

## Batch record

**Anchor executed against:** `content/SKILL.md` body start (L5, the file's first
post-preamble byte) → the line before `**Execute.**`. `grep -Fc '**Execute.** The
**filename** states'` returned 1, and the baseline range L5–166 agreed with the
anchor exactly — 12,024 bytes measured before marking, as the node brief records.
The eleven batches after this one cannot assume that agreement.

### The units, in document order

Twelve real units, all `class=triggering kinds=*`, and one residual. Byte counts
are post-marking (each unit's own marker line included, as a slice carries it).

| id | region | bytes |
|---|---|---|
| `skill-what-a-grove-is` | H1 + the opening paragraph | 422 |
| `skill-loop-diagram` | the mermaid block | 1,851 |
| `skill-spine-constraints` | `## The spine` + the seven | 1,920 |
| `skill-working-tree` | `## The loop` + the working-tree paragraph | 481 |
| `skill-bare-grove-dispatch` | *Sessions are launched by the `grove` CLI…* | 942 |
| `skill-self-driving-loop` | *Bare `grove` drives the whole loop…* | 1,047 |
| `skill-one-configuration` | `**One configuration, no other launch policy.**` | 1,234 |
| `skill-session-name` | *The driver computes this grove's session name…* | 831 |
| `skill-starting-a-new-grove` | `**Starting a new grove.**` | 1,622 |
| `skill-pick` | `**Pick.**` | 847 |
| `skill-do-not-pick-again` | `**Do not pick again.**` | 523 |
| `skill-bootstrap` | `**Bootstrap.**` | 993 |
| `pending-skill-loop` | `**Execute.**` → EOF | 40,077 |

**Coverage, verified rather than assumed:** the twelve sum to 12,713 = the
current L5–177 exactly; the residual is 40,077 = L178–EOF exactly; 12,713 +
40,077 + 281 preamble = 53,071 = the whole file. The 281 is the node brief's
preamble figure, confirmed independently here.

### The id-naming grain — the convention the other eleven inherit

1. **File-scoped prefix**, as the plan fixed: `skill-` here; `task-`, `driving-`,
   `grilling-`, `spec-`, `brief-`, `context-`, `adr-`, `continue-` elsewhere.
2. **The default unit is the bold-led block** — a `**Lead.**` plus every
   paragraph and list it introduces — and, where a `##` heading introduces prose
   that no bold lead subdivides, the heading plus that prose. An *un-led*
   paragraph sitting between bold-led blocks gets its own unit when it states a
   rule that stands on its own (three did here), and otherwise joins the unit
   above it.
3. **Split finer only when the split changes a `class` or a `kinds=` scope.**
   Two adjacent units with the same class and the same scope ship together in
   every mandate and defer identically, so the extra id buys nothing but an
   address no one uses. This is why the seven constraints are one unit and not
   seven.
4. **Merge coarser only when the parts cannot be separated at line
   granularity** — a marker is a whole unindented line, so a boundary that falls
   mid-sentence or inside a list item is not available. See finding F1.
5. **Name a unit for its subject, never its position** (`skill-pick`, not
   `skill-loop-4`), and place the marker immediately above the prose whose
   heading or bold lead names it, so `grep -n '<!-- unit: skill-pick'` is the
   whole lookup for a later batch adding a `defers=` member.
6. **Residual ids are `pending-<file>-<next-region>`**, always
   `class=triggering kinds=*`, never with `defers=`. Minted here as
   `pending-skill-loop`.

**One exception to (5), forced by the grammar and not a judgement.** The *first*
marker of a file must sit at body start, because any byte before the first marker
is `BodyBeforeFirstMarker`. So `skill-what-a-grove-is`'s marker stays on L5 with a
blank line beneath it, and that blank belongs to the unit. Every batch carving a
file's head meets this; nowhere else does it apply.

### Edge inventory rows owned: none — reported as planned

Zero `defers=` written and zero `class=procedural` units created, so (D), (R) and
(T) hold vacuously. Both sweeps run and recorded:

- **Outbound** — `grep -E '(SKILL|TASK-FORMAT|BRIEF-FORMAT|SPEC-FORMAT|ADR-FORMAT|CONTEXT-FORMAT|driving|grilling|continue)\.md'`
  over L5–177: **no hits**. The region references no other embedded file, exactly
  as the leaf body predicted.
- **Inbound** — `grep -rn 'SKILL\.md' content/` outside `SKILL.md`: fourteen hits,
  none of them an edge into this batch. Three cite units this batch created
  (`driving.md` L256, L619, L672, all citing a spine constraint) and all three
  target `skill-spine-constraints`, which is **triggering** — an illegal `defers=`
  target, so no edge is legal and none is needed. By the later-endpoint rule those
  hits belong to #5 (L256) and #8 (L619, L672), which should **decline** them with
  that reason rather than discover it against the build. The rest point into
  regions #9 and #11, or are the framing mention at `driving.md` L4.

**An inventory addition — a standing sweep exclusion the plan does not list.**
Four of those fourteen hits name **another repository's** `SKILL.md`, in vendored
provenance and citation lines: `grilling.md` L2 and `driving.md` L39, L624, L667,
all `mattpocock/skills`. None is a reference to this corpus at all. Batches #4, #5
and #8 will hit them in every `grep -rn '<F>' content/` and none is a trigger→body
edge — the same standing-exclusion shape as `SKILL.md`'s `## Reference files`
index.

### Zero procedural units, and why — the three candidates weighed

The leaf body named three candidate bodies "to weigh rather than to accept". All
three were weighed and none was carved. Two for the same mechanical reason and one
on the merits, and the distinction matters to the reviewer:

- **The `${session_name}` derivation recipe** is a genuine procedural body sitting
  behind a genuine condition (*the session name does not match*), and it is
  **unsplittable**: the whole paragraph is one physical line, markers are whole
  lines, and this pass edits no prose. Same shape as the node brief's L217–227
  fusion.
- **`pick`'s pre-order walk mechanics** — likewise unsplittable. Every sentence
  boundary inside `**Pick.**` falls mid-line; there is no line start between the
  condition and the mechanics.
- **`Bootstrap`'s read-order** — rejected **on the merits**, not on splittability.
  A session must perform Bootstrap before it can fetch anything, so a deferred
  read-order is a body that cannot arrive in time to be used. A procedure whose
  entire content is *what to do first* is triggering by construction.

Zero procedural units in the corpus's most condition-dense region is the expected
result, not a gap: this is the spine and the loop's opening, and every block in it
is a standing rule a session must already hold.

### Doubts, by id — for `finish-cycle-k32`'s aggregate handoff

1. **`skill-loop-diagram`** (1,851 B) — the one real judgement call here. It
   contains no condition stated *completely* anywhere in it; every step it names
   is owned in prose by a later region, so by the plan's default it is not an
   owner. It was nonetheless classified **triggering**, on two grounds: it is the
   corpus's only whole-loop overview, and — decisively — there is **no honest root
   for it as a body**, because no condition anywhere says *when you want the
   loop's shape, look here*. Rooting it from `skill-what-a-grove-is` would be
   precisely the artificial root the plan warns against. Flipping it is a
   one-marker change if the reviewer disagrees.
2. **`skill-starting-a-new-grove`** (1,622 B) — a `kinds=requirements` narrowing
   was weighed and **declined**. Only a `requirements` leaf is ever a fresh
   grove's first session, which is the case for narrowing; against it is that a
   `planning` session deciding a stage deserves its own grove is exactly the
   session that needs *"You never scaffold the tree yourself"*, and a scope that
   is too narrow fails silently in eighteen kinds. The asymmetry decided it. No
   unit in this batch carries a narrowed scope.
3. **`skill-bare-grove-dispatch`** (942 B) — largely human-facing prose (how to
   run the CLI, what migration does). Kept triggering because its operative half
   for a session is a prohibition — migration is the driver's work, not yours —
   but it is the weakest triggering claim in the batch after the diagram.

### Design findings

**F1 — three paragraphs are each one physical line, and the grain is bounded by
that.** `skill-bare-grove-dispatch`, `skill-self-driving-loop` and
`skill-session-name` are single lines of 942, 1,047 and 831 bytes. Markers are
whole unindented lines and this pass edits no prose, so no sub-paragraph boundary
was available in any of them even where condition and body plainly separate — most
visibly in `skill-session-name`, which fuses *suggest `/rename` once and move on*
with the two-command derivation recipe that answers it. This is the same finding
the node brief records at L217–227, and it is **not** confined to that paragraph:
de-fusing these is a prose edit for a later grove, not a marking decision.

**F2 — constraint 2's parenthesis is narrative, and it is also stale.** The
build-boundary aside inside `skill-spine-constraints` is neither a condition nor a
procedure — the prose the node brief's *Notes* asks to be flagged rather than
forced into a class. It is unsplittable regardless (indented, inside a numbered
list item, where a marker would be silently read as prose). Beyond class, it is
**stale on its own terms**: it says the binary "re-provisions [this methodology]
on every lifecycle invocation", which is exactly what
`docs/adr/mandate-delivers-the-methodology.md` retires. The root brief's stage-4
*Docs and records* list does not name `content/SKILL.md`'s prose, only its YAML
preamble — so this sentence is currently scheduled for repair by nobody.

**F3 — a residual is not evidence of anything.** `pending-skill-loop` is
`class=triggering kinds=*` over 40 kB, which would look alarming to a reader who
did not know it is a coverage placeholder. Recorded here only so the reviewer
reads intermediate commits correctly; #12 removes the last one.
