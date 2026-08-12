# evidence-moves-k26

## Goal

Classify **`content/driving.md` lines 264–414** (8,528 bytes): `## When to retire
research into ADRs versus leave it in docs/research/`, `## Reworking ADRs and
briefs as understanding shifts`, `## Verifying framework decisions against the
source`, and `## Verifying a claim about the repo itself`.

This is batch 6 of 12. The theme is **evidence discipline** — where a claim's
proof lives and when it becomes binding.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve `content/driving.md` **L264–L414**, consuming the front of
  `pending-driving-evidence`.
- Mint exactly one residual, **`pending-driving-doubt`**, covering **L415–L754**.
- If `research-moves-k25` left any `defers=` on `pending-driving-evidence`,
  **redistribute** it onto the real units you create. The list on the marker you
  are replacing is the checklist; account for every member or say why one is
  dropped.

### The judgement this batch exists for

All four sections are self-rooting — each names its own condition in its heading —
but the *grain* is the question. Two of them are conspicuously large relative to
their neighbours:

- **`## Verifying a claim about the repo itself`** (3,764 bytes) is the biggest
  section in the region and is mostly a worked procedure for turning a grep into
  evidence. The condition is small and sharp (*a session is about to assert
  "every X is now Y" about its own codebase*); the body is long. That asymmetry is
  the design working — keep the `if`, defer the `then` — and this section is the
  cleanest example of it in the whole corpus. Classify it that way deliberately,
  not by inertia.
- **`## Reworking ADRs and briefs as understanding shifts`** (1,914 bytes) states
  a rule that is *also* stated in `SKILL.md` (the ADR-set reconciliation
  paragraph) and in `ADR-FORMAT.md`. Three statements of one rule. Decide which
  one is the condition and which are bodies, and record the call — `lifecycle-k31`
  will meet the `SKILL.md` half and needs to agree with you.

### Cross-file deferral

- `ADR-FORMAT.md` and the `linkuistics:decision-records` pointer appear at L285ff.
  `guides-k24` has carved `ADR-FORMAT.md`, so this edge is available — write it
  where the reference is genuinely trigger→body.
- `SKILL.md` references in this region point at constraints. Not `defers=`.

### Size note

At 8,528 bytes this is the smallest `driving.md` batch. That is deliberate: the
region's grain is fine and its four sections are independent, so the session cost
is in the judgements, not the bytes. If it finishes early, **do not absorb
`pending-driving-doubt`** — `doubt-moves-k27` carries the review chain, which is
the single largest section in `driving.md` and needs its own context.

## Done when

- `content/driving.md` L264–414 is subdivided into real units;
  `pending-driving-doubt` covers L415–754 and nothing else.
- Any `defers=` inherited from `pending-driving-evidence` is redistributed and
  accounted for.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The three-way overlap on ADR reworking is recorded in this leaf's body with the
  call you made, for `lifecycle-k31`.

## Notes

- `## Verifying a claim about the repo itself` contains fenced shell examples.
  Do not split mid-fence.
- Doubts to carry forward, by id.
