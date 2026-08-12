# classification-k9 — brief

## Goal

Replace the trivial marking with the **real** one: subdivide `content/`'s
one-unit-per-file partition into the actual triggering/procedural split, with
`kinds=` scopes and `defers=` targets, across ~139 kB of embedded markdown.

This is the load-bearing, judgement-heavy work the grilling earmarked — wrong in
a way no compiler sees. It is a **pure `content/` edit**: no Rust changes, and
every existing unit is only ever subdivided, never merged or moved between files.

**This was a single leaf until `increments-integrate-k12`.** `increments-review-k11`
B5 found it larger than one focused session — one context asked to classify
139,136 bytes across nine files, make every unit-boundary and scope/deferral
judgement, preserve cross-file reachability, update the pinned complete id set,
verify the installed listing, and author the aggregate review handoff. The three
largest files alone are 51,524, 41,954 and 24,894 bytes. It is now a node, and its
first child is the `planning` leaf that derives the batches.

## The rule, and it is the only rule

**Keep the `if`, defer the `then`.** A rule's *condition* — that a situation
exists calling for something other than what this session is doing — is
`class=triggering` and ships in every mandate its scope admits. Its *body* — how
to act once that is decided — is `class=procedural`, ships in no mandate, and is
reached through `defers=`.

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
| `content/SKILL.md` | 51,524 | the spine and the loop; the highest density of conditions, and the finest grain — a kind's discipline is one bullet out of nineteen |
| `content/driving.md` | 41,954 | mostly procedural: habits and moves, entered once a session has decided to do the thing |
| `content/TASK-FORMAT.md` | 24,894 | the kind taxonomy and the filename grammar; `kinds=`-scoped units live here if anywhere |
| `content/SPEC-FORMAT.md` | 5,312 | format guide — largely procedural bodies behind a triggering "when you need one" |
| `content/grilling.md` | 4,686 | vendored; procedural, entered from `requirements` |
| `content/BRIEF-FORMAT.md` | 4,515 | format guide |
| `content/CONTEXT-FORMAT.md` | 3,447 | vendored; format guide |
| `content/ADR-FORMAT.md` | 2,015 | format guide, mostly a pointer to `linkuistics:decision-records` |
| `content/prompts/continue.md` | 789 | still live and still true under provisioning; leave its text alone |

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

## Why this is a node, and what the batching has to respect

**A blind one-file-per-child split is not automatically green.** The gate
`embed-wide-gate-k8` lands enforces `defers=` resolution, target class, and
procedural reachability *across the whole embed*. So a newly classified triggering
unit in one file may defer to a procedure whose file is still represented by a
single trivial `class=triggering kinds=*` unit — and the build rejects that twice
over: the target does not exist, and if a same-named one did it would be of the
wrong class. `content/SKILL.md` defers heavily into `driving.md`, `grilling.md`
and `TASK-FORMAT.md`, so this is the normal case, not a corner.

Batch by **deferral closure and session size**, not by filename. Every child must
leave `cargo build` and `cargo test` green on its own commit — that is the whole
reason the batching is planned before any of it is executed.

## Done when

- Every embedded markdown file is subdivided into real units; `cargo build` and
  `cargo test` are green, including reachability across the whole embed.
- The pinned complete id set in the test constant is updated deliberately — every
  new id named by a human decision, which is exactly the moment the design
  intends the classification to be confirmed.
- `grove-llm methodology` (from `addressable-embed-k7`) lists the real
  classification, and spot-fetching a triggering unit shows a `defers=` target
  that answers it.
- An **aggregate `review-impl` leaf** has run over the whole classification —
  cut inside this node, after the final batch, with a body naming the pre-
  classification baseline commit and **every** batch child's `<slug>-k<key>`
  handle, so the reviewer inspects the whole classification rather than only the
  closing commit. This is the leaf the design earmarked; the final batch's
  producer cuts it, and decides *what* to point the reviewer at, not *whether* to.

## Notes

- The review is not optional here in practice: the classification is the artifact
  the successor grove's composer and golden snapshots are built on, and a
  misclassification that survives is baked in behind bytes that look stable. Give
  the reviewer the units each batch was least sure about, by id — carry those
  doubts forward from child to child so the final batch can assemble them rather
  than reconstruct them.
- The single in-session reviewer a picked producer may spend is for one narrow,
  unexpected doubt — not for a second opinion on a batch. That is what the
  aggregate `review-impl` leaf is for.
- Verification here is structural by design; behavioural evaluation was
  considered and rejected during grilling. The honest behavioural check is the
  first real grove run after the successor grove ships, with a human watching —
  which is also why this classification wants to be *released and auditable*
  before a composer is written over it.
- If subdividing turns up prose that is neither a condition nor a procedure —
  narrative that exists only to make the document readable — say so rather than
  forcing it into a class. That is a finding about the design, and it belongs in
  a leaf, not in a marker.
- `content/SKILL.md` and `content/TASK-FORMAT.md` are edited by
  `step-suffix-redundancy-k10` and whatever it cuts, both of which run **before**
  this node. Classify the prose as it then stands; if either file still has
  uncommitted suffix work outstanding when a batch reaches it, that is a
  sequencing fault worth stopping on rather than classifying around.
