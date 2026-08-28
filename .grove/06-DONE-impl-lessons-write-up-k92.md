# lessons-write-up-k92

## Goal

Write the durable account of what this repository's formal-modelling campaign
taught about driving an LLM loop to produce checkable work — the root
decomposition's step 4, and the last leaf of the lessons workstream.

Two things it owes that nothing else in this tree does. It carries **what could
not bind**, because the methodology has no session that could violate it. And it
carries the **cost account**: what the campaign spent, what it returned, and
which parts of it were not worth the money.

## Context

**Everything you need is in `docs/`, and none of it is in `.grove/`.** Read, in
this order:

- [`docs/candidate-lessons.md`](../docs/candidate-lessons.md) — six candidates
  adjudicated, each with the file that decides it and what would have falsified
  it. Three moved: one dropped outright, one materially weakened with half its
  sentence falsified, one split into two claims resting on very different
  evidence. Its §*What binds, and what only gets written down* names the three
  clusters, and its §*Where the three landed, and what did not* is
  `methodology-changes-k91`'s output, written back into it.
- [`docs/loop-record.md`](../docs/loop-record.md) — every session with kind,
  node, outcome and commit, plus **the cost account in raw form**. Derived by
  `scripts/loop-record.py` at a pinned change id, so it re-derives after
  teardown. **Do not re-derive the cost account**; it is the raw form and this
  leaf's job is to read it, not to recompute it.
- [`docs/review-yield.md`](../docs/review-yield.md) — all eighteen review-chain
  bodies opened, the yield question answered, and §*Why the curve cannot be read*
  saying why the obvious reading of the number is unavailable.
- [`docs/formalism-findings.md`](../docs/formalism-findings.md) — the bounded
  Alloy 6 / Quint comparison, with per-entry costs and retained counterexamples.

All four are now reachable from `docs/ARCHITECTURE.md`'s *Documentation
ownership* table, which is where a stranger looks.

### What bound, and what did not — this is the split you must be honest about

`methodology-changes-k91` landed **three** rules in `content/`, and each is now
an inventory row in `docs/specs/corpus-rule-ownership.md` and a phrase in
`tests/rule_ownership.rs`'s single-source sweep:

- **A′** — *a control that has never been seen to fail is not a control*, into
  `references/execute.md`'s two-controls block.
- **B** — the provenance discipline, as `references/execute.md`'s new *The
  provenance of a measurement* section: finish every edit then measure; digest
  every subject either side; one measurement, one writer; a launcher's return is
  not a background job's completion; confirm a re-run item by item.
- **C** — *an integration's body carries the review's handle, never its
  findings*, across `references/decompose.md` (what a review writes into the body
  it cuts) and `references/integrate-review.md` (where an integration reads
  findings from). `decompose.md`'s *"or the findings verbatim"* is gone.

**Four things were considered and declined, and the write-up owns all of them:**

1. **Candidate 5b, the predicate half** — *a check written by calling the
   definition it is about stops measuring the moment that definition is
   repaired*. Declined because `candidate-lessons.md` §5 split 5a from 5b
   precisely so that 5b's single instance would not borrow 5a's five, and
   landing them side by side as one methodology rule re-joins the sentence.
   It is the cheapest failure to miss — nothing errors, the number just changes —
   so it is worth the write-up's space.
2. **A control shown to kill for the reason claimed**, established by the
   *isolating* measurement. The strongest form candidate 1 reached, and the
   corpus's own evidence for it is three named controls that were killing
   through the wrong mechanism. It is about model suites; Grove runs none.
3. **A module's environment counted as part of its control**, and **every module
   run against every claim the model has rather than the ones it declares**
   (candidate 4). Same reason.
4. **Candidate 2's model-level form** — a claim stated over the model's own
   history or classifier is self-certifying, and the repair recreates the hazard
   one level down. Its *loop*-level instance did bind, as C.

The generalisation the declines share is worth stating plainly rather than
apologising for: **a lesson binds only when there is a session that could violate
it.** Four true, well-evidenced lessons had no such session.

## Done when

- The account exists as a durable document, and **a stranger can read it**: no
  handle, model, obligation id or session key appears without enough context to
  mean something to someone who has never opened this repository. Handles are
  fine as citations; they are not fine as the explanation.
- **Every claim cites where it is measured**, and a claim that survives only as a
  recollection is dropped. That bar is the harvest's and it is not relaxed here —
  `candidate-lessons.md` §*What was checked, and how* records why (two citations
  in this tree named a test for what its filename suggested rather than what it
  asserts).
