# task-kind-taxonomy-k3 — brief

## Goal

Replace grove's closed set of **five** task kinds with a **seventeen**-kind set
that names the disciplines a cross-vendor workstream actually uses, and give
routing the two mechanisms those kinds need: a **policy** axis (kind → harness,
with family fallback) and a **per-leaf** axis (one leaf's harness, declared on
the leaf).

This node was decomposed from a planning leaf originally scoped as
"should harness selection move from per-grove env onto the leaf?". That framing
was wrong, and the grilling that reframed it is recorded under **Decisions**
below — read that section before the children; it is the whole of the design.

## Done when

- `leaf::Kind` carries the seventeen kinds; `--kind` gates on write and reading
  degrades, exactly as today.
- `work` is renamed `impl` without breaking any live grove's task files.
- Routing resolves a **family** (`review-*`, `integrate-review-*`) so a
  one-line policy covers all five of a family's kinds.
- A leaf can name its own harness, and the loop launches it there.
- `TASK-FORMAT.md`, `CONTEXT.md`, README and `--help` describe the new set; ADRs
  *task-kind-taxonomy* and *model-per-task-kind* are reworked **in place**.

## Decomposition

Position order is dependency order. `01` settles the written design before any
code moves; `02`–`04` are independent vertical slices, each landing green on its
own; `05` is the sweep that cannot run until the rest exists.

- `01` **taxonomy-spec** (planning) — the spec and the ADR reworks. Everything
  else cites it.
- `02` **kind-set** (work) — the enum, the env suffixes, and the `work` → `impl`
  rename. Demoable alone: seventeen kinds exist and route to the stamp.
- `03` **family-fallback** (work) — the family axis in `harness_override` and
  `model_for`.
- `04` **leaf-harness** (work) — the `**Harness:**` line, its peek, and its
  refusal semantics.
- `05` **config-sweep** (work) — live env migration and the doc surface.

## Pointers

- ADRs a session here must read: *task-kind-taxonomy* (the closed-set argument
  this node rewrites), *model-per-task-kind* (the routing mechanism it extends),
  *self-driving-loop*.
- Glossary terms in play: Task kind, Per-kind model selection, HITL/AFK.
- Both ADRs are reworked **in place** — merge / split / delete, never a
  superseding record (`linkuistics:decision-records`).
- The behavioural contracts this node depends on, stated without line numbers so
  they survive the code moving:
  - The loop driver **peeks the picked leaf's kind** before launching, and only
    when some routing env makes it matter — the unconfigured path stays a
    zero-subprocess launch.
  - A **rerouted** launch (launch harness ≠ stamped harness) must never inherit
    an unscoped value — not the base model var, not the global binary override.
    A codex profile name is garbage to pi.
  - Kind **reading degrades** (unrecognised ⇒ `work`, warn) but harness routing
    **refuses**: when the kind peek fails while a harness override is
    configured, the driver bails rather than launch on the wrong vendor. Model
    selection is a nicety; a misroute is not.

## Notes

**The user's actual configuration**, which is what the design was derived
against and what `05` must produce: claude leads, codex reviews, claude
integrates the review; research runs claude + codex, combined by claude, codex
or kimi. Everything on claude needs no configuration at all — it falls through
to the stamp. So the whole policy layer is two lines
(`GROVE_REVIEW_HARNESS=codex` plus the matching model var), and only the
research pair's second leaf and its combine step ever carry a per-leaf
declaration.

**Independent of the herdr subtree.** Nothing here touches herdr.

### Still open

- **Is the grammar enforced or only documented?** Recommended: *documented*.
  Sibling **positions are mutable** (`leaf-insert` shifts every later sibling),
  so a rule over sibling order would be invalidated by the one verb that exists
  to reorder work — grove would either re-validate whole subtrees on every
  insert or silently stop enforcing. The closed set already buys what mattered
  (kind inheritance on decompose, a model bucket per kind); sequence validation
  earns nothing further. Not confirmed by the user.
- **HITL/AFK per kind.** `requirements` is HITL (it holds the grilling).
  `planning`, stripped of grilling, is plausibly **AFK** — "given the spec, cut
  it into vertical slices" is something `driving.md` already gives an agent
  enough rules to do alone. That flips a row in the existing table and should be
  decided explicitly in `01`, not assumed.
- **Per-leaf model.** Rejected for now: every model in use is distinguishable by
  harness alone (claude ⇒ claude, gpt ⇒ codex, kimi ⇒ pi), so a `**Model:**`
  axis would be machinery for a case that is not live (constraint 4). Revisit
  the day one model family genuinely runs on two harnesses. Additive when it
  comes — a second optional line, not a design to unpick.

## Decisions

Settled during the grilling that produced this node. Ordered by topic, not
chronology.

### What the kinds are

