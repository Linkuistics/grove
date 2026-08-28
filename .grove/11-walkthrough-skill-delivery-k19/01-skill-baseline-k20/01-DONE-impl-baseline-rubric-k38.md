# baseline-rubric-k38


## Goal

Freeze the prompts, observable criteria, scoring rules, sample sizes, and
contamination controls for the no-skill walkthrough baseline before any model
run occurs.

## Context

- Method evidence: `docs/research/walkthrough-method.md`, especially
  `Observable behaviours for baseline and skill-enabled scenarios`.
- Proven artifact contract: `docs/specs/ordinal-fs-tree-book.md` and the reviewed
  book under `docs/ordinal-fs-tree/book`.
- Durable rubric: `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.

## Done when

- Every baseline and enabled-evaluation prompt is frozen verbatim.
- Each case names observable criteria, a deterministic score scale, five valid
  repetitions, and the exact evidence an adjudicator may use.
- Fresh-context, access-manifest, model, and contamination controls are fixed.
- One case names an external repository source by revision, path, and digest.
- The external source bytes are committed as a portable frozen fixture.
- No evaluated model has been run.

## Notes

Do not create the skill or execute a scenario in this leaf. Retiring and
committing this leaf is the causal boundary that later baseline leaves rely on.

## Decisions (running log)

The adversarial pass found that directory inventories alone cannot prove host
filesystem isolation. Runs therefore preserve raw events and invalidate any
undeclared tool access; the report states the broader OS-readable surface as a
limitation instead of claiming it was absent.

The same cases intentionally shape and evaluate the skill because the parent
contract requires rules to arise from observed gaps. Their comparison is
explicitly limited to gap closure. A separately commissioned paired transfer
probe tests behavior on a prompt and codebase selected only after skill bytes are
frozen, without permitting post-probe wording changes.

Scoring is binary and atomic, with sample classifications and retry boundaries
fixed here. Runtime alias drift is controlled by interleaved contemporaneous
no-skill repetitions during enabled evaluation. The external source is vendored
at its verified digest. Promised behavior is scored only where the case requests
a protocol or plan; concrete examples, ledgers, and checks have their own rows.
