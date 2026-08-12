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

## Review findings

The plan has findings worth acting on. Doubts 1, 2, 3, 4, 7 and 8 are real;
doubts 5 and 6 are dismissed. Two additional defects were found.

### F1 — High: later batch boundaries use coordinates that earlier batches mutate

The `SKILL.md`, `TASK-FORMAT.md`, and `driving.md` children describe their
regions as exact line ranges. Every batch before a later range inserts marker
lines into that same file, so the later range no longer names the prose it named
when `batches-k13` counted it. For example, `spine-k21` inserts markers above
`SKILL.md`'s baseline L167, but `execute-k29` still says to carve L167–245;
the four `driving.md` batches have the same drift. The heading names and
`pending-*` ids make the intended boundaries recoverable, but the bodies also
say that a residual covers exact stale lines, so an implementer following the
stronger wording can carve the wrong bytes without violating the prose goal.

The integration must make semantic boundary anchors and the predecessor's
`pending-*` id authoritative. Baseline line ranges and byte counts may remain as
orientation only if they are labelled explicitly as coordinates in the
pre-classification snapshot, never as execution-time boundaries.

### F2 — High: doubt 1 is confirmed; a `pending-*` marker is a lossy edge ledger

Parking `defers=` on a coarse residual and later redistributing it is not
mechanically safe. A member can be dropped while the build stays green whenever
the target procedure has another inbound path. The planned
`CONTEXT-FORMAT.md` path is a concrete instance: `guides-k24` can leave the
`SKILL.md` citation parked on `pending-skill-*`; if a later `SKILL.md` batch
drops it, the context-format procedures remain reachable through `grilling.md`,
so neither build nor reachability detects the lost semantic edge.

The child bodies compound this by giving the body-carving batch two choices:
park the edge, **or** leave it to the later referring-region batch and merely say
so. That means there is no unique owner. For this corpus, the cheaper safe rule
is the one proposed in doubt 1: do not add an edge to a `pending-*` source. If
the source is already carved, the target/body batch owns the inbound edge; if
the source is still pending, the later source batch owns the outbound edge once
the target exists. Rewrite every child hand-off to use that single rule and
remove redistribution as a protocol.

### F3 — High: doubt 2 is confirmed; filename grep is not a complete edge sweep

`content/prompts/continue.md:2-6` says “see the skill's Decompose step” without
spelling `SKILL.md`. It is a direct cross-file trigger/body relationship that
`grep -rn '<F>' content/` cannot find. Yet `finish-cycle-k32` declares the file
all-triggering, creates no procedure, and lists no edge back to the Decompose
body. The exact silent failure claimed impossible by “mechanical and complete”
therefore exists in the current corpus.

Filename grep is useful evidence, not completeness. The integration must add a
semantic cross-file/duplicate-rule inventory (including this launcher edge) and
make each batch reconcile the inventory entries it owns in addition to running
the filename sweep.

### F4 — High: doubts 3 and 4 are confirmed; the overlap table is incomplete and delegates the load-bearing calls backwards

The four listed overlaps are not exhaustive even for the rules they name:

- reviewer ownership also appears in `TASK-FORMAT.md:164-177`, not only
  `SKILL.md:198-215` and `driving.md:415-453`;
- externalize-vs-absorb also appears in `TASK-FORMAT.md:80-101`, not only the
  `SKILL.md` and `driving.md` regions;
- ADR reworking appears twice in `SKILL.md` (`217-224` and `550-554`) and in
  `TASK-FORMAT.md:151-160`, as well as the two sites and format guide the table
  names;
- the spec current-state rule appears in `SKILL.md:217-224` as well as its
  dedicated `## Specs` section and `SPEC-FORMAT.md`.

Those repetitions are expressed without reliable filename citations, so F3's
sweep does not recover them. As written, the earlier `driving.md` or format-guide
batch makes a condition/body ownership decision before the later hub batch has
classified — and in some cases before it has been told every site exists. A
sibling-body hand-off is reviewable after the fact but does not prevent the
first call being made from an incomplete set.

The planning integration should inventory all sites now and pre-decide, per
rule, which byte span carries the triggering condition and which spans are
procedural targets (or explicitly justify deliberate duplicate triggering
units). Once those calls are in the node brief and child bodies, the current
file order is acceptable; without them, putting `driving.md` first is not.

### Doubt 5 — dismissed: the size spread follows judgement density

`execute-k29` is only 5,454 prose bytes but owns six-file edge reconciliation
and several overlap calls, so merging it would hide the densest coordination in
a larger session. Merging `shape-cutting-k30` and `lifecycle-k31` would create a
23,778-byte region spanning two distinct rule families. `shapes-k23` is the
largest at 15,904 bytes, but nearly half is one coherent rejected-alternatives
section whose condition/narrative split is the point of the leaf; it is not yet
shown too large for one focused session, and its body already names
decomposition as the escape if execution proves otherwise.

### Doubt 6 — dismissed: the disjoint regions are one semantic decision

`TASK-FORMAT.md` L473–501 elaborates three producer bullets in L74–101. Keeping
both regions in `kinds-k22` lets one context make the scoped-kind decision once;
moving the tail later would replace spatial awkwardness with cross-session
semantic coordination. The line numbers still need the F1 baseline-only label,
but the non-contiguous shape itself is justified.

### F5 — Medium: doubt 7 is confirmed; (D), (R), and (T) are not the only obligations a subdivision creates

They are the only **cross-unit deferral-graph** obligations. A batch also creates
markers and ids, so it can newly violate marker syntax and fixed attribute
order, omit/illegally add `kinds=`, name an unknown kind, duplicate an id, or
place a would-be marker where the parser does not recognise it. Some unchanged
file properties really are preserved (trailing newline, path bytes, and an
already-balanced fence corpus), but the lemma currently generalises from those
to all per-file rules. `cargo build` catches the local mistakes; that makes the
plan executable, not the lemma true. Narrow the lemma and list the local
per-marker obligations each child still owns.

### F6 — Medium: doubt 8 is confirmed; `## Reference files` is not merely an eight-edges-or-zero graph choice

The index does not itself state a condition, and all eight point-of-use edges do
not turn it into one. Giving a triggering index eight `defers=` targets makes
every session receive an inventory and then fan out to procedures regardless of
which condition it encountered; giving it none leaves unexplained why these
bytes are triggering rather than narrative. The final child is being offered a
false binary for what the node brief already identifies as the design's hard
case: prose that is neither a condition nor a procedure.

Settle the intended class and inbound trigger in the plan, or explicitly carry
this section as a design finding for the aggregate classification review. Do not
make the most loaded final batch silently choose between two graph shapes that
both evade the classification question.

### F7 — Medium: the corpus arithmetic reaches the right total for the wrong reason

The twelve advertised regions do sum to 144,949 bytes, and the corpus is
145,233 bytes. But the 284-byte difference is **281 bytes of YAML preamble plus
three one-byte separator lines** (`SKILL.md` L246, L407, and L609), not a
284-byte preamble. Total partition means those blank lines will be absorbed by
an adjacent unit, so no byte is intrinsically lost, but no child currently owns
them according to its exact line-range contract. Assign each separator to the
preceding or following semantic region and correct the byte counts; otherwise
the arithmetic cannot serve as the claimed coverage proof.
