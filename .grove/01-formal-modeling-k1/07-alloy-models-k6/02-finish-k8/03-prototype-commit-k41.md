# commit-k41


## Goal

Extend `crates/grove-finish/models/finish.als` to the repository: the scoped
commit, the attempt identity's correlation ticket, the three dispositions
classified from evidence, the rollback licence and its exactness, and forward
recovery — `FN-03`, `FN-04`, `FN-14` – `FN-18`, twelve obligations. Owns `EN-09`.


## Context

`witness-k40` left the file green for `FN-01`, `FN-05` – `FN-13` and took the
finish scope's empty alloy cells to forty-five.
`crates/grove-finish/models/README.md` carries the bounds, the abstractions, the
mutation matrix, the four retained counterexamples and the six *what a green run
does not prove* caveats. **Read it before writing a command.** Two of its
sections are specifically this leaf's inheritance and are named below.

What exists that you can build on, and what each of it is worth:

- **The whole transaction, up to the attempt.** Six body steps — `WPrepare`,
  `WManifest`, `WReady`, `WPublish`, `WEvacuate`, `CommitAttempt` — as a phase
  machine, with `crash` between any two and `discard` as the unpublished
  witness's recovery. `doCommitAttempt` **records that a commit was attempted and
  mutates nothing**: no commit, no ticket, no anchor comparison, no disposition.
  That is the seam this leaf opens, and it is deliberately the whole of it.
- **The attempt identity and the repository anchor already exist as opaque
  pins.** `Txn.attempt` is drawn at `TxnOpen`, `Txn.anchor` records `Repo.rev`
  there, and `Man.mAttempt` / `Man.mAnchor` record both because `FN-12.a`
  requires it. **Nothing reads them back.** This leaf is where they stop being
  pins and become the operands of a classification, which is `FN-15`'s subject
  and the reason the `finish-k8` brief put `FN-03` and `FN-04` here rather than
  in `entry`.
- **The deletion fingerprint is a derived value** (`fun fingerprint`), not a
  field, and the manifest records it at write time. `FN-14`'s *exactly the
  expected deletions at their original paths and no unrelated change* is stated
  over it.
- **The lane is in the signature and still unused.** No obligation through
  `FN-13` distinguishes the three. `FN-16` – `FN-18` are the first that plausibly
  do — the rollback licence is stated differently for Git `HEAD` and for jj's
  working-copy change identity, and colocated jj has an index image the other two
  lanes do not. If a lane-specific claim appears here it is a **finding**, and
  `EN-16`'s collapse control that would make a lane-blind model visible is still
  `exits`'.
- **The mutation matrix has seventeen rows, all KILLED, each with named evidence
  that it fires.** Three retained rules govern how to write the next twelve; the
  shortest form is *a mutation the model cannot execute is not a control*.

What this leaf has to build, and none of it exists:

- **The repository as something that changes.** `Repo.rev` and `Repo.tracked`
  exist but no Grove action touches them: `doTopologyChange` is the world's, and
  every transaction step frames the repository. A commit is the first Grove
  mutation of it.
- **`EN-05` is the constraint that shapes this.** *No filesystem transaction can
  include a version-control commit.* The commit is therefore outside the
  filesystem transaction, and the interval between the evacuation and the
  recorded result is the whole problem `TODO.finish_process.md` is about. It is
  also Q2's counterfactual, and `EN-05`'s premise-break control is **Quint's**,
  not this family's — check the assumption table before writing one.
- **The correlation ticket**: the commit message naming the handle and the
  attempt identity, and the durable record that a given attempt completed. It is
  what a rootless retry accepts, and the glossary's *Avoid* is load-bearing —
  the quarantine is not a substitute and proves nothing.
- **Three dispositions classified from evidence, not exit status** (`FN-15`),
  and `FN-15.d`'s obligation is answered by *the instrument the catalogue names*:
  a witness, **or** a bounded-unreachability `check` over the full scope with its
  bound and result recorded per lane. An unlanded witness satisfies neither and is
  a `defer`; recording it as one is legitimate and is what
  `formal-synthesis-k16` reads.
- **`EN-09`** — a result arriving after the classification — is this leaf's, it
  controls `FN-15.a`, and it is run against the named witness set rather than the
  whole file.


## Done when

- Every obligation of `FN-03`, `FN-04`, `FN-14` – `FN-18` is answered by a
  `check` and its required `witness_` runs, all green under
  `models/run.sh --scope finish --family alloy --no-coverage`, with the finish
  scope's empty-cell count down to thirty-three.
