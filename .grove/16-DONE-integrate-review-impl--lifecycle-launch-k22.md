# lifecycle-launch-k22

**Integrates:** lifecycle-launch-k21

## Goal

Triage and integrate the two lifecycle cutover findings before the
configuration deletion leaves build on it.

## Context

- `lifecycle-launch-k21` records F1 and F2, anchored to `14c3328f`, with
  source and test evidence. `7f0d81e4` is the companion book rewrite.
- F1: a failed-session diagnostic claims the leaf remains live even when the
  harness retired or decomposed it before failing without signalling. Keep
  truthful rerun guidance and cover retirement before failure through the real
  driver, front and worker.
- F2: `selected_root_launches_only_while_its_directory_is_current` still
  launches `/bin/sh` directly through the loop's session launcher. Preserve
  its root-removal/replacement and epoch assertions using real dispatch and
  a deterministic harness selected by a policy in a temporary HOME.
- The review found no reversal of text-before-lock. Its missing automated
  test is optional, not a required finding.
- Follow the root brief's book/ADR/spec maintenance and full-check convention
  for source and test changes.

## Done when

- Every finding is classified and the actionable ones are integrated.
- Appropriate regression checks and `bash scripts/check.sh` pass on the final
  changes; books reproducing changed source are current.
- This leaf's decisions make any rejected finding explicit.

## Decisions (running log)

**F1 is a real issue, fixed by scoping the claim.** Reproduced through the real
driver, front and worker: a harness that ran `grove-llm leaf-retire` and exited
23 was told its leaf was still live. The failure line now says the leaf is live
only of a refused launch, in the clause that points at dispatch's diagnostic,
and keeps the rerun guidance for either case. Reading the tree after a failed
session to report the leaf's real state was the alternative. It was not built:
the loop holds no state and a rerun derives its position from the tree, so the
line needs to promise nothing. The refused-kind case keeps its own assertion
that the leaf stays live.

**F2 is a real issue, fixed where the test is.** The stale-root cases rename
`.grove` between `picked` and `launch_session`, a window only a test inside the
crate can reach, so the case stays a unit test. Its launch is now
`dispatch_run`'s argv through the real front and worker, with a policy in a
temporary HOME that runs a one-line harness. `env HOME=…` carries that HOME to
the front, because `testing/support.rs` forbids changing the test process's own
environment. The shared helpers are declared at file level in
`loop_driver.rs`: a `#[path]` inside the inline test module resolves against a
directory that does not exist.

**The text-before-lock test stays uncut.** The review found no violation and
called the test optional. Nothing here changes that order.

**No in-session reviewer was spent.** Each fix is held by a test that was seen
to fail first or that exercises the launch it claims.

**The `grove-loop` book was rebased for `loop_driver.rs`.** The fragment bodies
were regenerated from the source by script: only the endings block and the
tests' opening block changed, and every other body came back identical. The
ranges, the manifest, the index and the roll-up totals followed, and the two
paragraphs that described the old line and the old launch were rewritten by
hand. The chapter's measured figures were not re-measured, as before.
