# owner-switch-recovery-k89

**Integrates:** owner-switch-recovery-k88


## Goal

Triage the committed recovery-design review and apply valid corrections
before skill-loading-replan-k80 consumes the owner-delivery contract.



## Context

Read owner-switch-recovery-k88's findings, model-run evidence and inspection
limits from its commit. Its reviewed producer is `40be4d66`, the output of
owner-switch-recovery-k87, against `0eee04ac857d`.

The human's three decisions of 2026-11-02 stay closed. mise's
`[bootstrap.repos]` owns the owner's checkouts. Each checkout stays at a
published commit. No tool-neutral contract names mise. The design adds no
Ravel or Grove interface, and the switch stays inside finish after
promotion. The finish decision and `CLAUDE.md` still govern that boundary.

## Done when

Each finding has an evidence-based disposition. Valid corrections agree
across the owning specifications, model, evidence, views and downstream
handoffs. Record relevant verification and leave a coherent contract for
the planner.

## Notes

Use the integration family's triage procedure. The review is an input to
assess, not a list of mandatory repairs. This leaf owns any repairs and
their verification. The review changed no design, production or test code.

## Decisions (running log)

**Base.** On 2026-11-02 the working copy was empty above `479a75276f0e`, the
commit of owner-switch-recovery-k88. The findings come from that commit's
retired review file. The reviewed producer is `40be4d66`.

**Triage.** Each finding was checked against the cited source, not against
the review's wording. All five hold. None needs a redesign: each has a
bounded repair inside the k87 design.

- **F1: a real issue, with a contract stated unclearly.** `flagged` in
  `owner-switch.qnt` tests the expected and stage contents before the merge
  record. A resolved copy that returns to its expected content is therefore
  not flagged, although the live file still holds the merged update. The
  prose rule "flagged when it holds neither recorded content" allows the
  same reading. The repair gives the merge digest sole authority over a
  resolved copy, and the owner's merge applies the difference from that
  base. The model gains removals of other keys, in the live file and
  through a descriptor, so that the trace is reachable.
- **F2: a real issue.** Step 1 exempts the whole descendant tree, and the
  interval rule bars probes only. A delegate harness session or a
  background command of the run's session can therefore read a moved path
  or capture a checkout. The repair names the processes that stay and
  counts every other descendant. The k87 text also dropped the driver chain
  of finish from the exemption, which k86 had named. The repair restores it,
  because the same sentence now names what stays.
- **F3: a real issue.** The outside manifest is compared only while no run
  is open. The complete continuation copies a receipt without that
  comparison. The repair moves the acceptance check into step 14. Step 14
  compares the entries outside the switch set before it copies the receipt.
  A difference counts as a failed probe, and the complete row runs step 14.
  The model gains the outside manifest as one layout flag, the manual and a
  fifth property.
- **F4: a real issue.** An exclusive rename cannot move a retained entry
  onto an occupied original path, so the two-path control asks for an
  impossible inverse. The repair moves an entry back only to an absent
  path. An occupied path is a third value: the inverse leaves both entries
  and reports the cell. The spec gains a complete argument for entry parts
  in place of a model extension.
- **F5: a real issue.** `owner-cutover.mmd` reuses `C` and `D`, so Mermaid
  merges the preparation and recovery nodes. The repair gives the recovery
  nodes their own identities.

**Model changes.** The model changed first, as the k87 handoff requires.
`flagged` gives a merge sole authority over a resolved copy, behind the
mechanism `MERGE_AUTHORITY`. The owner's merge applies the copy's difference
from its base, so a removal reaches the live file. Outside programs and late
descriptors can remove other keys.

A layout flag stands for a changed entry outside the switch set, and a
classification clears it. The receipt's copy into the manual is step 14.
Behind `ACCEPT_CHECK`, step 14 compares that flag first, and a difference
inverts as a failed probe. A complete continuation runs step 14 when the
manual lacks the receipt. P5, `inv_acceptedOnlyClassified`, says that no
receipt reaches the manual with a changed outside entry.

Three instrument changes were needed to reach the new traces. Copy indices
are now drawn from the existing copies, not from 0 to 40. Before that, no
merge happened while outside budget remained. An `idle` outside phase and a
`receipt` crash phase serve two new focused worlds, `merge_return` and
`entries`. The `m_no_expect` and conflict witnesses now exclude a key that a
later removal superseded, so a legitimate removal does not read as a loss.

**Model run.** `mise exec -- task model:owner-switch` exited 0 with Quint
0.33.1, at 60 steps and the runner's sample counts. Ten invariants held in
the nine design worlds. Every witness was reached, every mutant witness
stayed unreached in those worlds, and each of eight mutants reached its
violation. The model digest `4b3002d9d23d`, the runner digest `66c9fcfb6065`
and the Taskfile digest `2aec4fba957f` were the same before and after the
run.

