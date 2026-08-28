# harvest-the-loop-record-k87 — brief

## Goal

Turn `.grove/` into a durable, cited evidence base about **how this LLM loop
behaved** — before anything can delete it — and cut the lesson leaves that the
evidence, rather than the recollection, supports.

## Context

**Why this leaf is first, and why it is urgent rather than merely early.**
`.grove/` is process state: grove deletes it at the finish cycle, by design
(constraint 6 — walk away and the tree is gone). Every other input to the lessons
workstream is durable — the models, the READMEs, `docs/formalism-findings.md`,
the ADR set — and can be read at leisure. This one cannot. The root brief says in
as many words that **nothing may run `finish` until this leaf has landed**.

**What is actually in there, and it is not a log.** The tree carries, per task:
the kind that ran, the brief chain it read, the body someone wrote for it before
it ran, and — for the retired ones — what that session recorded on the way out.
It also carries the *shape* decisions: which leaves decomposed into nodes, where
reviews were **inserted** rather than appended and why, which review chains were
cut and by whom, and which leaves were abandoned and at what point. Cross-read
against `jj log`, each leaf has a commit, so cost and elapsed time per session
are recoverable.

**The one measurement that motivated the whole workstream, and it needs checking
hardest.** From inside, review yield looked like it did *not* decay:
`honest-classification-k80`'s own in-session reviewer returned three substantive
findings against a green suite; the `review-design honest-classification-k84`
leaf beside it returned five more against `k80`'s repairs; and
`integrate-review-design honest-classification-k85` found two further defects
while verifying those five — one of them a shared-safety claim violated with no
model mutation at all. **That is one node.** Whether it generalises across the
tree is exactly what this leaf can answer and nobody has: count findings per
review across every chain in `.grove/`, classify them by whether they were
verified or dropped at integration, and say whether the curve is flat, rising or
falling. A flat curve is an argument against "one pass then ship" that costs
something to make; the recollection of one node is not.

**Attack the recollection, not just record it.** The root brief's *Candidate
lessons* were written from inside the campaign by a session that is not neutral
about it. Each is a claim to be evidenced or **dropped**, and the dropping matters
more than the keeping: a lessons artifact whose every prior belief survived
contact with its own evidence has not checked anything. Name, for each candidate,
where it is measured and what would have falsified it.

**Do not re-derive what the models already record.** The Quint and Alloy columns
already carry their own findings, controls, narrowings and *what a green run does
not prove*. This leaf's subject is the **loop** — sessions, reviews,
decompositions, abandonments, cost — and it reads the model artifacts only where a
loop-level claim needs one for evidence.

**A repo-wide claim is not evidenced by a clean grep.** Any count stated here —
findings per review, sessions per node, reviews per producer — is a claim over
the whole tree, so enumerate and classify rather than sweeping a pattern, and
control the instrument in both directions before trusting it. `handoff-audit-k66`
carried this discipline and was abandoned; its instrument note is worth reading
in `.grove/01-formal-modeling-k1/13-formal-synthesis-k16/11-ABANDONED-design-handoff-audit-k66.md`.

## Done when

- A durable artifact outside `.grove/` — under `docs/`, so it survives finish —
  carries the loop's record: every session with its kind, its node, its outcome
  (retired or abandoned), and its commit. Derived, not transcribed by hand, with
  the derivation reproducible.
- **The review-yield question is answered with a number and a method**, across
  every review chain in the tree and not just the one that prompted it: findings
  raised, findings that survived verification at integration, findings the
  integration added on its own. If the tree is too small to answer it, that is
  the answer and it is stated as such rather than padded.
- Each of the root brief's six candidate lessons is marked **evidenced**, with
  citations, or **dropped**, with the reason. At least one is expected to be
  dropped or materially weakened; if none is, say why the prior was that good.
- The cost account exists in raw form: sessions, elapsed time and commits per
  phase, so the write-up can say what the campaign cost without re-deriving it.
- The lesson leaves for step 2 of the root decomposition are **cut**, with bodies
  carrying the specific evidence each one is about — not a generic goal sentence.
  How many there are is this leaf's finding, not a number to hit.
