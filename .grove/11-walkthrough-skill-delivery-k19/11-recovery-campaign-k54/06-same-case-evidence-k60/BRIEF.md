# same-case-evidence-k60 — brief

## Goal

Produce the requirements-authorized exact-instrument replication records for the
three historical same-case scenarios and apply the unchanged primary acceptance
rule.

## Done when

- Scope elicitation, source/fragments, and exposition/assurance each preserve
  five enabled and five contemporaneous-comparator assignments in the historical
  ABBA/BAAB order, or the historical instrument's terminal shortfall record. The
  new no-skill arm is never called the pre-skill baseline.
- All three generation leaves complete before any adjudication leaf publishes a
  case result. Each generation leaf preserves the historical schedule and
  runtime record.
- Access, replacement, and scoring follow the historical rubric exactly.
  Stronger exposure audits, dual scoring, arm guesses, absolute gates, or fail-
  closed rules absent from that rubric are out of scope.
- Each case publishes historical-rubric row counts and material/regression
  classifications without changing any row or denominator.
- The same-case verdict applies the exact historical `R`, `G`, `2/5`, `10/15`,
  incomplete-sample, truncation, and exclusion semantics. It also preserves the
  original failed enabled campaign beside any authorized replication result.

## Decomposition

Each case has one generation leaf and one adjudication leaf so model execution,
access classification, and judgment do not share one runaway session. The three
generation leaves run first; only then do the three adjudication leaves run. The
final leaf combines only already-frozen scores. This prevents a published early
verdict from influencing later replacement handling even though the joint freeze
already fixes prompts, criteria, apparatus, and treatment bytes.

## Pointers

- Joint campaign manifest: produced by `campaign-freeze-k59`
- Historical case records: `docs/evaluations/writing-code-walkthroughs/`

## Notes

No leaf here edits the skill, the instrument, or a historical record. If the
requirements decision does not authorize execution, the node records that
disposition and externalizes the next work rather than manufacturing evidence.
