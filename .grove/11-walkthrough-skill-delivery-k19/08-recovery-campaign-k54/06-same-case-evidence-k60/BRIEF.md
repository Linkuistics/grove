# same-case-evidence-k60 — brief

## Goal

Produce complete paired records for the three generic same-case behavior
families and apply the jointly frozen primary acceptance rule.

## Done when

- Scope elicitation, source/fragments, and exposition/assurance each preserve
  five assigned control/enabled pairs or the frozen terminal shortfall record.
- All three generation leaves complete before any adjudication leaf publishes a
  case result. Within each generation leaf, pair arms run back-to-back and carry
  per-arm timestamps under the pair id.
- A deterministic access/outcome audit precedes behavioral scoring for every
  case, including replay of replacement legality over the complete attempt
  history, while the blind scorers receive only the normalized frozen surface.
- Every scored bundle receives two independent scores and forced arm guesses;
  disagreement resolution, guess accuracy, per-row counts, pair-aware results,
  absolute attainment, material change, and every treatment-delivery observation
  are published for each case without changing the freeze.
- The same-case verdict applies exact target, per-family, absolute, regression,
  empty-set, incomplete, protocol-failed, and unblindable semantics across all
  three cases. Missing data never contribute to attainment.

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

No leaf here edits the skill, the instrument, or the historical report. A poor,
protocol-failed, or unavailable result is still a complete leaf outcome when it
is recorded under the frozen rule.
