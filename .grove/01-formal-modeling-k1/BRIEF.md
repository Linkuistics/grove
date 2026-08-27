# formal-modeling-k1 — brief


## Goal

Use Quint and Alloy 6 to make the modular design precise before documentation or Rust implementation. Model the same claims independently enough to expose different blind spots, then reconcile them through explicit replay rather than treating agreement as proof.



## Context

Required scopes are Grove task-tree semantics, the finish/recovery protocol described by `TODO.finish_process.md`, and the lifecycle joining session completion, tree exhaustion, finish, interruption, and recovery. Alloy 6 must use temporal/behavioural modelling, not merely static relational snapshots. Quint must expose executable guarded actions and refusal outcomes.

Component models should live beside the crate they constrain (`crates/grove-task-tree/models/`, `crates/grove-finish/models/`, and the existing ordinal component); cross-component lifecycle models live in `models/system/`. A single repository runner must execute every model and fail if a tool silently did no work.

## Done when

- A shared claim catalogue defines state, actions, observations, environment assumptions, stable states, transient states, refusal outcomes, and recovery obligations without encoding either tool's syntax.
- Independently constructed Alloy 6 and Quint models cover all three scopes, include satisfiable witnesses, and check their documented safety/liveness properties with reproducible commands.
- Each model documents tool version, bounds or trace limits, solver/backend, fairness assumptions, abstractions, deliberately omitted details, and what a green run does not prove.
- Every unique counterexample or design finding is replayed in the other formalism or recorded as inexpressible with a reason.
- `docs/formalism-findings.md` contains a pre-registered, evidence-linked comparison and a bounded synthesis of when each formalism helped here.
- Formal conclusions settle model placement, the crate-facing semantic contracts, the fate of every proposed finish simplification, and whether ordinal root lifecycle deserves an implementation task.
- No material semantic question is deferred to the documentation or implementation phase.

## Notes

Do not alter product behaviour in this phase. Model/test harness work and factual experiment documentation are in scope; architecture/user documentation and Rust refactoring are not.

Required experiment fields for every material observation: Situation, Formalism, Caught or missed, Cost, Counterfactual, and Verdict. Pre-register comparison of Alloy 6 temporal behaviour with Quint actions; unique versus overlapping findings; counterexample quality; test derivation; component-local versus system-level placement; synchronization burden; state-space/tooling cost; and failure modes that could create false confidence. Both model families remain required regardless of the comparison result.


## The Alloy column is closed, and what that means for each remaining child

`alloy-models-k6` closed with all three scopes green and coverage asserted:
`crates/grove-task-tree/models/` (`TT-01` – `TT-25`, 103 commands),
`crates/grove-finish/models/` (`FN-01` – `FN-31`, 180 commands, 61 of 61 cells)
and `models/system/` (`SY-01` – `SY-14`, 73 commands, 25 of 25 cells).
`models/run.sh --scope <scope> --family alloy` runs each with **no**
`--no-coverage` anywhere. The durable record is the three family `README.md`s
and `docs/formalism-findings.md` entries 026 – 043.

**THE INDEPENDENCE BARRIER IS STILL UP AND THIS SECTION IS DELIBERATELY THIN
BECAUSE OF IT.** The protocol is *neither model reads the other before **both**
are green* — Alloy being green lifts nothing, because the Quint column does not
exist yet. So:

- **`quint-models-k10` and its three children MUST NOT open any `.als` file, any
  model-directory `README.md`, or entries 026 – 043.** They are written from
  `docs/specs/semantic-contract.md` alone, as the Alloy column was. A finding
  carried across during construction is recorded as such and **excluded from the
  unique/overlap count**, which is most of what the experiment is for.
- **`cross-model-replay-k15` is where the barrier comes down**, by design: it is
  the deliberate later step the protocol reserves, and it reads both.
- **`formal-synthesis-k16` inherits the whole Alloy record** and needs no summary
  here; what it must not be handed is a *pre-digested* one, since several of the
  comparisons it owes are about what each family reached on its own.
- **`matrix-reader-k50` is tool-shaped rather than finding-shaped** and may read
  the runner and each family's Q4 removal matrix without touching the barrier.

