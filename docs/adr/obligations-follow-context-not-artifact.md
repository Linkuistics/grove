# An obligation belongs to the scope that can execute its context

The claim catalogue's identifier prefix is a **crate assignment**, not a filing
convention: `models/run.sh` sends every `TT_`-prefixed command to
`crates/grove-task-tree/models/`, every `FN_` command to
`crates/grove-finish/models/` and every `SY_` command to `models/system/`, and a
command whose prefix disagrees with its directory is a hard placement error. So
deciding an obligation's prefix decides which crate must deliver the behaviour.

The three scopes are ordered by the approved crate dependency direction —
`grove-task-tree` → `grove-finish` → the application joint — and placement
follows that order in three clauses, applied in sequence:

1. **Direction.** An obligation may name states, actions, outcomes and artifacts
   from scopes at or below its own, and never one from a scope above it. An
   obligation that names something from above belongs to the **highest scope its
   text names**.
2. **Observation, not machinery.** A scope above may name a lower scope's
   *observation* — a fact readable at the boundary — and never its *step*. A
   clause naming a lower scope's step stays where it is only as a declared
   **cross-scope citation** to the obligation that owns the step, and the
   citation carries that obligation's declared narrowings with it.
3. **The joint is for what no crate delivers alone.** `SY-` is for a claim no
   single crate can deliver, not for any claim that merely mentions two.
4. **A quantifier is read in its own scope's admitted set.** Some terms the
   catalogue's *Vocabulary* defines are **partitioned across groups that belong
   to different scopes** — *action* is the one that matters, split across
   Observation, Tree mutation, Finish, Lifecycle and Environment. Such a term
   has no single owning scope, so clauses 1 – 3 cannot place an obligation
   quantified over it. That obligation is read **prefix-locally**: it ranges
   over exactly what its own scope admits, and its text SHALL say so, in
   `SY-14`'s idiom — *no **admitted** action*. A property that must hold across
   every scope's actions is therefore **not one obligation but one per scope**,
   each stated and checked where its actions live.

The rule is checkable rather than a preference: read the obligation's text, look
each term up in the catalogue's *Vocabulary*, take the highest owning scope, and
compare it with the prefix. It is falsifiable by exhibiting a term whose owner
disagrees, which is what every recorded instance was. Clause 4 is what makes
that procedure total: without it a group-spanning term returns no owner and the
lookup has nothing to compare.

## What a cross-scope citation may not do

A citation carries the cited obligation's declared narrowings (clause 2), and
**clause 4's prefix-local reading is such a narrowing**. So a scope may not cite
a lower scope's prefix-local obligation as evidence about an **action that
scope does not admit**. The cited command was executed against a different
action set; a green over that set says nothing about this one, and a citation
that hides the difference manufactures exactly the confidence the coverage
matrix exists to withhold.

This is not hypothetical, and it is why clause 4 exists. `TT-24.a` — *no action
mutates an entry whose ownership it cannot prove* — was landed under `TT-` with
prose asserting it "reaches both contexts wherever a model admits them", while
the finish column simultaneously cited its task-tree coverage as the
shared-safety evidence for a row whose mutation changes the **quarantine
reaper**. Both readings cannot stand: under the universal reading `TT-24.a` names
`grove-finish`'s and the lifecycle's actions and clause 1 moves it out of `TT-`;
under the prefix-local reading its commands never saw a reaper. The prefix-local
reading is the one that survives, because it is what both families' commands
actually check — Alloy's `TT_24a` quantifies over the task-tree file's own
transitions and Quint's reads a flag only that file's steps set. `TT-24.a` now
says *no admitted action*, and the row that cited it across the boundary reads
`none`.

## The trade-off it settles

The competing rule is *the scope that owns the artifact the claim names*, and it
is what the catalogue had. `TT-24` — fail-closed ownership — is filed under `TT-`
because its artifact, a foreign entry at a name Grove reserves, is the task
tree's. But two of its three contexts are `grove-finish`'s: a live finish or
recovery transaction, and the quarantine reaper. `grove-finish` depends on
`grove-task-tree` and not the reverse, so filing those obligations under `TT-`
asked the **lower** crate to deliver an **upper** crate's behaviour. That is a
dependency inversion, and it is the whole of the argument.

