# finish-scope-k75

**Reviews:** `finish-scope-k71`.

## Goal

Read `finish-scope-k71`'s landed dispositions adversarially, and decide whether
the reversal at its centre left the seven artifacts it touched saying the same
thing.

## Context

**This review exists because a specific signal fired, not because a spec was
edited.** `references/execute.md` says a picked producer spends **one**
in-session reviewer, and that a *second* need — normally re-review after a
substantive non-mechanical fix — is the mechanical signal that review has become
tree-sized work. `k71` spent its one reviewer, the reviewer **broke the
disposition that was then landed**, and `k71` reversed it across the catalogue,
both model files, the control file, both READMEs, the glossary and two ADRs.
That reversal is the substantive non-mechanical fix, and no fresh context has
read the state it produced — the reviewer read the *mid-flight* state, and read
it with `k71`'s conclusion still written into the model comments, which is a
declared flaw in that pass rather than a hidden one.

**It is inserted rather than appended, and the argument is the node brief's
own.** `lifecycle-scope-k72` is chartered to edit §*States* and both model
families against exactly the change under review, and this node's brief already
records that a gate checked after everything it gates is not a gate
(`obligation-placement-k63`'s `review-design` was inserted for the same reason).
Appending would leave a reviewer reconciling a diff with a tree `k72` had moved.

**What `k71` decided, in one line each**, so this session re-derives rather than
re-reads:

| item | disposition | class |
|---|---|---|
| 9 · `FN-25.b` | each blocked diagnosis IS its first sentence; the printed instances illustrate and do not exhaust it; `OwnershipConflict`'s topology clause gains the correlation proviso; `OwnershipConflict` wins where both hold | `MN` |
| 14 · `FN-28` | operands restated over Grove's own steps; new ADR `success-is-proved-by-the-ticket-not-the-tree` | `MN` |
| 23 + 24 | one repair: `Reserved(Quarantined)` joins the reserved class and the class moves ahead of `Absent` | `MC` |
| 28 | the general form of *never ungrades* is **declined** | `MN` |
| `W9SlotPending` | `doCommitAttempt`'s ordering guard returns `NoOp` and gains its own `why` | `MN` |
| `EN-08` / `FN-31.c` | declared unmeetable in the Alloy column, met in the Quint one | `MN` |

## Done when

- Each of the six is either confirmed or has a stated defect, with the evidence
  that decides it.
- The reversal's consistency is checked across every artifact it touched:
  `docs/specs/semantic-contract.md`, `crates/grove-finish/models/finish.als`,
  `finish.qnt`, `finish-controls.qnt`, `crates/grove-finish/models/README.md`,
  `CONTEXT.md`, `docs/adr/success-is-proved-by-the-ticket-not-the-tree.md`,
  `docs/adr/root-lifecycle-stays-with-its-receipt.md`, and the handoffs written
  into `lifecycle-scope-k72` and `handoff-audit-k66`.
- If there are findings worth acting on, an `integrate-review-design` leaf is
  cut; if there are none, this leaf creates nothing and retires.

## Notes

**The five doubts, sharpest first.** Each is stated as something that could be
wrong rather than as a question to consider.

1. **`Reserved(Quarantined)` may still be wrong, and the residual objection is
   already on the record.** `FN-19`'s witness said *an absent task root* before
   `k71` reworded it to *a free task-root name* — the catalogue itself used the
   loose phrase for that disk, which is what let the collision hide. Read
   `FN-22`'s three post-rename rows, `SY-05.a`/`SY-05.b`, and
   `src/finish_transaction.rs`'s `quarantine_and_dispose_with_checkpoint`
   together, and decide independently whether the deletion is settled when the
   rename lands. `k71` says no, and everything else rests on that.

2. **The member may have consequences `k71` declared away rather than checked.**
   Two `TT-` claims were declared unchanged: `TT-19` does not reach a standing
   quarantine (its recovery is a sweep that refuses nothing), and `TT-18`'s
   stages do not move (its reserved stage reads a *witness*, which lives beneath
   the root). Both are arguments, not runs. **Is there a third claim over the
   reserved class that does move?** `k71` found `FN-24.a` and `SY-05.b` only
   after a reviewer said its enumeration was incomplete — so the enumeration has
   already been wrong once.

3. **`FN-20`'s subject was narrowed, and narrowing is how claims die quietly.**
   It now reads over the **commit's disposition** and not the task root's state,
   and `finish.qnt`'s two-world instrument moved with it. The stated reason is
   that the wide reading would forbid `FN-21.b`'s reaper reading its own marker.
   **Check what the narrow reading stops catching**: the Quint column's previous
   instrument was strictly stronger, and something it caught may now be
   unchecked by either family.

4. **`W9SlotPending`'s branch now returns `NoOp`, which removes it from
   `FN-29.b`'s antecedent.** That is a shrinking antecedent, and a shrinking
   antecedent is how a check becomes vacuous. Confirm the branch is genuinely not
   a return — that the transaction can still complete or unwind from
   `PublishedP` — and that no command's reachability depended on the old atom.

5. **The `EN-08` declaration may be an excuse rather than a finding.** *A model
   that posits a disk under `EN-11` cannot also exercise `EN-08` at that disk* is
   a general claim made from one instance and a state count that was estimated
   rather than run. If it is wrong, the honest disposition is a deep witness the
   Alloy column can actually reach.

**What this review is not for.** The product question `k71` routed to
`handoff-audit-k66` — whether the shipped reaper should re-read the disposition
before disposing — is that leaf's and re-arguing it here duplicates it. Nor is
this a re-run: both cells are green, the digests are recorded either side, and
`k71`'s run lines are in its body. **If a finding needs a run, say which command
and why, rather than re-running the scope to look.**

## Findings

1. **[P1] `FN-25.a` still does not state a disjoint partition, and the Alloy
   check now exempts the reached counterexamples.** The contract says that each
   diagnosis is its broad first sentence and that the following instances do not
   exhaust it (`docs/specs/semantic-contract.md:891-904`), then says both that
   `OwnershipConflict` wins when both definitions hold and that no blocked state
   satisfies both (`docs/specs/semantic-contract.md:946-955`, `1950-1959`). The
   correlation proviso added to one illustrated topology instance cannot narrow
   the declared first-sentence definition *state is unrelated, ambiguous, or
   cannot be proved safe to mutate*. Alloy exposes the contradiction directly:
   `diagnosedRaw` permits both arms, `declaredDiagnosisOverlap` names two reached
   overlap classes, and `FN_25a` weakens `lone diagnosedRaw` by exempting them
   (`crates/grove-finish/models/finish.als:1339-1382`, `4906-4931`). Quint instead
   makes correlation win over every `unprovable` state except its much narrower
   `cannotClassify` predicate (`crates/grove-finish/models/finish.qnt:1837-1888`).
   Both models can therefore return one diagnosis, but neither establishes the
   contract's stronger claim that the definitions themselves are disjoint.

2. **[P1] `Reserved(Quarantined)` collapses an unsettled handoff and a proven
   success-with-cleanup-outstanding into one state whose documented meaning fits
   only the first.** The new table and prose define every standing quarantine as
   a transaction that is incomplete (`docs/specs/semantic-contract.md:414-418`,
   `454-464`, `487-489`), and the glossary and ticket ADR repeat that it is
   evidence a finish is unfinished (`CONTEXT.md:407-424`,
   `docs/adr/success-is-proved-by-the-ticket-not-the-tree.md:50-56`). Yet the same
   contract says an unchanged `Committed` after the fourth revalidation is
   `Applied` with the quarantine still holding the root, and that cleanup still
   outstanding must not undo success (`docs/specs/semantic-contract.md:1845-1848`,
   `2023-2034`). The Quint transition records exactly that state before disposal
   (`crates/grove-finish/models/finish.qnt:1722-1731`), while both classifiers put
   **any** standing quarantine in `Reserved(Quarantined)`
   (`crates/grove-finish/models/finish.qnt:756-767`,
   `crates/grove-finish/models/finish.als:1076-1082`). The product also returns
   success after the fourth proof even when disposal fails
   (`src/finish_transaction.rs:1953-1974`). Thus a cleanup failure after proven
   success is simultaneously `Applied` and a reserved state defined as
   unfinished. The handoff to `lifecycle-scope-k72` covers only the pre-fourth-
   revalidation window, so it would carry this conflation into lifecycle
   classification (`06-design-lifecycle-scope-k72.md:242-259`).

3. **[P2] The Alloy `EN-08`/`FN-31.c` disposition promotes an unrun scope-cost
   estimate into a logical incompatibility.** The catalogue and README say a
   model that posits the disk under `EN-11` *cannot* also exercise `EN-08`, but
   their supporting argument is that reaching the disk takes about seventeen
   states while the current expensive scope stops at thirteen
   (`docs/specs/semantic-contract.md:1130-1150`,
   `crates/grove-finish/models/README.md:1565-1582`). That establishes a current
   bound gap, not that the assumptions conflict: the described seventeen-state
   trace is itself a candidate that uses both. The deciding evidence is an Alloy
   `run` for the two `FN-31.c` resumptions at the calculated deeper bound, with
   `crash` removed as the negative control. Until that measurement exists, the
   honest column disposition is an unmeasured/dear bound gap rather than
   “unmeetable.”

4. **[P2] The `W9SlotPending` reversal left `FN-11`'s own non-vacuity argument
   describing behavior the model no longer has.** `doCommitAttempt` now returns
   `NoOp` with `W18EvacuationIncomplete`, but the same branch's comment says
   `gateEvacuated` “still refuses” the early attempt
   (`crates/grove-finish/models/finish.als:1841-1859`), and the `FN-11` command
   says the early attempt is a “REACHABLE refusal”
   (`crates/grove-finish/models/finish.als:2940-2950`). The core decision that an
   internal wait is not a completed outcome is sound; the defect is that the
   artifact still claims refusal is what keeps `FN-11` from holding by
   construction. Its explanation must be restated at the step/action grain and
   the existing applied-after-evacuation witness retained as the reachability
   evidence.

## Disposition verdicts

- Item 9 / `FN-25`: **defect**, finding 1.
- Item 14 / `FN-28`: **confirmed**. The contract makes the exact ticket and
  Grove's own steps the operands, preserves success across outstanding cleanup,
  and the ADR gives the two disk-shape counterexamples that rule out a `stat`
  receipt (`docs/specs/semantic-contract.md:2023-2050`,
  `docs/adr/success-is-proved-by-the-ticket-not-the-tree.md:1-41`).
- Items 23 + 24 / `Reserved(Quarantined)` and precedence: **defect**, finding 2.
- Item 28 / monotonic grade: **confirmed declined**. `FN-22` deliberately needs
  both post-rename `Committed -> NotCommitted` and `Committed -> Indeterminate`,
  so the proposed monotonic premise would remove required states
  (`docs/adr/root-lifecycle-stays-with-its-receipt.md:62-89`).
- `W9SlotPending`: **semantic decision confirmed; cross-artifact defect**,
  finding 4.
- `EN-08` / `FN-31.c`: **defect**, finding 3.
