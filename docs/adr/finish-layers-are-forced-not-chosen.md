# Finish keeps three layers, and each is forced rather than chosen

The finish protocol's three nested crash-safe transactions — the finish
transaction, the commit seam, and quarantine cleanup with its marker and
replacement — are **all kept**. Each of the four questions
`TODO.finish_process.md` raised answers **keep**, and none of the four is a
matter of taste: every cheaper protocol either breaks a shared-safety claim, or
requires a capability the environment does not supply.

This record replaces `TODO.finish_process.md`, which asked the four questions and
said to delete itself into an ADR if the answer turned out to be *keep it as it
is*. It did.

| question | verdict | what decides it |
|---|---|---|
| **Q1 — does the quarantine need to exist?** | **keep** | `EN-03` + `EN-08` + `FN-24.a`: in-place removal is multi-step, an interruption lands between two of them, and §*States* has no member for a partially removed task root |
| **Q2 — can the three dispositions become two?** | **keep** | `FN-15.d` answered by the **witness** branch in both families: `Indeterminate` is reachable on every lane under the incumbent |
| **Q3 — is the marker-replacement sub-transaction reachable?** | **keep** | `FN-31.a`'s witness lands in both families; the source state is reached rather than posited |
| **Q4 — what does finish still owe the user?** | **keep** | six of ten removal-matrix rows name a shared-safety obligation; the three that read `none` are forced by Q1 and Q3 |

**Each verdict is produced by a rule the catalogue fixed in advance**, not by
judgement, and the rule is worth naming beside the answer:

- **Q1** — *a candidate is checked against the shared-safety claims only*, and a
  counterfactual-capability control *adds a capability, to ask whether a cheaper
  protocol is admissible*. Admissible is what it returned; available is what a
  removal needs.
- **Q2** — `FN-15.d` is an **either/or** obligation, and *a question whose
  deciding witness is never reached is `defer`, not delete — the only exception
  being a check that positively establishes unreachability within a stated
  bound*. Both families took the reachability branch instead, which is the
  opposite answer rather than a missing one.
- **Q3** — *`FN-31.a`'s witness merely failing to land is a `defer`*. It landed,
  in both families, from a source state that is reached rather than posited.
- **Q4** — *a row reading `none` in both families is Q4's evidence for
  delete/replace; a row naming an obligation is evidence the artifact is
  protecting the user rather than Grove*. Six rows name one. The three that read
  `none` are read below for what a `none` can and cannot establish.

**"The model is smaller" is not evidence**, and no verdict above uses a line
count as one — the cost table is the price of the answer, never an argument for
it.

## What keeping it costs, which is the whole of the other side

| module | lines | role |
|---|---|---|
| `src/finish_transaction.rs` | 3,632 | preflight, witness, evacuation, rollback, quarantine handoff, recovery |
| `src/repo/finish_commit.rs` | 2,953 | the Git / native-jj / colocated-jj commit seam and its three dispositions |
| `src/finish_cleanup.rs` | 950 | post-commit quarantine disposal |
| `src/finish_cleanup/auxiliary.rs` | 1,257 | the cleanup marker protocol |
| `src/finish_cleanup/auxiliary/marker_replacement.rs` | 960 | the crash-safe marker-replacement sub-transaction |
| `src/finish_cleanup/unix.rs` | 535 | raw `openat` / `renameat2` / `unlinkat` wrappers, 31 `unsafe` blocks |
| `src/finish_cleanup/reaper.rs` | 79 | lease-owned reaping of orphaned quarantine |
| **total** | **10,366** | **34% of `src/`**, plus 6,701 lines of test |

For one operation, run once per grove, at the end. The 2026-08-17 simplification
pass measured this and left it alone as a redesign rather than a
contract-preserving simplification; the formal phase is what was asked to decide
it.

## Q1 — the quarantine is what an absent capability forces

