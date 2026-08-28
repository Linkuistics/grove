# review-yield-k89

## Goal

Answer the parent's second `Done when` clause with a number and a method:
**across every review chain in this grove, how many findings were raised, how
many survived verification at integration, and how many the integration added on
its own.** Say whether the curve is flat, rising or falling — or say that nine
chains cannot carry the question, if that is what the enumeration shows.

The deliverable is a durable artifact under `docs/` beside
[`docs/loop-record.md`](../../docs/loop-record.md), and it must be re-derivable
the same way: from `.grove/` read at a pinned revision, not from a transcription.

## Context

**The enumeration is already done and is not yours to redo.**
`docs/loop-record.md` lists all 72 leaves, all 16 nodes, and the nine review
chains, with their commits, and its controls are published in it. Read it. What
it deliberately does not do is open a single review body — that is this leaf's
whole subject.

**The nine chains, and every one of them found something.** No review in this
grove came back empty: nine `review-*` leaves, nine matching
`integrate-review-*` leaves, a clean pairing in both directions. That is itself a
result and it is already recorded; what is not known is *how much* each found.

**The hazard that decides whether the number means anything: findings are not
where you would look for them.** Three reviews carry no findings section at all
and their findings survive only inside the integration body:

| chain | where the review's findings are written |
|---|---|
| `experiment-baseline-k29` → `k30` | **only in the integration**, as `## Findings to integrate`, six numbered items |
| `model-contract-k31` → `k32` | **only in the integration**, as `### model-contract-k31 B1…B6` |
| `finish-verdicts-k77` → `k78` | **only in the integration**, as `### R1`, `### R2` |
| `task-tree-k55` → `k56` | in the review, `## Findings`, seven items |
| `finish-k57` → `k58` | in the review, `## Review findings`, `B1…B6` |
| `system-k59` → `k60` | in the review, `### Findings`, `F1…F6` |
| `obligation-placement-k67` → `k68` | in the review, `## Findings`, `F1…F3` |
| `finish-scope-k75` → `k76` | in the review, `## Findings`, four `[P1]`/`[P2]` items |
| `honest-classification-k84` → `k85` | in the review, `## Findings`, five items |

**A file-scoped counter reads the first three chains as zero and reports a
decaying yield curve — the exact opposite of the truth.** Count per *chain*, over
the review body and its integration together, and treat the pair as one
population with two dispositions.

**There is no notation to regex for, and reaching for one is the trap.** Those
nine chains use at least six enumeration styles: `1. **High — …**`,
`1. **[P1] …**`, `### F1 —`, `### B1 —`, `### 1.`, `### R1 —`. And
`integrate-review-design-honest-classification-k85` enumerates nothing: it
disposes its five inherited findings in an `## Outcome` prose section. A pattern
list is complete only as far as the list, and lengthening it moves the leak
rather than closing it. **Enumerate by reading each of the eighteen bodies and
classify what you find**; the tree is small enough that this is affordable and
large enough that nothing else is honest.

**The word *findings* is overloaded inside a single body, and it will corrupt a
keyword count.** `review-prototype-task-tree-k55` uses it in its `Done when` for
the **producer's** five declared findings — *"the five findings survive a hostile
read"* — and in its `## Findings` section for its **own** seven raised findings.
Two populations, one word, one file. Decide which population a mention belongs to
by reading it, and record the rule you used.

**The third channel, which the parent's candidate lesson depends on and which is
not in the chain table at all.** Producers also spent the leaf-wide in-session
doubt allowance (`references/execute.md`), and those findings are recorded in the
*producer's own* body, never in a `review-*` leaf. Eleven task bodies in this tree
discuss an in-session reviewer, and
`integrate-review-design-finish-verdicts-k78` carries a whole section titled
*"The in-session reviewer, and every finding classified"*. The motivating
observation in the root brief — three findings from `honest-classification-k80`'s
own reviewer, five more from `k84` beside it, two more added by `k85` — spans all
three channels, so a measurement that reports only the tree-level channel cannot
speak to it. Count the channels separately and say so.

**One node is not a curve.** The root brief's claim rests on `k80`/`k84`/`k85`.
Whether it generalises is the question; a result that merely re-states that node
with more words has not answered anything.

## Done when

- A durable artifact under `docs/` carries, per chain: findings raised, findings
  verified and acted on at integration, findings dropped or reclassified at
  integration, and findings the integration raised itself. Every count cites the
  body and the section it came from.
- **The classification rule is stated before the counts and applied to every
  finding**, including the ones it makes disappear. `integrate-review-*`'s own
  four-way triage — *a contract stated unclearly*, *a real issue*, *a real
  trade-off accepted visibly*, *noise* — is the obvious taxonomy and the sessions
  themselves used it; if you depart from it, say why.
