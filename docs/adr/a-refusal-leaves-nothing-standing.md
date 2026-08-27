# A refusal leaves nothing standing, and its reason names the question that failed

The semantic contract fixes three closed sets — six outcomes, twenty refusal
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

Three consequences follow, and each was a live mistake before it was written
down.

- **The unit is the action** — not the step, and not the operating-system
  process. A step that mutated nothing inside an action that has moved everything
  is a block, and an effect an *earlier* lifecycle transition applied does not
  make a later transition's clean stop one.
- **A rollback earns the refusal back.** `NotCommitted` reaches the commit and
  still ends `Refused`, because what the caller is handed is the tree it gave.
- **An intermediate state is not a return.** `FN-22.f`'s quarantine return
  settles to `Reserved(Published)` and the *attempt* completes as `Refused` from
  the restoration path; the outcome belongs to the attempt.

The rule is checkable rather than a preference, and it is now checked:
`FN-29.b`.

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
scopes. Every member added under this record is a question a *later* scope asks —
a commit's disposition (`DeletionNotCommitted`), a configuration
(`ConfigurationInvalid`), a launch generation (`GenerationContended`) — which is
the pattern under what the finish scope had recorded as three separate accidents,
and it predicts where the next gap is rather than merely listing the closed ones.

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