`m_merge_no_authority` lost a removal in 44 of 4,000 traces, and
`merge_return` reported the same trace in 48. `m_accept_unchecked` accepted
a changed outside entry in 61 of 4,000. In `entries`, the complete
continuation copied a receipt in 523 traces and inverted on an outside entry
in 448. The results are sampled, not proved.

**Specification.** The owner specification states the kept processes in
"Concurrent writers" and the Terms, and step 1 excludes only them. Step 14
compares the entries outside the switch set, and the complete row runs step
14 when the manual lacks the receipt. The merge rule and the entry rule of
"Rollback" follow F1 and F4. "The recovery model" names five properties and
eight mutants, and it argues entry parts in full.

The test seams gain a removal fault point, a merged-copy return, two step-14
controls, a reader control and the two-path controls. The coordinated
recovery row needed no change: it delegates each continuation to "Resuming
the switch".

**Evidence and views.** The evidence records the new run, the instrument
changes and the dispositions. The cutover view gives the recovery nodes `DR`
and `CH` and shows step 14. The state view adds the step-14 inversion, and
the README and the panel name the integration.

**Verification.** The prose checker reports 0 hard findings in the
specification, the evidence and the design README. A scratch resolver
resolved all 59 relative links in them. A control file with a missing anchor
made it report one failure. `shellcheck` passes on the runner, and
`diagrams.json` parses.

A duplicate-identity scan of `owner-cutover.mmd` found `C` and `D` in the
reviewed version and none in the repaired version. A browser rendered the
cutover view with 21 distinct nodes and 30 edges, with the preparation
labels beside the recovery labels. The state view rendered with 7 nodes and
16 transitions, the step-14 label included. Neither view showed a syntax
error, and the only console error was a missing `favicon.ico`. This session
removed the browser tool's two files.

The full `scripts/check.sh` was not run, because no Rust, plugin, release or
book file changed.

**Handoff to skill-loading-replan-k80.** These items amend the k87 handoff:

1. The switch program lists processes against the kept processes only. A
   delegate harness session, a background command and a Ravel or Grove
   command of the session count as other processes. The session ends its own
   delegates and background tasks before it runs the program.
2. Step 14 of the program compares the entries outside the switch set before
   it copies the receipt. A difference counts as a failed probe. The complete
   continuation runs step 14 when the manual lacks the receipt.
3. The merge entry records the copy's digest, and that digest is then the
   only recorded content. The merge applies the difference from the last
   recorded content, a removed key included.
4. The inverse renames an entry only onto an absent original path, whatever
   the entry's change identity. An occupied path is a third value, and the
   inverse leaves both entries. The purge procedure checks each retained
   entry against its recorded change identity.
5. The rehearsal task carries the new controls of the test seams. `task
   model:owner-switch` stays green with its ten invariants and eight
   mutants.

**In-session review.** This leaf spent its one narrow reviewer on the
argument for entry parts, the only repair that no checker covers. The
reviewer read an extract of the entry rules and the four properties, without
the triage. It reported four P1, two P2 and three P3 items. Each was
classified against the specification:

- **A real issue.** "Moves an entry of any identity" changed a third value
  that two cell rules forbid. The rules now name the entry exception.
- **A real issue.** Purge had no condition for a retained entry that changed
  after its run closed. Purge now requires its recorded change identity.
- **A real issue.** The pair definition left the retained path undefined
  while no run is open. An entry cell is then the original path alone.
- **A contract stated unclearly.** "Identity" is the change identity that
  the evidence defines, so no meaning of it is open. The switch text now
  names it. The drifted row now says "last record", as the model does.
- **A contract stated unclearly.** The claim that the inverse cannot repeat
  a failure held only for an occupied path. It is now limited to that case,
  and another failed rename stops each run until the owner repairs it.
- **Noise.** A parent replaced with a link still puts the entry at its
  original path. The excerpt's "four properties" heading was the extract's
  own wording. The owner's inverse is a program entry and runs no step 3.
- **A visible trade-off.** A moved-back changed entry reads as drifted and
  waits for the owner, as every third value does.

The text read clean afterwards: 0 hard prose findings, and 61 resolved
links. These repairs change no rule that the model covers, so the model run
above still applies. A second review is not needed: each repair restates a
rule or adds a condition, and none changes a checked mechanism.

**Retirement.** No ADR changes. Each repair restates or adds a rule inside
the k87 design. The k87 finding that the redesign fails the ADR test still
holds. No review was cut, and no live review names this leaf, so no
Creator line is written. The parent node keeps live work in
skill-loading-replan-k80 and two subtrees, so no node closes.
