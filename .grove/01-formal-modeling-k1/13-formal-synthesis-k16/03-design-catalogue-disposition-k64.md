# catalogue-disposition-k64


## Goal

Dispose every catalogue finding the formal phase recorded rather than fixed, and
land each disposition so that both model families still answer the manifest the
catalogue defines.

## Context

**`docs/specs/semantic-contract.md` is the runner's manifest, not a document
about the models.** `models/run.sh` reads the obligation list *out of* the
catalogue (its header, obligation 3, and the catalogue's own *Model paths and
the runner*). So a disposition that adds, removes or re-scopes an obligation
opens an empty `(family, obligation)` cell that **both** families must fill
before any coverage-asserting run is green. Classify every disposition as
**manifest-neutral** (prose, wording, a citation) or **manifest-changing**
before deciding it, because the second carries a model-work cascade and the
first does not.

**Enumerate the inherited set; do not work from the list below.** The list below
is classes and sharp items, written by `experiment-synthesis-k62` to size the
work — it is not the enumeration, and a list written in one session goes stale
against the tree. The instrument, with the controls that make its output
evidence rather than a clean grep:

```sh
grep -rn "formal-synthesis-k16" . \
  --exclude-dir=.jj --exclude-dir=target --exclude-dir=.grove \
  --exclude-dir=_apalache-out --exclude-dir=.review-tmp
```

97 sites across 11 files at the revision this leaf was cut. **Controls:** the
same command for a live sibling handle (`cross-model-replay-k15`) must find its
own sites — it found 14 — and for an invented handle (`formal-synthesis-k99`)
must find none. Clean-here alone proves nothing; clean-here plus dirty-there
cannot be produced by a broken instrument. Then classify every site: a
disposition, a pointer to one, or prose that merely names the leaf.

**The classes, and the sharp items in each.**

- **Closed-set additions — every one manifest-changing.** `RRolledBack`
  (a clean rollback has no member), `RConfigInvalid` (no refusal reason names an
  invalid configuration), `RGenContended` (the contention timeout's visible
  stop), `ONotEntered`, and `Stopped`. Each was *declared as an addition* by a
  model rather than smuggled into the catalogue's set, which is why they are
  here.
- **Claims false as literally worded.** `FN-25.b`; `SY-13` over the
  `Legacy`/`Foreign`/`Malformed` sinks; `TT-17`, contradicted by the
  catalogue's own `PartialScaffold`; `TT-15.a`, contradicted by `TT-24.b` and
  `TT-18` under one tree; `TT-20`'s prohibition on `Legacy`, which entry 048
  showed is **false of shipped behaviour** in the window after the charter and
  before the leaf.
- **Underdetermined claims the two columns resolved in OPPOSITE directions.**
  `FN-13`'s refuse-or-block, and whether `reap` is gated on the root's
  classification. **These are the two sharpest items on the list**, because an
  opposite resolution by two independent readers is stronger evidence that the
  text is underdetermined than either reader's own account. `reap` additionally
  has the product answer: `src/loop_driver.rs` reads no root classification on
  the reap path, so the catalogue gap is real and no product defect stands
  behind it.
- **Scoping.** Whether `SY-04.b` owes `SY-03`'s *a preflight is never a licence*
  prohibition for the **configuration** as well as for the layout — Quint's
  retained counterexample has `outcomeOn` gating on the driver's recorded
  verdict while the layout gate three lines below reads live. Whether
  `SY-04.b`/`SY-14.b` over-apply to `release-lease`, which touches no tree.
  `EN-11`'s controls column mis-attributing `TT-24.b`, whose dependency is
  `EN-13` — a row that has now been wrong twice, which is worth more than the
  one-word fix.
- **State-table completeness.** The shipped *ambiguous partial root scaffold*
  refusal is a fail-closed outcome the state table has no member for, and entry
  048 judges it **a better answer than either model gives**. Also the mid-flight
  block: an ordinary mutation that has already applied an effect and whose next
  create is no longer licensed is none of the three contexts *Outcomes* fixes.
- **Prose, manifest-neutral.** The catalogue's *`models/run.sh` is the one
  repository runner* section states **three** obligations where its own Q4
  paragraph makes **four**; the runner numbers four.
- **Model findings, not catalogue findings — route, do not dispose.** The
  `SY-10.b` / `SY-11.b` collision over `WGen`, where one obligation's apparatus
  constructs the state another forbids and no predicate over `waits` separates
  the two senses. `TT-24.c`'s uncontrolled Quint transcription belongs to
  `obligation-placement-k63`.

**What is already settled and must not be re-opened.** `TT-24.c`/`TT-24.d`
placement and the five other instances of that shape are
`obligation-placement-k63`'s, and this leaf applies the rule that leaf lands.
The ordinal root-lifecycle verdict is `finish-verdicts-k65`'s.

## Done when

- Every enumerated site is classified, and every disposition is **decide**,
  **route to a named sibling**, or **prose-only**. A site that turns out to name
  no decision is recorded as such rather than dropped.
- Each decided item lands in `docs/specs/semantic-contract.md`, and each is
  marked manifest-neutral or manifest-changing at the moment it is decided.
- For every manifest-changing disposition, **both** families answer the new or
  changed obligation with a property command plus its required witnesses, or
  with that family's own declared gap, and each affected scope's
  `models/run.sh --scope <scope> --family <family>` is green with coverage
  asserted. A run line is recorded for every scope touched.
- The model READMEs that recorded a finding *rather than fixing it* are updated
  in place to say how it was disposed, so a later reader meeting the
  counterexample finds the decision beside it.
- No catalogue finding is left saying `formal-synthesis-k16` owns it.

## Notes

**Expect this leaf to decompose, most likely by scope.** Three scopes, two
families, and a 1,537-line catalogue whose every manifest-changing edit costs
model work in both columns. Decompose only when the work in hand proves bigger
than one session, and cut the split `obligation-placement-k63`'s rule implies
rather than the split that looks tidy today.

**A disposition is a decision about the contract, so the ADR test applies.**
`content/ADR-FORMAT.md`'s when-to-write test decides whether an item earns a
record in `docs/adr/`; several of these — the closed-set additions especially —
are contract changes a later reader will want the cost of, not just the outcome.

`docs/formalism-findings.md` is a log and is **not** revised by this leaf beyond
recording an outcome in place where an entry named one as owed.
