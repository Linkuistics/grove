# shape-cutting-k30

## Goal

Classify **`content/SKILL.md` lines 247–406** (10,067 bytes): `**Cut the next
step, when it is needed.**` with the review chain and vendor pair bullets,
`**Neither shape gets a node directory.**`, `**Every step of a shape carries the
same bare stem**`, `**Declare the relationship in the body, by hand.**`,
`**The grammar is five fields; no relationship is one of them.**`, `**A chain is
not contiguous by construction…**`, and `**There is no exception to check.**`

This is batch 10 of 12.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve `content/SKILL.md` **L247–L406**, consuming the front of
  `pending-skill-shapes`.
- Mint exactly one residual, **`pending-skill-lifecycle`**, covering
  **L408–L760**.
- Redistribute any `defers=` inherited from `pending-skill-shapes`.

### Cross-file deferral

Both targets are already carved:

| `SKILL.md` site | target | carved by |
|---|---|---|
| L268 — *One narrow, unexpected doubt … may use its single in-session reviewer instead (`driving.md`)* | `driving.md` §*Doubting inside a picked Grove leaf* | `doubt-moves-k27` |
| L323 — *`TASK-FORMAT.md` carries the full reasoning* (bare stem) | `TASK-FORMAT.md` §*What the shapes are not* | `shapes-k23` |
| L327 — *`**Reviews:**` / `**Integrates:**` … (`TASK-FORMAT.md`)* | `TASK-FORMAT.md` | `shapes-k23` |

`shapes-k23`'s body names the unit holding the step-suffix reasoning; read it
rather than re-deriving it.

### The judgement this batch exists for

This region is unusually condition-rich, and the temptation is to classify the
whole thing triggering because it is all rules. Resist that — several of these
paragraphs are *justifications* of a rule stated elsewhere, and a justification is
not a condition.

- **`**Cut the next step, when it is needed.**`** is triggering, and the two
  bullets state the shapes' opposite construction. But `TASK-FORMAT.md`
  §*Composing the kinds* (`shapes-k23`) states the same asymmetry. Third
  overlap in this grove after Review-ownership and Decompose; **read
  `shapes-k23`'s and `doubt-moves-k27`'s recorded calls and stay consistent with
  them.** The mechanics — the exact `leaf-add` lines, who cuts what and when — are
  procedural.
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

- `content/SKILL.md` L247–406 is subdivided into real units;
  `pending-skill-lifecycle` covers L408–760 and nothing else.
- Every edge in the table above is written or explicitly declined with a reason.
- Any `defers=` inherited from `pending-skill-shapes` is redistributed and
  accounted for.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The consistency call against `shapes-k23` and `doubt-moves-k27` is recorded in
  this leaf's body.

## Notes

- Fenced blocks at L257–258 (indented `leaf-add` lines), L292 and L379–381
  (`text` fence). Do not split mid-fence.
- Doubts to carry forward, by id.
