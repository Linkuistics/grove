# ravel-skill-bounds-k57

## Goal

Enforce every skill bound, the resolution deadline and cancellation during
resolution, at both public seams. Each failure refuses with its documented code
and status, and no failure leaves staging or a partial descriptor.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Bounds", "Failures"
  and the `resolutionTimeoutMs` rules under "Settings".
- Ravel `docs/specs/result-job-completion.md`: outcome and diagnostic order
  for a consumed signal.

## Done when

- Each count and byte bound of the table refuses at its limit and passes below
  it, as `skill_limit_exceeded`. The refusal reports the actual value, the
  bound and the subject. Controls that remove each bound show that the check
  runs. The bounds are sources, search directories, mandatory files, one
  mandatory file and one resource. They are also source bytes, total bytes,
  entries in one source, the descriptor and the preamble.
- `resolutionTimeoutMs` accepts 1,000 to 3,600,000 and defaults to 120,000. A
  malformed value refuses as `settings_invalid`. A controlled delay times out as
  `skill_resolution_timeout`, exit 124. The refusal reports the effective
  deadline and the active source and operation. A larger valid deadline
  succeeds with every check.
- INT, TERM or HUP during resolution refuses as `skill_resolution_cancelled`.
  Ravel removes its staging, reaps its children and raises the same signal
  again. A consumed signal wins over a resolution refusal, and the report keeps
  that refusal as the original failure.
- Public fixtures cover the local parts of "Bounds and frames", "Slow valid
  source" and "Cancellation". For a local source, the controlled delay and TERM
  act during capture. The transport case belongs to
  `ravel-pinned-delivery-k25`.
- No bound truncates caller text, omits a directory or mandatory file, or
  disables delivery.
- The book reference and `CHANGELOG.md` state the bounds and codes. The
  affected Ravel checks pass. The bookmark advances by jj, and nobody pushes it.

## Notes

The prepared-prompt bound and the frame bound already exist from
`ravel-skill-launch-k55` and `ravel-schema-four-k53`. This leaf adds no second
check of them.
