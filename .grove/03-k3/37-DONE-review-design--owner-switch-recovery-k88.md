# owner-switch-recovery-k88

**Reviews:** owner-switch-recovery-k87

## Goal

Read the rebuilt recovery model of the owner switch adversarially before
skill-loading-replan-k80 consumes it. Produce findings and no edits.

## Context

The artifact is the Grove commit of owner-switch-recovery-k87. Read its diff
against its parent `0eee04ac857d`, the commit of owner-pack-checkouts-k86.

The subject is these parts of `docs/specs/owner-skill-delivery.md`:

- The new terms: cell, part, plan, program and kept copy.
- The sections "The switch inside finish", "Concurrent writers", "Resuming
  the switch", "The rehearsed plan", "Rollback" and "The recovery model".
- The purge conditions, the order-6 row and the switch seams.

The subject also includes the owner recovery row of
`docs/specs/coordinated-release.md`. The model is `docs/models/owner-switch.qnt`
with its runner `docs/models/run-owner-switch.sh`. Run
`task model:owner-switch`. The evidence is
`docs/owner-skill-delivery-evidence.md`: the k87 rows of "Publication by
atomic exchange", "The recovery model" and the disposition table at the end
of "Review corrections". k87's decision log records each design decision and
the verification.

The defects that k87 answers are R1 to R12 and the P3 items in the decision
log of owner-pack-checkouts-k86. Compare the finish order with `CLAUDE.md`
and `docs/adr/finish-promotes-after-the-grove-release.md`.

The human's three decisions of 2026-11-02 stay closed. mise's
`[bootstrap.repos]` owns the owner's checkouts. Each checkout stays at a
published commit. No tool-neutral contract names mise. The design adds no
Ravel or Grove interface, and the switch stays inside finish after promotion.

## Done when

Each finding names its location, its severity and a concrete trace or text
that shows it. Record what the review did not inspect. Cut an integration
leaf only if a finding is worth acting on.

## Notes

The model checks consistency with itself. It cannot show that the right
properties were stated. Attack the statement as well as the result. The
producer could not settle these doubts:

1. The model treats each part as a file. An entry moves by an exclusive
   rename, and the inverse moves back an entry of any identity. Look for an
   entry trace that loses a write or that the state table misreads.
2. The accepted case lets a run replace an outside write of a cell's earlier
   or switched value. mise tracks the shared files and syncs them between
   machines. Decide whether that case can undo a deliberate change on
   another machine without a report that the owner sees.
3. Exclusion now protects readers, captures and progress only. The run's own
   process tree may run, and the staging descendant trial is gone. Look for a
   reader or capture that this admits.
4. A flagged kept copy stops step 8 and step 13 until the owner merges it.
   Check that no flagged copy can keep finish from restoring native
   delivery.
5. Each continuation of a backup runs the program copy in that backup. A
   defect in that copy then has no supported repair route. Decide whether
   the design needs one.
6. The manual says that a receipt applies only while `mac-check` reports
   the switch complete. That rule is prose, and no model holds it.
7. The inverse leaves a Ravel settings file that holds no key. No
   measurement shows how Ravel 1.1 and Ravel 2.0 read it.
8. The results are sampled. Each world ran 4,000 samples of up to 60 steps
   with three parts. Some witnesses landed in fewer than 50 traces.
   Compare the model's `state` and table predicates with the specification's
   table row by row.

Use the review family's procedure. The review makes no change to design,
production or test code.

## Decisions (running log)

**Subject.** The reviewed producer is `40be4d66`, against `0eee04ac857d`.
The working copy began empty. This review uses the review family, the
codebase-design criteria and the Quint audit checklist. It changes only the
review record and its task-tree handoff.

**Model run.** `mise exec -- task model:owner-switch` exited 0 with Quint
0.33.1, at the runner's default 60 steps and sample counts. No counterexample
was observed for the nine invariants in any of the seven design worlds.
Every requested witness was reached, every mutant witness was unreached in
the design worlds, and every mutant reached its named violation. The
classification world used 12,000 samples; the unchecked-receipt mutant used
20,000; the other worlds used 4,000. The model, runner and Taskfile digests
were unchanged before and after the run. The model and runner match the
producer's recorded digests. These sampled results do not settle omissions
from the transition relation.

**Findings settled so far.** Three text traces warrant integration: the
reader exclusion admits a live descendant that still reads moved paths; the
complete row ignores changes outside the switch set; and the promised
inverse of an entry at both paths cannot perform its exclusive rename.
The findings below carry their locations, consequences and correction
boundaries. No repair is applied here.

