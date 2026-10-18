# document-checks-k6

## Goal

Extend the vendored checker so that it can gate whole prose documents, as
`skill-design-k5`'s spec defines. This is the half the document gate runs.

## Context

- The spec in `docs/specs/technical-communication.md` and the ADRs it cites
  define the contract. The vendored checker's behaviour is in
  `vendored-checker-k3`'s running log.
- The requirements name the gaps to close: separate limits for instructions and
  descriptions, a paragraph check, British spelling, whole-file checking, and a
  way to accept a justified departure. The spec decides the details.
- The checker ships with its own self-test and nothing else is tested here.

## Done when

- The checker takes a whole file, applies the limits for the text type the spec
  assigns, and reports and exits as the spec says. A justified departure is
  accepted and a bare one is not.
- Every modification is marked as ours, so the vendored base stays identifiable.
- The shipped self-test still passes under `scripts/check.sh`, and
  `bash scripts/check.sh` is green.
- Run on a few real documents from this repository (in the scratchpad, as an
  ad hoc check), it produces findings a human judges sensible. Record the
  false alarms you saw in the running log; the human logs failures in use.

## Notes

- Leave reply scoring to `reply-scoring-k7`.
