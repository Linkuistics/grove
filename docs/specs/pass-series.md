# pass-series

How a grove represents work that is done **again** — a repeated pass over one
subject, with a declared sequence, a declared exit and a declared bound.

The word is *pass series* and not *loop* on purpose. In this repository **loop**
already names the driver loop — the one-session-per-task cycle bare `grove`
runs — and `CONTEXT.md` heads two entries with it. A second reading of the same
word inside one ubiquitous language is a defect, not a shorthand.

## Problem

Grove has one ordering mechanism and one walk. Position orders siblings,
`pick` is a depth-first pre-order walk that cannot re-enter a retired leaf, and
there is no leaf state meaning *again*. So iteration today is **emergent**: a
session that wants more work to happen cuts its own follow-on, and nothing
anywhere records that a repetition was intended, how many are expected, or what
would end them.

That is not a hypothetical gap. It is what this grove's own tree did, twice,
in opposite directions:

- **The one pipeline designed to iterate never did.** Four books ran the
  editorial chain. Each carries exactly one `copy-edit`, one `art` and one
  `proof`; no stage was ever re-run. The correction mechanism in
  [`a-feedback-edge-is-forward-tree-growth`](../adr/a-feedback-edge-is-forward-tree-growth.md), and the
  one-re-run-per-stage bound it fixes in advance, **fired zero times in four
  documents**.
- **The corrections still arrived, uncapped, somewhere with no charter over
  them.** Of the 150 leaves anywhere under `crate-books-k14`, **36 were
  allocated after the last editorial stage leaf in the whole tree**
  (`proof--grove-loop-k182`), and 29 of those are flat siblings at that node's
  own level rather than inside any book. Once a document's node closed, its
  corrections became ordinary leaves at the parent — no sequence, no bound, and
  no session able to say whether the stream was near its end.

Both readings were taken while `loop-construct-k7` was the live leaf and before
that leaf's own additions to the tree. They are a **record of a run**, not a
running count: a later leaf under either node moves the figures without touching
what they establish.

Read together they say what the construct is for. The problem is not that grove
cannot express repetition; it plainly can, 36 times over. The problem is that
repetition arrives **undeclared**, so `find .grove` shows a list of leaves where
the reader needed to see a shape, a count and a stopping rule.

## Solution

A **pass series** is a node whose children are **pass nodes**, each pass node
holding that pass's step leaves. Every entry is the one node species and the one
leaf species; nothing in any name marks a series, a pass or a step. The shape and
the rejected marker token are
[`iteration-is-a-node-of-nodes`](../adr/iteration-is-a-node-of-nodes.md), and are not restated here.

What the series adds over an ordinary node is entirely in its `BRIEF.md`, and it
is four things written **before its first pass is cut**:

1. **The step sequence** — the kinds, in order, that one pass runs.
2. **The exit condition** — a fact a session can check at the end of a pass.
3. **The cap** — the number of passes, as a number.
4. **What happens at the cap** — which is always the same: stop and say so.

A node whose children happen to repeat, with none of those four written down, is
not a pass series. It is the emergent iteration above, which is the thing this
construct exists to replace.

```
14-render-quality-k40/              the series node
  BRIEF.md                          the four declarations, written before pass 1
  01-render-quality-k41/            pass 1 — an ordinary node
    BRIEF.md                        the leaf body pass 1 was cut with
    01-DONE-impl--render-quality-k42.md
    02-DONE-review-impl--render-quality-k44.md
  02-render-quality-k48/            pass 2
    BRIEF.md                        what pass 1 left, written by pass 1's last step
    01-DONE-impl--render-quality-k49.md
    02-review-impl--render-quality-k53.md      <- where the series is now
```

Two passes ran, the second is inside its review step, and the reader needed
neither grove nor a task file to learn it. That is constraint 6, and it is bought
by changing nothing.

## Decisions

**A pass is created the only way grove creates a node.** The last step of a pass
cuts the next pass as a leaf —
`grove-llm leaf-add <series-node> <stem> --kind <first step's kind>` — and the
session that picks that leaf finds it bigger than one session and runs
`grove-llm leaf-decompose`, which moves the leaf's body in as the pass's
`BRIEF.md`. So a pass's charter is written by the pass before it, which is the
only session that knows what this one has to carry forward. No verb is added and
no node is created by anything but decomposition.

**A one-step pass is a leaf and gets no directory.** The pass node earns its
place at two steps. A series whose passes are single sessions is a run of
sibling leaves under the series node, and is still a pass series: what makes it
one is the four declarations, not the directory depth.

**The steps of a pass are cut lazily, one at a time.** Each step's last act cuts
the next — unless a live later sibling under the pass node already holds it, in
which case it cuts nothing. That condition is read off the node's live entries;
there is no field. This is `references/editorial.md`'s rule and
`a-feedback-edge-is-forward-tree-growth`'s reasoning, reused unchanged: cutting
late is what lets the cutting session write the next body with the case it has
just met, and the condition is what stops a queue regrowing its own tail.

**Every pass node and every step leaf under a series carries the series' bare
stem as its whole slug.** No pass number, no step word. The kind field says which
step this is, and the position says which pass — both already parsed. A `pass-2`
slug sitting at position `03` after any insert is the disagreement
`TASK-FORMAT.md` warns a duplicating convention produces. The cost is real and
already paid elsewhere: `grove-llm resolve <stem>` on a series' stem is ambiguous
and answers by listing every match with its key, which is what that verb is built
to do.

