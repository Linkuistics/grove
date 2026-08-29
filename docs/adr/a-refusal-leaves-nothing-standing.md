# A refusal leaves nothing standing, and its reason names the question that failed

The semantic contract fixes three closed sets — six outcomes, twenty-one refusal
reasons, two blocked diagnoses — and, until this record, never stated the **map**
from a situation onto them. Two independently built model families each supplied
their own map, and where the two maps disagreed both families were green, because
a map nothing states is a map nothing can be checked against. This record states
it, in two clauses.

## 1 — the outcome is decided by what the action leaves, never by where it stopped

An action returns `Refused(r)` when the tree it hands back equals the tree it was
given — whether because no step ran at all, or because every step that ran was
undone. It returns `Blocked(b)` when an effect stands that the action could
neither carry to completion nor reverse. The operational content is the one
`FN-29` requires an operator to be able to read: **a refusal leaves nothing to
recover, and a block does.**

Four consequences follow, and each was a live mistake before it was written
down.

- **The unit is the action** — not the step, and not the operating-system
  process. A step that mutated nothing inside an action that has moved everything
  is a block, and an effect an *earlier* lifecycle transition applied does not
  make a later transition's clean stop one.
- **The action includes its unwind**, which is what makes *the tree it hands
  back* well-defined at all. A tree mutation is applied through
  `crates/ordinal-fs-tree`'s one interpreter, which unwinds every effect it
  applied on a reported error — `Error::Failed` says "the tree is as it was
  found", checked there as `inv_atomicity` — so a mid-flight collision is a
  refusal however far the operation had gone, and the only outcome that leaves an
  effect standing is an unwind that itself fails
  (`Error::FailedPartiallyRolledBack`). That one is the boundary's to report and
  the contract absorbs no member for it.
- **A rollback earns the refusal back.** `NotCommitted` reaches the commit and
  still ends `Refused`, because what the caller is handed is the tree it gave.
- **An intermediate state is not a return.** `FN-22.f`'s quarantine return
  settles to `Reserved(Published)` and the *attempt* completes as `Refused` from
  the restoration path; the outcome belongs to the attempt.

The rule is checkable rather than a preference, and it is now checked:
`FN-29.b`.

**The second consequence was the last one written and it corrected a scoping
argument this record had already made.** `FN-29.b` was placed in `grove-finish`
alone on the ground that "the task-tree scope has no block to be distinguished
from". A task-tree mutation *can* leave an effect standing — when the unwind
fails — so the ground is false. The placement stands for a better reason: that
outcome reaches the operator in the delegated boundary's own vocabulary, because
Grove prints the library's errors verbatim rather than re-wording them
([`CONTEXT-MAP.md`](../../CONTEXT-MAP.md)), so it is not a block the contract
names. **What made this invisible was an ungranted capability.** Two
independently built model families were free to invent an interpreter without an
unwind, one did, and it then reported the consequence as a missing row in the
contract's own outcome table rather than as a model without a capability. The
grant is now `EN-17`, and the general form is worth more than the instance: an
assumption no row names is one a model may quietly decline.

**The trade-off it settles.** The rejected reading is the step-local one — *this
step mutated nothing, therefore refused* — and it is not a straw man. It is
strictly cheaper to state and to check: a frame condition over one transition,
which is what let `crates/grove-finish/models/finish.als` express it at all,
where the action-level property needs the whole attempt in scope. Its cost is
that `Refused` stops meaning *nothing to recover*. Once a refusal can follow an
evacuation, the operator's first question after every refusal becomes *how far
did it get*, and the distinction `FN-29` exists to draw is gone. That cost is
not payable, so the cheaper reading is rejected.

**Reopen** only if `Blocked` ceases to be the protocol's recovery signal — if
Grove gains a durable per-attempt state an operator consults instead of the
outcome, the outcome no longer has to carry the distinction and the step-local
reading becomes admissible again.

## 2 — a reason names the question that was asked, and the set gains a member when a scope asks a new one

A refusal reason names **the question that was asked and answered no**, never the
gate that asked it. The catalogue already rested on this before it was stated: an
unsupported layout and an unreachable quarantine operand share `LayoutUnsupported`
because `SY-03` makes them one question asked at two gates.

