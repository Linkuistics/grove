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
