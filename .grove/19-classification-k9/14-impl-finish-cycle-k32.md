# finish-cycle-k32

## Goal

Classify the **last of the corpus** (11,621 bytes) and close the classification:

- `content/SKILL.md` **L610–L760** — `**Finish.**` with its three numbered steps,
  `**Resume is state-checked, never a marker file**`, `**Ending after step 2 but
  before step 3…**`, `## Artifacts`, `## Specs`, `## Reference files` and the
  `linkuistics` prerequisite note.
- `content/prompts/continue.md` **whole** (838 bytes).

This is batch 12 of 12. Its **last act is to cut the aggregate `review-impl`
leaf** the node brief requires.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve `content/SKILL.md` **L610–L760**, consuming `pending-skill-finish` in
  full. Redistribute any `defers=` it inherited.
- Carve `content/prompts/continue.md`. It is the launcher framing, it is **all
  triggering**, and it creates **no procedural unit** — so it needs no root and
  carries no reachability obligation. The node brief says to leave its text alone;
  that still holds. One or two units at most.
- **Mint no residual.** After this batch,
  `grep -rc '<!-- unit: pending-' content/` must return **0**.

### Cross-file deferral — `## Reference files` is the one to think about

`## Reference files` (L735–744) names **every** embedded file in a bulleted index.
The reflex is to give it an eight-member `defers=`. Weigh that against the
alternative: an index is a *list of what exists*, and the real trigger→body edges
for each guide were written at the point of use by batches 4–11. An index that
duplicates eight edges already written adds eight paths to the same bodies and
says nothing new.

Either reading is defensible and neither breaks the build. **Decide explicitly and
record the reasoning** — the reviewer will look here first, because it is the
single place in the corpus where the deferral graph's shape is a free choice
rather than a consequence.

The other edges in this region are ordinary:

| site | target | carved by |
|---|---|---|
| L713 — *Keep it a glossary and nothing else … (`CONTEXT-FORMAT.md`)* | `CONTEXT-FORMAT.md` | `guides-k24` |
| L704, L733 — *Shape and the seam-sketching rule: `SPEC-FORMAT.md`* | `SPEC-FORMAT.md` | `guides-k24` |
| L703 — *philosophy per `linkuistics:decision-records`* | **not embedded** — never a `defers=` target | — |

The `linkuistics` prerequisite note (L746–760) points at three *plugin* skills.
None is in the embed, so none can be a `defers=` target. That the note is
unfollowable from the embed is a fact about the design, not a defect — but if you
think it reads as a dangling promise, that is a **finding**, and it belongs in
your body.

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
- **`## Specs`** restates `SPEC-FORMAT.md`'s membership and grain rules more
  tersely. Fourth and last overlap in this grove; `guides-k24` classified the
  `SPEC-FORMAT.md` side. Read its call and stay consistent.
- **`prompts/continue.md`** is the launcher the driver prepends to every mandate
  today. Under the successor grove it becomes `content/MANDATE.md`'s framing unit,
  but that is not this grove's work — classify it as it stands.

## Done when

- `content/SKILL.md` L610–760 and `content/prompts/continue.md` are subdivided
  into real units.
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
- The `## Reference files` decision is recorded with its reasoning.
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

**Write its body yourself, and give it four things:**

1. **`**Reviews:**`** naming all twelve batch handles — `spine-k21`, `kinds-k22`,
   `shapes-k23`, `guides-k24`, `research-moves-k25`, `evidence-moves-k26`,
   `doubt-moves-k27`, `decompose-moves-k28`, `execute-k29`, `shape-cutting-k30`,
   `lifecycle-k31`, `finish-cycle-k32` — so the reviewer inspects the **whole
   classification** rather than only the closing commit.
2. **The pre-classification baseline commit**, by id. It is the commit
   `batches-k13` produced — the last commit before `spine-k21` touched
   `content/`. Resolve it (`jj log` for the commit whose description names
   `batches-k13`) and write the actual change id into the body, not the handle
   alone.
3. **The assembled doubts**, by unit id, gathered from all twelve leaf bodies.
   Every batch was asked to record what it was least sure about precisely so this
   step is assembly rather than reconstruction. Group them by the kind of doubt —
   scope calls, condition/body splits, the four cross-file overlaps
   (Review-ownership, Decompose, ADR-reworking, Specs), and any prose flagged as
   neither condition nor procedure.
4. **What the build cannot check**, stated plainly, because that is what the
   reviewer is for: whether each unit **reads correctly standing alone** (the
   fence half is mechanical; the prose half is not), and whether a triggering
   condition was misfiled as procedural — the silent direction, which yields an
   unasked question and no diff.

## Notes

- If the twelve batches surfaced prose that is **neither a condition nor a
  procedure** — narrative that exists only to make the document readable — collect
  those findings here too. That is a finding about the *design*, and the review
  leaf is where it gets adjudicated.
- Do **not** retire `classification-k9` yourself. Retiring this leaf leaves the
  review leaf live, so the node stays open and the cascade does not fire — which
  is correct: the node's `Done when` includes the review having run.
