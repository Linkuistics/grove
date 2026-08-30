**integrate-review-\*** (all AFK) — triage one review's findings and apply the
real ones. Shared discipline: verify each finding rather than performatively
agreeing, then classify it as *a contract stated unclearly* (fix the contract),
*a real issue* (fix the artifact), *a real trade-off* (accept it visibly), or
*noise raised for want of context*. What separates the five is **what the
session may change** — `integrate-review-impl` edits code freely;
`integrate-review-design` reworks the ADR set as `ADR-FORMAT.md` directs;
`integrate-review-planning` reshapes the tree; `integrate-review-prototype`
decides what the prototype *taught* and normally discards it;
`integrate-review-requirements` edits what was asked for, which it cannot always
do alone — the kind most likely to stop and ask.

**Read the findings from the review's commit, not from this task's body.** The
body names the review by handle; the findings live in the review's own commit,
and keeping them there is what makes rejecting one an ordinary outcome rather
than a session rejecting its own charter. Where a body does restate them, treat
the restatement as a pointer and grade against the artifacts. A finding you can
neither reproduce nor derive from the source is *noise raised for want of
context*, and classifying it that way is the triage working.

**Substantial redesign is not this session's work.** A finding that is real but
demands the artifact be rethought rather than repaired becomes a new producer
review chain beside the leaf being integrated; fixing it here would put an
unreviewed redesign into the tree under a leaf chartered to apply findings.
