# external-pack-contract-k164

**Integrates:** external-pack-contract-k163

## Goal

Triage the findings in external-pack-contract-k163's committed review, and
apply the real design corrections before pack-conformance-runner-k156 uses
the contract.

## Context

- The review's commit, identified by external-pack-contract-k163. Read its
  findings there, rather than treating this body as a list of required fixes.
- The reviewed artifact is `docs/specs/external-pack-contract.md`, with the
  ADRs, reconciled sections and views named by the producer
  external-pack-contract-k155. The review identifies its exact base.

## Done when

- Each finding has a source-supported disposition under the integration
  procedure. Apply accepted corrections to the design and its affected
  handoffs, and record accepted trade-offs or rejected findings with reasons.
- The contract and its cited records are coherent with the brief chain.
  Dependent leaves consume the settled design, and applicable documentation
  verification is recorded.
- Externalise substantial redesign as a producer/review chain before the
  dependent runner work, as the integration procedure directs.

## Notes

This step immediately precedes pack-conformance-runner-k156. Keep design
integration within its charter. Implementation and its tests belong to their
own leaves.

## Decisions (running log)

**Source.** The findings are in commit `0d55e1a4` of
external-pack-contract-k163. It reviewed `872faec9` against `ec33e3bd`. Each
finding below was checked against the current files, not against the review's
summary of them.

**F1, a real issue: a family across packs.** The root brief keeps the shared
review and integration procedures in core and moves most of their members to
domain packs. Rule 2 of the placement record had no output for that case.

The correction adds one branch. When one pack ships every kind of a family, the
file is in that pack. Otherwise it stays in the spine's `references/`. Every
kind loads the spine already, so the branch adds no dependency. The contract,
the record and the planned glossary and README changes now state it. The
prefix load sets `review-*` and `integrate-review-*` stay in core and bind the
kinds of the run.

**F2, a real issue: the order of publication.** The contract assigned a
published Grove commit to discovery-rollout-k6, but rollout runs before finish
publishes `main`. The owner delivery design already publishes this
workstream's line, in order 2 of its cutover contract.

The correction moves that publication earlier. The line goes to Grove's remote
as the bookmark `gh-issue-25` before a pack publishes a pin to a commit on it.
A published pack then runs `task check` from a clean checkout with its
committed public URL. Finish pushes only `main` and its tag, so the remote
bookmark keeps the recorded commits after the rebase. The contract forbids a
move or a deletion of that bookmark while a published pack records a commit
that only the bookmark holds.

k157, k158, k162 and the node brief changed to match. The rejected option was a
pin correction after finish. Each published pack could not check itself until
then. That option would also add a step to finish that the root brief does not
assign.

**F3, a real issue: a resource of another pack.** Under F1, a moved review
member reads `references/review.md` in core's spine. The contract's alone mode
forbade core and still required that path to resolve.

The correction states the owner by rule 3. The run tries the referring skill
and then the loaded skills in load order. It stops at the first loaded skill
that the pack does not ship, and it lists the path there as an external
resource. A path that resolves nowhere, in a file without an external load,
stays a missing file. The aggregate resolves each external resource. Two
controls and one clean fixture cover both outcomes.

**F4, a real issue: the fixtures.** The current clean controls copy only part
of core and read core's whole manifest. `run_runner` also discards the exit
status. Under the rule that an absent owner fails, those clean readings would
fail for the wrong reason.

The contract now requires complete fixtures, a clean reading before each
mutation, and an assertion of both the exit status and the diagnostic. The
k156 body carries the migration.

**Note on the edge form, a contract stated unclearly.** The pair's opening says
"Then load", which an exact match of "Load the" would miss. Rule 1 now states
the match. The first letter can be either case, and emphasis does not count.
The phrase can wrap inside one paragraph. Comments, fenced code and block
quotes give no edge.

A grep of core found the form only in the kinds' openings, so the rule adds no
false edge there. Its lowercase control was seen to work on a scratch input.

**Note on the kind table, a visible trade-off.** The reviewer showed a bounded
check that keeps kinds open. It needs a map from the table's pack names to the
supplied packs, which the runner does not take. The gap stays accepted, and the
contract now gives that reason instead of the broader claim.

**Note on `${CLAUDE_PLUGIN_ROOT}`, a contract stated unclearly.** The skills
documentation says that Claude Code substitutes it in plugin skill text, for
resources that a plugin's skills share. The contract claimed that no native
plugin has a stable shared path. The family-skill decision stands, because
directory delivery and the manual installer substitute nothing. The reason and
the rejected option now say so.

**Note on the missing-skill sentence, no change to the contract.** The review
found its target in `prompt.rs`. k161 now also records the Grove build that
renders the mandate, as the review asks.

**An integrator observation outside the review: core's aliases.** Rule 2 binds
the text of every pack, core included. Core's kind openings still write
`grove:grove`, and no leaf changed them. The contract now names that state and
gives the remaining openings to repository-moves-k5. Each moved kind takes the
bare-name opening in its own move. The k11 planning body carries the item.

**No new producer chain.** Each correction changes one section of the contract
or one placement branch. None of them rethinks the contract.

**Verification.** The checker of the technical-communication skill reports no
hard finding in each changed prose file. Two preparatory commits converted
this task file and the k11 body first. `bash plugins/grove/conformance.sh`
exits 0, and no plugin file changed. `diagrams.json` parses as JSON.

No Mermaid command is installed, so nothing rendered the two changed labels in
`conformance.mmd`. A search of the crates, the test support and the scripts
found no test that reads an edited document. Its positive control found the
names of two other specifications.

**No in-session reviewer.** Each correction is bounded, and a later leaf tests
it. k156 runs the controls, and k157 fetches the recorded commit from the
public URL. The release procedure shows the one claim that no leaf tests: it
pushes only `main` and the release tag.
