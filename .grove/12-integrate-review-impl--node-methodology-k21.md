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
