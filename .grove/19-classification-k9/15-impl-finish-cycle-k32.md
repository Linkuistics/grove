# finish-cycle-k32

## Goal

Classify the **last of the corpus** (11,621 bytes) and close the classification:

- `content/SKILL.md` **from `**Finish.**` to end of file** (baseline L610–760,
  10,783 bytes) — `**Finish.**` with its three numbered steps,
  `**Resume is state-checked, never a marker file**`, `**Ending after step 2 but
  before step 3…**`, `## Artifacts`, `## Specs`, `## Reference files` and the
  `linkuistics` prerequisite note.
- `content/prompts/continue.md` **whole** (838 bytes).

This is batch 12 of 12. Its **last act is to cut the aggregate `review-impl`
leaf** the node brief requires.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L610–760 is a baseline coordinate — four batches have
  inserted markers above your region by now.** Carve from `**Finish.**` to end of
  file, consuming `pending-skill-finish` in full. Yours is the one `SKILL.md` region
  the F7 separator correction does not touch: it ends at EOF, and the blank line
  before `**Finish.**` belongs to `lifecycle-k31`.
- **There is nothing to inherit from `pending-skill-finish`.** A residual never
  carries `defers=`.
- Carve `content/prompts/continue.md`. It is the launcher framing and it is **all
  triggering** — but it is **not** edge-free: it is the root for `## Reference files`
  (row 31, below). The node brief says to leave its text alone; that still holds.
  One or two units at most.
- **Mint no residual.** After this batch,
  `grep -rc '<!-- unit: pending-' content/` must return **0**.

### `## Reference files` — settled, and no longer your call

`batches-k13` left this to you as a free choice between eight unconditional
`defers=` members and none. `batches-k33` F6 found that a false binary — both options
evade the classification question — and the node brief settles it:

**`## Reference files` (L735–744) is `class=procedural`, rooted from
`prompts/continue.md`'s framing unit (row 31), and it writes no `defers=` of its
own.** The decisive argument is neither size nor redundancy: **the index's rows name
files, and a session cannot fetch a file.** `grove-llm methodology` addresses units
by id, so an index of filenames delivered into a mandate promises navigation the
delivery path cannot honour — while every genuine trigger→body edge for those guides
was written at its point of use by batches #4–#12 anyway.

Its eight filename mentions are a **standing sweep exclusion**: they will show up in
every `grep -rn '<F>' content/` and none of them is a trigger→body edge.

**Carry it to the aggregate review as a design finding**, which the node brief asks
for in these words: the index is narrative residue of the provisioned-skill era — it
exists so a reader of a skill *directory* knows what sits beside `SKILL.md`, and
mandate delivery replaces that job with `grove-llm methodology`'s listing. Whether it
survives at all is the successor grove's call.

**The `linkuistics` prerequisite note (L746–760) is a separate unit,
`class=triggering kinds=*`, with no `defers=`.** It states a genuine condition — a
session raising an ADR, sketching a spec's seams, or driving a jj-enabled tree should
consult the matching plugin skill — and its three targets are not embedded, so none
can be a `defers=` target. That the note is unfollowable from the embed is a fact
about the design, not a defect; if you think it reads as a dangling promise, that is
a **finding** for your body.

### `prompts/continue.md`'s Decompose reference is deliberately edgeless

`batches-k33` F3 read `continue.md` L6 — *"see the skill's Decompose step"* — as a
lost edge, on the grounds that it is a cross-file trigger→body reference the filename
grep cannot find. **The mechanism is real and is why the edge inventory exists; this
particular reference owes nothing.** Its target is `SKILL.md` `**Decompose.**`, which
`execute-k29` carves as family B's **owner** — a triggering unit. A `defers=` naming a
triggering unit is a build error, and none is needed: the condition ships in every
mandate. Record that reasoning rather than writing the edge.

The same test applies to `continue.md`'s other references (*"use the grove skill"*,
`grove-llm complete`, the handle rule): a reference to a condition needs no address.

### Edge inventory rows owned: 29, 30 and 31

| row | source | target | note |
|---|---|---|---|
| 29 | `## Artifacts`'s glossary paragraph (L707–713, *Keep it a glossary and nothing else*) | `CONTEXT-FORMAT.md` bodies | `guides-k24` carved the target; `guides-k24` reported this hit as *not yours* |
| 30 | `SKILL.md` L217–227 (family C/D/E owner, `execute-k29`) | your `## Specs` body | not optional; its only root |
| 31 | `prompts/continue.md`'s framing unit | your `## Reference files` body | not optional; its only root |

L703's *philosophy per `linkuistics:decision-records`* is **not embedded** and can
never be a `defers=` target.

### The judgement this batch exists for

- **`**Finish.**`** is HITL and states the loop's **only routine human gate**:
  the session proposes teardown and waits for explicit confirmation, and a
  headless run reports and stops. That is triggering and cannot be anywhere else.
  The three numbered steps — `finish-commit`, the `Recovery pending` handling,
  `complete --done` — are procedural, and `kinds=finish` is a genuine candidate
  scope for them, which makes this one of the very few places an explicit scope
  list is honest. But read `**Finish.**`'s opening again before scoping: *"You do
  not discover that a grove is finished — the driver does"* is addressed to every
  kind, because it tells a non-`finish` session **not** to go looking.
- **`## Artifacts`** is the four-row table plus *the glossary is load-bearing*.
  The glossary paragraph states terminology drift as the acute failure mode of
  multi-session work — a condition every session needs.