What it cost, measured rather than predicted. Alloy declared both cells
`out-of-bounds` and said filling them would answer the claim by construction.
Quint filled them, by importing a finish transaction, a quarantine reaper and a
ninth outcome into the task-tree model — and `inv_TT_24c` then restated the
model's own gate branch, with **no control able to kill it**. The coverage matrix
printed `TT-24.c alloy:gap quint:ok`, which reads as the declining family being
behind. It was not. **A model can always answer an obligation by importing
enough machinery to state it, at which point its property has no content beyond
the import** — so *can this scope state it* is the wrong test and *whose action
is it* is the right one.

Two further costs surfaced only when the re-statement was attempted, and both
argue the same way. `TT-24.c`'s antecedent does not survive translation: the
finish model's own predicate for *a foreign entry at a reserved name* is written
from the reaper's standpoint and fires on every ordinary forward path inside a
transaction. And its consequent is false — `finish.als` **refuses** where the
claim said it blocks, `finish.qnt` **blocks** at the same step, and both are
green against a claim whose text says only *fails closed*. An obligation that
changes truth value under translation was never checkable where it sat.

## Alternatives rejected

- **Keep the artifact rule and let both families declare gaps.** Self-refuting as
  a resting place by the catalogue's own text: a gap declared on both sides "is a
  finding about the catalogue rather than a covered obligation", and the runner
  already counts those separately. It also leaves required behaviour checked
  nowhere while the matrix reports two families' worth of honest work. Reopen if
  the catalogue ever stops treating a both-family gap as a finding.
- **Relax the runner's placement rule, so a `TT_` command may live in the finish
  directory.** Rejected because the prefix is the crate assignment: relaxing it
  means the catalogue stops saying which crate delivers a claim, which is the
  question this record exists to answer. It would also let the two families place
  one claim in different directories, which is the divergence the per-family
  coverage rule exists to prevent. Reopen only if models stop living beside the
  component they constrain.
- **A fourth prefix for cross-scope obligations.** Rejected because it needs a
  fourth model directory with no crate behind it, against the root brief's rule
  that models are owned beside the semantic component they constrain. The joint
  already has a home; clause 3 is what stops that home absorbing every claim that
  mentions two scopes. Reopen if a claim appears that two crates must *jointly*
  deliver and neither can deliver alone.
- **Read a group-spanning quantifier universally, and move the obligation to the
  highest scope any group belongs to.** This is clause 4's live alternative and
  it was rejected on cost and on truth. `TT-24.a` would become an `SY-`
  obligation, because the *action* partition reaches the Lifecycle group; the
  lifecycle model would then owe a fail-closed-ownership property over an action
  set it abstracts, in both families, and the task-tree column — which has the
  only model that can actually attempt a mutation against a foreign entry —
  would carry none. It also states something no command in the repository
  checks, since no single model admits every group's actions. Reopen if a scope
  ever appears whose model admits the whole action partition.
- **Require a control for every obligation, and let falsifiability decide
  placement.** Rejected as the *primary* test because it is necessary and not
  sufficient: it catches `TT-24.c`, whose transcription no control kills, and
  misses `TT-24.d`, whose imported reaper carried a control that fired
  perfectly well while still being another crate's action. It survives as the
  evidence a contested cell is reported with. Reopen never — the two tests are
  complements, not competitors.

## What enforces it

Placement itself is already deterministic: the runner refuses a command whose
prefix disagrees with its directory. What was invisible is the case above, where
both families are internally consistent and disagree with each other, so
`models/run.sh` now reports a **contested cell** — one family answering what
another declared out of reach — together with whether the answering family
carries a control naming that obligation. It is reported and never fatal: a
family may honestly answer what another cannot express, and a control is not
always available. The point is that the next reader of the coverage matrix meets
the fact instead of reading `gap` as a deficit.

**The report says what kind of answer the answering family gave**, because the
first version did not and that made its own evidence statement false. The
coverage matrix calls a cell complete only when a family supplies **both** a
property and a witness; the contested report credited a family with a
property alone, so a vacuous property whose antecedent nothing reaches was
printed as *answered* in the very line introduced to expose false confidence. It
now distinguishes a complete answer from a property-only one and counts the
latter separately. `models/run-controls.sh` carries the positive and negative
controls for both halves of the line — the completeness distinction, and the
extractor that decides whether the answering family carries a control.