- **The cost account is stated, not implied**, from `loop-record.md`'s raw form:
  what was spent, what came back, and which parts were not worth the money. The
  three abandoned phases and the permanently deferred `TODO.finish_process.md`
  rows are part of the answer, not an embarrassment to be smoothed over; the
  reasoning is in
  [`docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`](../docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md).
- **The which-is-which split above appears in the document**, with the reason for
  each decline. The three that bound are named as *shipped in `content/`*, so a
  reader can tell a rule that now governs sessions from a lesson that is only
  written down.
- **Candidate 3 is stated in its surviving conditional form**, not as the root
  brief's headline. *Not widening* was falsified twice. Do not reopen a verdict to
  make a sentence read better.
- The document is placed where `docs/ARCHITECTURE.md` can own it — a new row in
  *Documentation ownership*, beside the four campaign records already there — and
  is linked from `candidate-lessons.md` if that document's own reasoning is what
  it builds on.
- The usual triangle if any `content/` file is touched: `cargo test` green,
  `docs/specs/corpus-rule-ownership.md`'s row totals moved in the same commit.
  **This leaf is not expected to touch `content/`** — the bindings have landed —
  so a change there is a decision to record, not a formality.

## Notes

**`.grove/` is still perishable, and this is the last leaf that needs it not to
be.** Nothing here reads it: all five source documents are under `docs/`, and the
two derived ones are pinned to a change id so they re-derive after teardown.
Whether the grove is finished is the driver's call, not this leaf's.

**`loop-record.md` and `review-yield.md` are derived and hand-edit-forbidden.**
A pointer *into* either belongs in `scripts/*.py`'s template. Pointers *to* them
are ordinary links and already exist in `docs/ARCHITECTURE.md`.

**The build boundary, in case it surprises you.** `methodology-changes-k91` edited
`content/`, which reaches no session until the binary is rebuilt and installed —
`docs/adr/skill-delivers-the-methodology.md` and `references/grove.md`'s
`build-boundary-is-the-binary`. So the installed `grove-llm` and the provisioned
skill still carry the pre-change corpus, and a locally built binary warns about
the hash mismatch. That is the rule working, not a fault, and it is not this
leaf's to resolve.

## Decisions (running log)

- **The write-up lands at `docs/driving-a-checkable-loop.md`**, a new row in
  `docs/ARCHITECTURE.md`'s *Documentation ownership* beside the four campaign
  records. Named for the root brief's own subject sentence — driving an LLM loop
  to produce checkable work — rather than for the campaign, because a stranger
  reaching the table is looking for the subject and not for this repository's
  history. Not a `TODO.<subject>.md` (that row is for work not yet started) and
  not under `docs/adr/` (it is an account, not a decision).

- **The declines are stated as five claims under four decline reasons**, matching
  the charter's four (5b; the isolating control; the module pair — environment
  as part of the control, and every module against every claim; candidate 2's
  model-level form) while keeping the two halves of the third apart, because
  `candidate-lessons.md` §3 and §4 are separate verdicts resting on different
  evidence. The Part 3 preamble says four declines and five claims explicitly so
  the arithmetic is not left for the reader to reconcile.
- **Candidate 3 appears only in its surviving conditional form** — *a narrowed
  world is evidence about the wide one only when the narrowing is a strict subset
  that still reaches the subject* — with the original headline quoted once, as
  the thing that was falsified, and both falsifications stated. No verdict was
  reopened.
- **Section citations are given as the target's own heading text**, glossed where
  the heading carries repository jargon (`.grove/`), rather than paraphrased into
  something readable that would not resolve.
- **No handle, obligation id, model symbol or session key appears in the
  document.** Checked by pattern over the whole file: `-k<digits>`, the four
  obligation prefixes, and the `inv_`/`mutant_`/`relax_`/`scenario_`/`wit_`
  families all return nothing. Quotes that carried an obligation number are
  bracketed at the substitution rather than silently reworded.
- **Every quoted phrase was matched against its source file** by
  markdown-stripped substring, not by recollection: eleven quotes, all resolved,
  one of which was found to be a substitution and is now marked as one.
- **`content/` is untouched**, as the charter expected, so the triangle does not
  apply. `docs/ARCHITECTURE.md` is included as a string by
  `tests/composition_guidance.rs`, so that test was run against the new row: 16
  passed.
- **`docs/ARCHITECTURE.md`'s existing campaign row was re-subjected** rather than
  left alongside a near-duplicate: `candidate-lessons.md` now owns *whether the
  six self-reported lessons survive the evidence*, and the new document owns
  *what the campaign taught about driving an LLM loop, and what it cost*. One
  subject, one owner.
