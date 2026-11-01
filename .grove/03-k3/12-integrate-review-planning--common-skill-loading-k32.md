# common-skill-loading-k32

**Integrates:** common-skill-loading-k31

## Goal

Triage the planning review's findings on the staged common-loading tree. Apply
the real ones before the first implementation subtree runs.

## Context

Read the findings from the review's own commit in this tree, not from this
body. The reviewed artifact is the subtree that common-skill-loading-k9
planned, with the common-loading brief. The review also read Ravel `main` and
the `Ravel.gh-issue-2` task tree in
`/Users/antony/Development/AgenticOrchestration/Ravel` and its second
workspace. Do not edit that workstream's tree or signal its sessions.

Some findings can need a decision that only the human can make. An example is
the release order between two workstreams. Ask the human for such a decision
with the options, a recommendation and the evidence. Do not choose for them.

## Done when

- Classify each finding as a contract stated unclearly, a real issue, a visible
  trade-off, or noise, and record the reason.
- Apply each accepted finding in the brief or task that owns it. Cut a design
  leaf, and any review it earns, where a finding needs an unsettled public
  decision. Place that leaf before the work that depends on it.
- Record each handoff that needs a Ravel or tap change, with the owner who
  makes it. Do not edit those repositories here.
- After the edits, the subtree briefs, the tasks and the common-loading brief
  agree with each other. Every tree verb resolves each changed handle.

## Notes

No product code, owner configuration, installed skill or release changes here.
Keep the ending command from this session's own launch prompt.
