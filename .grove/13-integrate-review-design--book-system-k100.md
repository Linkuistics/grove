# book-system-k100

**Integrates:** book-system-k25

## Goal

Triage the `book-system-k25` findings against the committed book-system design
and apply the real ones, so the specification, the book, and the validator agree
on one contract.

## Context

- Findings: the `## Findings` section of `book-system-k25`, read from that
  leaf's own commit. Fifteen findings, each naming a location, the rule it
  violates, and a candidate repair. Triage them; the review's severities are
  its judgment, not this leaf's charter.
- Artifact under repair: `docs/specs/ordinal-fs-tree-book.md`, produced by
  `book-system-k6`.
- Requirements: `plan-k1`, the root brief, and `SPEC-FORMAT.md`'s current-state
  rule.
- Downstream consumers that already shipped against this design and may have to
  move with it: `crates/book-validation` (and its tests) and
  `docs/ordinal-fs-tree/book/` — 92 fragment definitions, `source-index.md`,
  and eleven pages, all of them `DONE`. A repair that changes an identifier
  domain, a directive form, or a diagnostic code is a change to all three.
- Frozen corpus: the fifteen production files listed in
  `ordinal-fs-tree-book-k10`'s brief. Repairs may change how the design
  *describes* them; they may not change the files.

## Done when

- Every finding is settled explicitly as accepted, accepted-in-part, or
  rejected, with the reason recorded in this leaf's running log. A rejection
  states what makes the finding wrong or not worth its cost, not merely that it
  was expensive.
- Each accepted finding is repaired in the design, and every artifact the repair
  invalidates is brought back into agreement in the same session — the
  specification, `docs/ordinal-fs-tree/book/`, and `crates/book-validation`
  together, never the spec alone.
- Where a finding names an unmade decision rather than an error, the decision is
  made and recorded, and the losing alternative and its reason are kept.
- Any repair too large to carry here is externalised as a leaf with a named
  scope, rather than left as a note in the spec; the spec does not gain new
  transitional or staging prose.
- Fragment and Markdown validation and the crate's existing verification pass
  after the changes, at the invocation the repaired design names.
- The design reads as one current-state artifact: no rule contradicting another
  rule, no example contradicting a table, and no statement about the tool that
  the tool does not satisfy.

## Notes

The review ran after every consumer of this design had already landed, so the
usual asymmetry is inverted: the cheap repair is no longer cheap, and a finding
that would have been obvious to accept before authoring may now be right to
reject. That trade is this leaf's to make and to record. What it must not do is
leave a finding unsettled because settling it is inconvenient — an accepted
contract defect with a recorded reason is a result; a finding nobody answered is
not.

Findings 1, 2, 4, 7, 8 and 15 each rest on a place where `book-validation-k7`
had to invent policy because the design supplied none. For those, the working
implementation is evidence of what the contract has to say, not evidence that it
already says it: repairing them usually means writing down what the code already
does, and only sometimes means changing the code.
