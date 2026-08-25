# entry-k39


## Goal

Start `crates/grove-finish/models/finish.als` with the transaction's entry
surface: the confirmation contract and the closed preflight precondition set,
`FN-01` and `FN-05` – `FN-08` (eight obligations), plus the `EN-02` mutation
that controls `FN-08`.


## Context

- `docs/specs/semantic-contract.md` §*Claims — finish and recovery*
  `FN-01`, `FN-05` – `FN-08` is this leaf's claim scope; §*Outcomes* fixes the
  closed refusal-reason set every guard failure must name.
- `models/run.sh` is finished and needs no work here. The measuring invocation
  is `models/run.sh --scope finish --family alloy --no-coverage`, and the
  README's run line carries `--no-coverage` until the whole `FN-` column closes.
- `crates/grove-task-tree/models/task-tree.als` is the house style. Its
  machinery is **not** importable — the runner treats each scope directory as
  its own family file set, and a `TT_` command here is a placement failure — so
  what carries over is the style, not the signatures.

Every obligation in this slice ends in a **refusal**: `FN-05` is the claim that
preflight mutates nothing, and its three obligations quantify over a set that
`FN-05.a` fixes as closed and exactly seven-membered. So this slice needs the
transaction's *entry* and none of its body — no witness species, no evacuation,
no repository mutation, no disposition, no quarantine. That is what makes it the
first child rather than a horizontal layer.


## Done when

- `crates/grove-finish/models/finish.als` exists and answers each of `FN-01.a`,
  `FN-01.b`, `FN-05.a`, `FN-05.b`, `FN-05.c`, `FN-06`, `FN-07` and `FN-08` with
  a `check` and its required `witness_` runs, all green.
- **`FN-05.a`'s seven preconditions are each reached as their own witness**, and
  the check establishes the set is closed — a model that can refuse for an
  eighth reason has found the catalogue's stated finding, and a model that
  cannot reach all seven has an unexercised guard rather than a closed set.
- `EN-02` runs as `run expect_unreachable_EN_02_<mnemonic>` with the assumption
  table's expected result: under a **single-device** scope `FN-08`'s witness — a
  layout that passes at lease acquisition and fails at the transaction's own
  operands — is unreachable, and `FN-08`'s property check stays green. It is an
  *exercise-removal*, so a green property is the expected result and not a
  survivor.
- `crates/grove-finish/models/README.md` records tool version, solver, bounds
  per command, the abstractions and deliberate omissions this file adopts, what
  a green run does not prove, the obligations claimed so far, the witness bound
  at which each first lands, and any declared gap in the shape the runner
  parses.
- One mutation per reported obligation, run before the green is believed, each
  with evidence that it actually fires.
- `models/run.sh --scope finish --family alloy --no-coverage` exits 0 and
  reports exactly the expected empty cells.
- Experiment 2 entry 031 is appended with the six required fields plus the
  pre-registration's four additions.
- The next leaf (`witness`, `FN-09` – `FN-13`) is cut as a sibling, its body
  carrying the machinery question this session's file actually leaves open.


## Notes

**Two decisions this leaf owns for the whole subtree, and both are the first
file's to make.**

1. **How coarse the tree abstraction is.** No `FN-` claim quantifies over
   names, positions, keys or slugs, so a finish model that rebuilds the
   task-tree model's `Filename` grammar has paid for machinery no claim reads.
   The candidate is entries as opaque objects with a type and a digest, and the
   root as the small state enum §*States* already fixes. Record the choice and
   what it costs: `FN-05.a`'s *no live finish leaf, or live ordinary work
   present* needs leaf liveness and the finish/ordinary distinction, and nothing
   finer.
2. **Whether the lane is a signature parameter from the first command or
   arrives with `commit`.** Nothing in this slice differs by lane — none of the
   seven preconditions is lane-specific — but `EN-16`'s control is a *collapse*
   to one lane, and a parameter retrofitted later is a parameter the earlier
   commands were never checked under. Deciding it here is cheap; deciding it in
   `commit` is a re-run of everything before it.

`FN-08` is the one precondition whose content is about the filesystem rather
than the tree: the same-device requirement is checked against **the
transaction's own rename operands** and is never satisfied by an earlier
lifecycle check. Its witness is a layout that passes at lease acquisition and
fails here, so the model needs two check points and two device readings — which
is exactly the dimension `EN-02` removes.

`FN-01.b` is easy to make vacuous. The deterministic guards *are* separate from
confirmation, so the two witnesses must be **distinct states** — one transaction
never entered for want of confirmation, one refused for want of the guard — and
a model where the guard failure implies the confirmation failure has collapsed
them. The catalogue says "distinct from the previous" for exactly this reason.

