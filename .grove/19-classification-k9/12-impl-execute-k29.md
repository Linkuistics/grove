# execute-k29

## Goal

Classify **`content/SKILL.md` from `**Execute.**` to the line before `**Cut the next
step, when it is needed.**`** (baseline L167–246, 5,455 bytes): `**Execute.**` and
its three bullets, `**Review ownership inside a picked leaf.**`, the *"Whichever
kind is running"* paragraph, and `**Decompose.**` with its two bullets.

This is batch 9 of 12. It is the **smallest region in the plan and by far the densest
in cross-file deferral** — six of the nine embedded files are named in these
80 lines, it owns **nine inventory rows**, and it carries the **owner** of four of the
six repeated-rule families.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L167–246 is a baseline coordinate — and by the time
  you open the file, batch #1 has inserted markers above it.** Carve from
  `**Execute.**` to the line **before** `**Cut the next step, when it is needed.**`,
  consuming the front of `pending-skill-loop`. Your region ends **including** the
  blank separator line before that anchor, per the marker-placement convention; that
  is the one-byte correction `batches-k33` F7 found (your baseline range is L167–246,
  not L167–245, and your size is 5,455 bytes).
- Mint exactly one residual, **`pending-skill-shapes`**, covering `**Cut the next
  step, when it is needed.**` to end of file, as `class=triggering kinds=*` **with no
  `defers=`**.
- **There is nothing to inherit from `pending-skill-loop`.** No batch parked an edge
  on it: a residual never carries `defers=`, and no edge may have a `pending-*`
  source. Batches 4–8 reported their `SKILL.md` hits as *not yours* instead, and
  those hits are the inventory rows below.

### The pre-decided calls in this region — four family owners and one body

This region is where the corpus's hub states four of the six repeated rules, and the
node brief settles every one of them. Apply the verdicts; do not re-decide.

- **Family B owner — `**Decompose.**` (L229–245).** **Triggering, `kinds=*`.** The
  corpus designates it: `driving.md` L595 reads *"`SKILL.md`'s Decompose step states
  the rule; this is the habit that honours it."* Its restatement in
  `driving.md` §*Externalizing surfaced work* is the **body**, and **row 18 is
  yours** — the second address into a unit `decompose-moves-k28` already rooted from
  `driving.md`'s framing unit.
- **Families C, D and E owner — the *"Whichever kind is running"* paragraph
  (L217–227).** **Triggering, `kinds=*`**, and **kept whole**. `batches-k13` told you
  to consider splitting it; `batches-k34` established that you cannot. It states four
  rules — raise ADRs sparingly, write a spec only at an agreement point, the ADR set
  is current-state, and the same rule governs `docs/specs/` — and the sentence
  boundaries fall **mid-line** (baseline L220 carries the end of the spec clause and
  the start of the ADR-set clause). Markers are whole unindented lines and this pass
  edits no prose, so the paragraph takes **one** class for all four rules. It is
  triggering because two of the four ship nowhere else at `kinds=*`: withhold *raise
  ADRs sparingly* and *write a spec at an agreement point* from the eighteen non-
  `design` kinds and you have manufactured an unasked question.
  **Record the fusion as a design finding in your body.** The grain of the
  classification is bounded by line boundaries here, and de-fusing the paragraph is a
  prose edit for a later grove, not a marking decision. It is the reason rows 20–22
  all leave from one unit.
- **Family A body — `**Review ownership inside a picked leaf.**` (L198–215).**
  **Procedural.** The owner is `TASK-FORMAT.md` §*In-session doubt is budgeted across
  the whole picked leaf* (#2), which carries the predicate and the five-row allowance
  table. **Row 17 is yours, and it is this paragraph's only root** — without it the
  unit fails reachability at the end of your batch.
  This paragraph states the predicate's **negative half** (*a `.grove/` directory in
  the checkout and inherited Grove control variables do not count*) that the owner
  does not. #2 and #7 both recorded that; add your own note so the aggregate reviewer
  sees it from all three sides.
  Its tail (L211–215, the escalated-review routing note) is about review target
  policy rather than the budget. Splitting it off is your judgement, not the family
  call; if you do, decide its class on its own merits.
- **`**Execute.**` (L167–196)** carries no family site. Triggering, `kinds=*` — the
  filename states the kind, and the closed set has nineteen members. Its three
  bullets describe *other* kinds' disciplines, which `kinds-k22` also states per
  kind; check its body for duplication before carving a second statement, and where
  the per-kind prose there ended up **procedural**, rows 24–25 are the addresses.

### Edge inventory rows owned: 17–25

Every target exists — `decompose-moves-k28` finished the last of them and said so in
its commit message.

| row | source (in your region) | target | note |
|---|---|---|---|
| 17 | `TASK-FORMAT.md` family-A owner | your *Review ownership* body | not optional; its only root |
| 18 | your `**Decompose.**` owner | `driving.md` §*Externalizing surfaced work* | second address into an already-rooted body |
| 19 | your `**Decompose.**` owner | `BRIEF-FORMAT.md` bodies (L237–238), `driving.md` §*What a good child leaf looks like* (L240) | |
| 20 | your L217–227 owner | `ADR-FORMAT.md` bodies (L217–218) | |
| 21 | your L217–227 owner | `SPEC-FORMAT.md` §*current-state* / membership / grain (L219–220) | |
| 22 | your L217–227 owner | `driving.md` §*Reworking ADRs and briefs…* (L220–224) | read `evidence-moves-k26`'s body for the id |
| 23 | your L217–227 owner (L224–227, *See `driving.md` for the field-guide habits…*) | `driving.md` grilling-moves bodies | the hub-side catch-all into `driving.md` |
| 24 | `**Execute.**` (L171–172) | `TASK-FORMAT.md` per-kind discipline bodies | conditional on `kinds-k22` having made any of them procedural; decline with that reason otherwise |
| 25 | `**Execute.**`'s `requirements` bullet (L176–179) | `grilling.md` bodies | |

Rows 20–23 all leave from the **same** unit, so its `defers=` is a multi-member
quoted list. That is legal and is the direct consequence of the fusion above.

### Scope

Nothing in this region is honestly narrower than `kinds=*`. `**Execute.**`'s
bullets describe per-kind discipline but are addressed to *every* session
deciding what a leaf of some kind will do — resist the pull to scope them, and if
you do scope one, say why. Remember there is no family shorthand: "every producer"
is spelled out in full or is `*`.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-skill-shapes` covers the rest of the file and nothing else, and carries
  no `defers=`.
- **Rows 17–25 are each reported** — written, or declined with a reason recorded in
  this leaf's body. Rows 18 and 19 target bodies that already have another inbound
  path, so the build stays green if either is dropped; the inventory is the net.
- `cargo build` and `cargo test` are green — this is the batch most likely to
  provoke the reachability, class and termination checks, because it writes the
  most edges.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The **fusion finding** on L217–227 is recorded, and the ids of the four family
  owners/bodies in this region are named for #10, #11 and #12.

## Notes

- 5,455 bytes is small for a batch by design. The cost here is the six-file edge
  set and four family owners, not the prose.
  **Do not absorb `pending-skill-shapes`** to fill the session — `shape-cutting-k30`
  has its own rows to reconcile.
- Doubts to carry forward, by id. You no longer *make* the overlap calls, so what
  belongs in the handoff is different and sharper: where a pre-decided verdict looked
  wrong once you had the prose open, and the fusion finding.
