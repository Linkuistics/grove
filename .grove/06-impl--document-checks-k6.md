# document-checks-k6

## Goal

Extend the vendored checker so that it can gate whole prose documents, as the
spec `docs/specs/technical-communication.md` defines in *The checker's
contract*. This is the half the document gate runs.

## Context

- The spec's section *The checker's contract* defines the contract. It covers:
  - the text types and their limits: the `auto` test for an instruction, 20
    and 25 words, and six sentences in a paragraph
  - what counts as prose in Markdown, commit messages and plain text
  - British spelling without a dictionary
  - the marker grammar (`exempt`, `depart`, `off`, `pending`, `on`), with the
    `marker` finding
  - the rule identifiers and levels, the `--markers` mode, the output formats
    and the exit codes
- The vendored checker's behaviour and its surprises are in
  `vendored-checker-k3`'s running log. The spec settles four of the surprises.
  They are the fence's kind and length, link targets and URLs, front matter,
  and the exit code for a file that is not UTF-8.
- Which files the gate checks, and its message, belong to the hook core in
  `claude-code-always-on-k11`. The `--log-start` and `--log-reply` modes belong
  to `reply-scoring-k7`.
- The checker ships with its own self-test and nothing else is tested here. The
  self-test's assertions on level names and messages follow the renames.
- This is more than the leaf's first brief named. If it does not fit one
  session, decompose and finish the first part (text types, the paragraph check
  and the markers), as `references/decompose.md` directs.

## Done when

- The checker takes a whole file, applies the limits for the text type, honours
  the markers, and reports and exits as the spec says. A justified departure is
  accepted, and a bare one is a `marker` finding.
- `spelling` flags American forms, and nothing that British usage also accepts,
  on this repository's documents.
- Messages name this skill's rules and never cite the standard's numbering or
  name. `nominalization` is `nominalisation`, and the levels are `hard` and
  `advisory`.
- Every modification is marked as ours, so the vendored base stays identifiable.
- The shipped self-test still passes under `scripts/check.sh`, and
  `bash scripts/check.sh` is green.
- Run on a few real documents from this repository (in the scratchpad, as an
  ad hoc check), it produces findings a human judges sensible. Record the
  false alarms you saw in the running log. The human logs failures in use.
- Where the code meets a defect in the spec's contract, the spec is fixed in
  place, and the running log says why.

## Notes

- Leave reply scoring to `reply-scoring-k7`.