- **`## Specs`** (L720–733) restates `SPEC-FORMAT.md`'s membership and grain rules
  more tersely. **Settled: it is a family-D body, `class=procedural`**, rooted from
  `SKILL.md` L217–227 (row 30). Its opening sentence restates the condition
  ("produced lazily by a `design` task *when the increment is a genuine agreement
  point*"), which ships from three other places — the owner, `TASK-FORMAT.md`'s
  `design` bullet, and `SPEC-FORMAT.md`'s opening — so do not carve it out as a
  fourth. Its *grain* is still yours.
- **`prompts/continue.md`** is the launcher the driver prepends to every mandate
  today. Under the successor grove it becomes `content/MANDATE.md`'s framing unit,
  but that is not this grove's work — classify it as it stands. Note that its framing
  unit is the root for `## Reference files`, which is exactly the job
  `MANDATE.md`'s framing unit inherits: *here is what you are holding, and here is how
  the rest is served*.

## Done when

- `content/SKILL.md` from `**Finish.**` to end of file, and
  `content/prompts/continue.md`, are subdivided into real units.
- **Rows 29, 30 and 31 are each reported** — 30 and 31 written (they are their
  targets' only roots), 29 written or declined with a reason.
- The `## Reference files` and `## Specs` verdicts are applied as settled, and the
  `## Reference files` design finding is carried into the aggregate review handoff.
- **`grep -rc '<!-- unit: pending-' content/` returns 0.** Run it and paste the
  result into the commit message; it is the mechanical statement that the
  classification is finished rather than merely green.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately and
  the last `pending-` id removed.
- **`grove-llm methodology` is verified out of a rebuilt, installed binary**: the
  listing shows the real classification, and spot-fetching a triggering unit shows
  a `defers=` target that answers it. This is the node brief's `Done when` and
  this batch is where it is checked — the eleven batches before it verified
  through the module seam.
- **The full edge inventory is assembled from all twelve leaf bodies** — every row
  written, declined with a reason, or added by a batch that found an unlisted edge —
  and carried into the review handoff. This is assembly, not reconstruction: each
  batch reported its own rows.
- **The aggregate `review-impl` leaf is cut** — see below.

## The aggregate review, which is this leaf's last act

The node brief requires it, and it is not optional in practice: this
classification is the artifact the successor grove's composer and golden snapshots
are built on, and a misclassification that survives is baked in behind bytes that
look stable.

Cut it beside this leaf:

```
grove-llm leaf-add classification-k9 classification --kind review-impl
```

`leaf-add` is correct here — a `review-*` step re-derives its citations from the
producer's commit, so it needs no `leaf-insert` care, and no sibling entry
after this leaf holds live work.

**Write its body yourself, and give it six things:**

1. **`**Reviews:**`** naming all twelve batch handles — `spine-k21`, `kinds-k22`,
   `shapes-k23`, `guides-k24`, `research-moves-k25`, `evidence-moves-k26`,
   `doubt-moves-k27`, `decompose-moves-k28`, `execute-k29`, `shape-cutting-k30`,
   `lifecycle-k31`, `finish-cycle-k32` — so the reviewer inspects the **whole
   classification** rather than only the closing commit.
2. **The pre-classification baseline commit**, by id. It is the commit that retires
   **`batches-k34`** — the last commit before `spine-k21` touches `content/`. **Not
   `batches-k13`'s**, which `batches-k13` itself named: `batches-k33` and
   `batches-k34` both land after it, so a diff from there carries planning churn.
   The corpus bytes are identical across all three, so this is about giving the
   reviewer a clean diff, not about which bytes were classified. Resolve it (`jj log`
   for the commit whose description names `batches-k34`) and write the actual change
   id into the body, not the handle alone.
3. **The assembled doubts**, by unit id, gathered from all twelve leaf bodies.
   Every batch was asked to record what it was least sure about precisely so this
   step is assembly rather than reconstruction. Group them by the kind of doubt —
   scope calls, condition/body splits, and any prose flagged as neither condition nor
   procedure. **The six repeated-rule families are a different kind of item now**: no
   batch decided them, so what the reviewer needs is not twelve calls to reconcile
   but the *pre-decided* verdicts plus every note a batch recorded that made a verdict
   look wrong with the prose open.
4. **The assembled edge inventory** — all thirty-plus rows with their outcomes, and
   any row a batch added. The reviewer's question there is the one no build asks: does
   each written edge address a body its source's condition actually raises, and does
   each *declined* row deserve its decline?
5. **What the build cannot check**, stated plainly, because that is what the
   reviewer is for: whether each unit **reads correctly standing alone** (the
   fence half is mechanical; the prose half is not), and whether a triggering
   condition was misfiled as procedural — the silent direction, which yields an
   unasked question and no diff.
6. **The three design findings this plan already knows about**, so the reviewer
   adjudicates them rather than rediscovering them: the `SKILL.md` L217–227 **fusion**
   (four rules in one unsplittable paragraph, which is why one unit owns families C, D
   and E), the **`## Reference files` index** as narrative residue of the
   provisioned-skill era, and whatever the twelve batches flagged as neither condition
   nor procedure.

## Notes

- If the twelve batches surfaced prose that is **neither a condition nor a
  procedure** — narrative that exists only to make the document readable — collect
  those findings here too. That is a finding about the *design*, and the review
  leaf is where it gets adjudicated.
- Do **not** retire `classification-k9` yourself. Retiring this leaf leaves the
  review leaf live, so the node stays open and the cascade does not fire — which
  is correct: the node's `Done when` includes the review having run.
