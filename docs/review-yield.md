<!-- DERIVED FILE — do not edit by hand. -->
<!-- Regenerate: python3 scripts/review-yield.py --rev wvwnwnqx -->
<!-- Pinned by change id, not commit id: in jj the working copy is a
     commit, so writing this file rewrites the commit id but never the
     change id. See docs/loop-record.md for the same reasoning at length. -->

# Review yield — what nine review chains actually found

This measures one thing: **across every review chain in one Grove
workstream, how many findings were raised, how many survived verification
at integration, and how many the integration added on its own.** It exists
because the campaign's own recollection — *review yield did not decay* —
was written from inside the campaign by a session that is not neutral
about it, and rests on a single node.

It is the companion to [`loop-record.md`](loop-record.md), which enumerates
the sessions and deliberately opens no review body. This one opens all
eighteen.

**The verdict, first, because it is the null one.** Nine chains and three
channels **cannot separate flat from falling**. What the tree does
establish is narrower and, for anyone deciding whether to pay for review,
more useful: **no review came back empty, at any point, including the four
cut latest against the smallest subjects — and every one of the 45
findings they raised survived verification.** The section *[Why the curve
cannot be read](#why-the-curve-cannot-be-read)* says what would have been
needed instead, and it is not simply more chains.

## What is derived and what is judged

Classifying a paragraph as a finding is a judgement and cannot be derived.
[`scripts/review-yield.py`](../scripts/review-yield.py) separates the two
and makes the judgement checkable:

- **Derived.** Every ATX heading, top-level ordered-list item and table row
  in all eighteen bodies is extracted from markdown block structure at a
  pinned revision — complete by construction, with no lexical pattern
  consulted. That is **248 enumerated items**.
- **Judged.** Each is assigned to one classified *enumeration block* — the
  unit the session itself chose. The table lives in the script.
- **Asserted, in both directions.** Every enumerated item must land in
  exactly one block, and every block must resolve to at least one item.
  The script exits non-zero if either side has a leftover, so this document
  cannot be regenerated against a tree it no longer describes.

That is the shape `models/run.sh` uses to assert obligation coverage, and
it is here for the same reason: a clean result from a broken instrument is
indistinguishable from a clean result from a correct one.

### The counting rule, fixed before any count was taken

A **finding** is one item at the top level of the enumeration *the session
itself used*, in the body where that session wrote it. Sub-bullets inside
one item are not separate findings; each item's sub-item count is recorded
separately as a **granularity** figure, because that figure is what shows
the unit is not comparable across chains.

Four exclusions, each published rather than silent:

| class | what it excludes | why |
|---|---|---|
| `X` | an integration's opening re-listing of the review's findings | five integrations open by restating the findings verbatim in substance; counting both would double every one of them |
| `C` | charter prose — a review's doubts, written *before* it ran | a doubt is what the review was asked to check, not what it found |
| `D` | published residues — doubts discharged, checks that stood | deliberately not findings; a review that reports only defects leaves the next session unable to tell what was checked |
| `V` | verification and run lines | evidence inside a finding, not further findings |

**Three channels are counted separately and never summed into one curve.**
`T` tree-level review → integration; `G` findings the *integrating* session
raised itself; `I` the in-session reviewer allowance, recorded only in a
producer's or integration's own body.

**One arithmetic discrepancy was found and is resolved by the rule.**
`honest-classification-k80`'s prose says its reviewer returned eight
findings; its own enumeration lists nine. The enumeration is counted.

## Five of nine dispositions are not in `.grove/` at all

The brief that commissioned this measurement warned that three chains
write their *findings* only in the integration body, and that a
file-scoped counter would score them zero. That is true, and the larger
version of it is this:

**Five of the nine integrations wrote nothing back into their own task
body.** Their bodies are charters the **review** session wrote at cut
time — which is exactly what `references/decompose.md` prescribes, since
*the creating session writes the new leaf's body* — and the integrating
session never returned to them. Every one of those five recorded a
per-finding disposition **in its task commit** instead.

So `.grove/` alone cannot answer this document's question for five of nine
chains, and the commit messages can for all nine. The commit message is
also the *more* durable record: finish deletes `.grove/` from the tip but
never from history, and Grove's retire-then-commit rule puts the
disposition in a message that survives teardown. **The loop's own commit
discipline is what makes review yield measurable at all.**

## Per chain

`raised` counts channel T only. `sub` is granularity — the sub-corrections
inside those findings, which is the number that shows the unit is elastic.
`+G` is what the integration raised itself.

| # | chain | producer | raised | sub | real | unclear | dropped | +G | disposition recorded in |
|--:|---|---|--:|--:|--:|--:|--:|--:|---|
| 1 | `experiment-baseline-k29` | `requirements` | 6 | 20 | 6 | 0 | 0 | — | **commit `xpyqrvqy`** — all verified against source before acting |
| 2 | `model-contract-k31` | `design` | 6 | 18 | 6 | 0 | 0 | — | **commit `wxrmplzu`** — six blockers, each applied and described |
| 3 | `task-tree-k55` | `prototype` | 7 | 7 | 7 | 0 | 0 | — | **commit `xwzuynow`** — all seven verified and acted on; none was noise |
| 4 | `finish-k57` | `prototype` | 6 | 6 | 6 | 0 | 0 | — | **commit `tkyxvkss`** — triaged against the model; B1-B6 each applied |
| 5 | `system-k59` | `prototype` | 6 | 6 | 6 | 0 | 0 | — | **commit `xrtrwuwt`** — six findings integrated; F3's claim withdrawn as the fix |
| 6 | `obligation-placement-k67` | `design` | 3 | 3 | 2 | 1 | 0 | 2 | the body — ## Decisions (running log), per finding |
| 7 | `finish-scope-k75` | `design` | 4 | 4 | 4 | 0 | 0 | 5 | the body — ## Decisions (running log), per finding |
| 8 | `finish-verdicts-k77` | `design` | 2 | 2 | 2 | 0 | 0 | 1 | the body — ## Decisions (running log), per finding |
| 9 | `honest-classification-k84` | `design` | 5 | 5 | 4 | 1 | 0 | 2 | the body — ## Outcome, a five-row verdict table |
| | **total** | | **45** | **71** | **43** | **2** | **0** | **10** | |

**Zero findings were dropped and zero were classified noise.** Two were
downgraded within `integrate-review`'s own four-way triage from *a real
issue* to *a contract stated unclearly* — `obligation-placement-k67`'s F1
and `honest-classification-k84`'s finding 3. One review's proposed
*remedy* was rejected in favour of the other option it had itself offered
(`finish-scope-k76` on finding 2); the defect stood.

## A 100% survival rate is what a broken instrument reads

This is the number that most needs attacking, and the shape it has is the
self-certifying one this campaign kept finding in its own models.

**In five of the nine chains, the session that "verified" the findings
had its own task body written by the reviewer.** The charter *is* the
finding list, and `Done when` is the finding list restated as obligations.
An integration in that position has no structural place to disagree: to
reject a finding it would have to reject its own charter.

The counter-evidence is real, and partial:

- The four late integrations state in as many words that they verified
  against the artifacts rather than against the review's summary, and one
  of them found the defect was **worse** than the review knew
  (`finish-scope-k76`: Quint's `inv_FN_25a` was true by construction, which
  no mutation could have moved).
- `finish-verdicts-k78`'s in-session reviewer **falsified a finding that
  `obligation-placement-k68` had raised itself** — *no shared-safety
  obligation in this repository is stated over the quarantine reaper's
  actions* is false, and the true statement is the narrow one about the
  reaper's *ownership proof*. So findings in this tree do get rejected.
- Both downgrades above are real gradings, not rubber stamps.

**But no tree-level review finding has ever been rejected.** Every
rejection observed in this grove came from the in-session channel, where
the reader owes the finding nothing. That asymmetry is the single most
actionable thing in this document, and the repair is cheap: an
`integrate-review-*` leaf's body should carry the review's **handle**, and
its findings should be read from the review's own commit — the handoff a
`review-*` step already uses — rather than transcribed into the
integration's charter, so that rejecting a finding is not rejecting one's
own task.

## The in-session channel, and the five leaves that declined it

Three sessions spent the allowance `references/execute.md` grants, and
between them raised **24 findings: 17 valid and
actionable, 3 a contract the producer had stated unclearly,
1 a visible trade-off accepted, 3 noise**.
5 more leaves considered the allowance and did not spend it, and
the recorded reasons are not the same reason — which is itself the
finding, because only two of the five are about the harness.

| leaf | kind | spent | raised | actionable | unclear | trade-off | noise | why / what happened |
|---|---|:-:|--:|--:|--:|--:|--:|---|
| `model-contract-k5` | `design` | no | — | — | — | — | — | declined: the doubt is one an in-session reviewer cannot discharge — the enumerated-assumption control is itself part of what needs challenging |
| `ordinal-root-lifecycle-k14` | `prototype` | no | — | — | — | — | — | declined, and no review leaf cut either: cross-model-replay-k15 is chartered to contest the verdict, so a review leaf would buy the same read twice |
| `obligation-placement-k63` | `design` | no | — | — | — | — | — | could not be spent: the harness forbade subagents. A review-design leaf was cut instead, and is called the stronger instrument |
| `task-tree-scope-k70` | `design` | no | — | — | — | — | — | declined on the merits: four executable tests that would break if the central claim were wrong were spent instead, as stronger evidence than a fresh context reading the same prose |
| `finish-verdicts-k65` | `design` | no | — | — | — | — | — | could not be spent: the harness forbade subagents. A review-design leaf was cut and INSERTED, because otherwise the decision would get no fresh-context challenge anywhere |
| `finish-scope-k71` | `design` | **yes** | 7 | 5 | 1 | 0 | 1 | the reviewer BROKE the landed disposition; the reversal that followed touched nine artifacts and is what earned review-design finish-scope-k75. One item is 'a contract I stated unclearly' |
| `honest-classification-k80` | `design` | **yes** | 9 | 6 | 2 | 0 | 1 | prose says eight findings, the enumeration lists nine (3 substantive + 2 qualifying + 3 mechanical + 1 noise); the enumeration is counted and the discrepancy published |
| `finish-verdicts-k78` | `integrate-review-design` | **yes** | 8 | 6 | 0 | 1 | 1 | the only in-session spend by an integration. One of its findings FALSIFIED a finding obligation-placement-k68 had raised itself |

**The in-session channel is the only one with a non-zero drop rate**
— 4 of 24 findings (17%) were classified a
trade-off or noise, against 0 of 45 at the tree level. That is
the asymmetry the previous section is about, seen from the other end.

**All three spends changed the session that made them**, which is not what
a second opinion does.
`finish-scope-k71`'s reviewer *broke the disposition that had already been
landed*, forcing a reversal across nine artifacts — and that reversal is
precisely what earned the `review-design finish-scope-k75` chain, by the
mechanical second-review signal. `honest-classification-k80`'s reviewer
swept all 63 library invariants against the candidate's module and found
two violated, one of which nobody was looking for. Neither of those is a
second opinion; both are a different instrument. And `finish-verdicts-k78`
withdrew a sentence it had already written — *this is now a measurement
rather than an argument* — because its reviewer showed the retained claim
was still trivial over the difference it was meant to judge.

**And the declines are not excuses.** `task-tree-scope-k70` spent four
executable tests instead, on the argument that a test which breaks if the
central claim is wrong is stronger evidence than a fresh context reading
the same prose. `ordinal-root-lifecycle-k14` declined **and** cut no review
leaf, because a later leaf was already chartered to contest its verdict —
*a review leaf here would buy the same read twice*.

<a id="why-the-curve-cannot-be-read"></a>

## Why the curve cannot be read

Findings per review, in the order the chains ran: **6, 6, 7, 6, 6, 3, 4, 2, 5**.
First five mean **6.2**, last four mean
**3.5**. That looks like decay. Three things say it
is not readable as one.

**1 · The gap is inside the noise.** A 2.7-finding difference between a
five-chain and a four-chain group, against Poisson variation at this mean,
is about **1.8σ** — before any confound. Reaching 3σ on that effect needs
roughly **twelve chains per group, so about 25 chains of matched subject
size**.

**2 · The subjects are not matched, and not randomly so.** The first five
reviews read first-of-their-kind artifacts: a pre-registration, a
130-obligation catalogue, and three whole model columns. The last four read
incremental design decisions on an artifact that had already been reviewed
two to five times. Fewer findings against a smaller subject is not decay.

**3 · The review rate rose while the yield fell, and that is the
explanation.** Review is not uniform over the tree — it is a function of
**session kind**:

| producer kind | ran | reviewed | rate |
|---|--:|--:|--:|
| `requirements` | 1 | 1 | 100% |
| `design` | 12 | 5 | 42% |
| `prototype` | 23 | 3 | 13% |
| `impl` | 1 | 0 | 0% |

42% of `design` leaves earned a review against 13% of `prototype` leaves —
and all three reviewed prototypes are the Quint column, which introduced
the shared runner three columns inherit. The twenty Alloy prototypes were
covered by a scheduled `cross-model-replay-k15` instead. **The loop
reviewed further down its own value curve as it went**, and marginal
artifacts yield fewer findings. The falling count is a selection effect of
a rising review rate, and every review at the margin still found something.

**4 · The unit is elastic, which is the deeper problem.** Sub-corrections
per finding run from **1.0 to 3.3** across the nine chains —
`experiment-baseline-k29`'s six findings carry
20 corrections between them, while six
chains are strictly one-to-one. And that measured range still understates
it, because **consequence is not captured at all**:
`finish-verdicts-k77`'s two findings are two whole verdict reversals that
flipped `keep` to `defer` and falsified an ADR's title, and they count the
same as a corrected transition count. **More chains is therefore the
weaker fix; a pre-registered severity scale applied by the reviewer is the
prior one**, because without a fixed unit a longer run measures more
enumeration style rather than more review.

## What this weakens, and what survives

**Weakened: *review yield did not decay*.** It is not established by this
tree and cannot be. The recollection that produced it — three findings from
`honest-classification-k80`'s own reviewer, five more from `k84` beside it,
two more added by `k85` — is accurate, and it spans **all three channels**,
so it was never a statement about review chains in the first place. Read as
one, it generalises from a single node; read as what it actually describes,
it says something different and true: *three independent readers of one
artifact each found what the two before them had not*.

**Survives, and is countable exactly:**

- **9 reviews, 9 integrations, a clean pairing in both directions, and not
  one empty review.** A review that finds nothing creates nothing and
  retires; none did.
- **45 findings raised, 45 verified, 0 dropped, 0 noise** at the tree
  level — with the caveat above that no mechanism for rejection exists in
  five of the nine chains.
- **10 findings the integrations raised themselves**, in the four
  chains that recorded such a thing. Integration is not transcription: it
  found a defect worse than the review knew, a control that read green
  because its module never reached the subject, a clause it had itself
  deleted that was checking something true, and an invariant violated with
  no model mutation at all — found by **narrowing** the environment rather
  than widening it.
- **24 findings from three in-session reviewers**, one of which
  broke a landed disposition and one of which falsified a finding an
  integration had raised itself.

## Residues — both directions, published rather than asserted

| direction | expectation | result |
|---|---|---|
| forward — every enumerated item classified | 0 unclassified | **0** |
| reverse — every classified block resolves | 0 empty blocks | **0** |
| block counts match the enumeration | 0 mismatches | **0** |
| positive — `F5 — the flat-menu measurement` | found in `system-k59` | **found** |
| negative — `F7 —` | absent (that review has six findings) | **absent** |

The negative control is the one that matters. Five of the nine reviews
number their findings `F1…`/`B1…`/`R1…`, and an instrument that matched the
shape rather than the content would happily report a seventh finding in a
review that has six — the same defect `task-tree-k55` found in
`models/run.sh`, which accepted `inv_TT_99_misspelled` as an obligation
because it checked the shape and never the manifest.

**The controls were shown to fail before they were trusted.** A control
that has never been seen to fail is not a control — this corpus's own rule,
one level up. Three deliberate mutations of the classification were run
against the committed bodies:

| mutation | expected | result |
|---|---|---|
| delete the block classifying `task-tree-k55`'s `## Findings` | forward residue fires | **7 unclassified items, exit 1** |
| add a block naming a heading no body has | reverse residue fires | **1 empty block, exit 1** |
| state 6 findings where the body enumerates 7 | count control fires | **1 mismatch, exit 1** |

The first of those found a real hole. Until it was run, a block classifying
the document *root* owned the whole document, so deleting a section's
classification left its items silently absorbed and the forward control
read clean — an unrecognised failure defaulting to success, which is the
same defect `task-tree-k55` found in `models/run.sh`. The root now owns
only items at the root, and the mutation fires.

**A second bug was found the same way, by a count that did not match.**
Every markdown table was losing exactly one data row: the separator line
reset the parser's in-table flag, so the first real row was eaten as a
second header. It read plausibly — one fewer row in every table — and
only the hand-written expectation caught it.

**248 items enumerated, classified as:** `F` 62, `T` 59, `C` 37, `G` 33, `X` 30, `D` 26, `I` 1

`I` is 1 because the in-session reviewer's findings are written as
unordered bullets, which this enumeration deliberately does not treat as
items — every explanatory paragraph in these files uses them. That channel
is counted by hand from the sessions' own four-way classification, and its
table above is where it is checked.

## What this record does not establish

- **It counts findings, not value.** A finding that flipped two `keep`
  verdicts and a finding that corrected a transition count are one each.
- **It cannot see a finding nobody wrote down.** A review that noticed
  something and judged it not worth cutting an integration for leaves no
  trace; that is by design (*a review that finds nothing creates nothing*)
  and it means this document's floor is firmer than its ceiling.
- **It says nothing about what review cost.** Elapsed time per session is
  in [`loop-record.md`](loop-record.md); attributing it to review would
  need a measure of effort neither document has.
- **One grove, one subject matter, one operator.** Every chain here reviews
  formal-modelling work in a repository whose whole purpose was to be
  rigorous about evidence. That is the least likely place in the world to
  observe review yield decaying.