- **Both directions are controlled.** Every finding counted resolves to a body
  and a section; every enumerated item in all eighteen bodies is either counted
  or explicitly excluded with a reason. Publish both residues.
- The in-session-reviewer channel is counted separately, or its absence from the
  count is stated as a limit with the reason.
- **The verdict is stated plainly, including the null one.** Flat, rising,
  falling, or *nine chains and three channels cannot separate these hypotheses* —
  and if it is the last, say what number of chains would have, so the write-up
  can report a bound rather than a shrug.
- Nothing in `.grove/` is edited beyond this leaf's own body.
- The next child is cut — the candidate-lesson adjudication — with the specific
  evidence this leaf found written into its body, including anything here that
  weakens the parent's *review yield did not decay* candidate.

## Notes

**This leaf may not decide a lesson.** Adjudicating the root brief's six
candidates is the next child's, and doing it here would let the yield number and
its interpretation be written by the same session, which is the self-certifying
shape this whole campaign kept finding in its models.

**Expect to weaken the prior, and treat that as the success condition.** The
candidate this measurement exists to test was written from inside the campaign by
a session that is not neutral about it. If every chain comes back rich and the
curve is flat, check the classification rule hardest — a taxonomy generous enough
to count restatements as findings will produce a flat curve out of any tree.

## Decisions (running log)

**The counting rule, fixed before any count was taken.** A *finding* is one item
at the top level of the enumeration **the session itself used**, in the body
where that session wrote it. Sub-bullets inside one item are not separate
findings; each item's sub-item count is recorded separately as a **granularity**
figure, because that figure is what shows the unit is not comparable across
chains. Excluded from the count, with reasons published: the producer's own prior
findings that a review was chartered to re-check (`task-tree-k55`'s two
populations of the word), doubts a review explicitly *discharged*, and a
session's statements about its own limits. Where a session's prose count and its
own enumeration disagree, **the enumeration is counted and the discrepancy is
published** — this happened once (`honest-classification-k80`).

**Three channels are counted separately and never summed into one curve.**
(T) tree-level `review-*` → `integrate-review-*`; (I) the in-session reviewer
allowance, recorded only in a producer's or integration's own body; (G) findings
the integrating session raised **itself** rather than inheriting.

**The disposition of every chain is recorded — five of nine only in the commit
message, and that is the leaf's first substantive finding.** Four integrations
(`k68`, `k76`, `k78`, `k85`) wrote a per-finding verdict into their own
`## Decisions (running log)` or `## Outcome`. The other five wrote nothing back:
their bodies are charters the **review** session wrote at cut time, exactly as
`references/decompose.md` prescribes, and the integrating session never returned
to them. Their dispositions are in the task commit, in full, per finding. So
`.grove/` alone cannot answer this leaf's question for 5 of 9 chains, and the
commit messages can for all 9 — and the commit message is the *more* durable
record, because finish deletes `.grove/` from the tip but never from history.

**Instrument hazard measured and published: `ABANDONED` contains the substring
`DONE`.** A glob or grep on the retire infix counts every pruned leaf as retired.
It read 54 producers where there are 37, and it read clean. The correct
instrument anchors on the position: `NN-DONE-<kind>-`.

