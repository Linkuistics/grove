# creator-reference-k59

**Reviews:** creator-reference-k38

## Goal

An adversarial, inspection-only read of the whole `creator-reference-k38` node:
the methodology amendment that makes a finishing session name its run on a
producer's reviews (`creator-methodology-k39`), and the Grove lifecycle cases
and owner documentation built on it (`creator-lifecycle-k40`). Produce
findings, not fixes.

## Context

The amendment changes rules every future session follows. Conformance rows
prove that wording is present on a loaded path. They do not prove that the
steps are coherent where a session meets them, at retirement and at node
close. The lifecycle cases prove what a review's launch selects from once a
fake session has followed the step. They do not prove a real session can
follow it from the text alone.

Read against the spec's `#identity-and-creator`, `#grove-integration` and the
Grove launch-boundary row of `#test-seams`, and against
`docs/adr/a-review-carries-its-creator-reference.md`. Each leaf's running log
records what it chose and the mutations it saw fail. A mutation shows only that
a test notices the break it was written for.

No session in this grove has yet run under the amended skills. Claude Code
receives them through the marketplace after the release push, and Codex
through the binary. So read the files. No session's behavior is evidence.

## What to doubt

- **Whether the step is coherent in the order a session meets it.**
  `references/retire.md` puts *Naming your run on what you finish* between
  retirement and the commit. A producer cuts its review as its last act. So
  which comes first, the cut or the naming, and does the text say? A fake
  producer that named its run before it wrote the cut review's `**Reviews:**`
  line left that review with no creator, and the review refused (k40's log).
  Walk a real session through the text for three shapes: a leaf that cuts its
  own review; a node close, where step 4 carries the step and the node's
  review is cut by the closing leaf; and a multi-level close.
- **Whether "search `.grove/` for the handle" finds what it must, and only
  that.** A review cut earlier can sit anywhere. A terminal review must not be
  rewritten. A handle quoted in prose or in a fenced example is not a
  `**Reviews:**` line. The fake sessions match the whole line, and skip
  filenames with a `DONE` or `ABANDONED` infix. Does the text's "every live
  review leaf whose `**Reviews:**` line already names its handle" lead a
  session to that same set?
- **Whether the fake sessions follow the convention or the test's
  convenience.** `SESSION` in `crates/grove/tests/loop_driver.rs` is the
  procedure. Compare it clause by clause with the retirement procedure. The
  dispatched attempt in the direct-finish case names its run and dies before
  it retires. Is that an attempt the methodology can produce, or one invented
  so the removal has something to remove?
- **Whether the lifecycle cases would notice a wrong creator.** Each asserts
  the reviewer launched and the creator the run recorded. Look for a wrong
  outcome every case still passes: a review that selects from the right
  provider for the wrong reason, a second live review the fake never reaches,
  a review nested inside a node, an abandoned producer.
- **A hand-written run line is kept as a second remedy.** The spec gives the
  declaration for a producer finished with no run. It is silent on a
  dispatched session that omitted its step. The documents and the adapter's
  `creator_line_missing` remedy lead with the declaration, and keep
  `**Creator:** run <run-id>`, written by the owner, for that case. Is that
  consistent with the retirement procedure's "from your own environment and
  from nowhere else", and with what *execution-recorded* is allowed to mean?
- **Four documents carry an account of the line.**
  `docs/CONFIGURATION.md#a-reviews-creator-line` is the whole one. The usage
  guide's review-composition section, configure-grove's *The creator line* and
  the dispatch README's Grove review section are shorter forms. Do they agree
  with one another, with the spec, and with the retirement procedure? Does any
  restate the step closely enough to drift from its one owner while staying
  under the conformance row's pinned phrase?
- **The scrub in `driver_command`.** It removes `HARNESS_DISPATCH_RUN_ID` and
  `HARNESS_DISPATCH_STATE_DIR` from every driver this suite launches. Does any
  other fixture that starts a session, in this crate or another, still inherit
  an ambient run? Does a real direct harness launched inside a dispatched
  session inherit one, and does the methodology's text guard a session against
  writing it?
- **The rescoped statements.** k39 scoped every statement that nothing reads
  the relationship lines to Grove's own code. Its sweep's classes are in its
  log. Look for one it classified wrongly, and for one written since.

## Done when

- Every doubt above has been read against the methodology text, the code and
  the tests, and each finding names its file, its line and a failure scenario,
  or the doubt is recorded as examined and found sound.
- A review with findings worth acting on cuts its `integrate-review-impl` leaf
  where `pick` reaches it next.
