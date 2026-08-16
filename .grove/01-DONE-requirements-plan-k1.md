# plan-k1

## Goal

Settle *what* should change about how a Grove session receives the **procedural
half** of the methodology. The triggering half stays where
[`skill-delivers-the-methodology`](../docs/adr/skill-delivers-the-methodology.md)
put it — byte-exact slices composed into `${prompt}`, selected by session kind.
What is in question is the *then*, not the *if*.

## Context

The repo is mid-transition. Mandate composition has landed; skill provisioning
has **not** yet retired — `src/launch.rs:14` calls `provision_installed()` and
`src/loop_driver.rs:114` calls `reverify_installed()`. `docs/specs/mandate-delivered-methodology.md`
calls that two-path window "transient and never a resting state". So the
retirement increment this grove touches has not run, which is what makes a change
of direction cheap now.

Today the procedural half is served by `grove-llm methodology <id>` from the
invoking binary's own embed — 69 procedural units, ~93 kB, held out of every
mandate and reachable only through that verb.

## Decisions

### The problem is the mandate itself, in three parts

Stated by the human, in their words:

1. **The wall of prompt text degrades behaviour.** Sessions do not always end —
   they finish their work and fail to signal, which under an interactive harness
   *stalls* the loop. Quality is affected, not merely cost.
2. **The text is no longer on-demand.** A mandate arrives whole, up front,
   whether or not the session's situation ever calls for any of it.
3. **The mechanism is alien.** A ~49 KiB argv prompt is unlike anything users
   recognise, and that will put them off.

Note that (1) is the failure the two increments immediately preceding this grove
were already chasing from the other end — `retire-next-steps-k2` added a
commit-then-complete reminder at retirement, `signal-unit-placement-k3` moved the
signal unit to compose last. Both treated the symptom. The diagnosis here is that
the wall is the cause.

### The first answer was measured and found not to reach the problem

The opening question was answered "split: conditions in the prompt, bodies in a
skill". Measurement against the shipped embed refuted it:

| set | bytes |
|---|---|
| universal triggering (`kinds=*`, in **every** mandate) | 45,032 |
| all triggering (any scope) | 58,900 |
| procedural (in **no** mandate) | 93,102 |
| whole `content/` corpus | 152,514 |

The procedural bodies are **already** out of every mandate. Moving them to a
skill changes mandate size by zero bytes and reaches none of problems 1 or 2.
The wall is made entirely of triggering units.

### Full reversal: the triggering half moves too

`${prompt}` shrinks back to a launcher plus the two facts the driver resolves at
runtime (the selected stable handle, the resolved version control). Everything
else is delivered as a **skill**, through the harness's native progressive
disclosure.

Two of the superseded design's own clauses carry this rather than contradict it:

- Its rejection of *"point at locations instead of slicing"* names a **live
  reopen condition** — *"Reopen if `content/` is ever restructured so that every
  rule is separately addressable — which is close to what unit markers now make
  true."* The 140 markers performed that restructuring.
- Its rejection of behavioural verification names the check to trust: *"the
  honest behavioural check is the next real Grove run after the change lands,
  with a human watching."* That check has now run and come back negative.

What this **does** overturn is the same clause's *"only as a supplement… never as
a replacement for triggering conditions."* That is deliberate, and it owes an
argument for why the unasked-question risk is now acceptable — see open
questions.

### Finding: 58.9 kB of "conditions" is itself evidence of misclassification

51 universal units averaging ~880 bytes each are not terse `if`s; they are prose
that *contains* a condition. A straight re-plumb of `content/` into a skill
therefore reinstates the ~51 kB monolithic `SKILL.md` the superseded design's
problem statement condemned. The reversal is a rewrite of `content/`, not a
change of delivery plumbing — and the unit classification is the raw material for
that rewrite rather than an obstacle to it.

### The end state is restructured, not re-plumbed

`content/` is rewritten into a progressive-disclosure skill — a short `SKILL.md`
of conditions, `references/` carrying procedures. A straight re-plumb was
rejected: a harness skill is not lazily read *within* itself, so provisioning
`content/` unchanged hands back a ~51 kB `SKILL.md` that lands in context in one
gulp. That moves the wall from launch time to first-invocation time rather than
dissolving it, and leaves problem 2 half-fixed.

### None of the mandate machinery survives

The composer, the marker grammar, the fence-state parser, the build gate, the
completeness invariant and `grove-llm methodology` all go. The 140 unit markers
are used as **scaffolding** for the rewrite — they already record which prose is
`if` and which is `then` — and are deleted with the rest.

The argument is that the apparatus exists to make *selective* delivery safe, and
once delivery stops being selective it has no job. Note also what the invariant
actually bought: *every triggering unit reaches every mandate its scope admits*,
a claim about the **document**, checked at build time. It never guaranteed the
session read it — and a session that received the text and did not act on it is
precisely the failure being reported.

### Grove provisions the skill, as it did before

`src/provision.rs` stays (355 lines, still called from `src/launch.rs:14` and
`src/loop_driver.rs:114`). Single-command `brew install` keeps working, and
`--content-hash` build pairing stays meaningful. Rejected: shipping via the
marketplace beside `linkuistics`, which is more familiar but makes installing
Grove two steps and lets skill and binary versions drift with no check left.

### Specificity survives as a pointer, not a slice

The driver knows the selected kind before the session exists, so `${prompt}`
names the kind's reference file directly. This *is* the "point at locations"
option the superseded ADR said to reopen once `content/` became addressable —
the win is kept, the wall is not.

### A short guaranteed core stays in `${prompt}`

**Reopened deliberately** after the human reported that the prior failure was
*observed*: sessions did not read the provisioned skill. That makes `${prompt}`
— the one channel a session cannot skip — load-bearing rather than merely a
launcher. It carries a forceful pointer to the skill, the kind's reference file,
the two runtime facts, and the conditions that must never be missed. Target a
couple of KiB, not 49.

The cost is admitted rather than argued away: this is the two-delivery-path state
the superseded ADR rejected outright. At this size the drift risk is small, but
the design owes a **rule** for what earns a place in the core — a list would go
stale silently, which is that ADR's own objection to manifests.

## Done when

Done. The *what* is settled; the open questions below are design work and are
handed to the `design` leaf cut from this session.

## Notes

**Glossary updated inline**, narrowly and deliberately. Most of what `CONTEXT.md`
says about provisioning describes the build that exists today and stays accurate
until the mechanism changes. What was false the moment the decision landed is the
forward-looking half — the sentences *predicting* provisioning retires — so those
are what changed, in `CONTEXT.md`'s **Global skill provisioning** entry and in
`CONTEXT-MAP.md`'s shared-target relationship. The remaining entries
(Methodology unit, Mandate slice, Triggering unit / procedural unit, Methodology
identity, Build pairing) are accurate descriptions of a live mechanism and are
reworked by the increments that remove it.

Handed to `design`:

1. What **rule** decides what earns a place in the guaranteed core?
2. Trigger strength — the frontmatter `description:`, the launcher's wording, how
   `SKILL.md` opens. This is the half that answers the *observed* prior failure,
   and nothing else in the design does.
3. The reference-file layout, including where per-kind discipline lives.
4. Reworking `mandate-delivers-the-methodology` and
   `mandate-delivered-methodology` in place, and the argument owed for
   overturning *"never as a replacement for triggering conditions"*.
