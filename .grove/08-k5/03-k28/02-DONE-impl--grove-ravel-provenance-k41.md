# grove-ravel-provenance-k41

## Goal
Produce Grove-owned provider evidence and caller context through the real launch
and retirement path, without any Ravel store, record verb or private protocol.

## Context
k40 adds the readable static assets; k39 activated both public Ravel callers.
The old adapter/Creator convention names `HARNESS_DISPATCH_RUN_ID` and resolves
`host.run`. Read `docs/adr/a-review-carries-its-creator-reference.md`,
`plugins/grove/skills/grove/references/retire.md`, TASK-FORMAT.md and the actual
Ravel 0.1.0 generic SDK before settling their replacement. The public SDK's
Context has generic facts/assessments/sources and its host has measured reads.

## Done when
- Caller-owned records and context preserve the actual selected provider origin,
  not today's mapping for a producer kind. Settle and document the exact
  production/consumption contract for k42, including missing/invalid evidence,
  finishing retries and node-close attribution.
- Exercise production through actual caller/policy sessions with the real
  independently installed pair and fake agents. Use public arguments/context
  only; any wrapper/record belongs to Grove and remains independently testable.
- Retirement/Creator instructions and their conformance/tests explain the new
  evidence and the explicit legacy original-v22-driver case. Never read an old
  Ravel store or change this grove's inherited ending channel. Preserve actual
  personal routing choices, arguments, bounds and grants.
- Reconcile relevant durable decisions and every changed source-pinned book in
  the same increment. Add/use reusable focused checks with complete saved logs.
  Keep the old bundled package checks green until k6.

## Notes
Do not infer an origin from an executable name or model string. If the public
boundary cannot provide launch evidence without a caller wrapper, make that
boundary explicit rather than adding a Ravel feature. k42 owns adapter review
selection and complete end-to-end missing/invalid Creator checks.

## Decisions (running log)

- Implement an opt-in Grove-owned `withProvenance` public-SDK wrapper. The
  selected command executes through `/usr/bin/env`, which supplies a versioned
  JSON `GROVE_PROVIDER_EVIDENCE` from the actual Selected labels, policy version
  and request identity. Original argv/labels are preserved. Retirement copies
  this receipt inline as `**Creator:** recorded <JSON>`; no durable lookup or
  Ravel state is needed. The receipt is an owner-policy selection attestation,
  not authenticated backend identity. Original-v22 sessions alone retain run
  references; new consumers refuse those with an owner-declaration remedy.
- Execute within the existing mandated implementation session. Focused plan:
  first prove installed-worker production and lifecycle retry/cascade behavior
  with failing tests; add the wrapper/parser and inherited-evidence scrub; then
  reconcile retirement grammar/conformance/ADR/glossary and changed source books;
  run reusable focused tasks plus full relevant handoff checks, retire and seal.
  k42 owns review routing and the complete missing/invalid Creator matrix.

- The single boundary reviewer found env assignment parsing, wrapper budget
  overhead and an unclear immutable-attempt claim. Env parsing is actionable:
  use fixed Bash `exec --` and an installed-worker executable containing `=`.
  Budget inflation is an accepted opt-in cost, documented with the 8192-byte
  receipt limit and unchanged owner limits. Clarify inline records are pinned by
  the task commit, with fresh UUID-v4 attempt identity and same-user attestation;
  no separately authenticated record lookup is promised. Executable fixtures
  cover retries/remapping/cascade and canonical receipt validation, so these
  fixes require no second reviewer.

## Handoff

`assets/ravel-policy/provenance.ts` supplies opt-in `withProvenance`,
`ProviderEvidence`, `parseEvidence`, the environment name and 8192-byte bound.
The final wrapper uses fixed privileged-mode Bash `exec --`, preserving literal
original argv (including `=` in executable names and empty arguments), labels,
loader, cwd and owner settings/grants. Every successful selection mints fresh
UUID-v4 attempt evidence. Canonical receipt validation rejects unsupported,
missing/invalid/duplicate/extra fields and modified encodings. The immutable
review evidence is the inline record pinned by the task commit; no lookup store
or authenticated backend association is claimed.

The actual installed Ravel 0.1.0 caller/policy fixtures produce receipts, retire
producers, replace an earlier unfinished attempt, close nested nodes and deliver
measured generic `facts.creator` evidence to a review. Remapping the producer and
retrying the review preserve the finishing record/provider. Terminal and
prefix-similar reviews are unchanged. Static/sample route, modifier, original
argument and secondary jj grant assertions run through the wrapper too. Cargo,
shared fixture helpers, both callers and version probes scrub enclosing receipts;
dirty policy/entry controls prove the isolation checks can detect a leak.

The durable contract is `docs/grove-provider-evidence.md`; its schema, context
shape, missing/invalid/legacy refusals, finishing retries and node attribution
are k42's inputs. k42 owns the review adapter/selection and complete Creator
refusal/resume matrix. Wrap the final consumer policy when opting into recorded
evidence; unwrapped policies retain owner-declaration behavior. The wrapper's
Bash dependency, argv/message overhead and explicit original-executable grants
for confined callers are visible costs. No actual personal routing file, bounds,
arguments or grants were edited. Original installed-v22 sessions retain their
own legacy run references and inherited ending/admission/teardown contract.

Retirement/TASK-FORMAT, the diversity procedure, rule table and firing controls,
Creator ADR, relevant spec/glossary and both changed source-pinned books are
reconciled. The environment classification tests distinguish selection evidence
from authority/routing settings. Both broad-suite failures were missing
classification allowances; the controls' old run wording was corrected without
weakening the creator-step tests.

Verification is saved completely under `target/k41/`. `task ravel:provenance
RAVEL=/opt/homebrew/bin/ravel` passes typecheck, six installed-worker policy tests,
12 lifecycle/isolation tests, two caller isolation cases, 17 guidance cases and
conformance. The broader activation run passes Grove (152) and grove-loop (386),
including its doctest. Corrected `task test:grove-llm -- --no-fail-fast` passes
all 227 cases; keyed-launch passes 64 and its doctest check. Authority passes nine,
skill provisioning nine, and pre-k6 policy compatibility 47. Per-case evidence in
`per-test-evidence.json` resolves the earlier failed classifications against the
complete final passing rerun: 882 unique passing cases, no unresolved failures.
Formatting, workspace clippy and shellcheck pass. Final books reconstruct 14,333
loop and 1,400 overview lines with zero deferred lines. Shipped conformance passes;
its suite fires all 21 controls with zero failures.

Key logs: `provenance-final.log`, `activation-final.log`, `llm-final.log`,
`keyed-final.log`, `authority-final.log`, `skills-corrected.log`,
`compatibility-final.log`, `clippy-corrected.log`, `format-corrected.log`,
`shellcheck-final.log`, `book-loop-final.log`, `book-overview-final.log`,
`conformance-corrected.log`, `conformance-suite-corrected.log`. Red controls and
superseded failure logs remain available too. The pinned Bun 1.4.2 directory is
`/Users/antony/Development/Ravel/target/tools`.

`measure.py` digests tracked project sources/docs/manifests/configuration
(excluding live `.grove/` task notes), the development compiler/dependencies,
retained compiled worker, installed Ravel front/worker/SDK layout, tools, complete
original Grove 22.1.0 keg and personal Ravel policy. Every one of the 3,042 final
subjects matches individually in `before-final.json`/`after-final.json` and
`final-digest-comparison.json`. The conformance inputs also match their own
pre-run snapshot. No installation, release, private store access or new driver
occurred. k42 remains live, so this retirement closes no parent node.
