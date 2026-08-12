# batches-k33

**Reviews:** `batches-k13`

## Goal

Disprove the classification batching plan **before twelve sessions are executed on
it**. Inserted ahead of `spine-k21` for the same reason `increments-review-k11`
was inserted ahead of the leaves it reviewed: a plan review that runs after the
plan is executed reviews nothing.

The artifact under review is the node brief's **`## The batching contract`**
section plus the twelve batch leaf bodies `spine-k21` … `finish-cycle-k32`. Read
`batches-k13` itself for what it was asked to produce.

## Context

`increments-review-k11` found **six** problems in the analogous artifact one level
up in this grove. That is the base rate to hold this plan against.

### What is mechanically self-checking, and therefore not your job

Each batch's greenness is proved by `cargo build` on that batch's own commit. If a
boundary is wrong, the gate says so loudly at the batch that hits it, and that
batch adjusts. **Do not spend the review re-deriving whether batch N is green** —
spend it on the claims below, which no build checks and which twelve sessions
inherit.

### The doubts, in the order they would hurt

**1. The `pending-` accumulate-and-redistribute convention is new to this session.**
The plan says a `pending-` residual unit may carry `defers=` accumulated by batches
that carved a *body* out of another file while the *referring* region was still
un-carved, and that the batch which later carves that region **redistributes** the
list onto the real units it creates. Nothing enforces the redistribution. The
claim that "the list on the marker being replaced *is* the checklist" is the whole
safety argument, and it is an authoring rule dressed as a mechanism.

Attack it: can a member be silently dropped? Is a `defers=` on a coarse residual
actually *honest*, or does it assert an edge from prose that does not contain the
condition? Is there a cheaper alternative the plan dismissed too fast — in
particular, simply forbidding cross-file edges into pending regions and letting
the referring region's own batch write them via its inbound sweep?

**2. The inbound sweep is claimed "mechanical and complete".** It is
`grep -rn '<F>' content/` plus a per-hit judgement. Is it complete? A trigger→body
relationship that the prose expresses **without naming the file** would be invisible
to it — and the plan offers no second net. Find one if it exists.

**3. The four cross-file overlaps are handled by a coordination protocol across
twelve sessions, with no enforcement.** The same rule is stated twice or three
times in the corpus in each of these cases:

| rule | sites | earlier batch records the call | later batch must honour it |
|---|---|---|---|
| in-session reviewer budget | `SKILL.md` *Review ownership*, `driving.md` §*Doubting…* | `doubt-moves-k27` | `execute-k29` |
| externalize vs absorb | `SKILL.md` *Decompose*, `driving.md` §*Externalizing…* | `decompose-moves-k28` | `execute-k29` |
| ADR reworking | `SKILL.md` L550ff, `driving.md` L285ff, `ADR-FORMAT.md` §*Why the set stays minimal* | `evidence-moves-k26` | `lifecycle-k31` |
| spec membership & grain | `SKILL.md` `## Specs`, `SPEC-FORMAT.md` | `guides-k24` | `finish-cycle-k32` |

Getting one wrong is consequential in **both** directions: duplicate the condition
on both sides and every mandate carries it twice; put it on neither and the mandate
carries it nowhere — the silent direction, with no diff. Should the plan have
**pre-decided** these four calls rather than delegating them to a hand-off
protocol? It had the files open and did not.

**4. `driving.md` (batches 5–8) is ordered before `SKILL.md`'s middle
(batches 9–12), and that was a choice, not a necessity.** The plan justifies it by
`driving.md` self-rooting — but its own decoupling lemma means `SKILL.md` could
equally have been carved first without edges, with the sweep filling them in later.
The consequence of the chosen order is that the `driving.md` sessions make the
overlap calls in doubt 3 **first**, without having seen how `SKILL.md` states the
same rules. Is the hub the better place to decide first?

**5. The batch sizes span 5,454 to 15,904 bytes.** `execute-k29` at 5,454 may be
too small to justify a session's fixed cost (bootstrap, build, test, commit); the
plan defends it as "six-file edge density, not prose volume". `shapes-k23` at
15,904 may be too large. Should `shape-cutting-k30` (10,067) and `lifecycle-k31`
(13,711) merge, or `shapes-k23` split? Twelve children was a judgement about
session size made without executing one.

**6. `kinds-k22` carves a non-contiguous region** — `TASK-FORMAT.md` L1–192 *and*
L473–501 — leaving a middle residual. Justified as "one closure: L473–501 is extra
guidance on three of the nineteen kinds". Is that worth asking one session to hold
two disjoint regions, when the alternative (move L473–501 into a later batch) costs
only that the batch runs after `guides-k24` and `decompose-moves-k28`?

**7. The greenness lemma claims (D), (R), (T) are the *only* obligations a batch
creates.** Check that against the spec's full malformation list
(`docs/specs/mandate-delivered-methodology.md`, *A malformed embed fails the
build*). The plan asserts the per-file rules — no unit declared, body text before
the first marker, fence balance at EOF, trailing newline, no control character in
the path, `kinds` required on triggering and forbidden on procedural, fixed
attribute order, id uniqueness — are all preserved because a batch only ever
*subdivides*. Is that true for every one of them? Subdivision does add markers, and
a marker is where `kinds`/`class`/order errors are introduced.

**8. `## Reference files` is left as a free choice to the final batch.** The plan
names it as "the single place in the corpus where the deferral graph's shape is a
free choice" and declines to decide it. Is deferring a graph-shape decision to the
last and most loaded session right, or should it have been settled here?

## Done when

- Each of the eight doubts above is either **confirmed as a real problem** with a
  concrete failure it would cause, or **dismissed** with the reason.
- Any problem the plan has that is **not** on that list is named — the list is the
  producer's own view of where it is weak, and the producer is not the best judge
  of that.
- Findings are recorded in this leaf's body.
- If there are findings worth acting on, an `integrate-review-planning` leaf is
  cut as this session's last act. Place it per the directory-local rule: the first
  sibling entry after this leaf whose subtree still holds live work is
  `spine-k21`, so use
  `grove-llm leaf-insert spine-k21 batches --kind integrate-review-planning` —
  the repair must land before any batch executes on the plan.
- If there is nothing worth acting on, create nothing and retire.

## Notes

- **Inspection only.** Do not write a `content/` marker, do not run `cargo build`
  to try a batch, and do not rewrite the plan — an integration leaf owns every
  fix. If a doubt can only be settled by trying a marking, that is itself a
  finding: it means the batching is underdetermined at that point.
- The corpus is 145,233 bytes across nine files, and the twelve regions sum to
  144,949 — the difference is `content/SKILL.md`'s 284-byte YAML preamble, which
  no unit covers by design. **Verify that arithmetic**; a region the plan failed to
  assign to any batch would be classified by nobody and caught by no build, because
  it would simply remain inside a `pending-` unit — and the final batch's
  zero-`pending-` check would then fail at the very end, twelve sessions late.
