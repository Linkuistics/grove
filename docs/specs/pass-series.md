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
repetition arrives **undeclared** — nothing says a repetition was intended, how
many are expected, or what would end them — so `find .grove` shows a list of
leaves where the reader needed to see a shape. What the construct fixes is that
the shape becomes visible and the count and the stopping rule become *written
down*; the count and the rule do not thereby become visible in `find`, and
*What the tree carries* below says plainly which of the two each fact is.

## Solution

A **pass series** is a node whose children are its **passes** — a pass that
needs more than one session is a node holding that pass's step leaves, and a pass
that is one session stays a leaf. Every entry is the one node species and the one
leaf species, and which of the two a pass is follows from the ordinary rule that
a leaf becomes a node when it proves bigger than one session; nothing in any name
marks a series, a pass or a step. The shape and
the rejected marker token are
[`iteration-reuses-the-existing-species`](../adr/iteration-reuses-the-existing-species.md), and are not
restated here.

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
neither grove nor a task file to learn **that**. The reader also learns it is
looking at a series rather than an ordinary node, because two sibling directories
carry one slug and nothing else in a grove does that — the discriminator the
[ADR](../adr/iteration-reuses-the-existing-species.md) weighs the marker token
against. What the reader cannot learn from this listing is the series' step
sequence, its exit condition or its cap, and no arrangement of names could carry
them. That is constraint 6 doing as much as it can, and it is bought by changing
nothing.

## Decisions

**A pass is created the only way grove creates a node.** The last step of a pass
cuts the next pass as a leaf —
`grove-llm leaf-add <series-node> <stem> --kind <first step's kind>` — and the
session that picks that leaf finds it bigger than one session and runs
`grove-llm leaf-decompose`, which moves the leaf's body in as the pass's
`BRIEF.md`. So a pass's charter is written by the pass before it, which is the
only session that knows what this one has to carry forward. No verb is added and
no node is created by anything but decomposition.

**A one-step pass is a leaf and gets no directory**, and that is the node rule
holding rather than an exception to it. A node is a leaf that proved bigger than
one session; a pass that never outgrows its one session never becomes one, for
exactly the reason no other one-session leaf does. So a series whose passes are
single sessions is a run of sibling leaves under the series node, and is still a
pass series: what makes it one is the four declarations, not the directory depth.
The cost is that this series' passes carry no shared-slug discriminator — sibling
*leaves* sharing a stem is what a review chain and a vendor pair already look
like — so a one-step series is identifiable only from its brief. That is the
narrow case the ADR's marker-token trade-off turns on, and it is named there.

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
is why a correction run gets no pass directory: it is not a pass. The test is
decidable by the session that has to apply it, because both halves are facts that
session holds: whether the repetition was written down before pass 1, and whether
this session is the one that found the defect.

**The subject is work that was completed and is deliberately run again, and there
is a third form of repetition outside it.** A leaf can also stay **live** across
several sessions — the loop stops, a human gate holds, and a later launch picks
the same handle again. That is not a pass and not a correction run: no new entry
appears at all. It happens in live groves and is not a hypothetical —
`Writegood`'s `impl--machine-supply-k22` is still live under its `measure-k18`
node and carries six commits whose own subjects number the readings first through
sixth. A series does not represent that form and is not meant to: nothing was
completed, so there is nothing to run again. Naming it matters because the
pass/correction pair is a partition of *this construct's* repetitions and not of
Grove's, and because the record below leans on what a pass's step is.

**The exit condition is evaluated by a pass's last step, and exit is the absence
of a cut.** The last step checks the condition; if it holds, it cuts no next pass
and the series simply has no live leaf left. Nothing is marked. *Another pass is
needed* is expressed by the next pass existing, which is the same answer
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

**The escalation is recorded in the series' `BRIEF.md`, because the tree cannot
hold it.** Stopping and saying so reaches the operator's terminal and nothing
else, and terminal output is not an artifact. A satisfied exit cuts nothing and a
reached cap cuts nothing, so the two outcomes leave a **byte-identical** tree: a
series node with no live leaf beneath it. The session that hits the cap therefore
writes what happened into the series brief before it stops — which pass reached
the cap, and what was still unfinished — because that brief is the only durable
place the fact can go without becoming the status file constraint 1 forbids. A
session that skips this leaves a series indistinguishable from one that exited
cleanly, and nothing will detect it.

**The cap bounds the series, not the work — and a brief that claims otherwise is
wrong.** This is the honest limit and it is measured, not feared: when the four
book nodes closed, 36 further leaves were allocated under their parent. Exit
relocated the correction traffic; it did not end it. So a series' exit means
*this shape stops here*, and the work it was over may continue as ordinary leaves
somewhere with no cap over them. A series whose `BRIEF.md` reads its cap as a
claim that the subject is finished has asserted something the construct cannot
deliver.

