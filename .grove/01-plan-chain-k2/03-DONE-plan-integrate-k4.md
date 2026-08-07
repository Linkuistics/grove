# plan-integrate-k4

**Kind:** integrate-review-requirements
**Integrates:** plan-review-k3

## Goal

Apply the verified findings from `plan-review-k3` while preserving the reviewed artifact's contract.

## Context

- Preserve explicit human decisions. A finding that would change them requires
  human confirmation; unclear contracts may be sharpened directly.

## Done when

- Every review finding is verified and classified, with real issues reflected
  in `CONTEXT.md`, the root brief, or the follow-on tree as appropriate.
- The integrated requirements are coherent enough for the design chain to use
  without reconstructing this interview.

## Integration

- `plan-review-k3 R1`, `R4`–`R6`, `R8`–`R9`, and `R18` were verified as
  requirements contradictions or omissions and are resolved in `CONTEXT.md`
  and the root brief.
- `plan-review-k3 R2`–`R3` required a product choice. The human delegated it;
  integration preserves both herdr behaviors through explicit config-owned
  policy: a visible `${herdr_settings}` splice and configured `HERDR_AGENT`,
  with no hidden harness inference.
- `plan-review-k3 R7` is a necessary consequence of the confirmed single driver
  pick. The requirement now uses prompt-visible Bootstrap-and-mandate as the
  ownership predicate; `config-driven-sessions-k6` must reconcile the ADR, spec,
  and doubt skill rather than silently redefining them.
- `plan-review-k3 R10`–`R14` are verified lifecycle and recovery gaps. Their
  required outcomes are pinned in `CONTEXT.md`; transaction and diagnostic
  mechanics remain design work for `config-driven-sessions-k6`.
- `plan-review-k3 R15`–`R16` correctly identified accidental design closure.
  Confirmed product choices are now separated from open design work, and
  `${prompt}` is required once anywhere rather than last.
- `plan-review-k3 R17` is verified. The root brief and
  `implementation-slices-k10` now enumerate the orphaned CLI, environment,
  receipt, methodology, plugin, documentation, and test surfaces.
- The leaf's one narrow adversarial reviewer found seven integration ambiguities;
  all were actionable. The final contract resolves mandates by stable handle,
  validates terminal kinds, scopes finish commits, leaves migration mechanisms
  to design, reserves argv word zero for a literal executable, narrows config
  errors to task-tree non-mutation, and rejects terminal verbs on `finish`.
