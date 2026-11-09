# session-probes-k281

## Goal

Add `session --harness claude|codex --route ravel|native` to the probe
program, on the measured launcher of probe-service-lifecycle-k280.

## Context

- The node brief states the observations that each route requires.
- probe-service-lifecycle-k280 delivers the launcher, its result line and
  the switch-side cleanup. Read its decisions log and its evidence section.
- `docs/evaluations/owner-skill-delivery/staging_trial.py` judges the same
  observations in the staging HOME: the mandatory order, unnamed and named
  skill reads, and the native listing. `scripts/harness_trial.py` holds the
  adapters of both harnesses.

## Done when

- A `ravel` session requires each observation of the node brief that the
  harness supports. A `native` session requires native discovery of the
  superseded skills and a native read of a Grove kind skill, other than
  `finish`, with one quoted line.
- Probe comparisons see each ordinary child of both native skill roots. They
  skip only the exact subtrees `synced` and `.system`.
- Each session mode passes once in the staging HOME, or carries a recorded
  limit. A control shows that each mode fails for a wrong layout.
- Each file that a probe session writes is recorded, with whether its MCP
  servers rewrite skill copies or agent definitions.