- `EN-09` is present as its own named command with the expected result the
  assumption table states, run against `FN-15.a`'s witnesses rather than the
  whole file.
- `FN-15.d` is answered by a witness or by a bounded-unreachability check with
  its bound and result recorded **per lane**, or recorded as a `defer` with the
  reason.
- **Every check runs at a bound at least as large as the widest first-landing
  bound among its obligation's witnesses**, measured rather than assumed, and the
  measurement is a **median of three** — see *Notes*.
- One mutation per obligation, each with **evidence that it fires** — one
  existing witness re-run under it, still landing.
- The family `README.md` gains the new bounds, the new abstractions, the
  witness-bound table's new rows, the mutation matrix's new rows, and any
  retained counterexample. If a claim holds on only some lanes, that is a finding
  and it goes in the README rather than into a lane-specific claim.
- Material observations are appended to Experiment 2 as entry 033.
- The next leaf (`handoff`, `FN-19` – `FN-22`, `FN-31`) is cut as a sibling, its
  body carrying what this file actually leaves open. The `finish-k8` brief
  expects `handoff` to decompose again.


## Notes

**What `witness-k40` actually left open, as against what it deferred.**

- **`FN-13`'s refusal has no reason in the catalogue's closed set**, and the
  model reports it under `WitnessPending` with the model-only `Sys.why`
  distinguishing it. The ADR says a tracked witness *blocks* (Recovery pending);
  the catalogue says *refused*. The model followed the catalogue. **Do not
  resolve this inline** — it is `formal-synthesis-k16`'s, and a `Blocked` outcome
  is `FN-25`'s machinery, which is `exits`'. If the commit slice needs `Blocked`
  for `FN-15.c`'s `Indeterminate`, say which of the two documents you are
  following and why, rather than quietly adopting one.
- **`evacuationComplete` requires the task root still present; `gateEvacuated`
  does not.** The check passes because the preflight's identity gate guarantees
  it upstream and nothing in the witness slice removes a root. The two sides are
  written apart on purpose: **the day a step removes the root — the quarantine
  rename, `handoff`'s — that divergence becomes a counterexample.** It is not
  yours to close, but know it is armed.
- **The step list is a proposal, not a checked claim.** `bodySteps` is written as
  one named thing so `FN-24.b` can quantify over it. Every step this leaf adds
  belongs in it, and each should have **at most one persistent effect** that is a
  same-directory rename (`EN-01`) or is itself decomposed — because `exits` will
  check exactly that, and a step that fails it is cheaper to notice now.

**Three predictors and three measurement rules, all inherited, none restated in
the family README's own voice more than once.** The predictors:

1. an **interval** claim needs interval-many states;
2. the bound must hold the **machinery of the transitions** the obligation
   quantifies over;
3. a **shape** claim under a free initial state must be restated over the
   transition relation — and `fact TransactionsStartWhereAProcessStarts` is the
   volatile half of that, narrowing state 0's `Txn.phase` to `Fresh + Opened`.
   **Anything this leaf adds to the transaction's volatile state inherits that
   fact's reasoning**, and a new phase reachable only from the free initial state
   is the defect it was written against.

The measurement rules: whole-suite totals do not compare across sessions; one
sentinel is not enough, so measure the widest command as well as the tightest;
and — new from `witness-k40` — **a single command's cost is bimodal within one
sitting** (`witness_FN_11` measured 2.0 s, 10.1 s, 2.0 s on three consecutive
runs of the same bytes). Take a median of three or report nothing.

**On cost, concretely.** Eight reachable transitions cost the *inherited*
commands ~+55% and took the suite from 23 commands / 23 s to 40 commands /
2 m 13 s. This leaf's twelve obligations will want at least a commit action, a
classification and a rollback; budget by counting transitions and prefer, in
order, a static scope switch, a narrowed antecedent, a smaller bound. **Pin a
switch that deletes state, leave free a switch that admits an action.**

**A frame-condition trap in a new place, worth one line of vigilance.**
`witness-k40` widened the shared "the transaction is gone" frame to clear the
lease verdict, which `doPreflight` frames explicitly — the preflight's whole
refusal branch became unsatisfiable and **eight inherited witnesses reported *no
instance* while every check stayed green.** A shared frame predicate that a later
slice widens is a change to every transition that ever used it, and the
*witnesses* are what report it. Run the full file, not the new commands.

Do not read the Quint side of Experiment 2. The independence protocol holds until
both families are green.
