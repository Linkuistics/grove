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
code moves; `02`–`05` are vertical slices, each landing green on its own (`04`
needs `03`'s lattice first); `06` is the sweep that cannot run until the rest
exists.

- `01` **taxonomy-spec** (planning) — the spec and the ADR reworks. Everything
  else cites it. **Done** — `docs/specs/task-kind-taxonomy.md` is the authority
  for `02`–`06`; both ADRs are reworked in place.
- `02` **kind-set** (work) — the enum, the env suffixes, and the `work` → `impl`
  rename. Demoable alone: seventeen kinds exist and route to the stamp.
- `03` **family-fallback** (work) — the family axis in `harness_override` and
  `model_for`, resolved **harness-major** (spec, *Routing*).
- `04` **required-model-vars** (work) — a kind that resolves no model var fails
  loudly. Added by `01`; sequenced after `03` because "required" is defined
  against the four-key lattice, and the ~9-var figure only holds once families
  exist. Mostly test blast radius.
- `05` **leaf-harness** (work) — the `**Harness:**` line, its peek, and its
  refusal semantics.
- `06` **config-sweep** (impl) — live env migration and the doc surface.
  **Done.** Both routing axes and the seventeen kinds now reach `--help`,
  `TASK-FORMAT.md`, `SKILL.md`, `driving.md`, `README.md`, `docs/grove.md` and
  the CHANGELOG; the live env is rewritten harness-scoped across all three
  harnesses and verified by 51 real launches. It surfaced `07` and `08`, so this
  node is **not** done.

Added by `06`, both consequences of `04`'s inversion rather than of the original
plan. Neither is a doc fix; both change launch-path behaviour or a recorded
contract, which is why they are leaves and not part of the sweep.

- `07` **bootstrap-leaf-kind** (impl) — `root-init` minted `planning` while
  `start.md` had that session **grill**, so a fresh grove did `requirements` work
  under a `planning` label. **Done** — settled as **`requirements`**, on the
  HITL generating rule rather than on the prompt's wording: a brand-new grove has
  nothing on disk but the human's own words, so keeping `planning` would have
  meant re-marking it HITL or exempting the bootstrap from a mark the spec
  insists is rule-generated. `root-init` takes no `--kind` (the `start` launch is
  routed before the verb runs), and *fresh-grove-start-contract* now carries the
  answer with the rejected reading and its reopening condition.
  **`GROVE_REQUIREMENTS_MODEL` is now the first-run var**, named in `README.md`.
- `08` **no-launch-config-check** (impl) — `--no-launch` returned before both
  config checks, so it printed `ready` and exited 0 where the next real launch
  failed on a missing model var. **Done.** Pre-flight is hoisted above the
  no-launch return and the dry run now calls the *same* `resolve_launch` the
  loop's next iteration would, so it fails on exactly what a launch fails on and
  names the same vars — verified against the old release side by side. The
  question the leaf left open ("peek as well as pre-flight, or pre-flight
  only?") is settled **both**: `Done when`'s first clause requires it, since the
  model requirement is per-kind and only the peek yields a kind. The two costs
  are recorded in *model-per-task-kind*'s Consequences — the dry run now needs
  `grove-llm` resolvable and the harness binary on PATH, both conditions of the
  launch it reports on. It still writes no stamp (`B3`). The readiness line now
  names the leaf, kind, harness and model; the three states (bootstrap / live
  leaf / no live leaves) render distinctly.

Added by `07`, and the same species as `06`'s own leftovers rather than of
`04`'s inversion: a stale claim the taxonomy sweep missed, in a file the leaf
that found it had no reason to enter.

- `09` **planning-grills-leftovers** (impl) — `docs/driving-a-grove.md` and one
  line of `docs/workflows/multi-step.md` still say a `planning` session opens
  with a grilling pass. `content/driving.md` was relabelled by `06`; its
  repo-facing sibling was not. **Done.** A third file turned up under the leaf's
  own "add it here" rule and was absorbed: `content/grilling.md`'s *title* read
  "the planning-task interrogation procedure" — the worst of the three, since
  `content/` is what every session reads at runtime. The field guide's worked
  example (`refactor-to-archon`) predates the taxonomy, so it now declares that
  once and keeps its **quotations verbatim**; back-dating a quoted brief to
  today's labels would falsify the record the doc exists to make traceable.

Added by `09`. A different species from `09`'s own subject — not a "planning
grills" claim but a stale *kind label* — which is why it is a leaf and not part
of that relabel.

- `10` **content-kind-labels** (impl) — `content/` still names `work` as a live
  kind (`driving.md`) and attributes the spec to a `planning` task
  (`SPEC-FORMAT.md`, ~4 sites) where `SKILL.md` and `docs/concepts.md` both say
  `design`. Aggravated by living in `content/`, which is provisioned to
  `~/.claude/skills/grove/` and so is read by every session on every grove.

## Pointers

- ADRs a session here must read: *task-kind-taxonomy* (the closed-set argument
  this node rewrites), *model-per-task-kind* (the routing mechanism it extends),
  *self-driving-loop*.
- Glossary terms in play: Task kind, Kind routing, Review chain / vendor pair,
  HITL/AFK.
- Both ADRs are reworked **in place** — merge / split / delete, never a
  superseding record (`linkuistics:decision-records`).
- The behavioural contracts this node depends on, stated without line numbers so
  they survive the code moving:
  - The loop driver **peeks the picked leaf's kind** before launching. This was
    gated on some routing env making it matter, keeping the unconfigured path a
    zero-subprocess launch; `04` removes that gate, because a required var must
    be checked every iteration.
  - A **rerouted** launch (launch harness ≠ stamped harness) must never inherit
    an unscoped value — not the base model var, not the global binary override.
    A codex profile name is garbage to pi.
  - Kind **reading degrades** (unrecognised ⇒ `impl`, warn) but harness routing
    **refuses**: the driver bails rather than launch on the wrong vendor. The
    asymmetry that justified this — "model selection is a nicety, a misroute is
    not" — no longer holds after `04`: a missing model is now an error too, so a
    **degraded peek bails unconditionally**, not only under a configured
    override.

## Notes

**The user's actual configuration**, which is what the design was derived
against: claude leads, codex reviews, claude integrates the review; research runs
claude + codex, combined by claude, codex or kimi. The **policy** layer is indeed
two lines (`GROVE_REVIEW_HARNESS=codex` plus `GROVE_CODEX_REVIEW_MODEL`), and only
the research pair's second leaf and its combine step ever carry a per-leaf
declaration.

**But "everything on claude needs no configuration at all" was wrong, and `06`
had to pay for it.** That held only under the pre-`04` rule where an unset var
fell through to the harness's own default. Once a kind with no model var is a
hard failure, *falling through to the stamp* still requires a var for every kind
the stamped harness runs — the stamp absorbs the **harness** axis, never the
**model** axis. The live `.zshenv` had no `GROVE_CLAUDE_*_MODEL` and no unscoped
var at all (it dated from the codex/pi trials), so every kind on every
claude-stamped grove failed at launch. The migration was ~27 vars, not three
renames: nine suffixes × three harnesses, **harness-scoped throughout**, because
an unscoped value would follow a kind onto a harness it was never written for and
this user drives groves stamped to all three.

Read the "about nine vars" figure in the spec and both ADRs with that
caveat — nine is the count for **one** stamped harness, and only if you are
willing to use the unscoped spellings.

**Independent of the herdr subtree.** Nothing here touches herdr.

### Settled in `01` — nothing here is open

All of it is now in `docs/specs/task-kind-taxonomy.md`; this is the pointer, not
a second account.

- **Grammar: documented, not enforced** — and on a stronger reason than this
  brief originally gave. Mutable positions are true but secondary; the
  load-bearing reason is that a grammar is a *relation between leaves*, which
  this node already refused when it rejected "the reviewer must not be the
  author". A non-blocking lint was costed and rejected too.
- **HITL/AFK is generated by a rule**, not enumerated: a kind is HITL when a
  human's own words are the session's input or deliverable. Two kinds qualify —
  `requirements` and `prototype`. **`planning` flips to AFK.** The mark
  *predicts* and does not permit: any kind may stop and ask.
- **Per-leaf model** stays rejected, unchanged.
- **New, and not anticipated here: no defaults.** A kind resolving no model var
  now **fails loudly**, inverting *model-per-task-kind*'s central rule. That
  killed the zero-subprocess launch path and grew leaf `04`.
- **Routing precedence is harness-major**, because the harness axis is a
  correctness axis and the kind axis only a preference axis.
- **The harness axis had no ADR at all.** It has been folded into
  *model-per-task-kind*, which now covers both axes — the family axis spans both,
  so they could not be recorded apart.

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
