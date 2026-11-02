# owner-pack-checkouts-k84

**Integrates:** owner-pack-checkouts-k83

## Goal

Triage the committed design review and apply the valid findings before
skill-loading-replan-k80 consumes the owner-delivery design.

## Context

Read owner-pack-checkouts-k83's findings from its commit and the reviewed
artifact of owner-pack-checkouts-k82. The review records the exact Grove and
Ravel contract revisions, evidence and verification limits. Preserve the
human's settled checkout owner, published-pin policy and tool-neutral product
contracts.

## Done when

Each finding has an evidence-based disposition. Valid corrections agree
across their owning contracts, evidence, views and handoffs. Record relevant
verification, and leave a coherent contract for the dependent planner.

## Notes

Follow the integration family's triage procedure. The findings are inputs to
assess, not an implementation checklist imposed by this task.

## Decisions (running log)

**Base.** On 2026-11-02 this checkout was at `e3070a5cc489`, the commit of
owner-pack-checkouts-k83. The reviewed artifact is `36f4bcb9`. Ravel's
local-source contract is `7a894c782037` in `Ravel.skill-directory-delivery`.

**Triage.** Each of the three findings reproduces against the cited text.

1. P2, a real issue. Lines 569–570 and 578–579 call the inverse, and line
   584 says that it restores the native state. Nothing covers an inverse that
   fails part of the way. A rerun then stops at step 3 on the moved entries.
   The pending-activation record names no backup. The fix keeps the design.
   A rerun reads the state of the switch set from the layout.
2. P2, a real issue. A receipt write that fails after passing probes has no
   branch, and step 3 stops on the moved entries again. Line 768 says that
   only finish runs the switch against the real HOME. Line 601 lets the owner
   run it. The same state reading gives an active layout its own route. That
   route reruns the checks and the probes, then writes the receipt.
3. P2, a real issue. A scratch measurement reproduced it with Git 2.56.0. A
   clone with `core.autocrlf=true` and `core.symlinks=false` had the same tree
   ID and an empty `git status --porcelain --ignored`. Its `SKILL.md` had
   CRLF line ends, and its link was a regular file. The local feature build
   of Ravel gave the two clones different `resourceDigest` values. A second
   plain clone at another path gave the plain digest again. The digest
   therefore does not depend on the path. The fix makes the digest the
   accepted identity and keeps the tree ID for the decision to rerun core
   trials.

The owner's effective Git configuration sets no `core.autocrlf`, `core.eol`,
`core.symlinks`, `core.attributesFile` or `core.sparseCheckout`. The global
file sets the LFS filters. The defect is in the check, not in today's
checkouts.

No finding needs a redesign. Each fix keeps the human's checkout owner, pin
policy and tool-neutral contracts, and adds no Ravel or Grove interface.

**Fixes.** The owner specification gains the term "switch set" and the
section "Resuming the switch". Step 3 of each run reads the state of the
switch set from the layout and the latest backup. The states are native,
partial, active, complete and unknown, and each has its next steps. Step 8
writes a new backup with the switched state and the evidence of steps 4, 6
and 7.

Step 9 and the inverse change each item atomically, and the inverse
acts only on an item in its switched state. The pending-activation record
names each backup and the state. Line 768's test seam now names the owner's
run. The rehearsals interrupt the switch, the inverse and the receipt.

For finding 3, the accepted identity of a source is its Ravel
`resourceDigest`. Step 6 resolves the drafted real settings in a disposable
HOME with the promoted `ravel`, and a probe compares the digest again. Step 4
gives a rerun's digest to `grove-core`. A control clone with
`core.autocrlf=true` must fail step 6. The tree ID still decides whether the
core trials run again.

The coordinated release design's step 5 records the accepted digests, and its
recovery row covers the stopped, reversed, partial and active cases by a
pointer. The evidence document records the scratch measurement and the
review corrections. The cutover view gains the state decision, the resumed
inverse and the edges for a failed step 9 and a missing receipt. The
final-session view and the design README follow.

**Verification so far.** The prose checker reports no hard finding in the
four changed Markdown documents. `diagrams.json` parses. Every relative
heading anchor in the changed documents resolves. Three explicit `<a id>`
targets that the resolver does not read exist in their files and predate
this change.

No Mermaid renderer is installed, so the changed views were not
rendered. A sweep for "tree ID", "inverse", "only finish runs",
"pending-activation" and `resourceDigest` across the checkout, `.grove/`
included, found one other owner of the switch handoff: the rollout brief.
Brief reconciliation belongs to skill-loading-replan-k80, as in k82.

