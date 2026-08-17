# A behavioural rule is covered by asserting its delivery

A rule the inventory classes **B** — *observable in a session's conduct* — is
enforced by a deterministic assertion that the rule is **present and reachable on
the composed loaded path of every kind it binds**, not by an evaluation that
watches a session behave. The instrument is a normalised paragraph match over the
loaded path the runtime actually composes (`src/prompt.rs`'s guaranteed core,
`content/SKILL.md`, `reference_file(kind)`, and the transitive closure of the
corpus files those name by path), and it runs on every `cargo test`.

**Reachable means reached by *this rule's* condition.** The walk is file-grained,
because that is what the owner record says an edge reaches — a file, whose in-file
conditions are then all available. Delivery is not: a rule counts as delivered
only when it is stated on the **static** path, which needs no condition, or when a
paragraph on the path states **that rule's situation** and the procedure sits in
that paragraph's own file or in a file it names by path. Following any mention of
the owner file instead would certify the case the owner record calls deleted in
effect — two conditions name one file, one of them is dropped, and the procedure
for the dropped one still reads as delivered to a session that has nothing telling
it to look.

**What binds is the boundary, stated rather than blurred.** The assertion covers
*the session was told, on a path it opens for its own kind*. It does not cover
*the session obeyed*, and no green run is evidence of obedience. That gap is the
whole cost of the decision and it is paid knowingly: the failure the corpus
refactor can actually commit is deleting a rule or rehoming it where no session
opens it, and that failure is exactly what a delivery assertion catches.

## Why not a model-in-the-loop evaluation

An evaluation that drives a real session and scores its conduct would measure the
property the class names. It is rejected as the *standing* instrument on three
grounds, in order of weight:

1. **It is not repeatable, so it cannot be a net.** The rewrites need a check
   that is green before them and green after them, run by every contributor on
   every commit. A scored session is a sample; a corpus rewrite that weakened a
   rule would fail it sometimes.
2. **It cannot localise.** A session that mis-conducts itself does not say which
   sentence went missing. The delivery assertion fails naming the rule, the kind,
   the unmet claim and the files that still state it.
3. **Cost.** Nineteen kinds times twelve rules is not an occasional expense.

The rejection is of the instrument's *place in the suite*, not of its value. A
model-in-the-loop measurement is the right tool for the claim delivery cannot
make — whether a session reads what it is handed — and this repository already
uses it there, deliberately and out of band, in
`docs/research/wording-micro-test.md`. **What would reopen this** is a rule whose
failure mode is invisible to delivery: one every session is demonstrably told and
demonstrably ignores. That is a measurement, not a suspicion, and it argues for an
eval *beside* these assertions rather than instead of them.

## Considered options

- **A phrase-uniqueness sweep over the whole corpus.** Rejected because it
  answers a different question. A sweep proves one file states a rule; it cannot
  tell a rule stated in a file a session opens from one stated in a file no
  session's path reaches, and the second is deleted in effect. The corpus already
  holds that failure. The two instruments are complements — uniqueness is the
  **S** half of a `B+S` row — and neither substitutes for the other. Reopen never;
  this is a scope statement, not a rejection.
- **Match against the whole embedded corpus rather than a per-kind path.**
  Rejected because it passes on precisely the regression the refactor risks: a
  rule moved into an unreachable file. Its one attraction is that it never goes
  red spuriously, which is the same thing said as a defect. Reopen if
  `src/prompt.rs` ever hands every kind every file, which would make the two
  measures identical.
- **Pin each rule to the file that states it today.** Rejected: eight leaves
  rehome rules on purpose, so a site pin fails on the intended change and reports
  nothing about the unintended one. The site is diagnostic output on failure, not
  an assertion.
- **Exact-phrase assertions instead of normalised alternative groups.** Rejected
  because a rewrite is licensed to reword, so an exact pin turns every legitimate
  edit red and trains a reader to re-point the test rather than read it. The
  groups name each rule's load-bearing clause and accept any wording carrying it;
  what keeps that from degenerating into a topic match is a **near-miss fixture per
  claim** — a plausible rewrite that keeps the rule's subject and drops *that
  claim's* force, and must fail. Per claim rather than per rule, because one
  fixture controls only the claim it fails: the rest are then free to be two common
  words, or to accept the rule's inverse, while the invariant still goes red on a
  sibling. Where co-occurrence cannot express the clause — a required conjunction
  that a sentence may name while deferring one term, or an instruction that
  contains another as a substring (`complete` inside `complete --done`) — a claim
  also names the wordings that **disqualify** a paragraph. Reopen for a rule whose
  wording is frozen: the two signal files are byte-frozen for this workstream, so
  their phrases are pinned exactly.
