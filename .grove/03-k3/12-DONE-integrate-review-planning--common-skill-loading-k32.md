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

## Decisions (running log)

Finding 1 is a real issue. Ravel main `24a54523` reserves request schema 3
for Rhai, keeps caller prompt bytes unchanged and uses worker protocol 1.
The directory design `b93bb927` instead assigns schema 3 to prepared prompts,
uses TypeScript helpers and targets protocol 2. The default Ravel workspace
still holds that design line; its files are not evidence of main's contract.
Separate design leaves will repair the Ravel contract and its Grove consumer
before local implementation. They own the public choices and earned reviews;
this integration changes no product specification or Ravel repository.

Finding 2 is a real issue. Grove's current resolver admits only stable 1.x
from 1.1.0, and its formula depends on unversioned Ravel and installs
`policy.ts`. The other workstream's live release and upgrade tasks advance
that formula to Rhai 2.0.0. Its release requires human approval, so this is a
coordination risk rather than proof that an incompatible release occurred.
The human has been asked to choose the release order. No hold, tap change or
message to the other workstream has been enacted here.

Finding 3 is a contract stated unclearly. Reproducible commits alone say
nothing about their landing base or eligibility for release. The repaired
design must pin its main base. Local and pinned stages will stay on the named
Ravel bookmark `skill-directory-delivery`, outside main, until the complete
contract passes acceptance and rollout integrates it. Each owning stage
rebases and checks its inputs at entry and records the resulting exact base;
rollout owns the final rebase, full checks and landing. This avoids publishing
the incomplete local subset. The Ravel owner must receive this handoff before
cross-repository implementation starts.

Finding 4 is a real scope issue, although the review did not measure session
length. Cleanup is independently reviewable documentation work. It now has a
sibling leaf directly after writer removal in the same unreleased native
increment. Writer removal still repairs every consumer it makes false; the
native node retains its existing completion criteria.

Finding 5 is a contract stated unclearly. A fixture can prove Grove's handling
of a supplied report and force-stop result, but cannot prove an agent follows
the mandate. Fixture ownership stays in the native implementation leaves.
Acceptance owns separate native-only missing-kind cases in actual Claude Code
and Codex, with injection disabled and the kind absent from isolated native
roots. Existing real installations remain untouched.

Finding 6 is a contract stated unclearly. The temporary local build must refuse
a valid Git source before configuration evaluation with a remedy naming the
unfinished pinned stage. It must not call that valid settings document invalid
or introduce a stable published error code. Pinned delivery removes this
temporary refusal and its fixture. The local build cannot reach a release.

Tree verification after the independent repairs resolved all sixteen changed
handles to existing entries. `brief-chain` returned root, common-loading and
local-delivery briefs for k24, and `kind` returned `design` for k34. The diff
was inspected. These are task-tree edits only; no product suite was run.
No ending signal was sent while the human's answer was pending.

The human then expanded the grove: "in this grove, do the ravel release work
as well, and coordinate the two", and "remove the current skill installations
and install the skills for the new ravel skill insertion", covering "all of
our skills ... such as testanyware". Ravel release execution and the owner's
full skill-delivery cutover now belong to this grove. Separate design leaves
own coordinated publication and the installation/resource inventory; rollout
owns concrete release, activation and removal work. The existing rollout
brief's versioned-formula route is the starting publication strategy, subject
to the coordinated design. This answer replaces the pending release-order
question; do not require a second generic approval to plan or execute the
authorized Ravel release and installation migration.

No release hold or message has been sent to the other workstream. The human
owns conveying that release ownership moved here; the coordinated release
design must verify a single release owner and exact landed inputs before any
publication, reconcile already completed steps, and preserve the other tree.
Any plugin tools, native hooks, adapters or harness-specific behavior must be
inventoried and preserved or explicitly repaired and tested alongside the
Markdown skill delivery. TestAnyware is an explicit member, not the limit of
the inventory. Existing installs are removed after replacement acceptance,
with lazy readers and this driver's pinned 1.1.0 ending preserved.

Final tree verification resolved all twenty-three changed handles to existing
entries after renumbering. The rollout inventory's brief chain is root then
discovery-rollout-k6, and the next coordinated-release task has kind `design`.
The reviewed diff changes task-tree Markdown only. No parent closes: the
common-loading node still has live design, implementation and acceptance work.
No review leaf names this integration as its producer, so there is no Creator
receipt to write. Product checks and release verification remain with the
implementation and release leaves; this session ran tree-interface checks.