`EN-03` grants that **there is no atomic recursive directory deletion**. So
removing a task root's contents takes more than one step; `EN-08` grants an
interruption between any two of them; and
[the catalogue's §*States*](../specs/semantic-contract.md#states) has **no member
for a partially removed task root**. The members it would fall into each mean
something else — `Legacy` (somebody else's tree, and fail-closed once migration
is withdrawn), `Malformed`, `Current(Spent)`, `PartialScaffold(_)` — which is
exactly what `FN-24.a` forbids: *never into a state that is indistinguishable
from a different one*. `FN-24` is **shared safety**, so the argument uses no
claim a candidate protocol is allowed to replace.

The only atomic step the environment grants is a same-directory rename
(`EN-01`), so the task root must leave its own name in one such rename, and the
target of that rename **is** the quarantine. `Reserved(Quarantined)` is the
stable state that makes the interruption classifiable, which is why removing that
member from §*States* kills `FN-24.a` outright — *a standing quarantine reads as
an ordinary grove*
([`crates/grove-finish/models/README.md`](../../crates/grove-finish/models/README.md),
control row 919). The Alloy column states the same conclusion about the shipped
protocol twice:
[`finish.als`](../../crates/grove-finish/models/finish.als) — *"`EN-03` — no
atomic recursive deletion — already forces the shipped removal to take entry by
entry"*, and *"getting rid of it is the one thing in the protocol that cannot be
one move, so it is the one thing an interruption can leave half-done."*

Everything below the quarantine follows from having one. Disposal is multi-step,
so it must be re-enterable (`FN-21.a`); the quarantine sits at a name `EN-13`
says a foreign entry may occupy, so a sweep needs a document proving what is
Grove's (`FN-21.b`, `FN-21.c`); a document that records progress must be advanced
without a reader ever seeing it absent or doubled (`FN-31.b`), which is the
replace transition. **The marker protocol is not a layer that grew to protect an
intermediate state the first two could avoid producing** — the intermediate state
is `EN-03`'s, and the first two layers cannot avoid producing it.

## Q2 — the third disposition is the shape of an external effect

`FN-15.d` is an either/or obligation — `Indeterminate` reachable by witness, *or*
positively unreachable within a stated bound — and **both families took the
witness branch under the incumbent**: `witness_FN_15d_{git,nativejj,colocatedjj}_indeterminate_reached`
first landing at nine states, and `wit_FN_15d_indeterminate_on_{git,native_jj,colocated_jj}`
in [`finish-controls.qnt`](../../crates/grove-finish/models/finish-controls.qnt).
`Indeterminate` is reachable on every lane, in both columns. The unreachability
branch was taken only under `relax_EN_05` — commit and evacuation as one step —
and even there by randomized simulation, where the catalogue demands an
exhaustive run.

`Recovery pending` exists because neither a commit nor its absence can always be
proven from outside the commit, and no lane escapes it. It is not surface the
protocol chose.

## Q3 — the replacement is reached, not posited

Both families land `FN-31.a`'s witness. Alloy reaches the source state at twelve
states by running the protocol from the disk an interruption mid-evacuation
leaves, through the rename, the marker and the removal, and crashing before the
marker is retired. Quint reads a flag the disposal steps set, and a state
requiring replacement is reached more often than a disposal runs to completion —
the replacement is *forced before* disposal can finish rather than occurring
occasionally after it.

The enumeration Q3 asked for is **one class, not a list**: a cleanup marker left
standing by a disposal that completed the removal it authorised and was
interrupted before retiring it. It exists because retiring the marker is
disposal's last step, and it is last because a document recording that a removal
has not happened cannot precede the removal.

## Q4 — six rows protect the user, three protect Grove, and none can be removed

The removal matrix in
[`crates/grove-finish/models/README.md`](../../crates/grove-finish/models/README.md)
names a shared-safety obligation for the reserved witness, the evacuation
manifest, its ready mark, the correlation ticket, the recorded anchor and the
deletion fingerprint; the index image is `abstracted`; and the quarantine, the
cleanup marker and the replace transition read `none` in both families. So the
answer to *how much of the machinery protects the repository as against Grove's
own intermediate artifacts* is: **three artifacts, and they are the cleanup
layer.**

**A `none` row is not a licence to remove**, and this is the matrix's limit
rather than either column's failure. Alloy's `none` rows are `argument` rows, and
what an argument row establishes is that no shared-safety claim **names** the
artifact — but `FN-24.a` names no artifact at all, and it is the claim the
quarantine exists to make satisfiable. Quint's three are one `const`:
`ATOMIC_DISPOSAL`'s true branch replaces the quarantine rename and every step
after it with a single `SDisposeInPlace`, and of the thirty-four instance modules
in `finish-controls.qnt` exactly one sets it true. **No control removes the
quarantine while `EN-03` still holds, and none can, because in this protocol the
artifacts and the missing capability are the same parameter.**

## The rule this record exists to stop anyone re-deriving

**A counterfactual-capability control measures admissibility, never
availability.** The catalogue's own class table says a counterfactual *adds* a
capability "to ask whether a cheaper protocol is admissible", and Q1's and Q2's
pre-registered delete/replace criteria were nonetheless written entirely in terms
of such a control passing. Both criteria are therefore satisfiable only under a
capability the assumption table records as absent — `EN-03` and `EN-05` — so as
pre-registered neither question could have returned `delete/replace` against the
shipped world however green the run. That is a defect in the criteria and not in
the evidence, and it was invisible until both columns were green and read
together.

The consequence for a reader of the models: **a green counterfactual is evidence
that a claim set does not depend on a mechanism, and is not evidence that the
mechanism can go.** Necessity can come from an environment assumption plus a
claim that names nothing.

## The constraints every answer had to hold

These bound any future proposal too, and they are recorded here because the
scoping note that carried them is gone.

- **The interval is the whole problem.** Between removing `.grove/` and recording
  that removal, a later invocation would read a fresh grove. Nothing may
  reintroduce a window where that is observable — which is why the correlation
  ticket rather than the tree is the evidence
  ([`success-is-proved-by-the-ticket-not-the-tree`](success-is-proved-by-the-ticket-not-the-tree.md),
  `FN-03`, `FN-28`).
- **Never rewrite history to clear a blocked state.** An unresolvable outcome
  stays blocked and operator-recoverable, naming the artifact, the recorded and
  observed topology, and the two restorable exits (`FN-26`).
- **Three VCS shapes stay symmetric** — Git, native jj, colocated jj
  ([the VCS seam](../ARCHITECTURE.md#symmetric-vcs-rule), `EN-16`), and a model
  that collapses the lane passes every property check, which is why the collapse
  is exercised rather than assumed.
- **The HITL boundary is not machinery.** `finish-commit` cannot attest that a
  human spoke through an opaque command; it is the deterministic last-moment
  guard, not a substitute for the confirmation contract (`FN-01`, and `EN-15`'s
  counterfactual, under which no obligation strengthens).

## The alternatives, and why each is rejected

| | what it removes | why rejected |
|---|---|---|
| **disposal in place** (Q1's candidate, `relax_EN_03`) | the quarantine, the marker, the replace transition — ~3,000 lines and all 31 `unsafe` blocks | needs atomic recursive deletion, which `EN-03` says does not exist. Admissible, unavailable |
| **two dispositions** (Q2's candidate, `relax_EN_05`) | `Recovery pending` and the recovery surface it generates | needs the version-control commit inside the filesystem transaction, which `EN-05` says is impossible |
| **fold the replacement into a branching write** | `marker_replacement.rs`, 960 lines | answers Q3 by construction rather than by reachability, which is a false-confidence incident rather than a finding. Both families reach the source state |
| **remove one of the three `none` rows on its own** | one of the quarantine, the marker, the replacement | not a mutation of the incumbent but a fourth candidate protocol — in-place *non-atomic* disposal — with the quarantine's resumption problem *and* an observable partial task root. Strictly worse than both candidates already checked |
| **keep the questions open** | nothing | the deciding evidence exists and is green in both families; leaving them open would carry a redesign into the documentation and implementation phases as an unresolved semantic question |

Deleting a fail-closed step is not a simplification when it converts a refusal
into a silent wrong state — the rule
[`task-tree-transactions-fail-closed`](task-tree-transactions-fail-closed.md)
records, whose worked example is withdrawing a legacy migration reader while
keeping its format classification, so that a tree classifying as empty gets a
format witness written over live work. Every candidate above was measured against
that rule and not against line count: **"the model is smaller" is not evidence.**

## What would reopen this

A filesystem primitive that makes recursive removal atomic with respect to
namespace visibility, or a version-control lane whose commit can be made atomic
with a filesystem transaction. Either would move an assumption out of the
environment table, and the counterfactual already run for it becomes evidence
about an available protocol rather than an admissible one. Nothing short of that
reopens Q1 or Q2; `finish-verdicts-k65` is the session that decided them, and the
evidence is [`docs/formalism-findings.md`](../formalism-findings.md) entries 026 –
048 and the three model `README.md`s.