**A predicate check.** The existing `flagged` function returns `false` when a
merged copy changes back to its recorded expected content. The same call
with one fresh unrelated write returns `true`, a positive control. These
were expression evaluations in the existing `writers` module, using Quint's
TypeScript REPL backend. No model file was changed. This is evidence about
the predicate on supplied content, not a claim that the current transition
relation can reach the first input: its unrelated writes only add fresh
tokens. Finding F1 states the omitted real-world trace.

## Findings

### F1 — P1: a merged kept copy can change without becoming flagged again

**Locations:** `docs/models/owner-switch.qnt:197–202, 533–557`;
`docs/specs/owner-skill-delivery.md:753–764, 982–985`.

The specification requires a copy that changes after its merge to be
flagged again. The predicate first excludes both the original expected
content and the original stage content, even for a resolved copy. Those
exclusions take precedence over comparison with `resolvedHolds`.

A concrete trace is: an exchange displaces an unrelated-key update into a
flagged copy; the owner merges it and records its digest; a writer through
the old descriptor then restores the copy's earlier unrelated-key content.
Its cell need not change. If the resulting content equals `expect`,
`flagged` is false. The live file still holds the merged update, so the later
edit is neither live nor reported in a flagged copy. Step 8, step 13 and the
purge condition can all admit that copy. This is not the accepted case of a
cell write of an earlier or switched value.

The expression check used these three contents:

```quint
val expected = { cell: 0, by: 0, rest: Set(7) }
val prepared = { cell: 1, by: 0, rest: Set(7) }
val merged = { cell: 0, by: 0, rest: Set(7, 8) }
flagged({ part: 0, holds: expected, expect: expected, hasExpect: true,
          prepared: prepared, exchanged: true, resolved: true,
          resolvedHolds: merged }) // false
```

Changing `holds` to `merged.with("rest", Set(7, 8, 9))` returned `true`.
`outsideRest` and `lateWrite` only union fresh tokens into `rest`, so the
sampled runs cannot expose a replacement that returns to older content.

The smallest useful correction is to give the recorded merge digest
authority for a resolved copy: any later difference must flag it, including
equality with an older expected or stage digest. Include replacement or
deletion of unrelated content among the model's outside writes, and require
a witness/control for the trace above.

### F2 — P2: the process-tree exemption admits readers that need exclusion

**Locations:** `docs/specs/owner-skill-delivery.md:575–583, 780–793`.

Detection of writes cannot protect a reader whose path disappears. Step 1
excludes the session's entire descendant tree. The interval rule expressly
bars a still-running probe, but it does not bar another descendant reader or
capture.

For example, a Claude Code session starts a Codex delegate that receives
native skill paths, then runs the switch while that delegate is still live.
The delegate is excluded at step 1 and before step 9 because it is in the
run's own tree. It is not a probe. Step 9 moves its native links, and its next
lazy read fails. An existing descendant capture has the analogous exemption
when step 5 advances a checkout. One foreground command prevents new agent
actions inside the interval; it does not stop an already-running descendant.

Keep the writer exemption separate from the reader/capture exclusion. Name
which necessary processes can stay, and require other descendant readers
and captures to end before their protected interval. A reader control must
be independent of the write-detection controls.

### F3 — P2: the complete continuation skips changes outside its switch set

**Locations:** `docs/specs/owner-skill-delivery.md:584–590, 823–841`;
`docs/models/owner-switch.qnt:70–74, 230–264`.

The manifest comparison outside the switch set is required only when no run
is open. A run remains open after its receipt. The active row has an extra
check for changed entries outside the switch set, but the complete row has
none.

Trace: step 13 writes the receipt; the session ends before step 14; a tool
or `mise dot sync` adds a native skill at a path outside the recorded switch
set, then exits. The original cells still hold their switched values. On the
next run, no process blocks step 2. Step 3 reruns the inventory, but an open
run skips the outside-manifest comparison. The complete row copies the
receipt and changes nothing else, so finish can succeed with unclassified
native guidance active. An `UNCOVERED` row is called a changed entry, but
the changed row requires no open run and cannot catch this layout.

Compare the outside manifest before accepting both active and complete
continuations, and define the stop/classification route for a difference.
Carry this case into the state predicates or an explicit rehearsal control.
The current model omits the outside manifest, so its state-table checks do
not address this trace.

### F4 — P2: the two-path entry control requires an impossible inverse

**Locations:** `docs/specs/owner-skill-delivery.md:767–768, 912–916,
1120–1123`; `docs/owner-skill-delivery-evidence.md:344, 359`;
`docs/models/owner-switch.qnt:60–63`.

The entry control puts an entry at both the original and retained paths,
then requires the owner's inverse to move the retained entry back. The
inverse uses an exclusive rename, which the recorded platform measurement
says refuses an existing target with `EEXIST`.