What follows is the widening rule. **The closed reason set gains a member exactly
when a scope asks a question no member names** — and reporting such a case under
the closest true member instead is not a smaller version of the same fix but a
different and worse one, because the reason then names a question that was not
asked.

The seventeen original reasons were drawn over the questions the **task-tree**
scope asks: preconditions and guards on a tree. The set is swept by **three**
scopes. Three of the four members added under this record are a question a
*later* scope asks — a commit's disposition (`DeletionNotCommitted`), a
configuration (`ConfigurationInvalid`), a launch generation
(`GenerationContended`) — which is the pattern under what the finish scope had
recorded as three separate accidents, and it predicts where the next gap is
rather than merely listing the closed ones.

**The fourth is the exception, and it is what the rule needed to be complete.**
`ScaffoldIncomplete(class)` is a question the **task-tree** scope asks, and the
seventeen were supposed to have been drawn over exactly those. What the original
draw missed was not a later scope's question but a **state the contract had not
finished defining**: a witnessless root, which had one member where the product
has three
([`a-witnessless-root-refuses-what-it-cannot-account-for`](a-witnessless-root-refuses-what-it-cannot-account-for.md)).
So the widening rule gains a second clause — **a scope also asks a question no
member names when a state is refined** — and the reason survives the refinement
without being restated, because it names the question and not the state's
extension.

**The trade-off it settles.** The rejected alternative is the one both families
independently chose, twice: report under the closest true member and keep the
case distinguishable with a **model-only observable** — `Sys.why` in
`crates/grove-finish/models/finish.als` and its Quint counterpart. It is
genuinely cheaper. Widening a closed set imposes a matching outcome on every
column that sweeps it, so a member costs model work in two families across every
scope that reaches it, while an observable costs one declaration. And a model has
no other move while the set is closed against it, so choosing it was correct of
both. Its cost is borne by the operator rather than by the model: an operator
told `WitnessPending` cannot learn from it that the *repository*, not the
filesystem, is what refused. A device a model must invent twice to say what an
outcome could not is evidence about the vocabulary, not about the model.

**Reopen** if the reason ever stops reaching an operator — if refusal reporting
moves to a structured cause chain that names the gate independently, the reason
no longer has to carry the question and a smaller set becomes defensible.

## Why one record and not five

Five closed-set gaps were referred here. Under clause 2 three of them are members
(above); under clause 1 one of them dissolves — a tracked witness needs no reason
because `FN-13`'s stop is a block, and a block carries a diagnosis — and one is
not a gap at all, because Grove has no confirmation gate and a call never made
returns nothing. The five were one missing map, and a record per member would
have recorded the symptoms.

## What this record does not claim

It does not claim either model family was careless. Both read the catalogue as
written, and the catalogue was underdetermined at exactly the two points where
they diverged; where it happened to supply the missing predicate in a neighbouring
sentence — `FN-17.b`'s *blocks rather than proceeds*, three lines below
`FN-16`'s *refused* — the two agreed without ever discussing it. The finding is
about the document.

It also does not change product behaviour. The shipped implementation reports
commit failures through the version-control error chain and materialises no
refusal-reason taxonomy; whether the shipped diagnostic adopts these names is
`handoff-audit-k66`'s, beside the other four product-facing diagnostic questions.

## What enforces it

**Neither the catalogue this record reads against nor the models that found the
defect still exists.** The catalogue was `docs/specs/semantic-contract.md`,
deleted with the campaign's apparatus (`delete-formal-models-k29`); the two model
families this record measures — `finish.als`'s step-local frame condition and its
Quint counterpart, and the `Sys.why` observable both columns had to invent — went
with `crates/grove-finish/models/` at `delete-finish-models-k30`. So what §2 says
about the catalogue being underdetermined at two points, and what the trade-off
section says about what `finish.als` could and could not express, are statements
about documents a reader can no longer open.

The rule itself is action-level and outlives all of it: an outcome is decided by
what the **attempt** leaves standing, never by where a step stopped. It reaches
shipped behaviour nowhere yet — the implementation still reports commit failures
through the version-control error chain and materialises no refusal-reason
taxonomy — so this record binds the *next* protocol rather than the current one,
and that is stated here rather than left to be discovered. The decision survived
the instrument that found it, which is the outcome that campaign was run to test,
and `docs/formalism-findings.md` keeps the record of how it was found.
