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
