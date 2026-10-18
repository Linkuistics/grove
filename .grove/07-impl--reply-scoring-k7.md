# reply-scoring-k7

## Goal

Give the checker a scoring mode that scores one reply and appends one record to
the reply log, as `skill-design-k5`'s spec defines. This is the half the reply
log runs. A reply is scored and never held back or sent back for a rewrite.

## Context

- The spec defines the log's location and fields, the exemption marker, and
  what text of a reply counts as prose. The requirements fix the measure:
  violations per 100 words, with per-rule counts. Code, commands, tool output,
  quoted material and identifiers are outside the measure.
- The checker's passive-voice check matches word shapes, not grammar, so its
  count is a noisy signal. The spec says how the log treats it.
- `document-checks-k6` extended the same file. Build on its limits and its
  profile for reply text.

## Done when

- Given a reply, the scoring mode appends exactly one well-formed record to the
  log and exits zero whatever the score is.
- Concurrent writers cannot corrupt the log (the spec says how the single
  append is made safe).
- A reply the human exempted is logged with the exemption mark, so it does not
  skew the measurement.
- The shipped self-test still passes and `bash scripts/check.sh` is green.
- Run over a few real replies from this session's transcript, in the
  scratchpad, it produces records a human finds sensible.

## Notes

- Which hook hands a reply to this mode, and how the switch is checked, belong
  to the always-on leaves.
