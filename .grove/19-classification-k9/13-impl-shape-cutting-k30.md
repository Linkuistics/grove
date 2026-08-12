# shape-cutting-k30

## Goal

Classify **`content/SKILL.md` from `**Cut the next step, when it is needed.**` to the
line before `**When a picked producer needs fresh review**`** (baseline L247–407,
10,068 bytes): `**Cut the next step, when it is needed.**` with the review chain and
vendor pair bullets, `**Neither shape gets a node directory.**`, `**Every step of a
shape carries the same bare stem**`, `**Declare the relationship in the body, by
hand.**`, `**The grammar is five fields; no relationship is one of them.**`, `**A
chain is not contiguous by construction…**`, and `**There is no exception to
check.**`

This is batch 10 of 12.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L247–407 is a baseline coordinate — batches #1 and #9
  have inserted markers above your region by the time you open the file.** Carve from
  `**Cut the next step, when it is needed.**` to the line **before** `**When a picked
  producer needs fresh review**`, consuming the front of `pending-skill-shapes`. Your
  region ends **including** the blank separator before that anchor, per the
  marker-placement convention — the F7 correction, which is why the baseline range is
  L247–407 and the size 10,068 rather than the L247–406 / 10,067 `batches-k13`
  recorded.
- Mint exactly one residual, **`pending-skill-lifecycle`**, covering `**When a picked
  producer needs fresh review**` to end of file, as `class=triggering kinds=*` **with
  no `defers=`**.
- **There is nothing to inherit from `pending-skill-shapes`.** A residual never
  carries `defers=`.

### The pre-decided call: `**Cut the next step…**` is a *body*

`batches-k13` called this paragraph triggering and asked you to reconcile it with two
sibling batches' recorded calls. The node brief settles it as **family F**:

- **The owner is `TASK-FORMAT.md` §*Composing the kinds — the two shapes*'s opening**
  (#3), which states *"reach for them by default, and argue yourself out of one rather
  than into it"* and *"they are built in opposite ways"*.
- **`**Cut the next step, when it is needed.**` (L247–268) is the body** for its
  restatement of that asymmetry — `class=procedural`, rooted from the owner, which is
  **row 32, yours to write**. Read `shapes-k23`'s body for the owner's id.
- What is *not* a restatement is still your judgement: *when* to decide for review
  ("the artifact is load-bearing — a spec, a decomposition you will build on for
  months, a subsystem") and the in-session-reviewer pointer at L266–268. If either
  reads as a condition in its own right, carve it as one and say why.

**The L266–268 in-session-reviewer pointer is a mention** of family A, not a site to
decide: its owner is `TASK-FORMAT.md` §*In-session doubt is budgeted…* (#2). An edge
from it into `driving.md` §*Doubting…* is legal and harmless — that unit is
procedural and already rooted by row 11 — but it is **not an inventory obligation**,
so write it only if the reference reads as a genuine trigger→body address.

### Edge inventory rows owned: 26, 27 and 32

| row | source (in your region) | target | note |
|---|---|---|---|
| 32 | `TASK-FORMAT.md` family-F owner | your `**Cut the next step…**` body | not optional; its only root |
| 26 | your `**Cut the next step…**` body | `TASK-FORMAT.md` §*The review chain* / §*The vendor pair* mechanics bodies | conditional: only where those units are procedural |
| 27 | `**Every step of a shape carries the same bare stem**` (L323), `**Declare the relationship…**` (L327) | `TASK-FORMAT.md` §*What the shapes are not* bodies | conditional the same way — `shapes-k23`'s body names the unit holding the step-suffix reasoning; **read it rather than re-deriving it** |

### The judgement this batch exists for

This region is unusually condition-rich, and the temptation is to classify the
whole thing triggering because it is all rules. Resist that — several of these
paragraphs are *justifications* of a rule stated elsewhere, and a justification is
not a condition.
- **`**Every step of a shape carries the same bare stem**`** is a naming rule
  (triggering: a session that does not know it will suffix its slugs) *plus* a
  long justification of why the step suffix was deleted (not a condition). Split
  it.
- **`**The grammar is five fields; no relationship is one of them.**`** is the
  paragraph most worth getting right. *Grove infers no relationship between
  leaves* is a genuine condition: a session that assumes an `X` requires a
  `review-X` after it cuts leaves it does not need, and nothing errors. The
  five-field enumeration behind it is procedural.
- **`**A chain is not contiguous by construction…**`** (3,314 bytes) is the
  largest block here. Its condition is *an integration consumes, so a gap
  corrupts*; its body is the directory-local placement rule with the three
  clarifying bullets and the fenced `leaf-insert` line. The asymmetry is stark and
  the split should be clean.
- **`**There is no exception to check.**`** is 464 bytes of pure justification for
  the preceding rule. If it is neither condition nor procedure, **say so as a
  finding** rather than forcing it — but note that *"a session that departs anyway
  owns the drift"* reads as a condition, so read it twice.

### Scope

`kinds=*` throughout, almost certainly. The chain and pair rules are addressed to
whichever session is deciding to cut the next step, which can be any kind.
`**A chain is not contiguous…**` speaks most directly to `review-*` and
`integrate-review-*` sessions, but a producer cutting the first step needs the
same condition — scoping it to the review kinds would withhold it from exactly the
session that opens the chain.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-skill-lifecycle` covers the rest of the file and nothing else, and carries
  no `defers=`.
- **Rows 26, 27 and 32 are each reported** — written, or declined with a reason.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- Whatever inside `**Cut the next step…**` you carved as a condition in its own
  right — rather than as part of the family-F body — is named with its reasoning.

## Notes

- Fenced blocks at L257–258 (indented `leaf-add` lines), L292 and L379–381
  (`text` fence). Do not split mid-fence.
- Doubts to carry forward, by id.
