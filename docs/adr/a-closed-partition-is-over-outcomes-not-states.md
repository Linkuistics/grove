# A closed partition is over outcomes, not states, and precedence is the claim

[`a-refusal-leaves-nothing-standing`](a-refusal-leaves-nothing-standing.md)
fixed the map from a situation onto the six outcomes. This record answers the
question one level down, inside `Blocked`: `FN-25` requires every block to carry
exactly one of `RecoveryPending` and `OwnershipConflict`, and the two
definitions the contract gives **overlap on states the protocol reaches by
design**. A partition over the *states* is therefore false; a partition over the
*carried diagnosis* is true, and the thing that makes it true is a stated
precedence.

## The decision

**Where both definitions hold of one disk, the block carries
`OwnershipConflict`, and `FN-25.a` claims that rather than disjointness.** The
rule is `TT-24`'s applied to a diagnosis: *the outcome names the strongest thing
Grove cannot account for*, because that is what decides what the operator must
not be told to run. Reporting `RecoveryPending` over a disk carrying an artifact
Grove cannot classify tells an operator to run a recovery against bytes Grove
cannot prove are its own, which is the fail-closed violation `TT-24` exists to
prevent.

The precedence has a **floor**, and it is half the decision. *`OwnershipConflict`
wins the overlap* must not become *`OwnershipConflict` wins*: a correlated block
with nothing unaccountable beside it carries `RecoveryPending`, which is what
`FN-22`'s revalidation table requires of every `Indeterminate` row and what
`OwnershipConflict`'s second instance's correlation proviso exists to make true.

## The trade-off it settles, and the alternative rejected

The alternative is to make the two definitions **genuinely disjoint**, by
narrowing `RecoveryPending` to *a correlated Grove-owned attempt is incomplete
**and** Grove can account for everything else it can see*. That is a real,
statable contract and it is rejected: it makes `FN-25.a` true by construction.
`crates/grove-finish/models/finish.als` names the trap in place — *an arm
narrowed until it cannot meet its neighbour is an arm that answers `FN-25.a` by
construction* — so the disjoint form buys a claim no model can fail and no
mutation can kill. It trades a false claim for a vacuous one.

The precedence form costs something real in exchange: the overlap must be
**declared**, exhaustively, and a class that reaches a block without being
declared has to fail the check rather than be resolved quietly by the
precedence.

## What made this worth a record rather than a line in the contract

The generalisable half is about the **check**, not about the diagnosis.

**A check that exempts its own declared counterexamples tests nothing where the
claim is hardest.** `finish.als` named the two reachable overlap classes in
`declaredDiagnosisOverlap` and then weakened `FN_25a` by exempting them, which
reads as honesty — the overlap is declared, in one place, beside the check — and
is not. It was measured: with the precedence relation **fully reversed**, so
that Grove offers a recovery against a disk it cannot account for, the exempted
check stayed **green**. It never tested the precedence at all.

The Quint column had the same hole wearing the other hat: its `FN-25.a` was *the
two diagnoses are disjoint*, which its if/else classifier made true by
construction — a claim no mutation could move. **Two independently built
families answered the same disks differently and both were green**, which is the
same failure `a-refusal-leaves-nothing-standing` records one level up, and for
the same reason: the contract stated a map that nothing checked.

**And a clause can be load-bearing for a claim other than the one it is labelled
with.** The exemption's first half, `lone diagnosedRaw or
declaredDiagnosisOverlap`, was labelled with the disjointness that was false and
was deleted with that label attached. Read as a check it says something else and
true — *the arms meet only where the file says they meet* — and without it a
third overlap class would satisfy *exactly one carried* and pass in silence. It
is kept beside the precedence clause rather than replaced by it. The rule this
leaves: **before deleting a clause you have just proved false, ask what else it
was checking.**

## Where this binds

`docs/specs/semantic-contract.md` §*Outcomes* and `FN-25.a`;
`crates/grove-finish/models/finish.als`
(`FN_25a_the_carried_diagnosis_is_unique_and_is_the_one_precedence_selects`,
whose four conjuncts are separately falsified by matrix rows 51, 52 and 65);
`crates/grove-finish/models/finish.qnt` (the `OWNERSHIP_WINS_THE_OVERLAP` dial
and `mutant_correlation_wins_the_overlap`); `CONTEXT.md`'s **Recovery pending /
Ownership conflict** entry.

Whether the **shipped** diagnostic adopts the precedence — or the two names at
all — is a product question and is `handoff-audit-k66`'s. This record binds the
contract and the models.
