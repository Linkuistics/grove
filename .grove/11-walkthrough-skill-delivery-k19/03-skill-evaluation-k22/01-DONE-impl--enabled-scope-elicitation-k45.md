# enabled-scope-elicitation-k45


## Goal

Run and score the five frozen scope-intake repetitions in fresh contexts with
`writing-code-walkthroughs` enabled.

## Context

- Frozen contract: `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- Baseline comparison: `docs/evaluations/writing-code-walkthroughs/baseline/scope-elicitation/`.
- Preserve outcomes under `docs/evaluations/writing-code-walkthroughs/enabled/scope-elicitation/`.

## Done when

- Exactly five valid skill-enabled repetitions and five contemporaneous
  no-skill controls use the frozen prompt, sample size, contamination controls,
  and interleaved ABBA/BAAB schedule.
- Each run records its arm, skill revision where applicable, access manifest,
  raw interaction, atomic
  criterion scores, and concise adjudication notes.
- Recurring omissions and variance are summarized without changing the rubric
  or skill wording.

## Notes

Infrastructure failures do not count as repetitions; preserve and label them
separately. This slice measures the pre-refinement skill and must not edit it.

## Decisions (running log)

The enabled campaign is decomposed from `skill-evaluation-k22` before running:
the three same-case groups share one pre-refinement skill revision, followed by
the separately frozen transfer probe, refinement/regression, and the final
comparison and deployment report. This prevents one scenario's findings from
changing the instructions measured by later initial scenarios.

The frozen local toolchain matches the baseline exactly: Codex CLI `0.150.1`,
executable digest `a14f9a…ca6b`, model alias `gpt-5.4`, high reasoning effort,
and GNU timeout `9.11`. The first contemporaneous control nevertheless failed
DNS resolution for both transports and emitted no final answer. Its complete
available evidence is preserved under
`docs/evaluations/writing-code-walkthroughs/enabled/scope-elicitation/`; no
replacement was launched through the same demonstrably unavailable route.
This is an invalid infrastructure attempt, not behavioral evidence, and the
leaf remains live pending a network-capable execution environment.

Full network access restored the frozen route. The resumed campaign reached a
terminal shortfall rather than the planned complete sample. Seven enabled
attempts explicitly discovered the skill, read `SKILL.md` with a shell tool,
and were invalid under Case A's no-tool boundary; enabled repetition 5 exhausted
all three attempts. Four enabled repetitions completed without a tool event,
but their discovery status is indeterminate because the raw stream has no
predeclared discovery event. There were zero observable skill-file reads or
uses among those four valid samples.

An adversarial protocol check found that control repetition 1 attempt 2's
missing terminal LF was not an authorized replacement reason under the frozen
exhaustive list. The record therefore keeps attempt 2 as the selected behavioral
sample, marks the campaign non-comparable for its byte-level prompt breach, and
preserves attempt 3 only as a post-breach diagnostic. A corrected fresh blind
adjudication scores the nine selected behavioral answers descriptively; no
five-versus-five endpoint or skill-wording effect is claimed. The rubric's
third-invalid-attempt rule requires stopping and reporting the shortfall, so the
leaf terminates with the full failure evidence rather than changing the rubric,
prompt, command, or skill.
