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
