# classification-k9

## Goal

Replace the trivial marking with the **real** one: subdivide `content/`'s
one-unit-per-file partition into the actual triggering/procedural split, with
`kinds=` scopes and `defers=` targets, across ~139 kB of embedded markdown.

This is the load-bearing, judgement-heavy leaf the grilling earmarked — wrong in
a way no compiler sees. It is a **pure `content/` edit**: no Rust changes, and
every existing unit is only ever subdivided, never merged or moved between files.

## Context

The rule, and it is the only rule: **keep the `if`, defer the `then`.** A rule's
*condition* — that a situation exists calling for something other than what this
session is doing — is `class=triggering` and ships in every mandate its scope
admits. Its *body* — how to act once that is decided — is `class=procedural`,
ships in no mandate, and is reached through `defers=`.

The asymmetry is the whole argument, so classify against it directly: withholding
a procedural body costs a lookup the session knows to make; withholding a
triggering condition yields an **unasked question**, which is grove's primary
failure mode — a session quietly absorbing work that should have been its own
leaf. When a unit is genuinely ambiguous, that asymmetry is the tie-breaker, not
size and not frequency.

`docs/adr/mandate-delivers-the-methodology.md` and the spec's *Keep the `if`,
defer the `then`* carry the argument in full; `CONTEXT.md`'s
[[Triggering unit]] / procedural unit entry carries the two `_Avoid_` traps.

### The corpus, largest first

| file | bytes | what to expect |
|---|---|---|
| `content/SKILL.md` | ~51 kB | the spine and the loop; the highest density of conditions, and the finest grain — a kind's discipline is one bullet out of nineteen |
| `content/driving.md` | ~42 kB | mostly procedural: habits and moves, entered once a session has decided to do the thing |
| `content/TASK-FORMAT.md` | ~25 kB | the kind taxonomy and the filename grammar; `kinds=`-scoped units live here if anywhere |
| `content/SPEC-FORMAT.md`, `content/BRIEF-FORMAT.md`, `content/grilling.md`, `content/CONTEXT-FORMAT.md`, `content/ADR-FORMAT.md` | ~2–5 kB each | format guides — largely procedural bodies behind a triggering "when you need one" |
| `content/prompts/continue.md` | ~0.8 kB | still live and still true under provisioning; leave its text alone |

`kinds=*` is the overwhelming default. An explicit list is for guidance genuinely
about one kind's discipline — and there is no family shorthand and no negation, so
a list that wants to say "every producer" is spelled out or is `*`.

### What will be tempting and is wrong

- **Classifying by size.** A long conditional is still a condition.
- **Classifying by frequency.** The test is whether the session could know to
  *ask*, not how often it needs the answer.
- **Leaving a triggering unit with no `defers=` when its body moved.** Absence of
  `defers=` is meaningful — it tells the session the unit is complete as
  delivered. A condition whose procedure went to another unit and does not name it
  is exactly the "unasked question" in miniature.
- **Reading a unit's own id as the address of its deferred body.** One namespace
  covers both classes, so that id fetches the unit again.
- **Splitting mid-fence.** The parser forbids it, so the build will say so — but
  the authoring rule behind it is the one no build checks: a unit must read
  correctly standing alone, because a mandate is units joined by a blank line and
  nothing else.

## Done when

- Every embedded markdown file is subdivided into real units; `cargo build` and
  `cargo test` are green, including reachability across the whole embed.
- The pinned complete id set in the test constant is updated deliberately — every
  new id named by a human decision, which is exactly the moment the design
  intends the classification to be confirmed.
- `grove-llm methodology` (from `methodology-verb-k8`) lists the real
  classification, and spot-fetching a triggering unit shows a `defers=` target
  that answers it.
- A `review-impl` leaf is cut, with the specific doubts written into its body.
  **This is the leaf the design earmarked**, and the producer's last act is to cut
  it — decide *what* to point the reviewer at, not *whether* to.

## Notes

- The review is not optional here in practice: the classification is the artifact
  the successor grove's composer and golden snapshots are built on, and a
  misclassification that survives this leaf is baked in behind bytes that look
  stable. Give the reviewer the units you were least sure about, by id.
- The single in-session reviewer a picked producer may spend is for one narrow,
  unexpected doubt — not for a second opinion on 139 kB. That is what the
  `review-impl` leaf is for.
- Verification here is structural by design; behavioural evaluation was
  considered and rejected during grilling. The honest behavioural check is the
  first real grove run after the successor grove ships, with a human watching —
  which is also why this classification wants to be *released and auditable*
  before a composer is written over it.
- If subdividing turns up prose that is neither a condition nor a procedure —
  narrative that exists only to make the document readable — say so rather than
  forcing it into a class. That is a finding about the design, and it belongs in
  a leaf, not in a marker.
