# node-methodology-k21

**Integrates:** node-methodology-k20

## Goal

Triage the review's findings against the shipped methodology and apply the
real ones, so that `node-documentation-k9` teaches from a corpus whose
refusal path and node-file guidance are coherent.

## Context

Read the findings from the review's own commit, by its handle above. The
artifact is the shipped skill corpus under `plugins/grove/skills/`, its rule
registry and the conformance checker with its controls. The installed 20.2.0
binary and this live tree's naming stay as they are until `node-cutover-k10`.

## Done when

- Each finding is classified and the accepted ones are applied at the rule's
  owner, with consumers reconciled and no naming history introduced.
- Any finding whose fix lies outside this artifact is routed, not absorbed:
  recorded where its owner will meet it, or cut as a leaf with its `--kind`.
- Plugin conformance and its controls pass; `bash scripts/check.sh` is green
  before committing.

## Notes

Phrase-pinned controls exist for the two rules the review reads most closely;
a rewording must keep, or re-pin, each pinned phrase.

## Decisions (running log)

1. F1 is a contract stated unclearly. The naming ADR, architecture's guarded
   snapshot contract and driver reference establish whole-tree refusal and
   operator recovery. Bootstrap now states the session's stop without a
   completion signal; BRIEF-FORMAT states the whole-read scope. The existing
   pinned phrases remain intact. This applies to grammar refusals, not every
   possible command failure.
2. F2 is a contract stated unclearly. BRIEF-FORMAT already assigns creation to
   leaf-decompose and root-init; the architecture requires creation within their
   plans. Replace the competing imperative with that fact and assign body edits
   to sessions. No runtime behavior or design decision changes.
3. F3 is a real terminology issue outside this artifact. Source confirms the
   cited help and diagnostics use brief for the file role. Route the correction
   and its book obligations to node-cutover-k10, the remaining code/release
   owner; do not absorb a code/book change into this methodology repair.
4. F4 is an accepted checker trade-off. The classifier intentionally verifies
   skill references, not concrete repository paths or full tree grammar. Fix
   the stale comment and state that concrete tree paths take the repo arm;
   leave classification and controls unchanged.
5. Graph discovery could not establish a project or generation: the CLI failed
   to start because an unverified generation was active. Verification uses the
   review commit a3ec3bf4 and its exact cited source/documentation, with no
   claim of graph coverage. The narrow wording repair needs no redesign or
   additional in-session reviewer.

## Verification

`bash scripts/check.sh` passed all eight principal checks, including plugin
conformance, its mutation controls, workspace Rust tests and final reconstruction
of all six walkthrough books. Both rule-deletion controls relevant to F1/F2
passed. SHA-256 checks before and after verification matched for bootstrap.md,
BRIEF-FORMAT.md, conformance.sh, conformance.test.sh and conformance/rules.tsv
under plugins/grove. These controls check pinned wording and reachability;
they do not establish semantic compliance by future sessions.

The existing ADR set already states the repaired contract and needs no change.
This root-level leaf closes no parent node: documentation, cutover and migration
remain live.
