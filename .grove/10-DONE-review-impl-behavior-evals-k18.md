# behavior-evals-k18

**Reviews:** `behavior-evals-k3`

## Goal

Adversarially read the lifecycle safety net `behavior-evals-k3` landed —
`tests/lifecycle_invariants.rs` and
`docs/adr/behavioural-coverage-asserts-delivery.md` — **before** the eight leaves
that fall into it start rewriting the corpus. A net that passes a rewrite which
dropped a rule is worse than no net, because it is read as evidence.

## Context

Sequenced ahead of `skill-router-k4` for the same reason `rule-ownership-k12`
was sequenced ahead of the leaves executing against the design: every leaf below
this one rewrites the corpus these checks measure, so a review that lands after
them reports on work already done. The producer spent **no** in-session reviewer;
the whole allowance went here.

The artifact is a deterministic check over a *composed loaded path* — the
guaranteed core, `SKILL.md`, `reference_file(kind)`, and the transitive closure of
the corpus files those name by path — asserting that twelve inventory rules across
the eight **B★** areas are stated somewhere on the path of every kind they bind.
It is inspection-only work: read the committed diff, the spec, the ADR and the
corpus. Run nothing.

## Done when

Each of these has a verdict, with the evidence that decided it:

1. **Are the matchers too loose?** Each claim is a conjunction of
   alternative-wording groups that must co-occur in one paragraph. The failure
   that matters is a claim satisfiable by prose that does *not* state the rule —
   the near-miss fixtures are the producer's control against exactly that, so the
   question is whether a *different* plausible rewrite slips through. Read
   `bigger-than-brief-decomposes` claim 0 and `review-budget` claim 2 first: both
   are two common words, and claim 2 (`bootstrap` + `mandate`) is the loosest in
   the table.
2. **Are they too tight?** The suite must stay green across eight rewrites that
   are *licensed to reword*. Check every claim against the canonical trigger
   sentences in the spec's *The trigger sentences* and against the owner-file
   statements the inventory prescribes — a group with no alternative matching the
   post-rewrite wording is a leaf blocked on a test rather than a rule protected.
   `externalize-by-default`, `retire-before-commit` and `one-focused-commit` cross
   a file boundary in the rewrite and are the exposed ones.
3. **Is the reachability model right?** Two judgements carry the per-kind claim:
   `SKILL.md`'s routing table is excised as a *selector* rather than read as ten
   edges, and a literal path occurrence anywhere in a file counts as an edge.
   The second is weaker than the spec's assertion 2, which also requires the
   naming sentence to be *about* the owned rule. Is the weaker test the right one
   here, or does it certify a state the design calls deleted-in-effect?
4. **Does the ADR earn its place, and is it right?** All three of the AND test, and
   the delivery-versus-conduct boundary stated where a reader meets it. Its sharpest
   claim is that a model-in-the-loop eval is the wrong *standing* instrument;
   attack that.
5. **Is anything in the eight areas uncovered?** The twelve rows are pinned by name
   in `the_table_covers_the_eight_required_areas_and_no_others`. Check them against
   the spec's **B★** markings rather than against the table — the table is the
   thing under review.

## Notes

- Findings only; the paired `integrate-review-impl` step owns every fix, and is
  cut only if this review finds something worth acting on.
- `content/` is out of scope for the producer and for this review: nothing in the
  corpus was edited, and the two signal files are byte-frozen for the workstream.
- The producer recorded a finding it deliberately did not act on:
  `tests/retire_guidance.rs` pins `SKILL.md` phrases that `skill-router-k4` will
  legitimately move. Judge whether leaving it to that leaf was right.