**What the tree carries, and what only the brief does.** The construct's
reader-facing claim is bounded, and stating the bound is part of the design
rather than an admission against it:

| fact | where a reader gets it |
|---|---|
| that this is a series, once it has repeated | the names — sibling entries under one node sharing one slug |
| that this is a series before its second pass | the brief only |
| how many passes have run, and which step each reached | the names |
| the step sequence, the exit condition, the cap | the brief only |
| whether the cap was fixed before pass 1 or changed after it | not the working tree — the brief is freeform and mutable and no entry records an edit. Recoverable from the **VCS**, by a reader who thinks to look and can tell which commit was pass 1 |
| whether the series exited on its condition or stopped at its cap | the brief only, and only if the escalating session wrote it there |

The last two rows are the honest limit. Nothing parses a brief, nothing checks a
declaration against what the passes did, and no arrangement of filenames could
carry either fact — so the declarations are **discipline the brief records**, not
a filesystem representation of the construct. What the filesystem represents is
the shape: which passes ran, in what order, and where each stopped.

The VCS is the deliberate exception to both: it holds the past (constraint 1), so
a reader willing to read history recovers what the working tree cannot show.
Neither limit is a gap a seam could close, for the reason `Test seams` gives.

**Three recorded decisions are reopened by this construct and all three are
kept.** Each is kept for a reason, not by omission:

| record | verdict | why it survives |
|---|---|---|
| *one task is one session* (`SKILL.md`) | kept unchanged | a series repeats **kinds**, never a leaf: pass two's step is a different leaf, so nothing here asks a retired leaf to run twice. The stronger reading — that a step is always exactly one session — is not this construct's to defend, and is already false elsewhere (see the third form of repetition above) |
| [`entries-are-never-removed`](../adr/entries-are-never-removed.md) | kept unchanged | pass two's step is a different leaf with a fresh key, which is the allocation rule working rather than a cost of it — read at the same moment as the figures above, this tree's maximum key **equalled** its entry count, so no key had ever been skipped or reissued |
| the outcome partition — live / `DONE` / `ABANDONED` (`references/retire.md`, `CONTEXT.md`) | kept unchanged | *another pass is needed* is ordering, not a state; a fourth word here is the `blocked` / `deferred` / `superseded` taxonomy that record already refuses |

**The discipline ships as a condition in the spine's register, not as a kind and
not as a family reference file.** No session reads this spec or the ADR; a
session reads `plugins/grove/skills/`, so *where the four declarations reach a
session* is a decision this design owes rather than one its implementation
inherits. The surface is `SKILL.md`'s loop register — which is a list of
conditions each naming the file whose procedure answers it — pointing at
`references/decompose.md`, whose *Choosing a composition shape* section already
owns the chain and the pair and gains the series as a third shape; and
`BRIEF-FORMAT.md`, which is where a brief's shape is stated and is therefore
where the four declarations belong. Two conditions are decomposition-shaped and
already fit that register: *when this work is to be done again, on a declared
sequence with a declared bound*, and *when a repetition was not declared in
advance*. The two alternatives are rejected on the same evidence:

- **A new session kind.** [`a-kind-is-an-open-token`](../adr/a-kind-is-an-open-token.md)
  makes one cheap to author, but a kind is a discipline a *session* runs under
  and a series has no session of its own — its passes run `impl`, `draft`,
  whatever the series declares. No leaf would ever carry the kind, so nothing
  would launch it.
- **A family reference file**, as `references/editorial.md` is for the four
  editorial kinds. A family file is loaded because a member's skill named it, and
  a series has no member kinds — so nothing would name it and nothing would load
  it.

One thing to expect from the chosen surface, and to write rather than gloss:
*Choosing a composition shape* currently opens by saying both habitual shapes are
flat siblings **because** a node means work that proved bigger than one session.
A series is a node, so the section's framing changes with the third shape rather
than merely gaining a bullet.

The name grammar is likewise untouched: a series adds no token, no infix and no
reserved word, so `grove-loop`'s `task_name` module and its canonicity are not in
this construct's blast radius at all. That is a decision rather than a
coincidence, and its cost is argued in the ADR.

## Test seams

**There is no *test* seam, and that is the design rather than a gap in it.**
Grove validates no cross-leaf grammar: nothing parses a step sequence, counts
passes against a cap, or detects a session that skips the escalation. Every one
of the four declarations is discipline, in exactly the position
`a-feedback-edge-is-forward-tree-growth` already puts its own bound and says so
rather than glossing it.

That is a statement about **testability**, and it is not a statement about
placement. *Where* the discipline is carried is a separate question, it is
settled under `Decisions` above, and it is not left to the implementation. "There
is no seam" answers the first question only.

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
  brief's horizon. A surface that must identify a series **before its second
  pass** is also the load-bearing reopen condition on the marker token, so one
  arriving would reopen that question rather than this one.
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
