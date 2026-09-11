# node-grammar-k13

**Integrates:** node-grammar-k12

## Goal

Triage the findings of the `review-planning` leaf `node-grammar-k12` against
the implementation plan at `node-grammar-k3`, apply the ones that are real, and
leave the root brief and the six `impl` bodies current and executable for
`distinguished-names-k6` to consume.

## Context

- The findings are in the review's own file, found by its handle; read them
  from there rather than from this body. They are anchored to commit
  `tpoponvo` (`091abec6`) and to `path:line` coordinates in that tree. The
  review's own insert of this leaf shifted the implementation leaves from
  positions 07–12 to 08–13 afterwards; handles are unchanged.
- The review questions the plan was read against are in the same file. The
  requirements are `plan-k1` and the root brief; the design is
  `node-grammar-k2` as integrated at `node-grammar-k5`. The interview is
  complete; do not re-interview.
- The artifact is task-tree prose: the root brief and the leaf bodies. Nothing
  under `crates/`, `docs/` or `plugins/` is part of this change. Where a
  finding names source, it names it to show what a leaf body has to account
  for, not to be edited here.

## Done when

- Every finding is triaged as applied, rejected with the reason, or a visible
  accepted trade-off, in this file's running log.
- The applied changes leave each `impl` body naming its surface, its seams and
  the book it owes, the cutover body's handoff executable by the sessions and
  the human it names, and the migration body's inventory matching the machine.
- `distinguished-names-k6` still follows this leaf; positions and keys of the
  implementation leaves are otherwise untouched.

## Notes

- Substantial replanning is not this leaf's: externalise it as a new producer
  review chain beside this leaf rather than absorbing it.

## Decisions (running log)

**1. F1 — unclear handoff contract, applied.** Read the findings from review
commit `zurvwzwz` (`282d0ba7`), against producer `tpoponvo` (`091abec6`).
`loop_driver.rs` waits for the session before interpreting its token; no token
stops the loop, while Relaunch enters tree transition again. The preparation
session can therefore return without a signal, leaving the cutover handle live, and
a human can approve the concrete plan in its running log before restarting.
The execution session must not stop its own waiting driver mid-cutover.
Its ending depends on the mandate's running version, not the PATH version.
`jj workspace list` confirms the default workspace is `../grove`; it must
follow the final converted change rather than leave a second old tree behind.
The cutover body and root summary will state actors, approval evidence, both
workspace roles and the resume path. The review's claim that an AFK command
has no possible live approval channel is too strong: `--ask-for-approval never`
governs tool approval, not user conversation. The stopped-loop record is the
reliable unattended handoff; existing explicit authorization remains valid.

**2. F2 — real inventory and teaching issues, applied.** A fresh listing of
`~/Development/*/.grove/FORMAT` and reads of their bytes confirm
`session-kinds-v1` in `APIAnyware.add-ocaml-target`, `Writegood` and
`grove.gh-issue-12`. Generalize the existing contents-based disposition to any
root `FORMAT`, including newly discovered trees, and correct the inventory.
The shipped `grove/references/driver.md` still teaches creation of a format
witness, while the lifecycle's witnessless-root contract no longer uses one.
Assign removal of that teaching to `node-methodology-k8`, which owns it.

**3. F3 — real size risk, accepted visibly with continuation guidance.** The
`grove-loop` manifest includes inline tests in the grammar/lifecycle roots;
the CLI source is also a `grove-llm` book root. The parser/fixture/book switch
is coupled under the explicit no-dual-reader contract. No independently green
split is established here; that does not prove that none could ever be found.
Name the supplied-name API plus caller adaptations as a candidate working seam
for `distinguished-names-k6`, with enforcement/conformance following if needed.
For `node-files-k7`, retain one completion boundary; if no working split can be
verified within the session, snapshot unfinished work, record the remaining
work in the existing log, and stop without retirement or signal. A human
restart selects the same live handle and resumes from its actual jj diff.
Do not invent a red child or treat a snapshot as a passing implementation.

**4. Evidence scope.** The graph CLI refused startup because an unverified
generation is active; no graph generation or coverage result is available.
Verification uses the exact source and manifest paths above, the review's
committed artifact, and fresh machine listings. This integration edits task
prose only; no product redesign or extra review context is needed.

**5. Verification and retirement.** `bash scripts/check.sh` exited 0: all
eight principal checks passed, including final validation of all six books.
The installed `grove-llm resolve` finds all six implementation handles at
positions 08–13, and `brief-chain` with the library leaf's path returns the
root brief. The initial handle argument to `brief-chain` was refused; its
help specifies a path, and the corrected invocation passed. The final diff
is confined to this task tree. `node-documentation-k9` already states its
surface, seams and book obligations and needs no edit. All review findings
are accounted for above; the root retains six live implementation leaves,
so retiring this integration closes no ancestor and changes no ADR decision.