**In-session reviewer.** One fresh reviewer received the switch, resumption,
rollback, cutover and recovery excerpts and their contract, without these
conclusions.
It reported two P1, seven P2 and two P3 findings. Each one reproduces
against the text:

- R1, P1, valid. A crash inside step 8 leaves a backup that step 3 trusts.
  A backup now becomes complete by one final rename, and step 3 sets an
  incomplete one aside.
- R2, P1, valid. A whole-file comparison of `~/.claude/settings.json` reads
  a tool's hook update or a marketplace reinstall as neither state, and the
  restore would revert that update. A file that other programs write is now
  compared at the switch's keys only. A drifted item goes to the owner's
  classification and a new native backup, as step 3 already does. The state
  that the first fixes called "unknown" is now "drifted".
- R3, P2, valid. Plugin state is a key in a shared file, so several plugin
  writes pass through other contents. Each file is now written once by a
  rename, after a check that it still equals its copy. Step 1 lists the
  programs that write those files.
- R4, P2, valid. No Ravel settings file exists today. Its absence is now its
  earlier state, and the inverse removes the file.
- R5, P2, valid. An entry at both paths is now in neither state.
- R6, P2, valid. Step 4 now reads the latest Grove release. That can be a
  patch release after a fix to the core skills.
- R7, P2, valid. In the active state, a stop in step 6 now counts as a failed
  probe.
- R8, P2, valid. The section "Rollback" now defines the native probe.
- R9, P2, valid. The coordinated recovery rows now name each state and keep
  any pending record. They separate a Grove patch release from a Ravel cut,
  and they cover a failed probe of a Grove kind skill.
- R10, P3, valid. `mac-check` now runs first. The receipt is written into the
  backup, then copied into the manual. The complete state reads the backup.
- R11, P3, valid. The backup now lies outside each tree that the instrument
  sweeps.
- The reviewer also asked that step 4 state the accepted `grove-core` digest
  when the tree IDs are equal. The step now states it.

R1 to R3 are substantive fixes, and no executable test covers them. That is a
second need for review, so this session cuts a focused `review-design` leaf
before skill-loading-replan-k80.

**Verification after the reviewer's fixes.** The prose checker reports no
hard finding in the four changed Markdown documents or in the new review
body. `diagrams.json` parses. All 23 relative anchors in the changed documents
resolve, the explicit `<a id>` targets included.

`task design:harness-selection` served the views, and a browser rendered the cutover, final-session and
pack-checkout views without a syntax error. The cutover view rendered 17 nodes
and each new branch label. The only console error was a missing
`favicon.ico`.

The browser tool wrote two files into `.playwright-mcp/` in the
checkout, and this session removed them.

A sweep of the changed files for step numbers found each reference consistent
with steps 1 to 14. No check script or walkthrough book reads the changed
documents, so `bash scripts/check.sh` would measure nothing of this change.
The finish integration runs it on the integrated result.

**Review cut.** The focused review owner-pack-checkouts-k85 is now before
skill-loading-replan-k80. Its body holds the doubts that remain. No provider
evidence is in this session's environment, so the review has no Creator line.
k80's notes now point at this handoff.

## Handoff to skill-loading-replan-k80

These obligations add to the handoff in owner-pack-checkouts-k82. The last
item replaces k82's item about the continuation. The leaves that
owner-ravel-skill-inventory-k38 and discovery-rollout-k12 grow consume them:

- The staging trials record each source's `resourceDigest` from
  `ravel skills resolve --json`. After the Ravel cut, rollout records each
  digest again with the published archives. The evidence names each accepted
  digest.
- Step 6 of the switch resolves the drafted real settings in a disposable
  HOME with the promoted `ravel`. Each digest must equal its accepted digest.
  A probe compares the digest again.
- Step 1 also lists each program that writes a file of the switch set.
- Step 8 writes each backup outside the trees that the instrument sweeps. A
  final rename makes it complete. Step 9 writes each file once by a rename,
  after a comparison with its copy.
- The rehearsals interrupt steps 8 and 9, the inverse and the receipt. Other
  controls change a hook, put an entry at both paths and clone one source
  with `core.autocrlf=true`.
- Steps 12 to 14 run `mac-check`, write the receipt into the backup and copy
  it into the manual.
- Rollout names the switch, its probes, the native probe and the section
  "Resuming the switch" in the finish handoff. That section states the five
  states and the owner's run after finish. The sentence of the rollout brief
  that names the switch, its probes and its inverse follows this item.