**Tree-level result: 45 findings raised across nine chains, 45 verified real,
zero dropped, zero classified noise.** Two were downgraded within the four-way
triage from *a real issue* to *a contract stated unclearly* (`k68`'s F1,
`k85`'s finding 3). One review's proposed *remedy* was rejected in favour of the
other option it offered (`k76` on `k75`'s finding 2); the defect stood.

**A 100% survival rate is what a broken instrument reads, and the shape here is
exactly the self-certifying one this campaign kept finding.** In five of nine
chains the session that "verified" the findings had its own task body **written
by the reviewer**: the charter *is* the finding list, so the integration has no
structural place to disagree. The counter-evidence is real but partial — the four
late integrations state they verified against the artifacts rather than the
review's summary, and `k78`'s in-session reviewer **falsified a finding `k68` had
raised itself**. Verification demonstrably happens and catches things; it has
never once been observed to reject a tree-level review finding.

**In-session channel: three spends, 24 findings, 21 actionable, 2 noise, 1
visible trade-off.** `finish-scope-k71` (7), `honest-classification-k80` (9
enumerated / 8 stated), `finish-verdicts-k78` (8). Four producers considered the
allowance and declined it, each with a recorded reason, and the reasons are not
the same: `model-contract-k5` — the doubt is one an in-session reviewer cannot
discharge; `obligation-placement-k63` and `finish-verdicts-k65` — the harness
forbade subagents; `task-tree-scope-k70` — four executable tests were spent
instead, as stronger evidence than a fresh context reading the same prose.

**The apparent decay is fully explained by a rise in the review rate, and the
rate is a function of session kind rather than of time.** Findings per review by
chain, in order: 6, 6, 7, 6, 6, 3, 4, 2, 5 — first five mean 6.2, last four mean
3.5. But review rate by producer kind is `requirements` 1/1, `design` 5/12 (42%),
`prototype` 3/23 (13%), `impl` 0/1; and all three reviewed prototypes are the
Quint column, which introduced the shared runner three columns inherit, while the
twenty Alloy prototypes were covered by a scheduled `cross-model-replay-k15`
instead (`ordinal-root-lifecycle-k14` records the reasoning: *a review leaf here
would buy the same read twice*). The loop reviewed further down its own value
curve as it went, and marginal artifacts yield fewer findings.

**Verdict: null, and the bound is stated rather than shrugged.** Nine chains and
three channels cannot separate flat from falling. The 2.7-finding gap between the
early and late groups is ≈1.8σ against Poisson noise at this mean — before the
subject-size confound, which is not random: the first five reviews read
first-of-their-kind artifacts (a pre-registration, a 130-obligation catalogue,
three whole model columns) and the last four read incremental design decisions on
an artifact already reviewed two to five times. **~25 chains of matched subject
size** would reach 3σ on that effect. But more chains is the weaker fix: the unit
itself is elastic by a factor of about six — `experiment-baseline-k29`'s six
findings carry ~16 sub-corrections while `finish-verdicts-k77`'s two are two
whole verdict reversals — so the prior repair is a **pre-registered severity
scale applied by the reviewer**, without which a longer run measures enumeration
style.

**What this weakens in the parent's candidate lesson.** *Review yield did not
decay* is not established, and cannot be from this tree. What **is** established,
and is the stronger claim: **no review in this grove came back empty, at any
point in the campaign, including the four cut latest against the smallest
subjects** — and the loop's response to that was to review more, not less.

**The instrument was built to fail, and it did, twice, before it was trusted.**
`scripts/review-yield.py` derives the *enumeration* — every heading, top-level
ordered-list item and table row in the eighteen bodies, complete over markdown
block structure rather than by a pattern list — and joins it against a
hand-written classification, asserting coverage **in both directions** and
exiting non-zero on any leftover. Three deliberate mutations were run against it
and all three fire (published in the artifact's residue section). The first found
a real hole: a block classifying the document *root* owned the whole document, so
deleting a section's classification left its items silently absorbed and the
forward control read clean — **an unrecognised failure defaulting to success,
which is the same defect `task-tree-k55` found in `models/run.sh`.** The second
bug came from a count that did not match: every markdown table was losing exactly
one data row, because the separator line reset the parser's in-table flag and the
first real row was eaten as a second header. It read plausibly.

**No ADR is owed, and the AND test is why.** The one candidate was the counting
rule — block grain rather than item grain, unordered bullets treated as prose —
and it fails *hard to reverse*: re-deriving under a different rule costs one
script run, and the rule is stated in the artifact before the counts. The finding
with real ADR shape is the repair below, and it is **not this leaf's to decide**:
methodology changes are the root decomposition's step 3, and this leaf's own
Notes forbid deciding a lesson here.

**The repair the measurement implies, handed forward rather than taken.** An
`integrate-review-*` leaf's body should carry the review's **handle** and read its
findings from the review's own commit — the handoff a `review-*` step already
uses — rather than having the review transcribe them into the integration's
charter. Today the charter *is* the finding list and `Done when` is that list
restated as obligations, so an integration cannot reject a finding without
rejecting its own task. That is the mechanical explanation for 45 of 45 surviving,
and it is a change to `references/decompose.md`'s chain rule, not to this tree.

**`docs/loop-record.md` is deliberately left at its pin.** It is a derived
snapshot at change `wzmyysov`, self-describing and re-runnable, so it does not
list `k89` or `k90`. Every count this leaf needed from it — nine chains, 37 ran
producers, the review rate by kind — was **re-derived independently** against the
current tree, as the parent brief requires when a count matters, and the
`ABANDONED`-contains-`DONE` hazard was found doing exactly that.

**The next child is cut: `candidate-lessons-k90`**, the candidate-lesson
adjudication, with this leaf's specific evidence in its body — including the four
bullets that weaken *review yield did not decay*, the five independent instances
of *a green suite is not evidence*, the two of *false greens are found by
narrowing*, the two candidates whose wording and evidence may not be about the
same thing, and the three measured instrument hazards. It is told it may
contradict this leaf's interpretation, and that a clean sweep of six surviving
candidates would mean the adjudication checked nothing.