**The kinds are parameterised, not flat.** Five producers — `requirements`,
`design`, `planning`, `prototype`, `impl` — each with its own `review-` and
`integrate-review-` step, plus `research` and `combine-research`: **seventeen**
kinds. A flat set (one `review`, one `integrate-review`) was recommended and
rejected. `planning` is reviewable — reviewing a decomposition ("are these
slices vertical, is anything missing") is a genuinely different read from
reviewing code, and that is the concession that carries parameterisation past
*task-kind-taxonomy*'s distinct-discipline bar.

**`design` is a new kind, not a rename of `planning`.** `planning` keeps the
methodological force it has today — the sole branch in the loop's Execute step,
the only kind that grows the tree. `design` is a *producer* whose artifact (a
spec, an ADR set) is reviewable, which a tree-growth session is not.

**`requirements` splits from `design`.** A requirements gatherer (*what* should
be built) is distinct from an architect/designer (*how*, given a spec). This
carves the grilling half out of today's `planning`, which currently does both.

**"Only `planning` grows the tree" survives, restated.** Two different things
were being called tree-growth. *Reactive* decomposition — a leaf proves bigger
than its brief and splits itself — is already **kind-agnostic**, and
`leaf-decompose` inherits the parent's kind, so a large-scope `requirements`
leaf becomes a `requirements` node with narrower `requirements` children, with
no new machinery. *Generative* decomposition — a session whose **deliverable is
the tree** — is `planning`, and only `planning`. The boundary rule: a `design`
session that finds itself cutting *implementation* leaves has drifted into
planning's job and should externalize a `planning` leaf instead.

**Rename `work` → `impl`.** `work` names both a member of the set and the
category containing it — `CONTEXT.md` and `TASK-FORMAT.md` both call the
non-planning kinds "work-shaped sessions".

### The two patterns

**There are two distinct patterns, not one grammar.** They differ in shape and,
crucially, in which routing mechanism they need.

- **The review chain** — `X` → `review-X` → `integrate-review-X`. Sequential,
  and **each step is a different kind**, so per-kind routing alone expresses it.
  This is the pattern that answers "reviews go to codex, integration goes back
  to the implementer."
- **The vendor pair** — `research-A` → `research-B` → `combine-research`. The
  two producers are **the same kind differing only by vendor**, which is exactly
  what a kind→harness *function* cannot express, so this is the pattern that
  requires a per-leaf declaration. Two, not N: the fan is a *pair*, so the
  combine step is binary and nothing needs to generalise past it.

The two mechanisms map one-to-one onto the two patterns. Neither is redundant.

**The patterns differ in character, not only in shape.** The review chain is
**adversarial** — the reviewer's job is to find fault. The research pair is
**breadth-and-confirmation** — two independent surveys, unioned.

**`research` stays breadth-seeking; `combine-research` carries the adversarial
move.** Two vendors on overlapping corpora can agree on something false, so a
purely confirmatory combine raises confidence exactly where it should lower it —
a correlated error laundered as corroboration. Running the *researchers*
adversarially does not fix that and discards the breadth the pair was run for.
So `combine-research`'s discipline is: union the coverage, flag every
disagreement, and treat **agreement without independent primary sourcing as a
red flag, not a confirmation** — the one check neither survey can perform on
itself.

### Routing

**Both mechanisms stay, because they answer different questions.**
`GROVE_<KIND>_HARNESS` is a **policy** — one rule, every grove, no tree knows
about it ("reviews go to codex, because that is what I pay for"). A leaf-level
declaration is a **fact about one leaf** — this one goes elsewhere *because its
sibling does not*. Per-leaf harness was first recommended for deletion, then
revived by the multi-vendor research case.

**Routing keys on a family, not only the full kind.** `GROVE_REVIEW_IMPL_HARNESS`
beats `GROVE_REVIEW_HARNESS`. Two families exist (`review-*`,
`integrate-review-*`); the other seven kinds stand alone. This is not a new
concept — grove already runs "specific beats general" on the harness axis
(`GROVE_<HARNESS>_<KIND>_MODEL` beats `GROVE_<KIND>_MODEL`); the family axis
extends the same rule along the kind axis. Without it the seventeen-kind set
would need the same policy written five times and hand-kept in sync.

The cost, to be paid honestly in `01`: *model-per-task-kind* states "no
fallback" as a load-bearing rule and rejects chains outright. That rejection
targeted falling back *across* kinds (research → work), which silently
downgrades to a model never chosen for that kind. A fallback *within* a family
the user explicitly configured is different in character — but it is still a
chain, so the ADR is reworked in place to carve out the family axis.

**The leaf names the seat; the env names who sits in it.** A per-leaf
declaration selects a harness; the model for that (harness, kind) pair still
comes from the environment.

**The 85-var surface is a ceiling, not a burden.** Seventeen kinds × five vars
looks alarming, but the stamp absorbs every kind that is not rerouted, so a real
configuration is two lines plus two leaf declarations.

### Rejected

**Parallelism.** `run_loop` launches one foreground session owning the real TTY
and watches one signal file, so N-vendor work is expressed as *sequential leaves
that do not read each other's output*, plus a combine step. Behaviourally
identical, since grove sessions share no context anyway; real concurrency would
need separate workspaces and separate loops.

**"The reviewer must not be the author."** Raised during the grill — a
codex-stamped grove reviews its own work under a global
`GROVE_REVIEW_HARNESS=codex` — and deliberately not adopted. grove expresses no
relation *between* leaves, and the payoff is the same unquantified one
*model-per-task-kind* already declined to buy.

**Reviews-run-on-codex is a commercial decision, not a methodological one.** The
driver is the subscription being paid for, not reviewer-bias mitigation. So
*model-per-task-kind*'s rejection of cross-family selection is untouched by this
node — this is a plain per-kind route to a harness already in the table, not a
re-opening of it.