- Nothing in `.grove/` is edited. Retired and abandoned bodies are the record;
  reading them is the work, rewriting them would destroy the subject.

## Decomposition

**The trigger fired, and it fired on the derivation exactly as this charter
predicted.** The parent body's own Notes named three separable pieces and one
condition: *if the derivation turns out to need its own instrument with its own
controls, it is a child and not a section.* It does. Four ways the obvious
instrument reads clean and lies were measured before any child was cut, and each
is recorded under Pointers below. So the harvest is a node, and its children are
cut lazily — one at a time, each by the session that has just learned what the
next one is.

The ordering is by **perishability first, then consumption**:

1. `loop-record-k88` — the structural record and the cost account. First because
   `.grove/` is the perishable input and because every later child consumes the
   enumeration this one produces. Cut and executed by the decomposing session.
2. **The review-yield question.** Cut by `k88`, which will know how many chains
   there are and where each chain's findings are actually written down. Held
   separate because counting findings needs a *classification* instrument with
   its own two-direction control, and because the count is the parent's most
   load-bearing claim.
3. **The candidate-lesson adjudication.** Evidenced or dropped, one verdict per
   candidate, each citing where it is measured and what would have falsified it.
   Held separate because it reads the model artifacts as well as the tree, and
   because it must be able to contradict (2) rather than inherit it.
4. **Cutting the lesson leaves** — the root decomposition's step 2. Last, because
   how many clusters there are is a finding of (2) and (3) rather than a number
   to hit. It may fold into (3) if the clusters are obvious by then.

Only child 1 exists. Do not pre-cut the rest: a leaf here is cut by the session
that can write the specific evidence into its body, which is the whole payoff of
cutting late.

## Pointers

- `docs/loop-record.md` and its derivation script — child 1's output. Every later
  child reads the record rather than re-walking `.grove/`, and re-derives rather
  than trusting it when a count matters.
- **Four instrument hazards, all measured, all of which read clean.** A session
  here that reaches for a count meets at least one of them:
  - **The commit store is shared by four jj workspaces**, and their key spaces
    collide — `blinded-read-k27` belongs to a different workstream while this
    grove's `k27` is `impl-collapse-application-k27`. A join on `k<key>` alone
    silently imports another grove's commit. Scope every revset to this
    workspace's own ancestry, and join on the full `<slug>-k<key>` handle.
  - **This repository has run at least seven groves in the same directory.**
    Commits touching `.grove/` across that ancestry number in the hundreds; this
    grove's window is a small fraction of them, anchored at the commit that
    scaffolded this tree. An unanchored `files(".grove")` overcounts by more
    than an order of magnitude and reports a plausible number.
  - **An earlier grove used a different filename grammar** (`NNN-<slug>.md`, no
    kind and no key). Any parser applied outside this grove's window must refuse
    rather than degrade.
  - **A review's findings are not always written in the review leaf.** Several
    chains record them only in the integration body. A file-scoped counter scores
    those chains zero and reports a *decaying* yield curve — the opposite of the
    truth. Count per chain, over the review body and its integration together.
- `handoff-audit-k66` (abandoned, in this tree) — the two-direction control
  discipline stated as a working sweep, and the `#Lnnn`-fragment note: the same
  instrument failing in the opposite direction, reporting 28 broken links of
  which 25 were fine.
- `finish-verdicts-k65` (retired, in this tree) — the leaf that burned a handle
  by letting a sweep's report live inside the sweep's own subject.

## Notes

**This leaf reads its own tree, which is the instrument hazard this campaign kept
meeting.** A sweep whose report lives inside its own subject measures itself —
`finish-verdicts-k65` burned a handle that way, and its note is still in the
tree. The artifact goes to `docs/`, and any count that would change by writing
the artifact is computed and frozen before it is written.

**Expect this leaf to decompose rather than run long.** Deriving the record,
answering the yield question and cutting the lesson leaves are three separable
pieces, and the decomposition trigger is the ordinary one: if the derivation
turns out to need its own instrument with its own controls, it is a child and not
a section.