The cost model in the node brief above applies from the first command: prefer a
static scope switch to a reachable transition, pin a switch that deletes state
and leave free a switch that admits an action, and measure the file's **widest**
command as well as its tightest.


## Decisions (running log)

**`finish-k8` was decomposed before this leaf ran.** Sixty-one obligations, a
lane parameter the task-tree scope did not have, and Q4's removal matrix is not
one focused session — `task-tree-k7` needed five for forty-three obligations and
no lane. The cut is by the machinery each claim group needs, and three claims sit
away from their catalogue section because a claim belongs with the machinery its
*witness* needs. The rationale, the five children and the two inherited
`TT-24` gaps are in the node's `BRIEF.md`.

**The tree abstraction is coarse, and the finish model shares nothing with the
task-tree model.** No `FN-` claim quantifies over names, positions, keys or
slugs, so an entry here is an opaque object with a type and a role. What the
seven preconditions actually read is leaf liveness, the finish/ordinary
distinction, an undigestible entry type and a tracked/untracked split — nothing
finer. Rebuilding `task-tree.als`'s filename grammar would have been machinery
no claim in this scope reads, and the runner keeps the two scopes in separate
directories for the same reason. Recorded in the family `README.md`.

**The lane is a signature parameter from the first command.** Nothing in this
slice differs by lane, but `EN-16`'s control is a *collapse* to one lane, and a
parameter retrofitted in a later slice is a parameter every earlier command was
never checked under. It earns its place immediately anyway: an absent lane **is**
an unsupported layout, which is how `FN-05.a`'s third member gets a state to fail
in at all.

**The lease gate is a recorded verdict, not a transition.** `SY-02` is the
lifecycle scope's claim and this file consumes it: a verdict cannot have been
recorded by a gate that did not pass. Only the part `FN-08` is about is
modelled — that the verdict never licenses the transaction's own operands. It
saves a transition, which the cost model says is the expensive dimension.

**`World.lane` is `var` and the devices are not.** `SY-03` says a preflight is
never a licence and each gate revalidates against its own operands, so the layout
must be able to *change* between the lease gate and the transaction's. A device
reading that changed would say the same thing twice; a lane that changes says it
once, cheaply.

**A material finding, and it is a counting mismatch in the catalogue.**
`FN-05.a` requires *each of the seven, reached*, and the closed refusal-reason
set can distinguish only six of them: the first member produces no refusal at all
(the transaction is never entered), and the third and fourth are the same reason
at two gates, which is what `SY-03` makes them. Six reasons cannot witness seven
members, and a family reporting six and calling the set covered has silently lost
one — which the runner cannot see, because coverage is per obligation and
`FN-05.a` is one. The catalogue now says the seven are not distinguishable by
outcome and that a family answering the obligation needs an observable of its
own. This model's is `Sys.why`, declared as an abstraction.

**The refusal-reason mapping this file chose is recorded, because the catalogue
does not state one.** Five of the six refusing members map to distinct closed
reasons; *layout unsupported* and *quarantine target unreachable* share
`LayoutUnsupported`. Whether the shipped diagnostic should distinguish the two
gates is `formal-synthesis-k16`'s and is not settled here.

**Every check runs at four states, and the bound is derived rather than chosen.**
The file was first written with every check at the sibling scope's minimum of
three, and the whole suite ran green. Measuring where each witness *first* lands
showed four of fourteen landing at four — including
`witness_FN_05a_p3_layout_unsupported`, which needs an intervening environment
action. So `FN-05.a` was green over a member it had never once reached. The rule
adopted, and passed to the siblings: **a check runs at a bound at least as large
as the widest first-landing bound among the witnesses of the obligation it
answers.**

**Three of the nine mutations reported as survivors and none was a fact about a
check.** Two added a constraint *underneath* a frame condition, making the branch
unsatisfiable — which reports exactly as a survivor does; the fix is to remove
the frame, not contradict it. One was the identity (`Op.confirmed' = Confirmation`
under a guard already requiring `some Op.confirmed`). Chasing the third found
that **no command in the file exercised the `Confirm` transition at all**, so
`FN-01.a`'s second conjunct was being checked over a transition the file never
demonstrated. `witness_FN_01b` now requires it. Every mutation in the matrix
carries evidence that it fires.

**`FN-05.b` and `FN-05.c` are declared thin rather than left to look strong.**
In the entry surface they are carried entirely by the frame conditions on
`doPreflight`, because no step here mutates anything; they separate two reachable
behaviours only once evacuation exists. The family `README.md` says so under
*what a green run does not prove*, and `witness-k40` is where they stop being
statements about the frame.
