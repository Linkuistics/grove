# baseline-scope-elicitation-k42


## Goal

Run and score the five frozen scope-intake repetitions in fresh contexts.

## Context

- Frozen contract: `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- Preserve outcomes under `docs/evaluations/writing-code-walkthroughs/baseline/scope-elicitation/`.

## Done when

- Exactly five valid no-skill repetitions use the frozen prompt and controls.
- Each run records its access manifest, raw final answer, criterion scores, and
  concise adjudication notes.
- Recurring omissions and rationalizations are summarized without changing the
  rubric or proposing skill wording.

## Notes

Infrastructure failures do not count as repetitions; preserve and label them
separately if one occurs.

## Decisions (running log)

The five planned first attempts used the frozen command and reached
`thread.started`, but the managed execution sandbox denied DNS resolution for
both WebSocket and HTTPS transports. They emitted no final assistant message and
are therefore invalid infrastructure attempts, not repetitions. The sandbox
also denied signalling the exact child PIDs after the failure was established,
so their frozen 20-minute wrappers were left to record the definitive exit state.
No replacement attempt was launched through the same demonstrably unavailable
network route in this session.

All five wrappers subsequently exited `124` at exactly 1,200 seconds. Each raw
stream contains 29 events, no tool call, and no final assistant message; each
run-directory post-manifest is empty. The five attempt records and their raw,
stderr, access-manifest, and post-state evidence are preserved under
`docs/evaluations/writing-code-walkthroughs/baseline/scope-elicitation/`.
The record also discloses two attempted interrupt control bytes that the sandbox
delivered to CLI stdin rather than as signals; they created no model-visible raw
event, and a replacement runner must close stdin explicitly.

After full network access was restored, replacement attempt 2 completed all five
planned repetitions normally with closed stdin, one final message each, no tool
or error events, and no run-directory writes. A randomized path-stripped bundle
was scored by one fresh blind context and the complete case was re-scored by a
second. The two scorers disagreed on 2 of 105 decisions: `A10` for repetition 5
and `A19` for repetition 3. Both resolve to `0` because the former never states
an authority rule conditional on corpus change and the latter never asks for a
mechanical proof requirement separately from judgment.

The resolved totals are 6, 6, 6, 5, and 5. `A02`, `A03`, `A14`, and `A15` are
present in all five; `A01` and `A04` are mixed; `A05`–`A13` and `A16`–`A21` are
repeated gaps. The durable summary names recurring omissions and the broad
questions used in their place without proposing skill wording.

The leaf-wide reviewer found one important completeness defect: the case README
held the exact prompt, but each planned-repetition Markdown record must hold it
under the frozen campaign-record contract. The byte-identical prompt is now
embedded in all five repetition records. The fix is mechanical and is covered by
extracting each record's `Exact prompt` block and comparing its digest with the
frozen rubric prompt; no second reviewer is commissioned.
