# formal-synthesis-k16 — brief


## Goal

Convert the checked models and replay evidence into binding design decisions and an executable hand-off to documentation.



## Context

This is the formal-phase gate. Its conclusions are local to this experiment, not universal claims about Quint, Alloy, or formal methods. Durable evidence belongs in model READMEs and `docs/formalism-findings.md`, not only in this task file.

## Done when

- Both model families and the common runner are green, have non-zero witnesses/checks, and document every bound, assumption, omitted behaviour, and unresolved tool limitation.
- The shared claim catalogue and component/system model READMEs agree on stable semantics, error taxonomy, VCS refinements, filesystem responsibility, and model-to-crate ownership.
- Experiment 2 contains a bounded synthesis: which formalism caught what, what neither established, cost/counterfactual/verdict, useful combined workflow, and concrete changes to design/tests/docs.
- Every proposed finish simplification from `TODO.finish_process.md` is classified keep, delete/replace, or defer with a claim and replayed evidence. “Model is smaller” is not evidence.
- The ordinal lifecycle experiment has a keep/defer/reject decision. If kept, insert an `impl` leaf immediately before `extract-task-tree-k24` using `grove-llm leaf-insert extract-task-tree-k24 ordinal-root-lifecycle --kind impl`.
- For each model-earned finish simplification, insert one narrowly named `impl` leaf immediately before `collapse-application-k27`, preserving the intended execution order. Do not create a generic “simplify finish” bucket.
- Documentation tasks have no unresolved semantic questions and all durable formal artifacts are linked from their component owners.

## Decomposition

The order encodes one dependency and one freeze.

1. **`experiment-synthesis-k62`** — the whole-repository run, recorded, and
   Experiment 2's bounded synthesis with every pre-registered hypothesis and
   measure decided against its own falsifier. **First, because a measurement
   must be frozen before its subject moves**: every later child edits the
   catalogue and cascades commands into both families, and the experiment
   compares the two columns *as independently built*.
2. **`obligation-placement-k63`** — where an obligation lives when its subject spans
   two component scopes. Six recorded instances of the `TT-24` shape, and the
   answer is what *model-to-crate ownership* means for the crate boundary the
   root brief approves. **Gates the next child**, because several inherited
   dispositions are of the form "restate this as an `FN-` obligation".
3. **`catalogue-disposition-k64`** — every remaining inherited catalogue finding,
   decided and landed, with the model commands that keep both families' coverage
   green. Expect this one to decompose by scope; the split is its own to choose,
   because child 2 changes which scope owns what.
4. **`finish-verdicts-k65`** — `TODO.finish_process.md` Q1 – Q4 answered keep,
   delete/replace or defer against the evidence the catalogue pre-committed to,
   the ordinal root-lifecycle verdict contested rather than inherited, and the
   resulting `impl` leaves inserted into `03-implementation-k3`.
5. **`handoff-audit-k66`** — the documentation phase carries no unresolved semantic
   question, and every durable formal artifact is reachable from its component
   owner.

Children 2 – 5 were cut by child 1's session, which is why their bodies carry the
specific inherited items rather than a generic goal sentence.

**A sixth entry sits between 2 and 3, and it is a review step rather than a new
question.** `obligation-placement-k63` cut
`review-design obligation-placement-k67` and **inserted** it at child 3's slot
rather than appending it, because child 3 is chartered to edit the very artifact
under review and this brief says child 2 *gates* child 3 — a gate checked after
everything it gates is not a gate. The keys are unchanged: `k64`, `k65` and `k66`
are still children 4, 5 and 6. If the review finds nothing it creates nothing and
retires.

## Notes

Retire the formal subtree only after the model commands have been rerun from a clean checkout-equivalent state. Review chains should be added here only for decisions whose uncertainty or blast radius warrants an independent session.

## Decisions (running log)

**This leaf is bigger than its brief, and the measurement that says so is the
coupling between the catalogue and the runner.** `models/run.sh` reads its
obligation manifest **out of** `docs/specs/semantic-contract.md` rather than
transcribing it (`run.sh` header, obligation 3; the catalogue's own *Model paths
and the runner*). So a disposition that adds, removes or re-scopes an obligation
is not a documentation edit: it opens an empty `(family, obligation)` cell that
**both** families must fill with a command before any coverage-asserting run is
green again. Several inherited dispositions are of exactly that kind — a
contention member for the closed refusal set (`RGenContended`), `RRolledBack`
and `RConfigInvalid`, restating `TT-24.c`/`TT-24.d` as `FN-` obligations, and a
`PartialScaffold` state-table member for the shipped ambiguity refusal. The
cascade is model work in two families plus a re-run whose Alloy task-tree cell
alone costs 1 h 57 m wall.

The size, measured rather than impressionistic: **97 handoff sites across 11
files name `formal-synthesis-k16`** and hand it a disposition. Controlled in
both directions — a live sibling handle (`cross-model-replay-k15`) finds 14
sites in the same command, an invented handle (`formal-synthesis-k99`)
finds 0 — so the figure is neither a broken grep reading clean nor a
loose pattern matching everything. Against that, the leaf's own `Done when`
carries seven items, of which one (the whole-repository green run) is a ~3 h
background measurement and one (the catalogue disposition) is a pass over a
1,537-line spec that cascades into 30k lines of models.

Decomposed with `grove-llm leaf-decompose`. The children and their order are in
the brief's *Decomposition*.

**The first child is the experiment synthesis, and the ordering argument is that
a measurement must be frozen before its subject moves.** Experiment 2 compares
**the two columns as they were independently built**. Every disposition child
after this one edits the catalogue and cascades commands into both families,
which moves M5's checked-claim counts, M7's run costs and the coverage figures.
Measuring after that would report figures for a third set of models that no
independence protocol ever governed. So the synthesis runs first, on the
artifacts entry 048 read, and the dispositions run against a frozen record.
The whole-repository run was already in flight when this was settled, which is
the same argument from the other end: it is the measurement of the tree as the
formal phase built it.