Trace: the switch moves entry A to the retained tree; another program
creates B at A's original path; the state is drifted; the owner's inverse
tries the exclusive rename. It fails without changing either path. Every
retry has the same result. Allowing an entry of any identity does not remove
the target collision. The general permission to restore a cell by hand does
not specify which of these two entries to keep at the discovery path, and
the control still requires the inverse itself to move the entry back.

Define preservation and owner resolution of the two-path collision before
requiring native restoration, and make the control expect that stop until
resolution. Include entry-path occupancy in a model or a complete argument
about entry recovery. The current file-only model cannot check this case.

### F5 — P3: the cutover diagram reuses preparation node identities

**Location:**
`docs/design/harness-selection-and-execution/owner-cutover.mmd:6–7, 23–24`.

The new drifted node reuses `D`, which already names draft preparation. The
new changed node reuses `C`, which already names re-inventory. Mermaid node
identities are shared across subgraphs, so these are the same two nodes
with replacement labels and both sets of edges. For example, preparation's
`C --> D --> E` now also connects the changed/drifted recovery nodes to the
staging-trial node. A render without a syntax error does not show that the
workflow nodes remain distinct.

Give the recovery nodes distinct identities and inspect both preparation
and finish in the resulting view.

## Review coverage and limits

The read covered the producer's committed switch changes, the cited R1–R12
and P3 dispositions in k86, the new terms, purge and order-6 conditions,
the switch seams, the coordinated recovery row, the full Quint model and
runner, the exchange and model evidence, and the two changed Mermaid
sources. The finish order agrees with `CLAUDE.md` and
`finish-promotes-after-the-grove-release`: the prepared switch remains
after Grove publication and Ravel promotion, before the inherited final
signal. No new durable decision was made, so the existing ADR set needs no
change from this review.

The codebase-design read supports one program owning state interpretation
and mutation, and the plan/program digest binding has a concrete rehearsal
purpose. The required failing-call contracts remain unresolved at the
boundaries identified in F1–F4.

The Quint checklist has these dispositions:

| Check | Result |
| --- | --- |
| Logic in thin guarded actions | Warn: several actions contain branches and updates directly. No separate correctness finding follows from their placement. |
| Typed phases | Warn: phases, records and outcomes use raw strings. The reviewed actions only construct the documented values. |
| Cohesive state | Pass: one `st: World` holds the world, and every action assigns that variable. |
| Witnesses | Pass for the runner's requested cases; every requested witness was reached. Warn: append-only unrelated writes omit F1. |
| Abstraction | Warn: entries, multiple cells per file, metadata, process exclusion, the program copy, outside-manifest changes and the manual are outside the model. |
| Nontrivial invariants | Pass for presence: loss, receipts and rehearsal admission have named properties and mutants. Their scope is limited to the modelled writes and cells. |
| Initialization and variable assignment | Pass in the executed runs and direct source read: `init` assigns `st`; actions assign the whole record. |
| Step composition | Pass for inclusion of the declared major actions. The explicit stutter means these runs do not establish recovery progress or absence of deadlock. |
| Initial and sampled invariant checks | No counterexample observed at initialization or along the executed sampled traces. This is not exhaustive verification. |

The six state predicates agree with the prose table for the model's
single-cell file parts. That agreement does not extend the table to the
omitted outside manifest. Flagged copies do not guard `ownerInverse` or
`resumeInverse`, so their presence alone does not block native restoration.
The accepted earlier/switched cell-write case is explicit, and the prose
requires each replaced value to be listed. Its owner-visible reporting,
the frozen program's repair route and the manual's use of `mac-check` were
not exercised. The no-key Ravel settings measurement remains assigned to
order 6; this review makes no compatibility claim from it.

No production/test code or design artifact was edited, and no test suite,
build, lint or formatter was run. The model task was run because this leaf
explicitly requires it. No real-HOME switch, native probe, filesystem
exchange measurement, actual-harness trial or browser render was performed.
The recorded platform measurements were read as evidence. There is no
claim about the whole installed collection or repositories outside the
bounded artifact.

Graph discovery used the indexed Grove project at generation
`2026-11-02T12:09:57Z` with Tier 2 verification. Runner discovery, its
one-hop caller/callee trace and the `check_dead` snippet had complete returned
pages. Coverage checks found no recorded gap in the cited Markdown and
runner. Quint had no graph symbols and was reported `not_tracked`, so its
full source was read directly. The changed Mermaid sources were also read
directly; the cutover source has recorded partial-parse coverage. These are
best-effort coverage signals, not a completeness proof.

**Handoff.** The five findings warrant an integration before replanning.
`owner-switch-recovery-k89` was inserted immediately before
`skill-loading-replan-k80`. It carries this review's handle and consumes
the committed findings for triage; it does not adopt them as its charter.
The parent remains live with that integration and the later planning work,
so this retirement closes no parent node.