**Sending work back inside a pass is not a new pass.** A step that meets a defect
an earlier step of *this* pass owns cuts a correction run inside the pass node,
under the rule `a-feedback-edge-is-forward-tree-growth` already fixes. A pass is a
repetition **the series declared in advance**; a correction run is a repetition
**the finding session decided**. That is the test that tells them apart, and it
is why a correction run gets no pass directory: it is not a pass.

**The exit condition is evaluated by a pass's last step, and exit is the absence
of a cut.** The last step checks the condition; if it holds, it cuts no next pass
and the series simply has no live leaf left. Nothing is marked. *Another pass is
needed* is expressed by the next pass node existing, which is the same answer
`a-feedback-edge-is-forward-tree-growth` gives one level down, and the reason no
fourth leaf outcome is wanted here.

**The exit condition may not be a yield curve, and this repository is why.**
[`docs/review-yield.md`](../review-yield.md) measured nine review chains across three channels and
reported, honestly, that it **cannot separate flat from falling** — the observed
gap is about 1.8σ before any confound, and reaching 3σ would need roughly 25
chains of matched subject size. The sharper half of that record is the one that
binds here: **no review came back empty, at any point**, including the four cut
latest against the smallest subjects. So *stop when a pass finds nothing* is not
merely unproven, it is a rule that would have fired **zero times in nine
opportunities** on the only evidence available. An exit condition must be a fact
about the subject — a gate that passes, a threshold declared in advance, a
checkable state — and never a judgement about the trend of what passes are
finding.

**The cap is a number fixed before pass 1, and reaching it escalates rather than
exits.** A session that would cut pass `cap + 1` stops and says so instead. The
number is written down in advance for the reason
`a-feedback-edge-is-forward-tree-growth` states for its own bound: it is the
point where an unattended series stops being a series and becomes an
oscillation, and stating it in advance is what stops it being chosen by a session
that is already inside the loop.

**The cap bounds the series, not the work — and a brief that claims otherwise is
wrong.** This is the honest limit and it is measured, not feared: when the four
book nodes closed, 36 further leaves were allocated under their parent. Exit
relocated the correction traffic; it did not end it. So a series' exit means
*this shape stops here*, and the work it was over may continue as ordinary leaves
somewhere with no cap over them. A series whose `BRIEF.md` reads its cap as a
claim that the subject is finished has asserted something the construct cannot
deliver.

**Three recorded decisions are reopened by this construct and all three are
kept.** Each is kept for a reason, not by omission:

| record | verdict | why it survives |
|---|---|---|
| *one task is one session* (`SKILL.md`) | kept unchanged | a pass's step is one leaf and one session; a series repeats **kinds**, never a leaf |
| [`entries-are-never-removed`](../adr/entries-are-never-removed.md) | kept unchanged | pass two's step is a different leaf with a fresh key, which is the allocation rule working rather than a cost of it — read at the same moment as the figures above, this tree's maximum key **equalled** its entry count, so no key had ever been skipped or reissued |
| the outcome partition — live / `DONE` / `ABANDONED` (`references/retire.md`, `CONTEXT.md`) | kept unchanged | *another pass is needed* is ordering, not a state; a fourth word here is the `blocked` / `deferred` / `superseded` taxonomy that record already refuses |

The name grammar is likewise untouched: a series adds no token, no infix and no
reserved word, so `grove-loop`'s `task_name` module and its canonicity are not in
this construct's blast radius at all. That is a decision rather than a
coincidence, and its cost is argued in the ADR.

## Test seams

**There is no seam, and that is the design rather than a gap in it.** Grove
validates no cross-leaf grammar: nothing parses a step sequence, counts passes
against a cap, or detects a session that skips the escalation. Every one of the
four declarations is discipline carried by a skill, in exactly the position
`a-feedback-edge-is-forward-tree-growth` already puts its own bound and says so
rather than glossing it.

What *is* covered, and needs nothing added: the shape itself. Every entry a
series creates is an ordinary node or leaf, so the existing name grammar, its
canonicity property and the tests over them cover a pass series the day it is
first written, with no new case.

The condition under which a seam becomes possible is already recorded: grove
growing a cross-leaf validator, which `a-feedback-edge-is-forward-tree-growth`
names as what would reopen its own *record the edge as data* option. Until that
exists, adding a checkable cap would mean adding state beside the tree, which
constraint 1 forbids and which no seam can rescue.

## Out of scope

- **A render verb, a terminal surface, or the DevTUI direction.** The filesystem
  representation is the minimal UI and is settled first; the rest is on the root
  brief's horizon. This is also the sixth and load-bearing objection to a marker
  token — a mark with no reader is state — so a surface arriving is what would
  reopen that question rather than this one.
- **A verb that creates a series, a pass, or a step sequence.** `leaf-add` and
  `leaf-decompose` already do it, and a verb would put grove in the position of
  interpreting a methodology it deliberately holds no opinion about
  ([`a-kind-is-an-open-token`](../adr/a-kind-is-an-open-token.md)).
- **Any code change.** Nothing in `grove-llm`, `grove-loop` or `ordinal-fs-tree`
  moves. The construct is expressible in the tree as it stands today, which is
  the strongest evidence available that the shape is the tree's own and not a
  new one bolted to it.
- **A default cap, or a library of series shapes.** The cap is per series and per
  subject, and a default would be exactly the number chosen by someone already
  inside the loop, which is what fixing it in advance exists to prevent.
