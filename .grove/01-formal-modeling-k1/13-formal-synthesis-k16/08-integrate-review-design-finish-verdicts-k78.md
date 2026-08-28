# finish-verdicts-k78

**Integrates:** finish-verdicts-k77

## Goal

Integrate the two findings from `finish-verdicts-k77`: Q1's `keep` verdict is
not established by the candidate that was run, and Q4's three cleanup-layer
`keep`s contradict the catalogue's still-binding classifier. Reconcile the
model, catalogue, README and ADR from one evidence set; do not preserve a
verdict by changing only its explanation.

## Context

### R1 — high: Q1's retained ownership check is vacuous, and “no control can” is a model coupling

The new retained-claim command does not exercise the claim it names.
`relax_EN_03` sets `ENV_BUDGET = 0` and both `ENV_PHASES` and `ENV_KINDS` empty
([`finish-controls.qnt`](../../../crates/grove-finish/models/finish-controls.qnt):824-865).
`envAdmitted` requires all three gates, so no foreign artifact can be installed
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):2220-2225).
The asserted `FN-32` invariant is only the absence of two mutation flags
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):3226-3234),
and this module carries no companion witness that reaches either unprovable
artifact. Its one reachability command reaches `SDisposeInPlace`, not the
ownership antecedent
([`finish-controls.qnt`](../../../crates/grove-finish/models/finish-controls.qnt):864-871).
The reported green therefore does not close the retained-set gap.

The reason given for declining the missing candidate is likewise about the
model, not the protocol. The state machine selects `SDisposeInPlace` exactly
when `ATOMIC_DISPOSAL` is true and `SQuarantineRename` otherwise
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):1706-1708);
the in-place action is itself defined as one atomic recursive deletion
([`finish.qnt`](../../../crates/grove-finish/models/finish.qnt):1862-1869).
That encoding cannot express `ATOMIC_DISPOSAL = false` together with a
no-quarantine, resumable disposal strategy, but a separate strategy/model dial
can. The README's inference that no control can exist because one `const` ties
the two together is therefore circular
([`README.md`](../../../crates/grove-finish/models/README.md):4031-4044).
Whether that candidate violates `FN-24` is the missing measurement, not a result
already produced by the current parameterisation.

This matters to the verdict rather than only the suite. The catalogue still says
Q1 is classified by the candidate retaining every shared-safety claim and
reaching the `FN-24` witnesses
([`semantic-contract.md`](../../../docs/specs/semantic-contract.md):296-307),
while the producer records that those witnesses were not run. Declaring that
classifier mis-typed invalidates it; it does not invert missing evidence into a
`keep`. Unless a replacement criterion and its evidence are landed coherently,
Q1 is `defer`, with the non-atomic no-quarantine control and the reachable
`FN-32` scenario as the commission.

### R2 — high: Q4's three `keep`s contradict the catalogue and consume the same missing control

The catalogue says a `none` row in both families is Q4 evidence for
`delete/replace`
([`semantic-contract.md`](../../../docs/specs/semantic-contract.md):344-355).
The Quint matrix records exactly that result for the quarantine, cleanup marker
and replace transition, but also says the three are one bundled control and
explicitly asks `finish-verdicts-k65` to commission artifact-specific removals
if the decision needs separation
([`README.md`](../../../crates/grove-finish/models/README.md):3962-4002).
The later `keep` text declines the commission solely because the model couples
the mechanism to `ATOMIC_DISPOSAL`; R1 shows that is not a protocol result.

Q3 does not close this gap. Its witnesses prove the replace transition is
reachable in the incumbent protocol, which confirms Q3 on its own evidence, but
they do not prove that the whole cleanup mechanism is irreplaceable. Until the
matrix rule is deliberately revised and re-measured, or the missing candidate
controls are run, the cleanup trio's Q4 disposition is `defer`, not `keep`.
The current catalogue is internally inconsistent where its classifier says
`delete/replace` and the paragraph immediately above it says all four are
`keep` ([`semantic-contract.md`](../../../docs/specs/semantic-contract.md):302-330).

### Claims that survived the review

- Q2 `keep` is independently supported by reached `Indeterminate` witnesses on
  Git, native jj and colocated jj in both families; it does not need the
  counterfactual-capability reinterpretation.
- Q3 `keep` is supported within the incumbent by the reached replacement source
  state and by the trace that produces the stale marker rather than positing it.
- `FN-13` remains shared safety. Total removal satisfying it vacuously means it
  cannot block that candidate; it does not make the class assignment false.
- `EN-03` and `EN-05` remain sound environment constraints for the targeted
  Unix/VCS implementation. The review found no available atomic recursive
  deletion or filesystem/VCS transaction construction.
- Root creation remains Grove-owned on depth: ignoring the quoted line counts,
  moving it would expose three new library concepts for one consumer while the
  Grove-specific format classification stays outside the seam. The ADR's
  second-consumer reopener is the right test.

## Done when

- The model can express a no-quarantine strategy independently of
  `ATOMIC_DISPOSAL`, and the intended non-atomic candidate is either run against
  the retained set or rejected by a runnable control rather than by the current
  branch shape.
- The Q1 candidate reaches an unprovable witness/marker case while asserting
  `FN-32`, and carries the `FN-24` witness coverage its classifier requires.
- Q1 and the three Q4 cleanup rows are reclassified from the resulting evidence:
  `keep` only if the replacement criterion positively establishes it, otherwise
  `defer` with the exact remaining commission. Q2 and Q3 remain `keep` unless
  that new candidate makes Q3 moot.
- `docs/specs/semantic-contract.md`, the finish model README, the finish-layer
  ADR, the formal-synthesis brief and implementation hand-off say one coherent
  thing. An ADR whose title says every layer is forced must not survive if Q1 or
  Q4 remains deferred.
- The finish-family model commands and coverage checks are rerun after the
  integration changes; the whole-repository run remains `handoff-audit-k66`'s.

## Notes

This integration task owns fixes and post-fix verification. The review ran no
model, test, build, lint or format command.
