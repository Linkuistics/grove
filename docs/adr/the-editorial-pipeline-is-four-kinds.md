# The editorial pipeline is four kinds

The publishing pipeline this campaign authors its books through is **draft →
copy edit → art → proof**, and it ships as four session kinds: `draft`,
`copy-edit`, `art`, `proof`. The developmental edit and the technical edit
become no kind at all — their charters are **folded into the draft**, which is
the merge target the measurement computed. Nothing is dropped, and the family
takes no `review-<stage>` or `integrate-review-<stage>` steps of its own.

Six stages were the candidate set (decision 11 of the campaign's requirements);
draft plus proof was the alternative they had to beat. Neither is what the
evidence supports.

## The trade-off

**The set was not chosen; it was read off an instrument written before the
evidence existed.** The pilot's preregistration fixed the judged outcome, the
alternative to beat, the attribution rule, the decision rule, and — because no
human sits between the measurement and this record — the table of *what each
verdict licenses*. `docs/evaluations/editorial-pipeline-pilot/README.md` filled
that table in over the `jj-workspace` book:

| Stage | `v` | `m` | `net` | Verdict | Consequence |
|---|---:|---:|---:|---|---|
| draft | — | — | — | not scored | the pipeline's first stage by construction |
| developmental edit | 3 | 2 | 2 | `Merge` | charter folds into **draft**; no kind |
| technical edit | 6 | 2 | 2 | `Merge` | charter folds into **draft**; no kind |
| copy edit | 6 | 5 | 5 | `Keep` | its own kind |
| art | 7 | 6 | 6 | `Keep` | its own kind |
| proof | 6 | — | — | not scored | the stated fallback, in both arms by construction |

`m` counts the marginal claims — those a simulated proof-only pass over the draft
did not reach. The threshold of three was fixed before any stage ran, on the
argument that a stage closing fewer than three distinct marginal defects across a
seven-chapter book contributes less than one defect every two chapters, which a
neighbouring stage absorbs without a session of its own.

**No stage is `Undetermined`, and that is the load-bearing fact.** The
preregistration names "a report that keeps a stage on the strength of an
`Undetermined` verdict" as the symmetric failure to a negative result, and fixes
that such a verdict licenses nothing — not extraction, not folding, and not a
substitute basis asserted downstream. There is no such verdict here, so nothing
in this record is an absence being read as a permission.

**Two of the four rows carry a qualification, and both are named rather than
smoothed over.**

- **Art's `Keep` rests on a counting adjudication, not on arithmetic alone.** Its
  six `A1` claims are counted as six because each needs its own corrective
  sentence. Under the defensible alternative — that "draw the relation as a
  table" is one family claim — art collapses to `net = 1` and a verdict of
  `Merge`. It is not `Drop` under either reading and not `Undetermined` under
  either; the collapse question is a judgement the rule settles rather than
  missing evidence. Extracting `art` therefore accepts the report's reading, and
  a future measurement that settles the counting rule the other way is a reason
  to revisit this row and no other.
- **Proof was never tested and could not have been.** It appears in both arms by
  construction and the comparator *is* proof applied to the draft, so no
  marginality is computable for it. Extracting it rests on its being the
  preregistered fallback. The descriptive split is worth carrying: one of proof's
  six claims was in its own whole-book class and five were in classes earlier
  charters owned, so in this pilot proof functioned as a net for other stages'
  misses more than as an independent contributor — but two of those five were
  handed to it deliberately by earlier stages, which is a pipeline working rather
  than a redundant stage.

**The fold has a precondition, and it is the most important limit on the page.**
Both merged stages fold into the draft, and the report states that this is safe
only while each book keeps getting a human-authored structure brief — precisely
the artifact that pre-empts the two folded charters. The pilot did not ask, and
could not have answered from one book, whether a drafting session *performs* those
charters or merely inherits the obligation. Two consequences follow and both are
binding: the `draft` kind states the two folded charters as its own obligations
rather than assuming the brief discharges them, and a `draft` session with no
structure brief stops and says so rather than drafting anyway.

**What the decision buys.** Two skills and two sessions per book, across the four
books that follow, against the six-stage design; and fifteen marginal defects
across a seven-chapter book, against the two-stage fallback — including every one
of the book's six tables and five of its six copy corrections, neither of which a
draft-plus-proof pipeline reaches. A result that had upheld two stages would have
been just as successful an outcome of the pilot; the measure did its job in both
directions at once.

## Considered options

- **Extract all six stages.** Rejected on the arithmetic: two of the four middle
  stages did not clear a threshold fixed before any of them ran. This is the
  outcome the whole preregistration exists to prevent — a pipeline shipped because
  it was already designed, called derived after the fact. Reopen only on a new
  preregistered measurement, over a book drafted the way the campaign's remaining
  books actually are.
- **Extract draft and proof only, the stated alternative.** Rejected on the
  arithmetic in the other direction: fifteen of the twenty-two defects the four
  middle stages closed were not reached by a proof-only pass over the draft. Two
  whole classes of work — the copy edit's terminology and prose corrections, and
  every table the art stage drew — the two-stage arm simply does not get.
- **Give the two merged stages a kind anyway, on the strength of their being one
  claim short.** Rejected because it is the threshold being chosen after the
  counts are visible, which is the one move preregistration exists to forbid. The
  margin is recorded here so the result is not read as a rout; recording a margin
  is not the same as spending it.
- **Reuse `impl` for the draft rather than minting a kind.** Tempting: it costs no
  token and no launch template, and the campaign's book leaves are `impl` leaves
  today. Rejected on two counts. Folding the developmental and technical charters
  into `impl` would bind every `impl` session in every grove to a structural and
  technical-accuracy discipline over a document it is not writing — a merge target
  computed for one pipeline leaking into an unrelated one. And `draft` carries a
  precondition `impl` must not gain: without a structure brief it stops. Reopen
  never; the fold is what makes them different kinds.
- **Prefix the tokens — `book-draft`, `book-copy-edit`.** Rejected because it
  confuses a finding with a discipline. The report's limit is that its *evidence*
  is about this campaign's remaining books rather than about editorial pipelines
  in general; the disciplines themselves are stated over a document and the
  contract that document declares, with every class supplied by that contract. A
  prefix bakes this repository's subject into a token installed on every machine,
  making the kind unusable for the next document pipeline while still shipping it
  everywhere. Reopen if a second, genuinely different editorial pipeline ever
  needs a `draft` of its own, which would make the collision real rather than
  hypothetical.
- **Give the family review chains, as the five producer kinds have.** Rejected
  because nothing measured them: eight more kinds on no evidence is this
  campaign's governing rule broken one level up. It is also redundant by
  construction — each later stage is a fresh-context read of the whole book
  against its own charter, so every stage but `proof` has its adversarial read
  scheduled, and `book-check --final --check all` decides the mechanical half that
  a review could not. Reopen if a stage is ever found producing defects the
  following stage's charter does not own, which is the case a review would have
  caught and this pipeline cannot.
- **Extract nothing and keep authoring books by hand.** Rejected: it discards a
  positive result. The pilot did not fail to find a pipeline; it found a smaller
  one than the campaign came in holding, and two of the four stages it kept are
  the two the campaign would have been least confident about in advance.

## What would reopen this

- **A second book's measurement.** This is one book, the smallest new corpus in
  the campaign — four roots, 698 lines, seven chapters — with no stage run twice
  and nothing replicated.
- **A book drafted without a structure brief**, which removes the condition the
  fold depends on. That is not a reason to un-merge two stages by assertion; it is
  a reason to measure again.
- **A `draft` re-run leaf cut against the same book more than once**, which is the
  observable form of the draft failing to carry a folded charter.
- **A settled counting rule that collapses a family of like claims**, which moves
  art's row and nothing else.

The evidence behind this record is
`docs/evaluations/editorial-pipeline-pilot/README.md` and the
[preregistration](../evaluations/editorial-pipeline-pilot/preregistration.md)
committed before the book it judges. How a stage that finds work an earlier stage
owns expresses that in a tree with no cycles is
[a feedback edge is forward tree growth](a-feedback-edge-is-forward-tree-growth.md);
what the art stage's medium may be is
[a book carries no asset](a-book-carries-no-asset.md); and why adding four kinds
is authoring four skills rather than editing the binary is
[a kind is an open token](a-kind-is-an-open-token.md).