**Two things are tool-neutral and therefore safe to carry, and both are about
the catalogue rather than about Alloy.** `docs/specs/semantic-contract.md` is the
shared subject of both families, so no session may edit it under the barrier —
which means the Alloy column recorded, rather than fixed, every catalogue
finding it made. **There are now several, they are named in the entries, and
`formal-synthesis-k16` owns the disposition of all of them.** The second: the
runner's coverage unit is the pair `(family, obligation)`, so a Quint cell an
Alloy command filled is still an empty cell until a Quint command fills it too.
That is the truth about the phase rather than a defect in the runner.

## The Quint column is closed too, and the barrier may now come down

`quint-models-k10` retired with all three scopes green, each reviewed as an
INSTRUMENT and each repaired against what that review found:

| scope | model | run | entry |
|---|---|---|---|
| task tree | `crates/grove-task-tree/models/task-tree.qnt` | `models/run.sh --scope task-tree --family quint` | 044 |
| finish/recovery | `crates/grove-finish/models/finish.qnt` | `models/run.sh --scope finish --family quint` | 045 |
| system lifecycle | `models/system/lifecycle.qnt` | `models/run.sh --scope lifecycle --family quint` — **86 commands, 25 of 25 cells, exit 0** | 046 |

**Three consequences for the remaining children.**

- **`cross-model-replay-k15` may now read both families.** The protocol was
  *neither model reads the other before both are green*, and both are. This is
  the deliberate later step the protocol reserved; nothing before it lifted.
- **`formal-synthesis-k16` inherits three entries whose claims have already
  been contested once**, and the contest is itself a measurement. Each Quint
  column was attacked by a `review-prototype` leaf whose subject was the
  instrument rather than the findings — the search shape, the narrowings, the
  controls — and each review found real defects in a suite that was green and
  coverage-asserted. Entry 046 carries the sharpest ledger: four of its six
  findings were invisible to any run of the suite itself, because a suite cannot
  ask whether a mutation kills anything besides its target, whether a
  measurement measures its subject, or whether a quoted number can be re-run.
  **The unit of that review — the pair (claim, the control that would falsify
  it) — is worth carrying into the synthesis as a general obligation.**
- **A whole-repository run is no longer red by construction**, since no Quint
  cell is empty for want of a column. Whether it is green is a separate
  question and no session here has measured it end to end; the per-scope runs
  above are what has been.

## The barrier came down, and what the replay changed for `formal-synthesis-k16`

`cross-model-replay-k15` read both families. Entry 048 is the record; four
things in it change what the synthesis is reading rather than adding to it.

- **The two columns were not written from the same catalogue.** Four Alloy
  sessions *edited* `docs/specs/semantic-contract.md` before the first Quint
  model existed, so the section above — "the Alloy column recorded, rather than
  fixed, every catalogue finding it made" — **is wrong**, and demonstrably so at
  `names-k33`, `guards-k37`, `ownership-k38` and `entry-k39`. Entry 026's and
  031's findings were fixed into the text the Quint column then read. **Their
  `alloy-only` tags are true and uninformative**, and every unique/overlap count
  in this experiment is asymmetric by construction in the Alloy direction. The
  pre-registration gained a fourth independence rule for it.
- **Six findings tagged `quint-only (pending replay)` are `both`.** They are
  tabulated in entry 048 finding 4. Two of the six were resolved in **opposite
  directions** by the two columns — `FN-13`'s refuse-or-block, and whether
  `reap` is gated on the root — and an opposite resolution is stronger evidence
  that the catalogue is underdetermined than either column's own account of it.
  Those two are the disposition list's sharpest items.
- **One Alloy check was a theorem of its own model.** `TT-20`'s fourth conjunct
  was green with no protocol premise at all, which is why the Alloy column never
  saw entry 044's counterexample. `task-tree.als` is corrected, the counterexample
  is retained and reachable, and the narrowing is declared — the same narrowing
  the Quint column declared, reached independently.
- **The product settles two of the questions the models left open.** `reap`
  reads no tree in the shipped driver, so entry 046's finding 4 has no product
  defect behind it. And entry 044's `PartialScaffold` finding *is* shipped
  behaviour, in one narrow window — after the charter, before the leaf — while
  outside that window the shipped code refuses with an *ambiguous partial root
  scaffold* diagnostic the catalogue's state table has no member for.

`quint-statement-shape-k61` was cut from the replay and sits ahead of the
synthesis so the Quint column's evidence is repaired before it is read.
