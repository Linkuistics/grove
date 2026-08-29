# Formalism findings

A running log of what using formal models to drive design and implementation
actually costs and actually buys. It is the raw material for a `linkuistics`
skill on choosing and using a formalism; it is **not** that skill, and it should
not be written as if it were.

## The models this log is about no longer exist

Every entry below was written while the model it reports on was on disk, and
many name a `.qnt`, a `.als` or a model-directory `README.md` by path. Those
files were deleted with the rest of the campaign's apparatus
(`delete-formal-models-k29`), together with the runner `models/run.sh` and the
specification they were checked against, `docs/specs/semantic-contract.md`.
What was kept is the evidence — this log, `candidate-lessons.md`,
`review-yield.md`, `loop-record.md`, `preservation-baseline.md`,
`driving-a-checkable-loop.md` and `results-of-formal-methods-trial.md` — because
the `linkuistics` skills the campaign produced cite them for their authority,
and a rule whose evidence has been deleted is a rule nothing can falsify. The rule and its
rejected alternative are recorded in
[`docs/adr/evidence-outlives-the-instrument.md`](adr/evidence-outlives-the-instrument.md).

So the paths below are **descriptions of where a finding was made**, not
navigable references, and they are written as plain names rather than links for
exactly that reason. Nothing in an entry was rewritten to hide the deletion: an
entry that says a model refused something still says it, and it is now a
statement about what happened rather than an invitation to go and re-run it.
The one directory the campaign's models delegated to, `docs/ordinal-fs-tree/models/`,
survives with its own runners, because the `ordinal-fs-tree` crate is live and
its tests still cite those models as standing authority.

The deletion took two leaves and the second is worth naming, because this
document's `FN-` entries — the largest family here — were still openable after
the first. `delete-formal-models-k29` took the `SY-` and `TT-` columns, the
runner, the controls and the catalogue; `delete-finish-models-k30` took the
`FN-` column, `crates/grove-finish/models/`, whose `README.md` several entries
below cite for a tabulated bound or a replay line. Those citations are now
descriptions like every other path here.

## Why this is a log and not a document

The two things this workstream is producing have opposite production schedules.

The **library** converges: decisions get settled, stop moving, and are written
once and edited in place. The **method** accumulates: you cannot know which
formalism suits which situation until you have used several and been burned by
at least one, so its artifact can only be written at the end, from evidence
gathered throughout.

Collapsing those two into one activity fails in both directions. Write the skill
only at the end, and the evidence is gone — a session reconstructing an earlier
session's modelling lesson from memory is exactly the failure this log exists to
prevent. Write it continuously, and you generalise from a single data point,
encoding the accidents of one design as principles.

So: **accumulate here, distil later.** Every session that reaches for a
formalism appends an entry before it retires. A later leaf turns the whole log
into a skill.

## What makes an entry useful

An entry saying "Quint was helpful here" is worth nothing. Six fields, and the
last two are the ones that matter:

1. **Situation** — what was being decided or specified, in one or two sentences.
2. **Formalism** — which one, and *why that one was reached for*.
3. **Caught / missed** — concretely what it found, and what it demonstrably
   failed to find. The misses are as informative as the hits and are the first
   thing a later reader will want.
4. **Cost** — wall-clock, tooling friction, what had to be learned first.
5. **Counterfactual** — what would have caught this earlier or more cheaply.
   *This is the routing evidence.* "Alloy would have found this in one line" is
   what later becomes "reach for Alloy when…".
6. **Verdict** — one line: reach for this again, for this kind of question?

Record misses and dead ends with the same care as successes. A log that only
contains wins produces a skill that only knows how to succeed.

## The hypotheses under test

Stated up front so they can be judged rather than quietly confirmed.

**H1–H3 and entries 001–025 are Experiment 1** — the `ordinal-fs-tree`
extraction, closed by the distillation at the foot of this file. A second
experiment is pre-registered below under *Experiment 2*, with its own hypotheses
numbered from H4; nothing here is restated or revised by it. When quoting a
hypothesis, say which experiment it belongs to.

- **H1 — the split is real.** Structural questions ("is this shape coherent?")
  and behavioural ones ("does this operation preserve it?") want different
  tools, and a practitioner can tell which they are holding *before* choosing.
- **H2 — the model leads profitably.** Writing the specification before the
  implementation produces better code than writing it after, and the
  disagreements between the two are worth more than the time they cost.
- **H3 — a specification can drive implementation.** An LLM given a checked
  formal model produces a more faithful implementation than one given prose of
  equivalent length. **Tested deliberately at entry 017 and not supported**: a
  prose-only arm violated no model claim. What the probe supports instead is
  narrower — a model pays *upstream*, in the prose it corrects, and an
  implementation reading already-corrected prose gets most of the benefit. Read
  017's confounds before quoting either half.

---

## Entries

### 001 — Prose review, as a baseline

**Situation.** A 365-line architecture document for an ordered-tree-on-disk
library — its data model, its single trait, its operation set, and eight stated
invariants. About to be reviewed by eye and signed off.

**Formalism.** None. This entry is the control: the reviewer asked, of each
operation in turn, *"could I write this as a state-machine action?"* — the
question a model forces — without writing anything.

**Caught.** Three defects, in roughly five minutes.

- *A blocking hole.* The trait could not name a node's distinguished child. Name
  construction was `compose(ordinal, key, parts)`, and a distinguished child has
  neither an ordinal nor a key by definition — so the promotion operation, whose
  whole job is to move a leaf's content into one, had no way to name its own
  output. The prose sentence describing it ("the leaf's content moves verbatim
  into the distinguished child") is perfectly good English and describes an
  impossible operation.
- *A stated invariant that was false.* "Ordinal density: within one node the
  ordinals are exactly `1..n`." Every operation *preserves* density, but nothing
  *establishes* it — allocation is `max + 1`, so a level that already contains a
  gap keeps it forever, and the tree is explicitly meant to be edited by hand.
  The real invariant is distinctness.
- *A missing precondition.* The attribute-rewrite operation took new parts, and
  species was derived from parts, so nothing prevented rewriting a leaf into a
  node — renaming a file into a directory.

**Missed.** Unknown, which is the point of recording this as a baseline. The
entries that follow are what a checked model adds *on top of* asking the
question.

**Cost.** Five minutes, no tooling.

**Counterfactual.** All three would have surfaced while writing the model, and
sooner: the first at the moment the promotion action could not be written at
all, the second at the moment `init` had to say whether the initial state is an
empty tree or an arbitrary well-formed one — a question prose never has to
answer and a specification cannot avoid.

**Verdict.** The *question* is doing much of the work, and it is free. Any
design review should ask it whether or not a model follows. What remains to be
measured is how much the checked model adds beyond it — which is the whole
experiment, and entry 001 is the line it is measured against.


---

### 002 — Alloy, on the structure of `ordinal-fs-tree`

**Situation.** The same architecture document as entry 001, after that entry's
three defects were fixed: a data model, one trait, an operation set and eight
stated invariants. The question was structural — is the shape coherent, does it
admit anything nonsensical, and can it represent everything the operations need.

**Formalism.** Alloy 6.2. Reached for because the questions were about *shapes*
rather than *sequences*: what a well-formed tree is, whether a claimed
isomorphism is one, whether every entry the library must name is reachable from
the trait's constructors. Alloy answers those with counterexample structures,
which is the shape of the answer wanted.

The model is written assume-guarantee rather than with global facts. Nothing the
document merely claims is an Alloy `fact`; claims are named predicates and every
command declares which it assumes. That is what let one file hold both the
corrected design (`check`, must find nothing) and a reproducible witness for
each defect the correction closes (`run witness_…`, must find an instance). The
witnesses turned out to be the more valuable half: they are the reason a later
reader can see *why* a law is there.

**Caught.** Eight findings. Two were found by the model unprompted — a `check`
failed where it was expected to pass — and those are the ones that count as
evidence about the tool rather than about the modeller.

*Unprompted, from failing checks:*

- **The grammar was not required to be canonical.** The document stated
  `parse(format(n)) == n` and not `format(parse(f)) == f`. A check that a node
  has at most one distinguished child failed, because two filenames may parse to
  one name — at which point two files on disk *are* one entry, sharing a key and
  an ordinal. The invariant list rules this out nowhere.
- **The three `Option` accessors were never tied together.** `ordinal()`,
  `key()` and `parts()` each return `Option` and the document never said when
  each is `Some`. A name of species `Leaf` with no ordinal is admitted: an entry
  that cannot be ordered, shifted or promoted, and that no triple names.

*Prompted — the questions came from the leaf's task file, the answers from the
model:*

- **`promote` still cannot name its output.** This is entry 001's blocking
  defect in a second place, and `distinguished()` does not reach it. Species
  follows from parts; naming the promoted node needs parts implying `Node`; and
  `Parts` is opaque with bounds `Clone + Eq`, so every `Parts` value the library
  can reach already belongs to some entry in the tree. Fixed by taking the
  node's parts from the caller, as `append` and `rewrite` already do.
- **The model corrected the prose argument for that defect.** The first
  formalisation said no reachable name had species `Node`, and Alloy agreed —
  but the formalisation was wrong: the library *can* reach a sibling node's
  parts. Restated with the snapshot's whole parts set, the true claim is
  narrower and still fatal: the reachable names all carry *another entry's*
  label. Getting a witness to fire for the right reason is where the model
  earned its keep, because the overstated version was persuasive prose.
- **Species agreement was an invariant nothing could enforce.** `parse` took a
  filename only, so it could not see that a name declaring a node sat over a
  regular file; the library could see it and had no `Self::Err` with which to
  report it. Fixed by passing what the listing found to `parse`.
- **A distinguished child that is a directory hides a subtree.** Species
  agreement covered `Leaf` and `Node` and said nothing about the third species,
  and a walk does not descend into a distinguished child. Alloy produced a tree
  satisfying every stated invariant with an entire subtree invisible to every
  traversal — the failure mode the Foreign/Malformed distinction exists to
  prevent, arriving by a road the document had not looked down. This is the
  clearest case of the bias that was asked for: *make a bad tree that satisfies
  every stated invariant*.
- **`by_label` is not implementable behind this seam.** The trait names no label
  type, so the operation has nothing to take as an argument; `Parts` equality
  answers *same label* only where the label determines every attribute. Deleted:
  it is `find` with a caller predicate, which the document already prescribes
  for every other attribute question.
- **The invariant list was vague about all of its members, not one.** Density
  was documented as preserved-but-not-established; the model shows that is true
  of *every* invariant, since the library validates nothing it is handed. Saying
  it of one implied the others were stronger.

Two claims were checked and found to need no invariant at all: *at most one
distinguished child per node* follows from a single distinguished name plus
filesystem name uniqueness, and *the parse verdict is total and disjoint*
follows from `Verdict` being a sum type. Both are now documented as free.

**Missed.** More than was caught, and predictably so.

- **Everything with a before and an after.** Three of the eight invariants —
  subtree preservation under shift, identity preservation under promotion, plan
  atomicity — cannot be *stated* in this model, let alone checked. So can "no
  key is ever reissued", which reads like a structural claim and is not.
- **The `by_label` argument is not really Alloy's.** The reasoning is that the
  API surface has no label type — a fact about the trait's signature, which
  Alloy cannot see. The model contributed a premise (label does not determine
  parts) and the conclusion came from reading the trait. Recorded as a miss:
  a reader looking for what the tool found should not be handed this one.
- **Small scopes lie, quietly.** Several checks initially passed over a universe
  too small to hold one node, one leaf and one distinguished child at once. In
  assume-guarantee style a `check` of *laws imply property* also passes for free
  if the laws are unsatisfiable. Two satisfiability witnesses now guard against
  it, and adding them found that a `for 5` guard was itself under-populated.
  Nothing in the tool prompts for this; it has to be a habit.
- **Cost curve is a cliff, not a slope.** `for 6` on the same model hit
  *translation capacity exceeded* — the four-ary `compose` relation is the
  driver. Interesting instances and affordable scopes were nearly the same size,
  which is luck rather than method.

**Cost.** About three hours, of which perhaps twenty minutes was tooling: Alloy
6.2 needs Java 17+ and the machine had 16, so a JDK and the Alloy jar had to be
fetched before anything ran. Alloy has a usable headless CLI (`exec -c`), but it
reports a found instance by printing a table and a not-found by printing
nothing, with exit code 0 either way — so a runner script had to be written
before pass/fail could be read at all. That script is now `run-alloy.sh` and is
the reason the model is re-runnable by a later session. No prior Alloy on this
machine; the relational idiom itself cost little, the scope arithmetic cost more
than expected.

**Counterfactual.** Three different answers, which is the useful part.

- **The canonicity gap** would have been caught by a free checklist question,
  cheaper than any model: *of every claimed isomorphism, is both directions
  stated?* The document said "isomorphic" and stated half. Entry 001's question
  — could I write this as a state-machine action? — would not have found it,
  because it is not an action.
- **The `Option`-completeness gap would have been caught by the type system, for
  nothing.** A name modelled as `enum Name { Positioned { ordinal, key, parts },
  Distinguished }` makes the bad state unrepresentable. This is the sharpest
  routing lesson so far and it points *away* from the tool: before modelling a
  structural property, ask whether the target language already forbids it. Two
  of this document's structural claims were free that way, and modelling them
  taught nothing except that the compiler works.
- **The species-agreement and `promote` defects** would have been caught by an
  implementation attempt — at the moment the code had to construct an error it
  had no constructor for, and at the moment `promote` had to name its output.
  Certainly, but later, and after the trait had consumers. The model found them
  before any code existed, which is the claim H2 makes and the first evidence
  for it.

**Verdict.** Reach for Alloy when the question is *can this shape represent what
I need, and does it admit what I do not want* — above all for reachability
questions, *is every thing I must name reachable from the constructors I have*,
which is where both of this design's blocking defects lived. Do not reach for it
for anything with two states in it, and check first whether the target language's
type system already guarantees the property; two of eight here did.

---

---

### 003 — Quint, on the operations of `ordinal-fs-tree`

**Situation.** The same architecture document, after entry 002's eight
structural findings were reconciled into it: five mutating operations, a plan
interpreter with a rollback, six stated refusals, and the three invariants
entry 002 recorded as ones Alloy could not *state* — subtree preservation under
shift, identity preservation under promotion, and plan atomicity.

**Formalism.** Quint 0.32.0. Reached for because every remaining question had a
*before* and an *after* in it. Quint answers those with counterexample traces,
which is the shape of the answer wanted, and its specs execute, so a property
could be run the moment it was written rather than at the end.

The file keeps entry 002's discipline in a different shape. Nothing the document
merely claims is built into the transition relation: claims are named `inv_…`
that must hold and `wit_…` that must be *reached*, and eight instances of one
parameterised module fix one question each. Two structural choices did the most
work:

- **The interpreter is a state machine, not a function.** A mutation is not
  atomic in the model — effects land one at a time, any one can fail — so every
  intermediate state is a state the invariants are evaluated at. Everything this
  entry records about interruption follows from that one decision.
- **Refusals are transitions, not disabled actions.** The tool's own idiom is to
  express a precondition as a guard, which makes a refusal simply *not happen*.
  That is the opposite of what totality testing needs: a refusal modelled as an
  impossibility can never be shown to be reachable *or* dead. Making every
  operation enabled for every argument, returning an outcome, is what turned
  "each refusal is a modelled outcome" into something with a pass/fail.

**Caught.** Eight findings. The first two are the ones that count as evidence
about the tool, because the model contradicted the brief that commissioned it.

*The model disagreed with the question it was asked:*

- **The shift order's stated reason is wrong, and its real reason is not in the
  document.** Both the document ("every destination is vacated before it is
  needed") and this leaf's own task file ("show that any other order collides")
  assert that highest-first prevents a collision. It does not, because there is
  nothing to collide: a name embeds a tree-unique key, so two siblings never want
  the same filename and *no* order collides. Lowest-first is refused only on a
  tree that already violates key uniqueness — two siblings sharing a key *and*
  its parts at adjacent ordinals, which `cp 01-foo-k5.md 02-foo-k5.md` produces.
  The reason that applies to every tree is the one nobody wrote down: under
  highest-first ordinals stay distinct at *every* intermediate state, so an
  interrupted insert leaves a level that is merely **gapped**, which the design
  admits — and run the other way it leaves a **duplicate ordinal**, which it does
  not. Given that the document already concedes process death is unrecoverable,
  that is what the ordering rule is actually buying.
- **Whether the destination check is sequential is a design decision the
  document had not made.** The first model checked every destination against the
  snapshot, which is the obvious reading of "a pure function of the snapshot".
  Under it, highest-first and lowest-first are refused in exactly the same cases
  — the ordering question is *vacuous*. Folding the plan through the snapshot
  instead, so the algebra meets each destination as the interpreter will, is what
  gives the rule anything to do. The document said neither, and the difference is
  observable.

*Unprompted, from failing invariants and unreached witnesses:*

- **`promote` cannot preserve the invariants it is between.** It must create the
  node before moving the leaf's content into it, and the node carries the leaf's
  own ordinal and key — so both are on disk at once: a transient duplicate key
  *and* duplicate ordinal, with no ordering that avoids it. The consequence
  reframes the whole invariant list: these hold of **quiescent** trees, not of
  every filesystem state, and the lock is what makes that safe.
- **A rollback that fails leaves a duplicate key, and the library created it.**
  The document promises rollback for reported errors and is silent on a rollback
  that itself fails, so the model made that a case rather than an assumption. On
  the promotion path the single undo is *remove the node just created*, so
  failing there strands the leaf and the node together. The document says a
  duplicate key "is a defect in the tree, not in the library, and the library's
  part is to not create one"; this is the one path by which it does. The damaged
  tree then strands later operations with an occupancy refusal — reached in 0.07%
  of traces, which is the rarest thing the model found and needed its own sample
  budget to find reliably.
- **"No key is ever reissued" is false as stated.** An invariant written the
  obvious way — no newly created object carries a key seen before — fails, and
  the trace shows why: `promote` creates a directory carrying the promoted leaf's
  existing key, deliberately. The claim is about **allocation, not creation**, and
  the document nowhere draws that line even though it draws it elsewhere ("the
  entry that was a leaf *is* the node").
- **`insert`'s refusal covers a case its rationale does not.** "Requires an
  existing occupant at the target ordinal" also refuses inserting into a *gap* in
  a hand-edited level, where "inserting past the last sibling is `append`'s job"
  plainly does not apply — and no operation fills the hole. A gapped ordinal can
  be occupied only by hand.
- **The occupancy refusal is dead for every tree the library builds.** Its
  witness fires in 0% of traces in the `pristine` and `hand_edited` instances and
  only on a duplicated key or a wrecked rollback. Worth keeping — the lock is
  *advisory*, so an uncooperative neighbour remains — but that is a different
  justification from the one the document implies, and without it written down
  the check reads as dead code to whoever next tidies up.
- **Nothing says how an operation names its target.** The model had to decide,
  chose the key (the document's own advice for durable references) plus a variant
  for the root, and the refusal list turns out to *depend* on that choice: under
  key designation "promote a distinguished child" is unreachable, because a
  distinguished child has no key.

**Missed.** Four, and the first is the one a later reader should be warned about.

- **Subtree preservation is not checked, it is arranged.** Modelling a rename as
  a name change on a stable entry id makes "a shifted node carries its subtree"
  true by construction. What the model actually checks is the honest half — that
  an insert's plan names no descendant. The other half is a property of
  `rename(2)`, below the abstraction boundary, and no amount of modelling above
  that boundary will reach it. A model can satisfy an invariant by construction
  and look exactly like a model that verified it.
- **The behavioural model checks its own plumbing.** Sibling-name uniqueness and
  "only nodes have children" are filesystem facts that Alloy could assert as
  `fact`s. In Quint they had to be *maintained* by the interpreter, so checking
  them tests my transition relation, not the design. That hazard is specific to
  executable state machines and has no analogue in Alloy.
- **Walk order.** Reachability is modelled; the depth-first, distinguished-first
  ordering is not, so `by_key`'s documented "first in walk order" tie-break on a
  duplicate-key tree is unchecked.
- **Unreachability is sampled, not proved.** `quint run` is random simulation, so
  "witnessed in 0 traces" is evidence and not a theorem. Every dead-branch finding
  above rests on a sample count. `quint verify` would settle it and was not run.

**Cost.** About one session. Tooling friction was near zero where entry 002's was
worst — `quint` was already installed and its evaluator fetched itself on first
run, against Alloy's JDK-and-jar hunt — but **a runner script was still needed,
for the same reason and in a different dress**: `quint run --witnesses` reports
"witnessed in 0 trace(s)" in prose and exits 0, exactly Alloy's
found-nothing-and-succeeded failure. Two tools, two idioms, one identical trap.
`run-quint.sh` is the answer and carries the per-instance claim table, which is
also the model's own account of what each instance shows.

**And then the same trap sprang a third time, on the runner that was built to
prevent it.** Re-running entry 002's Alloy suite as a final check produced seven
passing `check`s and thirteen failing witnesses — which reads as a model whose
laws became unsatisfiable. Nothing had changed. `run-alloy.sh` defaulted to
`java` on `PATH`, that JVM is 16, Alloy 6.2 needs 17, and a JVM that never
reaches Alloy prints nothing — which the runner cannot tell from *found no
instance*. So a dead tool reported as "every property holds and no witness
fires", the exact shape of a green suite plus a broken model.

The uniformity is what gave it away: **every** witness failing at once is a
signature no real defect produces, because the witnesses are the positive
control. That is the transferable technique and it is worth more than either
tool — a suite that contains only *must-hold* claims cannot detect that it did
not run, and one that also contains *must-be-reached* claims detects it
immediately. Both runners now find their own toolchain and abort on a
tool-did-not-run signature rather than recording it as a result.

Three instruments, three sessions, one failure: the model, the tool and the
runner each reported *found nothing* and *succeeded* with the same bytes.

Two time sinks, both self-inflicted and both worth naming because they are
generic:

- **A scope constant leaked into the design.** `MAX_ORD`, meant to bound
  *exploration*, was used inside the fold that builds a shift plan — so siblings
  above the bound were silently dropped and the model produced a counterexample I
  first read as a design defect. Entry 002's "small scopes lie, quietly" has a
  sharper sibling: a scope constant used inside a pure function of the design
  does not merely limit the search, it *changes the specification*.
- **A failure mode enabled everywhere starves everything else.** I/O failure was
  available at every step, and the simulator picks uniformly among enabled
  actions, so a three-effect plan completed about one time in eight and the
  interesting traces were never sampled. Putting failure behind a constant, and
  giving the rare instance six times the samples, is what made the suite both
  informative and non-flaky.

**Counterfactual.** Four different answers, and they do not all point at Quint.

- **The shift-order finding was half free.** *Of every uniqueness claim, ask what
  the name actually contains* — a checklist question costing nothing — shows
  immediately that a key-bearing name cannot collide with a sibling. The other
  half, what an interruption leaves, is not reachable by any amount of reading:
  it needs a model in which an operation can stop halfway.
- **`promote`'s transient duplicate would have surfaced in implementation**, at
  the moment the code had to create the directory before moving the file.
  Certainly, and later, and after the invariant list had been signed off.
- **The failed-rollback duplicate key has no cheaper route.** It needs two
  independent failures in one run. That is what a simulator does and what a
  reviewer does not.
- **The reissue restatement wanted entry 001's question asked against a
  *pair*.** "Could I write this as a state-machine action?" caught single
  operations; this needed "does this operation satisfy that invariant, read
  literally?" — still free, still prose, and it is the cheapest thing in this log.

**On H1 — the split held, but the line is in a different place.**

The planned boundary was *shape* versus *operation*. That is not where it fell,
and the honest description is sharper and more useful: the line is **how many
states the property mentions**. One state — structural, Alloy. Two or more —
before/after, throughout, ever — behavioural, Quint. That version is mechanical
and can be applied *before* choosing a tool, which is exactly what H1 claims a
practitioner can do; the shape/operation version cannot, and it mis-sorts the
case entry 002 already flagged, "no key is ever reissued", which reads structural
and quantifies over every past state.

Neither tool wanted the other's work. Alloy stayed inside its lane and said so.
Quint reached for nothing structural — but it had to **restate** the
well-formedness predicate in its own language to have anything to preserve, and
that restatement is a hand translation with no checker between the two files.
That is the concrete, previously unrecorded cost of running two formalisms on one
design, and it is a cost of the *split*, not of either tool.

**Verdict.** Reach for Quint when the property mentions more than one state, and
above all when the question is *what does an interruption leave*. Nothing else in
this log answers that — not a type system, not a structural model, and not
careful prose, which is at its most confident precisely there. Do not reach for
it for anything a single state settles: half of what it checked about a single
tree here was checking the model's own plumbing.

---

### 004 — Implementing against two checked models (`ordinal-fs-tree`'s seam)

**Situation.** The first implementation leaf of the library: the crate skeleton,
the name types, the `EntryName` trait, one reference domain implementing it, a
conformance kit for the trait's five obligations, and the test that keeps the
algebra out of `std::fs`. Two checked models and a reconciled architecture
document already existed; nothing about the seam was open. This is the first
entry about *consuming* a model rather than writing one.

**Formalism.** None written. Both suites were re-run first (green, ~2 minutes,
no friction), and then read as the specification. The instrument that actually
did work here was a third one this log has not been counting: **the target
language's type system.**

**Caught.** Four, and the first is the log paying for itself.

- **Entry 002's counterfactual changed the design one leaf later, exactly as
  written.** It said: *before modelling a structural property, ask whether the
  target language already forbids it* — and named the shape, `enum Name {
  Positioned { ordinal, key, parts }, Distinguished }`. Applied at the **seam**
  rather than only in a domain, it collapses the trait's three `Option`
  accessors into one `fn triple(&self) -> Option<Triple<'_, Parts>>`, and the
  obligation *a name is positioned or distinguished, never neither* stops being
  a thing any consumer can get wrong. The conformance kit checks four
  obligations, not five, and says which one it does not check and why. Cost: one
  trait method, one paragraph of `ARCHITECTURE.md`, one comment in
  `structure.als`. This is the second data point for the *already guaranteed*
  routing row, and it is a stronger one than the first: 002 observed the
  saving, 004 collected it.
- **The witnesses were the test suite; the checks were not.** Four of
  `structure.als`'s `witness_…` commands became executable Rust — a domain whose
  `compose` ignores its arguments, one whose grammar forgives an unpadded
  ordinal, one whose `parse` ignores what the listing found, one with a second
  distinguished name — each a positive control proving the conformance kit can
  say no. Not one `check` translated into anything: a check says the design is
  sound and hands you nothing to run. Entry 002 recorded the witnesses as the
  more valuable half for *explaining why a law is there*; they are also the more
  valuable half for *deriving tests*, and that is a different and sharper claim.
- **The models are silent on exactly where the work was.** Both hold no strings,
  by design — the entire grammar reduces to one round-trip law — so the
  reference domain's parse and format, the largest single piece of this leaf,
  was written with zero model coverage. The obligation is checkable and *how to
  hold it* was not modelled anywhere. The technique that discharges it is one
  line and no model suggested it: parse leniently, render the result, and refuse
  the input if it differs. Field-level padding rules drift; a whole-grammar
  re-render cannot.
- **Prior art violates the obligation, and reading it through the obligation is
  what showed that.** grove's own `src/tree_id.rs` is deliberately lenient on
  ordinal padding — `parse_position` accepts `5`, `Entry::name` renders `05` — so
  `format(parse(f)) == f` fails and `5-impl-x-k1.md` and `05-impl-x-k1.md` are
  one entry with one key at one ordinal. That is
  `witness_two_filenames_name_one_entry` standing in production code. It is
  recorded here and in the root brief as an input to increment 2 rather than
  fixed in passing.

**Missed.** Three, and the first is the one this workstream has now met in a
third dress.

- **Nothing checks that the code matches the model.** Entry 003 named the hand
  translation between `structure.als` and `operations.qnt` as the concrete cost
  of running two formalisms with no checker between them. There is a third
  vertex: the trait in `ARCHITECTURE.md` is prose with syntax highlighting, and
  nothing verifies that `src/name.rs` says the same thing. Folding three
  accessors into one was a deliberate, reconciled edit across three artifacts;
  an *accidental* divergence would have been caught by nothing at all. The
  discipline that substitutes for a checker is naming the claim in each test's
  comment, which is a convention and reads clean when it is broken.
- **Neither model has anything to say about ergonomics or the public surface.**
  Whether the seam should be four methods or six, whether `Triple` borrows or
  owns, what a conformance kit even *is* — all decided by reading the ADR and
  arguing with the document. The models constrain the design and do not produce
  it.
- **The grammar's ambiguities are invisible to both.** That a label may itself
  end in something shaped like a key (`01-draft-notes-i7-i3.md`) and the name
  still read one way only is a real property with a real test, and no model
  states it.

**Cost.** One session. Tooling friction was zero for the first time in this log:
both runners found their toolchains and reported per-claim pass/fail, which is
entry 003's own repair working. Re-running them cost about two minutes and
established, before anything depended on it, that a green suite here means the
suite ran.

**Counterfactual.** Three, pointing in three directions.

- **The `triple()` fold was free and was written down a session in advance.**
  Nothing needed to be discovered; the log had it. What that measures is the
  log's format rather than any formalism — the *counterfactual* field is the one
  that carried it, which is why it is the load-bearing field.
- **The grammar work wanted a property-based test, not a model.**
  `format(parse(f)) == f` over generated filenames is the instrument that fits,
  and it was not reached for; the fixed sample list in the conformance kit is a
  weaker version of the same idea, and it says so by reporting which obligations
  its samples never exercised. This is the first row this log can name and has
  no evidence for.
- **The prior-art defect would have been caught by an implementation attempt of
  the flip** — at the moment two grove trees disagreed about a key — which is to
  say much later and in a live task tree. Reading the old code *through* a
  stated obligation is a free technique and it is not a formalism.

**On H3 — untested here, and this entry is not evidence for it.** The
implementation was faithful to the models, but the same person wrote both
readings and there was no prose arm. `07-impl-h3-probe-k14` is the pre-registered
experiment; this entry is the uncontrolled observation that precedes it, recorded
so a later reader does not mistake it for a result.

**Verdict.** When implementing against an existing model, **read the witnesses
first** — they are the test suite, and the checks are not. And ask the type
system before you ask the kit: an obligation the language forbids is one no
consumer can break, one fewer check to maintain, and the only kind of guarantee
in this log that costs nothing to keep.

### 005 — Integrating an adversarial review of the seam (`ordinal-fs-tree`)

**Situation.** `seam-k17` read the seam `seam-k8` had built — one trait, one
reference domain, a conformance kit, a filesystem-boundary guard — with a
charter to attack the producer's own deviation from the architecture document.
It produced six findings and changed nothing. This entry is the integration:
what the findings were evidence *about*, beyond the code they fixed.

**Formalism.** None written. Both suites re-run before and after (green both
times, ~2 minutes each, no friction). The instruments that did the work were the
same two entry 004 named — the type system, and reading the code *through* a
stated obligation — plus a third this log has not counted: **an adversarial
reader whose brief names the specific judgement to attack.**

**Caught.** Four, and they share one shape: every one is a *claim about a check*
that was weaker than the claim it stood for.

- **"Rust discharges this obligation" was itself an unchecked claim, and it was
  half wrong.** Entry 004 collected `fn triple(&self) -> Option<Triple<'_,
  Parts>>` as the fold that made *a name is positioned or distinguished, never
  neither* unrepresentable. It closes *neither* and leaves *both*: a name could
  return a triple and report species `Distinguished`, because `species()` was an
  independent method. `src/name.rs` said so in one sentence; `ARCHITECTURE.md`,
  `structure.als`'s comment and `DISCHARGED_BY_THE_TYPE_SYSTEM` all said the
  obligation was discharged, full stop. **A partial discharge annotated in prose
  is a full discharge to the next three readers.**
- **The model stated a law the seam did not carry.** `SpeciesFromParts` is one
  of `structure.als`'s assumed trait laws, and every derivation rests on it — a
  shift is `compose(new_ordinal, key, parts)`, so a species free to vary with the
  ordinal turns a shift into a file renamed to a directory. The Rust trait
  exposed `fn species(&self)` beside the parts and required nothing, so a
  conforming domain could break a law the model was entitled to assume. Both are
  now unrepresentable — one `NameView`, and `positioned_species(parts: &Parts)`
  as an associated function with no `self`, no ordinal and no key — and the
  obligation count went from five to six because the second law was always an
  obligation and had been filed as a consequence.
- **Two tests cited a model claim and checked something weaker.**
  `SpeciesAgreementIsParsed` says a contradicted name is *malformed*; the kit
  accepted *anything that is not an entry*, so a domain returning `Foreign` —
  the one verdict a walk skips silently, taking the subtree with it — passed the
  check named after the claim that exists to stop exactly that. `RoundTripDisplay`
  is `v.seen = n`, name identity; the kit compared **renderings**, so a grammar
  could parse its own output into a different key and pass. In both cases the
  test drifted toward *the property that is easier to observe*, and the citation
  in the comment made the drift invisible.
- **A textual instrument documented the wrong hole.** The `std::fs` guard
  scanned for `fs::` and `std::fs` and its header named `use std::fs as f;` as
  the known limit — which it actually catches. The real hole was the ordinary
  grouped import `use std::{fs, path::Path};`, which contains neither token. Its
  exemption was also written by file *name*, silently excusing any
  `src/algebra/fs.rs`, while its header promised a rule about a path.

**Missed.** Two.

- **Neither model found any of this, and neither could have.** The trait laws
  are assumed predicates over an abstract `Trait` object; nothing connects them
  to Rust, so a seam that fails to carry one is invisible to the suite that
  assumes it. This is entry 004's first *missed* — nothing checks that the code
  matches the model — arriving one leaf later as two defects, exactly where it
  said it would.
- **The grammar is still unmodelled, and it is still where defects are.** The
  reference domain classified `01--i3` — its own shape with the label missing —
  as `Foreign`, so a walk would skip the file and, in the directory spelling, the
  whole subtree beneath it. `NothingRecognisedIsSkipped` is a checked claim
  about exactly that hazard and holds no strings, so it says nothing about which
  names a grammar recognises. Found by a reader, not by a tool, for the second
  entry running.

**Cost.** One session, of which the models were four minutes and the reading was
everything else. The fix cost more than the finding: closing the two trait-shape
defects changed the public surface (a `NameView`, a `PositionedSpecies`, and a
sealed blanket-implemented `EntryNameExt` so that `triple()` and `species()`
cannot be overridden back into existence), and every test and broken domain
moved with it. Doing this after the eight dependent leaves — where an appended
`review-*` step would have put it — would have been a rework of the subtree
instead.

**Counterfactual.** Three, and the first two are new rows.

- **A test citing a model claim should carry the claim's predicate, not its
  name.** Both weakened checks would have been caught at writing time by pasting
  the one line of Alloy beside the assertion: `v.seen = n` does not read like
  *the strings match*, and `SpeciesAgreementIsParsed`'s `Malformed` does not read
  like *not an entry*. The citation convention that entry 004 called "a
  convention that reads clean when it is broken" is precisely as good as its
  weakest citation, and the repair costs one comment.
- **A partial discharge should be split into two obligations, not annotated as
  one.** The prose qualification survived in the file that made the claim and in
  none of the three artifacts that repeated it. Either the language forbids the
  whole obligation — in which case say so and delete the check — or it forbids a
  nameable half, which is then a different obligation with its own name.
- **An adversarial reader is cheap and is not a formalism.** Five of the six
  findings came from a brief that named the judgement to attack ("the trait
  deviates from the document, deliberately — that judgement is the main thing to
  attack") rather than asking for a review. No model, no tool, one session, and
  the two structural findings were unreachable by either model in the repository.

**On H2 — mild evidence for, and not the kind expected.** The model was *right*
and the implementation had drifted from it; the model earned its keep by being a
written-down law an adversarial reader could hold the code against, not by
finding anything itself. A model with no reader is worth what an unread test is.

**Verdict.** Reach for the type system first, then for a reader briefed to
attack one judgement. And treat *the language discharges this* like any model
claim: it is a proof you did not write down, so state what it does **not**
cover, or split it until every part is either forbidden or checked.

### 006 — Implementing the first leaf the models barely reach (`ordinal-fs-tree`'s reading layer)

**Situation.** The lock, the snapshot, and the five reading operations — the
first leaf of the library that touches a filesystem. Two checked models and a
reconciled architecture document already existed. Unlike `seam-k8`, almost none
of this leaf's subject is *in* either model: `operations.qnt` models mutations,
and its handoff block records walk **order** as explicitly unmodelled.

**Formalism.** None written. Both suites re-run first — Alloy 20/20, Quint every
claim across all eight instances, every witness reached in a non-zero number of
traces — about five minutes, no friction. Three instruments did the work, and
only the first has appeared in this log before: the models read as a
specification, the type system, and **deliberate mutation of the implementation
to watch the tests fail**.

**Caught.** Four.

- **The model's recorded *miss* was the whole worklist, and reading it first is
  what made the leaf tractable.** Entry 004's verdict was *read the witnesses
  first — they are the test suite*. Its sibling is sharper: **read the misses
  first — they are the specification of what you have to get right unaided.**
  `operations.qnt` says walk order is unmodelled and that it resolves `by_key`
  by least internal id; that one sentence located every line of prose this leaf
  had to write and defend, before any code existed.
- **A tie-break the document never stated, found by composing two things the
  models state separately.** Walk order within a level was written as
  *distinguished child first, then children by ordinal* — total only if ordinals
  are distinct. But every invariant in this design is **preservation, not
  establishment** (both models are explicit), so a hand-edited level carrying a
  duplicate ordinal is a tree the library must still traverse; and a directory
  listing arrives in whatever order the filesystem chose. Together: `by_key`'s
  documented *first in walk order* would have named a **different entry on two
  machines holding byte-identical trees**. Neither model could have found it —
  both hold no strings and neither models a listing — but both supplied the
  premise, and the composition is free.
- **An instrument that made its own promised path undeclarable.** `seam-k8`'s
  `std::fs` guard scans every source outside `src/fs/` for the word `fs`, so
  `mod fs;` in `lib.rs` was a violation of the rule the guard exists to hold: the
  module could not be declared at all. The repair — exempt the declaration shape
  and nothing else — also closes the hole the guard's own header names as a known
  limit, because a *re-export* stays a violation and there is now no crate-root
  alias to launder a filesystem item through. The defect was invisible for two
  leaves because nothing had yet needed the module to exist.
- **A refusal neither model can pose: a filename that is not text.**
  `parse` takes a `&str`, so there is no verdict to be had and no domain error to
  carry — the library needs a refusal of its own, and it must **halt**, by the
  trichotomy's own argument: a name that cannot be read cannot be disclaimed
  either, and one mangled byte in a real name produces exactly this. Both models
  hold no strings *by design*, and that simplification — recorded in 002 and 003
  as a saving — has a cost this leaf paid: the entire class *the name is not a
  string* is outside what either can state.

**Missed.** Three.

- **The models had almost nothing to say about this leaf's code, and that was
  predictable in advance.** One definition was directly reusable —
  `structure.als`'s `descendable`/`visited` (descend into the root and into
  recognised nodes; not into a foreign name, not into a distinguished child), and
  transcribing it is exactly what the snapshot reader does. Beyond that: **five of
  this leaf's twenty-three behaviour tests name a model claim, and eighteen say
  they have none.** That is not a complaint — it is the measurement, and the H3
  probe wants it.
- **A branch that cannot be exercised on the machine it was written on.** APFS
  validates filenames as UTF-8 and refuses to create a non-UTF-8 one, so the halt
  above is unreachable on a stock macOS checkout. No model, tool or test found
  this: the *filesystem* did, by refusing the fixture. The test now asserts
  whichever fact is true on the host — the platform refused the name, or the
  library halted on it — because a skipped test reports what a passing one
  reports, which is this log's recurring instrument failure in its fourth dress.
- **Nothing still checks that the code matches the model.** Entry 004's first
  miss, unchanged. The compensating discipline is naming the claim in each test's
  comment, and 005 sharpened it to *quote the predicate, not the name* — which
  helps only where a claim exists at all. For eighteen of twenty-three tests here
  there is no claim to quote, and the discipline degrades to a sentence saying so.

**Cost.** One session. The models were five minutes. The mutation controls were
about fifteen: six deliberate breakages — the distinguished-first rule inverted,
`Malformed` skipped instead of halting, `flock` stubbed out, the listing made to
follow symbolic links, an import added to a real algebra module, a filesystem
item re-exported from the crate root — each rebuilt and run, each firing exactly
the tests it should and no others.

**Counterfactual.** Three, and the first is a new row.

- **Mutation controls are the instrument that answers *does this test test
  anything*, and they cost minutes.** Every test in this leaf passed the first
  time it ran, which is the state in which a suite is least trustworthy: a test
  that has never been seen failing is an assertion about the author's confidence.
  Six mutations turned "all green" into evidence, and two of them (the lock stub,
  the re-export) were the only proof that claims made in *prose* — "dropping the
  guard releases it", "a re-export stays a violation" — were true of the code.
  Not a formalism, cheaper than one, and applicable to any suite.
- **The order tie-break wanted a property-based test, and would have been
  positive evidence for a row this log opened empty.** `walk(shuffle(listing)) ==
  walk(listing)` over generated levels is the instrument that fits — the same
  shape entry 004 named for the grammar (`format(parse(f)) == f`) and had no
  evidence for. Two independent leaves have now reached for it and neither has
  run it; that pattern is itself the finding.
- **The undeclarable-module defect would have been caught by writing the guard's
  control from the consumer's side.** The question *what does the file that
  declares this module look like?* costs nothing and was never asked, because the
  guard was tested against synthetic source strings and never against the shape
  its own repository would take. A guard validated only on fixtures has not met
  its subject.

**On H2 — evidence for, of a kind the earlier entries did not produce.** The
model earned its keep here by **stating what it does not cover**. Nothing it
checked was used; the sentence recording its own limit is what directed the
session's attention to the only genuinely undecided question in the leaf. A model
that documents its misses is doing work even where it has no claims.

**Verdict.** Read a model's recorded **misses** before its claims — they are the
worklist, as its witnesses are the test suite. And before believing a green
suite, break the implementation six ways and watch it go red: it is minutes, it
is not a formalism, and it is the only thing that distinguishes a suite that
holds from one that was written to pass.

### 007 — Adversarial review of the first filesystem layer (`ordinal-fs-tree`)

**Situation.** `reading-k19` inspected the lock, snapshot, five reading
operations and their tests before the four mutation leaves could build on them.
The producer had already named five judgements to attack and recorded six
deliberate mutations, so the review treated those mutations as claims about the
instrument rather than rerunning them.

**Formalism.** None written or run. This was the same instrument entry 005 first
measured: a fresh adversarial reader briefed with specific judgements, reading
the committed diff against the architecture, both models' recorded misses, the
source and the tests. The review kind deliberately did not run test, build, lint
or model commands. Codebase-memory could not index this jj workspace because its
worker could not verify daemon coordination in the sandbox, so every cited file
was read directly and negative test-coverage claims were checked over the exact
test sources rather than inferred from an empty graph.

**Caught.** Four findings, three of them claims about what the tests established
rather than failures in the happy path.

- The accepted spelling `root/child/..` reads the same tree while lexical
  `parent()` locks `root/child`, not the directory the direct spelling locks.
  The lock therefore fails its same-tree/same-inode claim exactly on the
  no-canonicalisation test's own fixture. A terminal symlink has the same shape.
- The textual no-filesystem guard strips comment delimiters without lexing
  literals. A `"/*"` string can put it at positive comment depth and hide every
  later filesystem use while the guard reports clean.
- The test named for `(ordinal, key, rendered name)` aligns key order with name
  order, so an implementation that omits the key tie-break passes it. The
  recorded mutation inverted distinguished-first order and never exercised this
  differently wrong comparator.
- Public `Place` values carry only an arena index. A place from another builder
  silently names the current builder's node when their indices coincide, despite
  `Builder::add` promising that a foreign place panics.

**Missed.** This review did not execute the producer's verification evidence,
by design: `review-*` is an inspection-only kind, so environmental and runtime
failures remain the producer's recorded evidence until the integration leaf
changes code and reruns it. The non-UTF-8 trade was attacked and held: once the
seam accepts `&str`, treating unreadable bytes as `Foreign` would be a guess.
The walk comparator itself also held; the defect is in the fixture meant to
protect it.

**Cost.** One session. Most of the cost was reading the 1,600-line artifact and
its specifications; the disproofs themselves were small. The graph indexing
failure added a short fallback pass over exact sources but changed no finding.

**Counterfactual.** Each finding had a cheaper executable question the producer
could have asked. Acquire two guards for the direct and roundabout spellings and
ask whether they contend. Put opposing key/name order in the comparator fixture.
Feed the textual detector comment-shaped tokens inside every Rust literal form.
Pass a same-index node place between two builders. The shared lesson is sharper
than *mutate the implementation*: **mutate the assumption independently of the
happy-path structure the test author chose**. A stubbed lock proves presence,
not identity; one inverted sort key proves ordering matters, not that all later
keys are in the stated precedence.

**Verdict.** Reach for a named-judgement adversarial review before dependants
when a layer's tests and implementation were written together. Mutation controls
are evidence only for the semantic dimension they vary; ask which independently
wrong implementation each control still lets through.

### 008 — Integrating the reading layer's review (`ordinal-fs-tree`)

**Situation.** `reading-k20` applied `reading-k19`'s four findings to the reading
layer before the four mutation leaves build on it: lock identity, the textual
no-filesystem guard, the walk-order tie-break fixture, and the public
`Builder`/`Place` construction arena. The review changed no code and ran no
commands, so every claim in it arrived as an assertion to be checked.

**Formalism.** None written. Both suites were re-run as controls and are
unchanged — Alloy 20/20, Quint 148 claims, witnesses reached in non-zero traces —
because nothing here is modelled: the lock, the guard, walk *order* and the
construction seam are all below or beside what either model states. The
instrument this session actually used is the one entry 007 named — **mutate the
assumption, not the happy path** — turned on the *repairs*: every fix was watched
failing against the implementation it replaced before it was believed.

**Caught.** Three findings held, one held for the wrong reason, and one further
defect came out of testing the repair rather than the finding.

- **Lock identity held, exactly as stated.** A lexical `Path::parent` does not
  converge: `x/y/..` locks `x/y`. The repair is to name the directory `<root>/..`
  and let the kernel resolve it — `..` and a final-component symbolic link are
  resolved per component against the directory actually reached — which keeps the
  no-canonicalisation rule (the path is still the caller's spelling) while making
  the *lock* follow the tree. Both new contention tests fail against the lexical
  version and only those two do. A filesystem root is now refused on device and
  inode rather than on `parent().is_none()`, so `/..` is refused like `/`.
- **The guard's literal blindness held, and was worse than a per-line problem.**
  Ten synthetic sources, each a comment delimiter inside a different literal
  form followed by a real filesystem use, were run against the retired stripper:
  four were missed outright. The repair is to stop lexing by hand — the sources
  go through `proc-macro2` and a use of the filesystem is an `Ident` equal to
  `fs`. Comments never reach a token stream, a doc comment is a `Literal`, and
  raw-string hash counts and the `'a`-versus-`'c'` ambiguity become the lexer's
  problem. A hand-added violation in two real modules was watched failing, with
  correct line numbers, and removed.
- **The tie-break finding was real and its counterexample was false.** The
  review said the old fixture admitted an implementation ordering by rendered
  name alone. It does not: that mutation was run and the old fixture failed it.
  What the old fixture *did* admit — and what the first repair of it still
  admitted — is an implementation with **no rendered-name tie-break at all**:
  two names comparing equal keep their arrival order under a stable sort, and
  both fixtures happened to supply names in the order the rule prescribes. The
  finished fixture points key and name in opposite directions *and* inserts the
  equal-key pair in the order the tie-break must reverse.
- **Asking the same question of the comparator's first key found an untested
  one.** Every fixture in the file gave its entries ordinals and keys that rise
  together, so an implementation comparing the key *before* the ordinal passed
  all of them. One two-entry test now fails it.
- **The foreign-place finding held, and the reassessment it asked for changed
  more than the fix.** A `Place` now carries which builder handed it out. But
  the arena was public only so that an integration test could build a tree
  without a directory, and `ARCHITECTURE.md` never specified it: it is now
  crate-private and the pure algebra tests moved into the module beside it. A
  construction surface published for a test arrangement is production surface
  bought with no consumer.

**Missed.** Making a foreign place *unrepresentable* rather than rejected was
considered and dropped: the shape that does it — a nested-closure builder whose
level handle cannot escape its callback — forces recursion, and `read.rs` uses an
explicit worklist precisely because tree depth is the user's to choose and a
stack overflow is not a refusal a consumer can handle. So the seam is checked at
runtime, on an identity, rather than forbidden by the type system. Nothing here
was modelled, and nothing here suggests either model should have reached it.

**Cost.** One session. The repairs were small; the confidence was the expensive
part — six mutation runs, one throwaway harness rebuilding the retired stripper
to measure it, and a pair run of hand-added violations. One new *dev*-dependency
(`proc-macro2`), justified in the manifest on the same bar `libc` cleared: the
alternative was maintaining a partial Rust lexer, which is more surface than the
guard and fails in the same silent direction.

**Counterfactual.** Two of these were cheap executable questions the producer
could have asked, and entry 007 already named them. The two this session adds are
sharper and both are about *controls* rather than code:

- **A comparator with `k` keys wants `k` controls, and each one has to arrive out
  of order.** A tie-break control whose fixture supplies the entries in the order
  the rule prescribes cannot distinguish the rule from a stable sort — it is
  satisfied by construction, which is entry 003's warning arriving in a test
  instead of a model. Point each key against every later key, and insert the
  equal pair backwards.
- **A finding is a hypothesis, and running its counterexample is cheaper than
  believing it.** The tie-break finding's stated mutation cost two minutes to run
  and was wrong, while the real defect it pointed at was worse. An integration
  session that agrees with its review by reading it inherits the review's
  mistakes, including the flattering ones.

**Verdict.** Reach for mutation controls on the *repair* as a matter of course:
the fix and the test that proves it are written by one session, which is the
condition entry 007 identified as producing controls that only vary the dimension
their author was already thinking about. And for a boundary guard that reads
source text, use the language's own lexer — a partial one is a second
implementation of the hardest part of the language, maintained by whoever
happens to touch the test.

### 009 — Implementing the leaf the models were written for (`ordinal-fs-tree`'s interpreter)

**Situation.** `interpreter-k10`: the plan, effect and decision types, the single
interpreter with its exclusive create and its rollback, and the two operations
that make both observable — `append` and `append_many`. Unlike every earlier
implementation leaf, this one's subject is the model's subject: `operations.qnt`
*is* a plan interpreter, and its `failures` and `rollback_fails` instances exist
precisely to check what this code does.

**Formalism.** None written. Both suites re-run as controls first — Alloy 20/20,
Quint every claim across all eight instances with every witness reached in a
non-zero number of traces — about seven minutes, no friction. The instruments
that did the work were the model read as a specification, the type system, and
**nine mutation controls**.

**The measurement first, because it is this leaf's main evidence.** Entry 006
recorded that five of the reading layer's twenty-three tests named a model claim
and eighteen said they had none. Here it is **twenty-eight of forty-two naming a
claim and fourteen with none** — the ratio inverted, on a leaf of comparable
size, because the subject is inside the model rather than beside it. That is the
cleanest H3 evidence this log has: what a checked model is worth to an
implementation depends almost entirely on whether the model was written about
the thing being implemented, and the answer is legible *before* the leaf starts,
from the model's own handoff block.

**Caught.** Five.

- **Two of the model's comments became types, and the claims they carry stopped
  needing to be checked.** `operations.qnt` gives `Effect` three variants and
  then says of one: *`Remove` never appears in a forward plan — it is only ever
  generated as the undo of a `Create`.* In Rust that is two types: a plan holds
  `Create | MoveTo`, and the interpreter's own `Undo` holds `Remove | Restore`,
  constructible only from an effect the run just applied. `inv_rollbackRemovesOnlyItsOwn`
  is then structural. The same move on `Decision`: two variants and no third *is*
  the totality the model establishes by `decide`'s return type. This is entry
  002's counterfactual — *before modelling a structural property, ask whether the
  target language already forbids it* — arriving for the third time, and the new
  part is where to look for candidates: **a model's explanatory comments are
  where its unenforced invariants hide.**
- **Entry 003's sequential-destination finding was worth a test the operation
  under implementation cannot reach.** Whether the algebra folds the plan through
  the snapshot or checks every destination against it is invisible to every
  `append` test there is: an append composes names carrying a key no entry holds,
  so no destination it computes can be taken by anything, under either reading.
  The discriminating case is two siblings sharing a key *and* its parts —
  `cp 01-foo-i5.md 02-foo-i5.md` — under an `insert`'s shift, which is a later
  leaf. So the plan `insert` will build was built by hand and folded now, in both
  orders. A mutation control confirms the pair: replacing the fold with a
  snapshot check turns the correct plan into a refusal and nothing else in the
  suite notices.
- **A whole refusal class neither model can pose, and it is not the usual one.**
  Entry 006 found *the name is not a string* outside both models because both
  hold no strings by design. This leaf found the arithmetic equivalent: an
  integer in either model is **unbounded**, and a key and an ordinal are 32 bits.
  A hand-written name carrying `u32::MAX` makes `max + 1` impossible, and the
  choice — refuse, or wrap and re-issue a live key — is exactly the kind of
  question the models exist to force, and cannot. Two refusals (`KeysExhausted`,
  `OrdinalsExhausted`) and a third for content, which is unmodelled for the same
  by-design reason bytes always were. All three are now in `ARCHITECTURE.md`,
  which is the specification of record for what the models do not reach.
- **A mutation control found an untested claim the model's own uniformity hides.**
  *The interpreter claims each destination with an exclusive create* reads as one
  rule; on a real filesystem it is two mechanisms. `create_new` and `create_dir`
  refuse an occupied destination in one syscall — but `rename(2)` **replaces**
  its destination silently, so a rename has to look first, unfollowed, and macOS
  has no portable no-replace rename to do it atomically. `operations.qnt`'s
  `effectBlocked` is one function over all effects, which is right in a model with
  no syscalls and hides the split completely. Deleting the pre-rename look left
  all forty-one other tests green.
- **The model settled an API question it never mentions.** Whether a mutation
  consumes its guard or borrows it is not in either model — there is no handle in
  a model — but `inv_atomicity` decides it anyway: a guard that survived its own
  mutation would either plan the next one from a stale snapshot or have to
  re-read, and a re-read that fails after the effects landed is a mutation
  returning an error with the tree changed, which is the precise shape atomicity
  promises not to have. The invariant was the argument, in a place it makes no
  claim about.

**Missed.** Three.

- **Nothing still checks that the code matches the model.** Entry 004's first
  miss, unchanged and now more expensive: twenty-eight tests name a claim, and
  what holds them to it is a comment. The compensating discipline (005: quote the
  predicate, not the name) is followed here and is still a convention.
- **The model's `failures` instance checks atomicity of a *modelled* interpreter,
  and this leaf's interpreter is a different program.** They agree because the
  same person wrote one from the other; nothing derives one from the other, and
  no counterexample the model produced was ever replayed against the code. That
  is the honest reading of what *the model leads* bought here: a specification and
  a worklist, not a verified implementation.
- **The one design question with real freedom in it was outside both models
  again.** `Level::Created` — an effect naming a directory an earlier effect of
  the same plan creates — is in `planPromote` by inspection, but whether the
  implementation should carry it *now*, with no operation producing it, is a
  judgement about leaf boundaries that nothing checked. It is kept, tested by
  hand, and marked `#[cfg_attr(not(test), expect(dead_code))]` so that the leaf
  which finally builds it is forced to remove the marker.

**Cost.** One session. The suites were about seven minutes; the nine mutation
controls about fifteen, scripted so each patches, runs and reverts. The reading
was the expensive part, as at every leaf: `ARCHITECTURE.md`, the glossary, the
1,326-line model and five prior findings entries before a line was written.

**Counterfactual.** Three, and the first two are new rows.

- **A control per *mechanism*, not per property.** Entry 007 sharpened mutation
  controls to *mutate the assumption, not the happy path*; the next turn of the
  same screw is that one property implemented by two mechanisms wants two
  controls. *Claims its destination* is one sentence covering `create_new` and a
  look-then-rename, and the control for the first says nothing about the second —
  which is how eight controls could pass while a rename would have destroyed a
  neighbour's file. Ask of every checked property: **how many different pieces of
  machinery implement it?**
- **A model's idealisations are enumerable in advance, and each one is a refusal
  class.** No strings (006), no bytes, unbounded integers, no filesystem, no
  concurrency — `operations.qnt` lists most of them in its own handoff block.
  Reading that list as *the specification of what you must decide unaided* is
  entry 006's verdict; reading it as *the list of places the implementation will
  need a refusal the model cannot name* is sharper, and would have produced all
  three of this leaf's own refusals before any code existed rather than one at a
  time as the types demanded them.
- **When the discriminating case for a decision belongs to a later leaf, build it
  by hand now.** The alternative is that the decision sits untested until that
  leaf arrives, and a regression in it is silent — which is exactly what entry
  003 warned about when it said getting the fold wrong *makes the next leaf's
  ordering rule vacuous, and nothing will tell you*. Hand-building the plan the
  later operation will produce costs one fixture.

**On H2 — the strongest evidence so far, and it is about coverage rather than
quality.** The model led this leaf in the ordinary sense: the test list came off
the witness list, the refusals came off `Outcome`, and two design decisions came
from invariants that do not mention them. But the finding that generalises is the
measurement, not the experience — the model's value tracked, almost exactly, the
fraction of the leaf that was inside its scope.

**Verdict.** Reach for a behavioural model when the thing being built *is* a
state machine, and expect the payoff to be proportional to that overlap rather
than to the model's quality. Then check the model's stated idealisations for the
refusals it cannot name, and count the mechanisms behind each property before
believing one control covers it.

---

### 010 — Adversarial review of the plan interpreter (`ordinal-fs-tree`)

**Situation.** `interpreter-k21` inspected the plan types, the one filesystem
interpreter, `append`/`append_many`, their forty-two tests and the producer's
nine recorded mutation controls before `insert`, `promote`, `rewrite` and the
CLI could depend on them. Seven deliberate judgements were attacked against the
exact Quint predicates rather than their names.

**Formalism.** None written or run. This was a fresh, inspection-only
`review-impl` session: the producer commit, architecture, model handoff and
predicates, source, tests and verification narrative were read without running
test, build, lint, format or model commands. Codebase-memory could not index this
jj workspace because daemon coordination could not be verified, so exact source
reads replaced graph claims and every graph limitation was kept explicit.

**Caught.** Five findings: three implementation defects and two missing controls.

- `EntryName::Display` is never constrained to one normal path component. A
  domain satisfying all six obligations can compose `../outside` or an absolute
  rendering, and the interpreter joins it directly, allowing create, move and
  rollback outside the locked root. This is the strings-are-unmodelled miss
  turned from a refusal class into a confinement obligation.
- The algebra excludes a moved entry from occupying its own destination, but
  the interpreter's pre-rename look does not. The exact same-parts rewrite that
  `wit_rewriteToSameParts` requires to succeed is therefore refused by the
  filesystem layer; its named test stops at the plan boundary.
- `Report::paths` promises effect order while storing creations and renames in
  separate buckets and concatenating them. Every insert (moves then create) and
  promotion-with-child (create, move, create) disproves the promise.
- Atomicity is correctly implemented for a content write failure — undo is
  registered after exclusive creation and before `write_all` — but no failure
  seam reaches that interval. Moving the registration after the write leaves all
  current atomicity tests green and makes a real short-write error strand a
  partial file.
- Destination claiming has three filesystem mechanisms, not the two the
  producer counted. File create and rename are controlled; node `create_dir` is
  not. Replacing it with `create_dir_all` leaves the suite green and can make a
  rollback remove a neighbour's pre-existing empty directory.

**Missed.** The review did not execute the producer's mutation scripts or model
suites, by kind. The guard-consumption API, forward-effect/undo split,
pre-registration of move machinery, look-then-rename concurrency boundary,
three unmodelled refusals and singleton delegation all survived attack. The
twenty-eight/fourteen model-claim count is accurate; the issue is that a claim
name can stop at one implementation boundary while the predicate crosses two.

**Cost.** One session. Most cost was the complete architecture/model/test read;
the disproofs were small once the seven judgements fixed the search axes. The
failed graph indexing added a direct-source fallback but changed no result.

**Counterfactual.** Count mechanisms across boundaries, not within the module
where the property was first stated. Apply the plan test through the interpreter
for same-path moves. Inject failure after file creation, not only before an
effect. Exercise file create, directory create and rename separately. Feed the
name seam a value that is a valid model name and an invalid filesystem
component. Ask a mixed-effect report for its order. Each question independently
mutates an assumption the existing happy paths hold fixed.

**Verdict.** A named-judgement review remains valuable even when model overlap
and test density are high. Models and mutation controls partition by their own
abstraction boundaries; the cheapest adversarial move is to follow one promised
predicate across the next boundary and count the mechanisms again there.

### 011 — Integrating the plan interpreter's review (`ordinal-fs-tree`)

**Situation.** `interpreter-k22` applied `interpreter-k21`'s five findings to the
plan substrate before `insert`, `promote`, `rewrite` and the CLI build on it: the
name-confinement hole, the no-op rewrite the model requires to succeed, the
report's order promise, and the two missing mechanism-level controls for
atomicity and destination claiming. Four architectural judgements the review had
accepted were left alone, as were the look-then-rename race and its stated
boundary.

**Formalism.** Both suites re-run and **neither model changed** — which was the
finding to look for first, not a formality. Alloy 20/20 and every Quint claim
across all eight instances passed before and after, every witness reached in a
non-zero number of traces. Nothing here was a disagreement with a model: two of
the five findings were the code contradicting a claim that still says what it
said (`wit_rewriteToSameParts`, `inv_atomicity`), and the other three were in
territory both models exclude by design. So the models led by *not moving*, and
the code moved to meet them.

**Caught.** All five integrated, and each fix has a control that fails without
it — verified by making the exact wrong change and watching exactly one test go
red, five times.

- Confinement became the **seventh obligation**, and the first one the library
  *enforces* rather than assumes: every name a snapshot admits is rendered and
  checked when the tree is read, and every name a plan will place is checked
  before the first effect. `Error::NameIsNotOneComponent`, the architecture's
  obligations and refusals, the trait docs and the conformance kit were reworked
  as one contract, and two adversarial domains — one per boundary, each
  satisfying everything the algebra looks at — are its control.
- The interpreter now carries the algebra's mover exclusion across the boundary.
  A same-path move claims nothing and registers no undo, and the second of those
  matters more than it looks: an `Undo::Restore` for a no-op renames onto its own
  occupied path, so a *rollback* would have turned a clean unwind into
  `FailedPartiallyRolledBack`. The fix's own control is the negative half — a
  genuinely different occupied destination is still refused.
- `Report` keeps an ordered landing record beside its two species buckets, so
  `paths()` is the plan's own order while `created()` and `renamed()` stay in
  theirs — which is where the highest-first shift rule remains readable.
- A failure seam now reaches the interval between a leaf's exclusive creation and
  its content write, which is where the undo registration lives.
- Node creation got its own uncooperative-neighbour control, with a second
  failing effect behind it so that `create_dir_all` would be caught by the
  *unwind* removing a neighbour's directory and not only by the missing error.

**Missed.** Re-reading the forty-two tests against their predicates found three
that named no claim at all — two the producer's, one this session's own first
draft — which the review's count of twenty-eight naming and fourteen disclaiming
had not separated from the honest disclaimers, because a test with no line at all
is invisible to a count of two kinds. The account is now **forty-eight tests,
thirty naming a claim and eighteen saying they have none**. Separately, the lib
suite failed once, on its first run of the session, with
`FailedPartiallyRolledBack` where `Error::Failed` was expected, and did not
reproduce in three hundred further runs of the same binary; the assertion that
caught it carried no diagnostic message, which is why the first thing this
session had to do was add one. Unexplained, and recorded rather than tidied away.

**Cost.** One session. The five fixes were small; the confinement one was not,
because deciding *where* it is enforced is a contract across four artifacts and
the wrong answer — an obligation with no enforcement — would have left the
adversary the review described still able to leave the tree. The mutation round
cost minutes and is the only reason any of this is evidence.

**Counterfactual.** **When a model excludes a domain by design, that exclusion is
a worklist, not a disclaimer.** `operations.qnt`'s handoff names strings, bytes
and the filesystem as out of scope; the three findings with no model behind them
are one from each. The exclusions were written down two leaves before the defects
were found, and reading them as *therefore prose owns these, so say what prose
says* would have caught all three at the point they were introduced. The second,
cheaper move: an obligation the target language cannot make unrepresentable is
not automatically one the *library* cannot check — ask which of the two the
obligation is, because a cheap run-time check turns a silent corruption into a
refusal with recovery advice.

**Verdict.** Re-running an unchanged model after a code change is worth the
wall-clock: it is what separates *the code was wrong* from *the specification
moved*, and only the first is a fix. Reach for the handoff's own exclusion list
whenever a defect is found in territory a model does not cover — it names the
other places to look.

### 012 — Implementing the operation the ordering rule was written for (`ordinal-fs-tree`'s `insert`)

**Situation.** `insert-k11`: add a child at an occupied ordinal, shifting the
occupant and every later sibling up by one. One rename per shifted sibling and
one create. This is the operation `docs/formalism-findings.md` entry 003's two
headline findings were *about* — the shift order's real reason, and the
sequential destination check — so it is the first leaf where those findings had
to be turned into code and into tests rather than into prose.

**Formalism.** None written. Both suites re-run as controls first: Alloy 20/20
in sixteen seconds, Quint every claim across all eight instances with every
witness reached in a non-zero number of traces, in about four minutes. Nothing
in either moved, which is what makes the run a control rather than a step. The
instruments that did the work were `operations.qnt` read as a specification —
two witnesses in particular — and **four mutation controls**.

**The measurement.** Twenty-four tests: nineteen naming a model claim, five
saying they have none, and none naming neither. That is close to entry 009's
inverted ratio and for the same reason — the leaf's subject is inside the
model's scope. `insert` is the operation `planInsert`, `shiftIds`,
`inv_insertOnlyShifts` and four witnesses were written about, and the fraction
of a leaf a model is worth remains legible from the handoff block before the
leaf starts.

**Caught.** Four.

- **The most valuable thing the model supplied was a witness *pair*, not an
  invariant.** `wit_insertPastTheEnd` and `wit_insertIntoAGap` are one modelled
  outcome discriminated by a predicate — `a.at > maxOrdIn` against
  `a.at < maxOrdIn` — and that shape is what turned one refusal into one refusal
  carrying two pieces of advice. The document states the past-the-end rationale
  ("`append`'s job") and gives the gap case the same sentence, where it is simply
  false: no operation fills a gap, so the honest advice is *by hand*. The
  implementation answer is that the refusal carries the level's greatest ordinal
  so its message can decide which is true. **Superseded by entry 013**: the
  greatest alone decides *which of the two witnesses* is live and cannot justify
  the sentence the gap message then wrote, so the refusal now carries the level's
  whole occupied span. `inv_insertOnlyShifts`, by contrast,
  told the implementation nothing it did not already have from the document —
  the invariant confirmed, the witnesses *designed*.
- **A rule whose payoff is a state nothing observes is testable only if the model
  named the counterfactual instance too.** Highest-first buys distinct ordinals
  at every intermediate state; the other order passes through a duplicate. No
  passing test observes an interruption, so a test of that rule has to reason
  about the plan and read every state a crash could stop at off it — and it is
  worth nothing without a control, because an implementation shifting in *any*
  order passes a lone "every state is distinct" assertion on a well-formed tree.
  `wit_shiftTransientlyDuplicatesAnOrdinal` is live in `lowest_first` and dead
  everywhere else, and that is the control, named in the model: replay the same
  landings the other way and assert some state *is* duplicated. Mutation (a)
  below confirms the pair fires in both directions.
- **A refusal the models exclude spread to a new operation, and the document had
  written it as a list.** *Bytes supplied for parts that make a node are refused*
  was stated of `append` and `append_many`, which were the only operations that
  existed. `insert` creates an entry too, so it inherits the refusal — and the
  bullet had to be rewritten to state the rule over a property (a node is a
  directory and has nowhere to hold bytes, so it binds every operation that
  creates an entry) rather than over an enumeration the next operation falls off.
  Entry 011's routing row already says the exclusion list is the worklist; the
  sharpening is what to *write* when it is: prose covering a modelled
  idealisation must be quantified over the property, because there is no
  typechecker to notice when the list goes stale.
- **The previous leaf had already done this leaf's stated first job, because it
  built a test for an operation that did not exist.** The task file's first
  requirement was to fix the destination check if `interpreter-k10` had left it
  as a snapshot check. It had not: entry 009 records that leaf building `insert`'s
  shift plan *by hand*, in both orders, because no `append` test can tell a fold
  from a snapshot check. So the fold, its refusing twin and the mutation control
  behind them were all in place, and this leaf read them and moved on. H2
  evidence of a specific shape: a leaf that spends effort on the one case its own
  operations cannot reach pays for it in the next leaf, and the model is what
  told it which case that was.

**Missed.** Two, and the second is the more interesting.

- **Subtree preservation's assumed half, again, exactly as entry 003 warned.**
  *A shifted node is one directory rename, with nothing inside it touched* is a
  property of `rename(2)`, below the boundary both models stop at, and
  `operations.qnt` makes it true by construction. The only instrument that can
  hold the library to it is a directory with something in it — one integration
  test, `a_shifted_node_carries_its_whole_subtree_untouched`. What the model
  bought here was not coverage but the **accurate labelling of which half is
  which**, which is what let the checkable half (*the plan names no descendant*)
  be named for what it is instead of borrowing the whole invariant's authority.
- **The create being last is arranged in the model, not claimed by it.**
  `planInsert` appends the create after the shifts, so the order lives in the
  model's construction. `inv_ordinalsDistinctThroughout` *would* catch a
  create-first plan — the new entry and the unshifted occupant would share an
  ordinal in an intermediate state — but the model cannot build one, so the
  invariant never meets it. This is entry 003's first miss in a new dress: an
  invariant strong enough to catch a defect the model's own construction makes
  unreachable is indistinguishable, from the outside, from one that checked it.
  It is only detectable by mutating the *implementation*, which is what mutation
  (b) did.

**The mutation controls.** Four, run against the whole crate suite with
`--no-fail-fast`, each restored before the next:

| mutation | tests failed |
|---|---|
| (a) shift lowest-first | 9 |
| (b) create first, before the shifts | 17 |
| (c) drop the occupant check | 7 |
| (d) shift only the occupant, not the later siblings | 11 |

Every one was caught, and by tests in both halves — the pure algebra tests and
the on-disk ones. (b) is the one worth keeping: it is the only evidence that the
create's position is defended at all, since no model claim reaches it.

**Cost.** About one session, and the model work was reading rather than writing.
Re-running both suites was under five minutes of wall clock and produced no
change; it is bought as the control that separates *the code is wrong* from *the
specification moved*, and it stayed cheap because neither runner had to be
touched.

One friction worth recording for whoever writes the skill: `cargo test` stops
after the first failing test binary, so the first pass of mutation controls
reported only the unit half and silently omitted every integration test. The
numbers above are from `--no-fail-fast`. It is the same failure shape entry 003's
runners exist for — an instrument reporting less than it looked like it had — and
it is one flag, in a tool nobody would think to distrust.

**Counterfactual.** Nothing here would have been caught earlier by a *different*
formalism. The two findings that mattered came from reading a model already
written, three leaves earlier, at the level of its witnesses rather than its
invariants — so the counterfactual is about *how* a model is read on the way in:
**an implementer should read the witnesses before the invariants**. Invariants
say what must remain true, which the document usually already says; witnesses say
which cases are live, which distinctions are real, and — when one is reachable in
only one instance — where the control for a test goes. Entry 004 recorded that
four Alloy witnesses became executable tests and no `check` did; this is the
behavioural half of the same observation, and it now has two leaves behind it.

**Verdict.** Yes, and specifically: reach for the model's witness list when
implementing an operation it covers. It is the part of a model that is written in
the shape a test wants.

### 013 — Integrating the `insert` review (`ordinal-fs-tree`)

**Situation.** `insert-k24` applied `insert-k23`'s one actionable finding.
`Refusal::NoOccupantAtOrdinal` told the caller, of every unoccupied ordinal at or
below the level's greatest, that "something below it and something above it are
occupied". The state it carried — the greatest ordinal alone — proves only the
second half. A hand-edited level holding only ordinal 5, asked for
`Ordinal::FIRST`, reaches that branch with nothing beneath the request at all,
and `Ordinal::FIRST` is not a floor the library enforces, so the level is not
exotic. The review's four other doubts were settled in the implementation's
favour and left alone.

**Formalism.** Both suites re-run as controls. Alloy 20/20, Quint every claim
across all eight instances holding with every witness reached in a non-zero
number of traces — 148 claim results, no failures. **Neither model changed**, and
that is the finding's shape: nothing here contradicted a claim. The instrument
that did the work was `operations.qnt`'s two insert witnesses read a second time
at the level of their **predicates** rather than their names —
`wit_insertPastTheEnd` requires `a.at > maxOrdIn`, `wit_insertIntoAGap` requires
`a.at < maxOrdIn`. Only the second was re-run after this leaf touched the model,
and it touched nothing but a comment.

**Caught.** One, and where it came from is worth more than the fix.

- **The defect was transcribed from the model's prose, not from its predicate.**
  `wit_insertIntoAGap`'s doc comment read "a gap in the middle of a hand-edited
  level"; its predicate is `a.at < maxOrdIn` and nothing else, which is equally
  true of a hole between two occupants and of one below every occupant. Entry
  012 records the witness *pair* as the most valuable thing the model supplied,
  and it was — the two-way discrimination is real. What travelled with it was an
  unchecked sentence sitting inside a checked file, and the refusal message
  inherited its authority for free. The comment is now tightened to say what its
  predicate proves and what prose derived from it may therefore claim; that
  edit is why the Quint suite was run twice.
- **The fix is more carried state, not softer prose.** The refusal now carries
  the level's whole occupied **span** — least and greatest — as one
  `Option<(Ordinal, Ordinal)>` rather than two `Option`s that could disagree,
  since this refusal exists precisely because state carried for a message can be
  wrong. Three arms follow, each provable from the span: past the last sibling
  (`append`'s job), a hole **below** the least (no lower neighbour named, same
  by-hand conclusion, because `append` would take `greatest + 1` and not this
  ordinal), and a gap **strictly between** the two — where both neighbours are
  proven to exist, because an ordinal occupied by nothing is neither the least
  nor the greatest.

**Missed.** The five controls that existed could not see this, and the reason
generalises: both gap controls *arranged* an interior hole (ordinal 1 below,
ordinal 5 above) and then asserted only that the message contained `gap` and
`by hand`. A fixture that satisfies the stronger prose is invisible as evidence
for the weaker predicate — the same shape as entry 005's *a test that names a
claim and checks a weaker property*, one level up: here the **fixture**, not the
assertion, was doing the overstating. The insert account is now twenty-six tests,
nineteen naming a model claim and seven saying they have none; both new tests are
in the second group, and they say why — the model distinguishes this case from
past-the-end and does not distinguish it from an interior gap, so the message's
three-way split is the library's own.

**The mutation control.** One, and the new tests need it more than most because
they assert a **negative** — `!contains("something below it")` — which a typo
would let pass forever. Disabling the leading-hole arm so a leading hole falls
back to the gap message failed exactly the two new tests, one in each half of the
suite, and nothing else: 66/1 in the unit binary, 9/1 on disk. That is
simultaneously the proof that the new tests discriminate and a reproduction of
the review's complaint about the old ones.

**Cost.** Well under a session; the fix is twenty lines and the reconciliation
across `plan.rs`, `ARCHITECTURE.md`, `operations.qnt`'s comment and this log was
most of it. Two Quint runs at roughly four minutes each, the second bought by
choosing to edit the model comment rather than only the code that read it.

**Counterfactual.** **A message may claim only what the predicate that selected
it proves.** Entry 012's counterfactual — *read the witnesses before the
invariants* — stands, and this is its correction: a witness's predicate is a
**discriminator, not a characterisation**. It tells you this case is not that
one; it does not tell you what this case looks like, and the picture in the
reader's head (or in the comment beside it) fills the gap silently. The free
check, costing no formalism and no run, is to take each clause of an error
message and name the carried value that proves it — the clause with no value
behind it is the defect. That check applied when the message was written would
have caught this one leaf earlier, and it is worth applying to `promote`'s and
`rewrite`'s refusals before they are written rather than after.

**Verdict.** Yes to reading witnesses, with the sharpening above. And a second,
cheaper standing habit: prose inside a checked model file is **not checked**, so
when an implementation transcribes a model, transcribe the predicate and read the
comment as commentary. The tightened comment is the durable half of this fix; the
code would have been written correctly the first time if it had been there.

### 014 — Implementing the one operation that can damage a tree (`ordinal-fs-tree`'s `promote`)

**Situation.** `promote-k12`: turn a leaf into a node, with the node's parts
supplied by the caller, moving the leaf's bytes verbatim into the new node's
distinguished child and keeping the leaf's own ordinal and key — optionally
creating a first child in the same unit. It has the most that can go wrong of
any operation here, and it is the only one by which the library can damage a
tree it was handed: its single undo is *remove the node just created*, so a
rollback failing there leaves a leaf and a node sharing an ordinal and a key.

**Formalism.** None written. Both suites re-run as controls first: Alloy 20/20,
Quint every claim across all eight instances with every witness reached in a
non-zero number of traces. **Neither model changed**, and neither contradicted
the implementation. The instruments that did the work were `operations.qnt` read
as a specification — its `planPromote` chain, four witnesses and one long
explanatory comment — and **six mutation controls**.

**The measurement.** Twenty-two tests: seventeen naming a model claim, five
saying they have none, and none naming neither. The ratio is entry 012's again
and for the same reason — `promote` is squarely inside the model's scope. What
the count hides is *where* the five sit: four of the five are refusals or
arithmetic the models exclude by design, and the fifth is the control on a
document sentence.

**Caught.** Four.

- **The document states a refusal in two halves and one half cannot be
  asked for.** *A node is already a node, and a distinguished child has no
  ordinal to carry across; both are refused.* But an operation names its target
  by key and a distinguished child carries none — so `by_key` cannot answer with
  one, and neither can the model's `resolve`, whose `idsWithKey` filters on
  `isPositioned`. The model does not contradict the document; it declines to
  support half of it, silently, by having **no witness** for a case no argument
  reaches. This is entry 013's counterfactual moved one step earlier: 013 asks
  what a message may *claim*, and this asks what a refusal may *enumerate*. The
  fix is the same shape — state the check over what it reads (the species) and
  say which case actually arrives — and the test that holds it is a control on
  the document rather than on the code: the example tree holds two distinguished
  children and `by_key` answers with neither, for any key.
- **The model's longest explanatory comment was this leaf's hardest arithmetic.**
  `inv_freshKeysAreFresh` carries a paragraph insisting the claim is about
  *allocation*, not *creation*, because `promote` creates a directory carrying a
  key that already exists. The implementation consequence is exact and easy to
  get wrong in the invisible direction: the node consumes **no** key, so a first
  child takes `freshKey`, not `freshKey + 1`. A promotion that had "spent" a key
  on the node would leave a permanent hole in the key sequence and pass every
  invariant, because a skipped key breaks nothing — it is not a duplicate, and
  density is a property of ordinals rather than keys. Mutation (d) is the only
  thing in the crate that fails on it. Entry 009's routing row — *an unenforced
  invariant hiding in plain sight → the model's explanatory comments* — now has a
  second instance, and this one is stronger: the comment did not merely explain
  the claim, it was the only statement of the arithmetic anywhere.
- **A forced order is not an ordering rule, and the test should assert the
  forcing.** `insert`'s highest-first rule has a counterfactual instance
  (`lowest_first`) that makes the alternative real and gives a test its control,
  which is entry 012's headline. `promote` looks like the same shape — create,
  then move, and the intermediate state is exactly what the witnesses are about
  — but it has **no alternative**: the move lands in `Level::Created(0)`, the
  level the create produces, so the two effects reversed is not a worse plan but
  an unrunnable one. So the test asserts the *forcing* — that the second effect's
  level is the first effect's product — instead of asserting an order that
  nothing could have chosen differently. Reading the model for a counterfactual
  instance and finding none is what made the difference visible; without that
  question the test would have asserted the order and looked identical.
- **A recovery instruction is worth printing only if it describes the state it
  will be read in, and that is checkable.** `Error::FailedPartiallyRolledBack`
  already said *a node and a leaf sharing an ordinal and a key, with the node
  holding no distinguished child, is an interrupted promotion; removing either
  half resolves it.* Applying entry 013's habit — name the fact behind each
  clause — turns that into a test shape rather than a review: drive the state
  with the fault seam, assert each clause of the advice **as a fact about the
  directory**, then follow the advice and assert the tree reads cleanly again.
  Three clauses, three assertions about disk, one execution of the remedy. The
  message was already right; what was missing was anything that would notice if
  the operation's shape drifted away from it.

**Missed.** Two.

- **The most consumer-visible property of this operation has no model claim at
  all.** *The leaf's bytes move verbatim* is what a caller actually cares about,
  and content is unmodelled in both models by design — `operations.qnt`'s handoff
  block says so. The model can say the leaf's *object* becomes the distinguished
  child; only a file with something in it can say the bytes survived. This is the
  third leaf running to spend its integration tests on the excluded list, which
  is entry 011's routing row holding up well. What is new is the ratio: for
  `promote` the excluded property is not a corner, it is the point of the
  operation.
- **The model's rarest reachable state was the implementation's cheapest test,
  and the model's own cost figure is misleading about that.** Entry 003 records
  the failed-rollback duplicate key as reached in 0.07% of traces and needing its
  own sample budget to find at all. Reaching it in the implementation took
  `Faults::at_effect_and_unwind(1, 0)` — one line, deterministic, a few
  milliseconds. Rarity under random simulation measures how hard a state is to
  *stumble into*, and says nothing about how hard it is to *steer into* through
  an injected fault. Reading the 0.07% as "this will be expensive to test" would
  have been exactly wrong, and it is the kind of number that invites the
  inference.

**The mutation controls.** Six, run against the whole crate suite with
`--no-fail-fast`, each restored before the next:

| mutation | tests failed |
|---|---|
| (a) the node takes a fresh key instead of the leaf's | 10 |
| (b) the node takes the level's next ordinal instead of the leaf's | 8 |
| (c) drop the parts-imply-a-node refusal | 2 |
| (d) the child's key steps past a key the node did not take | 3 |
| (e) never move the leaf in — the content is silently left behind | 10 |
| (f) report the domain refusal before the not-a-leaf one | 1 |

Every one was caught. (d) and (f) are the two worth keeping: each is caught by
tests that exist for no other reason, and both are defects that leave a tree
every stated invariant holds of — a skipped key and a differently-ordered pair of
true refusals. (e) is the control on the missed property above: it fails ten
tests, and the four that matter are the on-disk ones, since the algebra tests
notice only that an effect is absent while the integration tests notice that a
consumer's file is gone.

**Cost.** Under a session. Both suites re-run in parallel, Alloy in about twenty
seconds and Quint in roughly four minutes, and neither moved — bought as the
control separating *the code is wrong* from *the specification moved*. The six
mutations were a scripted loop over one file, which is what made six affordable
where entry 012 ran four; the loop is worth keeping, and its one trap is that
`cargo test`'s ordinary failure line contains the word `error`, so a naive
"did it compile?" check reads every caught mutation as a build failure. That is
the same instrument-reporting-less-than-it-has shape as entry 012's missing
`--no-fail-fast`, arriving in the harness this time rather than in the tool.

**Counterfactual.** **Ask of every enumerated case whether an argument can
produce it.** The document's *both are refused* was written when nothing named a
target yet; by the time targets were keys, half of it was unreachable, and
nothing noticed because prose has no typechecker and an unreachable refusal
costs nothing until someone implements it. The model already carried the answer
— no witness, because no reachable case — but a **missing** witness is invisible
in a way a failing one is not: `run-quint.sh` reports what it was asked about,
and nobody asks about a case they think is live. So the cheap check is the
enumeration itself: for each case a refusal names, name the argument that reaches
it. It costs no formalism and no run, it is the same free check as 013's
*name the value behind each clause*, and it is the one that would have caught
this at the time the sentence was written.

**Verdict.** Yes to the model, and specifically to its **comments** for this
kind of leaf. `promote`'s witnesses told the implementation what states to
expect, which entry 012 already predicted; what the witnesses could not tell it
was the arithmetic, and the one paragraph of prose inside `operations.qnt` did.
Second standing habit, from the same place: when a model records a state as rare,
that is a fact about its simulator, not about your test.

### 015 — Integrating the `promote` review (`ordinal-fs-tree`)

**Situation.** `promote-k26` applied `promote-k25`'s one finding. The trait
bounds `EntryName::Parts` by `Clone + Eq` and nothing more, so a domain may
compare a leaf's parts equal to a node's while `positioned_species` still tells
them apart — a lawful equivalence, breaking no obligation and passing every
conformance check. Occupancy compared whole `NameView`s, and a promotion is the
one operation whose new name deliberately reuses the old one's ordinal and key,
so for such a domain the promoted node's destination looked occupied by the very
leaf it replaces and every valid promotion was refused as
`DestinationOccupied`. The review's other five doubts were settled in the
implementation's favour and left alone.

**Formalism.** Both suites re-run as controls: Alloy 20/20, Quint 148 claim
results across all eight instances, every witness reached. **Neither model
changed except in its comments**, and that is the entry's whole shape — not
because nothing contradicted a claim, but because *neither model can pose the
domain that does*. `structure.als` makes `Parts` an opaque atom and
`operations.qnt` makes it an `int`; in both, equality is identity, so *equal
parts imply equal species* is true by construction and asserted nowhere. The
instrument that found it was an adversarial reader holding the trait's stated
bound (`Eq`) against the model's representation of the same type (an atom) —
entry 005's shape, one level down: not a claim checked weakly, but a claim the
model gets for free from a representation the target language does not share.

**Caught.** One, and the correction is at the seam rather than in `promote`.

- **Name identity is the view *and* the species.** `EntryNameExt::same_name` is
  a new derived reading beside `triple` and `species` — sealed and blanket, so
  no domain can override it — and both halves of the occupancy fold now use it:
  the snapshot's children and the destinations earlier effects in this plan have
  already taken. The alternative was an eighth obligation on the domain (`a == b`
  implies equal species) and it was rejected on the conformance kit's own
  discipline: no sample of parts can *exercise* a congruence, because the parts
  that would demonstrate it exist only in a domain that violates it. The kit
  reports an unexercised obligation as a finding, so every well-behaved domain
  would have failed conformance in order to state a property only a misbehaving
  one can show. An obligation a test kit can never mark as reached is a comment
  with a `pub` in front of it.
- **The false claim it invalidates was a comment in the implementation.**
  `ops::promote` said the first destination cannot collide "because the node's
  parts differ from the leaf's". They differ as *values*; the guard compared
  them under the domain's `Eq`. The comment now names what actually separates
  them and what compares it.

**Missed.** The conformance kit does not detect the domain — deliberately, and
the reasoning is above — so the only instrument that reaches it is a test with
an adversarial domain in it. Nothing generalises that: the kit's samples are
values, and this is a property of an equivalence *relation* over values that a
finite sample cannot distinguish from identity. What the kit does guarantee is
the other end: a domain whose leaf and node spellings coincided fails
canonicity, which is what makes *not the same name* mean *two filenames*.

**The measurement.** Two tests, both saying they name no model claim, and both
saying why — neither model can pose the domain. That puts `promote`'s account at
twenty-four tests: seventeen naming a claim, seven saying they have none.

**The mutation controls.** Two, because the fix has two halves and a single
control cannot tell which one the test reached:

| mutation | tests failed |
|---|---|
| (a) drop the species from `same_name`, leaving the view | 2 — both new tests, nothing else |
| (b) `same_name` correct, occupancy back to `view() == view()` | 1 — the on-disk promotion only |

(b) is the one worth keeping. It proves the on-disk test reaches the *wiring* and
not merely the new method, which is the failure mode a mechanism-shaped fix
invites: a correct helper nothing calls passes any test written against the
helper.

**Cost.** Well under a session. The fix is roughly fifteen lines; the adversarial
domain is ninety, and writing it *was* the verification — a reference domain
cannot host this defect, so there was no cheaper reproduction. Reconciliation
across `name.rs`, `plan.rs`, `ops.rs`, `conformance.rs`, `ARCHITECTURE.md`, both
models' comments and this log was the bulk of it. Alloy about twenty seconds,
Quint about four minutes, both bought by comment edits rather than by any doubt
about the claims.

**Counterfactual.** **An opaque sort in a model is compared by identity; the
target language compares it by whatever bound the interface states.** Every
model in this workstream has such sorts — `Parts`, `Key`, `Label`, `Attrs` — and
each one silently promises the implementation a congruence it never asked the
domain for. The free check, costing no formalism and no run, is to list the
model's opaque atoms beside the trait bounds on the types they stand for, and
ask of each: *is the language's comparison as fine as the model's?* `Eq` is an
equivalence relation, `Ord` a total preorder, `Hash` neither — none of them is
identity, and the model's is. That check applied when the seam was written would
have caught this before `promote` existed, and it is the structural sibling of
011's *the exclusion list is the worklist*: what a model represents as
structureless is exactly where the implementation carries structure the model
cannot see.

**Verdict.** Yes to the adversarial reader on a trait bound, and a standing habit
for this workstream: when a model's abstraction and the language's bound differ
in *strength*, the implementation must close the gap itself rather than push it
onto the consumer — a check the consumer cannot be tested against is not an
obligation.

### 016 — Implementing the operation the model had already finished (`ordinal-fs-tree`'s `rewrite`)

**Situation.** `rewrite-k13`: replace an entry's parts, keeping its ordinal, its
key and its species. One rename. It is the last mutation, so its landing is what
`h3-probe-k14` was waiting on, and — with no removal operation
(`docs/adr/entries-are-never-removed.md`) — it is also how a domain retires an
entry, which is why it matters more than its size suggests.

**Formalism.** None written. Both suites re-run as controls: Quint, every claim
across all eight instances with every witness reached in a non-zero number of
traces; Alloy 20/20. **Neither model changed, and neither contradicted the
implementation** — the third leaf running. The instruments that did the work were
six lines of `operations.qnt` (`planRewrite`), its three claims
(`inv_rewriteKeepsPlace`, `wit_rewriteToSameParts`,
`wit_refusedRewriteSpeciesChange`), and **five mutation controls**.

**The measurement.** Fourteen tests: ten naming a model claim, four saying they
have none, and none naming neither. The highest claim ratio and the smallest test
count of any leaf here, and the two go together — this operation is entirely
inside the model's scope, so what is left over is small and is exactly the
model's own exclusion list.

**Caught.** Three, and none of them is a defect. That is the finding.

- **A six-line model function settled the one question the leaf brief left
  open.** The brief asked whether the species check would end up being *the same
  code* as `promote`'s, and told this leaf to say why if not. `planRewrite`
  answers structurally: `speciesOfParts(p) != speciesOf(n)` beside `planPromote`'s
  `speciesOfParts(p) != Node`. The same comparison with a different right-hand
  side — one a constant, one read off the target — which is why they are not
  shared code and should not be. The only thing extractable is the trait call
  itself, and a function wrapping `N::positioned_species(parts) == x` would be an
  abstraction shallower than the expression it hides. The model did not decide
  this; it made the decision one line of reading instead of one of judgement.
- **The refusal's payload followed from the model by subtraction.**
  `PromotePartsNotNode` carries only a key, because with two positioned species
  *not a node* determines *is a leaf*. The same arithmetic says
  `RewriteSpeciesChange` needs the **entry's** species and only that: the supplied
  one is then determined, and carrying both would be two fields that can
  disagree — `NoOccupantAtOrdinal`'s recorded lesson (entry 013) reappearing on a
  refusal a tenth its size. The model supplies only that `speciesOf` is total over
  a two-valued domain; the conclusion is one step from there, and mutation (e) —
  carry the supplied species instead — is what holds it.
- **The property this operation is named for was discharged before the operation
  existed.** `wit_rewriteToSameParts` requires a rewrite to the parts an entry
  already carries to succeed. Both halves were built by the interpreter leaves:
  the algebra's — occupancy excluding the object being moved, which is why
  `Effect::mover` exists at all — and the interpreter's — the same-path
  short-circuit, which also registers no undo, since an `Undo::Restore` onto its
  own occupied path would turn a clean rollback into `FailedPartiallyRolledBack`.
  Each already had a test naming this witness. Adding `rewrite` turned two
  anticipated properties into two live ones and required **no code**. This is the
  clearest H2 evidence in the log so far: a specification that names an
  operation's edge case makes the layer beneath it build for that case, and the
  operation then lands as a rename.

**Missed.** Three, and all three are on the list the model publishes about
itself.

- **The subtree assumption, and this time it is the operation's main risk.**
  `operations.qnt`'s handoff names *a directory rename carrying its subtree* an
  assumption rather than a property: an entry references its parent by a stable id
  there, so it is true by construction. For `insert`'s shift the exposure is low —
  a shift is `compose` with a new ordinal and can disturb nothing else. For
  `rewrite` the caller hands in **new parts**, so the rename that must carry a
  subtree is precisely the one a reader could plausibly imagine as a rebuild. Only
  a real directory can say it did not. Fourth leaf running to spend its
  integration tests on the excluded list.
- **One modelled outcome, two messages, and the two directions are not
  symmetric.** `RefusedRewriteSpeciesChange` covers leaf→node and node→leaf alike,
  and only one has a remedy: `promote` turns a leaf into a node and moves its
  content rather than discarding it, while *nothing* turns a node into a leaf —
  its children would have nowhere to go, and entries are never removed. A single
  message for the single modelled outcome would have been advice that fails when
  taken, half the time, and no claim could notice. Entry 013's habit is what
  caught it; the model contributed the outcome and nothing about what may be said
  inside it.
- **Content, again — but the instrument got sharper.** No model reaches *the bytes
  are unchanged*, and content equality is a weak stand-in for it: bytes read and
  written back identically compare equal. The **inode** does not. Two tests here
  assert inode identity across the rename, for a leaf and for a node's grandchild,
  which says the library did not read and rewrite rather than that the result
  happened to match. It is available only because the crate is Unix-only by
  design — the same platform assumption that is invisible in the interface is
  visible in the test suite.

**Cost.** The smallest of any implementation leaf here. Reading — the brief chain,
the architecture's `rewrite` row and refusals, `planRewrite` and its three claims,
and the prior leaves' settled-since notes — was the bulk of the session; the
implementation is about forty lines of code under rather more doc comment, and
fourteen tests. Re-running both suites as controls was the longest single step
and found nothing, which is what a control is for. Five mutation controls, each a
one-line edit and a test run: composing with `Ordinal::FIRST` (4 failures),
landing the rename in the root (1), dropping the species check (2), dropping
occupancy's mover exclusion (2, one of them a test the interpreter leaf wrote),
and carrying the supplied species in the refusal (2). Nothing had to be learned
first.

**Counterfactual.** Nothing here would have been caught earlier, because nothing
was caught — so the counterfactual is about the no-op instead, and it is the
strongest one available. Had `rewrite` been implemented **before** the
interpreter, the same-path rename would have surfaced as a defect at the boundary
between two layers that are each correct alone: the algebra proves a plan
applicable, and the layer applying it refuses that plan. Layer-crossing defects
are expensive exactly because neither side looks wrong. The witness put the fix in
both layers before either had a caller, and the ordering that did it —
interpreter first, its operations after — was `library-k6`'s decomposition, taken
from the model's shape rather than from the document's operation table.

**Verdict.** Reach for it again, and note that the routing lesson is about *when*
a model pays rather than whether. A model written per operation, before
implementation, turns an operation of this size into transcription; what is left
over is the model's own exclusions (bytes, message text) and its own assumptions
(a directory rename carries its subtree). Three leaves running, those are where
the tests go — which is now predictable enough to plan a leaf's test budget from
the handoff block before writing a line.

### 017 — The pre-registered model-versus-prose probe (`ordinal-fs-tree`'s `insert`)

**Situation.** `h3-probe-k14`: test H3 — *an LLM given a checked formal model
produces a more faithful implementation than one given prose of equivalent
length* — deliberately rather than by impression. The probe, its arms, its
measure, its prediction and its falsification condition were written by
`library-k6` before any implementation leaf ran, and are in the leaf body
unrevised. `insert` was the pre-registered probe; the fallback (`promote`) was
not needed, because the result is not degenerate.

**Formalism.** None written, and that is the point: the instrument here is a
**controlled comparison**, not a model. Arm A is `insert` as `insert-k11`
produced it, model-led. Arm B is a fresh context given the `ARCHITECTURE.md`
sections covering `insert` and a doc-stripped listing of the crate surface, and
nothing else — no `operations.qnt`, no `structure.als`, no tests, no arm A, no
`docs/formalism-findings.md`. Both are scored by `insert-k11`'s own suite,
unchanged.

**The numbers.**

| | arm A (model-led) | arm B (prose-only) |
|---|---|---|
| insert tests run | 26 | 26 |
| tests failed | 0 | **4** |
| distinct claims touched by a failure | — | 2 (`wit_insertPastTheEnd`, `wit_insertIntoAGap`) |
| **model claims violated, adjudicated** | 0 | **0** |
| stated refusals missing | 0 | 0 |
| stated refusals wrong | 0 | 0 |
| compiled as submitted | yes | **no** |

The 26 tests are 18 naming a model claim, 7 saying they have none, and **1
naming neither** — `the_reports_paths_are_in_the_order_the_effects_landed`,
which names `Report::paths()`'s contract, a thing no model holds. **Counted by
reading the suite, never by adding up the per-leaf numbers** — the standing rule
of the increment, and it is not pedantry: an earlier leaf reported fourteen
tests as ten-naming-a-claim plus four-saying-they-have-none, and a re-read of
the forty-two then in the suite turned up three naming *neither*. A sum of two
labels cannot see the third.

**Input sizes, in lines, since the hypothesis says *of equivalent length*.**
Arm B: 372 lines of `ARCHITECTURE.md` prose + 195 lines of API listing = **567**.
Arm A: the whole of `ARCHITECTURE.md` (957) + `CONTEXT.md` (221) +
`operations.qnt` (1349, of which ~160 are `insert`-relevant: `shiftIds`,
`shiftEffect`, `planIsApplicable`, `guardDestinations`, `planInsert`,
`inv_insertOnlyShifts` and six witnesses) + `structure.als` (571) + a 106-line
task body + the crate as it then stood. **The two inputs are not equivalent and
not disjoint: arm A's input strictly contains arm B's.** This probe therefore
compared *model + prose* against *prose*, which is not the comparison H3 states,
and no design available to this leaf could have fixed it — arm A's session had
already run, and denying it the specification of record was never an option.

**Caught.**

- **The prediction failed on all three of its points, and that is the result.**
  It predicted the prose arm would produce a working happy path and fail on at
  least two of: the shift order; the sequential destination check; the gap
  refusal. Arm B got all three. It shifted **highest-first**, and stated the
  intermediate-state reason for it — not collision — in its own comment. It
  left the destination check entirely to `Plan::guarded`, correctly, citing *the
  plan is checked against itself, in order*. It refused the gap, carried the
  span, and rendered four distinct messages where arm A renders three-and-a-half.
- **The suite is live, not vacuous.** A mutation control — arm B's shift
  replayed lowest-first, a two-line edit — fires **7 further tests** (5 unit, 2
  on-disk), including both halves of the ordering pair
  (`the_shift_runs_highest_ordinal_first`,
  `the_same_shifts_run_lowest_first_pass_through_a_duplicate_ordinal`). So arm
  B's 22 passes are earned. Without this step, *arm B violated no claim* and
  *the suite cannot see a violation* are the same observation.
- **All four of arm B's failures are on assertions arm A wrote, not on outcomes
  a model states.** Each of the four asserts a literal substring of a refusal
  message — `` `append`'s job ``, `something below it`. In every one of the four,
  the `assert_eq!` on the refusal **value** — the variant and its carried span —
  ran first and passed. Arm B's messages send the reader to `append` for both
  past-the-end and empty-level, and give the interior gap and the leading hole
  different advice, which is what the claims and the document ask for; they say
  it in other words. **The pre-registration's own defence of the measure — that
  every test cites a claim `library-k6` fixed before either arm — is half
  true.** The claims predate both arms. The assertions do not, and the assertions
  are what a second implementation actually runs against.
- **Arm B's one real defect is a shape the log already recorded.** It encoded the
  span as two independent `Option<Ordinal>` fields rather than one
  `Option<(Ordinal, Ordinal)>`, and its `Display` match is non-exhaustive: the
  type admits `(None, Some(_))`, which its own comment says "cannot arise". The
  crate does not compile until an arm is added. That is entry 013's lesson —
  *two fields that restate each other are two fields that can disagree*, which
  is exactly why `NoOccupantAtOrdinal` carries one field — arriving as a
  compiler error rather than as a wrong message. Neither model reaches it: a
  refusal's payload is unmodelled in both.
- **Refusal precedence is observable and no arm's authority.** Arm A checks the
  occupant before the content-for-a-node refusal; arm B checks content first.
  `planInsert` fixes only *missing → not a node → no occupant*, content being
  unmodelled, and the document fixes nothing. Arm B flagged this itself,
  unprompted. A call that is doubly bad gets different refusals from the two
  arms, and nothing in the specification says which is right.

**Missed.**

- **The asymmetry that favours arm A, stated in the leaf and confirmed.** The
  probe was pre-registered, so the plan that cut the increment listed it among
  the leaves — as *the pre-registered model-versus-prose experiment*, without
  naming what it would measure or which arm was which. Arm A therefore ran under
  a diffuse awareness that such an experiment existed somewhere in the subtree;
  arm B ran under none. More decisively, **the measure is arm A's own
  artifact** — arm A cannot fail a suite it wrote, so the *0 vs 4* headline is
  not a comparison of two implementations against a neutral instrument. It is a
  comparison of one implementation against another's tests. The adjudicated
  0 vs 0 is the number that survives that, and it is the one the verdict rests on.
- **The comparison H3 states was never run and cannot be run retrospectively.**
  See *Input sizes*. A real test of H3 needs both arms cut from the same brief,
  at the same time, neither owning the measure, and a suite written by a third
  party from the claims alone.
- **Nothing was learned about the operations the probe did not cover.** One
  operation, one prose arm, one sample. `promote` — the fallback — would have
  been the harder case, because its transient invariant break is stated in the
  prose as a consequence rather than as a rule.

**The decisive confound, and the thing worth keeping.** The prose arm B read is
**model-derived prose**. `ARCHITECTURE.md`'s *Why the shift runs highest-first*
says collision is *not* the reason and the intermediate state is — and entry 003
records that the document's original stated reason was wrong and that
`operations.qnt` is what found it. The gap half of the occupant refusal exists
in the document because `wit_insertIntoAGap` surfaced a case the document's
rationale did not cover. The span the refusal carries, and the three messages it
separates, come from `insert-k23`'s review of arm A. **Every point on which arm
B succeeded is a point the model had already corrected in the prose.** So the
right reading is not *the model was unnecessary* but *the model had already been
spent, upstream, and its payment was banked in the document*. Read that way this
probe is evidence for H2, not against it, and it relocates H3's question from
*model or prose?* to *has this prose been through a model yet?*

**Cost.** One session. Assembling arm B's inputs and the isolation conditions
(~30 min of reading and extraction), one subagent run (93 s, 3 tool calls: two
reads and one write — consistent with its report that it read nothing else), a
scoring scaffold that is the crate copied out of the workspace with arm B's
function and refusal spliced in and the refusal's *name* mapped field-for-field
in the tests, and one mutation control. No tooling to learn. The scaffold is the
only fiddly part, and it exists because `Plan`, `Effect` and `Decision` are
`pub(crate)`: the pre-registration's "the crate's public types and the `insert`
signature" is literally insufficient to compile a plan, so arm B was given a
doc-stripped internal listing as well. That is a deviation from the
pre-registration, forced by mechanics, and it hands arm B `Plan::guarded`
ready-made — which is one of the three things the prediction expected it to get
wrong.

**Counterfactual.** The cheap instrument that would have made this probe
decisive costs nothing and was available from the start: **have the measure
written by a session that implements neither arm**, from the model's claims
alone, before either arm runs. `library-k6` fixed the *claims* in advance and
believed that was enough; it was not, because a test is a claim plus an
assertion, and the assertion is where an author's own wording gets baked in. The
second, and cheaper still: **record whether the prose an arm reads has already
been through a model.** Had that been asked at pre-registration time, this probe
would have been designed against `ARCHITECTURE.md` *as of entry 001* — the
prose-review baseline, before Alloy and Quint corrected it — and it would have
tested what H3 says.

**Verdict on H3 — not supported by this probe, and the probe is why.** Arm B,
given only model-corrected prose, violated **zero** model claims on a suite with
a demonstrated live control, and failed only on literal wordings of its scorer's
own messages. H3 as stated — *a model beats prose of equivalent length at the
implementation site* — does not survive that, and the honest entry says so
rather than rescuing it. What replaces it is narrower and better supported by
the whole log: **a model earns its keep upstream of the implementation, in the
prose it corrects, and a session implementing against already-corrected prose
gets most of the benefit without reading the model.** That is consistent with
every incidental H3 observation here — 012's *the invariants confirmed what the
document already said; the witnesses are what taught it something* and 016's
*the model turned a judgement call into a reading* — and it is a claim the
distilled skill can act on, where H3 as written was not.

Arm B's code was throwaway by construction and has been deleted; it was written
into a scratch path outside the crate and never entered the repository. Its
decisive content is quoted above.

### 018 — Designing an interface neither model can reach (`ordinal-fs-tree`'s CLI)

**Situation.** Settling what the CLI *is*, now that there is a library for it to
expose: generic dispatcher or concrete binary, which verbs, what each prints and
to which stream, how a refusal reaches the operator, and what is out. The
deliverable is `docs/ordinal-fs-tree/CLI.md`; `cli-k16` builds from it.

**Formalism. None — and the decision not to reach for one was made by an existing
routing rule rather than by impression.** Entry 009's rule is *how much will a
model be worth to this leaf? — the fraction of the leaf inside the model's stated
scope, legible from the handoff block before the leaf starts.* Every question
here is about argv strings, output streams, exit codes and help text.
`operations.qnt`'s handoff block names strings and bytes as excluded by design;
`structure.als` holds no strings either. Predicted coverage: zero. The prediction
held — nothing in the finished document could have been stated as a claim in
either file. Applying entry 003's mechanical test gives the same answer from the
other side: count the states the property mentions, and a question about what a
verb prints mentions **none**, which is neither structural nor behavioural.

**Caught.** Three things paid, and none of them was a model run.

*The models' idealisation lists, read as a worklist* — entry 011's row — located
the CLI's own refusals before any were written. Content and non-UTF-8 names are
outside both models by design, and both are exactly where the CLI has to decide
something the library cannot decide for it.

*A modelled obligation discharged by an interface.* **The species follows from
the parts** is a `structure.als` assumption and a trait signature; it became a
verb-grammar rule — **a noun prefix appears exactly where the operator chooses a
species, and nowhere else** — and `Refusal::ContentForANode` fell out as
unreachable, because no verb that composes module parts accepts bytes. That is
the same move as folding three `Option` accessors into one view (entry 002's
counterfactual), one layer further out: an argument surface can discharge an
obligation the way a type can.

*A probe instead of an argument.* The load-bearing placement claim — *a binary
under `src/bin/` trips the no-filesystem guard, because it must name
`ordinal_fs_tree::fs::read` and the guard refuses the identifier `fs`* — was
settled by planting `src/bin/probe_tmp.rs` and watching
`the_algebra_cannot_reach_the_filesystem` fail naming its own line, then moving
the same file outside `src/` and watching it pass. Ninety seconds, against a
reading of a `proc-macro2` token scan and its one carve-out.

**Missed.** Nothing, and the word is doing no work here: no model was run, so
there is nothing to compare an answer against. The honest negative is the one
above — the routing rule said *neither* before the leaf started, and re-reading
the finished document found no claim either file could have carried.

**Cost.** No tooling. One reading of `operations.qnt`'s handoff block to route,
one probe, and the ordinary reading of `ARCHITECTURE.md`, `CONTEXT.md`, the two
ADRs and `linkuistics:cli-tool-design`. The routing decision itself was the
cheapest part of the leaf and is the part worth reproducing.

**Counterfactual.** Nothing would have caught this earlier, because there was
nothing to catch — which is the point of recording it. What is worth extracting
is that **the leaves where a formalism is the wrong instrument are identifiable
before the leaf starts, by the same reading that tells a leaf how much a model
will be worth to it.** A log with no such entries would let the distilled skill
imply that reaching for a formalism is always at worst neutral; it is not, and
the evidence that it is sometimes plainly wrong has to be as recorded as the
evidence that it is sometimes decisive.

**Verdict.** No — and the routing said so in advance, which is the H1-shaped
result here: a practitioner could tell *before choosing* that they were holding
neither kind of question. H2 is untouched: the model led nothing here because
there was nothing for it to lead.

### 019 — Building the interface neither model can reach (`ordinal-fs-tree`'s CLI)

**Situation.** Implementing `docs/ordinal-fs-tree/CLI.md`: the `syllabus` binary,
twelve verbs, its contract tests, and the six things that document met and
deliberately did not resolve. The leaf that closes increment 1, and the first
time the seam is driven by code written against a real terminal rather than
against the architecture document.

**Formalism. None run, and entry 018 had already said so** — this leaf is the
other half of that prediction, and it is worth recording as the *confirmation*
rather than as a second no. Predicted coverage was zero and the measured account
is **31 tests, 8 naming a model claim, 23 saying they have none, none naming
neither**. The eight that do name one are not about the CLI: each checks that a
*modelled* outcome survives the trip out through argv and back through stdout,
which the on-disk suites had already discharged in-process.

**Caught.**

*A design document's reachability table predicted a suite that did not exist
yet.* `CLI.md`'s *Which refusals this CLI can reach* was written before any
argument parsing existed, marking six refusals reachable and seven not. Writing
the contract tests was reading that table: every refusal it marked reachable has
a test naming its witness, every one it marked unreachable has none, and **not
one turned out to be reachable after all**. That is entry 004's *witnesses are
the test suite* one layer further out again — a table over `wit_refused*` is a
test plan, and building it at design time is what makes the implementation leaf's
suite a transcription rather than an invention.

*The library's refusals speak the library's vocabulary, and a domain cannot
change them.* `syllabus lesson-add 4 sections` answers **"the entry with key 4 is
a leaf, which holds nothing. Children go in a node — promote it first, or name a
node."** In this domain there are no leaves and no nodes; there are lessons and
modules. `Error::Malformed` and `Error::Reserved` carry `EntryName::Err`, so a
*parse* failure reaches the operator in the domain's own words — the design went
out of its way to arrange that — but `Error::Refused` carries `Refusal`, which is
not generic over `N` and holds no domain value at all. So the half of the error
surface a conforming tree actually meets in normal use is the half the domain
cannot speak for. **Accepted rather than fixed**, and the reasoning is the
finding: the library's words are *true* (a leaf is a regular file), a `Refusal<N>`
would put a second domain-facing rendering into a seam whose ADR says the name
type is the only one, and a CLI that re-words the condition itself is exactly
what entry 017 measured going wrong. What is lost is stated instead of hidden.

*The claim label is prose, so a count of it measures the counter.* The crate
carries 204 tests and the convention is *each test names the claim it discharges,
or says it has none*. A regex over the claim identifiers and the phrase *no model
claim* leaves 22 unclassified — and **every one of them is labelled**, in a form
the pattern did not hold: *Discharges the assumed half of subtree preservation
under shift*, *Discharges `RefusedDestinationOccupied` against the snapshot
itself*, *Discharges every trait obligation Alloy states*, and a handful whose
suite is labelled once in its **module header** instead (`conformance_kit.rs`:
*each broken domain but the last is one of `structure.als`'s witnesses… the last
has no witness and can have none*). The crate-wide account is therefore
**204 tests, all labelled**, and the two-label split cannot be computed
mechanically at all.

That is the mirror of `interpreter-k22`'s finding, and the pair is the useful
shape. It found three tests naming *neither*, which only re-reading the
forty-two could see; this finds twenty-two that a count reports as naming neither
and that are all labelled. A count over-reports in one direction and
under-reports in the other, so **an account is counted and then read**, and the
number a leaf states about its own file — small enough to read — is worth more
than any crate-wide split a script produces.

*Three of `CLI.md`'s six watch items resolved to "sound, do not promote".* The
report's landing order correlated with a rename's origin by path is ten lines and
is sound because a plan claims every destination exclusively. The CLI building
its own paths is safe because every name a snapshot admits has already been
checked to render as one path component. The ordinal argument proved *good*
rather than awkward: the refusal carrying the level's occupied span closes the
discovery loop from argv, which is the payoff `insert` spent two leaves on.
`reference::Status::from_token` was the one permitted micro-change and was taken
— a domain that renders a token it cannot read back forces every consumer to
write the mapping a second time.

**Missed.** No model was run, so nothing was missed by one. The honest negative
is sharper than at 018: the vocabulary finding above is a defect in the
*interface* that both models are structurally incapable of posing, because a
message is a string and neither file holds one. Two entries now (011, 019) put a
real interface defect inside a stated idealisation, which is the strongest
recurring evidence in this log that **the exclusion list is a worklist and not a
disclaimer**.

**Cost.** No tooling beyond `cargo`. The expensive part was reading the library's
public surface — `Report`, `Refusal`, `Error`, `Entry`, `Container` — closely
enough to build paths and labelled traces from values that were designed for a
programmatic consumer rather than for a terminal. `CLI.md` had already paid for
every decision, so the leaf was transcription plus the six watch items.

**Counterfactual.** The vocabulary defect is the one thing here that a cheaper
instrument would have caught: **compose one example error message in the
consumer's own words at design time.** `CLI.md` did exactly that for exit codes
and for the landing trace, and did not do it for a refusal — it said refusals
reach the operator *verbatim, as `Display`* and never wrote one out in the
syllabus's words. One line of the document would have shown a message about
leaves and nodes in a tool that has neither, before there was any code to
discover it against. The rule generalises past errors: **a design that promises a
rendering should render one.**

**Verdict.** No formalism, as predicted, twice over — but the leaf is not
evidence-free about the tools. It is evidence for the *artifacts a model leaves
behind*: a witness table written at design time drove a suite, and an
idealisation list named the one defect the models could not. H2 is supported
indirectly and narrowly, in the same shape as 017 — nothing here was led by a
model, and everything here was led by documents a model had already corrected.

### 020 — Implementing the domain the models are stated about (grove's `EntryName`)

**Situation.** The flip's *expand* stage: grove's implementation of
`ordinal_fs_tree::EntryName`, written before any verb moves onto it, and with it
the decision the library's canonicity obligation forces — grove's own grammar is
lenient on position padding, so `05-impl-a-k1.md` and `5-impl-a-k1.md` are one
entry occupying two files. Tighten, or waive knowingly.

**Formalism. None written; two instruments reached for, and they answered
different questions.** The first is the library's conformance kit, which is
`structure.als`'s witnesses turned into a runnable checker a consumer points at
its own domain — the closest thing in this workstream to a model that ships. The
second is deliberate mutation of the implementation (entry 006's instrument),
reached for as the control on the first, because a kit that says *conforming* is
making a claim about the samples it was handed and not about the domain.

**Caught.** The mutation found the kit reading clean while the domain was broken.

*A conformance kit reports coverage of an obligation, not coverage of the case
that could fail it.* The fixture was ten listings drawn from what a real
`.grove/` holds — the charter, a live leaf, both terminal marks, a node
directory, `FORMAT`, a foreign `README.md`, and the three transaction sentinels.
Disabling the domain's canonicity check left the kit **green**. Its canonicity
check is `format(parse(f)) == f` over the filenames it is handed, so a grammar
that accepts `5-…` and renders `05-…` is only ever caught by a `5-…`; and its
second kind of finding — *this obligation was never exercised* — cannot see the
gap either, because the other nine listings did parse, which is what the kit
counts as exercising it. Two near-miss listings later, the same mutation is red.

That is the failure mode the kit's own header is about, one layer in. The kit
distinguishes *no samples* from *samples*. It cannot distinguish *samples* from
*samples that pose the question*, and the distinction is invisible from inside:
both produce the same report.

*Two further mutations, as controls on the rest.* Classifying `.grove/FORMAT` as
`Reserved` rather than `Foreign` — the mistake the leaf brief names first,
because `Reserved` halts and `FORMAT` is in every healthy tree, so it would make
every grove command refuse every grove tree — fired exactly one test. Dropping
`default-features = false` from grove's dependency line fired exactly one. Both
were the intended test and no other.

**Missed.** Neither model reaches any of this and both say so: they hold no
strings by design, so the grammar — the largest single piece of the domain — has
no model coverage at all. Entry 004 recorded that gap and named
property-based testing as the instrument that fits; this is the second domain to
meet the same question and the second not to reach for it. What was used instead
is one adversarial sample plus a mutation, which is cheaper and strictly weaker:
it establishes that *this* lenient spelling is refused, where
`format(parse(f)) == f` over generated names would establish the law.

**Cost.** Minutes. Three mutations, each a one-line edit and one `cargo test`;
the fixture fix was two lines. The instrument needs no tooling and no setup,
which is most of why it is worth reaching for at all.

**Counterfactual.** The kit's own parameter documentation asks for exactly what
was missing — sample listings "should include the domain's own well-formed names,
its distinguished child, at least one foreign name, **and any near-miss the
grammar is meant to refuse**". The fixture was filled from *what a healthy tree
holds*, which is a different question, and the near-misses fell out. **Read a
kit's parameter documentation as a checklist of what to supply, not as a
description of what you have.** The stronger, more general form is the one the
mutation actually taught: **a coverage report cannot distinguish an obligation
reached from an obligation reached by a case that could fail it — only a
mutation can**, so any suite that reports its own coverage wants one mutation per
reported obligation before the report is believed.

**Verdict.** Reach for the mutation every time a kit or a suite reports its own
coverage; the two answer different questions and the report is the one that
cannot detect its own blind spot. The kit itself remains worth its cost — it
turned five stated obligations into one line in a test — but its verdict is a
statement about a fixture, and this entry is what that sentence means in
practice.

### 021 — Deciding whose words a refusal speaks (grove's verb set)

**Situation.** The flip's *expand* stage, second leaf: settle how an
`ordinal-fs-tree` refusal reaches a grove operator — print verbatim, re-word, or
reopen `docs/adr/entry-name-is-the-only-seam.md`, which named grove as the
condition that would reopen it — and write the refusal-**reachability table** for
grove's whole verb set before any verb flips. The two halves are one leaf because
the table is what says how many refusal variants the vocabulary question is
actually about.

**Formalism. None, and two instruments, both read off this log's own routing
table rather than invented here.** Entry 019's row — *whose words does this error
speak? — compose one example message in the consumer's own vocabulary at design
time* — and entry 014's — *an enumerated refusal: name the argument that reaches
each case*. Entry 009's sizing rule predicted zero model coverage before the leaf
started, and entry 003's mechanical test agrees from the other side: *which
words does this sentence use* and *can an argument produce this case* mention no
state of the tree at all, so the question is neither structural nor behavioural.
Both models hold no strings by design and neither reaches a `Refusal`'s payload.
The prediction held: nothing in the finished table could have been stated as a
claim in either file.

**Caught.** Both instruments contradicted a claim the brief chain carried as
settled, and in opposite directions.

*The message was carried and the inference from it was not checked.*
`docs/ordinal-fs-tree/CLI.md` quotes the offending sentence verbatim, and every
brief downstream quotes it again — so entry 019's instrument looks discharged.
What none of them did is read it **against grove's glossary, clause by clause**.
Doing that: *the entry with key 7* ✓ (grove says *key*), *is a leaf* ✓
**extensionally** — every positioned regular-file entry under `.grove/` is a
grove **Leaf** — *which holds nothing* ✗, *Children go in a node* ✓ (grove's
**Node directory**), *promote it first* ✗, *or name a node* ✓. Four of six
clauses are true in grove, and the two that break are not the ones the chain
predicted: the leaf brief says an operator *reads a true statement about regular
files as a false statement about their tree*, and the nouns are exactly the half
that survives. **The defect is in the verb the message names, not in the nouns it
uses** — `promote` is an operation grove's verb set does not have, and
`grove-llm`'s operator is an LLM that will try it.

*The count cut the question from ten variants to one, for a structural reason
that generalises.* **A consumer whose preconditions are stronger than the
library's pre-empts the library's own refusals for free**, because it has to
classify its target before it can call at all. Grove has four such preconditions
the library cannot see — an outcome infix, a session kind, `finish`-reservation,
brief-ness — and they retire `PromoteNotLeaf`, `PromotePartsNotNode`,
`PromoteNoDistinguished` and `RewriteSpeciesChange` behind checks grove keeps for
unrelated reasons. What is left reachable from an ordinary argument is
`TargetNotNode`, alone.

*The same refusal, two consumers, opposite reachability, decided by one
argument's shape.* `NoOccupantAtOrdinal` is reachable in all three of its
messages from the syllabus CLI, whose `insert` takes an ordinal, and unreachable
from grove, whose `leaf-insert` takes the **entry** whose slot the new leaf takes
and reads the ordinal off it in the snapshot the insert plans from. Three
messages the library spent two leaves getting right (entries 012, 013) cannot be
produced by grove at all. That is entry 018's *an argument surface discharges an
obligation the way a type does*, applied to a refusal instead of an obligation.

*A negative control on the count, and it failed the prediction.* The leaf brief
asked which refusals grove reaches that the syllabus could not, expecting
hand-edited trees to widen the set. **None.** `DestinationOccupied` and the two
exhaustion refusals are reachable in both and by the same route — a hand-written
name — and `ContentForANode` is discharged by both verb sets. Hand-editing widens
*which trees* reach a refusal, not *which refusals* are reachable.

**Missed.**

- **The table is a reachability claim with no witness behind it.** Entry 003's
  row answers *is this refusal reachable at all?* with *either formalism, via a
  reachability witness*; none was built. This table rests on reading thirteen
  verb bodies against ten variants, which is the same standing `CLI.md`'s table
  had — and `CLI.md`'s was confirmed only *after* `cli-k16` transcribed it. So
  this one is a prediction until the migrate leaves do the same, and its value is
  precisely that it can be found wrong.
- **`Failed` and `FailedPartiallyRolledBack` are asserted from the library's own
  documentation rather than driven.** Grove has no fault seam, and the library's
  internal one is `pub(crate)`. Their reachability *in the wild* is the library's
  claim, repeated.
- Neither model reaches any of this and both say so, which is now the third
  consecutive grove-side leaf to record that the largest piece of the work — the
  grammar in 020, the message text here — has no model coverage at all.

**Cost.** No tooling. One composed message, which is minutes, and one reading of
ten `Refusal` variants and seven non-`Io` `Error` variants against thirteen verbs,
which is not — the verb-by-verb pass is most of the leaf and there is no way to
shorten it, because the answer for each cell is a fact about one function's
arguments. No `src/` change and no test change: 1228 passing before and after,
the same count `domain-k29` left.

**The prediction, in entry 017's terms, so a later reader can check it.** 017
measured a second wording drifting: four failures, all on literal message
substrings the scoring arm had authored. This decision writes **no** second
wording, so it creates zero new message substrings and cannot drift that way. The
exposure it does create is the opposite one — a pre-check that duplicates a
library check and goes **stale** if the library's widens, so grove would refuse
where the library would have proceeded, silently and in the right words. The
cheap guard is stated with the rule: make the pre-check the library's own
predicate read off the snapshot (*contents are `Some`*), never a second predicate
(*the path `is_dir`*). Falsification: a migrate leaf that writes `is_dir` there,
or any later `TargetNotNode` case grove's check does not cover.

**Counterfactual.** **Carrying an offending message is not the check; reading it
against the target glossary is.** `CLI.md` carried the sentence and three
downstream briefs carried it again, each also carrying a one-line summary of what
was wrong with it — and the summary was wrong about which half breaks, which
nothing downstream could see because a summary of a message cannot be checked
clause by clause while the message can. The check costs one paragraph: put each
clause beside the glossary entry it lands on and mark it true or false. That is
entry 013's *name the carried value behind each clause* pointed at a **foreign**
message rather than at one's own, and it would have sized this question at
`crate-k7`'s close.

**Verdict.** H1-shaped, and with one new observation the distilled skill should
carry. The routing said *neither formalism* before the leaf started and was
right. What paid instead was the accumulated log: two instruments, an
adjacent-entry sizing rule and a negative control, all read out of the routing
table rather than invented — **the first leaf in this workstream where the log,
and not a model, was the instrument.** That is the second experiment's own
deliverable behaving as intended, and it is evidence for it that no single
modelling episode could produce.

### 022 — Transcribing an inherited reachability table (grove's marking verbs)

**Situation.** The flip's *migrate* stage: `leaf-retire` and `leaf-prune` onto
`ordinal-fs-tree`'s `rewrite`, the first mutation grove performs through the
library. Two of the leaf's obligations are about someone else's written claim
rather than about code — `refusals-k30`'s reachability table says these two verbs
reach exactly one refusal, `DestinationOccupied`, and that table's own stated
guarantee is that each migrate leaf transcribes its rows into a suite and finds
them wrong if they are.

**Formalism. None written, and none needed: the instrument is *composing the
fixture the row implies*.** Entry 019 named it in the constructive direction —
write the table at design time, then transcribe it into the suite. This is the
first time a later leaf ran the second half against a table it did not write.
The models were read, but only their recorded **misses**: `structure.als` leaves
walk order unmodelled, so `by_key`'s tie-break on a duplicate-key tree rests on
prose (entry 006 found the same gap from the other side, and the node brief
carries it as a known miss).

**Caught.** Two things, and the second is the one with teeth.

*The row is wrong, and writing the test is what says so.* The table's evidence
for `DestinationOccupied` was *a `DONE` twin sitting beside the live leaf so that
retiring it lands on a taken name*. Composing that fixture makes the reason
visible immediately: the occupying name must be **exactly** the name the mark
would place, and both the outcome infix and the key are parts of one name — so
the twin necessarily carries the live leaf's key. There is no tree in which the
destination is occupied and the key is not duplicated. The row could not have
been checked by reading; it needed the fixture, because the argument is about
what two names have in common.

*And a duplicated key is a live defect, not a curiosity.* `rewrite` is called
**by key**. With two entries under key 1, `by_key` answers with whichever the
walk reaches first — the very tie-break the model does not establish — so
`leaf-retire` aimed **by path** at the live leaf rewrote the `DONE` twin onto its
own name, changed nothing, and printed the twin's path as the retired leaf. A
silent success aimed at the wrong entry, which is worse than any refusal, and the
first written test found it in one run. The fix is grove's own precondition
(`task_tree::addressable_key`, refusing a key that names more than one entry) and
it makes `DestinationOccupied` unreachable from both verbs, which is why the row
is now corrected rather than merely annotated.

**Missed.** Nothing in either model reaches any of this, and both say so. What is
worth naming is *which* miss paid: the recorded gap that mattered was not the one
about strings (entries 004 and 020's grammar hole) but the one about **order** —
walk order unmodelled, therefore `by_key`'s tie-break unestablished. That miss
had been recorded twice, by two leaves, in two layers, and neither had a consumer
that could feel it. This leaf was the first with a verb that names an entry one
way and operates on it another, which is the shape that turns an unmodelled
tie-break into a wrong answer.

**Cost.** Under an hour, and almost all of it in the second finding. The row's
falsification cost one fixture; the defect it exposed cost one function and two
tests. No tooling, no model run.

**Counterfactual.** Reading the table harder would not have done it — the row is
written the way anyone would write it, and its error is a fact about names
composing rather than about the library. What *would* have found it earlier is
one question asked at design time and cheap to ask: **for each refusal you mark
reachable, write the fixture's filenames out in full.** The collision and the
duplicate key are the same string, and seeing them written down is the whole
argument. The general form is the sharper one: **a reachability claim about an
operation named by key is a claim about key uniqueness too** — so a table
covering a by-key surface owes a row for *two entries share a key*, which
`refusals-k30`'s did not have and which is where both of this entry's findings
live.

**Verdict.** Transcribe an inherited table by building each row's fixture, never
by re-reading the row. The table earned its cost either way — it named which four
of ten variants to think about, and one leaf's worth of checking corrected it —
which is the arrangement entry 019 proposed working exactly as proposed.

### 023 — Transcribing the same table a second time (grove's grow verbs)

**Situation.** The flip's *migrate* stage again: `leaf-add`, `leaf-add-pair` and
`leaf-insert` onto `append`, `append_many` and `insert`. The leaf inherits three
rows of `refusals-k30`'s reachability table and the same obligation entry 022
discharged for the marking verbs — transcribe them into a suite, and find them
wrong if they are. This is the second run of that protocol, on rows written by
the same session, and it is worth logging *because* it is a repeat: what a
protocol finds the second time is evidence about the protocol.

**Formalism. None written again, and the instrument is the same — compose the
fixture each row implies.** What differed is where the argument had to be read
from. Entry 022's row fell to the fixture alone: the collision and the duplicate
key were the same string, and writing the filenames out settled it. Neither of
these two fell that way.

**Caught.** Two more rows wrong, by two different mechanisms, and one seam
finding that no row predicted.

*`TargetNotNode` was wrong on the page, and needed no fixture at all.* The row
said **yes** and then named its own contradiction in the next clause — *Grove
keeps its own check in front of it*. Both sentences are true of the design and
only one can be true of an operator. What settles it is not judgement but the
consumer's argument surface: grove's distinguished child (`BRIEF.md`) is an entry
carrying **no key**, so it can never be handed to an operation as a target
however the refusal is worded, and clause 2's *classify before calling* is
therefore forced rather than chosen. The pre-emption is permanent, and the row
is now **no**. This one was legible by reading, and had not been read.

*`DestinationOccupied` was wrong in the algebra, and needed both the fixture and
the interpreter.* The row predicted it from a hand-edited tree carrying a copied
leaf that duplicates a key — grove's trees *are* hand-edited, so the premise was
sound. It is still unreachable, and the reason is in two places at once: an
`append` composes its name with `max + 1` over the whole tree, so no existing
entry can carry it; and a **shift** composes `(ordinal + 1, key, parts)`, whose
only possible occupant is the sibling one ordinal higher, which is itself a mover
and has already vacated — because the renames run highest-ordinal-first and
`Plan::refusal` folds the plan through the snapshot **in effect order**,
accumulating `vacated` as it goes. The fixture (`operations.qnt`'s `corrupted`
instance written in grove's grammar) shows the outcome; only reading the fold
shows why. Entry 003's model result — highest-first, argued from the intermediate
state rather than from collisions — turns out to discharge a refusal as a side
effect, and `ops.rs` notes it in passing while the design document's table did
not carry the consequence.

*And the seam finding, which no row could have predicted: **the library allocates
the key, and the consumer's content embeds it**.* `NewEntry` takes its bytes
before the library composes the name, and grove's leaf body opens with the handle
`# <slug>-k<key>`. So a content-carrying domain cannot render content from the
answer and must **predict** the allocation — `max + 1` over the same snapshot,
which is the library's own rule mirrored on the consumer's side. `tree_id::next_key`
was supposed to die here; what actually happened is that its *rule* survived in a
place the seam did not anticipate. The prediction is checked against the report
and the verb refuses to claim success on a disagreement, because the silent
failure is a leaf whose first line contradicts its own filename.

**Missed.** The table missed both rows and could not have caught the seam
finding, which is not a refusal at all. More usefully: entry 022's own
counterfactual — *for each refusal you mark reachable, write the fixture's
filenames out in full* — would have caught **neither** of these. The first is not
about filenames, it is about a clause the row contradicts two lines later; the
second is about the order in which a plan is folded, which no filename shows.
That is a limit on a counterfactual one entry old, and worth recording as such.

**Cost.** Two hours for the leaf; perhaps forty minutes on the table. The
`TargetNotNode` row cost one reading and one loop over five parent arguments. The
`DestinationOccupied` row cost one fixture and one careful read of
`plan.rs::refusal`, and the read is where the confidence came from — the fixture
passing only shows *this* corrupted tree survives.

**Counterfactual.** Two, at different levels.

For the table: **a row whose justification contains the words *we keep our own
check in front of it* is not a reachable row.** That is a mechanical test, it
applies to any consumer-side reachability claim, and it would have caught the
`TargetNotNode` row at the moment it was written. Generalised: a reachability
table over a layered surface must say *reachable by an operator* or *reachable by
the algebra*, and it is worth nothing if it silently means both.

For the algebra: **a refusal whose reachability depends on the order effects are
folded cannot be settled from an operation's signature.** `DestinationOccupied`
is the only refusal in this library that is a property of the *plan* rather than
of the arguments, and it is the only one whose row was wrong for a structural
reason twice — once here and once at entry 022. A table over an
operation-plus-interpreter design should mark those rows and check them against
the interpreter, not against the operation.

**Verdict.** Reach for the protocol again — it has now corrected three rows of
one table across two leaves, at well under an hour each. But do not expect the
same instrument twice: entry 022's fixtures found a fact about names, and this
leaf's found a fact about fold order and a fact about who allocates. The
transcription is what is repeatable; *what it takes to falsify a row* is not, and
a leaf that budgets only for a fixture will re-affirm a row it should have
broken.

### 024 — Transcribing the same table a fourth time, and reading an error message as a specification (grove's `leaf-decompose`)

**Situation.** The flip's *migrate* stage, last mutation: `leaf-decompose` onto
`promote`. Two obligations, and they turned out to be different in kind. The
first is the inherited one — transcribe this leaf's rows of `refusals-k30`'s
reachability table into a suite and find them wrong if they are; the row was
`DestinationOccupied`, the only one still predicted **yes** and the one that
looked likeliest, since a promotion's destination is composed from an ordinal and
a key that already exist. The second came from the leaf brief rather than from
the table: *read* `docs/ordinal-fs-tree/ARCHITECTURE.md`'s **When rollback
fails** *before deciding what grove reports*.

**Formalism. None written, for the fourth time, and the instrument for the row
was the same — compose the fixture the row implies.** What is worth logging is
that the row fell to neither of the two mechanisms the previous entries recorded.

**Caught.** One more row wrong, by a third distinct mechanism; and one gap that
is not a refusal at all and that no reachability table could hold.

*`DestinationOccupied` is unreachable from `leaf-decompose`, and what discharges
it is a consumer-side check added two leaves earlier for an unrelated reason.*
The algebra reaches the refusal perfectly well: promote composes the node as
`(leaf's own ordinal, leaf's own key, node parts)`, so an entry already carrying
that name is possible on a hand-edited tree, which is exactly what the row said.
But such an entry is a **node carrying the leaf's key**, so the key is duplicated
tree-wide — and `task_tree::addressable_key`, which `marking-k32` wrote because
`by_key` silently marked the wrong twin, refuses before anything is planned.
Entry 022's finding retired a row nobody was looking at when it was written.

*The recovery advice for the one tree state this library can damage was
addressed to a reader nobody had.* `Error::FailedPartiallyRolledBack` states the
diagnosis and the fix in as many words — *a node and a leaf sharing an ordinal
and a key, with the node holding no distinguished child, is an interrupted
promotion, and removing either half resolves it* — and grove was already decided
to print it verbatim (entry 021). What neither the record nor the table noticed
is **who is holding that tree when it matters**: never the process that made it,
which reported and exited, and always a later command, to which the library says
nothing at all, key uniqueness being an obligation on the domain that no
operation checks. So the only wording available for the state the library warns
about is the consumer's — and grove's existing one, `addressable_key`'s *give one
of them a fresh key*, is **wrong** here, because the node and the leaf are one
entity caught mid-shape-change and a fresh key would make two of it. The
signature is exact enough to recognise (two entries under one key, one node one
leaf, same ordinal, node holding no `BRIEF.md`) and grove now gives the library's
own recovery for it.

**Missed.** The table missed the row, as it has missed one on every leaf that
transcribed it — four for four. More usefully, and this is the entry's point:
**both previous counterfactuals would have missed this one too.** Entry 022's
(*write the fixture's filenames out in full*) does not apply — the argument is
about a check that runs before any name is composed. Entry 023's (*mark the rows
whose reachability depends on fold order and check them against the interpreter*)
does not apply either, for the same reason: the interpreter is never reached.
That is now two entries whose counterfactuals failed to predict the next
falsification, and it is worth saying plainly — a counterfactual written at the
moment of a falsification tends to describe *that* falsification's mechanism, and
the mechanisms have not repeated once in four leaves.

Nothing formal missed the recovery gap, because nothing formal was asked. Worth
being precise about why: **both models describe operations, and this is a
question about the interval between two of them, across a process boundary.**
`operations.qnt` has no notion of a run ending, and `structure.als` has no notion
of a tree being *met* rather than held. A model would have had to be asked *what
does the next command see* — which is not a claim either model's vocabulary can
state, so this is a limit of the modelling frame and not a miss inside it.

**Cost.** Under two hours for the leaf. The row cost one fixture and ten minutes,
because the argument was already written down in `marking-k32`'s finding and only
had to be recognised as applying. The recovery gap cost one careful reading of
two short document sections — the ones the leaf brief named — plus about twenty
minutes deciding whether writing a message was clause 3 broken (it is not: there
is no library wording in play, the one that exists having been printed by a
process that has gone).

**Counterfactual.** Two, and the first is deliberately about the *protocol*
rather than about this row.

**Stop writing per-falsification counterfactuals for the reachability table, and
write the standing rule the four leaves actually support:** a consumer-side
reachability row is a claim about the *composition* of the consumer's
preconditions with the algebra, so it goes stale whenever either side changes and
it cannot be settled once. What is repeatable is the transcription; a table of
this kind should therefore be marked *checked by leaf X against revision Y*, and
a leaf that adds a precondition should be expected to retire rows it was not
looking at. **A falsifiable prediction, in entry 017's terms:** the next consumer
of a refusing library that keeps its own preconditions in front of one will find
its reachability table wrong at a rate that does not decay with the number of
rows already corrected — because each correction is a fact about a different
mechanism, and there were four mechanisms in four leaves here.

For the recovery gap, and this one *is* mechanical and cheap: **read every error
variant whose message describes a persistent state of the artifact, and ask which
of your own commands can meet that state later.** A library's error message is a
specification of a tree, and the consumer is the only party that can act on it
after the failing run has gone. `entries-are-never-removed` makes this
structural rather than incidental for this library — a damaged tree is never
tidied away — and the same reading applies to any store whose failures persist.
**Three of the library's nine `Error` variants describe a persistent state of the
tree**: `Malformed` and `Reserved`, which `reading-k31` already gave grove's own
precedence in `task_tree::diagnose`, and `FailedPartiallyRolledBack`, which had
no owner until this leaf. `Failed` is the control — it says in as many words that
the tree is as it was found, so a retry is safe and no later command needs to
recognise anything.

**Verdict.** Reach for the transcription protocol again — four leaves, four
corrected rows, well under an hour each, and it is the only instrument that has
found anything about this table. Do **not** reach for its counterfactuals as
routing advice; their predictive record is 0 for 2. And add the error-message
reading to the checklist for any leaf that consumes a fallible library: it is the
cheapest thing in this log per finding, and it is not a formalism at all.

### 025 — Deleting a module, and finding that a green test was aimed away from what it judged (grove's contract stage)

**Situation.** The flip's *contract* stage, and the last leaf of the increment:
delete grove's tree algebra now that `ordinal-fs-tree` owns it, and establish
*grove's tree modules are gone, grove supplies a domain impl* as something
checked rather than asserted. Two obligations, and both were inherited rather
than chosen. `migration-k36` had left a **falsifiable prediction** — re-aiming
`tests/session_kind_guidance.rs`'s well-formedness oracle from the withdrawn
lenient grammar to the canonical one would surface at least one guidance example
the canonical grammar refuses. And the node had asked for the deletion to be
verified the way `content/references/execute.md` requires a repo-wide claim to
be verified: enumerate then classify, with a positive and a cross-tree control.

**Formalism. None, for the third leaf running and the fourth of seven in this
increment**, and the entry exists because that number is now large enough to be
a finding rather than an omission. The instruments were a re-aimed oracle and a
lexical sweep; neither is a formalism, and the second is this entry's subject.

**Caught.** Two things, and the second is the one worth carrying.

*The prediction is false, and it could not have fired.* No guidance example is
refused by the canonical grammar. But the withdrawn grammar's leniency was not
confined to the oracle the leaf was sent to re-aim: the same file's **candidate
scanner** matched a position of exactly two digits, and its **shape explainer**
did the same. While a lenient parser decided, that narrowness cost nothing — a
name the scanner skipped would have been accepted anyway — so neither could ever
fail, and neither was wrong until the parser under them was tightened. Widened to
a digit run, with a control asserting all four widths reach the parser, the
corpus is still clean — but it is now clean **measured** rather than clean by
construction. *A green test can be aimed away from the very class it exists to
judge, and the tightening that makes it wrong changes nothing in it.*

*The deletion sweep found thirty-three stale references, and not one was a `use`
line.* The compiler had already discharged the linkage claim — `cargo build
--lib` was clean the moment the four modules were removed, with the positive
control `lifecycle-k35` recorded. What it cannot see is prose, and twelve files
carried module-header essays arguing about modules that no longer existed: a
crate root essay, a version-control seam paragraph, three test-file headers, a
manifest comment. The instrument is `tests/removed_surface.rs`'s own method,
applied to a second subject — enumerate every module-shaped token under `src/`
and `tests/`, classify each against a live set read **off disk** and a listed
withdrawn set — and it is total by construction where a pattern list is complete
only as far as the list.

**Missed.** The lexical sweep's own path scope missed the one file in **no**
tree: `Cargo.toml`'s header said the library is what grove's tree modules *are
being extracted into*, present tense, and no sweep rooted at `src/` and `tests/`
can ever reach a root manifest. It was found by the separate pass
`execute.md` demands over the *summary and overview* layers — the module table,
the context map, the crate manifest — which is a different reading of the same
claim and not a longer pattern list. Two of that pass's three findings were in
files the enumerator does cover but under wordings it does not match
(*surviving path-walking readers*, *verbs that have not flipped*), which is the
honest limit: a token sweep finds names, and a stale claim need not contain one.

**Cost.** Under an hour for both instruments, most of it spent on the thirty-three
essays rather than on the machinery. The sweep is about eighty lines and reuses a
file that already had the shape. No tooling and nothing to learn first.

**Counterfactual.** For the sweep, nothing cheaper exists and the entry is not
proposing one. For the oracle, the cheap and general move is the one that was
missed at the point the grammar was tightened, two leaves earlier: **when a
predicate is tightened, find every test-side filter that was built against the
loose one.** The oracle was re-aimed because a brief named it; the scanner and
the explainer were found only by asking what else in that file had encoded the
old rule. A leniency that is safe to duplicate while the parser is lenient
becomes a blind spot the moment it is not, and the blind spot is silent in both
directions — it neither fails nor reports what it skipped.

**Verdict.** Reach for enumerate-then-classify again for any *this is gone*
claim, and reach for it in preference to a grep every time; it is cheap, it is
complete by construction, and its cross-tree control is what makes a clean result
mean something. On the aggregate question this increment was asked to answer:
**four of seven flip leaves reached for no formalism at all, and none of the four
was the worse for it.** The pattern is not that refactoring is model-free — it is
that a refactor *onto an already-checked library* inherits its models rather than
needing new ones, and what it needs instead is instruments that check the
*consumer's* claims: transcription suites, equivalence tests between the two
implementations while both are live, and sweeps over the surface the compiler
cannot read. Three of those four leaves reached for one of exactly those. That is
routing evidence about when a model earns its place, and it is the shape
`formalism-skill-k38` should carry forward.

### 026 — The first temporal Alloy model of grove's own tree (task-tree names and identity)

**Scope.** Task tree, **component-local** (`crates/grove-task-tree/models/`).
`TT-01` – `TT-10`: names, identity, and the four operations of the entry-name
algebra. `TT-11` – `TT-25` are two later leaves'.

**Independence protocol: held.** No Quint model of this subject exists yet, and
`docs/ordinal-fs-tree/models/operations.qnt` — a different subject and a
different experiment — was not opened either. Nothing was carried across.

**Situation.** Experiment 1 used Alloy for static relational structure and Quint
for behaviour, which is what H1 predicted. Experiment 2 puts both on behavioural
questions, so this is the first Alloy 6 model in the workstream that has to be
*temporal*: `var` state, primed transitions, and a trace in which an operation
either applies or returns a named refusal. It is also the first model written
against a **claim catalogue** rather than against a design document — the
question of whether that changes what a model finds is most of what Experiment 2
is for, and this entry is the first data point.

The subject is grove's shipped tree semantics, already green under 1,210 tests.
Any finding here is a finding against working code, which is a harder bar than
Experiment 1's.

**Caught.** *Nothing about grove's shipped behaviour.* What the model caught was
**the catalogue**, three times — the specific shape the pre-registration named as
a borderline and classified in advance as material. Two were found by having to
write a total transition; the third by an actual counterexample, and it is the
one worth reading first.

*A task name inside a foreign directory was an entry, and should not have been.*
`TT-06.b` produced a counterexample: an insert into a directory whose **own**
name is outside the task grammar, on a level whose positions began at 2, leaving
the level non-gapless after a correct shift. The catalogue defines an entry as
"anything directly or transitively beneath the task root" — and *transitively
beneath* is not what grove does. Its walk descends into the task root and into
**nodes**, and into nothing else, so a perfectly well-formed task name inside a
foreign directory is invisible: not an entry, holding no position on any level
grove orders, its key not in the counter, and a malformity in it not stopping the
tree. The catalogue now says *reached* rather than *beneath*, `TT-04` extends the
foreign rule to whole subtrees, and `TT-06` says which directories it quantifies
over. The counterexample is retained as a witness.
**M1** `alloy-only`. **M2** `structure`. **M3** **2** — the trace named the
transition and the level; reading it took a couple of minutes once `-t text` was
in play, and it did not transcribe straight into a test. **M4** `none` — the
shipped walk is already correct, so there is nothing to write a failing test
against; the defect was in the description.

The other two were found without the solver, by the obligation to make every
action total:

*`TT-01.b` did not say which refusal a non-canonical spelling gets.* The claim
requires the spelling "refused naming the canonical one" and the refusal-reason
set is closed, but no row of it names a spelling refusal. Writing the model
forced a choice, and a choice made inside one model is a divergence the
comparison would later measure instead of the formalisms. The catalogue now fixes
it as `Malformed(MalformedEntry(entry))` — a non-canonical spelling is
task-shaped and does not parse *completely*, which is that reason's own
definition, and it is the same reason an unknown session kind gets.
**M1** `alloy-only`. **M2** `refusal`. **M3** n/a — there was no counterexample
to read; the finding is an absence in the catalogue, surfaced by having to write
a total transition. **M4** `none`.

*`TT-07`'s byte clause is unreachable as the omissions stood.* The claim says a
shift changes "never any file's bytes", and the deliberate-omission row reduces
an entry to "identity and type only" — so read together, both families are
required to check a clause neither can express. The fix was already in the
catalogue's own vocabulary: the **entry digest**, an opaque equality defined for
`FN-12`, and the `TT-` section simply never cited it. The model carries a
`Digest` per object that a rename never touches, and `TT-07` is checked in full.
**M1** `alloy-only`. **M2** `structure`. **M3** n/a. **M4** `none`.

**All three findings falsify H7**, by the terms H7 sets for itself: none yields a
Rust test that fails against the pre-fix implementation, because none is a defect
in the implementation. The pre-registration anticipated exactly this row —
*shipped behaviour the model says is fine, but a catalogue required to describe
it never described it* — and said such a finding is more interesting than the
ones that convert. It is worth being precise about why: a claim catalogue is a
*new* artifact, so the class of defect available to find is "the specification is
not checkable", and that class has no failing test by construction. H7 was
written with Experiment 1's subject in mind, where the artifact under test was
code. **H7 is falsified at the first entry, and the falsification is about the
subject rather than about the formalisms.**

**Missed — and this is the entry's real result.** The suite reported itself
green, witnesses included, while checking **nothing**. Two defects, each hiding
the other:

1. `doRewrite` constrained `Sys.act'` only through `i = DoneI implies Retire` and
   `i = AbandonedI implies Prune`. For `i = LiveI` neither fired, so the
   predicate admitted a rewrite that **labelled itself any action at all** —
   `AddLeaf`, outcome `Applied`. And a "rewrite to Live" under a canonical
   grammar renames an entry to the name it already has, so the tree does not
   change.
2. An Alloy 6 trace is a lasso: the last state must loop. A state reached by a
   tree-changing action can loop neither back to the idle initial state (the
   tree differs) nor to itself (repeating the action changes it again). **At
   `2 steps` no applied mutation exists at all**, so every check conditioned on
   `Sys.res' = Applied` — `TT-04` through `TT-09`, ten of the sixteen
   obligations — was vacuously true.

The witnesses exist to catch exactly (2). They did not, because (1) let the
solver forge the action label each witness was looking for, and a forged
`AddLeaf` that changes nothing satisfies the lasso. **One hole in the transition
relation defeated the whole witness discipline at once**, which is worth stating
as a general rule: *a paired witness proves reachability only if the transition
relation cannot lie about which action fired.* Write every action's outcome as a
total function of its guard **before** writing any command, and the control
works; leave one case unconstrained and the control reports on a fiction.

It was found by re-reading the transitions, not by any command, and it stood for
roughly half an hour. **M8 false-confidence incident**; the bound change that
fixed it is **M7** by clause 2 of the material-finding rule, since no
tool-neutral claim was invalidated — nothing had been recorded on the strength of
the false green.

Two things about it are worth carrying beyond this model. The first is that the
*hazard the pre-registration named third* — the vacuous invariant — arrived
through the route it did not name: not an antecedent that happens to be
unreachable, but a **trace-length bound that makes an entire class of transition
unreachable**, with the witness control neutralised by a separate defect. The
second is that a green suite plus green witnesses is still not evidence; the only
thing that distinguished the fiction from the fact was mutation, and reading.

**Missed, more ordinarily.** Nothing about grove, and the honest reading is that
there was little to miss here: `TT-01` – `TT-10` are the claims closest to the
`ordinal-fs-tree` boundary, and that boundary is the one part of grove already
modelled twice and already implemented against those models (entries 002–019). A
green run here is substantially Experiment 1's work re-confirmed through grove's
own vocabulary. The claims that could surprise — selection, root classification,
guards, fail-closed ownership — are `TT-11` – `TT-25`, and they are not in this
entry.

**Three observations that are not findings.** None affects a tool-neutral claim,
so by the material-finding rule they are prose and M7, not counts.

*The catalogue is not extractable by a match, quite.* It documents the obligation
shape by showing it, and the example inside the fenced block is byte-identical to
a real obligation line — a manifest extractor counts `TT-02.a` and `TT-02.b`
twice. The runner skips fenced blocks and the catalogue now says so. The general
form is worth keeping: **a document that is also a manifest has to say which of
its lines are the manifest**, because its own worked example is indistinguishable
from its content.

It recurred within the hour, which is the part worth recording. The family
`README.md` is *also* a manifest — the runner reads declared gaps out of it — and
writing a worked example of the gap line into it reintroduced the same defect.
It happened not to fire only because the placeholder `TT-nn.x` has letters where
the pattern wants digits: luck, not design. The rule is therefore not "fix this
document" but **skip fenced blocks wherever a manifest is read**, which is what
the runner now does in both places, with a control that the real declaration is
still seen.

*Alloy's CLI reports a temporal counterexample and shows nothing of it.* The
default `-t table` renders a trace as an empty grid — every field column blank.
The tool says "counterexample found" and the trace is unreadable, which is an
**M3 score of 0 produced by a flag rather than by a defect**. `-t text` prints
the full valuation per state and the same counterexample scores 2. This is one
step from an M8 incident in the other direction: a reader who concludes "Alloy's
temporal counterexamples are useless" has measured a default, not the tool. The
runner passes `-t text` and `models/README.md` says why.

*Three performance cliffs, all about how a property is spelled rather than what
it says, and one bound that simply costs what it costs.* Gaplessness written as `#positions != #entries` costs an
Int-arithmetic encoding that took `TT-03` alone past three minutes; the same
property stated relationally — no repetition, a position 1 present, no position
whose predecessor is absent — runs in seconds. Quantifying an operation's
*fresh* objects over `Obj` rather than over `FileObj`/`DirObj` makes the
promotion transition `5^4` combinations instead of `2 × 3 × 3`. And a check whose
antecedent is broad — `Sys.act' in groveActs` — costs more than the same claim
written four times, once per action, because each narrow antecedent pins the
transition.

The third carries a rule worth keeping, because the obvious fix is the wrong
one: **when a check will not finish, narrow the antecedent, not the bound.**
Shrinking a bound buys the green run at the cost of what the run was evidence
about, and it can quietly put the check below the bound at which its own witness
first lands — at which point the check has no content and the pair still reports
green. Splitting one command into four kept the bound and removed the work.

**Cost.** One session. **M5 authoring:** about 1 h on the runner and 2 h on the
model, over 16 obligations — 0.19 h per checked obligation for a component-local
model, which is the figure H8 will compare against `models/system/` when that
exists. **M7 state-space and tooling:** the Alloy scope runs about **30 minutes**
wall on the measurement host, SAT4J, at the bounds recorded on each command;
`witness_TT_07` is nine of those minutes on its own and two checks are three to
four each. **M7 wrangling:** about 3 h — the three performance cliffs, the
output-format discovery, one dead end where every check ran at 3 steps before the
unconstrained initial state was noticed, the half hour the false green stood, and
the hour the mutation pass cost including its three replaced mutations.
**M6 synchronization: zero**, there being no second family yet; recorded as a
baseline rather than as an absence.

Against Experiment 1's M7 baseline — 17 s for Alloy over 20 commands — this is
**30 minutes over 37**, and the pre-registration's warning applies in the
direction it expected: the subject has behavioural state now, and the cost went
up by two orders of magnitude. That is the first evidence that Experiment 2 is
modelling something Experiment 1 was not.

**The mutation pass, run before the green was believed, and what it cost.**
Sixteen mutations, one per obligation: thirteen killed their check on the first
attempt. The three that did not are a finding about the control rather than
about the model.

Two of them — *insert may drop an object*, *promotion leaves the leaf on disk* —
turned out to be **unsatisfiable against the model's own filesystem facts**: an
object cannot leave `onDisk` while keeping a parent, nor stay on it without a
name. The mutated transition never fires, its check is vacuously true, and the
run reports green **exactly as a surviving mutation does**. So: *a mutation the
model cannot execute is not a control; it is a second vacuity wearing the first
one's clothes*, and nothing in the runner can tell the two apart — the same
reading that produced the mutation has to confirm the mutated model still admits
the situation. This is the mutation-testing analogue of an equivalent mutant, and
it is worse than one, because an equivalent mutant at least runs.

The third survived honestly and is worth keeping: dropping species mismatch from
the halting reasons left `TT-02.b` green, because a file carrying a node name
lands in `nodeDirs`, has no charter child, and the tree halts under
`NodeWithoutCharter` instead. Two reasons overlap on exactly that tree — which is
a fact about the catalogue's reason table that only a mutation could have
surfaced, and which the `guarding` leaf should keep in view when it models the
rest of the table. **M7**, an hour.

**Counterfactual — for the incident.** Nothing in the pre-registration's control
set would have caught it. The runner's *fail on zero work* fires when a command
set is empty or a witness never lands; here every command ran and every witness
landed. *Every invariant has a paired witness* was satisfied. The bound was
recorded, as the *scope trap* control asks — and the recorded bound was the
defect, which is a case that control cannot see, because it records the bound
rather than testing it. The only instrument that would have caught it is the one
the catalogue names last and this session had not yet run: **one mutation per
reported obligation**. A deliberate break of the append's position rule would
have left `TT-06.a` green, and that is the signal. The lesson is a sequencing
one — *run the mutation pass before believing the first green run, not after* —
and it is cheap enough that there is no argument for the other order.

**Counterfactual.** Would a careful prose read of the catalogue have found the
two material findings? For `TT-01.b`, plausibly — a reader checking each claim
against the closed refusal set could notice the gap. For `TT-07`, less likely:
the claim and the omission row are eleven hundred lines apart in one document,
and each is correct on its own. What forced both was **having to write a total
transition function**: every action must return exactly one outcome, so a claim
that does not say which outcome is a hole the model cannot leave open. That is
the same mechanism entry 021 recorded from the other side, and it is the clearest
thing this entry has to say about *why* a formalism helps here — not that it
searched a state space, but that it refused to accept an unstated case. Note
what that implies: neither finding needed the solver at all.

**Verdict.** Temporal Alloy 6 is usable on this subject and the modelling cost is
ordinary, but this slice is the wrong place to judge the *formalism* — the claims
nearest the already-modelled boundary are the ones least likely to surprise.
What it establishes is procedural, and both halves are worth carrying:

- **Write the transitions as total functions first and the checks second.** The
  totality obligation is what found both catalogue holes, and neither needed the
  solver. It is also what would have prevented the incident: the incident's first
  defect *is* a transition that was not total.
- **Run the mutation pass before believing the first green run.** A green suite
  with green witnesses was, here, entirely fictional, and mutation is the only
  control that distinguishes the two.

**The seam, exercised in both directions.** `models/run.sh --scope task-tree
--family alloy --no-coverage` reports 37 of 37 commands passing and exits 0. The
same run with coverage asserted reports the same 37 passes and exits **1**, on
16 complete cells and 27 empty ones — which is the phase's remaining work
reported as such. A missing model directory and a family with no model file each
abort at *zero work* rather than reporting a clean sweep. That is the third
runner obligation working: the catalogue rather than the model is the source of
truth about what has been checked.

**The model, as required per entry.** Alloy 6, Corretto `21.0.12.1+9-LTS`, SAT4J
(distribution default). Bounds per command, the common shape being
`for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 3 steps`;
`TT-03` runs one filename short of that, which is the difference between 72
seconds and not finishing in three minutes, and the bound is recorded on the
command. Every command runs with `-n`, so an overflow of `plus[7, 1]` is not a
counterexample. No fairness assumption: nothing in `TT-01` – `TT-10` is an
eventuality. Abstractions and deliberate omissions are listed in
`crates/grove-task-tree/models/README.md`; the one added beyond the catalogue's
fixed list is that a directory the walk does not enter is present on disk and
outside the model's `visited`, which is the third finding turned into
machinery. **What a green run does not prove:** every result is about at most
three files, two directories, six filenames, three states, one working tree and
one cooperating process — and the three-state bound is sufficient only because
the initial state is unconstrained, which is `EN-11` cashed out as a modelling
decision rather than a claim anyone checked. Nothing here is a proof about
arbitrary trees, and nothing here is evidence about `TT-11` – `TT-25`.

### 027 — Eleven obligations, no findings, and a mutation that could not fire (task-tree selection)

**Scope.** Task tree, **component-local** (`crates/grove-task-tree/models/`).
`TT-11` – `TT-16`: the pre-order walk, terminality, the reserved finish leaf, and
the empty/ambiguous observation outcomes. `TT-17` – `TT-25` are the `guarding`
leaf's.

**Independence protocol: held.** No Quint model of this subject exists. Neither
`docs/ordinal-fs-tree/models/operations.qnt` nor any Quint file was opened.

**Situation.** The second slice of the same Alloy 6 file, and the first one whose
subject is *reading* rather than mutating: `select` and `resolve`, their outcomes,
and the rule that a live finish leaf is reserved rather than blocking. Entry 026
left the file green for `TT-01` – `TT-10` after a false-confidence incident that
cost a suite's worth of trust, so this slice was run under the procedure that
incident produced — transitions written as total functions first, commands
second, mutation pass before the green is believed.

**Caught: nothing.** No counterexample about grove's shipped behaviour, and — a
change from entry 026 — **no finding about the catalogue either**. Every one of
`TT-11` – `TT-16` transcribed into a command without forcing a choice the
catalogue had not already made. Eleven obligations, twenty-two commands, all
green on the first complete run, and eleven mutations that each broke exactly the
check they were aimed at — one of them only after being re-aimed, which is this
entry's one incident and is recorded below rather than smoothed over.

That is a **miss recorded as a miss**, and it is the second data point on the
same axis as entry 026's verdict: *the claims nearest an already-settled boundary
are the ones least likely to surprise*. `TT-11` – `TT-16` are the most fully
specified claims in the task-tree scope — `TT-13` and `TT-15` were both sharpened
by `model-contract-k31`/`k32` before any model existed — and a model of a claim
that has already been argued to a fixed point finds nothing, because the finding
already happened in prose. **No material findings. `M4 = none` for the slice**,
and there is no defect to write a failing test against.

**Two things the modelling did establish, neither of them material by the
counting rule.** Both are recorded because the entry would be misleading without
them.

*`TT-11` and `TT-14` are separable claims, and only the mutation pass shows it.*
Written naively, `TT-14` — *selection is not a scheduler* — reads as a restatement
of `TT-11`'s pre-order and is tempting to leave out. It is not: re-defining
`precedes` to order siblings by **key** instead of position leaves `TT-11`'s check
green (the walk still returns the `precedes`-minimum, and `TT-11` is stated in
terms of `precedes`) while `TT-14` fails, because `TT-14` names the position
field. `TT-14` is the command that catches a scheduler the *walk itself* has been
taught to respect, which is the only place such a thing could hide.

*`TT-11`'s "depends on no state outside the tree" is not checkable, and saying so
is the honest result.* A model cannot check the absence of a variable it does not
have. The walk is written as a function of `loc` and `nm` alone and the README
says so under its own heading, rather than a command pretending to establish it.
This is the shape entry 021 recorded from the other side: the useful output of a
formalism is sometimes a clear statement of what it is *not* evidence for.

**One false-confidence incident (M8), and it is the fourth of its kind here.**
`TT-12`'s first mutation — widen promotion's applied guard from `t in liveLeaves`
to any leaf, so a terminal entry can be taken off disk — **survived**. It was not
a survivor. `doDecompose` still carried its `RefAlreadyTerminal` clause, so on a
terminal target both implications fired against a `one Result` field; the
transition was unsatisfiable, never fired, and the check passed vacuously. **The
runner reports a mutation that cannot execute exactly as it reports one that
executed and was not caught**, and only reading tells them apart. Entry 026
retained two of these; this is the third and fourth cumulatively, and the pattern
is now specific enough to name a rule: **a mutation is a control only with
evidence that it fires.** Re-aimed (remove the refusal clause as well), `TT-12`
was caught in 30s, and `witness_TT_09c_promotion` finding an instance *under the
mutation* is the cheap evidence that it executed. Stood **~3 minutes**.
**Genuine failing checks this slice: zero, at zero hours.** Both halves of H10's
ledger, and this slice contributes only to the false-confidence side.

**A second self-inflicted one, on the instrument rather than the model (M7).** The
bound probe that establishes where each witness *first* lands was run by
string-substituting the `steps` scope in a copy of the file. The pattern included
a trailing `\n}` that the file does not contain, so two probes silently changed
nothing and reported the witness landing at a bound it had never been run at.
Caught by the result being implausible — `TT-14` cannot land in fewer states than
its own three actions — and corrected with an assertion on the substitution
count. **Wrangling: ~10 minutes.** The generalisation is the same one the runner
already embodies at a larger scale: *a transformation that silently matched
nothing reports what a transformation that worked reports*.

**Cost.** Authoring ≈ **2 h** for 11 obligations (**M5**: 0.18 h/obligation,
against entry 026's 16 obligations at a markedly higher rate — the machinery this
slice needed, the walk and two observation transitions, is a fraction of what the
name algebra needed). **M6 synchronization: 0.** No second family exists yet.

**M7 — state-space and tooling.** The selection slice's 22 commands cost **4m 40s**
wall against the `TT-01` – `TT-10` slice's ~30 minutes for 37, and the ratio is
the finding worth carrying: **an observation is cheap because it cannot change
the tree.** The lasso argument that forces every mutation command to `3 steps`
does not apply to a read — the state a read reaches loops to *itself* — so the
selection commands run at `2 steps`, and eleven of the twenty-two finish in five
seconds. Two exceptions locate the boundary precisely: `TT-12` and `TT-13.c`'s
checks quantify over grove *mutations* and need `3 steps`; `witness_TT_14` needs
**`4 steps`**, because *two orderings of the same work selecting differently* is
`select · hand-edit · select` and it finds no instance at 2 or at 3. The claim
that `2 steps` admits no tree change is itself checked rather than argued:
`witness_TT_09a_append` finds nothing there. The full task-tree Alloy suite is
now **37m 36s** wall / **2581s** CPU for 59 commands, against entry 026's ~30
minutes for 37 — the 22 added commands cost about a quarter of what the first 37
did. Mutation pass ≈ **7 min**, bound probes ≈ **4 min**, wrangling **~10 min**
(above).

**Counterfactual.** Prose review would have reached the same place on ten of the
eleven obligations, because the catalogue had already been argued into shape.
The eleventh is `TT-14`: a prose reviewer asked whether *selection is not a
scheduler* needs its own test, next to a pre-order that is already specified,
would very plausibly answer no. The mutation that orders siblings by key is what
answers yes, and it is not an argument a reader produces — it is a thing done to
the model. That is one obligation's worth of value from the formalism, and it
came from the **mutation pass**, not from the solver: no check here ever searched
a state space for a defect that was there.

**Verdict.** On this slice the model is a **transcription instrument**, not a
discovery instrument, and the honest summary is that it found nothing the
catalogue did not already contain. Two things nevertheless survive it, and both
are procedural rather than semantic:

- **The mutation pass is now carrying the whole evidential load.** Every finding
  of substance in both task-tree entries came from mutating the model, and none
  from a solver-found counterexample. Entry 026's verdict said run the mutations
  before believing the green; this slice sharpens it — **a mutation is a control
  only with evidence that it fires**, since an unsatisfiable one is
  indistinguishable from a survivor at the runner's interface.
- **Cheapness follows the claim's shape, not the claim's importance.** Reads cost
  an order of magnitude less than writes at the same bounds, for a reason
  (`2 steps` versus `3`) that is a property of the lasso and generalises to every
  observation claim in the catalogue — `FN` and `SY` included, where the same
  distinction between a read and a transaction step will be worth drawing early.

**The seam, exercised.** `models/run.sh --scope task-tree --family alloy
--no-coverage` reports **59 of 59** commands passing and exits 0, in **37m 36s**
wall / **2581s** CPU. Its matrix reads **27 complete cells, 0 declared gaps, 16
empty, of 43** — and every empty cell is `TT-17` – `TT-25`, which is exactly the
`guarding` leaf's scope and nothing else. No command named an obligation the
catalogue does not define. Coverage asserted, the same run exits **1** on those
16, reported as the phase's remaining work rather than as a defect.

**The model, as required per entry.** Alloy 6, `org.alloytools.alloy.dist.jar`,
Corretto `21.0.12.1+9-LTS`, SAT4J (distribution default), every command with
`-n`. **Bounds**: `for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug,
2 Digest, 2 steps` for the observation commands; `3 steps` for `TT-12` and
`TT-13.c`; `4 steps` for `witness_TT_14`. The bound at which **each** witness
first lands is tabulated in `crates/grove-task-tree/models/README.md`, separately
from the bound its check ran at — `TT-11`, `TT-12` and `TT-13.c` need the second
directory and the other eight do not. **Fairness**: none assumed; nothing in
`TT-11` – `TT-16` is an eventuality. **Abstractions added by this slice**: a
resolution's reference is `one sig Query` (an optional key and an optional slug,
one atom per trace); what an observation reported is *state* (`Sys.got`,
`Sys.gotTerm`) rather than a derived value, which is what makes `TT-16`
falsifiable at all; `brief-chain` and `kind` are not represented, no obligation
distinguishing them from `select` and `resolve`. **What a green run does not
prove**: every result is about at most three files, two directories, six
filenames, and two to four states — and, specific to this slice, `TT-11`'s
"depends on no state outside the tree" is established **by construction and not
by any command**, so a green run is not evidence for it. It is evidence that,
given a walk defined over `loc` and `nm`, selection is that walk's minimum.

### Routing table (under construction)

Filled in from the entries above as evidence accumulates. Empty rows are honest;
guesses are not.

| kind of question | formalism | evidence |
|---|---|---|
| structural — "is this shape coherent, and can it even represent what I need?" | Alloy | 002: found two defects unprompted, sharpened two more, produced a tree satisfying every stated invariant with a subtree invisible to every traversal |
| reachability — "is every thing I must name reachable from the constructors I have?" | Alloy | 002: both blocking defects across entries 001 and 002 were this question; it is the one Alloy answers best |
| already guaranteed — "can the target language forbid this outright?" | none — check first | 002: two of eight structural claims were free from Rust's type system; modelling them taught nothing. 004: collected a third at the seam — folding three `Option` accessors into one removed an obligation from every consumer, at the cost of one trait method. 005: that collection was half wrong and said so nowhere it was repeated — a discharge claim needs the scepticism a model claim gets |
| stated-isomorphism — "are both directions written down?" | none — a free question | 002: the canonicity gap was half an isomorphism, and asking costs nothing |
| behavioural — "does this operation preserve the invariant, from any reachable state?" | Quint | 003: confirmed. All three invariants Alloy could not state were checkable, and six further defects came out of the same file |
| interruption — "what does a crash halfway through leave behind?" | Quint | 003: the ordering rule's real payoff, and `promote`'s unavoidable transient duplicate, are both invisible to every other method in this log |
| ordering — "does the order of these steps matter, and why?" | Quint | 003: the stated reason was wrong and the real one was unwritten; a model that stops halfway is the only thing that separates them |
| dead branch — "is this refusal reachable at all?" | either, via a reachability witness | 003: the occupancy refusal fires in 0% of traces on any tree the library builds. Sampled, so evidence and not proof |
| routing itself — "which of the two am I holding?" | count the states the property mentions | 003: one state → structural; two or more → behavioural. Mechanical, and applicable before choosing. Supersedes shape-versus-operation, which mis-sorts "no key is ever reissued". 018: **zero → neither** — a question about what a verb prints mentions no state of the tree at all, and the same reading that sizes a model's worth to a leaf (009) says so before the leaf starts |
| out of scope — "the model says it excludes this; what does that oblige?" | none — the exclusion list is the worklist | 011: three of five interpreter defects sat in the three domains `operations.qnt`'s handoff names as excluded (strings, bytes, the filesystem). The exclusions were written two leaves before the defects; reading them as a list of what prose must now state would have caught all three at introduction |
| enforceable obligation — "the type system cannot forbid this, but can the library check it?" | none — ask before writing it down as an assumption | 011: six of seven `EntryName` obligations are genuinely uncheckable; the seventh costs one string test at two boundaries and turns an escape from the locked tree into a refusal with recovery advice. 002's counterfactual asks whether the *language* forbids it; this is the question after that one |
| already arranged — "did the model check this, or make it true?" | neither — a free question | 003: subtree preservation is true by construction of the state shape, and a model that satisfies an invariant by construction is indistinguishable from one that verified it |
| a message derived from a model — "may this refusal say what it says?" | none — name the carried value behind each clause | 013: `wit_insertIntoAGap`'s predicate `a.at < maxOrdIn` discriminates the case and does not characterise it; the comment beside it did, and the refusal's message transcribed the comment. One clause, no value behind it, one wrong error message for every hand-edited level with a leading hole |
| did it run at all? — "is this suite green, or dead?" | must-be-reached claims beside the must-hold ones | 003: a JVM too old made Alloy print nothing, which its runner read as thirteen unfired witnesses and seven holding checks. Only the witnesses distinguish the two, and every one failing at once is a signature no real defect produces |
| deriving tests — "what do I actually run against the implementation?" | the model's must-be-reached witnesses | 004: four Alloy witnesses became executable broken-domain tests; not one `check` translated into anything runnable |
| grammar — "can two filenames name one entry?" | property-based testing *(untested)* | 004: both models hold no strings by design, so the largest piece of the implementation had no model coverage at all. The instrument that fits is `format(parse(f)) == f` over generated names, and it was not reached for. 020: the second domain to meet the question and the second not to reach for it — one adversarial sample plus a mutation is what was used, and it establishes one spelling where the law would establish all of them |
| model-claim-to-test — "does this test still say what the claim said?" | quote the claim's predicate beside the assertion | 005: two tests named a claim and checked a weaker property — `v.seen = n` became string equality, `Malformed` became *not an entry* — and both drifted toward the property that was easier to observe |
| partial discharge — "the language forbids some of this obligation" | split it into two named obligations | 005: the qualification survived in the file that made the claim and in none of the three artifacts that repeated it; the unqualified half was a real defect |
| a written law nobody holds the code against | an adversarial reader briefed to attack one judgement | 005: five of six findings, including both trait-shape defects, which neither model could reach; the models supplied the law and a reader supplied the comparison |
| does this test test anything? — "the suite is green, but was it ever red?" | deliberate mutation of the implementation | 006: six breakages, each firing exactly the expected tests; the only evidence for two claims made in prose (dropping a guard releases the lock, a re-export stays a violation). Minutes, no tooling, any suite |
| order stability — "does this answer depend on the order the input arrived in?" | property-based testing over permutations *(untested)* | 006: `by_key`'s documented tie-break was machine-dependent on a hand-edited tree, because listing order is arbitrary and ordinal distinctness is only preserved. Found by composing two model premises; `walk(shuffle(l)) == walk(l)` is what would have found it directly, and is the second leaf running to name this instrument without using it |
| what do I have to get right unaided? | the model's own recorded misses | 006: the handoff block's *walk order is unmodelled* located every line of undefended prose in the leaf before any code existed. The mirror of 004's *witnesses are the test suite* |
| a case the platform makes untestable — "the branch exists and the host cannot reach it" | assert which fact is true on the host, never skip | 006: APFS refuses non-UTF-8 filenames, so a halting branch is unreachable on macOS; a skipped test reports what a passing one reports |
| how much will a model actually be worth to this leaf? | the fraction of the leaf inside the model's stated scope | 009: 5 of 23 tests named a claim where the model barely reached (006); 28 of 42 where the leaf's subject *was* the model's. Legible from the handoff block before the leaf starts |
| what refusal will I need that no model can name? | the model's own stated idealisations, read as a list | 009: no strings, no bytes, unbounded integers — three refusal classes, one per idealisation, and `operations.qnt` lists them itself |
| one property, how many mechanisms? | a mutation control per mechanism, not per property | 009: *claims its destination* is `create_new` for one effect and look-then-rename for another; eight controls passed while the second was undefended |
| an unenforced invariant hiding in plain sight | the model's explanatory comments | 009: *`Remove` never appears in a forward plan* was a comment, and became two types that make it unrepresentable |
| how do I test a rule whose payoff is a state nothing observes? | the witness that is live in only one instance — it names the control | 012: highest-first buys distinct ordinals at every intermediate state, and a lone "every state is distinct" assertion passes under any shift order. `wit_shiftTransientlyDuplicatesAnOrdinal` is reachable only in `lowest_first`, so replaying the same landings the other way is the control the model itself points at |
| reading a model on the way into an implementation | the witnesses before the invariants | 012: the invariants confirmed what the document already said; the witness *pair* discriminating one outcome by a predicate is what turned one refusal into two messages. The behavioural half of 004's *witnesses are the test suite* |
| a refusal that covers a modelled idealisation | state it over the property, never over a list of operations | 012: *bytes for a node are refused* was written of the two operations that existed, and `insert` fell off the list. There is no typechecker for prose, so the quantifier is the only thing that ages well |
| an enumerated refusal — "can an argument actually produce each case it names?" | none — name the argument that reaches each case | 014: *a node and a distinguished child are both refused* was half unreachable, because a target is named by key and a distinguished child carries none. A model declines to support such a case by having **no witness**, which is invisible in a way a failing claim is not. The free check after 013's *name the value behind each clause* |
| a state a model reports as rare — "will this be expensive to test?" | no — ask what steers into it, not what stumbles into it | 014: the failed-rollback duplicate key is reached in 0.07% of traces and needed its own sample budget (003); reaching it in the implementation is one line of the internal fault seam. Rarity measures the simulator, not the test |
| an order that looks like a rule — "could it have been the other way?" | look for the model's counterfactual instance; its absence is the answer | 014: `insert`'s shift order has `lowest_first` and `promote`'s has nothing, because the second effect lands in the level the first creates. A forced order is a consequence, and the test should assert the forcing rather than the order |
| a recovery instruction — "does the advice describe the state it will be read in?" | drive the state, assert each clause against disk, then follow the advice | 014: three clauses of `FailedPartiallyRolledBack`'s promotion advice, three assertions about a real directory, and the remedy executed. Turns 013's review habit into a test |
| an opaque sort — "what does the model assume about comparing this?" | none — read each atom against the target language's bound | 015: `Parts` is an atom in Alloy and an `int` in Quint, so *equal parts imply equal species* is free in both; Rust bounds it by `Eq`, which is any lawful equivalence. A conforming domain lost every promotion to a `DestinationOccupied` refusal. `Eq`/`Ord`/`Hash` are all coarser than a model's identity |
| a question the brief left open — "should these two checks be one piece of code?" | none — read the two model functions side by side | 016: `planRewrite` and `planPromote` differ in one operand, a constant against a value read off the target. The shared part is one trait call, and wrapping it would be an abstraction shallower than the expression. The model turned a judgement call into a reading |
| what may a refusal carry? — "which of these fields is derivable from the others?" | count the model's variants for the sort | 016: two positioned species, so *the entry is a leaf* already says *the parts make a node*. `promote` carries none and `rewrite` carries one, and both follow from the same count. The narrow form of 013's lesson about state carried for a message |
| an edge case the model names for an operation that does not exist yet | build it into the layer beneath, with a test naming the witness | 016: `wit_rewriteToSameParts` was discharged by the interpreter leaves in both layers; the operation it belongs to later landed with no code. Written the other way round it is a layer-crossing defect, where the algebra proves a plan applicable and the applier refuses it |
| the bytes did not change — "how do I say that without trusting a comparison?" | the inode, not the content | 016: bytes read and written back identically compare equal; an unchanged inode says nothing read them. Available because the crate is Unix-only, which is otherwise invisible in the interface |
| a design document's reachability table — "which of these cases can an argument actually produce?" | none — write the table at design time, then transcribe it into the suite | 019: `CLI.md` marked six refusals reachable and seven not, before any argument parsing existed; every reachable one got a test naming its witness, every unreachable one got none, and not one was reachable after all. Entry 004's *witnesses are the test suite*, one layer out, and the constructive form of 014's question |
| whose words does this error speak? | none — compose one example message in the *consumer's* own vocabulary at design time | 019: a library refusal says *a leaf, which holds nothing. Children go in a node* to a tool that has lessons and modules. Parse failures carry the domain's own error and algebraic refusals carry none, so the half the operator meets in normal use is the half the domain cannot speak for. A design that promises a rendering should render one |
| is this per-test convention countable? | count, then read whatever the count could not classify | 019: a regex left 22 of 204 tests unclassified and every one was labelled — in prose the pattern missed, or once in a module header. The mirror of `interpreter-k22`'s three tests naming *neither*, which only re-reading could see: a count over-reports one way and under-reports the other, so an account is counted **and** read, and a leaf's statement about its own file beats any crate-wide split a script produces |
| universal — "does this hold for all inputs, not just those a checker reached?" | Lean *(untested)* | — |
| model or prose at the implementation site? | ask instead whether the prose has already been through a model | 017: the pre-registered probe's prose arm shifted highest-first for the right reason, delegated the sequential destination check correctly, and refused the gap with a carried span — violating zero model claims. Every point it got right is one `operations.qnt` had already corrected in `ARCHITECTURE.md` (003, 013). H3 as stated is not supported; a model spent upstream and banked in the document is |
| a suite as a measure of a second implementation — "can the other arm fail this?" | have it written by a session that implements neither, from the claims alone | 017: fixing the *claims* before both arms was not enough. All four failures landed on literal message substrings the scoring arm had authored; the `assert_eq!` on each refusal's value passed. A test is a claim plus an assertion, and only the first predated the arms |
| an obligation with nowhere left to go — "can the *interface* forbid this?" | none — read the obligation against the argument surface, not just the type system | 018: *the species follows from the parts* became a verb-grammar rule (a noun prefix exactly where the operator chooses a species), and `ContentForANode` became unreachable by construction. 002's *ask whether the target language already forbids it*, one layer further out — an argument surface discharges an obligation the way a type does |
| a claim about a guard — "will this change trip the detector?" | none — plant the violation and watch it fail | 018: *a binary under `src/bin/` trips the no-filesystem guard* was settled in ninety seconds by a probe that failed naming its own line, against a reading of a token scan and its carve-out. 006's deliberate mutation, aimed at a design decision rather than at a suite |
| a suite that reports its own coverage — "does *exercised* mean the case that could fail it ran?" | none — one mutation per reported obligation | 020: a conformance kit read green over ten real listings while the grammar's canonicity check was disabled, and reported the obligation exercised, because nine other listings parsed. The kit can tell *no samples* from *samples* and not *samples* from *samples that pose the question* |
| filling a fixture for someone else's checker | none — read its parameter documentation as a checklist | 020: the kit asks for "any near-miss the grammar is meant to refuse"; the fixture was filled from what a healthy tree holds, which is a different question, and the near-misses fell out |
| an inherited claim that a message is wrong in my words | none — put each clause beside the glossary entry it lands on and mark it true or false | 021: `CLI.md` carried the offending sentence verbatim and three briefs carried it again, each with a summary of what was wrong — and the summary named the wrong half. Four of six clauses are true in grove; the defect is the operation the message names, not the nouns. A summary of a message cannot be checked clause by clause and the message can. Entry 013's *name the value behind each clause*, pointed at a foreign message |
| the same refusal in two consumers — "why is it reachable there and not here?" | none — read it against each consumer's argument *shape*, not its verb count | 021: `NoOccupantAtOrdinal` reaches all three of its messages from a CLI whose `insert` takes an ordinal, and none from a verb that takes the entry whose slot is taken and reads the ordinal off it. 018's *an argument surface discharges an obligation the way a type does*, applied to a refusal |
| a consumer whose preconditions are stronger than the library's | none — count the checks it needs for its own reasons before counting the library's refusals | 021: four grove preconditions the library cannot see (outcome infix, session kind, reserved kind, brief-ness) retire four species refusals behind checks grove keeps anyway, cutting a ten-variant vocabulary question to one. The pre-check must be the library's own predicate read off the snapshot, or it is a second wording at the level of code |
| an inherited reachability table — "is this row I did not write actually right?" | none — build the row's fixture and write its filenames out in full | 022: `DestinationOccupied` was marked reachable from `leaf-retire` via a `DONE` twin; composing the fixture shows the occupying name and the live one must share a key, because the outcome infix and the key are parts of one name. Unreachable, and the row could not have been checked by re-reading it. The second half of 019, run by a leaf that did not write the table |
| an operation named by key — "what else is this reachability claim assuming?" | none — add the row for *two entries share the key* | 022: `by_key` answers with whichever entry the walk reaches first, and walk order is one of `structure.als`'s recorded misses. `leaf-retire` aimed by path at one twin marked the other and reported success. A recorded model miss with no consumer that could feel it, meeting the first consumer that could |
| is it gone? — "this pattern/module/surface has been removed" | none — enumerate then classify, with a positive and a cross-tree control | 025: a clean `cargo build` discharged the *linkage* claim and said nothing about prose; enumerating every module-shaped token and classifying it against a live set read off disk found thirty-three stale references in twelve files, not one of them a `use` line. A pattern list is complete only as far as the list; an enumeration is complete by construction |
| a predicate was just tightened — "what else was built against the loose one?" | none — grep the *test* side for filters encoding the old rule | 025: re-aiming one oracle at a canonical grammar left the same file's candidate scanner and shape explainer still matching the lenient rule, so the sweep reported a clean corpus about a class it never handed to the parser. A duplicated leniency is free while the parser is lenient and silent in both directions the moment it is not |
| a claim about the repo itself — "did I sweep the whole claim, or the files I thought of?" | none — read the summary and overview layers as a separate pass | 025: the token sweep could not reach `Cargo.toml` from any path scope rooted at `src/`, and two more stale claims sat in files it *did* cover under wordings it did not match. A token sweep finds names; a stale claim need not contain one |

---

### 028 — Four obligations, one model defect, and a vacuity whose cause was the bound (task-tree root identity)

**Scope.** Task tree, **component-local** (`crates/grove-task-tree/models/`).
`TT-17` – `TT-20`: format decided by the witness's content, the fixed
classification order, the reserved witness that refuses everything else, and the
format witness landing last. `TT-21` – `TT-25` are the `guards` and `ownership`
leaves'. Assumption mutations `EN-04` and `EN-12` were run here; `EN-07`, `EN-08`
and `EN-14` are `guards`', `EN-11` is `ownership`'s.

**Independence protocol: held.** No Quint model of this subject exists. Neither
`docs/ordinal-fs-tree/models/operations.qnt` nor any Quint file was opened.

**Situation.** The third slice of the same Alloy 6 file, and the first whose
subject is the task root *itself* rather than what is inside it. Entries 026 and
027 left `TT-01` – `TT-16` green over a tree with no identity at all: `TaskRoot`
was a bare atom, always present, carrying no name, no content and no
classification. This slice gives it a format witness whose **content** decides,
a reserved slot, an initialisation that is two filesystem steps, and a `crash`
between them.

**The leaf that was mandated was decomposed before any of it was written.**
`guarding-k35` charters `TT-17` – `TT-25` — sixteen obligations and four
independent machinery layers — against two retired siblings that spent a whole
session each on ten and six. It became a node; this entry is its first child.
**M5 is therefore reported per obligation and not per leaf**, and the
decomposition itself is a datum about modelling cost: the unit that fits a
session here is *one machinery layer*, not *one claim group*.

**Caught: one, and it is about the model rather than about grove or the
catalogue.** `InitScaffold` — root initialisation's first filesystem step — was
written with its own guard and no root cascade at all, so on a tree holding a
reserved witness it refused with `RefNotAnEntry`: a refusal naming nothing and
recovering nothing. `TT-19`'s check found it in **7s**, on the first run of the
command. The fix is a split that the catalogue implies and the model had elided:
initialisation runs the **reserved** half of the cascade and not the **format**
half, because a witnessless root is what it is *for*. That split is `TT-18`'s
ordering made operational.

It is a **model defect, not a product defect**, so by the counting rule it is not
material — but it is the first counterexample in three slices that a *solver*
found rather than a mutation, and that is worth separating from the miss column.
`TT-19` is a claim of the form *every operation, without exception, does X*, and
the class of defect it catches is **an operation nobody thought of as an
operation**. Prose review of the same change would have to enumerate the action
set correctly to catch it; the check quantifies over the action set by
construction. **`M4 = none` (no material finding). One non-material catch, at
7s.**

**Two findings about the catalogue's own margins, both recorded rather than
resolved here.**

*`TT-17` is only half checkable, and the checkable half had to be found.* Stated
literally — *classification depends only on the format witness, never on any task
entry's text* — the first half is true by construction of any classification
function written from the witness, so a command restating it cannot fail. What
*can* fail is the temporal half: a hand edit that changes every name in the tree
and leaves the witness alone must not move the root between format families. That
is the conjunct the mutation breaks, and without it `TT-17` would have joined
`TT-11` in the README's *answered by construction* paragraph. **The general
shape: a claim of the form "X depends only on Y" is checkable only as "changing
not-Y does not change X".**

*`EN-12` had nowhere to be false.* *A name renders as exactly one path component*
is supplied for free by a model that has no paths — a `Filename` in a directory,
and the filesystem fact that names are unique per directory. The assumption
table's `controls` column forbids a blank cell, so the mutation needed a
mechanism built expressly for it, and `TT-01.a` had to be restated over
*denotation* rather than over *reading* to have somewhere to fail. **An
assumption a model gets for free is an assumption that model cannot control**,
and the honest response is to build the mechanism rather than to record the
assumption as untested — but the cost is real and belongs in the ledger.

**One false-confidence incident (M8), and it is the fifth cumulatively — but the
first whose cause is the BOUND rather than an unsatisfiable transition.**
`TT-19`'s mutation — *any recovery settles any reserved witness*, dropping the
match — **survived**. It was not a survivor. `TT-19` is mostly refusals, and a
refusal changes nothing, so its check had been written at `2 steps` alongside the
observation commands; but its exception clause is about the matching *recovery*,
and an admitted recovery **settles the witness**, which is a tree change. At two
states no applied recovery exists, the clause was vacuous, and the runner
reported green exactly as it reports a real survivor. At `3 steps` the mutation
is caught in 10s. Stood **~4 minutes**.

This is the same lasso argument entry 026 paid for and the README documents, met
from a direction where it does not look like it applies: **most of the claim was
a read, and one clause was not.** The four prior incidents were all
*unsatisfiable mutations*, whose fix is to re-aim the mutation; this one's
mutation was correct and the command around it was too narrow, so the fix is the
opposite. The distinction is now in the README, because the two failure modes
present identically at the runner's interface and are repaired differently.

**Fire evidence, and its asymmetry, stated for the first time.** Entry 027's rule
— *a mutation is a control only with evidence that it fires* — was applied to all
four. Two of the four witnesses land under their own mutations. The other two
cannot, because each asserts the very thing its mutation removes. The evidence
there is the counterexample itself: **a check that passes can pass for want of a
reachable situation, while a check that fails has the situation in hand.**
`TT-20`'s counterexample shows `InitScaffold`, `Applied` and `CurrentFmt` in one
state, which is the mutated transition executing.

**Cost.** Authoring ≈ **2 h 20 m** for 4 obligations (**M5**: 0.58 h/obligation,
against 0.18 for the selection slice and a markedly higher rate for the name
algebra). The ratio is the point: **this slice's cost is machinery, not claims.**
Four obligations needed a witness species, a witness content, a classification
order, a two-step transaction, a transaction-open state, an interruption action,
three recovery actions, three refusal reasons and two new `Sys` fields — against
the selection slice's one walk and two read transitions for six obligations.
**M6 synchronization: 0.** No second family exists yet.

**M7 — state-space and tooling, and this slice's is a cost imposed on work
already done.** Adding the root layer nearly doubled `TT-03`'s check —
already the file's tightest command, run one filename short of its neighbours —
from **68s to not finishing at all** until the antecedent was narrowed. The cause is not the new claims: it is that four new
transitions (`InitScaffold`, `InitPublish`, `Crash`, and three recoveries) are
encoded into **every** trace the solver searches, including traces of commands
whose guards those transitions can never pass. The repair is the trade the README
already records — narrow the antecedent, not the bound — spelled as an explicit
`CurrentRootThroughout` bundle every earlier command now carries: current format,
no reserved witness, no transaction in flight, and no root-lifecycle action.
`TT-03` came back at **122s**.

The whole-scope figure is the one to carry: **2581s CPU for 59 commands became
3648s for 71 — +41% CPU for +20% more commands**, and none of the added 41% is
spent on the new claims.

**The generalisation, and it is the one this entry contributes.** In a
*temporal* relational model, the cost of a new action is paid by every command in
the file, not by the commands that mention it. A model built claim-group by
claim-group therefore has a **superlinear** cost curve unless each slice pins the
state the earlier slices did not know about — and the pinning is not merely an
optimisation, because writing it down is what exposed that four earlier
commands (`TT-02.a`, `TT-02.b`, `TT-03`, `TT-13.c`) had been assuming a
current-format root all along. `Malformed` is walk-derived, and `TT-18` orders it
behind reserved-witness and format classification, so on a tree carrying a
reserved witness the refusal is `WitnessPending` and not `RefMalformed`. The
arrival of the root layer **exposed** that assumption rather than creating it.

**Counterfactual.** Prose review would have reached `TT-17` and `TT-18` unaided —
both are transcriptions of a table the catalogue already fixes. It would not have
reached the `InitScaffold` cascade omission, because that is a defect of
*enumeration* and the reviewer would have to enumerate correctly to find it. Nor
would it have reached the `TT-19` bound vacuity, which is not a property of the
claim at all but of the instrument. **One obligation's worth of value from the
solver (the catch), one from the mutation pass (the bound), and two
transcriptions.**

**Verdict.** The pattern from entries 026 and 027 holds and gains a third
dimension:

- **The solver earned its place once, on the claim shaped *every operation
  without exception*.** That is the shape worth spending a model on: a universal
  quantification over a set the author must otherwise enumerate by hand. The two
  claims that merely fix an order found nothing, as transcriptions do.
- **Machinery, not claims, is what a modelling session costs.** The unit that
  fits a session is one machinery layer; `guarding-k35` was decomposed on exactly
  that observation before a line was written, and this slice's M5 is the evidence
  it was right.
- **A green suite decays as the model grows.** Two commands that were fast became
  one that would not finish, with no claim changed and no defect introduced. A
  model family checked incrementally needs the *pinning* discipline as a standing
  rule, not as an optimisation reached for when something stalls.

**The seam, exercised.** `models/run.sh --scope task-tree --family alloy
--no-coverage` reports **71 of 71** commands passing and exits 0, in **3648s
CPU** (53m wall on a contended host). Its matrix reads **31 complete cells, 0
declared gaps, 12 empty, of 43** — and every empty cell is `TT-21` – `TT-25`,
which is exactly the `guards` and `ownership` leaves' scope and nothing else. No
command named an obligation the catalogue does not define, and no placement error
was reported.

**The model, as required per entry.** Alloy 6, `org.alloytools.alloy.dist.jar`,
Corretto `21.0.12.1+9-LTS`, SAT4J (distribution default), every command with
`-n`. **Bounds**: `for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug,
2 Digest` throughout this slice, with `1 steps` for the two classification
claims, `2` for `TT-19`'s witnesses, `3` for `TT-19`'s check and `TT-20`'s
interrupted witness, and `4` for `TT-20`'s check and its uninterrupted witness.
The bound at which **each** witness first lands is tabulated in
`crates/grove-task-tree/models/README.md`, separately from the bound its check
ran at. **Fairness**: none assumed; every obligation here is a safety or
reachability claim and none is an eventuality. **Symmetry**: no `exactly` scope,
so Alloy's symmetry breaking is free to collapse isomorphic instances — sound for
the existential witnesses and for the universal checks alike. **Deliberately
omitted**, beyond the earlier slices' list: the root state `Absent` (`SY-05`'s),
the witnesses' filesystem placement, the three reserved classes' mechanisms, and
the bytes a fresh scaffold writes (one opaque digest atom for both scaffolded
entries). **What a green run does not prove**: everything is about the stated
bounds — at most 5 objects, 6 filenames, four states, one working tree and one
cooperating process. `TT-21`'s non-cooperating writer is not excluded by anything
here and is not this slice's claim.

---

### 029 — Concurrency in a file that had none, and two more vacuities of the bound (task-tree guarding)

**Scope.** Task tree, **component-local** (`crates/grove-task-tree/models/`).
`TT-21` – `TT-23`: one snapshot per operation, guards shared for observation and
exclusive for mutation, and a bulk mark that validates before it moves and
converges on re-run. Assumption mutations `EN-07`, `EN-08` and `EN-14` were run
here; `EN-11` is `ownership`'s, and `TT-24` – `TT-25` with it.

**Independence protocol: held.** No Quint model of this subject exists. Neither
`docs/ordinal-fs-tree/models/operations.qnt` nor any Quint file was opened.

**Situation.** The fourth slice of the same Alloy 6 file, and the first whose
subject is not a tree at all. Entries 026 – 028 left `TT-01` – `TT-20` green over
a model with **one** process and **atomic** operations: every action was one
transition, and there was nothing an operation could be *in the middle of*.
`TT-21` and `TT-22` are claims about exactly that middle — what a classification
reads while an operation runs, and what a second process may do while it holds a
guard — so neither was expressible before this slice, and the layer that makes
them expressible is the whole of what it cost.

**What the ADR decided, before the model did.**
[`bulk-marks-are-not-atomic`](adr/bulk-marks-are-not-atomic.md) says a
mutating method **consumes** its `WriteGuard`: *N* marks are *N* critical
sections. That is not a detail — it fixes the shape of the entire layer. An
exclusive guard never spans a state boundary, so `holds` only ever carries
`Shared`, a mutation is one self-contained step, and `TT-22.b` has to be checked
over the mark's **acquisition** rather than over a held exclusive guard. Reading
the ADR first is what kept the observation and the mutation asymmetric; the
symmetric design — open-exclusive, mutate, release — was two states more
expensive on every `TT-23` command and would have contradicted the decision it
was supposed to model.

**One model defect, found by the solver, and it is a lifetime.** `TT-21.b`'s
check found in **9s** that a bulk mark's plan and an observation's listing were
sharing one field. Three transitions: a `Mark` sets the plan and the listing that
validated it, an `Open` replaces the listing, a `Release` clears it — each step
correct on its own — and the plan is left alive with nothing to have been
validated against. The ADR states the missing clause in six words: *a plan
outlives the guard that validated it.* An observation's listing does not. Two
fields, two lifetimes; the model had one of each.

This is the shape entry 026's *shift-order* finding had and entry 028's
`InitScaffold` cascade had: **not a claim the author got wrong, but a
consequence of two correct rules that no one had put beside each other.** It is
also the first of the four where the missing clause was already written down —
in an ADR the leaf brief cited and the author had read. Prose review had the
sentence and did not connect it to a field; the check quantified over every
trace and did.

**A second model defect, found by applying the first one's lesson rather than by
the solver — and it paid for itself twice.** The counterexample was repaired
minimally: `planNm` was added, and `doMark` went on writing the observation's
fields too. That left one branch — the mark that finds nothing of its plan still
live — clearing an observation's listing while the classifications drawn from it
survived, an inconsistency unreachable at the slice's bound and therefore
invisible to every check. The repair is the counterexample's lesson taken all the
way: **`Open`/`Classify`/`Release` write only the observation's fields, `Mark`
only the plan's, and each frames the other**. Completing the separation removed
the latent inconsistency *and* took the slice's four slowest commands from 88s,
42s and 37s to under five seconds each.

The generalisation is worth as much as the defect. **An under-framed transition
is a latent inconsistency and a search-space multiplier at the same time**, and
the second symptom is the one that shows up first: a command that is slow for no
reason the claim explains is worth reading as a framing question before it is
worth reading as a bound question. Three of this file's four performance repairs
have now come from constraining state rather than from shrinking a scope.

**Two false-confidence incidents (M8), and they are the sixth and seventh
cumulatively — but the third and fourth whose cause is the BOUND.** Both `TT-21` mutations
**survived** their first run. `TT-21.a`'s — the mark's acquisition ignores
another process's guard — and `TT-21.b`'s — a classification answers from the
live tree instead of the listing — each reported green at `3 steps` exactly as a
real survivor would. Neither was one. `TT-21` is a claim about an **interval**:
what happens between an operation taking its guard and the classifications it
makes from it. The shortest violating trace is *open · interleave · classify*,
which is four states; at three there is no room for the interleave, the
antecedent is unreachable, and the check is vacuous. Both are caught at
`4 steps`, in 4s and 15s. Stood **~6 minutes** and **~2 minutes**.

Entry 028's `TT-19` incident said what to do about a bound vacuity — widen the
command, do not re-aim the mutation. This pair says **when to suspect one**, and
the rule is cheap enough to apply before writing the command rather than after
the mutation survives:

> When a claim's subject is an **interval** rather than a step, count the states
> the interval needs before anything can happen inside it, and set the bound
> from that count rather than from the slice's default.

Three of the file's seven incidents are now this one failure mode, against four
unsatisfiable mutations. The two present identically at the runner's interface —
a green `check` — and are repaired in opposite directions, which is why the count
is kept separately.

**A third vacuity (M8, averted) the design avoided rather than survived, and it is worth as
much as the two it did not.** In the concurrent scope no grove mutation exists:
`add-leaf`, `insert-leaf`, `decompose`, `retire` and `prune` are single-process
actions. Had the process-scope switch been left free rather than pinned into
`CurrentRootThroughout`, the solver would have been at liberty to satisfy
*every* `TT-01` – `TT-16` witness by picking the concurrent scope — in which
those actions do not fire — and to discharge *every* check vacuously. The whole
of `TT-01` – `TT-20` would have reported green while checking nothing. That is
the file's retained incident verbatim, in a third set of clothes, and the reason
it did not happen is the standing rule entry 028 wrote down after the last one:
**each slice pins the state the earlier slices did not know about.** The rule was
adopted as an optimisation and paid off as a correctness control.

**Two obligations, one mechanism, and the pair of mutations that separates
them.** `TT-21.a`'s mutation breaks `TT-22.b` too, and **no mutation was found
that breaks `TT-21.a` alone**: every cooperating tree change under a held guard
is an applied mark, and an applied mark has acquired, so `TT-21.a`'s violations
are a subset of `TT-22.b`'s. They separate from the other side. `TT-22.b`'s own
mutation — *the plan is validated before the guard is taken, so a refusing mark
never acquires* — breaks `TT-22.b` and leaves `TT-21.a` green, because a refusal
changes no bytes. **A mutation that breaks two obligations is not a failed
control; a PAIR of mutations that cannot tell them apart in either direction
would be.** Recording the overlap and its direction is more informative than
contriving an isolation, and the same reading applies to the `TT-02.b`
overlap entry 026 kept.

**`EN-07` carries no weight in this scope, and the control is what establishes
it.** *Two open descriptions of one directory do not share a lock* is the
assumption behind the ADR's third rejected option — hold Grove's own guard around
the whole run and let the library take its guard inside it, which deadlocks. The
deadlock is really in force: the nested acquisition is unreachable under the
incumbent and admitted the moment the descriptions share. But **every**
`TT-21` – `TT-23` check leaves `EN_07` free, so all six are checked over the
broken assumption as well as the incumbent, and all six are green either way. The
assumption table predicted it — its expected-result column names `SY-11.b`, the
lifecycle scope's — and this is the second such row after `EN-04`. Two of the
five Alloy-owned assumptions turn out to buy nothing any `TT-` claim rests on,
which is a fact about where the catalogue's assumptions do their work rather than
a fact about the controls.

**`EN-08`'s second half needed an argument rather than eleven commands.** The
table's expected result is *every named witness unreachable **and** every
property check still green*. The first half is two `expect_unreachable_`
commands. The second needs none, because no check in this file asserts `EN_08`:
each is already checked over the traces that contain `crash` and over the traces
that do not, and green over the superset is green over the subset. Writing the
argument down is the cheaper control and the more durable one — a later slice
that starts asserting `EN_08` inside a check invalidates it visibly.

**One finding about the catalogue, and it landed there.** `TT-22` serializes an
observation against a mutation, and the catalogue's closed outcome set has
nothing for the caller that is *waiting*. Modelling forced the question, because
the obvious representation — a failed guard as an **absent transition** — makes
`TT-22` true by construction and breaks the file's totality rule besides. The
answer is that the caller sees nothing: `src/tree_access.rs` acquires with
`flock` and no `LOCK_NB`, so the tree lock **blocks** and no invocation returns
while it is held. The set is right and the silence was not; the catalogue's
*Outcomes* section now says so, and names the model's `Deferred` as the
abstraction it is. This is the fourth finding about the catalogue across the
experiment and the first reached by **needing a representation** rather than by
reading the document — the model asked a question the prose had never been posed.

**Cost.** Authoring ≈ **3 h** for 6 obligations (**M5**: 0.50 h/obligation,
against 0.58 for the root-identity slice and 0.18 for selection). The rate is the
interesting number, and it sharpens entry 028's reading rather than repeating it.
This slice's machinery is the **largest yet** — two process atoms, a mode, four
static environment switches, seven per-process `var` fields, five transitions,
two more `Sys` fields, an outcome the catalogue does not have, and a second step
relation — and yet its per-obligation rate is *lower* than the root-identity
slice's, which built less. Machinery is still what a session costs; what changed
is that **six obligations amortise one layer better than four do**, which is an
argument for cutting a slice at the machinery boundary and then taking every
claim that boundary reaches. `guarding-k35`'s three-way cut did exactly that, and
this is the second slice's evidence for it. **M6 synchronization: 0.** No second
family exists yet.

**M7 — state-space and tooling, and this slice inverted entry 028's result.**
Entry 028's headline was that four new transitions took the whole scope from
2581s to 3648s CPU: +41% for +20% more commands, none of it spent on the new
claims. This slice added **five** transitions, an eighth `Result` atom, two `Sys`
fields and seven per-process `var` fields — and took the whole scope from **3648s
CPU over 71 commands to 4002s over 88**: **+10% CPU for +24% more commands**. It
is not free, and the honest number is the sentinel's: `TT-03` went from **132s to
158s**, a real +20%. But it is a different order of imposition, and the
difference is the switch. `Env.concOn` is **static**, so the
whole concurrent branch of `step` is a constant the translator folds away for
every command that pins it off; entry 028's four transitions were reachable
disjuncts of `ordinaryStep` and had to be encoded into every trace whether their
guards could fire or not.

**The generalisation, and it is this entry's contribution to the cost model.** In
a temporal relational model, *narrowing the antecedent* — entry 028's rule — pays
only for the traces the solver still has to enumerate. **Putting the new
transitions behind a static switch removes them from the encoding**, which is a
strictly stronger repair and is available whenever a new layer is a *scope* the
older claims are not stated over. The test for it is a question about the claims
rather than about the solver: *is there a command that needs both the old
transitions and the new ones in one trace?* Here there is not — no `TT-`
obligation is about `add-leaf` running under a second process's guard — so the
switch costs nothing in expressiveness and the whole layer is free to the
fifty-nine commands that predate it. Where the answer is yes, the switch is
unavailable and the narrowing rule is what remains.

**Counterfactual.** Prose review would have reached `TT-22.a` and `TT-22.b`
unaided: shared-for-readers, exclusive-for-writers is a transcription of a rule
the ADR states in one sentence. It would not have reached the orphaned plan
listing — the reviewer would have to hold three transitions and two lifetimes in
mind simultaneously, and the ADR sentence that decides it is nine paragraphs away
from the sentence about guards. Nor would it have reached either bound vacuity,
which are properties of the instrument and not of the claims. **One obligation's
worth of value from the solver, two from the mutation pass, and three
transcriptions** — the same ratio as entry 028, on a slice that cost more.

**Verdict.**

- **The ADR was the design input the catalogue was not.** The catalogue fixes
  *what* `TT-22` and `TT-23` claim; `bulk-marks-are-not-atomic` fixes the shape
  of the machinery that can hold them, and reading it first is what made the
  asymmetric guard obvious rather than a discovery. A model built from the claim
  catalogue alone would have been symmetric, more expensive, and wrong about the
  decision it was checking.
- **A model defect that an ADR already names is still a model defect worth
  finding.** The clause was written, cited by the brief, and read. What the
  solver added was the *conjunction* — the one trace in which two correct rules
  meet — and that is a search no reader performs.
- **Bound vacuity is now the file's dominant failure mode**, three of seven
  incidents, and it has a predictor: an interval claim needs interval-many
  states. Adopting the predictor turns a mutation-pass repair into a
  command-authoring rule.
- **Static switches beat narrowed antecedents where the claims permit them.** The
  cost curve entry 028 called superlinear is superlinear in the transitions a
  command must *encode*, not in the transitions the file *contains* — and a scope
  switch is the difference.

**The seam, exercised.** `models/run.sh --scope task-tree --family alloy
--no-coverage` reports **88 of 88** commands passing and exits 0, in **4002s CPU**
(1h 02m wall). Its matrix reads **37 complete cells, 0 declared gaps, 6 empty, of
43** — and every empty cell is `TT-24` – `TT-25`, which
is exactly the `ownership` leaf's scope and nothing else. No command named an
obligation the catalogue does not define, and no placement error was reported.

**The model, as required per entry.** Alloy 6, `org.alloytools.alloy.dist.jar`,
Corretto `21.0.12.1+9-LTS`, SAT4J (distribution default), every command with
`-n`. **Bounds**: `for 4 but 4 Int, 2 FileObj, 1 DirObj, 4 Filename, 2 Slug,
2 Digest` throughout this slice — narrower than the file's common shape, because
every guarding obligation is flat — with `2 steps` for `TT-23.a`'s witness, `3`
for the `TT-22` and `TT-23` checks, `4` for both `TT-21` checks and `TT-22.a`'s
witness, `5` for the `TT-21` witnesses and `TT-23.b`'s, and `6` for `TT-22.b`'s.
Four commands run at `6 Filename` because `one sig CharterF in Filename` consumes
one atom and they need four *entry* names. The bound at which **each** witness
first lands is tabulated in `crates/grove-task-tree/models/README.md`, separately
from the bound its check ran at. **Fairness**: none assumed, and here that is a
statement rather than a formality — `Deferred` is a wait, so a liveness claim
that a waiting operation eventually acquires would need one. The catalogue makes
none, so a trace in which one process defers forever is admitted and refutes
nothing. **Symmetry**: no `exactly` scope. **Deliberately omitted**, beyond the
earlier slices' list: the fifteen other task-tree operations in the concurrent
scope (one observation shape and one mutation shape, since no obligation
distinguishes which mutation holds the guard), a bulk mark's target subtree (the
task root), the step between a plan's validation and its first rename (one step,
licensed by the guard that covers both), `Refused(DestinationOccupied)` in a
plan, and a lock wait's representation as the non-catalogue outcome `Deferred`.
**What a green run does not prove**: everything is about the stated bounds — at
most 5 objects, 6 filenames, six states, one working tree, and **two**
cooperating processes. A non-cooperating writer is excluded by nothing here and
is not meant to be; that is `EN-06`, and `TT-21.b` exists to say so.

### 030 — Two obligations out of reach, a subset that emptied the model, and an assumption that controlled nothing (task-tree ownership)

**Scope.** Task tree, **component-local** (`crates/grove-task-tree/models/`).
`TT-24` and `TT-25`: fail-closed ownership, and done-ness derived from the
absence of a live leaf. Assumption mutation `EN-11` was run here, the last of the
five Alloy-owned rows. With this slice the **Alloy column of the task-tree scope
is complete** and only the Quint column is empty.

**Independence protocol: held.** No Quint model of this subject exists. Neither
`docs/ordinal-fs-tree/models/operations.qnt` nor any Quint file was opened.

**Situation.** The fifth and last slice of the same Alloy 6 file, and the first
whose headline result is what the model **cannot** say. `TT-24` is one artifact —
someone else's bytes at a name Grove reserves — met in three contexts, and the
catalogue fixes the outcome of each rather than letting a model choose:
`Refused(ReservedNameOccupied)` for an ordinary operation,
`Blocked(OwnershipConflict)` inside a finish or recovery transaction, and a
decline-and-continue for the quarantine reaper. This file has an ordinary
operation. It has no transaction of that kind, no `Blocked` in its `Result` set,
and no reaper. Two of `TT-24`'s four obligations are therefore **declared gaps**,
and the reason each is out of reach is different: `TT-24.c` lacks an *outcome*,
`TT-24.d` lacks a *subject*.

**Declaring the gap was the substantive decision of the slice, and the
alternative was available.** A ninth `Result` atom and a token transaction would
have made both cells green in an afternoon. What that buys is a model answering
`TT-24` **by construction**: the catalogue's *one artifact, three contexts, one
decided outcome* table exists precisely because a model free to pick the outcome
has decided the question it was built to check. The declared gap is the weaker
claim and the honest one, and the runner reads it — a gap is a covered cell, so
the column closes without pretending.

**A finding about the catalogue, and it is the fifth of the experiment.**
`TT-24` is the only `TT-` claim whose obligations are stated over `FN-`
*contexts*, while the runner's placement rule sends every `TT-`-prefixed command
to the task-tree directory. Two of its four cells can therefore only ever be
filled by the finish scope, in either family — the Quint column will meet the
same wall for the same reason. Whether `TT-24.c` and `TT-24.d` should be
re-stated as `FN-` obligations is `formal-synthesis-k16`'s to settle; what this
slice contributes is that the question is structural rather than a modelling
convenience, and that the placement rule and the claim numbering disagree here
for the first time.

**One model defect, self-inflicted, and it emptied the model rather than
breaking it.** `reservedRefusal` had to split in two — a reserved *witness*
refuses with `WitnessPending` and names its recovery, an occupant Grove cannot
classify refuses with `ReservedNameOccupied`, names the entry, and names no
recovery. The witness half was first written `Slot.occ in WitnessClass implies …`.
`in` is **subset**; `Slot.occ` is a `lone` field; the empty set is a subset of
every set. On a root with no reserved artifact at all the antecedent was
therefore **true**, every ordinary transition was forced to refuse
`WitnessPending` against its own applied branch, and the transition relation
became **unsatisfiable**.

Every check in the file then passed. Not *some* — an unsatisfiable transition
relation discharges universally quantified temporal properties trivially, so the
four new checks reported green within seconds of being written, and `TT-19`,
`TT-24` and `TT-25` would all have shipped that way. This is the file's retained
false-confidence incident met a fourth time and from a new direction: the first
three were vacuous **checks**, and this was a vacuous **model**.

**What caught it is the cheapest control in the file and the one worth
generalising.** Not a check — the checks were the victims. Two `witness_` runs
did not land, which the discipline treats as a fact to explain rather than a
bound to raise; and the explanation came from a **reachability bisect against the
previous revision**: copy the pre-change file, run the same probe against it,
find the situation in 7s, and bisect the edits until the probe goes dark. Six
one-command runs localised it to the split.

> A witness that stops landing is a report about the **model**, not about the
> bound. Before widening anything, re-run the probe against the last revision
> that was green — a green probe there names the change as the cause, and a red
> one says the situation was never reachable.

The narrow lesson is worth stating too, because nothing about it is Alloy-
specific: **a `lone` field's emptiness makes `in` an antecedent that fires when
nothing is there.** The correct spelling — `some (Slot.occ & WitnessClass)` —
was already in the file, in `doRecover`, written correctly by the slice that
introduced the slot. The shorter form was reached for while *splitting* a
predicate, which is where the two spellings stop being equivalent.

**M8, and both of this slice's incidents with the hours they stood.** The
subset antecedent stood **~7 minutes** — four checks written, run green, and the
two unlanded witnesses read in the same batch — and never left the session.
`TT-24.a`'s bound vacuity, below, stood **~4 minutes**. Against them on the same
ledger: the four genuine failing checks of the mutation pass cost **~25 minutes**
of run time and produced four confirmed controls. **M3 is not scorable for this
slice's model defect**, and the reason is worth a line: no counterexample
produced it. The instrument was an **absent instance**, which the 0–3 scale has
no value for — a measure shaped for counterexamples is silent about the failure
mode that mattered most here.

**One false-confidence incident of the bound family (M8), the fifth, and it
extends the predictor into a second dimension.** `TT-24.a`'s mutation —
`initialise-root` clears a directory it did not create — **survived** its first
run at the guarding slice's `2 FileObj`, reporting green exactly as a real
survivor would. It was not one. The claim mentions two objects, an actor and
something unprovable; the transition that violates it brings a charter and a
first leaf of its **own** before it can trample anything, which is three files
before the tree holds one. At two the mutated transition could not fire. Caught
at `3 FileObj` in 22s.

Entry 029's predictor was about states — *an interval claim needs interval-many
states*. This is the same failure in the object dimension, and the two combine
into one authoring rule that costs nothing to apply before the fact:

> The bound must hold the **machinery of the transitions the obligation
> quantifies over**, not only the objects and states the obligation names.

**`EN-11` controls one fewer claim than the catalogue said, and the run is what
established it.** *Any well-formed tree is reachable by hand edit* is listed as
controlling `TT-02`, `TT-03`, `TT-13.c`, `TT-16`, `TT-24.b` and `TT-25`. Five
hold: with `hand-edit` removed, a species mismatch, a malformed node, two live
finish leaves, an occupied reserved name and a two-level-deep live leaf are all
unreachable. **`TT-16` does not**, and the reason is not subtle once run: a
resolved *terminal* entry is something Grove's own actions build — allocate,
retire, resolve — so its witness never needed the assumption. The row is
corrected in the catalogue, and the model ships a **positive** control
(`witness_EN_11_a_resolved_terminal_entry_needs_no_hand_edit`) rather than an
`expect_unreachable_` written to the table instead of to the model. This is the
third assumption row this file has found to carry less weight than the table
predicted, after `EN-04` and `EN-07` — but the first where the table was *wrong*
rather than *self-aware*, and the difference matters: `EN-04` and `EN-07` named
their own non-dependence in the expected-result column.

**Removing the action was not removing the assumption, and that is `EN-11`'s own
methodological finding.** The first attempt guarded `doHandEdit` with the switch
and stopped there. Every named witness stayed reachable, because this model
realises `EN-11` in **two** places: the `hand-edit` action, and the
**unconstrained initial state** that the README's `3 steps` argument rests on in
as many words — *every single transition is reachable from state 0*. An
exercise-removal that takes away the action and leaves the free initial state
removes nothing and reports green. The switch takes both, and the scope it leaves
is a world Grove's own actions had to build from an empty task root — which then
needs `5 steps` and the root-lifecycle actions in scope to be a fair test at all,
and a companion witness showing Grove still builds a tree under it.

> An assumption realised as a *modelling decision* as well as an action is
> removed only when both go. The tell is a removal under which nothing becomes
> unreachable.

**`TT-25` is answered by construction, and this is the second entry in that
family.** *A node is never marked* cannot fail here: the grammar gives a `NodeSp`
name no outcome infix at all, so a marked node is not spellable and no mutation
of an action can produce one. Entry 026 recorded the same shape for `TT-11`'s
*depends on no state outside the tree* — a model cannot check the absence of a
variable it does not have. What both slices did instead of faking a command is
name the falsifiable residue: for `TT-25`, that the transition making a node done
writes **nothing** to the node, and that done-ness reads the **whole** subtree —
the latter stated over `d.^(~loc)` rather than over the model's own `nodeDone`,
exactly as `TT-14` names `fPos` rather than `precedes`, so that re-defining the
mechanism has somewhere to fail. It does: re-defining done-ness to read only a
node's children breaks `TT-25.b` and leaves `TT-25.a` green.

**A modelling-economy result, and it is reusable.** `TT-24.b`'s refusal reason
**carries the entry**, so the occupant had to be a real filesystem object — but
what makes it *reserved* did not have to be a `Filename`. The reserved name is
represented as a **pointer from the slot** (`Slot.occAt`), the mirror of what
already made a witness reserved: the slot holds it. The alternative — the
reserved spelling as an atom, `one sig ReservedF in Filename` — would have
consumed a `Filename` atom in **every** command in the file, and the nine-minute
`TT-07` witness runs at six with nothing spare.

> Represent membership of a reserved namespace as a **relation from the reserved
> thing**, not as an atom inside the namespace. The first costs one field on a
> singleton; the second is paid for by every command that never mentions it.

**One mutation overlap, recorded rather than contrived away.** `TT-25.a`'s
mutation breaks `TT-09.d` as well, and it must: the only transition in this model
that can make a node done is a rewrite, and `TT-09.d` already says a rewrite
touches nothing but its target. The pair separates from the other side —
`TT-09.d`'s own mutation, *a rewrite may move the entry*, breaks `TT-09.d` and
leaves `TT-25.a` green. Second such pairing in the file after
`TT-21.a`/`TT-22.b`, and the same reading applies: a mutation breaking two
obligations is not a failed control; a pair that cannot tell them apart in
**either** direction would be.

**Cost.** Authoring ≈ **1.5 h** for four obligations and two declared gaps
(**M5**: 0.38 h/obligation against 0.50 for guarding, 0.58 for root identity,
0.18 for selection). Machine time is excluded, as in every entry — the suite runs
unattended. The rate is the lowest of the three machinery-bearing slices and the
reason is structural rather than a matter of the claims being easy: **this is the
first slice that added no transition at all.** What it added is two `lone` fields
on singletons, a ninth `Result` atom, a static switch, and two derived functions.
`TT-24` and `TT-25` needed no new *action* because the actions that violate them
already existed; what was missing was something for a refusal to **name** and
something for done-ness to be **read from**. That makes it cheap to *author* —
but not free to *run*, and the difference is M7's, below: state costs where a
trace is widest even when it costs a tight command nothing. **M6 synchronization: 0.** No second
family exists yet.

**M7 — state-space and tooling, and this slice overturned the file's own
measuring practice.** The whole scope went from **4002s CPU over 88 commands to
6888s over 103** (1h 57m wall), which reads as +72% and is not. Two corrections
have to be applied before that number means anything, and finding both was most
of this slice's M7 work.

**Whole-suite totals do not compare across sessions.** `TT-11`'s check —
untouched since the selection slice — costs **61s** in the measurement behind
entry 029's figures and **77s on that same unmodified file** re-run during this
slice. The host is ~24% slower, independently confirmed by `witness_TT_07`
landing at 668s on the baseline against a drift-adjusted prediction of ~680s.
Corrected, the pre-existing scope is ~4970s, so this slice's imposition on it is
about **+15%**, plus ~860s for its own fifteen commands.

**One sentinel is not enough, and `TT-03` is the wrong one for a slice like
this.** Entries 028 and 029 adopted `TT-03` as the honest number because it is
the file's **tightest** command, run one filename short of its neighbours. That
makes it sensitive to a new *transition* — which is what those slices added — and
nearly blind to new *state*. A same-host A/B:

| command | pre-slice | with ownership | Δ |
|---|---|---|---|
| `TT-03` (tightest) | 156s | 138s | −12% |
| `TT-11` | 77s | 75s | −3% |
| `TT-15.a` | 51s | 56s | +10% |
| `witness_TT_07` (largest) | 668s | **987s** | **+48%** |

Read the sentinel alone — as this session first did, and wrote down before
checking — and the slice looks free. It is not. A ninth `Result` atom and two
`lone Obj` fields are present in **every state of every trace**, so they are paid
where the trace is **widest**, and the tightest command in the file is the one
least able to show it.

> Measure the **largest** command as well as the tightest. The tight one prices
> transitions; only a wide one prices state.

The three slices together still give a cost model, with that correction folded
in: four reachable transitions cost +41%; five behind a static switch cost +10%;
two fields and an atom cost ~+15% overall and **+48% on the widest command**.

The three numbers together are the file's cost model, and it is not about size:

> In a temporal relational model, cost is paid per **transition** encoded into
> every trace, and barely at all for state a command can pin. Four reachable
> transitions cost 41%; five behind a static switch cost 10%; two fields, an
> atom and a switch cost what this slice cost.

**Counterfactual.** Prose review reaches `TT-25` unaided — *a node is never
marked* is a sentence, and the grammar makes it true. It reaches the `TT-24.c`
and `TT-24.d` gaps too, once someone asks where `Blocked` would live; the model
forced the question earlier and made the answer a runner-visible artifact rather
than a note. It does **not** reach the subset antecedent: no reader checks
whether an antecedent is vacuously true on the empty set, and the symptom —
everything green — is the one prose review is least equipped to doubt. Nor does
it reach `EN-11`'s over-claim, which is a fact about what Grove's own actions can
build and needs a search to establish. **One obligation's worth of value from the
mutation pass, one from the witness discipline, one from the assumption control,
and two transcriptions.**

**Verdict.**

- **A declared gap is a result, not a shortfall.** Two of `TT-24`'s four cells
  are closed by saying what this model cannot express and why. The alternative
  was a model that answers the claim by construction, and the catalogue's own
  three-context table exists to forbid exactly that.
- **The witness discipline paid for the whole slice.** Every check the slice
  added passed while the model could do nothing at all. Only the reachability
  probes noticed, and a bisect against the previous revision localised the cause
  in six one-command runs. A suite of checks alone would have shipped it.
- **Vacuity now has a two-dimensional predictor.** States for interval claims,
  objects for transition machinery — and both are cheap to apply while writing
  the bound rather than after a mutation survives.
- **An assumption is removed only when every realisation of it is.** `EN-11`
  lived in an action and in a modelling decision about the initial state, and the
  removal that took only the action controlled nothing while reporting green.

**The seam, exercised.** `models/run.sh --scope task-tree --family alloy` —
**without** `--no-coverage`, which is the point — reports **103 of 103** commands
passing and exits 0, in **6888s CPU** (1h 57m wall). Its matrix reads **41
complete cells, 2 declared gaps, 0 empty, of 43**. No command named an obligation
the catalogue does not define, and no placement error was reported. This is the
first run in the experiment where a scope's coverage is asserted rather than
deferred, and it is what closing a column looks like: the flag comes off the
README's run line, and the runner rather than the prose is what says the column
is complete.

**The model, as required per entry.** Alloy 6, `org.alloytools.alloy.dist.jar`,
Corretto `21.0.12.1+9-LTS`, SAT4J (distribution default), every command with
`-n`. **Bounds**: `TT-24.b`'s pair at the guarding slice's narrow `2 FileObj,
1 DirObj, 4 Filename` with `2` and `3 steps`; `TT-24.a`'s witness there too and
its **check** at `3 FileObj, 2 DirObj, 6 Filename, 3 steps`, widened by the
mutation rather than by the claim; `TT-25.a` at the file's common
`3 FileObj, 2 DirObj, 6 Filename, 3 steps`; and `TT-25.b` at **`3 DirObj`**,
wider in that dimension than anything else in the file, because a node, a node
beneath it and the task root is the shallowest tree in which a subtree reading
and a children reading of done-ness can differ. The seven `EN-11` controls run at
`5 steps`, the shortest run-up in which Grove's own actions build anything from
an empty root. The bound at which **each** witness first lands is tabulated in
`crates/grove-task-tree/models/README.md`. **Fairness**: none assumed; no claim
in this slice is a liveness claim. **Symmetry**: no `exactly` scope.
**Deliberately omitted**, beyond the earlier slices' list: the reserved *name* as
a spelling (a pointer from the slot instead), a finish or recovery transaction
and the `Blocked` outcome (`TT-24.c`, declared gap), the quarantine reaper
(`TT-24.d`, declared gap), and a marked node — which is not an omission but an
unspellable, since the grammar gives a node name no outcome infix. **What a green
run does not prove**: everything is about the stated bounds — at most six
objects, six filenames, five states, one working tree and one process for these
two claims. In particular `TT-24.a` is green over the actions this file has, and
the two contexts it does not have are exactly what the declared gaps say.

---

### 031 — Seven preconditions and six reasons, and three mutations that were not controls (finish entry surface)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-01` and `FN-05` – `FN-08`: the finish
transaction's **entry surface** — confirmation, the closed preflight
precondition set, task-root identity pinning, the deletion fingerprint, and the
quarantine operand. Assumption mutation `EN-02` was run here. This is the first
slice of a new scope and a new file; fifty-three of the scope's sixty-one alloy
cells remain empty and are named as such by the runner.

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**Situation.** `finish-k8` was decomposed before this slice ran — sixty-one
obligations against `task-tree-k7`'s forty-three, plus a lane parameter that
scope did not have — into five children cut along the machinery each claim group
needs. This is the first, and it is the only one of the five whose obligations
all end in a refusal or in a transaction that is never entered. That is what
makes it a vertical slice rather than a foundation layer: it needs the
transaction's entry and none of its body, and it is verifiable on its own.

**The material finding is a counting mismatch in the catalogue, and the model
found it by trying to write the witnesses.** `FN-05.a` fixes the preflight
precondition set as *closed and exactly* seven-membered, and requires *each of
the seven, reached* as its witness. The catalogue also fixes seventeen closed
refusal reasons — and never states the mapping between the two lists. Writing
seven witnesses is what makes the mapping unavoidable, and it does not exist:

- the **first** member produces no refusal at all. Confirmation is an operator
  input Grove cannot verify (`EN-15`), so declining is a transaction that is
  never entered rather than one that refuses — which is `FN-01.a`'s content read
  from the other end;
- the **third** and **fourth** — an unsupported layout, and a quarantine target
  unreachable from the transaction's own operands — are the **same** reason.
  `SY-03` says no later gate consults an earlier layout check and each
  revalidates against its own operands, which makes them one question asked at
  two gates. The reason names the question; only the gate distinguishes them.

Six reasons cannot witness seven members. A family that reports six witnesses and
calls `FN-05.a` covered has silently lost a member, and nothing in the runner
would notice, because coverage is per obligation and `FN-05.a` is one. The
correction is in the catalogue: `FN-05.a` now says the seven are not
distinguishable by outcome, that a family answering it needs an observable of its
own, and that introducing one is what the obligation requires rather than a gap.
This model's is `Sys.why`, declared as an abstraction in the family `README.md`.

> **[disposed by `closed-set-additions-k74`]** **This entry is the first of the three that made the pattern, and it is the one the disposition DECLINED to treat as a set gap.** Read together with 032 and 033 the shape looked like an under-populated reason set; read against this entry's own sentence — *the reason names the question; only the gate distinguishes them* — it is the set answering the right question at the right grain. `Sys.why` distinguishes **gates**, which is a modelling need, and a device invented to tell two gates apart is not evidence that the contract owes a reason per gate. So the catalogue's rule is now stated rather than merely relied on (*a reason names the question that was asked and answered no, never the gate that asked it*), and the widening rule that follows from it is narrow: the set gains a member exactly when a **scope** asks a question no member names. Three of the five referred cases were that; this one was not, and neither was `FN-01`'s. Record: `docs/adr/a-refusal-leaves-nothing-standing.md`, clause 2.

> `M1 alloy-only` · `M2 refusal` · `M3 = 2` — the trace names the transition and
> the reported precondition, but the *absence* of a seventh distinguishable
> outcome is read off the signature rather than out of any one trace ·
> `M4 = none` — there is no defect to write a failing test against; the shipped
> preflight refuses correctly and the catalogue simply under-described what
> witnessing it requires. This is the pre-registration's third borderline,
> and by its own terms it falsifies **H7**.

**One false-confidence incident, and the predictor that would have caught it was
already written down.** Every check in the file was first written at `3 steps`,
which is the sibling scope's minimum for a behavioural command, and the whole
suite ran green — twenty-three commands, checks and witnesses alike. Then the
witness bounds were measured, by re-running each witness at `1..5 steps` and
taking the first that lands. Four of the fourteen first land at **4**, and one of
those four is `witness_FN_05a_p3_layout_unsupported`: reaching an unsupported
layout *at the preflight* needs an intervening `TopologyChange`, because the
lease gate's recorded verdict proves the layout was supported when it was taken.

So at `3 steps` `FN-05.a` — the check whose whole content is *the preflight
refuses exactly when some member of the closed set fails* — ran green having
never once seen the third member fail. It was believed green for about twenty
minutes. `task-tree-k7`'s second bound-vacuity predictor names it exactly: **the
bound must hold the machinery of the transitions the obligation quantifies
over**, not only the objects the obligation names. The member is not an object;
it is a state reachable only across an environment action.

> A check's bound is not a property of the check. It is the largest bound at
> which any witness of the obligation the check answers first lands.

Every check in the file now runs at 4. Applying the rule cost one re-run and
found the vacuity in the same pass; applying it while writing the bounds would
have cost nothing.

`M7` (bounds), and `M8` — a believed green, twenty minutes standing, caught by
the witness-bound measurement rather than by any check.

**Three of the nine mutations were not controls, and each failed differently.**
The pass is one mutation per obligation, run before the green is believed. Six
killed their check first time. The other three are the slice's most transferable
result, because all three *reported as survivors*:

- **Two were unsatisfiable.** `FN-05.b` and `FN-05.c` say a failed precondition
  leaves the tree and the repository byte-identical, and the mutations added
  `Slot.occ' = Reserved` and `Repo.rev' != Repo.rev` into `doPreflight`'s refusal
  branch — which already sat underneath `treeSame and repoSame`. The branch
  became unreachable, the check stayed green for want of an antecedent, and the
  report read *SURVIVED*. The fix is to **remove** the frame condition rather
  than contradict it. This is the third independent recurrence of one rule, met
  from a third direction — `selection-k34` found it in an antecedent the model's
  facts forbade, `ownership-k38` in a subset antecedent that emptied the whole
  transition relation, and here in a frame condition:

  > **A mutation the model cannot execute is not a control**, and it reports
  > exactly as a surviving one does. Every mutation needs evidence that it
  > *fires* — one existing witness re-run under it, still landing — and the
  > evidence is cheapest to take at the moment the mutation is written.

- **One was a semantic no-op.** `FN-01.a`'s second conjunct — confirmation
  changes only by the world's own action, which is what *and is never attested*
  means in a model — was mutated by having `doTxnOpen` set
  `Op.confirmed' = Confirmation`. But `doTxnOpen`'s guard already requires
  `some Op.confirmed`, and `Op.confirmed` is a `lone` field of a `one sig`, so
  the assignment changes nothing whatever. Moved to `doDecline`, whose guard is
  `no Op.confirmed`, it killed the check immediately. A no-op mutation is a
  fourth species of the same failure: not an unreachable branch, but a reachable
  one in which the mutation is the identity.

**The no-op mutation found a real hole, which is the argument for chasing a
survivor rather than recording it.** Investigating why it changed nothing showed
that **no command in the file exercised the `Confirm` transition at all**. Every
witness could satisfy *some confirmation is present* from the unconstrained
initial state, so the conjunct forbidding any other action from setting it was
being checked over a transition the file never demonstrated. The check was not
vacuous — the transition is reachable, and Alloy quantifies over all traces — but
the *witness discipline* had a hole in it that the coverage matrix cannot see,
because the runner counts a witness per obligation and there was one.
`witness_FN_01b` now requires the `Confirm` action, at a cost of one state.

> A witness proves its obligation's situation is reachable. It does not prove
> that the transitions the obligation's *check* quantifies over are exercised,
> and the coverage matrix cannot tell the two apart.

**Two decisions this slice made for the whole subtree.**

- **The tree abstraction is coarse: no filename grammar.** No `FN-` claim
  quantifies over names, positions, keys or slugs, so an entry here is an opaque
  object with a type and a role. What the seven preconditions actually read is
  leaf liveness, the finish/ordinary distinction, an undigestible entry type and
  a tracked/untracked split — nothing finer. Rebuilding `task-tree.als`'s
  grammar would have been machinery no claim in this scope reads, and the two
  files stay independent for the same reason the runner keeps them in separate
  scope directories.
- **The lane is a signature parameter from the first command**, even though
  nothing in this slice differs by lane. `EN-16`'s control is a *collapse* to one
  lane, and a parameter retrofitted in a later slice is a parameter every earlier
  command was never checked under. It also earns its place immediately: an absent
  lane **is** an unsupported layout, which is how `FN-05.a`'s third member gets a
  state to fail in at all.

**Cost.** Twenty-three commands, **38s wall** for the whole file, of which two
commands are 9–10s and the other twenty-one are under a second each. The two are
`witness_FN_05a_p5` and the `EN-02` control; the second is the expensive shape by
construction, since establishing that *no* instance exists means exhausting the
space rather than stopping at the first model. Against `task-tree.als`'s 6888s
CPU for 103 commands, this file is nearly free — and the reason is that it has no
`Int`: there are no positions and no keys in this scope, so the bitwidth
arithmetic that dominates the sibling file has no counterpart here. That is a
finding about where a temporal relational model's cost actually goes, and it
sharpens the cost model rather than adding to it. **These figures do not compare
across sessions**; `task-tree-k7` measured one untouched command at 61s in one
sitting and 77s in another.

**`EN-02`, as the assumption table requires.** Class *exercise-removal*, so the
expected result is that a **named witness becomes unreachable** while the
property stays green — not that anything fails.
`expect_unreachable_EN_02_a_single_device_layout_cannot_fail_the_operand_gate` is
`witness_FN_08`'s body at `1 Device`, and finds no instance, while `FN-08`'s
check stays green. The control was widened to `4 steps` — one state wider than
the witness's first landing — because *no instance* at exactly the witness's own
bound is the weaker statement.

**Counterfactual.** Prose review reaches `FN-06` and `FN-07` unaided; both are
single sentences about a single gate. It reaches `FN-08` too, since the catalogue
already states the two-gate distinction in words. It does **not** reach the
seven-versus-six counting mismatch: that needs someone to sit down and try to
name seven distinguishable outcomes, which is precisely what writing seven
witnesses is and what reading a claim is not. Nor does it reach the `3 steps`
vacuity, whose symptom is a green run. **One obligation's worth of value from the
witness-bound measurement, one from the mutation pass, and six transcriptions.**

**Verdict.**

- **Writing the witnesses is what audits the catalogue.** The check for
  `FN-05.a` could have been written, run and believed without the mismatch ever
  surfacing. Enumerating seven reachable states is what made six reasons visibly
  too few, and it is the second time in this experiment that the *witness* half
  of the discipline produced the finding while the check half produced nothing.
- **A surviving mutation is a hypothesis, not a result.** Three of nine survived
  and none of the three was a fact about a check. Two were unsatisfiable, one was
  the identity, and chasing the third is what found a hole in the witness set.
  Recording a survivor without investigating it would have recorded three
  falsehoods.
- **A check bound is derived, not chosen.** The largest first-landing bound among
  an obligation's witnesses is the number; picking the file's conventional
  minimum instead is how a check ends up green over a member it never reached.
- **What a green run of this slice does not prove is most of the protocol.**
  `FN-05.b` and `FN-05.c` are carried entirely by frame conditions on
  `doPreflight`, because the entry surface contains no step that mutates
  anything. They separate two reachable behaviours only once evacuation exists —
  the `witness` sibling's — and until then they say *the claim is stated and the
  instrument works*, not *the protocol was tested*. The family `README.md` says
  so in those terms rather than leaving a reader to infer it from a green run.

**The seam, exercised.** `models/run.sh --scope finish --family alloy
--no-coverage` reports **23 of 23** commands passing and exits 0. Its matrix
reads **8 complete cells, 0 declared gaps, 53 empty, of 61**. No command named an
obligation the catalogue does not define, and no placement error was reported.
The `--no-coverage` is on the run line deliberately and comes off when the
column closes, which is four sibling leaves away.

**The model, as required per entry.** Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`, SAT4J (distribution
default), every command with `-n` and `-t text`. **Bounds**: the common shape is
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, N steps`; every `check` at
`4 steps`, witnesses at 3 or 4, and the `EN-02` control at `1 Device, 4 steps`.
No `Int` in the file. The bound at which **each** witness first lands is
tabulated in `crates/grove-finish/models/README.md`. **Fairness**: none assumed;
no obligation in this slice is a liveness claim. **Symmetry**: no `exactly`
scope. **Abstractions**: the coarse tree described above; `Sys.why` as a
model-only precondition observable; the lease gate as a recorded verdict rather
than a transition, since `SY-02` is the lifecycle scope's claim and this file
consumes it; and the unconstrained initial state, which is `EN-11` cashed out as
a modelling decision rather than as a `hand-edit` action — a licence `exits` will
have to remove rather than an action, when it runs `EN-11`'s own control.
**Deliberately omitted**: the entire transaction body — the witness build,
evacuation, the manifest, the commit, the dispositions, the quarantine, the
cleanup marker and recovery. `Slot` is in the signature and is never occupied,
which is what gives `FN-05.b` something the preflight *could* have mutated.
**What a green run does not prove**: everything is about the stated bounds — at
most three entries, two devices, four states, one transaction and no lane
distinction — and, beyond the bounds, the four caveats in the family `README.md`.

---

### 032 — A licence that was about the tree, and a refusal the closed set cannot name (finish witness)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-09` – `FN-13`: the **reserved witness** —
build under a preparing name, publish by one atomic rename, evacuate the root
into it beneath a manifest marked ready last, and never commit it. Eight
obligations, seventeen new commands, no assumption mutation (`EN-08`'s
exercise-removal is `exits`'). The scope's empty alloy cells fall from
fifty-three to **forty-five**.

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**Situation.** `entry-k39` left `finish.als` green over a two-step transaction in
which every outcome is a refusal. This slice adds the transaction's *body*: six
steps, `crash` between any two of them, and `discard` as the recovery an
unpublished witness admits — the file's first reachable-transition addition, and
its first claim (`FN-11`) whose witness is an **interval** rather than a state.

**The material finding is that `EN-11`, as this corpus cashes it out, silently
weakens every claim about tree shape — and it took a six-step body to make that
visible.** `EN-11` says *any well-formed tree is reachable by hand edit*, and
both scopes implement it as an **unconstrained initial state** rather than as a
`hand-edit` transition, because it is cheap and it saves every witness a run-up.
`entry-k39` records it as an abstraction and nothing went wrong: its transactions
are two steps long and its witnesses need at most a just-opened transaction.

With a six-step body it goes wrong in two distinct ways, and only one of them has
a fix.

- **The volatile half is a modelling defect, and it broke three checks at once.**
  An unconstrained initial state does not just hand-edit the *tree*; it also
  hands you a `Txn.phase` in the middle of the body — a running transaction
  nobody started. `FN-12.a` failed on a manifest half-written by no step
  (`Manifested`, only the anchor field present); `FN-11` failed on a published
  witness over an **absent task root**; `FN-12.b` failed on an undigestible entry
  inside an entered transaction the preflight would have refused. All three are
  one counterexample wearing three hats, and one line fixes it: constrain state
  0's phase to `Fresh + Opened` — where a *process* starts — and leave the disk
  entirely free, which is what `EN-11` actually licenses.
- **The tree half has no fix, and that is the reusable rule.** `FN-09.b` — *no
  preparing witness ever holds an evacuated entry* — written the obvious way,
  `always (Slot.occ = Preparing implies no Slot.wHolds)`, fails at state 0 on a
  hand-edited preparing witness with something inside it. And it *should*: under
  the free-initial-state licence, **every "never" claim about tree shape is false
  unless it is restated as a claim about what the protocol does**. Making it a
  `fact` would make the check vacuous and its mutation unsatisfiable, which is
  the exact trap entry 031 records twice. It is stated over the transition
  relation instead.

> `M1 alloy-only` · `M2 structure` · `M3 = 3` — the three counterexamples name
> the state and the field, and reading *why* that state is unreachable-in-fact
> takes the phase machine as well as the trace · `M4 = none` — there is no
> shipped defect here; the model's own licence was over-broad, and what changes
> is the model and the corpus's rules for writing one.

**This is the third bound-and-vacuity predictor the corpus carries**, and it sits
beside `task-tree-k7`'s two rather than replacing them:

1. an **interval** claim needs interval-many states;
2. the bound must hold the **machinery of the transitions** the obligation
   quantifies over;
3. a **shape** claim under a free initial state must be restated over the
   **transition relation** — and if the model also leaves *volatile* state free,
   the initial state must be narrowed to where a process starts.

Predictor 1 was applied before the fact and worked: `FN-11`'s witness was budgeted
for the interval from the start and landed at nine states, first try. Predictor 3
was learned from three failures in one run.

**The second material finding is a hole in the catalogue's closed refusal-reason
set, and — for the second time in this scope — a *witness* found it, not a
check.** `FN-13`'s stated witness is *a commit attempted while the witness is
tracked, **refused***. None of the seventeen closed refusal reasons names a
tracked witness. The model reports it under `WitnessPending` — the closest true
member, an artifact at a reserved name Grove can prove is its own — and keeps the
case distinguishable through the model-only `Sys.why`, which is the same device
`FN-05.a`'s two `LayoutUnsupported` members already needed. The consequence is
operator-visible: the reason cannot say that the **repository**, not the
filesystem, is what is blocking.

The two documents also disagree about the outcome's *kind*.
`task-tree-transactions-fail-closed` says a tracked witness "keeps the witness
unwalkable as **Recovery pending**" — a `Blocked`, which is what `TT-24`'s own
context table implies for a transaction that has already mutated — and the
catalogue says **refused**. This model followed the catalogue, because the
catalogue is the sole input to the formal phase. `formal-synthesis-k16` picks the
exit: add a reason to the closed set, or restate `FN-13` as a block.

> **[disposed by `closed-set-additions-k74`]** **The second exit was taken, and the reason set gained nothing here.** The commit is attempted only after publication and evacuation (`FN-11`), so an effect stands that the action can neither complete nor undo — a `Blocked`, diagnosis `RecoveryPending`. The catalogue's `FN-13` witness is corrected, `finish.als` moved to meet `finish.qnt`, and `task-tree-transactions-fail-closed` and the shipped post-commit verification both already said so. A block carries a diagnosis rather than a reason, so `W8WitnessTracked` stays a model-only observable and no longer maps through `reasonOf`. **This model's method was right and the sentence it followed was wrong**, which is why the correction landed in the catalogue rather than in this column. The operator-facing residue — telling the repository from the filesystem in the diagnostic — is `handoff-audit-k66`'s. Record: `docs/adr/a-refusal-leaves-nothing-standing.md`.

> `M1 alloy-only` · `M2 refusal` · `M3 = 2` — the trace names the transition and
> the outcome, but the *absence* of a fitting reason is read off the closed set
> rather than out of any trace · `M4 = none` — there is no defect to write a
> failing test against; the catalogue under-described an outcome it requires.
> Like entry 031's counting mismatch, this falsifies **H7** by H7's own terms —
> and it is the same failure mode twice in one scope: **a catalogue that fixes
> two closed sets and never states the map between them.**

**Caught / missed.** Caught: the three initial-state counterexamples above; the
missing refusal reason; and one hole in the file's own coverage — no command in
`entry-k39`'s twenty-three ever reached `doPreflight`'s **success** branch, since
all fourteen of its witnesses are refusals. That is the same shape as the
undemonstrated `Confirm` transition entry 031 found, discovered the same way, by
asking what the witnesses actually *execute* rather than what they assert.
`witness_FN_09a_the_transaction_is_entered_by_a_preflight` is the fix and the
licence every body witness now runs on. Missed, in the sense of not attempted:
`FN-24.b`'s question about whether the six steps are a *complete* set of crash
boundaries. The step list is written as one named thing (`bodySteps`) precisely
so `exits` can quantify over it, but this slice's six steps are a **proposal**,
not a checked claim.

**One self-inflicted failure worth recording, because it is a frame-condition
trap in a new place.** The refactor that gave the transaction an attempt identity
also rewrote its "the transaction is gone" frame to clear the **lease verdict**.
`doPreflight` frames that verdict explicitly and `TxnStateWellFormed` requires it
of any non-`Fresh` phase, so the preflight's entire refusal branch became
unsatisfiable — and **eight of `entry-k39`'s fourteen witnesses reported *no
instance* in one run** while every check stayed green. Checks going green while
witnesses vanish is the signature the runner exists to catch, and it caught it.
The rule generalises: **a shared frame predicate that a later slice widens is a
change to every transition that ever used it**, and the witnesses, not the
checks, are what report it.

**Cost.** Eight reachable transitions. The same six unchanged entry-surface
commands run 0.9 s each on `entry-k39`'s file and 1.4 s on this one — **~+55%**,
an A/B on one host in one sitting. The suite goes 23 commands / 23 s → 40
commands / 2 m 13 s, most of which is the seventeen new commands at 1.6–2.7 s
each. That places eight reachable transitions above the model's prior worst case
(four transitions, +41%) on the inherited commands, which is what the cost model
predicts, and it says the *marginal* cost of a transition is not linear in the
count.

**A third measurement rule, and it is new.** `task-tree-k7` established that
whole-suite totals do not compare across sessions. Add: **a single command's cost
is bimodal within one sitting.**
`witness_FN_11_the_interval_between_publication_and_commit` measured 2.0 s,
10.1 s and 2.0 s on three consecutive runs of the same bytes — a 5× swing with
nothing changed. SAT4J's search is not a stopwatch, so a slice that reports one
sentinel's before-and-after is reporting noise; every figure above is a median of
three.

**Counterfactual.** Without the model, the volatile-initial-state defect is
invisible — it is a property of the *model*, so no amount of reading the shipped
Rust would surface it. The `FN-13` reason hole is different: it is legible from
the catalogue alone by anyone who reads `FN-13`'s witness beside the closed
reason list, and the honest claim is that the model **forced** the reading rather
than that it alone could produce it. The same is true of entry 031's counting
mismatch, and two data points now say the same thing about what this instrument
is doing: it is a discipline that makes you write down the outcome of every
branch, and the catalogue's gaps are where that discipline has nothing to write.

**Verdict.** The slice is green — 40 commands, 8 obligations, 16 of 61 alloy
cells filled, one mutation per obligation and **all eight KILLED as first
written**, each with a named existing witness re-run under it and still landing.
No protocol defect. Two catalogue-level findings, both from writing witnesses.
Three model-level counterexamples, all from one over-broad licence, all retained
in the family `README.md`. The corpus gains a third vacuity predictor and a third
measurement rule.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`. **Solver**: SAT4J
(distribution default), every command with `-n` and `-t text`. **Bounds**: the
common shape is
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, N steps`,
with `N` from 3 to 10; each check at or above the widest first-landing bound
among its own obligation's witnesses, tabulated in
`crates/grove-finish/models/README.md`, and the `EN-02` control unchanged at
`1 Device, 4 steps`. No `Int` in the file. **Fairness**: none assumed; nothing in
either slice is a liveness claim. **Symmetry**: no `exactly` scope.
**Abstractions**: the coarse tree, with the two reserved names modelled as one
slot carrying a **class**; the manifest's ready mark read as *both* the ready
mark and the written-and-verified record; the attempt identity and repository
anchor as opaque pins nothing reads back; the digest as the catalogue's opaque
equality; `Sys.why` widened from a precondition observable to a condition
observable; the body's step order as a phase machine rather than a refusal per
out-of-order step, which is a scoping of the totality rule to what an
*invocation* returns; and the initial state narrowed in its **volatile** half
only. **Deliberately omitted**: everything from the commit onward — the commit
itself, the correlation ticket, the three dispositions, rollback, forward
recovery, the quarantine, the atomic root rename, the cleanup marker and the
reaper. `doCommitAttempt` records that a commit was *attempted* and mutates
nothing. **What a green run does not prove**: everything is about the stated
bounds — at most three entries, two devices, two attempt identities, two digests
and ten states, one transaction, no lane distinction — and, beyond the bounds,
the six caveats in the family `README.md`, of which two are new: that the
manifest is never revalidated at the digest step because no reachable antecedent
exists for one, and that `evacuationComplete`'s *the task root is still present*
is discharged upstream by the preflight rather than by the commit attempt's own
gate.

### 033 — A commit whose cost was in the trace length, and a third outcome the closed set cannot name (finish commit and disposition)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-03`, `FN-04`, `FN-14` – `FN-18`: the
repository as something that changes — the scoped deletion commit, the
correlation ticket, the three dispositions classified from evidence, the rollback
licence and its exactness, and forward recovery. Twelve obligations, thirty-one
new commands, and the `EN-09` exercise-removal. The scope's empty alloy cells
fall from forty-five to **thirty-three**.

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**Situation.** `witness-k40` left the transaction's body six steps long and
stopped at the point a commit is *attempted*: `doCommitAttempt` recorded that one
had been tried and mutated nothing. This slice opens that seam. `EN-05` — *no
filesystem transaction can include a version-control commit* — is what shapes all
of it: the commit sits outside the body, so the interval between the evacuation
and the recorded result cannot be closed, and every obligation here is about what
can be *known* across that interval and what may be done on the strength of it.

**The material finding is that the cost of a reachable transition is superlinear
in the trace length it is reachable at, and a slice that measures only its
cheapest commands will be wrong about its own file by a factor of four.**
`task-tree-k7` established that one sentinel is not enough because the tightest
command is nearly blind to new *state*. This is the sharper form of the same
lesson, and it is about *transitions*. Four new reachable transitions —
`Recover`, `Classify`, `Settle`, `ResultArrives` — cost the inherited
entry-surface commands **+30%** (1.3 s → 1.7 s, 1.4 s → 1.8 s, medians of three
on one host in one sitting), which is in proportion to the witness slice's eight
costing them +55%. The same four cost the file's widest inherited command,
`FN-13` at ten states, **+128%** (2.5 s → 5.7 s) — more than twice what
proportionality predicts. A transition enabled only in the last two phases of a
ten-state trace is nearly free for a four-state command and dear for a ten-state
one.

> `M1 alloy-only` · `M2 cost` · `M3 = n/a` — this is a measurement rather than a
> counterexample · `M4 = none` — nothing shipped is wrong; the corpus's cost
> model was.

The corollary is a budgeting rule the two remaining siblings need, and both are
long-trace slices: **budget by counting transitions × the bound they are
reachable at, not by counting transitions.**

**A fourth entry in the bound register, and the cheapest of them to check for.**
Twenty-three inherited witness bounds were re-measured under the commit slice by
sweeping each command from 2 to 14 states and taking the first that lands.
Twenty-two are unchanged. One moved:
`witness_FN_11_the_interval_between_publication_and_commit`, 9 → **10**, and
`FN-11`'s check with it. The cause is not state space. `doCommitAttempt` used to
frame everything, so the state it produced differed from its predecessor in
`Sys.act` alone and the trace closed its lasso on the spot; it now advances the
phase, so the trace needs one more state to stutter into. **A step that stops
being a no-op costs a state to every witness that ended on it.** It cost one
`FAIL  … no instance` in the first full run, which is the runner reporting
exactly what it exists to report.

**The second material finding is a third outcome the catalogue's closed refusal
set cannot name — and it is the third of one shape in one scope.** The catalogue
maps the `NotCommitted` disposition to *rolls back and yields `Refused`*, and
none of the seventeen closed reasons names a rolled-back finish: `NoTrackedDeletion`
and `RootIdentityChanged` are each false of a transaction whose fingerprint was
fine and whose root never moved. The model adds **one** refusal atom of its own,
`RefRollbackNotCommitted`, and records it as an addition rather than smuggling it
in. Beside entry 031's seven-preconditions-against-six-reasons and entry 032's
tracked witness, that is three, and the diagnosis is now specific: **the
catalogue fixes closed sets and never states the map between them**, and every
place a model is forced to write down what a branch returns is a place the map is
missing.

> **[disposed by `closed-set-additions-k74`]** The catalogue's reason set gained **`DeletionNotCommitted`**, and both columns now spell the member rather than their own atom. **This entry's diagnosis was right and was the disposition's starting point**: the catalogue fixes closed sets and never states the map between them. What the disposition adds is which half of the map was missing where. The *reason* vocabulary was answering the right question all along — a reason names the question asked, not the gate that asked it, which is why `Sys.why` is a modelling need rather than a contract gap. The *set* was drawn over the **task-tree** scope's questions and is swept by three, so every gap so far is a question a later scope asks: a commit's disposition (this one), a configuration, a launch generation. Of the five referred members, three were granted, one dissolved (`FN-13`'s stop is a block, and a block carries a diagnosis), and one was no gap at all (Grove has no confirmation gate). Record: `docs/adr/a-refusal-leaves-nothing-standing.md`.

> `M1 alloy-only` · `M2 refusal` · `M3 = 2` — the absence of a fitting reason is
> read off the closed set, not out of a trace · `M4 = none`. Like 031 and 032 this
> falsifies **H7** by H7's own terms, and for the third time the finder was a
> *branch that had to return something*, not a check going red.

**`Indeterminate` is reachable, on every lane, which is Q2's evidence and the
answer this leaf owed.** `FN-15.d` offers two branches — a witness, or a
positively-established bounded unreachability — and this file takes the first,
so no `defer` is recorded. What makes it reachable is exactly `EN-05`: with the
commit outside the transaction, the world may move the repository between the
attempt and the classification, and a moved anchor with no ticket for this
attempt is a state in which neither outcome can be proven. Whether the
counterfactual — commit and evacuation as one step — makes it *unreachable* is
`relax_EN_05`, which is **Quint's** row in the assumption table and is not
evidence this family can produce.

**Caught / missed.** Caught: the two mutation failures below; the missing refusal
reason; the bound move; and the anchor's lane-blindness, which is written up as a
limit rather than a defect — twelve obligations landed and exactly *one* of them,
`FN-17.a`, reads the lane at all. Missed, in the sense of not attempted:
whether `interruptedMidEvacuation` — the posited disk fifteen of the eighteen new
witnesses start from — is *reachable*. It is written to be exactly what the six
body steps plus a `crash` produce and is not checked to be; the check would be
`FN-22`'s revalidation table and is `handoff`'s. Also missed: native jj's
**partial-commit hazard**, where the deletion stays in the change and the
unselected witness moves into a *successor*, so success is the exact ticket-named
*parent*. That needs a notion of successor revisions this file does not have, and
it is the sharpest thing `formal-synthesis-k16` should not read this green run as
covering.

**Two of twelve mutations did not land as first written, after eight that did —
and both are rules this file already carried, met from a new direction.**

- **A frame stated in two places must be removed from both.** `FN-14`'s mutation
  first dropped the unrelated-work frame from `commitLands` alone; the check
  stayed green because `doCommitAttempt`'s common part frames the world too. It
  is the same class as the witness slice's mutation 2 approached from the
  opposite side — there a conjunct that changed nothing was *added*, here a
  conjunct that constrained nothing was *removed* — and the general form is one
  sentence: **the frame you must remove is every frame, and a redundantly-stated
  one hides the other copy.**
- **Writing the claim apart from the transition protects the claim from a
  mutation aimed at the transition.** `FN-16.b`'s mutation first reordered the
  classification so the anchor was consulted before the result. The check stayed
  green, because the settle branch reads `rollbackLicensed` — written apart, as
  house style requires — which still carried *and the result is absent*. That is
  the discipline working exactly as designed, and it is precisely why the *aim*
  of a mutation has to be checked as carefully as its satisfiability. The row as
  run mutates the licence and lets the settle branch on it.

**Counterfactual.** The superlinear cost result needs the model and a stopwatch
and nothing else — it is not a fact about grove at all, and no amount of reading
the shipped Rust would produce it. The rolled-back-finish reason hole is legible
from the catalogue alone by anyone who reads the disposition-to-outcome mapping
beside the closed reason list; the honest claim, as with 031 and 032, is that the
model **forced** the reading rather than that it alone could produce it. The
reachability of `Indeterminate` is the one result here that is genuinely
model-shaped: it is a statement about what the incumbent protocol admits at
stated bounds, and the only cheaper way to reach it is to argue about it.

**Verdict.** The slice is green — **71 commands, 12 obligations, 28 of 61 alloy
cells filled**, one mutation per obligation and **all twelve KILLED**, each with a
named existing witness re-run under it and still landing. No protocol defect and
no counterexample that was a defect in the catalogue: for the third consecutive
slice in this scope, every finding came from writing down what a branch returns.
One catalogue-level finding, one cost law, one bound-register entry and two
mutation-discipline lessons. `FN-15.d` is answered by witnesses rather than by a
`defer`.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`. **Solver**: SAT4J
(distribution default), every command with `-n` and `-t text`. **Bounds**: the
common shape is unchanged —
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, N steps` —
with `N` from 2 to 10 and **the ceiling unmoved**; the commit slice adds no scope
dimension, since `Disposition`, `Report` and `Reproducible` are `one sig`s and
the correlation ticket is a relation over atoms that already existed. Each check
runs at or above the widest first-landing bound among its own obligation's
witnesses, all forty-one of them measured by sweep and tabulated in
`crates/grove-finish/models/README.md`. The `EN-02` control is unchanged at
`1 Device, 4 steps`; `EN-09`'s runs at `9 steps`. No `Int` in the file.
**Fairness**: none assumed; nothing here is a liveness claim. **Symmetry**: no
`exactly` scope. **Abstractions**: the anchor as a single lane-blind revision;
the commit's landing as a free branch of the attempt step and the reported result
as a free `lone` value; the forward settle's release of the witness and manifest
as a stand-in for disposal; `FN-17.a`'s *before the witness is removed* as a
conjunction rather than an ordering; `Blocked` as an outcome with **no
diagnosis**, because the partition is `FN-25`'s; and — the load-bearing one —
fifteen of eighteen witnesses starting from `interruptedMidEvacuation`, a posited
disk, because reaching a settled disposition from a fresh grove is ten
transitions and a retry that has lost its artifacts is twelve.
`witness_FN_15b_git_committed_reached_from_a_fresh_grove` pays that cost once, at
ten states, so the other fourteen are shortcuts of something rather than of
nothing. **Deliberately omitted**: the quarantine, the atomic root rename, the
four revalidation points, disposal's re-entrancy, the cleanup marker and its
`replace` transition, the reaper, the `Blocked` diagnoses, the index image, and
the lanes table's commit mechanisms including native jj's partial-commit hazard.
**What a green run does not prove**: everything is about the stated bounds, and
beyond them the eleven caveats in the family `README.md`, of which five are new —
the lane still being a property of the signature rather than a measured fact; the
unchecked reachability of the posited disk; `resultProven`'s fingerprint conjunct
going vacuous once the manifest is released, which is `FN-03`'s own content;
`Indeterminate`'s reachability saying nothing about the counterfactual; and the
commit's scoping being checked at the level of a tracked set rather than at the
level of a pathspec, a fileset or a successor revision.


### 034 — A transition that cost a sixth of what the cost law predicted, and an invariant the world can break (finish quarantine and the atomic root rename)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-19` and `FN-20`: the quarantine, the atomic
rename of the whole task root into it, and *a leftover artifact is garbage, never
a receipt*. Two obligations, four new commands, one new reachable transition. The
scope's empty alloy cells fall from thirty-three to **thirty-one**.

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**Situation.** `commit-k41` ran the transaction to a settled disposition and
released the witness and the manifest in place, recording that release as an
*abstraction of disposal*. This slice replaces the first half of it with the real
thing: a `Committed` classification now renames the whole task root — witness and
evacuated tree intact — into the quarantine in one step, and the settle that
follows disposes what the rename produced. It is the first slice in the scope
that **removes the task root**, and three things were armed against exactly that
day.

**The material finding is that the cost of a reachable transition is governed by
how many places it is ENABLED, not by the depth it is reachable at — and the
previous slice's law, applied literally, was a factor of six too pessimistic.**
`commit-k41` measured four transitions at **+128%** on the file's widest command
and concluded that marginal cost is superlinear in trace length; it budgeted this
slice as the worst case that law describes, since a quarantine rename is
reachable only at the far end of a ten-state trace. Measured, medians of three,
one host, one sitting, clean A/B with no bound moving on any sentinel: `FN_13`
(ten states, the widest inherited) **5.73 s → 6.01 s, +5%**; the two
entry-surface sentinels +4% each; `witness_FN_11` +7%. Per transition the
previous law predicted about +30% on the widest.

What separates the two measurements is the **enabling surface**. `commit-k41`'s
four included a `Classify` re-runnable at three phases and a world-driven
`ResultArrives` enabled at three more; `QuarRename` is enabled at exactly one
phase *and* one disposition and nothing else in the file reaches it. The
operative form of the law from here: **budget by the number of (phase, guard)
points a transition is enabled at, multiplied by the bound they are reachable
at.** That is the same advice the corpus already gave — *prefer a narrowed
antecedent* — arriving from the other side with a number on it, and it is the
first time in this experiment that a cost prediction has been wrong in the
**cheap** direction. A slice that trusted the arithmetic would have bought a
static scope switch it did not need.

**Formalism.** Alloy 6, temporal, one file, four new commands
(`FN_19`, `witness_FN_19`, `FN_20`, `witness_FN_20`), seventy-five in total.

**Caught / missed.** Caught: one retained counterexample (below); the cost
result; three inherited witness bounds moving; and a mutation that killed a
neighbour. Caught *by not going red*, which is the more interesting one: the
`evacuationComplete` / `gateEvacuated` divergence — written apart since
`witness-k40` against the day a step removed the root — **did not become a
counterexample**, because the rename is two transitions past the commit attempt
and `doTxnOpen` refuses an absent root, so the protocol's own ordering makes the
divergence unreachable. That is weaker than *the gate enforces it* and the two
sides were left unedited; the first thing that will re-enter a transaction over a
rootless disk is `FN-22`'s revalidation. Missed, in the sense of not attempted:
whether `interruptedMidEvacuation` is reachable — still `FN-22`'s. Also
deliberately not reached: `doClassify` was **not** opened to the phase between
the rename and the settle, because that is precisely `FN-22`'s revalidation
*after the quarantine rename* and writing it here would answer two of that
table's ten rows by construction.

**A retained counterexample, and it is the witness slice's first met from the
other side.** `FN-19`'s *no partial or empty task root is ever observable*,
restated as the invariant *one root identity is never in two places at once* and
written over **every** step, fails on a `Swap`: the world swapping the task root
is constrained only by *the identity changes*, so the solver picks the
quarantine's own identity, and the model cannot know that moving the quarantine
directory back over `.grove/` took the quarantine with it. The witness slice's
retained counterexample 1 is the same lesson under a free **initial state** — *a
shape claim under `EN-11` must be restated over the transition relation*. This is
`EN-11` as a **transition**, since `doSwap` and `doTopologyChange` are the hand
edit made first-class, and the restatement it forces is one clause: the invariant
is preserved by the transaction's own steps and is not a property of the disk.
**A claim about what a protocol never does is never a claim about what the world
never does**, and this file now carries that at both grains.

**A third way for a mutation to fail its aim, and it reports like the other
two.** `entry-k39` found that a mutation the model cannot **execute** is not a
control; `commit-k41` found that a mutation aimed at the wrong half of a claim is
not one either. This slice adds: **a mutation that kills its target and a
neighbour has not isolated what the target uniquely says.** `FN-20`'s obvious
mutation — the classification refusing to reach `Committed` while a leftover
*quarantine* exists — was satisfiable, killed `FN-20`, and killed `FN-03` with
it, because `FN-03`'s third conjunct says nothing about a quarantine. The row as
run mutates on the witness's presence instead, which is the antecedent `FN-03`
already carries and `FN-20`'s deliberately drops, and it leaves `FN-03`, `FN-04`
and `FN-15.b` green. The general form: **check what a mutation leaves standing,
not only what it takes down.**

**A fifth entry in the bound register, and the cheapest yet to predict.** Three
inherited witnesses moved 9 → 10 — `witness_FN_03`, `witness_FN_16b`,
`witness_FN_18` — and their three checks with them. All three are the file's only
witnesses that run through the **forward settle**, and the slice put a step in
front of it. The register already carried *a step that stops being a no-op costs
a state to every witness that ended on it*; this is the other half — **a step
inserted into a path costs a state to every witness that passes through it**.
Forty-three witnesses were swept from 2 to 14 states to establish that exactly
three did.

**A `Q4` row that could not be decided, and the reason is itself evidence.** The
quarantine's own removal-matrix row is not written. Row 30 breaks `FN-19`, which
is *incumbent mechanics* and is exactly what `TODO.finish_process.md` Q1 asks
about, so it is not an answer; row 31 breaks `FN-20`, which **is** shared safety
and which the catalogue states over the **role** — *no artifact a transaction
leaves behind is a receipt for it* — so a protocol leaving nothing behind
satisfies it vacuously. The one shared-safety claim this slice reaches therefore
says **nothing against disposal-in-place**. The obligations that could name the
quarantine, `FN-24` and `FN-27`, are not in the file yet.

**Cost.** Seventy-five commands, **3 m 05 s** wall for the whole file, against
seventy-one in 2 m 40 s for `commit-k41`'s — and the four new commands are most
of the difference. One new reachable transition, one `one sig` with a single
`var lone` field, one new phase atom, no new scope dimension. Roughly one
session, of which the bound sweep was about two and a half minutes at concurrency
five and the mutation work rather more, because one mutation had to be re-aimed
after it took a neighbour down with it.

**Counterfactual.** The enabling-surface refinement to the cost law needs the
model, a stopwatch and a *prior wrong prediction*; it is not a fact about grove
at all and no reading of the shipped Rust produces it. The `doSwap` counterexample
is genuinely model-shaped: it is a statement about what the model's own world
transitions license, and prose review would have accepted *the root is never in
two places* without asking who else can move a directory. The divergence staying
unreachable is the one result here a careful reader of the step order could have
argued to — the model's contribution is that it is now **checked at ten states**
rather than argued.

**Verdict.** The slice is green — **75 commands, 2 obligations, 30 of 61 alloy
cells filled**, one mutation per obligation and **both KILLED**, each with a named
existing witness re-run under it and still landing. No protocol defect and no
counterexample that was a defect in the catalogue: for the fourth consecutive
slice in this scope, every finding came from writing down what a branch returns
or from measuring what a slice cost. One cost law corrected, one retained
counterexample, one bound-register entry, one mutation-discipline lesson, and one
Q4 row deliberately left for the sibling that will have the claims to decide it.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`. **Solver**: SAT4J
(distribution default), every command with `-n` and `-t text`. **Bounds**: the
common shape is unchanged —
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, N steps` —
with `N` from 2 to 10 and **the ceiling unmoved**, since the rename sits one
transition past a classification that was already the ninth. The slice adds no
scope dimension: `Quar` is a `one sig` and `2 RootId` was already bought by
`FN-06`'s swap, which is what `FN-20`'s witness spends it on — at one atom, *a
leftover quarantine beside a task root that is not the one inside it* is
inexpressible rather than false. Each check runs at or above the widest
first-landing bound among its own obligation's witnesses, all forty-three
measured by sweep and tabulated in `crates/grove-finish/models/README.md`;
`FN_20` is the first command in the file to sit deliberately **above** that
floor, at 8 against a witness landing at 7, because it is the slice's only
shared-safety claim. No `Int` in the file. **Fairness**: none assumed; nothing
here is a liveness claim. **Symmetry**: no `exactly` scope. **Abstractions**: the
quarantine as a second place a `RootId` can be rather than as a copy of what the
root holds, which is what makes *witness and evacuated tree intact* a frame
condition; the forward settle as **still** a stand-in for disposal, now clearing
a quarantine rather than releasing artifacts in place; the occupied quarantine
target as a `Blocked` with a model-only `why` rather than a named
`OwnershipConflict`, because the diagnosis partition is `FN-25`'s; and no
filename grammar, so *the quarantine target is occupied* is one condition here
where the shipped protocol has a per-handle, per-attempt family of them.
**Deliberately omitted**: disposal's re-entrancy, the cleanup marker and its
`replace` transition, the reaper, the four revalidation points and their ten-row
table, the `Blocked` diagnoses, and everything `commit-k41` already listed. **What
a green run does not prove**: everything is about the stated bounds, and beyond
them the thirteen caveats in the family `README.md`, of which two are new and one
is rewritten — the divergence being unreachable rather than enforced, which is
the rewrite; `FN-20`'s strongest form
being **non-interference**, which quantifies over pairs of traces and is
therefore inexpressible as an Alloy check at all; and the fact that this file
demonstrates, in `witness_FN_19`, a state it has no transition capable of leaving,
because the only thing that disposes of a quarantine over an absent task root is
the reaper.

---

### 035 — A check written stronger than its claim, and a table that had to become data (finish revalidation)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-22`: the four revalidation points, the
ten-row table, and the quarantine **return** as its own transition. Ten
obligations — the largest single claim group in the scope — twenty-six new
commands, four new reachable transitions and **one enabling point removed**. The
scope's empty alloy cells fall from thirty-one to **twenty-one**.

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**Situation.** `commit-k41` built the restoration and `quarantine-k43` built the
quarantine rename, and neither put any recheck around either one; both recorded
the absence deliberately, naming this slice as the reason. `FN-22` says the two
handoffs are the only two, that the disposition is rechecked immediately before
and after each, and that **every observed disposition at every point has a stated
corrective action and a stated stable state** — including the observations that
settle successfully.

**The material finding is that a check written stronger than the claim it answers
does not fail; it deletes states, and a sibling claim that needs those states is
then answered BY CONSTRUCTION two slices later.** `FN_03`'s first conjunct read
`always Repo.tickets in Repo.tickets'` — history never shrinks, under any step,
the world's included. The catalogue's claim is narrower: the ticket *SHALL
survive the destruction of every artifact the transaction owns*. The comment
beside the check had said *under Grove's own steps* since `commit-k41`; the check
said more than its own comment.

With history append-only under every step, `resultProven` is **monotone**: a
`Committed` observation can never become anything else. The catalogue's table has
two rows that are exactly that transition — `Committed -> NotCommitted`, a
rollback that succeeds and ends `Refused`, and `Committed -> Indeterminate`, a
block — and says in as many words that collapsing them *would let a block be
reported as a refusal, which is exactly the distinction `FN-29` requires the
operator to be able to make*. A slice that had not read the check against the
catalogue's wording would have written both obligations, watched their witnesses
fail to land, widened the bound, given up, and reported them green.

This is a **new failure mode**, not a new rule. The three catalogue-level
findings this scope already carries (entries 031, 032, 033) are all one shape —
*the catalogue fixes closed sets and never states the map between them* — and all
three were found by writing down what a branch returns. This one is found by
reading a **sibling's check** against the catalogue, and it is invisible to every
command in the file that contains it: an over-statement has no counterexample of
its own. The narrowing is the same one `FN-19`'s third conjunct took after the
`doSwap` counterexample, so the file now carries *a claim about what a protocol
never does is never a claim about what the world never does* at three grains — a
free initial state, a world transition over the tree, and a world transition over
history.

**The second finding is about how to state a table at all.** A table is the shape
a mutation cannot falsify row by row: a per-row mutation kills a row, and nothing
kills *the table is complete*. `FN-22.a` is written as a **total function over
four points and three dispositions** (`tableAction`, `tableOutcome`), held apart
from every transition, with a check binding every Grove step taken at a point to
it. Delete a row and the function goes partial, `Sys.act' = tableAction[..]` is
false, and the check is red — **a missing row is a counterexample rather than a
silence**, which is what the leaf's brief asked for. What the construction cannot
catch is a combination reachable in the world that enables no Grove step at all;
that remains a silence, and sixteen witnesses are what fill it.

Writing the totality down has an immediate consequence for mutation discipline,
and it is a **fourth rule about aim** in this corpus. `FN-22.a` binds the action
and the outcome at every point, so it is strictly stronger than the
action-and-outcome half of every other row — and **any mutation that changes
which arm runs, or what it returns, kills `FN-22.a` too.** An isolating mutation
for the other nine rows must therefore aim at the **stable-state column**, the
one the totality claim does not carry. One row was re-aimed for exactly this
reason after killing its neighbour.

**Formalism.** Alloy 6, temporal, one file, twenty-six new commands (ten checks
and sixteen witnesses), **101 in total**. Four new transitions: `Revalidate`, the
recheck after the restoration; `QuarReturn`, the rename's inverse; and two the
world's — `CommitMoves`, this attempt's own commit landing late or being undone,
and `RootNameTaken`, the task-root name occupied while the quarantine holds the
root. One new phase, `Restored`.

**Caught / missed.** Caught: the `FN-03` over-statement; five retained
counterexamples, two of which changed a **transition** rather than a check, which
no earlier counterexample in this file had done; three mutations that failed
their aim, one in each of the three ways this file had already recorded — the
first time all three have appeared in one slice; thirteen inherited witness
bounds moving, twelve of them for a reason that is a sixth bound-register entry;
and the file's first ceiling movement since `witness-k40`. Caught *by not going
red*: the `evacuationComplete` / `gateEvacuated` divergence was predicted by
`quarantine-k43` to fire here and did not, because `doCommitAttempt` is enabled
at two phases and both are upstream of any handoff — the honest reading has
hardened rather than changed across three slices.

**The debt `commit-k41` took on is paid, and the answer is yes.** Fifteen of that
slice's eighteen witnesses — and every witness in the file resting on
`interruptedMidEvacuation` — start from a **posited disk** written to be exactly
what the six body steps plus a `crash` produce, and never checked to be one. The
family `README.md` recorded it as a limit and named this table as the check. The
honest check turned out not to be the table at all but a **witness**: run the
body up to the disk. It runs `TxnOpen`, the preflight, all six body steps with a
partial evacuation, a `crash` and the confirmation a later launch supplies, and
**first lands at eleven states, finding nothing at ten**. Ten was the ceiling from
`witness-k40` onward, so the thing that had prevented the check for three slices
was a **bound**, not an argument — and paying the debt is what moved the ceiling.
This is the only entry ever removed from that file's *what a green run does not
prove* list.

> The methodological point is worth separating from the result. A limit recorded
> as *we did not check X* had a stated reason (*the check would be `FN-22`'s
> table*) that was **wrong about its own instrument**, and the wrongness was
> cheap to discover and expensive to leave: had the predicate been
> over-permissive, fifteen witnesses would have been testifying about a state no
> execution reaches and **no check would have said so**, because a check
> quantifies over all traces and does not care which one a witness picked. Read
> an inherited limit for whether its stated remedy is actually the remedy.

**A cost law wrong in the cheap direction for the second consecutive slice.**
Medians of three, one host, one sitting, clean A/B with no bound moving on any
sentinel: `FN_13` (ten states, the widest inherited) **6.07 s → 6.93 s, +14%**;
the two entry-surface sentinels +9% and +11%; `witness_FN_11` +15%.
`quarantine-k43`'s law — budget by (phase, guard) points times the bound they are
reachable at — predicts about **+53%** for this slice's net five points against
`commit-k41`'s twelve at +128%. The variable being over-counted is **dwell**: all
five of this slice's points sit at phases a trace passes through once, whereas
`commit-k41`'s `Classify` and `ResultArrives` were enabled at phases a trace can
rest in, contributing a successor at many depths. The third statement of the law:
**budget by the number of states of a trace at which a transition is enabled** —
a phase a trace passes through once is one state, however many guards select it.
Two slices have now found the arithmetic pessimistic, by six and then by four,
while the ordering it recommends (a static scope switch, then a narrowed
antecedent, then a smaller bound) has been right every time. **Take the ordering;
do not take the multiplier.**

**A sixth entry in the bound register, found by a third question.** Twelve
inherited witnesses went up by exactly one, and every one of them ends on a
`Classify`. Nothing about them changed and nothing about the classification's
effect changed; it stopped being **enabled** at `Classified`, so a trace that
ended on a classification can no longer close its lasso by running the same
classification again. The register's three shapes are now: a step that stops
being a no-op costs a state to every witness that **ended** on it; a step
inserted into a path costs a state to every witness that **passes through** it;
and a step that stops being enabled costs a state to every witness that **closed
its lasso** on it. They are worth keeping apart because each is found by a
different question. Fifty-nine witnesses were swept from 2 to 14 states to
establish which thirteen moved.

**Two counterexamples that changed a transition, which is a first for this file.**
`FN-22.j` fails on a `Classify` taken at a pending handoff: the classification
was re-runnable at `Classified`, so a trace could observe `Indeterminate` with
the rename pending, re-derive the disposition, report `Applied` and take no
corrective action — **a fifth revalidation point the catalogue did not
authorise**, and one that is invisible until something states how many points
there are. The fix is the transition, not the check. And `FN-17.a`'s new ordering
conjunct fails on a **lane change between the restoration and the release**: the
split this slice was forced to make opened a window that the shipped one-step
settle does not have, and closing it needed `SY-03`'s own rule — every gate
revalidates against its own operands. That second one is a design finding for
`formal-synthesis-k16`: it is the hazard the implementation inherits if
`FN-24.b` ever forces the same decomposition.

**An abstraction removed rather than added.** `FN-17.a`'s *before the witness is
removed* was a conjunction in `commit-k41`, recorded as an abstraction, with the
decomposition question deferred to `FN-24.b`. `FN-22`'s *after restoration* row
cannot be stated without a state after the restoration — a one-step settle can
only observe what it observed before its own effect, since the restore branch
frames the whole repository — so the split was forced by the claim rather than
chosen, and the ordering is now real.

**Cost.** 101 commands, **5 m 46 s** wall for the whole file, against 75 in
3 m 05 s. Roughly one session: the bound sweep of fifty-nine witnesses at
concurrency six was the largest single block, and the mutation work was larger
still, because ten mutations were each run against the other nine `FN-22` checks
and six inherited neighbours rather than against their target alone.

**Counterfactual.** The `FN-03` over-statement is not reachable by reading the
shipped Rust — there is no Rust for this yet — and it is not reachable by reading
the catalogue either: it is a statement about what a **model's own check** made
unreachable, and it took a second claim needing those states to expose it. The
`interruptedMidEvacuation` result is the sharpest thing here that a prose review
could not have produced at all: *is this posited state one an execution reaches*
is a reachability question over a nine-transition trace, and the answer arrived
as a bound. The revalidation table's totality is the one result a careful reader
could have argued to — every row is in the catalogue — and the model's
contribution is that a missing row is now **red at eleven states** rather than
unnoticed.

**Verdict.** The slice is green — **101 commands, 10 obligations, 40 of 61 alloy
cells filled**, one mutation per obligation and **all ten KILLED**, each with a
named existing witness re-run under it and still landing, and each swept against
fifteen neighbouring checks to show it isolates. No protocol defect and no
counterexample that was a defect in the catalogue: for the fifth consecutive
slice in this scope, every finding came from writing down what a branch returns,
from reading a check against its own claim, or from measuring what a slice cost.
One new failure mode, one cost law corrected again, five retained
counterexamples, one bound-register entry, one mutation-discipline rule, one
abstraction removed, and one three-slice-old debt paid.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`. **Solver**: SAT4J
(distribution default), every command with `-n` and `-t text`. **Bounds**: the
common shape is unchanged —
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, N steps` —
with `N` now from 2 to **12**, the first ceiling movement since `witness-k40`.
The slice adds no scope dimension: `Restored` is one more `Phase` atom and the
four `RevPoint`s are static `one sig`s used only by the claims. Each check runs
at or above the widest first-landing bound among its own obligation's witnesses,
all fifty-nine measured by sweep and tabulated in
`crates/grove-finish/models/README.md`; `FN_20`'s deliberate margin above that
floor has been **absorbed** by a bound movement unrelated to the claim, which is
recorded there as a caution about stating margins as numbers. No `Int` in the
file. **Fairness**: none assumed; nothing here is a liveness claim. **Symmetry**:
no `exactly` scope. **Abstractions**: the revalidation points as **states** rather
than as a `var` field, which costs no state space and makes the two before-points
one moment distinguished by which handoff was pending; `observed` written apart
from `doClassify` and bound to it only through the evidence predicates;
`doCommitMoves` narrowed to the three phases a point can observe it at; the
failed return's *report both* as an observable **state** rather than as
diagnostic text; the four `Blocked` rows carrying model-only `why` values rather
than the catalogue's `RecoveryPending`, because the partition is `FN-25`'s and
`exits`' — `exits` now inherits three such values, not one; and the forward
settle still passing through `FN-22.i`'s stable state and out of it in one step,
because disposal is `FN-21`'s. **Deliberately omitted**: disposal's re-entrancy,
the cleanup marker and its `replace` transition, the reaper, the `Blocked`
diagnoses, and everything the earlier slices already listed. **What a green run
does not prove**: everything is about the stated bounds, and beyond them the
caveats in the family `README.md` — one **removed** (`interruptedMidEvacuation`,
answered) and three new: that `observed` and `doClassify` agree is argued rather
than checked; that the four points are the right four rests on the catalogue's
count of handoffs, so a handoff by a third mechanism would fall outside all four
antecedents; and that a `Blocked` this table produces is `RecoveryPending` is the
catalogue's word, not this file's.

---

### 036 — A sweep priced against four transitions, and Q3 answered by a state the protocol produces (finish disposal)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-21` and `FN-31`: disposal's re-entrancy, the
cleanup marker's create / replace / remove transitions, the replacement's
atomicity with respect to readers, and the reaper. Seven obligations, seventeen
new commands, four new reachable Grove transitions, one sweep outside the phase
machine, two new phases and one new scope dimension. The scope's empty alloy
cells fall from twenty-one to **fourteen**, and with it `handoff-k42`'s subtree
is complete.

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**Situation.** Every slice before this one disposed the quarantine in the same
transition that revalidated it: `FN-22.i`'s stable state — *task root `Absent`,
quarantine holding the root* — was passed through and out of in one move, and
three READMEs recorded that as an abstraction of disposal. `FN-21.a` says
disposal is *re-enterable from any interruption*, and `EN-03` says there is no
atomic recursive directory deletion, so a disposal that is one transition has no
interruption point to be re-enterable **from** and the claim is unstateable. The
slice's whole shape follows from that: write a cleanup marker, remove what it
authorises removing, retire the marker — and a **sweep** to resume it, because
`doTxnOpen` requires `some Root.rid` and the disk an interruption after the
rename leaves has none.

**The material result is a price for a transition shape this corpus had never
measured: a sweep enabled at a phase a trace can REST in.** `revalidation-k44`
corrected the cost law to *budget by the number of states of a trace at which a
transition is enabled* and predicted the reaper would be the first expensive
thing in the scope, naming this slice as the first chance to see the law wrong in
the **dear** direction. Three variants of one file, one sitting, medians of three,
on `FN_13` (ten states, the widest inherited command):

| variant | median |
|---|---|
| the disposal slice with **no `Reap` at all** | 7.54 s (+7% on the inherited file) |
| as it ships — `Reap` guarded on *something at a reserved name* | **8.34 s** (+11% on top) |
| the same with `Reap`'s antecedent widened to every `Fresh` state | 8.68 s (+15% on top) |

**One sweep at a dwell phase costs more than four transitions at pass-through
phases put together.** The four disposal steps, the two phases and the `2 CMark`
scope dimension are +7% between them; the sweep alone is +11%. This is the first
of three consecutive slices in which the law's arithmetic did **not** need
correcting — the direction and the ordering were both right — and it is the first
time the corpus can put a number on *prefer a narrowed antecedent*: the guard
`some Cleanup.present or some Quar.qRid` against an unguarded `Txn.phase = Fresh`
buys about four points of the sweep's fifteen, at no cost in reachable behaviour,
because a sweep over nothing is a no-op. The second-order reading matters as
much: +11% is a fifth of what `commit-k41`'s four transitions cost on the same
sentinel, so *expensive* here means *worth narrowing*, not *worth avoiding*.

**`TODO.finish_process.md` Q3 is answered, and by a state the protocol produces
rather than one the model permits.** Q3 asks whether the marker-replacement
sub-transaction — 960 lines of shipped Rust nested inside the cleanup of a
crash-safe protocol — is reachable at all, and asks for the states requiring a
*replace* to be enumerated. The answer is **yes, by witness, first landing at ten
states**, and the enumeration is one class:

> a cleanup marker left standing by a disposal that completed the removal it
> authorised and was interrupted before retiring it — Grove's own, its target
> gone. A sweep will collect it; a **new** attempt reaching the after-rename
> point first must supersede it.

**The answer falls out of `FN-21.a`'s machinery, which is what makes it evidence
about the protocol rather than about the file.** The marker is retired last
because a document recording that a removal has not happened cannot go before the
removal; a protocol that retired it earlier would have no stale markers, no
replacement — and no re-enterability. And the source state is **reached, not
posited**: a second witness runs the protocol from the disk an interruption
mid-evacuation leaves, through the rename, the marker and the removal, and
crashes before the marker is retired, at **twelve states**. Without it the answer
would have rested on a hand-edit `EN-11` permits, which is the debt `commit-k41`
took on and `revalidation-k44` paid; this slice did not open a second one. The
two commands are separable and both are needed: a mutation that narrows the
replacement to foreign markers kills the first and leaves the second standing.

> **What `formal-synthesis-k16` should not read into it.** The Alloy family says
> the transition is reachable under the **incumbent** protocol at these bounds,
> so *delete the replacement* is unavailable on this evidence — it does not say
> the sub-transaction earns its 960 lines. The catalogue omits the marker's byte
> layout; `EN-01` **grants** the atomicity the replacement rests on rather than
> the model establishing it; and Q1's counterfactual — disposal in place under
> `relax_EN_03` — is Quint's, and would remove the quarantine, the marker and the
> replacement together. Q3 is answered *within* the incumbent; Q1 is what could
> make the question moot.

**A third bound-vacuity predictor, and it is produced by this corpus's own rule.**
The file's witness-bound rule is *a check runs at least as high as the widest
first-landing bound among its obligation's witnesses*, floor 4. `FN-31.c`'s two
witnesses land at **3** and **4**, because both posit the disk an interruption
leaves and run the sweep over it. Run at 4, the check's first conjunct has **no
reachable antecedent at all**: `MarkerReplace` first occurs at ten states, so the
check would have been green and empty and its mutation would have reported
exactly as a survivor. The two predictors `task-tree-k7` left cover the case in
principle — *the bound must hold the machinery of the transitions the obligation
quantifies over* — but what is new is **where the wrong number comes from**. It is
not carelessness; it is the file's own rule, applied to an obligation whose
witnesses are cheap posited disks and whose antecedent is a deep transition. The
two had coincided in every earlier slice because every witness ran the protocol
up to the thing it witnessed.

> **The witness-bound rule is a floor, and it is below the real floor whenever an
> obligation's witnesses posit a disk its antecedent has to be run up to.** Read
> the check's antecedent for the deepest transition it names, and take the larger
> of the two numbers.

**A seventh entry in the bound register, and it sharpens the second rather than
adding a shape.** Splitting the forward settle put two transitions into a stretch
five inherited witnesses run through, and the register says a step inserted into a
path costs a state to every witness that **passes through** it. **Two moved**:
`witness_FN_03` 10 → 12 and `witness_FN_18` 10 → 11. The three that did not each
end **on** the inserted step rather than past it, and their final assertion was
re-anchored from `Settle` to `MarkerCreate` — the same position in the trace. So
*passes through* is the operative word and a witness ending at the insertion point
does not: **ask whether a witness needs to reach past the insertion or only to
it.** Sixty-nine witnesses were swept from 2 to 14 states to establish that
exactly two did. The ceiling stayed at twelve.

**Formalism.** Alloy 6, temporal, one file, seventeen new commands (seven checks
and ten witnesses), **118 in total**. Four new Grove transitions —
`MarkerCreate`, `MarkerReplace`, `Dispose`, `MarkerRemove` — plus `Reap`, which is
a **sweep** and is deliberately in neither `bodySteps` nor `txnActs`: it takes no
operator confirmation, `FN-24.b` should not be asked of it, and `FN-22.a`'s *none
is skipped* conjuncts correctly do not reach it, because a sweep never had a
disposition to revalidate. Two new phases (`Disposing`, `Disposed`) and one new
scope dimension.

**The scope dimension is the slice's one false-confidence save, and it is worth
its own paragraph.** `FN-31.b` is *no reader observes the marker absent, **nor
observes two markers***. Modelled the obvious way — `one sig Mark { var there:
lone Marker }` — the second prohibition is **inexpressible** and half the claim is
true by construction. So a marker is an **atom** and what is `var` is which
markers stand at the reserved name; `#Cleanup.present = 2` is a state the model
can be in, a remove-then-create replacement is a trace it can take, and the
mutation that adds a second marker kills the check. That is the fifth time in this
scope a claim would have been answered by an encoding, and the first time the fix
was a **scope dimension** rather than a rewritten conjunct.

**Caught / missed.** Caught: two retained counterexamples, **both a check written
wider than the claim it answers**, which is now this file's most common failure
mode and its third and fifth grain respectively; a third bound-vacuity predictor;
a seventh bound-register entry; three mutations that failed their aim, two of them
the *unsatisfiable branch* trap met in one slice against a **fact** and against a
**contradiction with the conjunct being replaced**; and Q3's answer. Missed by
construction, and recorded: disposal's content removal is one step where `EN-03`
says the shipped one is entry-by-entry, and `inTreeWitnessOwns` errs towards
declining because the file has one `Slot` and cannot tell a witness inside the
quarantine from one at the reserved name in the tree.

**A fourth rule about mutation aim, for overlapping subjects rather than for a
table.** Two mutations killed `FN-22.i` alongside their targets, because that
check had been written to assert the marker's own content — exactly one marker
afterwards, naming this quarantine. True, and `FN-31`'s. The fix was to the
check, not to the mutations: **when two obligations describe the same artifact
from two directions, the one whose subject it is not should not describe it at
all.** `revalidation-k44`'s rule was the same lesson for a table's totality; this
is it for a shared subject, and both were found by sweeping what a mutation leaves
**green**.

**A fifth mutation shape: a control whose kill is a witness that stops landing.**
`FN-31.a` is answered by a witness, so its mutation has to be able to make the
trace disappear rather than make a check go red. Narrowing `doMarkerReplace` to
foreign markers leaves **every check in the file green** — the protocol simply
stops at `Quarantined` with an owned marker standing, a liveness hole no safety
check notices — and the witness stops landing. For a reachability-answered
obligation, a green suite under the mutation is the expected result rather than a
survivor, and the runner's missing-instance `FAIL` is the control.

**Cost.** 118 commands, **7 m 39 s** wall for the whole file, against 101 in
5 m 46 s. Roughly one session, of which the bound sweep of sixty-nine witnesses
from 2 to 14 states at concurrency five was by a distance the largest block —
larger than the mutation work, which is a reversal of the previous slice and is a
consequence of sweeping from 2 rather than from each witness's inherited bound.

**Counterfactual.** Q3 is the one result here that no amount of reading could have
produced: *enumerate the states that require replacing rather than creating a
marker* is a reachability question over a twelve-transition trace, and the answer
arrived as a witness plus a bound. The cost isolation is the second — three
variants of one file measured in one sitting is not something a review produces,
and the number it yields (a sweep at a dwell phase ≈ four pass-through
transitions) is the kind of thing the next slice budgets against. The two
counterexamples a careful reader could in principle have argued to; what a reader
could not have done is notice that the file's **own** witness-bound rule would
have made `FN-31.c` vacuous, because that requires knowing at what bound
`MarkerReplace` first occurs.

**Verdict.** The slice is green — **118 commands, 7 obligations, 47 of 61 alloy
cells filled**, one mutation per obligation and **all seven KILLED**, each with a
named witness re-run under it and still landing, and each swept against eleven
neighbouring checks to show it isolates. No protocol defect and no counterexample
that was a defect in the catalogue: for the sixth consecutive slice in this scope,
every finding came from writing down what a branch returns, from reading a check
against its own claim, or from measuring what a slice cost. One question from
`TODO.finish_process.md` answered, one cost law confirmed rather than corrected,
one bound-vacuity predictor, one bound-register entry, two mutation-discipline
rules, two retained counterexamples, and one abstraction removed.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`. **Solver**: SAT4J
(distribution default), every command with `-n` and `-t text`. **Bounds**: the
common shape gains one dimension —
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark,
N steps` — with `N` still from 2 to **12**; the ceiling did not move. `2 CMark` is
`FN-31.b`'s and is described above. Each check runs at or above **both** the
widest first-landing bound among its own obligation's witnesses **and** the bound
at which the deepest transition its antecedent names first occurs, all sixty-nine
witnesses measured by sweep and tabulated in
`crates/grove-finish/models/README.md`; `FN-21.b` and `FN-21.c` carry a stated
margin of three over their floor. No `Int` in the file. **Fairness**: none
assumed; nothing here is a liveness claim, and the one liveness hole the slice
met — a mutation that strands disposal at `Quarantined` — is recorded as
something no check notices rather than modelled. **Symmetry**: no `exactly` scope.
**Abstractions**: the cleanup marker as atoms-plus-presence rather than a `lone`
field; the marker's reserved **name** replaced by `cTarget`, which is what the
sweep reads a name for; `inTreeWitnessOwns` as *a task root is present to hold a
witness*, which errs towards declining; disposal's content removal as one step
where `EN-03` says the shipped one is entry-by-entry; disposal's terminal state
stated over the two names disposal owns rather than over the tree; `Reap` outside
`bodySteps` and `txnActs`; and `OwnershipConflict` as a model-only `Sys.why`
member serving both the sweep's decline and the replacement's, because `FN-25`'s
partition is `exits`' — `exits` now inherits **four** such values, and this is the
only one the catalogue itself names. **Deliberately omitted**: the marker's byte
layout, the reaper's lease, and everything the earlier slices already listed.
**What a green run does not prove**: everything is about the stated bounds, and
beyond them the caveats in the family `README.md` — one **removed** (the
quarantine this file leaves behind can now be cleaned up, and by a witness rather
than a property, which is twice in two slices) and five new, of which the two
sharpest are that the sweep's concurrency safety is unmodelled because the shipped
reaper is lease-owned and nothing here has a lease, and that disposal's removal is
not re-enterable *within* a step.

### 037 — A classification nobody had written down, and a slice whose cost was flat (finish crash boundaries)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-24`: *every interruption lands in exactly one
stable state*, and *every step of the transaction has at most one persistent
effect*. Two obligations, twenty-nine new commands, **no new transition, no new
`var` field and no new scope dimension** — the first slice in this scope that
adds none of the three. The scope's empty alloy cells fall from fourteen to
**twelve**.

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**The leaf decomposed before it modelled anything, and that is part of the
result.** `exits-k46` was cut as one session for fourteen obligations. Four of
them need machinery no sibling slice built — a stable-state classification of the
disk, a persistent-effect enumeration over sixteen steps, the `Blocked` diagnosis
partition four slices deliberately declined to build, and hook suppression —
where every earlier slice of `finish-k8` added one or two. It became a node with
three children (`crash`, `blocked`, `exits`), cut along **machinery** rather than
along the catalogue's section order, which is how the two levels above it were
cut. This entry is the first child's.

**Situation.** `crash` has been a first-class action since `witness-k40` and is
enabled at every step boundary; six slices have acted on the disk it leaves.
What none of them asked is what that disk **is**. `FN-24.a` is *the next
invocation classifies the result into exactly one stable state, and never into a
state indistinguishable from a different one* — a claim about a classification
the file did not contain. Writing it down, as data, apart from every transition,
is the whole of the slice, and it produced two findings about the catalogue in
the first hour.

**The material result is that §*States*' own classification order contradicts the
load-bearing property stated three paragraphs below it.** The table orders
`Absent` first and the three `Reserved` classes after it; the property beneath it
says *no transient state may be observable as a different stable state*,
instancing *a task root whose deletion is not yet proven is never `Absent`*. The
two are in tension exactly once a reserved name can be occupied while the
task-root name is **free** — a disk the finish protocol creates in one rename and
the task-tree scope never produces. With `Absent` first, the disk an interruption
immediately after the quarantine rename leaves reads `Absent`; and *not yet
proven* is reachable there rather than hypothetical, because the world can undo
the deletion commit between two of Grove's steps. The model orders the whole
`Reserved` class before `Absent`, and the mutation that restores the catalogue's
order is what makes the check red — so the control and the finding are the same
object.

**And the table has no row for a disk the protocol routinely produces.** A
disposal that has released its reserved witness while its quarantine still stands
is not `Reserved` — those rows are about the witness — and every `Current` row
would call it an ordinary grove. The model adds `Reserved(Quarantined)`, which
the catalogue licenses in as many words (*`TT-18`/`TT-19` are stated over the
reserved class rather than over its members*), and a second mutation shows it is
load-bearing: remove the arm and `FN-24.a` goes red on exactly that disk.

> **Both findings came from one cheap move this corpus had not made: write the
> classification down as data, apart from every transition, and ask a check
> whether it is total and unambiguous.** Six slices had acted on the disk without
> ever stating what the disk is. The move costs no state — it is `fun`s over
> static atoms — and it is available to any model whose subject has a documented
> state table.

**A cost shape the law had not met: flat rather than proportional.** Every slice
before this one cost most on the widest sentinel and least on the tightest —
disposal +13% / +11% on the entry surface against **+19%** on `FN_13`. This one
is the other way round: **+8% and +9% on the two tight sentinels, +1% on the
widest**. The absolute movement is nearly the same number of milliseconds in all
four (0.17 s, 0.20 s, 0.10 s, 0.14 s), which is what a constant looks like across
commands whose totals differ by a factor of four.

| command | disposal slice | crash slice | |
|---|---|---|---|
| `FN_08` (4 steps) | 2.03 s | 2.20 s | **+8%** |
| `FN_07` (4 steps) | 2.24 s | 2.44 s | **+9%** |
| `FN_13` (10 steps, widest inherited) | 8.16 s | 8.26 s | **+1%** |
| `witness_FN_11` (10 steps) | 3.83 s | 3.97 s | +4% |

> **The cost law is about STATE. Static atoms and static relations cost a roughly
> CONSTANT amount per command, not a percentage of it** — they enlarge `univ` and
> the per-command translation, both paid once whatever the bound. A percentage on
> a tight sentinel is a large fraction of a small number; do not extrapolate it.

That is a fourth statement of one law rather than a fifth law: *budget by the
number of states of a trace at which a transition is enabled* prices this slice
at zero, and zero was very nearly right. It also gives `task-tree-k7`'s *one
sentinel is not enough* its converse case — that rule was written because the
tight sentinel went **down** while the wide one rose; here the tight sentinel is
the only one that moved at all.

**The first slice whose inherited bounds could not move, and the argument beats
the sweep.** No transition, no `var` field, no `fact`: two abstract signatures of
`one sig` atoms cannot make a trace exist or stop existing, so no inherited
witness's first-landing bound can move. That is a proof rather than a
measurement. Eight inherited rows spread across the full depth range (2, 2, 7,
10, 11, 12, 12, 12) were re-measured anyway as the control the argument is worth
nothing without, and each still lands at its recorded bound and at no smaller
one. A sibling that adds a `var` field, a transition or a fact does not inherit
the argument and owes the full sweep — which, at sixty-nine witnesses, was the
largest single block of the previous slice's session.

**A fourth way for a mutation to fail its aim, and it is a property of the
claim's own data.** `FN-24.a`'s mutation was first written as *delete one pair
from the precedence relation*, so that the post-rename disk would match two
stable states with neither classified before the other. It **SURVIVED** — the
same disk also matches `Reserved(Quarantined)`, whose own pair into `Absent` the
mutation had not touched, so the resolution had an understudy.

> **A mutation to one row of a total order is not a mutation to the order.** A
> precedence relation is transitively redundant by construction, so any single
> edge a classification rests on has alternatives. Mutate the whole ranking, as
> an alternative the claim is stated against.

**`FN-09.a`'s *exactly one rename* does not frame the whole tree, and the mutation
for a whole-disk claim is what found it.** `FN-24.b`'s control is a publication
that also deletes a standing cleanup marker: two persistent effects in one step.
It kills `FN-24.b` and leaves `FN-09.a` **green**, because that check's frame —
written before the marker existed — asserts the root, the manifest, the
repository and the world and not the marker. Two readings were available and the
file takes the second: the marker is `FN-31`'s subject and a check whose subject
it is not should not describe it, which is `disposal-k45`'s fourth rule about aim
applied to a *check* rather than to a mutation. What follows is a caveat the
family README now carries: *nothing else moves* in this file means *nothing else
the claim's own frame names*, and `FN-24.b` is the only check that quantifies
over the whole disk at once.

**`EN-08` was run and it does not control everything the assumption table says it
does.** The table names `FN-09`, `FN-10`, `FN-24`, `FN-31.c`, `SY-12`, `TT-20`
and `TT-23.b` as the witnesses that become unreachable when `crash` is removed.
Three controls confirm `FN-09.a`, `FN-09.b`, `FN-10.a` and all sixteen of
`FN-24.a`'s do. **`FN-31.c`'s two do not** — both *posit* the disk an
interruption leaves rather than running `crash` to reach it, so they keep landing
with the action gone. That is a fact about the realisation rather than about the
assumption, and it is the sort of thing an exercise-removal exists to make
visible: a posited disk and a reached one are interchangeable for the *claim* and
not for the assumption's *control*.

**Formalism.** Alloy 6, temporal, one file, twenty-nine new commands (two checks,
twenty-four witnesses and three `EN-08` controls), **147 in total**. Two new
static abstract signatures — `Stable` (seven members) and `Effect` (nine) — and
`fun`s over them read only by claims. `FN-24.a`'s witness is sixteen commands,
one crash point per member of `bodySteps`, because a crash ends the transaction
and no single trace can visit all sixteen; each runs at its own step's depth
(6–13) rather than at the check's, which is what keeps the sweep affordable.

**The persistent-effect grain is the slice's one design decision worth naming.**
`FN-24.b`'s subject is *the effect*, not *the field*, and a field-by-field count
reports a correct protocol as a defective one three separate ways: a
same-directory rename touches two names and `EN-01` makes it one effect; removing
a directory removes what is inside it, so releasing the reserved witness is not
also four writes; and moving entries between two names is one move however many
entries move. Counted by field, the completed refusal reads as four persistent
effects and the atomic root rename as two. **Two steps have more than one
effect and both are declared, with what decomposing them would take** — which is
what the obligation asks for in as many words. They are named in one place, so
narrowing the check and declaring the abstraction are the same edit; a check
quietly weakened until it passes and a declaration are otherwise
indistinguishable in a green run.

**Caught / missed.** Caught: two catalogue findings (the classification order,
and a missing row); a fourth mutation-aim rule; a fourth statement of the cost
law with a new shape; one over-narrow inherited frame (`FN-09.a`); one sentence
of this file's own prose corrected by the enumeration (`doReap`'s
content-removal branch has two persistent effects, not one); and one limit of an
assumption's control (`EN-08` over `FN-31.c`). Missed by construction, and
recorded: `FN-24.a`'s totality conjunct is true by construction given the arm
set, so the claim rests on its two ordering conjuncts; and the sixteen step
boundaries are witnessed individually rather than by one execution.

**Cost.** 147 commands, **10 m 33 s** wall for the whole file, against 118 in
7 m 39 s — of which the sentinel A/B puts the slice's tax on the inherited
commands at 1–9%, so most of the extra 2 m 54 s is the twenty-nine new commands
themselves. `FN_24b` at twelve states is 51.7 s and is now the dearest single
command in the file; `FN_24a` at thirteen is 5.3 s, which is the flat-cost result
from the other side — a wide antecedent whose consequent is a function of static
data is cheap at any bound. The session's largest block was neither the
bound sweep nor the mutations but the **restatement** — the classification was
written twice, because the first version made `FN-24.a`'s third conjunct true by
construction by strengthening the `Absent` arm instead of moving the order, and a
mutation is what showed it. That is the cheapest kind of rework and it happened
because a mutation was run before the green was believed.

**Counterfactual.** The two catalogue findings are the result no amount of
reading produces: both require asking whether a *set of eleven rows with a stated
order* is total and unambiguous over the disks one particular protocol reaches,
which is a question about reachability and not about prose. A careful reader
could have noticed the `Absent` tension by inspection; nobody had, across six
slices and two scopes, and what surfaced it was a check with a mutation aimed at
it. The flat cost shape is the second — it needs an A/B on four sentinels in one
sitting, and its value is that the next slice adding only static structure can
budget it at approximately nothing.

**Verdict.** The slice is green — **147 commands, 2 obligations, 49 of 61 alloy
cells filled**, one mutation per obligation and **both KILLED**, each with a named
witness re-run under it and still landing, and `FN-24.b`'s swept against fourteen
neighbouring checks to show it isolates. No protocol defect: for the seventh
consecutive slice in this scope, every finding came from writing down what the
model already implied, from reading a check against its own claim, or from
measuring what a slice cost.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`. **Solver**: SAT4J
(distribution default), every command with `-n` and `-t text`. **Bounds**: the
common shape is unchanged —
`for 3 but 2 Device, 2 RootId, 2 Rev, 3 Entry, 2 AttemptId, 2 Digest, 2 CMark,
N steps` — and **the ceiling moved from 12 to 13**, by exactly one command:
`witness_FN_24a_a_crash_after_the_cleanup_marker_is_removed` needs disposal's
last step and then an interruption. `FN-24.a` runs at 13 (both bound rules
agree); `FN-24.b` at 12, where the witness rule gives 11 and the antecedent rule
gives 12 and the larger wins — `disposal-k45`'s third predictor in its ordinary
form, applied before the fact. No `Int` in the file. **Fairness**: none assumed;
neither obligation is a liveness claim. **Symmetry**: no `exactly` scope.
**Abstractions**: the stable-state classification as seven of the catalogue's
eleven rows plus one model-only member; each arm the catalogue's row verbatim so
the ORDER carries the claim; persistent effects counted at the effect's grain
rather than the field's; and the two declared multi-effect steps (`Dispose`, and
`doSettle`'s restore branch), each with what decomposing it would take.
**Deliberately omitted**: `Reserved(Migrating)`, `PartialScaffold`, `Legacy`,
`Foreign` and `Malformed` — five task-root states no finish transition produces.
**What a green run does not prove**: everything is about the stated bounds, and
beyond them the caveats in the family `README.md` — five new, of which the two
sharpest are that `FN-24.a`'s totality conjunct is true by construction and that
`EN-08`'s control does not reach `FN-31.c`'s witnesses.


---

### 038 — One condition stated under two closed names, and a green run over an empty state (finish blocked diagnoses)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-25` and `FN-26`: the closed partition of
`Blocked` over `RecoveryPending` and `OwnershipConflict`, its disjointness,
exhaustiveness and per-lane reachability, and *history is never rewritten to
clear a block*. Four obligations, seventeen new commands, and **`EN-16`** — the lane
collapsed to one — run as the scope's fourth assumption control. The scope's
empty alloy cells fall from twelve to **eight**.

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**Situation.** `FN-25` is the one claim in this scope that four consecutive
slices deliberately did not build. `commit-k41`, `quarantine-k43`,
`revalidation-k44` and `disposal-k45` each reached a condition the catalogue
diagnoses — an indeterminate commit, an occupied quarantine target, a late
`Committed` after the restoration, an incomplete return, a foreign cleanup
marker — and each gave it a model-only `Sys.why` rather than extending
`BlockedOutcome`, recording in as many words why: a slice that named the
partition in the SIGNATURE would answer totality, disjointness and
exhaustiveness by construction, which is the false-confidence shape rather than
a finding. **That accumulated abstinence is spendable exactly once, and this is
what it bought.**

**The material result is three catalogue findings, and all three are the same
document defect at a grain this corpus had not recorded: THE CATALOGUE STATES
ONE CONDITION UNDER TWO CLOSED NAMES.** The three findings before them in this
scope are all *closed sets with no map between them*. These are worse, because
each definition is individually complete and reads correctly:

- `OwnershipConflict`'s second clause — *the observed topology matches neither
  the recorded anchor nor the expected result* — is the classification's
  `Indeterminate` written out. §*Handoff and cleanup*'s ten-row table produces
  three `Blocked` rows for `Indeterminate` and diagnoses every one of them
  `RecoveryPending`, by name. Read literally the two definitions are not a
  partition at all: **every `RecoveryPending` state the protocol reaches
  satisfies the other name's definition.** `FN-25.a` is red at nine states.
- `RecoveryPending`'s third sentence — *and the outcome cannot yet be proven
  either way* — is false of two of that same table's rows, both of which it
  diagnoses `RecoveryPending`. Taken as a conjunct it makes `FN-25.b` false.
- `OwnershipConflict`'s first clause is a **general sentence with three printed
  examples**, and the general sentence is the one the model needs: a Grove-OWNED
  witness whose manifest names a different finish handle falls through both
  diagnoses when the clause is modelled as its example (*no owner*) and is
  covered when it is modelled as its sentence (*cannot be proved safe to
  mutate*).

The first two are the same sentence read from two sides. The edit they imply is
citation-sized and is written down in the family README; what is not
citation-sized, and is what `formal-synthesis-k16` inherits, is that **nothing in
the catalogue decides which diagnosis wins where both hold.** This file chooses
`OwnershipConflict`, on the fail-closed rule, and states the choice as a
two-element precedence relation so that deleting the edge leaves two survivors
rather than silently reversing.

**The methodological result is a sixth grain of the vacuity rule, and it is the
first that is not about a bound.** The first form of every `FN-25` command read
the diagnosis in the state a block LANDS in. `doSettle`, `doRevalidate` and
`doQuarReturn` all block through `txnGone`, so that state carries no attempt
identity, no handle and no anchor — and `resultProven`'s first conjunct then
reads `none in ticketedAttempts`, which Alloy makes **vacuously true**. Every arm
evaluated alike; `lone diagnosedRaw` and `one diagnosed` both held, for the wrong
reason, and the exhaustiveness check reported the only counterexample that
exposed it.

> **A check can be green because its subject is EMPTY rather than because its
> subject is well-behaved.** The five vacuity predictors this corpus carries are
> all about a bound too small to REACH a state. This one is about a state that is
> reached and is empty, and the tell is a relational subset over a `lone` field:
> `none in X` is true. An outcome is what an INVOCATION returns, and the
> invocation still holds its operands when it decides — so a claim about an
> outcome is read in the state the outcome is decided in, unprimed against a
> primed result.

Four antecedents were probed for reachability before any green was believed, and
**two of the four would otherwise have been vacuous**: `FN-25.c`'s lane-blindness
antecedent (a step that changes the lane and nothing else) and `FN-26`'s first
conjunct (a Grove step immediately after a block).

**`EN-16` was run and its named set is EXACT.** The assumption table names
`FN-15.b`, `FN-15.c`, `FN-15.d`, `FN-17` and `FN-25.c`, and `crash-k47` found
`EN-08`'s overstated its reach by one obligation, so the same question was asked
here. Eleven commands in the file pinned a lane before this slice and every one
of them answers one of the four; the six this slice adds answer `FN-25.c`; and
the file has exactly three lane-sensitive predicates —
`preflightCommitReproduced`, `canReproduceHere` and `reproductionStands` — all
three vacuous on `GitL`, which is why `FN-17.b`'s block branch is unreachable
under the collapse as well as `FN-17.a`'s reproduction. **Every `FN-` property
stays green under the collapse**, which is the whole content of the control: a
lane-blind model passes this file.

**A fifth way for a mutation to fail its aim, and it is specific to claims stated
ACROSS a state boundary.** `FN-26`'s first mutation freed `doRecover`'s
repository frame. `doRecover` guards on `Txn.phase = Entered` and a block leaves
`Fresh`, so the mutated transition is not reachable *after* the claim's
antecedent holds within the bound, and the check reported green exactly as a
survivor would. Aimed instead at `doQuarRename`'s own blocking branch — inside
the antecedent rather than after it — it kills. **Two of the four mutations also
kill a neighbour**, and in both cases the neighbour is a conjunct written ABOUT
the clause the mutation edits, which is `disposal-k45`'s fourth rule met for the
first time where the shared subject is a predicate rather than a disk object.

**Formalism.** Alloy 6, temporal, one file, seventeen new commands (four checks, ten
witnesses and three `EN-16` controls), **164 in total**. Two new static
abstract signatures — `Diagnosis` (two members) and `BlockField` (four) — six
predicates, three functions and a two-element precedence relation, all of them
read only by claims. No transition, no `var` field, no `fact` and no scope
dimension: the second consecutive slice that adds none of the four, and the
ceiling stays at thirteen.

**Caught / missed.** Caught: three catalogue findings, all three the same defect
shape and new to this corpus; a sixth vacuity grain that is not about a bound; a
fifth way for a mutation to miss its aim; the confirmation that `EN-16`'s named
set is exact where `EN-08`'s was not; and one property that read as obviously
true and is false — *a diagnosis is only ever produced under a selected lane*,
which `SY-03`'s `var` lane refutes at nine states. Missed by construction, and
recorded: at the reserved WITNESS name the two arms are complements, so `FN-25.a`
and `FN-25.b` have no content there; `FN-26`'s `BObserved` arm is a fact of the
signature; and `OwnershipConflict`'s second clause never fires alone within the
bound, which is recorded as a checked bounded-unreachability rather than as prose.

**Cost.** 164 commands, **12 m 08 s** wall for the whole file, against 147 in
10 m 33 s — of which the sentinel A/B puts the slice's tax on the inherited
commands at **1–6%**, so essentially all of the extra 1 m 35 s is the seventeen
new commands. **The flat-cost law now has a coefficient.** The crash slice added
sixteen static atoms and moved the four sentinels by 0.17 / 0.20 / 0.10 / 0.14 s;
this one added **six** and moved them by 0.12 / 0.08 / 0.10 / 0.07 s — the same
absolute shape at roughly a third to a half the size, on commands whose totals
differ by a factor of four. Budget static structure at about **10 ms of
translation per atom per command** and stop reading the percentage. The four new
checks are 7.2 s, 7.6 s, 5.2 s and 15.4 s at thirteen states; `FN_24b`'s 51.8 s
is still the file's dearest command. `FN_26` costs twice its siblings because its
third conjunct is nested (`always (... implies always (...))`), which is the one
thing in the slice that is not a function of static data. The session's largest
block was neither the bound sweep — argued, with a six-witness control — nor the
mutations, but **finding out where the diagnosis had to be evaluated**: three
successive forms of the arms were written and probed before the fourth, and each
rewrite was cheap only because a counterexample rather than a green was what
prompted it.

**Counterfactual.** The three catalogue findings are what no amount of reading
produces, and the reason is specific: each requires holding two closed
definitions and a ten-row table six hundred lines apart against the set of states
one protocol actually reaches. Four slices read those definitions and none
noticed; each of them recorded, correctly, that naming the partition was not its
job. **The abstinence is what made the finding available, and spending it once is
what collected it** — which is the strongest evidence this experiment has
produced for deferring a claim rather than answering it early and cheaply.

**Verdict.** The slice is green — **164 commands, 4 obligations, 53 of 61
alloy cells filled**, one mutation per obligation and all four KILLED, each with
a named witness re-run under it and still landing. Three catalogue findings, no
protocol defect, and the eighth consecutive slice in this scope whose findings
came from writing down what the model already implied rather than from a solver
disagreeing with it.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`. **Solver**: SAT4J
(distribution default), every command with `-n` and `-t text`. **Bounds**: the
common shape is unchanged, and **the ceiling does not move** — all four checks at
13, all ten witnesses at 9, and the three `EN-16` controls at 8–10. No `Int`.
**Fairness**: none assumed; nothing here is a liveness claim. **Symmetry**: no
`exactly` scope. **Abstractions**: the diagnosis read in the deciding state
rather than the landing state; `OwnershipConflict`'s first clause widened from
its printed example to its general sentence, with the complementarity that costs;
`BObserved` true in every state of the file; and the precedence itself, which is
a design decision rather than a reading. **Deliberately omitted**: nothing new.
**What a green run does not prove**: six new entries in the family README, of
which the two sharpest are that the partition has no content at the reserved
witness name and that the precedence is this file's proposal rather than the
catalogue's.


### 039 — An operand the world can forge, and a claim with no isolating mutation (finish exits)

**Scope.** Finish / recovery, **component-local**
(`crates/grove-finish/models/`). `FN-02`, `FN-23`, `FN-27.a` – `FN-27.c`,
`FN-28`, `FN-29` and `FN-30`: intent persisting as the finish leaf, recovery's
idempotence, *nothing unrelated is mutated on any outcome*, the single successful
exit, a refusal as a complete outcome, and hook suppression. Eight obligations,
sixteen new commands, `TODO.finish_process.md` **Q4's ten-row removal matrix**,
and the runner's matrix-reader question. **The scope's empty alloy cells fall
from eight to zero and the column closes.**

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened.

**Situation.** The last slice of a column, and the only one in the scope whose
brief charged it with a deliverable that is not a claim: Q4's matrix, which four
earlier slices had each declined to write on the grounds that the shared-safety
obligations that could decide its two hardest rows did not exist yet. `FN-27` and
`FN-28` are those obligations.

**THE MATERIAL RESULT IS ABOUT THE SHIPPED PROTOCOL AND IT IS THE FIRST IN THIS
SCOPE THAT IS.** `FN-28` says *a finish succeeds exactly when the exact
attempt-bound commit is proven and the task root is absent*. The second operand
reads as a fact about the disk and **is not one the protocol can hold**: after
the quarantine rename the task-root name is free, one world transition occupies
it (`doRootNameTaken`, which exists for `FN-22.h`) and a second gives what it put
there the **quarantined root's own identity** (`doSwap`, constrained only by
`Root.rid' != Root.rid`). Three successive formulations of the claim's conjuncts
— *after the completing step both operands hold*, *the completing step is reached
only over both*, and *no disposal step takes either back* — were each falsified by
that one trace and by nothing else.

> **A finish cannot report success by looking at the task-root name.** The only
> durable evidence is the correlation ticket. `FN-03` already says the ticket
> must survive the *destruction* of every artifact the transaction owns; this
> adds that it must survive the *re-creation* of one, because a name is not an
> artifact and the world owns the namespace. A `grove finish` that decided
> success by stat-ing the task root would report failure on a grove someone had
> simply started using again, and success on one where the quarantine had been
> moved back over it.

The model states the operands as things Grove **establishes and preserves**,
never as things that hold. This is the corpus's *`EN-11` as a transition* rule
met a fourth time and the first time its consequence lands on the implementation
rather than on the model.

**THE METHODOLOGICAL RESULT IS A SIXTH WAY FOR A MUTATION TO FAIL ITS AIM, AND
IT IS THE FIRST THAT IS A PROPERTY OF THE CHECK SET.** `FN-29`'s row took three
forms and none isolates:

- aimed at *every block leaves an artifact standing*, it kills `FN-22.a` first,
  whose own third conjunct is *the witness is only ever released, on the rollback
  path, at the after-restoration point*;
- aimed at *the completed refusal leaves nothing of the transaction*, it is
  **unsatisfiable** against `fact BodyPhaseMatchesDisk` (`Settled implies no
  Slot.occ`) and reports exactly as a survivor — caught only because the
  fire-evidence witness stopped landing;
- aimed at the task root's own name, it fires and kills four neighbours.

> **A claim every one of whose conjuncts is another claim's subject has no
> isolating mutation.** It is still checked and its mutation is still a control;
> what it is not is a control *for that claim alone*, and the honest record is
> the neighbour list rather than a fourth attempt. The five earlier ways to miss
> an aim are properties of the model, of the frame, of the fact set, of the
> claim's own data, or of reachability. This one is a property of the SUITE.

**A SEVENTH ENTRY IN THE BOUND REGISTER, AND IT SHARPENS THE `var`-FIELD RULE
RATHER THAN OBEYING IT.** The crash slice left *a sibling that adds a `var`
field, a transition or a fact owes the full witness sweep*. This slice adds one
`var` field and does not owe it, and what replaces the sweep is a
**monotonicity proof**: no guard, no `fact` and no existing command reads
`World.hookRan`; twenty-eight of twenty-nine transitions frame it and the
twenty-ninth leaves it free; so every pre-existing instance extends to an
instance of the new file by setting the field absent throughout, and no
first-landing bound can rise. Eight inherited witnesses spanning 2 to 12 states
were re-measured as the control and every one still lands where it did.

> **A `var` field that adds only free choice owes a proof and a control; a
> `var` field any guard, fact or existing command reads owes the full sweep.**

**AND THE COST MEASUREMENT CONTRADICTS THE SAME ARGUMENT'S OPTIMISTIC READING.**
Monotonicity is about which instances exist, not about what they cost. The
four-sentinel A/B gives −6.8%, +17.5%, +3.7% and −3.3%, and the file's **dearest**
command moved least in both seconds and per cent while an 8-second command moved
+1.4 s. **A `var` field is not static structure even when nothing reads it**: it
adds a boolean per state, so its cost tracks trace length and the size of the
transition encoding rather than the difficulty of a command's own claim, and the
last two slices' uniform 10 ms-per-atom law does not apply to it.

**Q4 IS ANSWERED AND TWO ROWS READ `none`.** The catalogue names the ten
removable artifacts; the matrix is in the family README with an evidence class
per row. Seven rows cite a numbered mutation, one is `abstracted`, two rest on
argument. **The quarantine's row reads `none`** — `FN-20` survives its removal
vacuously, `FN-24` was settled by the crash slice, `FN-27` names three fields of
the workspace and nothing on the task root's side, `FN-28`'s naming of
`QuarRename` is the incumbent realisation of a role the class register explicitly
discounts, and `TT-24`'s proof is the manifest and the reserved-name document
rather than the quarantine. **That is Q4's delete/replace criterion met for the
first time in either family** and it is evidence *for* Q1's candidate rather than
a decision of it: Q1 additionally requires disposal-in-place to be checked under
`relax_EN_03` with `FN-24`'s witnesses at no greater a bound, which nothing here
runs. **The replace transition's row reads `none` too**, transcribed from row 45:
narrowing it away leaves every check in the file green and stops one witness
landing — a liveness hole no safety claim sees.

> **[annotated by `honest-classification-k80`]** *`FN-24` was settled by the
> crash slice* is true of that slice and is not a clearance for a candidate
> protocol. `FN-24.a`'s two **failable** conjuncts were guarded on the
> incumbent's own artifacts — `no Slot.occ and no Quar.qRid` here,
> `groveReservationStands` in the Quint column, reached independently — so a
> candidate holding neither satisfies them vacuously. Measured under the
> available in-place candidate the Quint column now runs
> (`wit_FN_24a_the_artifact_guarded_encoding_accepts_it`), and the rule is
> [`a-shared-safety-claim-names-the-role-not-the-artifact`](adr/a-shared-safety-claim-names-the-role-not-the-artifact.md).
> **Nothing in this entry's counts moves**: the annotation adds no command, and
> Alloy's own repair is `alloy-candidate-k82`'s, so this row's `none` stands as
> the measurement this slice made.

**Two rows of the matrix are the kind of thing a matrix is for.** The evacuation
manifest's row was predicted by argument to break `FN-17.a` and the mutation
breaks `FN-02` instead: `FN-17.a`'s first conjunct is guarded on entries actually
returning and its second on the release, so a restoration that returns nothing
slips beneath both, and *intent persists as the finish leaf* is the first
shared-safety claim that notices. And the cleanup marker's row cites **`TT-24`**,
an obligation **no command in this directory may answer** under the runner's
placement rule — a third consequence of the `TT-24.c` / `TT-24.d` placement
question, which `formal-synthesis-k16` now inherits three times over.

> **[disposed by `routing-and-prose-k73`]** The `TT-24.c`/`TT-24.d` placement question was settled by `obligation-placement-k63` (`obligations-follow-context-not-artifact`); the class-register disagreement this paragraph goes on to record is routed to `finish-verdicts-k65`, because its consequence is a row of Q4's matrix.

**A twelfth finding, and it is about this file's own record.** `FN-13` is
*shared safety* in the catalogue's class register and this README's commit-slice
note has called it *incumbent mechanics* for three slices. It changes the
reserved witness's matrix row from a note into an answer. Recorded rather than
silently corrected.

**The runner question is decided: yes, narrowly, and it is a leaf.** The
catalogue names the ten artifacts in one sentence in one place, so the row set is
derivable from the catalogue exactly as the obligation manifest is — a reader can
assert, in both directions, that every named artifact has a row, that every row
names a named artifact, and that every cited obligation exists. It cannot decide
*first broken*; what reaches that is the **citation discipline** instituted with
the matrix, where each row cites a numbered mutation with its own fire-evidence
and neighbour sweep, an argument, or an abstraction. No `review-prototype` was
cut, and the leaf is `matrix-reader-k50`, placed ahead of the Quint column
because that column will owe a matrix of its own.

**Formalism.** Alloy 6, temporal, one file, sixteen new commands (eight checks
and eight witnesses), **180 in total**. One `one sig` (`Hook`), one static field
and **one `var` field** (`World.hookInstalled`, `World.hookRan`) — the first
`var` field added to this file since `commit-k41` — four functions, five
predicates and one posited disk. No transition, no `fact` and no scope dimension;
the ceiling stays at **thirteen**.

**Caught / missed.** Caught: an operand of a shared-safety claim that the world
can forge, with a design consequence; a sixth way for a mutation to miss its aim;
four retained counterexamples, three of them the free-initial-state rule met at
new granularities; a commit whose handle is empty landing a revision and no
ticket; the class-register discrepancy on `FN-13`; and the fact that `FN-17.a`
does not catch a restoration that returns nothing. Missed by construction, and
recorded: `FN-28`'s *branch, bookmark and worktree topology* is checked as *Grove
moves recorded topology only at an internal commit*, so an integration performed
BY a commit is invisible; `FN-30` models whether a hook ran and never what it
would do, so the implication between `FN-30` and `FN-27` is unchecked; and
`FN-23` does not cover a recovery whose terminal state depends on when it was
interrupted, which is `FN-21.a`'s.

**Cost.** 180 commands, **14 m 33 s** for the whole file, against 164 in
12 m 08 s (`blocked-k48`) — the sixteen new commands account for essentially all
of the +2 m. `FN_02` is the slice's dearest command at **45 s** and its
counterexample-carrying first form took **3 s**: a check that fails early is
cheap and a check that must exhaust the space is not, so a slice that budgeted
from its first red run would have budgeted an order of magnitude low. The
session's largest block was neither the bound sweep — argued, with an
eight-witness control — nor the matrix, but **three claims that each had to be
restated after a counterexample**, and all three restatements were the same rule.

**Counterfactual.** The tenth finding is not available by reading. It requires
holding `FN-28`'s two-operand definition against the set of states two *world*
transitions can produce, one of which (`doRootNameTaken`) exists for an entirely
different obligation and the other of which (`doSwap`) is constrained only by an
inequality. No amount of care in reading the catalogue produces *the world can
put the quarantined root's own identity back at the task-root name*; a solver
produces it in three seconds, three times running, against three different
formulations. **This is the clearest single instance in this experiment of the
model finding something about the implementation rather than about itself.**

**Verdict.** The slice is green — **180 commands, 8 obligations, 61 of 61 alloy
cells filled, coverage asserted, `--no-coverage` gone from the run line**, one
mutation per obligation and all eight KILLED, six of the eight isolating cleanly
on a twenty-six-check sweep. One protocol finding, two catalogue/record findings,
four retained counterexamples, and the alloy column of the finish scope closed.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6,
`org.alloytools.alloy.dist.jar`, Corretto `21.0.12.1+9-LTS`. **Solver**: SAT4J
(distribution default), every command with `-n` and `-t text`. **Bounds**: the
common shape is unchanged and the ceiling does not move — all eight checks at
**13**; the eight witnesses first land at 11, 4, 9, 10, 9, 9, 10 and 8, measured
by sweep from 2 to 14. No `Int`. **Fairness**: none assumed; nothing here is a
liveness claim. **Symmetry**: no `exactly` scope. **Abstractions**: *the task
root is absent* read as `Txn.pinned not in Root.rid`; a user-supplied hook as one
static atom with one `var` boolean for *has run*, with what a hook would mutate
deliberately unmodelled; the internal commits as exactly `CommitAttempt` and
`Settle`. **Deliberately omitted**: the index image, which has no counterpart in
this file and is Q4's one `abstracted` row. **What a green run does not prove**:
four new entries in the family README, of which the two sharpest are that
`FN-28`'s topology conjunct cannot see an integration performed by a commit, and
that `FN-29`'s second conjunct has no isolating mutation.


### 040 — A fact that stated the claim it was protecting, and three greens over nothing (lifecycle admission)

**Scope.** System lifecycle, **system-level** (`models/system/`) — the first
entry in this experiment whose model is not component-local. `SY-01.a`,
`SY-01.b`, `SY-02`, `SY-03`, `SY-11.a` and `SY-11.b`: one live driver per
working tree, the layout proved before any tree exists, a preflight that is
never a licence, and the guard order. Six obligations, fourteen commands, a new
`models/system/lifecycle.als`, and the two Alloy assumption mutations this scope
owns (`EN-07`, `EN-14`). **The lifecycle scope's empty alloy cells fall from
twenty-five to nineteen.**

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened. The sibling scopes' Quint columns were not read.

**Formalism.** Alloy 6.2. Not a choice — the node brief fixes both families, and
this is the Alloy column.

**Situation.** The first file of a scope whose whole discipline is *compose at
observations, never at machinery*: a lifecycle model that reads the task-tree and
finish contracts through the smallest observation that decides a claim, and
carries none of their machinery. Six obligations is the smallest slice in this
experiment, and it was chosen small because the file did not exist and three
foundational decisions had to land with it — how coarse the composition boundary
is, whether the guard stack is one ordered relation or three acquirable ones, and
whether a guard *wait* is observable at all.

**THE MATERIAL RESULT IS THAT A CLAIM RESTING ON A PLATFORM PROPERTY HAS NO
PROTOCOL-LEVEL MUTATION, AND THE ASSUMPTION TABLE HAS NO ROW FOR IT.** `SY-01.b`
says *ownership is released by process death as ordinarily as by return*. In any
model that represents an advisory lock honestly, that is a construction fact —
the kernel releases the lock when the holder ceases to exist — and **two separate
mutations against it are made unsatisfiable by the model's own facts**, which
reports exactly as a survivor does. The half that is not construction (a return
releases too; a successor proceeds) is checked and witnessed; the release half
cannot be.

> **`EN-01` – `EN-16` contain no row for kernel release of advisory locks on
> process death, and `SY-01.b` rests entirely on it.** Every other premise a
> claim's correctness rests on has a row and a mutation with a stated expected
> result — that is what the table is *for*, and what makes *agreement mistaken
> for proof* controllable. This one is granted silently, in both families, by
> whoever writes the crash transition. Adding the row is not this leaf's call:
> it imposes a mutation on the Quint column and changes what `formal-synthesis-k16`
> reads off the coverage matrix, so it is recorded here and named for that leaf,
> exactly as `SY-05`'s design constraint was at entry 039.

> **[disposed by `routing-and-prose-k73`]** Routed to the model owners rather than disposed as a catalogue finding: an `EN-` row for process death is an assumption-table entry with a mutation in both columns, and no catalogue text is contradicted while it is absent. It stays owed and is now owed by whoever next revises the assumption table.

**M1** `alloy-only`. **M2** `interruption`. **M3** — no counterexample; the
finding arrives as *two mutations that will not fire*, which is score **0** by
the ordinal's own terms and is the second time in this experiment that a
zero-score signal was the informative one. **M4** `none` — there is no defect to
write a failing test against; the correction is a catalogue row.

**THE METHODOLOGICAL RESULT IS A THIRD SHAPE OF FALSE CONFIDENCE, AND IT IS THE
FACT SET RATHER THAN THE BOUND OR THE FRAME.** Three greens over nothing in one
session, each caught by a different instrument:

1. **A `lone` field under `not in` is false when the field is empty.**
   `p.waits not in p.holds` reads as *nobody waits for what they already hold*
   and is **false** exactly when `p.waits` is empty, `none in X` being true for
   every `X`. The fact therefore said *every process is blocked in every state*,
   and the file had **no instances at all** — `some Proc` was unsatisfiable.
   Every check reported *no counterexample*. Caught by the **witnesses**, all six
   of which returned nothing in the same run; a check-only suite would have
   reported a perfect green over an empty universe. **M8**, stood one round.
2. **A construction fact that states a claim makes the claim vacuous.**
   `SY-11.a` — *every path takes lease, then generation, then tree, in that
   order* — was green while the free-initial-state fact asserted `TreeG in p.seen
   implies EpochG in p.seen`, which **is** that order. Three mutations against
   three different acquisition sites all survived before the cause was found.
   Caught by the **mutations**, and by nothing else: the check was green, the
   witness landed, the runner was satisfied. **M8**, stood two rounds.
3. **A construction fact that states a claim also disarms the control aimed at
   it.** `always all g: Guard | lone holds.g` says *one live driver per working
   tree* — `SY-01` — as a fact, and `expect_fail_EN_14_SY_01a` therefore could
   not fire. Caught by the **assumption control**, which is the one thing in the
   suite whose expected result is failure. The construction fact is now one
   holder **per root**, and the lease sits outside the guard exclusivity rule.
   **M8**, stood two rounds.

> **The three instruments are not redundant and this session needed all three.**
> Witnesses catch an empty universe. Mutations catch a vacuous check. An
> assumption control catches a fact that has absorbed the claim — and it is the
> only one that can, because it is the only command that fails when the model is
> *too strong*. The sibling scopes recorded the first two; the third is new, and
> its shape is that **a premise-break control is also a vacuity detector for the
> claim it names**.

**M1** `alloy-only` for all three. **M2** `structure`. **M3** — 3, 2 and 0
respectively: the first announced itself as a total absence of instances, the
second as three surviving mutations that had to be read against the fact set, and
the third produced no trace at all. **M4** `none` — all three are model-level;
their durable residue is the family README's two incidents and this entry.

**A composition seam, recorded rather than solved.** `SY-02`'s fourth conjunct —
*a tree operation happens only under a lease* — is stated **for a driver only**.
A session reaches the tree by matching a live generation, and a live generation
exists only because a driver holding a lease opened one; but `launch` is not
modelled in this slice, and written unqualified the conjunct is **false at state
0**, which the check reported twice. The session half is `SY-10`'s. This is the
first instance in the lifecycle scope of the shape the finish subtree predicted:
a `SY-` claim part of whose content belongs to another obligation's machinery.

**Caught / missed.** Caught: an assumption the table does not carry; three
false-confidence incidents with three different catching instruments; a vacuous
conjunct beside a real one (*what a process has taken is totally ordered by
`below`* is true of any subset of three guards, and read as content); a witness
first landing at exactly its own bound; and the composition seam above. Missed
by construction, and recorded: `SY-01.b`'s release half; the session half of
`SY-02`; and the guard-order clause at two of five acquisition sites, which is
**unexercised within these bounds** — a grant cannot violate an order the wait
already satisfied, since `seen` does not change while a process is blocked, and
the take-tree site's clause can only be violated by a re-acquisition nothing
admits. Both become load-bearing when a slice resets `seen` per iteration.

**Cost.** Fourteen commands, **13 s wall** for the whole file under
`models/run.sh` (322% CPU, 39.8 s user), or about **12 s serially**. Authoring
≈ **2.5 h** for six obligations (**M5**: 0.42 h/obligation), of which roughly
**1 h** was the three incidents. **M7 wrangling**: about 20 m, essentially all of
it the `lone`-under-`not-in` bisect, which presents as a silent total
unsatisfiability with no error and no diagnostic.

> **THE COST LAW IS UNMEASURABLE AT THIS SIZE, AND THAT IS THE H8 DATUM.** The
> JVM-plus-parse floor on this host is **0.58 s**, measured three times on a
> one-signature file, so the entire fourteen-command file spends about **4.6 s**
> actually solving and every command sits within 2× the floor. The sibling
> scopes' law — states-at-which-a-transition-is-enabled × the bound, plus ~10 ms
> of translation per static atom per command — has nothing to bite on here, and
> a percentage carried out of this file would be run-to-run noise. **The reason
> is the placement.** A system-level model that reads two contracts at their
> observations is cheap in exactly the way a third copy of them would not be, and
> **0.42 h/obligation sits inside the component-local range already recorded**
> (0.18, 0.50, 0.58 at entries 028, 029 and 030). On this one file, system-level
> placement costs no more per obligation than component-local placement — which
> is the operand **H8** names, offered as one point rather than as a result.

**Counterfactual.** Two of the three incidents are not available by reading.
The `lone`-under-`not-in` fault is visible to a careful reader *in principle* and
was not visible to this one; what made it findable in minutes was that the
witnesses all went empty at once — a review of the same text would have read the
fact as its English gloss, which is what the author did while writing it. The
vacuous `SY-11.a` is worse: the fact and the claim are on different pages of the
same file, both correct in isolation, and the only thing that connects them is a
mutation nobody would run by hand. The `EN-14` incident is the clearest — the
control exists precisely to be run, and it is a **two-line command that
falsifies a fact a reviewer would have approved**.

**Verdict.** The slice is green — **14 commands, 6 obligations, 6 of 25 alloy
cells filled, 19 correctly reported empty**, one mutation per obligation with
four killed, two recorded as unkillable with the reason, and both assumption
controls firing with the counterexamples retained. One catalogue finding
(`EN`-table row for kernel lock release), three false-confidence incidents, one
composition seam, and the lifecycle scope's Alloy column opened.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6.2.0
`.202501090817` (git `794226d`), `org.alloytools.alloy.dist.jar`, Corretto
`21.0.12.1+9-LTS` — the host's default `java` is 16.0.1 and below Alloy 6's
floor. **Solver**: SAT4J (distribution default), every command with `-n` and
`-t text`. **Bounds**: `for 3 but 2 WtId, N steps`, `N` from 3 to 6 and **N is
STATES**; the six witnesses first land at 2, 4, 2, 4, 5 and 2, measured by sweep
from 1 to 8. `3 Proc` is load-bearing — `SY-11.b`'s cycle needs two blocked
processes and its non-vacuity witness a third holder. No `Int`. **Fairness**:
none assumed; nothing in this slice is a liveness claim, and `SY-13`'s
existential reachability is a sibling's. **Symmetry**: no `exactly` scope.
**Abstractions**: a guard *wait* as an observable state (`Proc.waits`,
`Deferred`) — the catalogue is explicit that a wait is not an outcome, and
`SY-11.b` is a claim about waiting, so the abstraction is this file's own and is
declared as one, exactly as the task-tree model declared its `Deferred`; the task
root as *present or absent*; two process roles and no generation value; the later
layout gate as one action standing for every subsequent revalidation.
**Deliberately omitted**: `launch` and `reap`, two of the seven Lifecycle
actions, because no obligation in this slice reads a spawn. **What a green run
does not prove**: two obligations have no firing protocol-level mutation, two
order clauses are unexercised within these bounds, and `SY-11.b`'s back edge is
unreachable *by construction* with `EN-07` granted — which is what makes the
`EN-07` control, rather than the check, the evidence that the check can see one.

### 041 — Three survivors with three different causes, and two stops the closed set cannot name (lifecycle iteration)

**Scope.** System lifecycle, **system-level** (`models/system/`). `SY-04.a`,
`SY-04.b`, `SY-08`, `SY-10.a` and `SY-10.b`: at most one lifecycle transition an
iteration under a live configuration, selection taken once and not recomputed,
and a stale session that cannot act. Five obligations, sixteen new commands
(thirty in the file), six new reachable transitions, seven new `var` fields and
two new signatures. **The lifecycle scope's empty alloy cells fall from nineteen
to fourteen.**

**Independence protocol: held.** No Quint model of this subject exists, and no
`.qnt` file was opened. The sibling scopes' Quint columns were not read.

**Formalism.** Alloy 6.2. Not a choice — the node brief fixes both families, and
this is the Alloy column.

**Situation.** The admission slice was the loop's guard stack; this is the turn
the loop takes once it holds it. The slice was cut around the machinery its
claims need — an iteration boundary, a configuration that can go invalid under a
running loop, a launch window that is a state rather than an atomic step, and a
launch generation with an identity, which `admission-k51` deliberately did not
have. Two of its inherited obligations were carried into it as open questions:
`Proc.seen` had never been reset, and two of `SY-11.a`'s five acquisition-site
clauses had survived every mutation.

**THE MATERIAL RESULT IS THAT THE CLOSED OUTCOME SET CANNOT NAME TWO STOPS THE
CATALOGUE'S OWN CLAIMS REQUIRE TO BE VISIBLE.**

> **`SY-10.b` says a contended generation *times out into a visible stop*, and
> §*Outcomes* has no name for that stop.** It is not a `Refused`: the closed
> refusal-reason list carries `EpochStale`, which is `SY-10.a`'s **mismatch**,
> and nothing for a handoff timeout. It is not a `Blocked`: §*Outcomes* scopes
> blocks to *a transaction stopped part-way*, and `FN-25`'s two diagnoses are
> both about finish ownership, with the partition explicitly *over `Blocked`
> outcomes and nothing else*. The situation is shipped and documented —
> `one-live-driver-per-working-tree` says the driver "stops `blocked`" when
> post-reap invalidation times out against an orphaned shared guard.
>
> **`SY-04.b` is the same finding's second instance.** *Full configuration
> validation SHALL precede every transition* requires validation to be able to
> fail, and no refusal reason names a configuration failure either.
>
> **[disposed by `closed-set-additions-k74`]** **Both are now catalogue members, and one of them changed level on the way.** The reason set gained `ConfigurationInvalid` and `GenerationContended`; the closed **outcome** set gained nothing, so `Stopped` is now `Refused(RefGenContended)` in `lifecycle.als`. The *not a `Refused`* half of the `Stopped` argument was circular — it rested on the reason set being closed against the case — and `lifecycle.qnt`, which placed it as a refusal, was the column that had it right. A word collision drove the wider reading: `one-live-driver-per-working-tree`'s "stops `blocked`" is the *epoch invalidation* being blocked, not the catalogue's `Blocked(b)`. Record: `docs/adr/a-refusal-leaves-nothing-standing.md`.
>
> The model declares `Stopped` and `RefConfigInvalid` as its own, exactly as it
> declares `Deferred`. **Adding the two reasons is not this leaf's call**: a new
> member of a closed set imposes a matching outcome on the Quint column and
> changes what `formal-synthesis-k16` reads off the coverage matrix, which is
> the same reason entry 039's `SY-05` constraint and entry 040's missing `EN`
> row were recorded rather than acted on.

**M1** `alloy-only`. **M2** `refusal` — and this is the third `refusal`-class
finding in this experiment reached by writing a total action and discovering the
outcome set has no member for one of its branches. **M3** — 0 for both: neither
arrives as a counterexample, both arrive as *there is nothing to write here*,
which is the second time in this experiment that a zero-score signal was the
informative one. **M4** `none` — there is no defect to write a failing test
against; the correction is two rows in a closed set.

**THE METHODOLOGICAL RESULT IS THAT A SURVIVING MUTATION IS THREE DIFFERENT
THINGS, AND ONLY A DIFFERENTIAL PROBE TELLS THEM APART.** Eleven mutations were
run for five obligations. Three survived, and each survived for a different
reason:

1. **An unsatisfiable mutation, and its shape is new.** **M11** added
   `World.gen' != World.gen` to `doTimeout` — beside that transition's own
   `launchSame` frame, which says `World.gen' = World.gen`. The transition
   became unreachable and reported exactly as a survivor. The general rule is
   cheap to apply before the fact and was not in the finish scope's register:
   **a mutation that ADDS a conjunct must be checked against the transition's
   FRAME predicates, because a frame is the one place a model states the
   opposite of a mutation without naming the field.** Rewritten to replace the
   frame rather than contradict it, it fires immediately. This is a **seventh**
   entry for `crates/grove-finish/models/README.md`'s *six ways for a mutation
   to fail its aim*.
2. **A bound too small to reach the mutation's antecedent — the *scope trap*,
   confirmed rather than predicted.** `SY_10a` was written at five states, was
   green, and **M10b survived at five and again at six**. The defect — a grant
   admitting a session whose generation rotated while it was blocked — needs six
   transitions to build: the wait, the holder's release, the driver's iteration
   boundary, the driver's rotation, the driver's death, and the grant. At seven
   it fires. The check's bound is now seven. **M8**, stood two rounds.
3. **A live mutation the check's own shape cannot see.** **M5b** — the take-tree
   site dropping its order clause — survived, and a differential probe shows the
   mutation is *not* inert: *a `TakeTreeA` whose predecessor state already had
   `TreeG in p.seen`* is **unsatisfiable in the original and satisfiable in the
   mutant**. `admission-k51` predicted this clause would become load-bearing the
   moment a slice reset `seen`, and it did. But `SY_11a` is stated over
   `p.seen' - p.seen`, the guards **newly** seen, and a re-acquisition adds
   nothing to `seen` — so the check's antecedent is empty on exactly the traces
   the mutation newly admits.

> **THE SHAPE CHOSEN TO BE ROBUST AGAINST A NEW SITE IS BLIND TO A REPEAT AT AN
> OLD ONE.** `admission-k51` wrote `SY_11a` over every guard newly seen rather
> than over a list of transition names, deliberately, so that a sixth
> acquisition site could not silently escape it. That decision was right and it
> has a cost nobody had priced: the quantifier ranges over *first* acquisitions
> only. No `SY-` obligation states anything about re-acquisition, so inventing
> one is not this file's to do; `formal-synthesis-k16` inherits it.

> **[disposed by `routing-and-prose-k73`]** Routed to the model owners: the defect is in `SY_11a`'s shape and not in anything the catalogue says. A re-acquisition property, if ever wanted, is a new `SY-` obligation and a new cell in both families.

**And the fourth possibility did not occur, which is worth recording too.** M5 —
the grant site's clause — survived again, and the probe is unsatisfiable in the
mutant as well as in the original, so it is genuinely still belt on fastened
braces. The reason is now structural rather than incidental: **`doIter` is
guarded on `no p.waits`, so a blocked process cannot cross an iteration
boundary** — the one thing that could have changed `seen` under a wait is
exactly what the boundary's own guard excludes.

**M1** `alloy-only` for all four. **M2** `structure` for 1 and 3, `ordering` for
2. **M3** — 0, 2 and 0: the first and third produced no trace at all and had to
be read against a probe, where the second's trace named the transition once the
bound admitted it. **M4** `none` — all four are model-level; their durable
residue is the family README's three incidents, its mutation matrix, and this
entry.

**A composition seam CLOSED, and it was closed where the catalogue put it rather
than where it was found.** `admission-k51` recorded `SY-02`'s fourth conjunct —
*a tree operation happens only under a lease* — as stated **for a driver only**,
with the session half owed to `SY-10`. It is now `SY_10a`'s third conjunct (an
ambient tree operation happens only while the session holds an epoch guard)
composed with its second (a session acquires one only at a matching generation).
**Widening `SY_02` instead would have been false**: a driver may release its
lease while a generation it opened is still live, so *some driver holds a lease*
is not an invariant of the protocol. This is the first seam in this experiment
that was declared owed by one slice and discharged by the next, and the useful
part is that the discharge is in a *different* command from the declaration.

**A design observation, recorded and not acted on.** `SY-04.b` gates every
Lifecycle transition but `acquire-lease` on a valid configuration — the
exemption is `SY-02`'s own word, *refused at lease acquisition before
configuration validation* — and `release-lease` is therefore **unreachable under
an invalid configuration**. A driver whose configuration goes invalid mid-loop
cannot release its lease by returning. That is not a sink: `SY-01.b` makes
process death an ordinary release, and it is the exit the shipped driver takes.
`SY-13` (*no stable state is a sink*) is the `sessions` sibling's and this is
the first concrete state it will have to classify.

**Caught / missed.** Caught: two refusals the closed set cannot name; three
survivor causes with three different diagnoses; a check green at a bound too
small to reach its own mutation; a quantifier blind to re-acquisition; a
composition seam discharged; and four of six inherited witnesses landing one
state later than the admission slice measured them. Missed by construction, and
recorded: the epoch record's two *inactive* write points, collapsed into
rotation; *byte-identical* read at the grain of a task root that is present or
absent; and the launch's choice of child process, which no obligation reads.

**Cost.** Thirty commands, **37 s wall** for the whole file under
`models/run.sh` (355% CPU, 126 s user), or **37 s serially**. Authoring ≈
**3.0 h** for five obligations (**M5**: **0.60 h/obligation**), of which roughly
**0.7 h** was the three survivors. **M7 wrangling**: about 25 m, of which the
larger part was the M11 frame contradiction and the M10b bound sweep — both of
which present as an ordinary passing mutation and neither of which announces
itself.

> **THE SECOND H8 POINT, AND IT IS DEARER THAN THE FIRST.** `admission-k51`
> recorded 0.42 h/obligation; this slice records 0.60. The system-level mean
> over the two files is **0.50 h/obligation**, against the component-local
> points already logged (0.18, 0.50, 0.58 at entries 028, 029 and 030). **On two
> files, system-level placement still costs no more per obligation than
> component-local placement** — and the rise within the scope has a nameable
> cause that is not placement: this slice paid for three survivors and a bound
> correction, where the first paid for three false-confidence incidents. Offered
> as a second point, not as a trend.

> **THE COST LAW IS STILL UNMEASURABLE AT THIS SIZE, AND THE SECOND MEASUREMENT
> CONFIRMS THE FIRST.** The JVM-plus-parse floor is 0.58 s, so a thirty-command
> file spends about 20 s of 37 s actually solving. The slice added **six
> reachable transitions** — which the task-tree scope priced at +41% CPU for
> four — and **every command still sits between 0.96 s and 1.43 s**, within
> 2.5× the floor. The one exception, `SY_10a` at 2.37 s, also had its bound
> raised from 5 to 7 in the same edit, so the two are not separable and no
> percentage is carried out of this file. **A model composed at observations
> does not pay the transition-count law**, because its transitions carry almost
> no state.

**Counterfactual.** None of the three survivors is available by reading. The
frame contradiction is two lines forty apart in one predicate, both correct in
isolation, and the reviewer's eye reads a frame as background. The bound is a
number a reviewer approves without a model to run it against. The third is the
worst: **the check, the mutation and the clause are each individually right**,
and what is wrong is a relationship between a quantifier's range and a
transition's effect that no reading of any one of them exposes. The differential
probe that settled it is four lines and took two minutes, and it is the only
instrument in this session that answered a question the check set could not.

**Verdict.** The slice is green — **30 commands, 11 obligations, 11 of 25 alloy
cells filled, 14 correctly reported empty**, one mutation per reported
obligation with nine firing and two investigated to a stated cause, and both
inherited assumption controls still firing. Two catalogue findings in one shape
(no closed-set name for either of two required visible stops), one methodological
result (three survivor causes, and the probe that separates them), one seventh
mutation failure mode, one `M8` scope-trap incident, one composition seam closed,
and one design observation handed to `SY-13`.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6.2.0
`.202501090817` (git `794226d`), `org.alloytools.alloy.dist.jar`, Corretto
`21.0.12.1+9-LTS`. **Solver**: SAT4J (distribution default), every command with
`-n` and `-t text`. **Bounds**: `for 3 but 2 WtId, N steps`, `N` from 4 to 7 and
**N is STATES**; the seventeen witnesses first land at 3, 4, 3, 5, 3, 3, 3, 5, 4,
4, 4, 3, 5, 6, 4, 6 and 2, measured by sweep from 1 to 8 — **the whole sweep
re-run, because a slice that touches a field a guard reads owes the full sweep
rather than a monotonicity argument, and four of the six inherited witnesses did
move.** `3 Proc` is load-bearing; `Leaf` and `Gen` take the default 3 where 2
would serve, for margin. No `Int`. **Fairness**: none assumed, and `SY-10.b` is
deliberately **not** a liveness property — §*Deliberate omissions* models clocks
and timeouts as non-determinism, so `doTimeout` carries no clock and the check
says only that a wait ends in the waiter's own step and reports something.
**Symmetry**: no `exactly` scope. **Abstractions**: `Proc.waits`/`Deferred`
(inherited, and reused by `SY-10.b` rather than duplicated); `Stopped` and
`RefConfigInvalid` (this slice's, and a finding); `IterA` as an iteration
boundary that is not a catalogue action; the task root as present or absent; an
opaque `Leaf` handle in an unordered `live` set, the walk being `TT-11`'s; the
epoch record's rotation write only. **Deliberately omitted**: the record's two
inactive write points; the launch's choice of child; observation outcomes
(`Empty`, `Ambiguous`) as distinguishable. **What a green run does not prove**:
`SY-11.a` is blind to re-acquisition at an existing site, `SY_10a`'s conjunct 2
needed a bound two states above the one first believed, and `SY-01.b` still has
no protocol-level mutation.


### 042 — A claim the operator can falsify, and an invariant no transition could establish (lifecycle roots)

**Scope.** System lifecycle, **system-level** (`models/system/`). `SY-05.a`,
`SY-05.b`, `SY-06.a`, `SY-06.b`, `SY-07.a` and `SY-07.b`: task-root absence as
the complete fresh-tree discriminator, a fresh root that carries a first live
leaf, and exhaustion yielding exactly one finish leaf. Six obligations, sixteen
new commands (forty-six in the file), eight new reachable transitions, six new
`var` fields on `World` and one new signature. **The lifecycle scope's empty
alloy cells fall from fourteen to eight.**

**Independence protocol: held.** No Quint model of this subject exists, no
`.qnt` file was opened, and the sibling scopes' Quint columns were not read.

**Formalism.** Alloy 6.2. Not a choice — the node brief fixes both families, and
this is the Alloy column.

**Situation.** Admission is the loop's guard stack, iteration is the turn it
takes, and this is what the loop is holding: a grove scaffolded, worked,
exhausted, finished, and whose name then belongs to nobody. The leaf arrived
already constrained by `finish-k8`'s hardest result — **absence is not a fact the
protocol can hold**, because after the quarantine rename the name is free, the
world can occupy it, and it can give what it put there the quarantined root's own
identity (entry 039). Three formulations of `FN-28` died on that one trace, and
this leaf was told to model absence as something Grove *establishes and
preserves* rather than as something that *holds*, on pain of re-deriving the same
counterexample at its own cost. It did not re-derive it. **The trace is a
`witness` here rather than a counterexample**, and the mutation that turns it
back into one (M12b) is the evidence that the formulation is doing work rather
than merely being quiet.

**THE MATERIAL RESULT IS THAT ONE CATALOGUE CLAIM IS FALSE AS WORDED, AND THE
OPERATOR IS THE COUNTEREXAMPLE.**

> **`SY-04.b` says an invalid configuration *leaves the working tree
> byte-identical*, and it does not scope that to Grove.** This slice is the first
> to give the world a way to write the tree — `hand-edit` (`EN-11`) and
> `foreign-write` (`EN-13`), both first-class Environment actions in §*Actions* —
> and the check, inherited green from the `iteration` slice, failed immediately.
> The counterexample is three states long and needs no protocol at all: the
> configuration is invalid, the operator hand-edits a `Legacy` tree into the task
> root's name, and the tree is not byte-identical. Grove did nothing. **The claim
> constrains Grove's transitions, not the operator's hands**, and the wording
> does not say so.

The correction inside the model is one conjunct (`Sys.res' != Environmental`).
The correction the catalogue owes is the same distinction in prose, and it is not
cosmetic: read unqualified, `SY-04.b` promises something no filesystem-hosted
tool can promise, and a reader building a recovery path on *the tree is unchanged
after a refusal* would be building on the wrong guarantee. The class is the same
one entry 038 named — **a claim stated over a system when it is true only of a
component** — and it is worth noting that the `iteration` slice could not have
found it: with no environmental writer in the file, the unqualified conjunct was
green and correct about everything the model could reach.

**M1** `alloy-only` (no Quint model of this scope exists yet). **M2** `structure`
— it is a scoping error in a claim's subject, not an ordering, interruption or
eventuality one. **M3 = 3**: the trace names `HandEditA` at state 1 and shows
`World.cfg = InvalidCfg` in every state; it transcribes directly into a failing
test. **M4** `none` — there is no code defect to write a test against; the
shipped implementation does not claim to stop the operator, only the catalogue's
sentence does.

**THE SECOND RESULT IS METHODOLOGICAL, AND IT IS A NEW FAILURE MODE FOR A FILE
WITH A FREE INITIAL STATE.**

> **`SY_07a` reported a counterexample at state 0, with no transition in the
> trace at all**: a free initial state holding two finish leaves. Nothing was
> wrong with the design, the transitions or the claim — `doAllocFinish` is the
> only site that writes `World.fin` and it writes exactly one. What was wrong is
> that *exactly one finish leaf* is an invariant the transitions **preserve** and
> cannot **establish**.

The general form, and it is the mirror of the failure this file has already
recorded twice in the other direction: **in a model with a free initial state,
every checked invariant is either establishable or preserve-only, and a
preserve-only invariant needs a state-0 fact justified by what the steps already
require.** Written as an `always` fact it asserts the claim and every mutation
against it survives (entry 040's shape). Written as nothing at all it reports a
counterexample about the initial state rather than about the design. **Both
failure modes look like a result**, and the only thing that separates them is
reading the trace instead of the verdict — which, in this instance, took under a
minute because the trace was two lines and one of them said `(loop)` on state 0.

The file now carries four state-0-only facts and the discipline is explicit:
`TracesStartWithNobodyBlocked`, `LeasesStartBoundToTheLiveRoot`,
`SelectionsStartInsideTheTree` and `TheTreeStartsWithAtMostOneFinishLeaf`. None
carries `always`, and each is justified as *what the step already required*.

**THE THIRD RESULT IS THE COMPOSITION DECISION THE LEAF WAS CUT TO MAKE, AND IT
ANSWERED ITSELF ONCE THE OBSERVATION WAS NAMED CORRECTLY.**

`SY-06.b` must complete a `PartialScaffold` and refuse a `Legacy` tree — two of
`TT-18`'s eleven root states — and the node brief's rule is compose at
observations, never at machinery. The question it left open was whether
`World.rooted` should grow into a small classification or the classification
should be imported as opaque predicates. **Neither, quite.** The two states enter
as opaque marks (`World.partial`, `World.legacy`) whose content is entirely the
task-tree model's, and the thing this file owns turns out to be a **difference
between two observations rather than a state**:

> the observation that **enables** a completion is `partial + legacy`, their
> union — which at this scope is *exactly* what *the format witness is absent*
> means, with no format witness anywhere in the signature. The observation that
> **decides** it is `partial` alone: the exact known subset.

`SY-06.b`'s own words are *by the exact known subset, never by the mere absence
of the format witness*, and that sentence is precisely the difference between the
two. **The mutation writes itself** (M14: decide on the union) and it fires at
three states. The result generalises: where a claim forbids deciding on a coarser
signal than the one that should decide, model both signals as observations and
make the check the biconditional between them — no machinery from the owning
model is needed, and none was imported.

The one clause this file could not state is recorded rather than solved:
`SY-06.b` also says the completion runs *before any format classification*, and
this file has no classification step because the marks arrive already made. The
ordering is `TT-18`'s. That is the **fourth instance** of the `TT-24` placement
shape — two `TT-` gaps unfillable from either sibling directory, `finish-k8`'s Q4
instance, and this — and it is cheap for `formal-synthesis-k16` only because it
is written down.

**Caught / missed.** Caught: the catalogue scoping error above, at a cost of one
run; the state-0 counterexample; and the `witness_SY_10a` regression, which is
the fourth item and is worth its own line — **a construction fact added for one
obligation made a different obligation's inherited witness unreachable.**
`AnAbsentRootHasNoEntries` is unarguable (a tree that is not there has no entries
in it), and it silently killed `witness_SY_10a`, which said `always no
World.rooted` as its way of spelling *before it touches the tree*: with no root
there are no live leaves, so no selection, so no **launch**, and the witness needs
one. It reported *no instance*, which is what a wrong witness reports and also
what a broken one does. The replacement (`always Sys.act not in TreeAct`) is
strictly stronger and is what the claim actually says. **Missed by reading, all
four**: the mutation matrix and the runner found every one, and no review of the
diff would have caught the third — the fact and the witness are 700 lines apart
and each is correct.

Not caught, and stated as a limit: this slice's six obligations produced **no
survivors**, so it contributes nothing new to the three-survivor-causes result
entry 041 established. Seven mutations, seven firings, each swept from two states
upward: **all seven first fire at 3 against checks that run at 5**, two states of
margin apiece. That sweep is part of the *no survivors* claim rather than an
extra — entry 041's own M8 incident was a mutation that survived at two bounds
and fired at a third.

**Cost.** Authoring ≈ **2 h 45 m** for six obligations (**M5**: **0.46
h/obligation**), of which roughly a third was the composition decision above and
almost none was debugging: the file compiled and ran with two failures, both
diagnosed from their traces in under five minutes each. **M6 synchronization: 0**
— the Quint column does not exist. **M7 wrangling: ~0.2 h**, entirely the bound
sweeps, which are instrument time rather than tool-fighting.

> **THE THIRD H8 POINT, AND THE FIRST TIME THE SYSTEM-LEVEL COST CURVE SAYS
> ANYTHING.** The three lifecycle leaves now read **0.42**, **0.60** and **0.46**
> h/obligation (mean **0.49**), against the component-local files' 0.18 – 0.58.
> The system-level arm is not dearer per obligation; it is **flatter**, and the
> variance is about the difficulty of the composition question rather than about
> the number of claims.
>
> **WHERE THE RUN COST WENT IS THE NEW DATUM, AND IT IS NOT WHERE THE TRANSITION
> COUNT LAW PREDICTS.** The file went from thirty commands at **37 s wall** to
> forty-six at **105 s and 113 s** across two runs — 53% more commands for
> roughly 190% more wall. The transition-count law would put that on the eight
> new transitions. It is not there. **Thirty-odd commands did not move at all**,
> still sitting between 1.27 s and 1.68 s, within 3× the 0.58 s JVM floor. Three
> commands carry the whole increase: `SY_05a` (new, 4.16 s), `SY_10a`
> (2.37 → 3.23 s) and `witness_SY_10a` (**1.59 → 8.62 s**). What those three
> share is not a transition count — it is `Grove`, the one new **free signature**,
> at the default scope 3. Six new `var` fields on a `one sig` cost nothing
> measurable; one free `var`-referenced sig costs a factor of five in exactly the
> commands whose search ranges over its atoms across states. **The static-atom
> law (~10 ms per atom per command) understates this by two orders of magnitude,
> because what a free sig costs is search and not translation.** The parallel
> utilisation falling from 355% to ~230% is the same fact seen from the runner:
> fewer commands are short enough to overlap.
>
> **AND ONE STATE OF MARGIN CAN COST 8×, ON A SATISFIABLE SEARCH.**
> `witness_SY_06b_a_legacy_tree_refused_rather_than_completed` lands at 3. At 5
> states it takes **10.42 s**; at 4 it takes **1.27 s**. Nothing about the
> instance changes — the extra state is pure search space. This matters because
> the *scope trap* discipline pushes bounds upward by default, and entry 041
> raised six witnesses' bounds on exactly that reasoning. The rule it suggests is
> **margin is bought one state at a time and priced per command**: raise a bound
> where a mutation needs it, and measure rather than round up.

**Counterfactual.** The catalogue scoping error is available by reading, but only
to a reader who already has both environment actions in front of them and thinks
to ask what an operator can do while a configuration is invalid — and the two
slices that wrote and inherited `SY-04.b` had neither action in the file. The
state-0 counterexample is not available by reading at all: everything about the
design is right, and what is wrong is a property of the *claim's shape* against a
*free initial state*, which is a relationship between two design decisions made
in different sessions. The `witness_SY_10a` regression is the least available of
the three — the fact and the witness are 700 lines apart, both correct, and the
failure is a chain of four implications through machinery neither one mentions.
The composition result is the opposite case and worth saying so: it **was**
available by reading, once the question was posed precisely enough, and the node
brief posed it. That is a point for the briefs rather than for the tool.

**Verdict.** The slice is green — **46 commands, 17 obligations, 17 of 25 alloy
cells filled, 8 correctly reported empty**, one mutation per reported obligation
with all seven firing and each swept for its firing bound, and all three
assumption controls firing including `EN-14`'s `SY-05` half, which
`admission-k51` declared owed. One material catalogue finding (`SY-04.b` scoped
to a system where it is true only of a component, M3 = 3), two methodological
results (the establishable/preserve-only invariant classification for a free
initial state; a construction fact for one obligation breaking another's
inherited witness), one composition result (a claim about *deciding on the wrong
signal* modelled as the biconditional between an enabling and a deciding
observation, importing no machinery), one placement instance recorded for
`formal-synthesis-k16`, and two cost results (a free signature, not a transition
count, is where system-level run cost lives; and margin is priced per command).

> **[disposed by `lifecycle-scope-k72`]** **Granted, and generalised once rather than patched twice.**
`SY-04.b` now reads *for anything Grove does*, and the generalisation is a
reading rule at the head of §*Claims — system lifecycle*: an `SY-` claim binds
the world only where it says so and names the assumption or state-table property
that makes the wider reading checkable. A blanket *`SY-` claims are about Grove*
would have been wrong — `SY-05.b` binds the world deliberately, at `EN-14` — and
that is why the rule is about naming rather than about excluding. This entry's
finding and entry 043's `SY-13` sink are the two instances of the class, and they
are decided together. Record:
`docs/adr/a-lifecycle-claim-says-what-it-is-over.md`.

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6.2.0
`.202501090817` (git `794226d`), `org.alloytools.alloy.dist.jar`, Corretto
`21.0.12.1+9-LTS`. **Solver**: SAT4J (distribution default), every command with
`-n` and `-t text`. **Bounds**: `for 3 but 2 WtId, N steps`, `N` from 4 to 8 and
**N is STATES**; this slice's nine witnesses first land at 4, 4, 4, 5, 7, 3, 3, 3
and 2, and **the six inherited witnesses and the seven `SY-04.a` ones were all
re-swept from 2 and every one of them landed exactly where it did before** — the
expected result, since this slice's one change to an existing transition removed
a guard (`doSelect`'s `some World.live`) and removing a guard only adds traces.
`Grove` takes the default 3 and is the file's expensive scope; `3 Proc` is
load-bearing for `SY-11.b` and for `SY-06.b`'s successor witness; `Leaf` and
`Gen` take 3 for margin. No `Int`. **Fairness**: none assumed. **Symmetry**: no
`exactly` scope. **Abstractions**: `Proc.waits`/`Deferred`, `Stopped`,
`RefConfigInvalid` and `IterA` (inherited); **new here** — `initialise-root` as
**two** steps, because `PartialScaffold` is defined by the interval between them
and a single opaque step answers `SY-06.b` by construction; `allocate-finish-leaf`
split out of the opaque tree step, because one opaque step cannot carry an actor
rule for one mutation and not another; `World.partial`/`World.legacy` as opaque
marks owned by `crates/grove-task-tree/models/`; `doProveCommit` and
`doSettleDeletion` as one opaque step each for `FN-11` and `FN-19`; a grove's
identity as an atom with no contents. **Deliberately omitted**: the quarantine,
the correlation ticket, the manifest, the witness slot, the format witness, the
exact known subset, the classification order, leaf state and retirement, and the
other nine members of `TT-18`. **Granted about the world**: neither environment
action deletes the task root — §*States* says a root whose deletion is not proven
is never `Absent`, and `EN-14` is the one place its negation is exercised; a
`hand-edit` that could empty the name would put `EN-14`'s counterexample inside
the assumed scope. **What a green run does not prove**: `SY-06.b`'s ordering
clause is imported and unchecked here; `SY-05.b` is one half of a joint claim
whose other half is the finish scope's; `SY-05.a`'s first conjunct has no
isolating mutation (M12c fires it and `SY-05.b` together); and the three limits
entry 041 recorded all still stand.


### 043 — A step that never learned a classification, and a claim quantified over the operator's imagination (lifecycle sessions)

**Scope.** System lifecycle, **system-level** (`models/system/`). `SY-09.a` –
`SY-09.c`, `SY-12`, `SY-13.a`, `SY-13.b`, `SY-14.a` and `SY-14.b`: the session's
three endings, the crash, the stable-state sweep and the `Blocked` persistence
sweep. Eight obligations, twenty-one new commands (**seventy-three** in the
file), three new reachable transitions, four new `var` fields on `World`, five
new static atoms and **no new signature**. **The lifecycle scope's empty alloy
cells fall from eight to ZERO: the `SY-` column is closed, coverage is asserted,
and `--no-coverage` is gone from `models/system/README.md`'s run line.**

**Independence protocol: held.** No Quint model of this subject exists, no
`.qnt` file was opened, and the sibling scopes' Quint columns were not read.

**Formalism.** Alloy 6.2. Not a choice — the node brief fixes both families, and
this is the Alloy column.

**Situation.** Admission is the guard stack, iteration the turn, roots what the
loop is holding; this is the session itself — how it ends, what an interruption
does to it, and the two exhaustive sweeps the catalogue asks for. It is the last
of four slices in one file, which turns out to matter for the cost result below.

**THE FIRST MATERIAL RESULT IS A MODEL DEFECT THAT THREE SLICES OF CHECKS COULD
NOT SEE, AND A SWEEP FOUND IT ON ITS FIRST RUN.**

> **`doTreeOp` — *any observation, creation or mutation of the task tree* — never
> learned the root classification that the `roots` slice introduced.** That slice
> added `World.partial` and `World.legacy` and taught `doInitRoot`,
> `doAllocFinish` and `doProveCommit` about them. It did not come back to the
> opaque step, so in the model a plain tree operation would **append a live leaf
> to a `Legacy` root** — a tree with no format witness, which the shipped
> `grove-llm` refuses with `FormatLegacy` and of which §*States* says a
> whole-tree classification *stops every read and mutation*.

**Every check was green and every witness landed.** None of the seventeen prior
obligations reads a tree operation against a classification — `SY-06.b` owns the
legacy refusal and owns it at `complete-scaffold` — so the defect was invisible
to the entire inherited command set and to its mutation matrix. What walked into
it was `SY-13`'s sweep, which asks of **every stable state** what leaves it.

**The general form, and it is a new argument for sweeps as instruments:** a
per-claim check set is shaped by the claims, and a transition no claim mentions
is checked by nothing. **A sweep over the state space is shaped by the state
space instead**, so it reaches transitions the claim set does not — which is
exactly what makes `SY-13` worth having as more than a box to tick. This is the
first finding in the corpus produced by *state-space* coverage rather than by a
claim, a mutation or a bound.

**M1** `alloy-only`. **M2** `structure`. **M3 = 3** — the probe's trace names
`TreeOpA` at the state where `World.live` grows against `World.legacy`, and
transcribes directly. **M4** `none` — the shipped implementation classifies
before it operates; the defect is the model's, and the durable correction is the
one conjunct plus the rule below, not a test. By clause 3 of the material-finding
rule this is therefore an **observation rather than a countable material
finding** — recorded in the entry prose, as the rule directs, because the
durable consequence is methodological.

**THE SECOND RESULT IS A CATALOGUE FINDING, AND IT IS ENTRY 042'S CLASS A SECOND
TIME.**

> **`SY-13` says *from ANY stable state there SHALL exist a bounded sequence of
> admitted actions reaching either a live leaf to run or a terminal
> disposition*, and `EN-11` says any well-formed tree is reachable by hand
> edit. Together the antecedent ranges over states the loop never produced — and
> a `Legacy` tree is one of them and is a SINK.** Every admitted action refuses
> `FormatLegacy`; neither terminal disposition is reachable; the only exit is an
> operator action, which §*Actions* places outside the admitted set by
> construction.

**The catalogue knows the shape and declines both repairs.** Its own note says a
`Malformed(reason)` tree is not a terminal disposition because folding it in
"would let the claim be satisfied by a tree nobody can act on" — which is right,
and which leaves the claim **false** rather than weak, on `Malformed`, `Legacy`
and `Foreign` alike. The repair it does not consider is the one taken here:
**quantify over the stable states the loop's own admitted actions reach**, and
make *Grove never manufactures one of the others* a checked claim (`SY_13a`
conjunct 1, mutation M21) rather than an assumption. The class is entry 038's
and 042's — **a claim stated over a system when it is true only of a
component**, here of *what Grove does* as against *what the tree can be in*.

**M1** `alloy-only`. **M2** `structure`. **M3 = 3**. **M4** `none` — there is no
code defect; the sentence is what is wrong. This is the borderline the
pre-registration resolves as **material** (*shipped behaviour the model says is
fine, but a catalogue required to describe it never described it*), and it
**falsifies H7 by H7's own terms**, as entry 042's did.

**AND THE FIRST RESULT MASKED THE SECOND, WHICH IS THE PAIR'S REAL LESSON.**
Before the `doTreeOp` repair, the differential probe *found an instance*: the
opaque step let a live leaf into the `Legacy` root, so the sink was not there.
**A model defect that makes a state escapable reports exactly as a design in
which it is escapable**, and the two are separated only by asking whether the
escape is one the shipped system would take. The corpus's three survivor causes
have an analogue here for *reachability* probes, and this is it.

The sink is established rather than asserted, by a **differential probe** with a
positive control — the instrument `admission-k51` used on M5/M5b:

```text
   ... eventually (X puts the root in that state
                   and after (always only admitted actions and eventually atGoal))
                                          6 states      9 states
  X = hand-edit -> a Legacy root          NO INSTANCE   NO INSTANCE
  X = initialise-root -> PartialScaffold  instance      instance
```

*No instance* alone would be a statement about the probe. Side by side and at
two bounds, the pair is a statement about the design.

**THE THIRD RESULT IS THE SHARP QUESTION THE LEAF WAS HANDED, AND THE ANSWER IS
A DESIGN FINDING RATHER THAN A MODELLING ONE.**

`roots-k53` left a concrete state to classify: `SY-04.b` gates every Lifecycle
transition but `acquire-lease` on a valid configuration, so **`release-lease` is
unreachable while the configuration is invalid** — a driver whose configuration
goes bad mid-loop can neither release, nor open an epoch, nor launch, nor reap,
nor close. A probe confirms it. Whether that is a *sink* turns on one question
and the brief named it: **does process death count as an admitted action?**

**It does not.** §*Actions* puts `crash` in the Environment group whose guard
column reads *none — these are the world's*. So the exit the shipped driver
actually takes — die, and let the kernel release the lease, which is `SY-01.b` —
is outside the set `SY-13` quantifies over, and under the catalogue's own
definitions the state is a dead end.

**It is nonetheless out of `SY-13`'s scope as this file states it, and the
scoping decision is the answer rather than an evasion.** §*States*' stable /
transient distinction is defined over **task-root states**: *a stable state is
one an ordinary invocation may observe and act on*, and every state in the table
it introduces is a root classification. A driver's own process state is not a
§*States* state at all. Recorded because the alternative reading is available
and would make `SY-13` false a third way.

**What the finding really is: `SY-04.b`'s gate is over-applied.**
`acquire-lease` is already exempt because it runs before configuration
validation; **`release-lease` deserves the same exemption for a stronger
reason** — a release touches no tree and launches nothing, so there is nothing
for a configuration to be valid *for*. Two repairs are available (exempt the
release, or admit process death) and `formal-synthesis-k16` inherits the choice.
**M1** `alloy-only`. **M2** `structure`. **M3 = 2** — the probe names the state
and the absent transition but not the repair. **M4** `none`.

> **[disposed by `lifecycle-scope-k72`]** **NEITHER REPAIR IS OWED, AND THE PROBE'S OWN GAP — it names the
absent transition but not the repair — IS WHY.** The dead end was a finding about
a WORD. The catalogue used *lifecycle transition* for two sets and defined
neither: this column read §*Actions*' Lifecycle GROUP, `lifecycle.qnt` read the
stage-changing steps, the two sets have **no member in common**, and both were
green. `release-lease` is not a transition — it advances no stage and writes no
tree — so nothing was ever gating it but this column's reading, and the dead end
dissolves with the definition. **Admitting process death is refused explicitly**:
`crash` is the world's, so admitting it moves what every reachability claim
quantifies over, and a sweep in which the loop may always die finds no dead end
anywhere — the argument this catalogue already makes for hand edits.
`CONTEXT.md`'s *Admitted action* stands unchanged. Record:
`docs/adr/a-lifecycle-claim-says-what-it-is-over.md`.

**THE FOURTH RESULT IS COMPOSITIONAL AND IT INVERTS A RULE THIS FILE WROTE ONE
SLICE AGO.** `roots-k53` split `allocate-finish-leaf` out of the opaque tree step
and generalised it: *prefer splitting out a named transition to widening an
opaque one, wherever a claim is about **which** mutation*. `SY-14.b` is the case
that rule does not cover — *every* action on a blocked tree refuses, naming it —
and the right shape is the opposite: **one opaque `doBlockedRefusal`, and a
single `no World.blocked` conjunct in `mayTouchTree` carrying the whole claim.**
A per-action branch would be twenty-six copies of one sentence with twenty-six
chances to omit one. **The rule generalises: split when the claim distinguishes
actions, fold when it quantifies over them**, and the mutation is the same edit
in both directions (M24 removes the one conjunct).

**Caught / missed.** Caught: the `doTreeOp` defect and the `SY-13` sink, both by
the sweep and both on its first run; the invalid-configuration dead end, by a
probe written because the brief named the question. **Missed by reading, all
three.** The `doTreeOp` defect is the least available of them — the conjunct
that is missing is missing from a predicate 400 lines from the claim that would
have wanted it, and its three sibling transitions each carry the conjunct
correctly, so a diff review sees three right answers and no fourth question.

Not caught, and stated as a limit: **all twenty-one new commands were green on
their first run**, before a mutation was written, in a file whose own README
says a green run is not evidence. Nine mutations were then run against all eight
checks, and **seven of the eight are isolating** — the best isolation any slice
in this corpus has produced, for a stated reason: this slice's obligations are
about four different fields, where `SY-05`'s four conjuncts and `SY-11`'s two
halves were about one apiece. **M23 is the exception** (clearing a block fires
`SY-14.a` and `SY-14.b`, because the second asserts the attempt's frame), and
rather than record the overlap and stop, a second mutation was written against
`SY-14.a`'s *other* conjunct and it isolates. **A claim with two conjuncts about
different things has an isolating mutation even when one of its conjuncts does
not**, and looking for it cost one run. All nine were swept from 2 states
upward: seven first fire at 3, two at 2, against checks that run at 5.

**Cost.** Authoring ≈ **1 h 5 m** for eight obligations (**M5**: **0.14
h/obligation**), of which the largest single block was choosing instruments for
`SY-13` that are not liveness properties. **M6 synchronization: 0** — the Quint
column does not exist. **M7 wrangling: ~0.2 h**, the bound and isolation sweeps,
which are instrument time.

> **THE FOURTH H8 POINT CLOSES THE SYSTEM-LEVEL ARM, AND ITS MAIN CONTRIBUTION
> IS A WARNING ABOUT THE ARM.** The four lifecycle leaves read **0.42**, **0.60**,
> **0.46** and **0.14** h/obligation. The fourth is a third of the cheapest of
> the others, and the naive reading — *system-level modelling gets cheap* — is
> wrong. **The four points are not four independent samples of system-level cost.
> They are a declining marginal-cost curve inside ONE FILE.** `admission` built
> the vocabulary, the guard stack and the frame discipline; `iteration` added the
> loop; `roots` added the tree and the one expensive signature. By the time
> `sessions` arrived, eight obligations needed **three transitions and four
> fields**, and every one of them rode on machinery three earlier sessions had
> paid for.
>
> **What follows for H8 is a counting rule, not a number.** *Authoring hours ÷
> checked claims* aggregated over `models/system/` mixes the cost of building a
> model with the cost of adding claims to one, and those are different
> quantities. The component-local arm has the same structure and the same
> hazard. `formal-synthesis-k16` should either report the arms as curves or
> compare **first slices to first slices**; the four-point mean (**0.41**) is
> reported here for continuity and should not be used as a per-obligation
> estimate for a new scope.
>
> **AND THE RUN-COST PREDICTION FROM ENTRY 042 HELD, WHICH IS THE FIRST TIME A
> COST LAW IN THIS CORPUS PREDICTED BEFORE THE FACT.** Entry 042 concluded that a
> free `var`-referenced signature, not the transition count, is where
> system-level run cost lives. This slice was budgeted on that: **no new
> signature**, five static atoms, four `var lone` fields on the `one sig World`.
> Measured as an A/B on one host in one sitting — **46 → 73 commands (+59%) for
> 104 s → 124 s wall (+19%)**, against `roots`' +53% commands for +190% wall.
> Parallel utilisation went **up**, 251% → 361%, which is the same fact from the
> runner: the new commands are short enough to overlap.
>
> **AND THE CHEAPEST THING THIS SLICE DID TO RUN TIME WAS MAKE A TRANSITION MORE
> CORRECT.** The `doTreeOp` repair is one conjunct narrowing the step everything
> reaches, and it moved the file's two dearest inherited commands further than
> any bound change has: `witness_SY_10a` **8.62 s → 2.28 s**, `SY_05a`
> **4.16 s → 1.72 s**, and two of this slice's own new checks from ~9.8 s to
> ~1.8 s. **A guard on a widely-reached transition removes traces from every
> command at once, where a bound removes states from one.** The corpus's
> budgeting order (a static switch, a narrowed antecedent, a smaller bound) has
> been about a *claim's* operands; this is the same move applied to a
> **transition**, and it is a fourth entry on that list. It arrived as a
> correctness repair and the speed was a side-effect, which is the honest order.
>
> **One variance worth a line.** `witness_SY_14b` measured **9.93 s** once and
> **1.31 s** on each of two re-runs of the identical command and file. The
> sibling scopes' rule — *whole-suite totals do not compare across sessions* —
> sharpens to: **a single per-command reading does not compare with itself.**

**Counterfactual.** The `doTreeOp` defect was not available by reading and was
not available to any *claim-shaped* instrument either — three slices of checks,
witnesses and mutations passed over it. It required an instrument shaped by the
state space, and `SY-13` is the only obligation in the catalogue that is one.
The `SY-13` sink is available by reading only to a reader who holds the
catalogue's `terminal disposition` note and `EN-11` in mind at once and thinks
to compose them; the note's own last sentence shows the author within one step
of it. The invalid-configuration dead end **was** available by reading — the
brief found it and handed it over — and what the model added was the answer to
the question the brief could not settle: whether process death is admitted. That
is a point for the briefs and a point for §*Actions* being a closed table.

**Verdict.** The slice is green and **the `SY-` column is closed** — **73
commands, 25 obligations, 25 of 25 alloy cells filled, zero empty, zero declared
gaps, coverage asserted**, all four assumption controls firing or unreachable as
their table requires including `EN-08`'s two halves, and nine mutations for eight
obligations with all nine firing, seven isolating, and each swept for its firing
bound. One material catalogue finding (`SY-13` quantified over states the loop
never reaches, M3 = 3, M4 = none, falsifying H7 by its own terms), one design
finding for `formal-synthesis-k16` (`SY-04.b` over-applied to `release-lease`),
one model defect found by state-space coverage rather than by a claim, one
compositional rule inverted with its condition stated, and two cost results (a
pre-registered budget that held; and a correctness repair that was also the
file's largest speed-up).

**Model facts** (the pre-registration's fourth addition). **Tool**: Alloy 6.2.0
`.202501090817` (git `794226d`), `org.alloytools.alloy.dist.jar`, Corretto
`21.0.12.1+9-LTS`. **Solver**: SAT4J (distribution default), every command with
`-n` and `-t text`. **Bounds**: `for 3 but 2 WtId, N steps`, `N` from 4 to 9 and
**N is STATES**; this slice's seventeen witnesses first land at 4 (three
`SY-09`), 4 (seven `SY-12`), 8 (`SY-13.a`), 4 and 3, 3, 3 (`SY-13.b`), 3
(`SY-14.a`) and 2 (`SY-14.b`). `witness_SY_13a` is the file's only command at 9
and the only one whose first-landing bound is 8; **one state of margin cost it
2.16 s → 5.20 s**, a third data point for *margin is priced per command*. **No
new signature; no `Int`.** **Fairness**: none assumed — and this is the slice
where that matters most, see below. **Symmetry**: no `exactly` scope.
**Abstractions**: `Proc.waits`/`Deferred`, `Stopped`, `RefConfigInvalid`,
`IterA`, `initialise-root` as two steps, `allocate-finish-leaf` split out (all
inherited); **new here** — `World.signal` (two flags and their absence) and
`World.ending`/`World.halted` as **separate** fields, so that the inference
`SY-09.c` forbids is statable and so that *the loop continues* is not derivable;
`doRecover` as one opaque step carrying only *recovery ran and could not
settle*; `doBlockedRefusal` as one opaque step standing for every admitted
action attempted on a blocked tree. **`Blocked` is NOT an abstraction** — it is
the catalogue's own outcome, imported as a `Result` member carrying **no
diagnosis**. **Deliberately omitted**: `FN-25`'s two diagnoses and their
partition, `FN-26`'s two restorable exits and the operator action that takes
them, `FN-20`'s recovery classification, the signal file's path and bytes, and
everything the three earlier slices omitted. **What a green run does not prove**:
`SY-13`'s antecedent is narrowed and the narrowing is a finding rather than a
fact; `SY-14`'s *until an operator acts* has no operator action in this file, so
what is checked is *no admitted action*; `SY-12`'s three no-repeat conjuncts are
`SY-05`'s neighbours and are not isolating; and the limits entries 041 and 042
recorded all still stand.

**NOT LIVENESS, STATED ONCE MORE BECAUSE THIS IS THE SLICE THAT COULD HAVE
CHEATED.** `SY-13` is existential reachability by the catalogue's own words, and
manufacturing it as liveness — `always (stable implies eventually atGoal)` —
would have been one line and would have been green on a model with no hostile
scheduler in it. It is not written that way. The existential half is carried by
**`run` commands**, a run being an existential over traces: `witness_SY_13a`
exhibits the longest admitted sequence within the bound (**five admitted
actions, six transitions, eight states**) and the four `witness_SY_13b` runs are
the sweep, one per stable class. The checks carry the half a run cannot. **No
command in this slice contains an `eventually` inside an `always`**, which is
the mechanical form of the rule and is checkable by reading.

---

## Distillation — where each entry landed

`formalism-skill-k38` turned this log into
`plugins/linkuistics/skills/model-led-development/`. The log stays as it is: it
is the evidence, and the skill's citation keys (`[003]`, `[017]`, …) are these
entry numbers, so a reader who doubts a rule can come back here and check it.

**This pass covers entries 001 – 025 only**, which was the whole log when it ran.
Everything from entry 026 on is Experiment 2 and is distilled by *Distillation,
second pass* at the end of this file; a reader chasing a `[044]` or `[synthesis]`
key wants that section, not this table.

**Every one of the twenty-five entries is cited.** What follows is therefore an
account of *how much* of each survived and, where an entry contributed less than
it holds, why — because silent omission and considered omission look identical
afterwards.

| entry | where it landed | what did not survive, and why |
|---|---|---|
| 001 | the reachability row, and the free question about isomorphisms | its three defects are the baseline's, not routing evidence; 002 supersedes it as a routing datum |
| 002 | the structural row, the *already guaranteed* question, isomorphism, scope traps | the eight findings themselves — they are about one design, not about the tool |
| 003 | the state-counting rule, the behavioural/interruption/ordering rows, all four modelling traps, the dead-tool defence | the shift-order finding as such; it is the design's, and its *shape* is what the skill carries |
| 004 | witnesses-are-the-test-suite, the reading order, the *untested* property-based row | — |
| 005 | quote-the-predicate, the partial-discharge rule, the adversarial reader | — |
| 006 | read-the-misses-first, mutation controls, the platform-untestable row, the second *untested* row | — |
| 007 | mutate the assumption, not the happy path | its four specific findings; the sharpening is the transferable half |
| 008 | `k` keys want `k` controls, mutate the repair, run a finding's counterexample, use the language's lexer | — |
| 009 | the sizing rule, the idealisation list as refusal classes, one-control-per-mechanism, comments hide unenforced invariants | — |
| 010 | count mechanisms **across** boundaries | its five findings; the counting lesson is what generalises |
| 011 | the exclusion list is the worklist, the can-the-library-check question, re-run the unchanged model as a control | — |
| 012 | witnesses before invariants, the one-instance witness names the control, quantify prose over the property | — |
| 013 | a predicate discriminates and does not characterise, name the value behind each clause, prose in a checked file is unchecked | — |
| 014 | name the argument that reaches each enumerated case, rarity measures the simulator, a forced order is not a rule, recovery advice as a test shape | — |
| 015 | the opaque-sort question (`Eq`/`Ord`/`Hash` are coarser than identity) | the conformance-kit reasoning about congruences, which is specific to that kit's design |
| 016 | read two model functions side by side, refusal payload by subtraction, the inode rather than the content, an edge case built into the layer beneath | — |
| 017 | the whole *what this evidence does not support* section, and the *has this prose been through a model?* replacement | the probe's mechanics (the scoring scaffold, the arm-B isolation conditions) — reproducible from the entry, not needed to act on the result |
| 018 | routing to *neither*, the argument surface discharges an obligation, plant-the-violation probes | — |
| 019 | the design-time reachability table, whose-words-does-this-error-speak, count-then-read | — |
| 020 | one mutation per reported obligation, read a kit's parameter docs as a checklist | — |
| 021 | put each clause beside the glossary entry, stronger-preconditions pre-empt refusals, and the *log as instrument* note | its ten-variant verb-by-verb table, which is grove's |
| 022 | write the fixture's filenames out in full, a by-key surface owes a duplicate-key row | — |
| 023 | *we keep our own check in front of it* is not a reachable row, fold-order refusals need the interpreter | — |
| 024 | an error message read as a specification, mark a table *checked by X against revision Y*, and the 0-for-2 counterfactual record | — |
| 025 | enumerate-then-classify, the tightened-predicate sweep, the summary-layer pass, equivalence tests | — |

### One thing the skill carries that no entry stated in full

Entry 025 names **equivalence tests between two live implementations** as one of
three consumer-side instruments and stops there. The scheduling rule that makes
the instrument usable was stated at the increment's close rather than in an
entry, and is recorded here because `.grove/` does not survive the finish:

> While a verb group is mid-migration there are two live implementations of one
> contract, so an equivalence test costs less than the review that would
> otherwise stand in for it, and it **fails** rather than reassures. **Those
> tests die with the old side**, which is why the evidence has to be spent before
> the contract stage rather than after. Three of seven migration leaves left
> every one of grove's ~130 CLI-contract tests untouched on the first run, and
> the changes the other four forced were each recorded with the reason they were
> forced — which is what turned *this is a pure refactor* into a measured claim
> instead of an argued one.

### What the distillation could not verify

The skill was **not** pressure-tested against a no-skill control, which
`superpowers:writing-skills` asks for and `linkuistics:authoring-conventions`
scopes to behaviour-shaping wording. It is a reference and routing guide, which
that house note exempts — but the exemption is a judgement, not a measurement,
and this log's own standard is to say so rather than imply the check was made.
The instrument that fits is the one entry 017 names: a measure written by a party
that authored neither the skill nor its baseline.

---

### 044 — The first Quint model of grove's own tree, and six things the catalogue does not say (task-tree, component-local)

**Scope.** Task tree, component-local
(`crates/grove-task-tree/models/task-tree.qnt`). All 43 `TT-` obligations.

**Revised in place by `integrate-review-prototype-task-tree-k56`**, after the
adversarial review `review-prototype-task-tree-k55` read the model and the
runner against this entry. Six things changed and each is marked *[k56]* where
it appears: a sixth catalogue finding is recorded (`TT-15.a`); the bulk-mark
finding is restated over what the corrected instrument now establishes rather
than over an inference; the second missing-outcome case is narrowed to the
situation that actually supports it; the `TT-20` trace is counted once, at six
transitions; the *Instrument* paragraph gains what the review found wrong with
the **runner**; and the cost line gains the integration. Nothing recorded here
was weakened without a run behind the weakening — the replay lines are in
`crates/grove-task-tree/models/README.md`.

**Independence protocol — held, with one disclosure.** This session opened no
`.als` file, no model-directory `README.md`, and no entry in 026 – 043 while the
model was being built; it was written from
`docs/specs/semantic-contract.md` alone, exactly as
the Alloy column was. **The disclosure:** *after* the model was complete and
green, locating the next free entry number was done with a heading grep, which
printed the titles of entries 026 – 043 on screen. Several of those titles are
finding-shaped ("a refusal the closed set cannot name"). No finding below was
reached after that moment, and nothing below was altered by it — but the breach
is recorded rather than argued away, and the correct move next time is
`grep -oE '^### [0-9]+' | tail -1`, which leaks nothing. `cross-model-replay-k15`
should treat the M1 tags below as this column's own claim and re-derive them.

*[k56]* **The integration held it too, and had one thing to give up for it.**
`integrate-review-prototype-task-tree-k56` opened no `.als` file, no Alloy
section of any model README, and no entry in 026 – 043; it read this entry, the
Quint section, the catalogue, the model, the runner and the review. The price is
named in *The instrument under the instrument*: the reverse-coverage fix binds
both drivers, and this session could not run `--family alloy` to see it land.

**Situation.** Build an independent, executable account of grove's task-tree
claims: current-format initialisation, selection, decomposition,
insertion/addition, retirement, invalid and foreign roots, opaque entries and
terminality — with every refusal exercised rather than filtered out, and with
crash and hand-edit as first-class behaviours. The Alloy column had already
closed the same scope; this one may not read it.

**Formalism.** Quint 0.32.0, rust evaluator backend, bounded randomized
simulation. Apalache reached but not completed; see *Missed*. Bounds `MAX_OBJECTS = 14`, `MAX_DEPTH = 6`, `MAX_POS = 6`, trace
depth 24, 8000 samples, fixed default seed `0x5e0a51d3c0ffee01`. No fairness
assumptions: nothing here is a liveness claim.

Two structural choices, and they are the same two entry 003 made about the
library — reached again, independently, about a different subject:

- **Every action is total.** Each computes a `Decision` from its snapshot and
  transitions in every case, so a refusal is a value rather than an absent
  transition. Quint's own idiom is the opposite, and taking the idiom would have
  made every refusal claim in the catalogue unfalsifiable by construction.
- **The interpreter is a state machine.** Effects land one at a time, `crash` may
  fire between any two, and every intermediate state is a state the invariants
  are evaluated at. Everything below about interruption follows from that.

A third choice is this subject's own. `TT-21` says every classification an
operation makes comes from ONE listing — which an executable model satisfies for
free, because that is simply how one writes it. So the model carries a
`ONE_SNAPSHOT` dial and a `mutant_two_listings` instance in which later steps
classify from the live tree, and the claim dies there. Without it `TT-21` would
be a green tick over nothing.

**Caught.** *[k56: five, now six]* Six material findings, one observation, and
one instrument that had to exist before any of them counted.

*The two that need interleaving, and that a static model cannot pose:*

- **`PartialScaffold` is not robust to a foreign write.** `TT-20` says an
  interrupted root initialisation classifies `PartialScaffold`, "never as
  `Current(*)` and never as `Legacy`". `PartialScaffold` is defined by an EXACT
  closed subset of the root's contents; `EN-13` grants that a foreign entry may
  appear at any name. Interleave one `foreign-write` with one `crash` during
  `initialise-root` and the extra entry drops the tree out of the subset and
  through to `Legacy` — the classification the claim names as forbidden. In the
  product: an interrupted `root-init` plus any stray file in `.grove/` — an
  editor swap file, a `.DS_Store`, a half-synced artifact — after which grove
  reads its own interrupted work as somebody else's legacy tree.
  The safety argument for the exact subset (every value a completion writes is
  fixed in advance) survives defining the state by the PRESENCE of the scaffold's
  own entries instead, because a foreign entry is not something completion
  writes. *Claim affected:* `TT-20`, and the `PartialScaffold` row of the state
  table. *Durable correction:* the catalogue, and the classification's shipped
  behaviour. *M1* `quint-only` (this column's claim). *M2* `interruption`.
  *M3* **3** — the trace names its six transitions (`beginOp`, three `stepOp`s,
  `foreignWrite`, `crashNow`) and transcribes directly into a test. *[k56: this
  entry said "four transitions" while the model README described six; the
  counterexample is one trace and it is now counted once, at six.]*
  *M4* named below.
- **A bulk mark cannot converge if `AlreadyTerminal` refuses its plan.** `TT-23.a`
  requires the whole plan validated before the first rename; `AlreadyTerminal` is
  a refusal for a single mark. Validate a bulk plan the way a single mark is
  validated, and after an interruption the re-run refuses on the member the
  interrupted run already marked — so `TT-23.b`'s convergence is not merely
  unproven, it is **unreachable**, and the property
  [`bulk-marks-are-not-atomic`](adr/bulk-marks-are-not-atomic.md) exists to buy
  is gone. A member already in the plan's TARGET state must be admissible and a
  no-op. That requirement is implied by two claims and stated by neither.
  *[k56]* **What the instrument now establishes, which is more than the
  inference it replaced.** As first written, the convergence ghost kept sticky
  booleans: any later completed bulk mark set `bulkRepaired`, divergence was
  measured against *that* operation's keys, and a refused retry — which never
  becomes a running operation — could not set `bulkDiverged` at all. The claim
  in this entry was therefore an argument from `TT-23.a` plus `AlreadyTerminal`,
  not an executed result, and the review was right to say so. The ghost now
  retains the interrupted **request**: only the identical key list repairs it,
  "the same result" is every member of *that* plan in the target state, a retry
  refused by the bulk validator falsifies the property where the refusal is
  decided, and a `hand-edit` clears a plan whose tree is gone. On top of that,
  `mutant_bulk_strict` turns the unstated requirement into a control — the same
  model with target-state idempotence off **violates** `TT-23.b`. The finding
  stands; its evidence is now a run rather than a reading.
  *Claim affected:* `TT-23.a`/`TT-23.b`. *Durable correction:* the catalogue,
  shipped behaviour, and a Rust test. *M1* `quint-only`. *M2* `interruption`.
  *M3* **2** *[k56: unchanged, and the reason is unchanged]* — the tool reported
  an unreached witness, which named the situation but not the cause; the cause
  came from reading the plan validator. A model mutation now proves the cause,
  but it was written *after* the diagnosis, so it does not raise the score the
  tool earned.

*The three about the catalogue's own text, which needed no interleaving at all:*

- **`TT-17` is contradicted by the catalogue's own `PartialScaffold`.** "The
  classification SHALL depend only on the format witness, never on any task
  entry's text" — while `PartialScaffold` is decided by an exact comparison
  against a task entry's name AND bytes, and is ordered ahead of the format
  stage. The model checks `TT-17` over the Current/Legacy/Foreign decision, which
  is what its own witness is about, and declares the narrowing.
  *Claim affected:* `TT-17`. *Durable correction:* the catalogue. *M1*
  `quint-only`. *M2* `structure`. *M3* n/a — reached by construction, not by a
  counterexample.
- **`EN-11`'s controls column mis-attributes `TT-24.b`.** The row's expected
  result is that with `hand-edit` removed, every named witness becomes
  unreachable. `TT-24.b`'s witness is reached in ~2% of traces with `hand-edit`
  gone, because `EN-13` grants that foreign entries may appear AT ANY NAME and
  `foreign-write` alone supplies one. The dependency is on `EN-13`. This is
  structurally the same mistake the catalogue already caught and annotated for
  `TT-16` **in the same row** — so the row has now been wrong twice, which is
  worth more than the one-word fix: an assumption's controls column is a list
  nobody re-derives once written. *Claim affected:* `EN-11`. *Durable
  correction:* the catalogue. *M1* `quint-only`. *M2* `structure`. *M3* **3** —
  the control reported "REACHED in 100 trace(s)" against an expectation of zero,
  which is the whole diagnosis.
- **Two situations have no member of the closed refusal set.** (a) An ordinary
  operation meeting a `PartialScaffold` root: `FormatLegacy` is the reason
  `TT-20` explicitly forbids, and `WitnessPending` names a reserved witness that
  is not there. The model refuses `WitnessPending(RPreparing)` as least-wrong and
  declares it. (b) An ordinary mutation **that has already applied an effect**
  and whose next create is no longer licensed, because a non-cooperating writer
  took the destination between the listing and the step. The
  `semantic-contract.md`'s *Outcomes* table fixes three contexts —
  an ordinary operation *before any transaction*, a finish or recovery
  transaction, and the reaper — and this is none of them: `Refused` means the
  tree is byte-identical and the shifts already landed, while the row that
  licenses `Blocked` is about a transaction. The model returns
  `Blocked(OwnershipConflict)`, on the table's own reasoning that a caller which
  has already mutated owes a block rather than a refusal.

  *[k56] (b) was stated too widely and is narrowed here.* The model originally
  returned `Blocked(OwnershipConflict)` for **every** blocked create, including
  an ordinary collision *before any effect had landed* — and that case has no
  gap at all: the tree is byte-identical, so it is a refusal, and the closed set
  names the reason (`ReservedNameOccupied(entry)` when the occupant is the
  artifact `TT-24.b` is about, `DestinationOccupied` otherwise). What looked
  like a missing outcome there was a model that had not read the table's own
  partition. `collisionOutcome` now draws the line, and the finding is stated
  over the half that survives it — which is also the half that needs
  interleaving, and which is reached in ~12% of `scenario_foreign_sibling`'s
  traces (`wit_finding_an_ordinary_mutation_blocked_after_a_partial_mutation`).
  *Claim affected:* the closed refusal set and the `TT-24` context table.
  *Durable correction:* the catalogue. *M1* `quint-only`. *M2* `refusal`.
  *M3* n/a for (a); *[k56]* **3** for (b) — it now has a runnable witness and a
  named trace shape.
- ***[k56]* `TT-15.a` is contradicted by `TT-24.b` and `TT-18`, and it is the
  sixth catalogue issue this column found.** `TT-15.a` requires selection on a
  spent tree to report `Empty`. A current root with no live task and a foreign
  artifact at a name Grove reserves classifies `CurrentSpent` — the walk stage is
  reached and finds nothing live — while `TT-24.b` requires that same tree to
  refuse `Refused(ReservedNameOccupied(entry))` and `TT-18` puts that refusal two
  stages ahead of anything the walk says. All three statements are the
  catalogue's, and under one tree they are inconsistent; `TT-24.b`, whose whole
  purpose is fail-closed ownership, is the one that must win. The model guards
  `TT-15.a` with `walkStageReached`, which states the staging premise the
  obligation leaves implicit. This is a third **narrowing**, structurally
  identical to `TT-17`'s, and the original version of this entry recorded only
  two — which is why it is here rather than only in the model README. *Claim
  affected:* `TT-15.a`. *Durable correction:* the catalogue. *M1* `quint-only`
  (this column's claim). *M2* `structure`. *M3* n/a — reached by construction,
  not by a counterexample.

*The observation, which carries no claim and is therefore not countable:* the
catalogue notes that a model needing the guard-wait to be observable must
introduce it as an abstraction of its own. This model does not need to: the
waiting caller is a member of `pend`, which is a real transition and keeps
`TT-22` falsifiable without adding a tree-level twin of `LeaseHeld` to an
outcome set that has no room for one. The catalogue's note anticipates a cost
that at least one family does not pay.

*The instrument.* *[k56: six premise-breaks, now seven]* Seven premise-break
controls, each naming an obligation that must DIE, and all seven do —
`EN-01`→`TT-20`, `EN-10`→`TT-05` and `TT-12`, `EN-13`→`TT-04` and `TT-24.d`, and
the two model mutations `ONE_SNAPSHOT`→`TT-21.a` and
`BULK_TARGET_IDEMPOTENT`→`TT-23.b`. Five exercise-removal controls confirm their
witnesses go unreachable. Until those ran, the 43 green obligations were 43
unfalsifiable greens.

*[k56] The instrument under the instrument, and this is the review's largest
result.* The controls above are controls on the MODEL. Nothing was a control on
the RUNNER, and `review-prototype-task-tree-k55` found three false greens in it
— each one a way for a green run to mean nothing, and none visible from any
model:

1. **A dead backend reported as a verdict.** `quint verify`'s non-zero exit was
   read as a tool failure only when its output matched one of five known
   strings; every other non-zero exit was recorded as "model-checked … no
   counterexample" for each invariant in the batch. `JVM_ARGS=-Xmx6m` — a heap
   too small for the JVM to read its own jar — matches none of the five, and
   reported 43 verified properties from a backend that never started. The
   defect is the SHAPE of the test, not the length of the list: a list of known
   deaths makes the failure nobody predicted the green one. A verdict now
   requires Apalache's own report (`[violation] Found an issue`, `❌ <name>`) and
   every other non-zero exit aborts with exit 2.
2. **A dead simulator reported as a control PASSING.** `quint run`'s exit code
   was read directly as held/violated, so a `quint` that could not parse, could
   not find its module, or died on startup was recorded as "violated" — and in a
   **premise-break** control, "violated" is what the control requires. A dead
   tool was therefore evidence that an assumption was carrying weight. The
   simulator is now classified the same way: `error: Invariant violated` is a
   result, anything else non-zero aborts.
3. **Reverse obligation coverage that checked a shape rather than the
   catalogue.** Every syntactically valid `TT_nn` was accepted, so
   `inv_TT_99_invented` was counted under a matrix key nothing reads and the run
   stayed green — while the runner's own header advertised that direction as one
   of its four obligations. Each parsed obligation is now validated against the
   manifest, with the same claim-level relaxation the Q4 matrix already makes,
   and a claim-level citation is REPORTED as crediting no cell rather than
   vanishing.

All three are now controls of their own — `models/run-controls.sh`, seven of
them, each mutating a copy of the repository and asserting the named fatal
diagnostic. **The transferable result is the shape**: an instrument that turns
an unrecognised failure into a verdict will always fail in the direction that
looks like success, because success is what "nothing matched" defaults to. Both
model families' green runs to date were read through this runner, and both
drivers share its coverage accounting.

**One consequence is owed forward and is stated as an obligation rather than as
a caveat.** The reverse-coverage check now binds Alloy commands too, and the
session that wrote it was under the independence barrier: it opened no `.als`
file and could not run `--family alloy`. The check is as lenient as the Q4
matrix already is, so an Alloy command can only newly fail by naming an
obligation the catalogue does not define at all — the defect the check exists to
catch. **`cross-model-replay-k15` opens the barrier, and re-running
`models/run.sh --family alloy` under this runner is its first obligation.**

**Missed.** Recorded with the same care, because two of these bound what a green
run here is worth.

- **No model checking, and it took three distinct barriers to establish that.**
  This is the entry's largest single cost and its most transferable result, so
  the barriers are separated rather than summarised, because only the third is
  about the subject.
  1. **A reporter, not a checker.** In one file holding the library, `base`, six
     assumption mutations, a model mutation and four focused scenarios,
     `quint verify` 0.32.0 died in `json-bigint/stringify` with
     `RangeError: Invalid string length` at every depth including
     `--max-steps=2`. Nothing to do with the model; the intermediate JSON simply
     exceeded V8's maximum string length. **Splitting the controls into a second
     file removes it entirely** and Apalache then runs. The first reading of this
     — "Quint cannot model-check this subject" — was wrong, and it was wrong in
     the direction that would have quietly cost the experiment a whole column.
  2. **One predicate, spelled naturally, put the model out of reach.** `gapless`
     was written `ps == 1.to(n)`; Apalache refuses a non-constant integer range
     outright. Restating it as a cardinality plus a bound — `n` distinct
     positions all in `[1, n]` ARE `1..n` — is equivalent and constant-range-free.
     A single line of ordinary Quint decided whether the model was checkable.
  3. **Reachable, and not affordable.** Past both, the full `base` instance
     exhausts a 4 GB JVM heap at `--max-steps=3`, and a deliberately tiny
     `verify_small` — 6 objects, depth 2, a three-action menu, no `hand-edit` —
     reached `State 3` and then ran past 25 minutes without finishing depth 3, on
     a 16-core / 128 GB host with a 24 GB heap. The cost is in the encoding
     rather than the bounds: every transition quantifies over `reached`, a
     bounded unrolling of set union/filter/flatten that a simulator skips and a
     symbolic backend must encode in full at every step.

  **Every `TT-` property here is therefore bounded randomized simulation and
  nothing more.** A green run is evidence over 8000 sampled traces at depth 24,
  not a proof over reachable states, and the `wit_unreach_` controls are evidence
  of unreachability *within* that budget rather than the bounded-unreachability
  instrument `FN-15.d` and `FN-31.a` will need. Had a `TT-` obligation required
  that instrument, this column could not have supplied it. Alloy's `check` has no
  equivalent limitation, and that asymmetry between the families is real —
  but it is an asymmetry of **cost at this encoding**, not of capability, and
  those are not the same claim.
- **`TT-18` remains close to a restatement.** Classification order is what
  `classify` IS in an executable model, and the invariant that reads it back is a
  transcription. `TT-14` and `TT-17` were rescued from the same fate only by
  building perturbation functions (`perturbSlugs`, `perturbText`,
  `mirrorPositions`) and stating each claim as "the answer is unmoved when
  everything that is not the input is changed" — the general shape for making a
  *depends-only-on* claim checkable in an executable model. `TT-18` got no such
  instrument, and is reported green on weaker evidence than its neighbours.
- **Four witnesses could not be reached by unfocused simulation** at any budget
  this suite can afford — `TT-07` at 0.03%, `TT-08` and `TT-09.c` at 0.06%,
  `TT-04`'s renumbering witness at 0.08%. Each needed a `scenario_` instance
  narrowing the action menu. That is a real Quint cost with no Alloy analogue: a
  bounded model checker asked for an instance either finds one or reports none,
  where a sampler reports "not yet" in exactly the shape of "never".

**Cost.** Authoring: one session, roughly one hour wall from bootstrap to a green
suite, model and runner driver together. Run: **19m 59s wall, 1226s CPU** for
`models/run.sh --scope task-tree --family quint`, 109 commands plus 43 skipped
model-checking properties. Against the
pre-registration's M7 baseline — Experiment 1's Quint suite at 3m 29s wall /
2527s CPU for 148 claims — this is **5.7× the wall clock for 109 commands**, and
the pre-registration's own warning ("if its Quint run does not cost materially
more, the suspicion is that it is not modelling the interesting states") is
satisfied in the direction it wanted. The CPU figure is *lower* because
Experiment 1 spent most of its on one instance at six times the sample budget;
here the cost is spread, and the dominant single item is the base witness run
evaluating 43 predicates per state.

Model checking, as its own line item: **roughly ninety minutes** across five
probe runs to establish the three barriers under *Missed*, of which the first
two were tool defects and the third is the subject's. Two durable artifacts came
out of it — the file split and the `gapless` restatement — and both survive the
finding that model checking is unaffordable, which is what makes them cost
rather than waste.

*[k56] Integration, as its own line item.* Applying this review cost roughly one
session: the two runner false greens and the reverse-coverage direction, the
bulk-mark instrument rewritten around a retained request, the collision-outcome
partition, seven runner controls that did not exist, and this entry. Run cost to
verify: one full `--scope task-tree --family quint` suite, plus about four
minutes of runner controls, which run at `QUINT_SAMPLES=1` precisely so that an
instrument check is not priced like a suite. The suite after integration:
**19m 46s wall, 1209s CPU** for 111 commands — within noise of the 19m 59s /
1226s figure below, which is worth recording as a null result. Two more
commands, a retained plan list, a retained decision and an extra branch in the
blocked path cost nothing measurable; the cost of this model is where *Missed*
says it is, in `reached`'s bounded unrolling, and not in the ghosts. **The review cost about a third of
what the model cost and changed what every one of its greens is worth**, which
is the number worth carrying: the producing session cannot run the controls that
would falsify its own instrument, because it is the thing being controlled for.

Wrangling, distinct from run time and from the above: perhaps twenty minutes, in
three places.
`nondet x = S.oneOf()` binds only the next expression inside an `all { }` block,
which silently scoped a binding out of the rest of an action. `--invariants`
takes an array, and a shell that does not word-split an unquoted variable
(zsh) hands the whole list to quint as one expression, whose error message
prints all thirty-six names and says none of them is valid. Doc comments
(`///`) between the variants of a sum type are a parse error where ordinary
comments are not.

**Counterfactual.** *[k56: recounted over six findings, and one of them moved
sides.]* Four of the six material findings are about *what the catalogue says*
and needed no execution at all — `TT-17`, `EN-11`, `TT-15.a`, and the
`PartialScaffold` half of the missing refusal reasons. A careful adversarial
read of `docs/specs/semantic-contract.md` against itself would have found all
four, and `review-design-model-contract-k31` is where that read happened and did
not. The mid-flight half of the missing-outcome finding moved the other way
under review: stated widely it looked like a reading result, and once narrowed
to the case that supports it, it is an interleaving result like the other two. What
the model added there was **not insight but obligation**: every claim had to be
stated as something that could fail, and stating `TT-17` as a runnable predicate
is what made its contradiction with `PartialScaffold` unavoidable rather than
merely available.

*[k56]* The three that needed the model genuinely needed it, and needed this
*kind* of model. `PartialScaffold`-versus-`foreign-write` is a four-transition interleaving
of two independently granted assumptions; the bulk-mark convergence failure is
visible only once "the plan is validated" and "re-running converges" are both
executable and one is interrupted. The mid-flight block needs a world
actor moving between an operation's listing and its next step. None is reachable
by reading. A structural model with no crash action and no world actor reaches
none of them either — which is the pre-registration's `EN-08`/`EN-11` point
arriving as evidence rather than as a principle.

**Verdict.** Reach for Quint again for exactly this: **claims about what an
interrupted operation leaves behind, when the world can also write.** Its cost
here was almost entirely in making rare interleavings land — four scenario
instances exist for no reason but sampling — and its ceiling is that on a model
this size there is no model checker behind it at all, so nothing it reports is
ever more than bounded evidence. Do not reach for it to audit a specification's
internal consistency; that is a reading task, and the model only forced the
reading.

**What a green run here does not prove.** Not that any `TT-` property holds — only
that no counterexample appears in 8000 sampled traces of depth 24, at
`MAX_OBJECTS = 14`, `MAX_DEPTH = 6`, `MAX_POS = 6`, with two cooperating
processes, with the digest an opaque equality, and with `hand-edit` drawing from
an enumerated family of 25 trees rather than from every well-formed tree
`EN-11` grants. Not that `TT-18` is checked in any strong sense. *[k56]* Not the
literal text of `TT-17`, `TT-20` or `TT-15.a`, each of which is checked over a
declared narrowing because the catalogue contradicts itself there — nor `TT-10`
over anything but an operation's own arguments, which is a declared
qualification rather than a narrowing; all four are listed in
`crates/grove-task-tree/models/README.md`, *Narrowings and
qualifications, each declared*.

> **[disposed by `task-tree-scope-k70`]** All three narrowings are gone, and in
> every case the **catalogue** moved rather than either model. `TT-17` split into
> `.a` (the format decision, unchanged in force) and `.b` (the witnessless
> decision reads bytes, not only names); `TT-15.a` gained the staging premise its
> guard had been supplying, because classification is a function of the tree and
> an outcome is a function of the operation; `TT-20`'s prohibition on `Legacy`
> narrowed to the window in which ownership is provable, and the interrupted root
> met by a foreign write now classifies `PartialScaffold(Ambiguous)`. `TT-10`'s
> qualification stands and was deliberately not reopened. From that revision on,
> the sentence above is history: no `TT-` obligation here is checked over less
> than its text. Not anything at
all about the finish or lifecycle scopes, whose Quint columns do not exist, and
whose empty cells keep an unqualified `models/run.sh` red — which remains the
truth about the repository rather than a defect in the runner.

**Derived tests.** For the implementation phase, in the existing black-box
binaries rather than in a new seam (`Test seams`, *findings reach the product
through the existing black-box binaries*):

| finding | test |
|---|---|
| `PartialScaffold` vs a foreign write | scaffold a root, interrupt before the format witness lands, drop one foreign file beside the scaffolded entries, and assert the root classifies `PartialScaffold` — not `Legacy`. Red against the exact-subset definition. **[`task-tree-scope-k70`: the decided contract is narrower and the test must be too.** Assert `PartialScaffold(Ambiguous)` — which **refuses** — and only once a root-init-exclusive entry has landed. With the charter alone and no leaf, `Legacy` is the contract and a test asserting otherwise is red against the decision rather than against the code.**]** |
| bulk-mark convergence | interrupt a bulk mark between two of its renames, re-run the identical invocation, and assert it succeeds and leaves every named entry marked — not that it refuses `AlreadyTerminal`. Red against a validator that treats an already-marked member as invalid. |

*[k56: three, now four.]* The other four findings — `TT-17`, `EN-11`, `TT-15.a`,
and the missing refusal reasons — are catalogue corrections with no shipped
defect behind them, and legitimately record **`M4 = none`** — the
pre-registration's third borderline, which each of them therefore falsifies H7
by the terms H7 sets.

*[k56]* The mid-flight block has no derived test either, and for a different
reason: what it asks for is a decision the catalogue has not made — whether an
ordinary mutation that has already applied an effect owes a `Blocked` or a
fourth context row of its own. A test written before that decision would encode
the model's least-wrong choice as though it were the contract.
`formal-synthesis-k16` decides; the witness is runnable in the meantime.

> **[disposed by `task-tree-scope-k70`]** **Neither.** The withheld test was the
> right call and the decision went outside the two options this entry framed.
> `crates/ordinal-fs-tree` — the catalogue's delegated boundary — applies every
> mutation through one interpreter that **unwinds what it applied** on a reported
> error (`Error::Failed`, "the tree is as it was found", checked there as
> `inv_atomicity`), and its destination check exists for precisely this collision
> because "the lock is advisory". So the outcome is a **refusal**, unconditionally,
> and the missing thing was never a context row: it was an ungranted assumption,
> now `EN-17`. The model had built an interpreter without the capability because
> nothing granted it, then read the consequence as a gap in the catalogue.
> **The general form is worth more than the instance** — an assumption no `EN-`
> row names is one a model may quietly decline, and this is the second recorded
> case (the other is process death under `SY-01.b`). The derived test now exists
> and is the one this entry could not write: interrupt an insert between its
> shifts and its create, take the destination from outside the lock, and assert
> the operation refuses with its own shifts undone and the intruder untouched.


### 045 — A protocol whose own steps fall outside the assumption they rest on, and six things the catalogue does not say (finish/recovery, component-local)

**Scope.** Finish and recovery, component-local
(`crates/grove-finish/models/finish.qnt`). All 61 `FN-` obligations.

**Revised in place by `integrate-review-prototype-finish-k58`**, after the
adversarial review `review-prototype-finish-k57` read the model, its twelve
scenarios and its eight model mutations against this entry. Six things changed
and each is marked *[k58]* where it appears: **finding 6 is reframed** — half the case it
rested on is not a `Blocked` outcome at all, so the catalogue conflict is about
the swap's OUTCOME first; the model now refuses the half the catalogue says is
refused, and the half that blocks keeps the partition finding; the
`FN-25.b` instrument gains the comparison its prose always claimed and did not
make; the rootless completion proof becomes attempt-bound; `Current(Live)`
stops being an undeclared vacuous branch; the model-mutation count goes from
eight to eleven, because `FN-14`, `FN-26` and `FN-30` were true by
construction; and the mutation and Q4 matrices say which of their rows are
bundles. **What the review did NOT overturn** is the reachability record: all
129 witnesses were classified protocol-established, 0 construction-established,
0 unclear — the search dial is not manufacturing this column's witnesses. Every
weakening below has a run behind it; the replay lines are in
`crates/grove-finish/models/README.md`.

*[k58]* **The integration held the independence barrier.** It opened no `.als`
file, no Alloy section of any model README, and no entry in 026 – 043; it read
this entry, the Quint section, the catalogue, the model, the controls and the
review.

**Independence protocol — held, with one disclosure.** This session opened no
`.als` file, no Alloy section of a model-directory `README.md`, and no entry in
026 – 043 while the model was being built; it was written from
`docs/specs/semantic-contract.md` alone, exactly as
the Alloy column was and exactly as entry 044 was. **The disclosure:** *after*
every finding below had been reached and recorded in the model, locating the next
free entry number was done with a heading grep, which printed the titles of
entries 033 – 043 on screen. Several of those titles are finding-shaped, and one
of them — "a third outcome the closed set cannot name" — is the same *shape* as
finding 2 below. The timing is checkable and it is why the disclosure is worth
making rather than waving away: `RRolledBack` and `ONotEntered` were both written
into the model, with their declaring comments, before the grep was run, and the
diagnosis-partition counterexample had already been found and fixed. **No finding
below was reached after that grep**, and none was renamed to match a title. What
the grep does mean is that this column cannot claim to have been blind to the
*existence* of Alloy findings in the finish slice — only to their content.
`cross-model-replay-k15` is where that is settled, and it should treat finding 2
as an overlap candidate with a known contamination risk rather than as a clean
`quint-only`.

**Situation.** Build an independent, executable account of Grove's finish and
recovery protocol — the entry, the preflight, the witness, the evacuation, the
commit, the two handoffs, the disposal and the recovery — as guarded actions with
total outcomes, and check all 61 `FN-` obligations under all three VCS lanes,
with crash and restart as first-class behaviour.

**Formalism.** Quint 0.32.0, rust evaluator backend, bounded randomized
simulation: 8000 samples at depth 24, fixed seed `0x5e0a51d3c0ffee01`.
`quint verify` (Apalache 0.56.1) **completes and returns a verdict**, on the
reduced `verify_small` instance at the runner's own default depth: all 61
property commands at `--max-steps=4` in 445s, no violation *[k58: 377s before
the integration added three consts to the model's state]*. That is more than
the task-tree column got — it could not finish depth 3 at all — and less than it
sounds; see *What a green run here does not prove*. One library (`finish.qnt`,
2,736 lines) carrying 61 property commands, one unfocused instance, one
verification instance, and a
controls file carrying twelve focused
scenarios, seven assumption mutations and — *[k58]*, after the review —
**eleven** model mutations. 228 commands in all, 19 of them inverted
`inv_fail_` controls.

**The model is a protocol step machine, not a filesystem**, and that is the
structural decision everything else follows from. `crates/grove-task-tree/models/`
owns what a task tree *is*; this file owns the ORDER of the steps that end one,
what each makes persistent, and what a crash between two of them leaves behind.
The task tree is abstracted to the set of entries the transaction evacuates.
`Step` is a closed twenty-six-member type and `persistentEffect` is a total
function on it, because `FN-24.b` cannot be answered at all by a model whose
steps are implicit in its actions.

**Caught.** Six material findings and three observations. Every one is a defect
in the tool-neutral catalogue rather than in shipped behaviour — this phase
changes no product behaviour — so each is recorded and **not fixed**: the
independence barrier freezes `docs/specs/semantic-contract.md`, the Alloy column
recorded rather than fixed its own, and `formal-synthesis-k16` owns the
disposition of all of them.

**1. `EN-01` is narrower than the protocol's own steps, and `FN-24.b` is what
makes that visible.** `EN-01` grants atomicity to *a same-directory rename* and,
in the catalogue's own words, "to nothing else". `FN-24.b` then requires every
step of the transaction to make at most one persistent effect, and for that
effect to be a same-directory rename **or to be decomposed**, with anything else
declared. Writing the step list out, six of the protocol's most load-bearing
steps are neither:

| step | effect | why it is not `EN-01`'s |
|---|---|---|
| `SEvacuate` | cross-directory rename | `root/<e>` → `root/<witness>/<e>` |
| `SRestore` | cross-directory rename | the same, backwards |
| `SCreatePreparing`, `SWriteManifest`, `SCreateMarker`, `SReplaceStage` | create | atomicity rests on `mkdir`/`O_EXCL`, which the catalogue never grants |
| `SCommit`, `SReproducePreflight` | version-control commit | not a filesystem effect at all — which is what `EN-05` says |
| `SRemoveWitness`, `SReplaceCleanup`, `SDisposeEntry`, `SRemoveMarker`, `SRemoveQuarantine` | remove | individually atomic and idempotent, which is what makes disposal re-enterable — but not renames |

Nothing here is *wrong* about the protocol: a cross-directory rename is atomic on
any POSIX filesystem, and `FN-08` already proves the operands are on one device
before the transaction runs. What is wrong is the assumption the claims are
stated to rest on. `EN-01` should say *a rename whose operands are proved to be
on one device*, which is what `FN-08` establishes and what the protocol actually
uses; or `FN-24.b`'s declared list becomes permanent and the catalogue owes an
`EN-` assumption for exclusive creation. Affected: `EN-01`, `FN-24.b`, and
through them `FN-09`, `FN-11`, `FN-17`, `FN-19`.
*M1* `quint-only` (pending replay) · *M2* `structure` · *M3* n/a (found by
writing the enumeration, not by a counterexample) · *M4* `none` — a catalogue
defect with no pre-fix failing behaviour to write a test against, which
**falsifies H7 by the terms H7 sets for itself**.

**2. The closed refusal-reason set has no member for a clean rollback.** `FN-29`
requires a `NotCommitted` finish to "leave the grove exactly as it was, with the
finish leaf live and selectable", and to "be distinguishable by the operator from
a block". A refusal that says nothing is not distinguishable from anything; and
of the seventeen refusal reasons, every one names a precondition or a guard and
none names *the deletion commit is absent*. The model adds `RRolledBack` and
declares it as an addition.
*M1* `quint-only`, **with the contamination risk the disclosure above names** ·
*M2* `refusal` · *M3* n/a · *M4* `none`.

**3. The closed outcome set has no member for a transaction never entered, and
that makes `FN-01.a` unfalsifiable.** `FN-05.a` says the first preflight member
"produces no refusal at all — the transaction is simply never entered". In a model
where every action is total, "never entered" is an *absent transition*, and an
absent transition makes `FN-01.a` true by construction — which is the exact
hazard totality exists to remove. The model adds `ONotEntered`, exactly as the
catalogue anticipates a model naming the guard wait it also refuses to put in the
set. The catalogue should either name the non-entry or state that `FN-01.a` is
checked over an abstraction every family must add.
*M1* `quint-only` (pending replay) · *M2* `refusal` · *M3* n/a · *M4* `none`.

**4. `FN-13`'s witness says "refused" where the catalogue's own rule says
"blocked".** The witness is "a commit attempted while the witness is tracked,
refused". But `FN-11` puts a complete evacuation and a ready manifest in front of
every commit, so by the time one is attempted the transaction has published a
witness and moved every entry — and *Outcomes*' **One artifact, three contexts,
one decided outcome** says a caller who has already mutated is owed
`Blocked(OwnershipConflict)`, because "a transaction has already mutated, so a
block is the honest stable state". The two statements cannot both hold of this
transition. The model blocks, and records the tension.
*M1* `quint-only` (pending replay) · *M2* `refusal` · *M3* n/a · *M4* `none`.

**5. `FN-22`'s table and the blocked-diagnosis definitions disagree about
`Indeterminate`.** `FN-22`'s last general row says `Indeterminate` observed at any
point performs no handoff and leaves `Reserved(Published)` carrying
**`RecoveryPending`**. The `Blocked` diagnoses say `OwnershipConflict` is where
"the observed topology matches neither the recorded anchor nor the expected
result". An `Indeterminate` *caused by* a topology drift satisfies both, and it
is reachable. The model resolves it — a correlated artifact Grove has just proved
is its own defeats "cannot classify it as its own", so correlation wins — and
that resolution is a choice the catalogue should make rather than leave to two
independently written models.
*M1* `quint-only` (pending replay) · *M2* `refusal` · *M3* 2 — the counterexample
named the transition · *M4* `none`.

**6.** *[k58, reframed — the case this rested on divides, and only half of it is
a `Blocked` outcome the partition is stated over. The finding survives on both
halves, for two different reasons.]* **`FN-06` and *One artifact, three contexts*
disagree about what a mid-transaction root swap PRODUCES, and the disagreement
is resolvable on a fact the transaction already knows.** `FN-06` says the swap
is "a refusal rather than a mutation applied elsewhere", and its witness is "a
swap between two steps, **refused**". *Outcomes*' **One artifact, three
contexts** says a caller who has already mutated is owed
`Blocked(OwnershipConflict)`, because "a transaction has already mutated, so a
block is the honest stable state". Both are stated over the same transition and
the catalogue never says which governs.

The model first sent **every** identity mismatch to a block, and that is what
produced the original form of this finding: a blocked state with no correlated
manifest (the swap can precede the write) and an untouched topology, which
neither diagnosis's predicate covered. But `FN-25`'s partition is expressly over
**`Blocked` outcomes and nothing else**, so a case the catalogue calls a refusal
cannot be evidence about that partition — it is evidence about the outcome, one
level up. That is the review's point and it stands.

The resolution is that the two statements are not actually in conflict: they
partition on whether the transaction has made anything **persistent** yet.
Before its first persistent effect the tree and the repository are
byte-identical, so `FN-06`'s refusal is available and is what the model now
returns (`RootIdentityChanged`, witnessed in 3.8% of `scenario_edit_txn`
traces); after it, the block is the honest stable state (7.4% reach the swap, so
3.6% block). **What the catalogue owes is that split**, which it does not make
anywhere — and `FN-06`'s bare "refused" is wrong for half of its own transition.
This is the same shape as finding 4, which is why the two should be
dispositioned together.

**And the partition finding survives the reframe, for the other half.** The
post-persistent swap is a block by the catalogue's own three-contexts rule, so
it is squarely inside `FN-25` — and at that moment the correlated manifest may
still be absent and the topology untouched, so **neither** diagnosis's predicate
holds. That is measured rather than argued: deleting the model's swap clause
from `unprovable` violates `inv_FN_25b` in `base` and in `scenario_edit_txn`.
The resolution stays `OwnershipConflict`'s own definition — whatever sits at the
reserved name now sits inside a root Grove did not pin — and the catalogue still
does not say so. So finding 6 stays one finding and the entry's count of six is
unchanged — but the catalogue owes it on **two** counts, and on firmer ground
than the single one it was recorded as: the outcome split, and the diagnosis for
the half that blocks.
*M1* `quint-only` (pending replay) · *M2* `refusal` · *M3* 2 — the trace named the
step (`SWriteManifest`) and the state (`WPreparing`, no manifest) · *M4* `none`.

**Three observations, which are not countable findings** because no tool-neutral
claim changes:

- **`Indeterminate` after a proven commit has exactly one cause.** Found by
  `FN-22.g`'s witness failing to land. Obscuring the commit's result decides
  nothing once the exact ticket exists — the evidence classifier reads the ticket
  first — and dropping the ticket alone restores the anchor, which classifies
  `NotCommitted`. The only route to `FN-22`'s `Committed -> Indeterminate` row is
  an operator action that drops the result **and** moves the topology: a rebase
  over the attempt's own commit. Worth stating because that row is the one
  `FN-22` insists must not be collapsed into its neighbour.
- **Eleven model mutations were needed, against `task-tree-k11`'s two** —
  *[k58]*: eight were written by the producing session and **three more were
  owed and missing**, which is the sharper form of the same observation. An
  executable model of a *protocol* satisfies most of its own ordering claims by
  being written in that order, and the count is the measure of it.
  `mutant_short_preflight` kills `FN-06`, `FN-07`, `FN-08` and `FN-12.b`
  simultaneously: four obligations resting on the order the preflight happens to
  check things in. That is worth knowing before any of the four is cited as
  evidence about anything — and *[k58]* it is a **bundle** control, since it also
  skips the layout precondition; `mutant_unproven_ownership` is the second, since
  it also disables the root-identity stop. Six of the eleven are minimal.
  *[k58]* **The three that were missing are the transferable part.** `FN-14`,
  `FN-26` and `FN-30` were each asserted over a field **no transition in the
  model could make bad**: the commit wrote `hooksRan: false` unconditionally, and
  `unrelatedMutated` and `historyRewritten` were initialised false and only read.
  Their witnesses landed, and were protocol-fed, and the safety properties were
  still true by construction — so a reachability audit alone would have passed
  all three. The test that catches it is not "is the witness fed by a
  transition" but **"is there any transition that could make this property
  false"**, and the cheap way to ask it is to try to write one.
- **`FN-31.a`'s witness is forced, not occasional.** The marker records disposal
  progress, so the FIRST disposal step already carries a value the next must
  supersede: a state requiring a replacement is reached in 40.9% of
  `scenario_march` traces against the 28.0% that run a disposal to completion.
  The replace transition is reachable *before* a disposal can finish. Q3's reachability
  question therefore answers *reachable* — and Q4's row for the same transition
  reads `none`, because removing the whole quarantine mechanism breaks no
  shared-safety obligation. Those are different findings about the same object and
  `formal-synthesis-k16` owns which one decides Q3.

> **[disposed by `routing-and-prose-k73`]** Routed to `finish-verdicts-k65`, which owns all four `TODO.finish_process.md` verdicts.

**Missed.** Recorded with the same care, because two of these bound what a green
run here is worth.

- **The index image is a flag nothing reads.** The colocated lane backs up the
  user's Git index before the preflight snapshot can export into it, and restores
  it on an uncommitted result. In this model that is `repo.indexRestored`, set and
  never consulted: no obligation is stated over it, so Q4's row for it is declared
  `abstracted` rather than answered. A whole lane-specific mechanism this column
  says nothing about, and the one Q4 row this family cannot supply.
- **No eventuality is checked.** `FN-23`'s idempotence and `FN-21.a`'s
  resumability are the two claims that read like liveness, and both are checked
  here as an invariant plus a reachability witness. Nothing in this column is a
  temporal property, and a claim of the form "recovery *will* terminate" is not
  made and could not be made without a fairness premise the models have no
  grounds to grant.
- **`FN-25.b`'s "exhaustive sweep of the blocked states" is exhaustive only
  within 8000 samples at depth 24.** The obligation asks for a sweep; what it gets
  is a bounded search. *[k58]* The witness said `blockedStatesSeen > 0` — **one
  reached block, reported as an exhaustive sweep** — and now reports the block
  SITES the search reached instead.
- *[k58]* **The `FN-25.b` agreement was claimed and was not being checked**, and
  this is the entry's own worst miss. The instrument is sound in design — two
  encodings of one partition, compared — but the invariant only asked whether the
  classifier returns exactly one answer, and every block site derived its
  diagnosis from that same classifier. One site did not: the tracked-witness
  commit named `OwnershipConflict` by hand, over a state with a correlated,
  provably Grove-owned artifact, which is `RecoveryPending`. The comparison now
  exists (`carriedDisagreed`) and that site classifies like every other. **A
  claimed instrument that is not wired up produces exactly the output of one that
  is**, which is the *vacuous invariant* hazard at the level of the property
  rather than of the witness — a form the pre-registration does not name.

**Cost.** Authoring: one session, from bootstrap to a green suite; *[k58]* plus
one integration session for the review above, whose whole cost was six findings
triaged, four model changes, three new mutations and two re-runs of the suite. The
distribution is worth recording because it is not where it was expected: the
model's *state and step machine* took about a third of the session, its
*claims* another third, and the last third went almost entirely on **making
witnesses reachable** — which is the cost item this scope has and the task-tree
scope did not.

**The reachability cost is the finding about method, and it is quantified.** The
subject is a transaction of roughly twenty steps, and while it runs every
environment action is enabled at every step. An unfocused search therefore
reaches the end of the transaction with probability `(1/k)^20`, and at 8000
samples every claim from the commit onward would have been reported green on a
witness that never landed. The remedy is a *search dial* — `ENV_BUDGET`,
`ENV_PHASES`, `ENV_KINDS` and per-kind narrowing — and twelve `scenario_`
instances built on it. It removes no behaviour; `base` grants a budget no trace
can spend, at every phase, of every kind, and every PROPERTY is checked there
unfocused. What it cost is twelve instances and four rounds of "this witness
does not land, why". Two of those rounds were findings in their own right: one
produced the `Indeterminate`-after-a-commit observation above, and one produced a
two-transition shortening of the march (`SEvacuate` and `SRestore` now leave for
the next step on their last entry rather than spending a transition discovering
there is nothing left to move), without which the refusal branch and the success
branch could not both fit inside the runner's depth of 24.

**Run cost (M7).** `models/run.sh --scope finish --family quint` — **exit 0,
4m 25s wall / 317s CPU** on a 16-core host, 228 commands, 61 of 61 coverage
cells complete, Q4's removal matrix 10 of 10 rows. *[k58]* Those are the
post-integration figures; the producing session's were 4m 05s / 293s over 223
commands, so the review's three extra mutations and two extra witnesses cost
about 8% of the run. Bounds: one evacuable entry,
three lanes, depth 24, 8000 samples, seed `0x5e0a51d3c0ffee01`. Backend: the
rust evaluator.

That is **an order of magnitude cheaper than the task-tree column's 19m 46s**,
and the reason is worth recording rather than being read as this scope being
easier: the task-tree model unrolls a tree walk to `MAX_DEPTH` on every
transition, and this one does not have a tree. What this scope pays for instead
is *reachability engineering* — the twelve scenarios — which costs authoring
hours rather than run seconds, and therefore lands in M7's `wrangling` column
rather than in its run wall-clock. **A comparison of the two Quint models on run
cost alone would be measuring the presence of a tree walk.**

**One M8 entry, and it is the runner's rather than the model's.** A full runner
invocation was first taken as `models/run.sh … | tail -45`, which reports the
exit status of `tail` — so a run that had gone red printed a summary and was
read as green. Caught within one command, by noticing the runner was re-running
invariants individually (which it only does after a batch fails) while its
output claimed nothing had failed. It stood for about fifteen minutes. The
runner is not at fault; the pipeline was. Recorded because the pre-registration's
**dead tool** hazard has a sibling nobody named — *the muffled tool*, a runner
whose verdict is discarded by the shell around it — and one line of it is worth
more than a paragraph after it happens again.

**Counterfactual.** For each material finding: would prose have caught it?

1. **`EN-01` narrower than its own steps** — no, and this is the clearest case in
   the set. The assumption and the steps that rest on it are three sections apart
   in a 1,537-line document, and nothing in prose makes anyone write the step
   list out. `FN-24.b` is an obligation that *demands the enumeration exist*, and
   the finding falls out of writing it. A reviewer reading the catalogue would
   have to hold "same-directory" and "root/<e> → root/<witness>/<e>" in mind
   simultaneously and notice they disagree.
2. **No refusal reason for a clean rollback** — possibly. A careful reader
   checking `FN-29` against the seventeen reasons could find it. What forced it
   here is totality: the model had to *return something*.
3. **No outcome for a transaction never entered** — no. This is only visible once
   an action is required to be total, and it is the second time in this
   workstream that totality has produced a missing-member finding.
4. **`FN-13` refused vs blocked** — possibly, by the same reader as (2). What
   forced it here is that the model reaches the commit step with a published
   witness and an evacuated tree in its own state, so the question "what can
   honestly be said here" is unavoidable.
5. **`FN-22` vs the diagnoses on `Indeterminate`** — no. Two sections, both
   correct in isolation, contradicting only on a state that has to be constructed.
6. **The mid-transaction swap** — no, and it was not caught by reading either.
   It came from writing the outcome down and having to choose one: `FN-06` says
   refused, the three-contexts rule says blocked, and an executable model must
   return exactly one value. *[k58]* The entry first credited this to the
   `FN-25.b` agreement instrument; the review showed the agreement was not being
   checked at the time, so the credit is wrong. What actually produced it is
   **totality**, for the third time in this entry (findings 2, 3 and 6), and that
   is the transferable part. The agreement instrument is still worth the two
   lines it costs — once wired up it found a real disagreement immediately — but
   it found a different thing.

**Verdict.** Reach for Quint again for exactly this: **a protocol whose claims
are about the ORDER of its steps and about what an interruption between two of
them leaves behind.** The step list as a closed type with a total
`persistentEffect` function turned `FN-24.b` from an unanswerable obligation into
a table a reader can check, and it produced the largest finding in the set.
Reach for it also for **partitions that must be exhaustive**: encoding the
classification twice and checking the agreement is cheap. *[k58]* But encode it
twice **and then actually compare the two** — this column wrote the second
encoding, described the comparison in the obligation's own prose, and asserted
something weaker; a call site that named its own diagnosis went green for the
whole of the producing session. The property to write is
`carried == classified`, not `classified.size() == 1`.

Do **not** reach for it — on this subject, at this size — expecting model
checking to establish the properties. See below.

**What a green run here does not prove.** Not that any `FN-` property holds over
the reachable states — only that 8000 randomized traces of depth 24 found no
counterexample, under one entry, three lanes, and the environment budget each
instance declares.

**The model-checked result is real and it is shallow, and the gap between those
two facts is the honest reading.** `quint verify` finishes on `verify_small`
over all 61 properties at `--max-steps=4` and reports no violation, and *[k58]*
still does after the integration. That is the
direction simulation cannot give, and it is a result the task-tree column could
not obtain at any depth. But the incumbent protocol's shortest path from entry
to a settled refusal is **eleven steps**, so a depth-4 check reaches the
published witness and stops — it verifies the beginning of the transaction and
says nothing about the commit, either handoff, disposal, or recovery. Quoting
"model-checked, no counterexample" without the depth beside it would be the
pre-registration's *scope trap* stated as a result.

**Depth is what costs, not the number of invariants**, and that is a
transferable measurement: 3 invariants at depth 3 took 373s and 61 invariants at
depth 4 took 377s — *[k58]* 445s after the integration widened the state by
three consts, which moves the figure and not the conclusion. A reader planning a
Quint verification budget on this subject
should price the depth and treat the property count as free.

And nothing here is evidence about the index image, which is abstracted to a
flag no obligation reads.

**Derived tests.** For the implementation phase, in the existing black-box
suites rather than in a new one:

- **`tests/finish_lifecycle.rs`** — a finish whose commit is reported as failure
  while the exact attempt-bound commit exists must classify `Committed` and settle
  forward (`FN-15.a`). The model reaches it by nondeterminism in one step; the
  test needs a seam that lies about the exit status.
- **`tests/finish_lifecycle.rs`** — a task root swapped between two steps of the
  transaction must stop with a diagnostic naming the artifact, and must not apply
  its pending effect anywhere (`FN-06`). *[k58]* **Two cases, not one**: swapped
  before the transaction's first persistent effect it must REFUSE and leave the
  tree and repository byte-identical; swapped after, it must BLOCK with a
  diagnosis. Which one a given step gets is finding 6.
- *[k58]* **`tests/finish_lifecycle.rs`** — a retry that has lost every local
  trace must not settle forward on a deletion commit whose message names this
  finish handle and a **different attempt** (`FN-03`, `FN-04`). The model cannot
  reach the state; the test can construct it, and it is the one place the
  attempt half of the correlation ticket earns its keep.
- **`tests/finish_lifecycle.rs`** — after the quarantine rename, an operator
  action that both drops the result and moves the topology must return the
  quarantine atomically and block, not refuse (`FN-22.g`). The observation above
  is what says this is the only way to reach that row, so it is the only shape
  the test can take.
- **`src/finish_cleanup/auxiliary/tests.rs`** — a disposal resumed after an
  interruption inside the marker replacement must reach the same terminal marker
  value as an uninterrupted one (`FN-31.c`), and no reader may observe the marker
  absent (`FN-31.b`).

### 046 — A claim that contradicts its own definition, and a loop that satisfies eighteen of its claims by being written down (system lifecycle, cross-component)

**Scope.** The system lifecycle, cross-component
(`models/system/lifecycle.qnt`). All 25 `SY-`
obligations.

**REVISED BY `system-k60`, WHICH INTEGRATED `system-k59`'s REVIEW OF THIS
INSTRUMENT.** The review's subject was the one thing `cross-model-replay-k15`
will not read — the search shape, the narrowings, and the controls this column's
green was reached with — and it produced six findings, all six of which were
verified and applied. **What changed here is marked inline.** Two of the changes
are downgrades of claims this entry originally made, one is a withdrawal of a
finding, and one is a correction of a measurement that could not be reproduced.
The barrier held through the integration too: no `.als` file and no entry in
026 – 043 was opened.

| review finding | what it was | disposition |
|---|---|---|
| F1 | `driverStep` forced a launched finish session through teardown and interpreted the signal before invalidating the epoch | **model repaired** — a live session may now end at any point, the methodology's reopening exit is modelled and witnessed, and reap / invalidate / interpret are two steps in production's order |
| F2 | the launched session was modelled as continuously holding the launch generation | **abstraction rebuilt** — the epoch is a shared/exclusive lock; `SY-11.b`'s control survives over the faithful graph |
| F3 | `mutant_no_signal_is_done` was claimed as a bundle control over three endings and killed only one | **claim withdrawn, four isolating controls added** — every obligation now has one, and each new mutant asserts its neighbours green |
| F4 | the `SY-14.a` sweep read "an action succeeded" as "a block was cleared" | **measurement corrected**, and **finding 3 below is half withdrawn** as a result |
| F5 | the flat-menu figure came from an uncommitted instrument and did not reproduce | **instrument committed**, measurement re-run; **observation C below is corrected** |
| F6 | `SY-05.b` checks two values one setter manufactures together | **claim downgraded** to the internal consistency check it is |

**Independence protocol — held, with one disclosure that matters more than
usual.** This session opened no `.als` file, no Alloy section of any
model-directory `README.md`, and no entry in 026 – 043. It was written from
`docs/specs/semantic-contract.md` alone, as the
Alloy column was and as entries 044 and 045 were.

**The disclosure.** Locating the next free entry number was done with a heading
grep, exactly as entry 045 discloses doing, and it printed the titles of entries
031 – 043 — which include the Alloy column's **four lifecycle entries**, 040 –
043. This is a sharper contamination than 045's, because those four are entries
about *this scope*. Two of the titles are shape-adjacent to findings below:
041's *"two stops the closed set cannot name"* to finding 2, and 043's *"a step
that never learned a classification"* to finding 4. The timing is checkable in
the model's own text:

- **Findings 1, 2 and 5 pre-date the grep.** `RConfigInvalid`'s declaring
  comment, the `loopAdmits` comment on the missing deferral refusal, the
  `namesTheBlock` comment, `SWEEP_ALL_STABLE` and `isHandEditRefusal` were all
  written before it, and are the commits that carry them.
- **Findings 3 and 4 post-date it.** Both were reached while debugging why two
  witnesses would not land, hours after the grep. Neither was renamed to match a
  title, and neither has content taken from one — only a title was ever visible —
  but this column cannot claim to have been blind to the *existence* of an Alloy
  finding about a step and a classification.

**`cross-model-replay-k15` should treat findings 3 and 4 as overlap candidates
with a known contamination risk, and only findings 1, 2 and 5 as clean
`quint-only` candidates.**

**Situation.** Build an independent, executable account of the joint — sessions,
exhaustion, finish entry, interruption, restart, blocked recovery, ownership
conflict, and the two successful exits — as guarded actions with total outcomes,
composing the task-tree and finish/recovery *observations* without re-deriving
either component's internals, and check all 25 `SY-` obligations.

**Formalism.** Quint 0.32.0, rust evaluator backend, bounded randomized
simulation: 8000 samples at depth 24, fixed seed `0x5e0a51d3c0ffee01`. One
library (`lifecycle.qnt`, ~2,250 lines) carrying 25 property commands and 20
witness commands, one unfocused instance, one verification instance; a controls
file (`lifecycle-controls.qnt`) carrying five focused scenarios, two assumption
controls and twenty-three model mutations. **86 commands in all, 35 of them
controls** — 24 inverted `inv_fail_` mutations, two `wit_unreach_` assumption
removals, eight neighbour properties asserted green inside the five isolating
mutants, and one positive control inside `scenario_flat_menu`. Full suite:
**3m 40s wall, 254s CPU**, green,
`models/run.sh --scope lifecycle --family quint`.

*(Counts as of `system-k60`'s integration. As the producer left it: ~1,750
lines, 21 library witnesses, three scenarios, eighteen mutations, 72 commands,
2m 05s. The growth is `system-k59`'s repairs — the isolating controls and their
neighbour assertions, the committed flat-menu instrument, and the two witnesses
that moved into scenarios once a live session could end at any point.)*

**`quint verify` (Apalache 0.56.1) completes and returns a verdict.** On the
reduced `verify_small` instance, three state invariants at `--max-steps=4`:
**`NoError` in 983s**. That is a real result at a real depth, and it is smaller
than it sounds — depth 4 does not reach a scaffold, let alone a finish; see
*What a green run here does not prove*.

**The model is a COMPOSITION, and that is the structural decision everything
follows from.** `crates/grove-task-tree/models/` owns what a tree is;
`crates/grove-finish/models/` owns the transaction that ends one. Here the tree
is a ten-field summary with `TT-18`'s classification order over it, and the
finish is a five-phase cursor plus the two facts the lifecycle reads off it —
whether the deletion is proven, and whether the ticket is persistent. No
positions, no keys, no walk, no manifest, no quarantine, no lanes. `SY-05` is
the clearest justification: `FN-11` and `FN-19` are checked next door, and
`SY-05.b` is the claim that they *compose* into a sound inference.

**Caught.** Five material findings and three observations. Every finding is a
defect in the tool-neutral catalogue rather than in shipped behaviour — this
phase changes no product behaviour — so each is recorded and **not fixed**: the
independence barrier freezes `docs/specs/semantic-contract.md`, both sibling
columns recorded rather than fixed their own, and `formal-synthesis-k16` owns
the disposition of all of them.

**1. `SY-13` contradicts its own definition of *terminal disposition*, and the
contradiction is reachable.** The claim quantifies over "any stable state" and
requires a bounded sequence of **admitted** actions to a live leaf or a terminal
disposition. It then defines a terminal disposition as *"an ending from which
the loop has no further admitted action of its own"* — and immediately says
there are **exactly two**, naming a proven finish and a blocked tree, and
explicitly excluding `Malformed`.

> **[disposed by `lifecycle-scope-k72`]** **The catalogue takes the repair it had not considered.** `SY-13`
now quantifies over *any stable state Grove's own admitted actions can reach*,
and carries the companion in the same sentence — *and Grove SHALL never
manufacture one of the others* — as `SY-13.a`'s first conjunct rather than as an
assumption, which is what makes the narrowing sound rather than convenient.
`lifecycle.qnt` gained `mutant_grove_manufactures_legacy` so the companion is
falsifiable; `mutant_literal_sy13` is now a control on the DISPOSED text rather
than a declaration that the column checks less than the contract says. **A fourth
member joined the excluded class after this was decided**:
`task-tree-scope-k70`'s `PartialScaffold(Ambiguous)`, which Grove's scaffold
never writes and from which Grove has no admitted exit. Record:
`docs/adr/a-lifecycle-claim-says-what-it-is-over.md`.

Those two sentences disagree. `Legacy`, `Foreign` and `Malformed` are reached by
a hand edit and left by a hand edit, and a hand edit is not an admitted action —
`SY-13`'s own note puts operator actions outside the admitted set *by
construction*, which is what makes a blocked tree terminal. So each of the three
is an ending from which the loop has no further admitted action, which is the
**definition** of a terminal disposition; and each is excluded by the
**enumeration**. Under the enumeration, `SY-13.a` and `SY-13.b` are simply false
at all three.

The model narrows the sweep to the classes Grove's own actions can produce and
declares it; `mutant_literal_sy13` runs the literal text and
`inv_SY_13b_no_stable_state_is_a_sink` dies. The `relax_EN_11` control is the
other half of the evidence: with `hand-edit` removed, all three classes become
unreachable, which is the measurement that the sink set is exactly the
hand-edit-reached one. Either the enumeration grows a third member — *a refusal
state a hand edit reached* — or `SY-13` quantifies over the admitted-reachable
stable states and says so. Affected: `SY-13.a`, `SY-13.b`, and the `States`
table's `Legacy`/`Foreign`/`Malformed` rows.
*M1* `quint-only` (pending replay, clean — pre-dates the grep) · *M2* `eventuality`
· *M3* 2 — the mutant names the class and the trace names the state · *M4*
`none`, a catalogue defect with no pre-fix behaviour to write a test against,
which **falsifies H7 by the terms H7 sets for itself**.

**2. Three lifecycle stops the closed refusal set cannot name.** The catalogue's
refusal reasons are closed and enumerated, and three things the `SY-` claims
require a caller to be told have no member:

| the stop | required by | what the model had to do |
|---|---|---|
| the configuration is invalid | `SY-04.b` — "an invalid configuration leaves the working tree byte-identical", which is a claim about a **refusal** | declared `RConfigInvalid`, outside the closed set |
| this transition is deferred to the next iteration | `SY-04.a` — "at most one lifecycle transition per iteration" | modelled as a loop-control **guard**, not an outcome, so that half of `SY-04.a` is not falsifiable through the outcome vocabulary at all. **NARROWED by `system-k59`**: the review confirmed `mutant_many_transitions` genuinely exercises the counter and makes the invariant fail, and rejected the stronger reading that the catalogue therefore OWES a "deferred" refusal — `SY-04.a` says "at most one", not "refuse the second". What survives is the qualification, not a catalogue debt |
| the tree is blocked, and here is which block | `SY-14.b` — "every action on a blocked tree SHALL refuse **naming it**" | read `WitnessPending` as `RecoveryPending`'s refusal and `ReservedNameOccupied` as `OwnershipConflict`'s, which is the mapping `TT-24`'s three-context table makes for the same two artifacts — but the catalogue never states it for `SY-14.b` |

The third is the one to watch, because it is the one where two families can
diverge silently: a family that spelled the block refusal differently would
check a different claim and still report green. Affected: the `Refusal` set,
`SY-04.a`, `SY-04.b`, `SY-14.b`.
*M1* `quint-only` (pending replay; **contamination risk noted** — 041's title is
the same shape, though this finding pre-dates the grep) · *M2* `refusal` · *M3*
n/a — found by writing the enumeration out, not by a counterexample · *M4* `none`.

**3. `SY-14.b`'s quantifier reaches the lease, and taken literally it makes a
blocked tree unexitable — but `SY-14.a` was never the problem, and that half of
this finding is WITHDRAWN.**

**What `system-k59` found (F4).** The sweep behind this finding defined
"cleared" as *some admitted action returned `Applied`*. That is not `SY-14.a`,
whose subject is a block being CLEARED — a state transition. Under the corrected
measurement, which computes the SUCCESSOR's block through the same `outcomeOn`
classifier the real actions use, `release-lease` and `validate-config` succeed on
a blocked tree and change no block, so **neither is a counterexample to
`SY-14.a` and its narrowing is withdrawn**: the sweep for that half is now over
the LITERAL admitted set. The narrowing was hiding a faulty measurement rather
than answering a wide quantifier, and the same defect had been propping up
`mutant_block_clears`, which stopped firing the moment the measurement was
fixed. What remains of the finding is below, and it is about `SY-14.b` alone. "**No admitted action** SHALL clear a block, and
**every action** on a blocked tree SHALL refuse naming it." The admitted action
set includes `acquire-lease`, `layout-preflight` and `release-lease`. Sweeping
the literal set against a blocked tree, `release-lease` succeeds — as it must,
or the blocked driver can never let go of the working tree — and a session's
finish-leaf creation refuses `ReservedKind` rather than the block, so `SY-14.b`
fails. **This entry originally said `SY-14.a` failed there too. It does not**,
and that was the measurement defect: neither `release-lease` nor
`validate-config` changes the block, and `SY-14.a` forbids CLEARING one rather
than succeeding at all.

None of those is a defect in the protocol. What `SY-14.b` is, is a quantifier
stated one grain too wide: the claim is about actions **on the tree**, and the
catalogue's own `FN-26` — two operator-restorable exits from a block — is
incompatible with the literal reading, since an operator cannot restore anything
through a process that may not release its lease. The naming sweep is
`ADMITTED.filter(touchesTree)` and the narrowing is declared. Affected:
`SY-14.b`, and through it `SY-13`'s "terminal disposition". **`SY-14.a` is no
longer affected.**
*M1* `quint-only` (pending replay; **contamination risk noted** — post-dates the
grep) · *M2* `refusal` · *M3* 2 — the runner names the failing obligation and the
sweep names the action · *M4* `none`.

**4. The catalogue never says which lifecycle actions are gated on the root's
classification, and `SY-05.a` silently requires that `reap` is not.** This is
the finding that cost the most to reach, and the one whose failure mode is the
worst kind.

`reap` sits in the `Lifecycle` action group with the guard "lease, then epoch".
Nothing anywhere says whether it is refused on an absent, legacy or malformed
task root. Gating it on the root — which is the reading every other
tree-adjacent action takes — produces this: the driver runs the finish, proves
the deletion, the root becomes `Absent`, and the session that committed the
teardown ends. `reap` is then refused `RootAbsent`. The loop can never collect
that ending, never opens a new iteration, and **never scaffolds the fresh grove
that `SY-05.a` says a missing task root MEANS.**

The claim does not come out false. It comes out **unwitnessable** — the runner
reports `wit_SY_05a … never reached in 8000 samples`, which is the runner's
obligation 2 working exactly as designed, and which a suite without a witness
obligation would have reported as a green `SY-05.a` over a lifecycle that
physically cannot happen. The catalogue owes a sentence saying that `reap`,
`close-epoch` and `release-lease` read no tree. Affected: the `Actions` table,
`SY-05.a`, `SY-09`.
*M1* `quint-only` (pending replay; **contamination risk noted** — post-dates the
grep, and 043's title is shape-adjacent) · *M2* `ordering` · *M3* 3 — the
unlanded witness transcribes directly into a Rust test that drives a full
teardown and asserts the next invocation scaffolds · *M4* `none` in this phase;
the derived test belongs to `formal-synthesis-k16`'s seam work, and it is the
one finding here that **has** a pre-fix behaviour to write against if the
shipped driver shares the gate.

> **[disposed by `routing-and-prose-k73`]** The seam work is routed to `handoff-audit-k66`. The gate question itself is **disposed**: the catalogue's `Actions` table now states that only `launch` is gated on the root's classification, so the shipped driver does not share the gate and the derived test asserts the un-gated reading.

**5. `SY-03` says "no later gate" without saying what a gate is.** "No later
gate SHALL consult an earlier layout check; each SHALL revalidate against its
own operands." The catalogue never enumerates the gates. A session's ambient
operation revalidates the layout in this model because `SY-10.a`'s "before it
may touch the tree" implies that it must, not because `SY-03` says so — and a
family that read the gates as the driver's own three would check a materially
weaker claim and report the same green. Affected: `SY-02`, `SY-03`, `SY-10.a`.
*M1* `quint-only` (pending replay, clean) · *M2* `ordering` · *M3* 1 · *M4* `none`.

**Observation A — a deterministic loop satisfies its own claims by being
written down, and eighteen dials is what that costs.** The task-tree column
needed two model mutations and the finish column eleven. This one needs
**eighteen**, and the reason is structural: "the layout is proved at lease
acquisition", "validation precedes every transition", "at most one transition
per iteration", "selection is not recomputed before launching", "a stale
operation is refused before it touches the tree" are each true of `driverStep`
*because `driverStep` is written in that order*. Every one would carry a green
tick over no evidence at all. **The honest count is lower than eighteen**: four
obligations (`SY-07.a`, `SY-09.a`, `SY-09.b`, and `SY-14.b`) have no isolating
mutation and die only inside a bundle, and the README says so rather than
letting a reader read them as separately evidenced.

**REVISED [k59-F3].** The count is now twenty-three, and the qualification that
went with it is withdrawn — because `system-k59` checked it and found it was
false in the reader's favour AND against it at once. The entry said four
obligations (`SY-07.a`, `SY-09.a`, `SY-09.b`, `SY-14.b`) died only inside a
bundle. Targeted runs showed `mutant_no_signal_is_done` changed only the `k == 3`
arm of the reap classifier — the other two arms were constants — so
`inv_SY_09a` and `inv_SY_09b` stayed GREEN under it. **The bundle control was
not weak evidence; it was no evidence.** Four isolating mutations now exist, and
each asserts its neighbours green as runner commands, so "this target fails while
its neighbour holds" is a result rather than a claim. The observation itself
survives and is strengthened: a deterministic loop satisfies its own claims by
being written down, AND a reader cannot tell a bundle control from an isolating
one without running it.

**Observation B — `SY-13.a` is existential reachability, and an executable
model cannot search for it.** "There EXISTS a bounded sequence of admitted
actions reaching…" is not an invariant, and a simulator cannot answer it by
sampling. The instrument here is two halves that are worthless apart: a
**declared** escape map over the closed stable-class set, iterated exhaustively
and unrolled to the bound; and a **corroboration witness** that the model's own
transitions actually walked the longest declared chain, edge by edge. The map
alone is fiction with a green tick; the walk alone is one path. This is a
general shape for existential claims in an executable model and it is worth
carrying: `formal-synthesis-k16` should expect any `SHALL exist` claim to cost a
declared structure plus a corroboration obligation.

**Observation C — the search dial for this scope is not a budget, it is the
determinism of the subject.** Both sibling columns dial the search because their
subjects are a tree of unknown shape and a twenty-step transaction. A driver
loop is deterministic, and modelling it as a uniform choice over its whole
action vocabulary does not model a driver — it models a random walk that shares
the vocabulary. **CORRECTED [k59-F5], AND THE CORRECTION IS A THIRD OF THE ORIGINAL CLAIM.**
This entry originally reported *5 of 25 witnesses under a flat 27-disjunct menu
at 2,000 samples, against 23 of 25 under `driverStep`*. Neither figure survived
review. The flat variant reached no commit and could not be re-run — an
unreconstructable number cannot support the claim it was quoted for — and a
reconstruction disagreed with it by a factor of three. `scenario_flat_menu` is
that instrument now, committed, differing from `base` in the single constant
`FLAT_MENU`, and the comparison is over the twenty witness commands the library
carries:

| arrangement | 2,000 samples | 8,000 samples |
|---|---:|---:|
| flat menu | 12 of 20 | 17 of 20 |
| `driverStep` | 20 of 20 | 20 of 20 |

The observation stands and its magnitude does not. Arranging the menu as the
loop it models buys the whole witness set at a quarter of the samples; it does
not turn five into twenty-three. A flat menu reaches most shallow and mid-depth
witnesses given enough samples and misses the deep ones. Nothing was removed
from the model to get there, and `scenario_flat_menu` is the proof of that,
being the same model with the menu rearranged.

**The methodological lesson is the one that generalises**: a search-shape
comparison is an EXPERIMENT, and an experiment whose apparatus is not committed
is a number, not a result. The runner cannot carry it — a runner command is a
claim that a witness lands, and this measurement's content is how many do not —
so it lives as a committed instance plus a replay line in the README.

**M8 — false-confidence incidents, and the control arm.** Two incidents were
caught inside the producing session; **`system-k59` found four more that had
survived it, and that is the most useful number in this entry.** A column that
reports its own two incidents and ships four is exactly what the *vacuous
invariant* and *scope trap* hazards predict, and it is why an instrument review
was cut as a leaf rather than left to `cross-model-replay-k15`, which
re-derives findings and would not have re-derived any of these.

| incident, found by the REVIEW | what made it invisible | how long it stood |
|---|---|---|
| `mutant_no_signal_is_done` was documented as a bundle control over three endings and killed only one; `SY-09.a` and `SY-09.b` had NO control at all | the README asserted the bundle in prose, and the runner has no way to ask "does this mutation kill anything else?" | the whole producing session and its green run |
| the `SY-14.a` sweep read *some action returned `Applied`* as *a block was cleared*, and `mutant_block_clears` was firing THROUGH that defect | a broken instrument propping up its own control: both the claim and its control read the same wrong test | the whole producing session |
| the launched session was modelled as continuously holding the launch generation, which is what made `SY-11.b`'s cycle reachable — Grove has no such lock | the model was bent mid-build until the control fired, and a fired control reads as evidence | the whole producing session |
| the flat-menu comparison quoted 5 of 25 from an instrument that reached no commit; reconstruction gave 13–14 | a measurement with no committed apparatus cannot be contradicted by re-running it | the whole producing session |

**And one finding the review could not have reached by re-deriving.**
`driverStep` forced a launched finish session through teardown, so decline,
early failure, no signal and the methodology's reopening exit were all
unreachable — while the Rust loop accepts any of the three signal dispositions
after any launched session. **Every `SY-` property was checked over a world in
which a finish session cannot decline.** No obligation came out false; the
world was simply smaller than the claims, which is the shape of false confidence
this experiment exists to measure.

The producing session's own two incidents, both caught inside it:

| incident | how it was caught | how long it stood |
|---|---|---|
| the flat menu: 25 green properties over 5 landed witnesses | the runner's witness obligation, on the first suite run | one run |
| `inv_SY_13b` green over a sink test that could not fire, because `reap` counted as a tree-touching action and returned `Applied` from almost every state | fixing finding 4's gate made the test fire, and it immediately found real sinks | two runs |

The control arm — genuine failing checks, with the effort each took: the
unreached `wit_SY_05a` (finding 4, the session's longest single diagnosis); the
`SY-14` sweep failures (finding 3); a `SY-03` violation traced to an ambient
operation that skipped the layout gate (finding 5); a `SY-11.b` cycle reachable
in `base` because a caller could take the tree guard *after* joining the
generation queue; and `SY-07.a` failing on a **refused** finish-leaf creation,
because the flag read a record computed before the outcome was known. Four of
those five were model defects and one was a catalogue finding, which is itself
the shape of the ledger H10 is about.

**What a green run here does not prove.**

- Every `SY-` property in this column is established by **bounded randomized
  simulation**: 8000 samples, depth 24, two processes, one environment budget of
  three. `quint verify` completes at `--max-steps=4`, which does not reach a
  scaffold — so the model-checked result covers the lease, configuration and
  epoch gates and nothing past them.
- **Five witnesses land in fewer than a hundred traces of 8000** and two more
  needed a `scenario_` instance to land at all. They are deterministic under the
  fixed seed and therefore not flaky, but they have almost no margin, and the
  *scope trap* hazard says to record that rather than round it to green. The
  thinning has one cause: once a live session may end at any point, a
  twenty-one-move march to a proven finish is a coin flip per step. The counts
  are in the model README's table.
- **`SY-05.b` is an internal consistency check over the composition
  abstraction, not a composition result [k59-F6].** It says the summary never
  records absence before the deletion is proven; it does not say `FN-11` and
  `FN-19` compose in Grove, because one setter manufactures both operands. The
  command is named for what it checks.
- The narrowings are narrowings: `SY-13`'s sweep and `SY-14.b`'s quantifier are
  each checked over less than their literal text, and `SY-04.a`'s cap is
  checked over a counter rather than through the outcome vocabulary. Each gap is
  a finding above rather than a covered obligation. **`SY-14.a` is not narrowed**
  — that narrowing is withdrawn.
- Nothing here is evidence about the Alloy column, which neither this session,
  nor `system-k59`, nor `system-k60` read. The `(family, obligation)` matrix is
  what settles that, and `cross-model-replay-k15` is where the barrier comes
  down.

**What the review checked and did NOT break, recorded because a discharged
doubt is evidence too.** `system-k59` attacked five further conclusions and
each survived, which is what makes the six it broke worth acting on:

- **The `SY-13` catalogue contradiction (finding 1) is real.** The review
  re-read the claim's two sentences independently and agrees the definition of
  *terminal disposition* and its exact two-member enumeration disagree. This is
  now a finding two readings reached, not one.
- **`ENV_BUDGET = 3` hides no falsification.** The only two mutants configured
  with budget 4 — restart-repeat and guard-disorder — still violate their target
  invariants at budget 3 under the runner's samples, depth and seed. The current
  mutation set supplies no evidence of a `SY-` falsification hidden by the
  budget.
- **The two thin witnesses were real protocol paths**, not focused-scenario
  constructions: `SY-06.b` completes the normal partial scaffold and then meets
  a hand-edited legacy tree; `SY-07.a` reaches a spent tree, appends the
  sentinel, crashes, and reuses it after restart — hostile rather than ordinary,
  but genuinely `EN-08` and `EN-11`.
- **`driverStep` conditions on no history flag.** The suspicion that a search
  guard read `hist` and made a claim about history true by construction is
  stale against the committed artifact; `hist.envUsed` guards environment
  actions, which is the declared budget.
- **The depth-limited `verify` result is quoted with its depth**, and no `SY-`
  obligation is presented anywhere as model-checked. The shortest-path figures
  the review verified — eight driver moves to a completed scaffold, nineteen to
  a proven finish — were re-measured on the repaired model: **eight and
  twenty-one.** The two extra moves are exactly what putting the epoch
  invalidation and the signal interpretation in separate steps costs per reap,
  which is the order `src/loop_driver.rs` takes.

**One thing for `formal-synthesis-k16` that is about the METHOD rather than
about grove.** This column was reviewed as an INSTRUMENT — not "are the findings
right" but "does the search shape, the narrowing set and the control set support
what the column claims" — and the review found six real defects in a suite that
was green, coverage-asserted, and had already reported its own false-confidence
ledger. Four of the six were invisible to any run of the suite itself, because
the suite cannot ask whether a mutation kills anything besides its target,
whether a measurement measures its subject, or whether a quoted number can be
re-run. **The unit of that review is the pair (claim, the control that would
falsify it)**, and it is worth carrying into the synthesis as a general
obligation rather than as a fact about this scope.


### 047 — A contract with no domain words in it, and one interval no library can close (ordinal root lifecycle, an experiment rather than a column)

**Scope.** Neither a scope nor a column. `ordinal-root-lifecycle-k14` is a
`prototype` leaf whose deliverable is a *decision* — whether atomic root
lifecycle is a deep, domain-independent `ordinal-fs-tree` capability or stays
private to `grove-finish` — so this entry records an experiment, not a model
family. **Nothing here credits a catalogue obligation**, and the prototype's
claims are deliberately named `RL-` so that no reader and no runner can mistake
them for `TT-`/`FN-`/`SY-` coverage.

**The instrument is not in the repository, on purpose.** `models/run.sh`'s
obligation 2 fails the whole run on any `.als` or `.qnt` outside its four known
scope directories, so a prototype model committed anywhere would either break the
runner or have to be adopted into a scope — which is the production
implementation this leaf forbids. It was built in the session scratchpad; the
dials, the claims, the commands and both retained counterexamples are recorded
here and in the task file, and the successor leaf named at the foot rebuilds it
inside a scope if it is ever wanted.

**Situation.** `ordinal-fs-tree` today has no root creation and no removal at
all, and grove's `root-init` and `finish` both reach past it into `std::fs` and,
for the cleanup layer, into raw `openat`/`renameat2`/`unlinkat`. The root brief
says generic filesystem mechanics live in `ordinal-fs-tree` **by default** and
that any Grove-owned filesystem operation must be a *documented semantic
exception established by the formal work*. This is that establishment. The
library's own architecture already reasons about the case and then ships neither
verb: the advisory lock is taken on `<root>/..` rather than on the root
explicitly so that "the tree's creation and destruction fall under the same lock
as every ordinary operation", and that reasoning is called "general rather than
domain-specific".

**Formalism.** Quint 0.32.0 — one throwaway module, nine assumption dials,
eleven `RL-` invariants, six outcome witnesses, six instances. Quint rather than
Alloy because the question is *what does the protocol do when the caller's answer
moves and the process dies*, which is a guarded-action question; the structural
half (can the vocabulary be stated at all) is not a model question and was
answered by an enumeration instead. Simulation via `quint run`, and — decisively,
as it turned out — symbolic checking via `quint verify` (Apalache backend) at
`ITEMS = 1`, `--max-steps=14`, ~18 s per claim uncontended.

---

#### The vocabulary result, and why it was enumerated rather than swept

**Done when #1 is answered affirmatively.** Every identifier in the prototype's
*code* was extracted and classified — 175 non-keyword identifiers — and **none**
names a workstream, a task, a session, a version control system, a commit, a
branch, a bookmark, a handle, a ticket, a witness, a quarantine or a manifest.
Two match the bare word *kind* (`Kind`, `kind`) and mean "whether this item can
be fingerprinted"; they are classified, not swept away.

The enumeration is the point. A banned-word list is complete only as far as the
list, and the sibling nobody thought to list is exactly what leaks — so every
token was extracted first and classified second. **Two controls make the clean
read evidence rather than a broken instrument reading clean everywhere:**

| corpus | domain-loaded identifiers |
|---|---|
| the prototype's **code** — the contract | **0** (2 false positives on *kind*) |
| the prototype's **comments** — the citations back to the catalogue | 16 |
| `crates/grove-finish/models/finish.qnt`, code | **103** |
| `crates/grove-task-tree/models/task-tree.qnt`, code | **70** |

The translation is one-to-one and cost nothing: witness → *reservation*,
manifest → *ledger*, quarantine → *the root moved aside*, commit outcome →
*verdict*, finish handle → *identity*, task entry → *item*, `Committed` /
`NotCommitted` / `RecoveryPending` → `VApplied` / `VAborted` / `VUnknown`. That
last row is the whole of the caller's participation: a three-valued grade of an
effect the library never learns anything else about.

#### What was replayed, from both families

| replayed | from | how it lands in the prototype |
|---|---|---|
| `TT-20` the format witness lands last; an interrupted init is `PartialScaffold`, never `Current` | Alloy, entry 028 | `RL-1`; creation's identity token is published by one same-directory rename after the scaffold |
| entry 028's own repair — initialisation runs the **reserved** half of the cascade and not the **format** half | Alloy | the create direction has a `Partial` root state distinct from both `Absent` and `Valid` |
| `FN-19` restated as *one root identity is never in two places at once*, **over the transition relation and not over the disk** | Alloy, entry 042's retained counterexample | `RL-11`, written as *no step of the protocol produces* rather than *the disk never holds* |
| `FN-09.b` no preparing witness ever holds an evacuated entry | Alloy | `RL-2` |
| `FN-11` evacuation precedes deletion | Alloy | `RL-3`, as conservation |
| `FN-12.b` an undigestible entry type is refused **before any mutation** | Alloy | `RL-4`, with `mutant_opaque_late` as its control |
| `EN-01` is narrower than the protocol's own steps — evacuation is a **cross-directory** rename and is not granted atomicity | Quint, entry 045 finding 1 | `effectOf` separates `ESameDirRename` from `ECrossDirRename`; evacuation and restoration are the latter, which is what makes `RL-2` and `RL-3` non-trivial |
| the closed outcome set has no member for a transaction **never entered** | Quint, entry 045 finding 3 | `ORejected`, a distinct terminal outcome |
| `FN-22`'s four revalidation points, and the two `Committed` departures that are not the same event | Alloy, entry 035 | the four `DReval*` steps and `DReturn` |
| `FN-20` no artifact a transaction leaves behind is a receipt, stated over the **role** | Alloy | `RL-6` — and this is where it broke |

#### Retained counterexample 1 — the interval no library can close

`RL-6`, `quint verify`, `verify_monotonic`, 14 steps. The trace is eleven states
and the last three are the whole finding:

```
 8  step=DSettle           root=Held    stagePresent=true  moved=false
 9  step=DRevalAfterSettle root=Absent  stagePresent=true  moved=true   <- the settle rename
10  step=SDone (crash)     root=Absent  stagePresent=true  moved=true
```

Between the settle rename and disposal the container's root is **`Absent`** —
which no reader can distinguish from a container that was never created — while
the staging area still holds every item. `moved` and `stagePresent` are *model*
variables; a reader sees the root and nothing else.

Nothing inside the container closes it. By `FN-20`'s own role the leftover may
not be read as evidence that anything happened, and the library's entire world is
`<root>` and `<root>/..`, so it has nowhere to put a receipt that is not the
leftover. **This is `TODO.finish_process.md`'s "the interval is the whole
problem", arrived at from the library's side**, and the answer it forces is that
the receipt is irreducibly the *caller's*: Grove closes it with the correlation
ticket `FN-03` requires to live outside disposable state, and a library with no
notion of "outside" cannot own that.

`M1 quint-only` (Alloy did not reach it because no Alloy slice put the library's
own address space in scope). `M2 interruption`. `M3 = 3` — the trace names the
transition and transcribes directly. `M4 = none`, and the reason is the third
borderline in *Measures*: there is no defect to write a failing test against.
Grove already does the right thing; what changes is a **decision record**, which
survives both model families being deleted, so it is material.

#### Retained counterexample 2 — four revalidations are necessary and not sufficient

`RL-5`, `quint verify`, `verify_small`, 14 steps:

```
 9  DRevalAfterSettle  verdict=VApplied  truth=VApplied   <- last revalidation
11  DDispose           verdict=VApplied  truth=VAborted   <- the caller ungrades
12  SDone  outcome=OApplied              truth=VAborted   <- disposed anyway
```

After the last revalidation there is always a suffix in which the caller's grade
can move, and by then disposal has destroyed the ability to return. Adding one
dial — `VERDICT_MONOTONIC`, *once the caller grades an effect applied it never
ungrades it* — makes `RL-5` hold at every instance and **leaves counterexample 1
untouched**, which is what proves the two have separate causes rather than one.

The obligation is domain-independent to state and **impossible for the library to
check**. The catalogue carries it today only in lane-shaped form, as `FN-26`
— *history is never rewritten to clear a block* — which the Quint finish model
dials as `HISTORY_IMMUTABLE`; the dial is that family's switch and appears
nowhere in the catalogue, so a reader chasing this finding chases `FN-26`. `M1 quint-only`. `M2 eventuality`. `M3 = 3`. `M4 = none`.

#### The measurement that nearly did not happen: a control that killed nothing

The first full sweep had `mutant_no_revalidation` killing **no invariant at
all** — a control asserting nothing, which is the failure entry 046's review
named. Two causes, and both were in the claims rather than in the model:

- `RL-4` was written `(anyOpaque and REFUSE_OPAQUE_FIRST) implies …`, so it was
  **vacuously true in exactly the instance meant to falsify it**;
- `RL-7` carried a literal `or not(REVALIDATE)` escape clause — the control
  written into the check.

Restated over the contract, `mutant_opaque_late` kills `RL-4` alone and
`mutant_no_revalidation` kills `RL-5` and `RL-7` and nothing else. **A claim that
mentions a dial cannot be falsified by turning that dial**, and it reads exactly
like a claim that holds.

Revalidation only became measurable at all once the caller's *actual* grade
(`truth`) was separated from the library's last *observation* of it (`verdict`).
While the model resampled the verdict at each revalidation point, revalidation
was a coin toss rather than a reading, and no control over it could have meant
anything.

#### `M8` — a false-confidence incident, and it is about the runner rather than the model

**The same `quint run` command, at the same budget, returns `[ok]` on one run and
`[violation]` on the next, and nothing in the output distinguishes the two.**
Measured: ten identical invocations of a *control* that `quint verify` proves
violated —

```
quint run … --main=mutant_reentrant_caller --invariant=inv_RL_9_no_reentrancy \
            --max-samples=20000 --max-steps=30
   9 [violation]
   1 [ok]
```

At 1-in-10 per command, a sweep of ~55 commands is more likely than not to
contain at least one silent false green per full run. The mechanism is specific
and worth naming because it is invisible: this model's environment actions
(`doCrash`, `doForeignWrite`, `doReentrantOp`) are **absorbing** — each ends the
trace — and **enabled at almost every state**, so a uniform sampler reaches the
tail of an eleven-step protocol with probability about `(1/3)^11`. Adding one
further environment action (`doDrift`) part-way through the session silently
turned four previously-red controls green, with no signal of any kind.

Two consequences, and the second is the transferable one.

1. **A `quint`-simulated negative is not evidence here.** Every negative result in
   this entry is `quint verify`; simulation was used only for witnesses (a
   positive result, where a hit is a proof and a miss is merely a miss) and only
   in the `quiet` instance, which removes the absorbing actions so the tail is
   reachable at all.
2. **The enabled-everywhere-and-absorbing shape is a property a reader can spot
   before running anything**, and it is the same lesson entry 042 reached from the
   cost side — *price a slice by the program points a transition is enabled at* —
   arriving from the soundness side instead. It belongs in the synthesis as a
   general obligation on any simulated column, not as a fact about this
   prototype.

`M8`: one incident, stood roughly 40 minutes, caught by disagreement between the
two backends rather than by anything inside the suite.

#### The result matrix

`quint verify`, Apalache, `--max-steps=14`. `verify_monotonic` is the candidate
as finally proposed; the mutant rows are its controls.

| instance | RL-1 | RL-2 | RL-3 | RL-4 | RL-5 | RL-6 | RL-7 | RL-8 | RL-9 | RL-10 | RL-11 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `verify_monotonic` (the candidate) | ok | ok | ok | ok | ok | **RC1** | ok | ok | ok | ok | ok |
| `verify_small` (no caller obligation) | · | · | · | · | **RC2** | **RC1** | · | · | · | · | · |
| `mutant_identity_first` | **kill** | · | · | · | · | · | · | · | · | · | · |
| `verify_opaque_late` | · | · | · | **kill** | · | · | · | · | · | · | · |
| `mutant_no_revalidation` | · | · | · | · | **kill** | · | **kill** | · | · | · | · |
| `mutant_reentrant_caller` | · | · | · | · | · | · | · | · | **kill** | · | · |

All six outcome witnesses — applied, refused, blocked, rejected, created,
returned — were reached by simulation in the `quiet` instance, so no invariant
above is vacuous for want of its terminal state.

#### What a green run here does not prove

`ITEMS = 1` for every symbolic result, so nothing about multi-item interleaving
is established beyond what simulation at `ITEMS = 2` sampled. Depth 14, so a
protocol suffix longer than that is unexamined. One reservation, one staging
area, one attempt; no concurrent second caller; the disposal marker protocol, the
reaper and the three-lane commit seam are **deliberately omitted** as incumbent
mechanics rather than candidate contract. Fairness is not assumed anywhere and no
liveness property is claimed — every `RL-` claim is a safety claim.

#### Cost

`M5` — one session, roughly 2 h from bootstrap to decision, of which model
authoring is about 50 min and the rest is reading the two model families, the
catalogue and the seam ADR. Eleven claims. `M7` — `quint verify` at `ITEMS = 1`
runs ~18 s per claim uncontended and times out past 400 s at `ITEMS = 2` or when
two Apalache processes share the machine, which is what produced four spurious
`TIMEOUT` rows on the first pass and cost about 15 min of re-running serially.
`quint run` at 20 000 samples × 30 steps is ~0.6 s per claim and, per the `M8`
finding above, is not load-bearing for any negative here. Two Quint syntax costs
worth naming for a later reader: `match` cannot be applied to a primed variable,
and a `nondet` binding scopes over the single expression that follows it, so
every draw has to be wrapped in its own `all { … }`.

#### Counterfactual

**Prose would have reached the alternatives and not the interval.** The three
options and their trade-offs are argued from `entry-name-is-the-only-seam` and
the two architecture documents without running anything. What prose does not
produce is retained counterexample 1: the settle-to-disposal window reads as a
*small* gap in every prose account of the protocol — including
`TODO.finish_process.md`'s own, which names the interval and treats it as a
constraint to hold rather than as a boundary that decides ownership. Seeing a
reader classify that state as a **fresh container** is what turns it from a
constraint into an argument about where the operation can live.

**Alloy would have reached counterexample 1 and probably not counterexample 2.**
RC1 is a reachability question over a static shape and is exactly the kind of
thing entry 042's slice met from the other side; RC2 is about an environment
value moving *between* two observations, which is a temporal-operator question
Alloy 6 can pose but which the action-with-a-drift-step shape states in one line.

**The cheapest instrument in this entry is not a model.** The vocabulary
enumeration took about ten minutes, needed no tool, and answered the leaf's first
`Done when` outright — with a two-directional control that a model run would not
have supplied.

#### Verdict

Reach for a **throwaway executable model** again when the question is *where does
this operation live*, and the deciding evidence is a state the protocol produces
rather than a property it violates. Two conditions on doing it the same way
again: check every negative with the symbolic backend, because a simulated green
on an absorbing-environment model is not a result; and write every claim over the
contract, because a claim that names a dial cannot be killed by that dial.

**Decided:** [`root-lifecycle-stays-with-its-receipt`](adr/root-lifecycle-stays-with-its-receipt.md).
`formal-synthesis-k16` therefore inserts **no** ordinal root-lifecycle
implementation leaf before `extract-task-tree-k24`, and carries the narrowed
successor question that record states.

> **[disposed by `routing-and-prose-k73`]** The narrowed root-*creation* successor question is routed to `finish-verdicts-k65`; the general-form obligation the same record defers (*once the caller grades an effect applied it never ungrades it*) is routed separately to `finish-scope-k71`, because gaining it adds an `FN-` claim and a cell in both families.
>
> **[decided by `finish-scope-k71`]** The general form is **DECLINED**, and no obligation or cell was added. `FN-22`'s revalidation table carries two rows that are exactly the transition it forbids — after the quarantine rename, `Committed -> NotCommitted` and `Committed -> Indeterminate` — and the catalogue insists the two must not be collapsed, so an obligation forbidding the regrade would forbid both rows; granting it as an assumption instead deletes the states those rows need, which is entry 035's finding met a second time — *a check written stronger than its claim does not fail, it removes states*. The consequence is a **stronger** argument for the record's rejection than the one it carried: the coordinator's caller obligation is not merely unverifiable by the library, it is one this contract requires Grove to break. The record is edited in place, and `docs/specs/semantic-contract.md` carries the decline beside `FN-26`.



### 048 — What the two columns could not have found alone, and a catalogue the second column never saw (cross-model replay, all three scopes)

**Scope.** All three, and the model is neither component-local nor system-level:
the subject is the **pair**. Every result below is a property of two model
families read against each other, and none of them is visible from inside
either.

**Independence protocol — deliberately ended, which is this leaf's whole
licence.** The protocol was *neither model reads the other before both are
green*; both are green, so this session read both, exactly as
`quint-models-k10`'s brief section reserved. Nothing before it lifted, and this
entry names every place a prior tag has to change as a result.

**Situation.** Replay each column's unique evidence in the other formalism
rather than compare their prose; re-run the Alloy family under the runner
`task-tree-k56` repaired while it could not reach Alloy; and decide, for the
high-risk findings, whether a counterexample is abstract or product-relevant.

**Formalism.** Both, plus the shipped binary. Alloy 6 on Corretto
`21.0.12.1+9-LTS`, SAT4J, at each command's own bound. Quint read rather than
re-run, except where a claim about its encoding is made below. Product replay
against `target/debug/grove-llm` 19.3.0, built from this revision — a fixture,
not a test, because this phase changes no product behaviour.

**Caught.** Six material findings. Two are corrections to `task-tree.als` and
one of those is a false-confidence incident; three are about the experiment
rather than about either tool, and the last of those is the largest thing in
this entry.

---

**1. The two columns were not written from the same catalogue, and one
direction of the unique/overlap count is guaranteed by construction.**

The node brief says *"the Alloy column recorded, rather than fixed, every
catalogue finding it made."* It did not. `docs/specs/semantic-contract.md` was
**edited by four Alloy sessions** — `names-k33` (2026-08-24 23:10),
`guards-k37`, `ownership-k38` and `entry-k39` (2026-08-26 01:40) — and the first
Quint model landed at `task-tree-k11` on **2026-08-26 20:30**, after all four.
The `names-k33` diff is not a record: it rewrites *Entry* from "anything
directly or transitively beneath the task root" to "anything the walk
**reaches**", and it fixes `TT-01.b` to name `Malformed(MalformedEntry(entry))`.

Both are entry 026's findings, and **the Quint column read the corrected text**.
The same holds for entry 031's seven-preconditions/six-reasons mismatch, whose
correction `FN-05.a` now carries in the catalogue's own words. Those findings
are tagged `alloy-only`; the tag is true and it is uninformative, because Quint
was not in a position to find them — they had been removed from its search
space before it began. **M1's `alloy-only` count is therefore not a measurement
in the direction the hypothesis reads it**, and H4's *mutually discriminating*
test has to be computed over findings that were reachable by both columns or it
is scoring an artifact of commit order.

The reusable rule is sharper than the incident: **an independence barrier over
two models is not a barrier at all while their common ancestor is writable.**
The protocol named the models and forgot the catalogue, and freezing it later —
which is what the Quint sessions did, and recorded — closed the door after four
edits had gone through.
*Claim affected:* none — this is about the experiment. *Durable correction:* the
pre-registration's independence protocol, and every M1 tag on entries 026 – 033.
*M1* n/a (a finding about the measure) · *M2* `structure` · *M3* n/a · *M4*
`none`, and it falsifies **H7** by H7's own terms.

---

**2. `TT-20`'s fourth conjunct was a theorem of `rootState`'s own body, and it
is why the Alloy column could not see entry 044's counterexample.**

Entry 044's sharpest finding is that `PartialScaffold` is not robust to a
foreign write: the state is an EXACT closed subset, `EN-13` grants a foreign
entry at any name, and one stray file during an interrupted `root-init` drops
the tree out of the subset and through to `Legacy` — the classification `TT-20`
forbids by name. The Alloy column checked `TT-20` and reported green.

It reported green because its fourth conjunct read

```alloy
(no inFlight and no Slot.occ and no Fmt.fmt and isPartialScaffold) implies {
  rootState = PartialScaffoldR ... }
```

and `rootState`'s own body is `... else (no Fmt.fmt and isPartialScaffold)
implies PartialScaffoldR ...`. The conjunct is a **theorem of the definition**.
Checked with **no protocol premise at all** — no `GroveGrammar`, no
`SingleProc`, no transition relation — it is green at its own bound:

```sh
# scratch copy of task-tree.als, this command appended:
check TT20_c4_tautology_at_its_own_bound {
  always ((no Slot.occ and no Fmt.fmt and isPartialScaffold)
            implies (rootState = PartialScaffoldR
                     and rootState not in currentFamily
                     and rootState != LegacyR))
} for 4 but 4 Int, 3 FileObj, 2 DirObj, 6 Filename, 2 Slug, 2 Digest, 4 steps
# -> no counterexample
```

Entry 044's counterexample is a tree that **stops being** a partial scaffold, so
it never enters this antecedent. The check could not have failed, on any world,
at any bound.

**This is an M8 false-confidence incident and it stood for the life of the
file** — through five slices, a mutation matrix, an assumption table and a
retained-counterexample list, none of which could reach it, because every one of
those instruments asks whether a check *fails* and this check's antecedent was
never satisfied by the interesting state. The vacuity classes this corpus
already carries are *a bound too small to reach a state* and *a state reached
and empty*; this is a third — **an antecedent that excludes exactly the
counterexample the claim is about**, and the only thing that found it was a
second formalism having found the counterexample first.
*Claim affected:* `TT-20`. *Durable correction:* `task-tree.als` and this entry.
*M1* — the finding is `both` at the catalogue (entry 044 reached it) and
**replay-only** as an instrument defect · *M2* `structure` · *M3* **3** — the
tautology check names it in one command · *M4* `none`.

---

**3. `task-tree.als` excluded the world from an open transaction, and its own
comment said it should not.**

Replaying entry 044's counterexample found no instance, and the cause is not the
bound. `step` read

```alloy
some inFlight implies (noReport and procQuiet and (doInitPublish or doCrash))
```

— *"an OPEN transaction admits only its own next step or an interruption"* —
while `concStep`, two predicates above, says *"`EN-06` grants only that
COOPERATING processes are serialized, so `hand-edit` and `foreign-write` land at
any point during an operation and no guard excludes them."* Both are in the same
file. The second is the catalogue's (`EN-06`, `EN-13`); the first silently
made the world stop while grove scaffolds.

`doHandEdit`, `doForeignWrite` and `doIdle` compounded it by asserting
`no inFlight'` — so on the one path where a world action *could* fire during a
transaction, it would have **closed grove's transaction**, which is not
something the world can do.

Corrected: the two world actions frame `inFlight`, and `step` admits them while
one is open — `doForeignWrite` behind `Concurrent`, exactly as `step`'s other
branch already gates it, because it is the non-cooperating writer and the
single-process scope reaches the world through `doHandEdit`. The replay then
lands, and is retained as
`run witness_finding_a_world_write_during_an_open_scaffold_reaches_legacy`:
scaffold, one hand edit, interrupt, `rootState = LegacyR`, four states.

`TT-20`'s **third** conjunct then failed, and it failed for a reason this file
had written down three times about other claims: `some inFlight implies no
Fmt.fmt` — *while the transaction is open there is no witness to observe* — is a
claim about the WORLD, and only grove's half of it is true, because
`doHandEdit` leaves `Fmt.fmt'` deliberately unconstrained. **A claim about what
a protocol never does is never a claim about what the world never does**, met
for the fourth time and the first time inside a *claim* rather than inside a
model defect. It is now stated over grove's own applied step.

Conjunct 4 is restated over what an interruption LEAVES, which is a fact about
`doInitScaffold`'s effects rather than about `rootState`'s definition, and
narrowed the way entry 044 narrowed it — **an initialisation the world did not
touch** — reached independently and stated the same way. It now has a control:
a scaffold whose leaf carries a differing byte kills it, where the charter-less
mutation tried first did **not**, because the catalogue's subset is a *subset*
and a charter-less scaffold is legitimately inside it. That inert first attempt
is worth the line: entry 040's rule — *an unsatisfiable mutation reports exactly
as a survivor* — applies to a mutation that is merely **inert**, and telling the
two apart cost one run.
*Claim affected:* `TT-20`, `EN-06`, `EN-13`. *Durable correction:*
`task-tree.als` and its README. *M1* replay-only · *M2* `interruption` ·
*M3* **2** — the trace names the transition; the cause came from reading `step`
· *M4* `none` in this phase.

---

**4. Six findings this experiment counts as `quint-only` are `both`, and one
pair resolved the same conflict in opposite directions.**

The Quint entries tag their findings `quint-only (pending replay)` and say so
honestly. Read against the Alloy column, six are overlaps reached independently:

| the finding | Quint | Alloy | verdict |
|---|---|---|---|
| the closed refusal set has no member for a clean rollback | 045-2 `RRolledBack` | 033, `RefRollbackNotCommitted` | **`both`** — same hole, same repair, both declared as additions. **[disposed by `closed-set-additions-k74`]** the catalogue gained `DeletionNotCommitted`; two independent families inventing one missing member was the strongest single item on the whole disposition list |
| `FN-22`'s table and the blocked diagnoses disagree about `Indeterminate` | 045-5 | README ce. 15 | **`both`** — both resolve it the table's way |
| a blocked state falls through both diagnoses | 045-6 (post-persistent swap) | README ce. 14 (manifest names another handle) | **`both`** — different states, one defect: `FN-25.b` is false as literally worded |
| `SY-13` is false on `Legacy`/`Foreign`/`Malformed` sinks | 046-1 | 043 result 2 | **`both`**, and both chose the same repair — quantify over the admitted-reachable stable states |
| no refusal reason names an invalid configuration | 046-2 `RConfigInvalid` | 041, `RefConfigInvalid` | **`both`**. **[disposed by `closed-set-additions-k74`]** the catalogue gained `ConfigurationInvalid` |
| `release-lease` is special because it touches no tree | 046-3 | 043 result 3 | **`both`**, different claims cited (`SY-14.b` / `SY-04.b`), one catalogue gap |

Entry 045's disclosure asked that its finding 2 be treated "as an overlap
candidate with a known contamination risk rather than as a clean `quint-only`".
It is an overlap, and the contamination risk does not decide it: `RRolledBack`
and `ONotEntered` were in the model with their declaring comments before the
grep, which the commits carry, and the Alloy finding is at a different claim
(`FN-33`'s disposition mapping) than the title that was visible. Recorded as
`both` on the evidence, with the risk noted rather than resolved away.

**The pair worth reading is `FN-13`.** Both columns found that its stated
witness — *a commit attempted while the witness is tracked, **refused*** —
conflicts with *Outcomes*' rule that a caller who has already mutated is owed
`Blocked(OwnershipConflict)`. **They resolved it in opposite directions**: Alloy
followed the catalogue and refused, "because the catalogue is the sole input to
the formal phase"; Quint blocked, on the three-contexts rule. Two independent
readings of one document, both reasonable, incompatible. That is stronger
evidence that the catalogue is underdetermined than either column's own account
of it, and it is evidence only a replay can produce.
*Claim affected:* `FN-13`, `FN-25`, `FN-29`, `SY-13`, `SY-14.b`, `SY-04.b`, and
the closed refusal set. *Durable correction:* the M1 tags on entries 045 and
046, and `formal-synthesis-k16`'s disposition list. *M1* — this finding **is**
an M1 correction · *M2* `refusal` · *M3* n/a · *M4* `none`.

> **[disposed by `routing-and-prose-k73`]** The `reap` half is **disposed**: the catalogue's `Actions` table now states that only `launch` is gated on the root's classification, and that the other six Lifecycle actions read and write no task tree. Both families and `src/loop_driver.rs` already implement it, so the edit was manifest-neutral. `FN-13`'s refuse-or-block half is **not** disposed here and is `closed-set-additions-k74`'s.

> **[disposed by `closed-set-additions-k74`]** `FN-13` **blocks**, `RecoveryPending`; Quint was right and `finish.als` moved. But the finding above is sharper than the answer, and the sharper part is what the disposition kept: **neither column was reading a discriminator, because the catalogue never stated one.** Alloy's `treeSame` is a *step* frame condition; Quint's test is *action*-level. Both are correct readings of *fails closed*, and both stayed green — the failure a suite cannot report. §*Outcomes* now says `Refused` and `Blocked` are separated by what the **action** leaves, never by where it stopped, and `FN-29.b` checks it. The two other opposite-resolution items fall out of the same rule and fall out in **opposite directions**: `FN-10.b`'s discard refuses (nothing applied), a later step blocks. **The obligation was controlled against this very finding**: reverting `finish.als`'s `FN-13` to a refusal makes `FN_29b` produce a counterexample, so the rule that took a cross-model replay to find is now one a single family's run would catch. Record: `docs/adr/a-refusal-leaves-nothing-standing.md`.

---

**5. Where one column made the right choice silently, it learned nothing — and
that is a property of how the claim was stated, not of the tool.**

Three of Alloy's findings do not replay into Quint, and none of the three is a
Quint deficiency:

- **`SY-04.b` is false as worded (entry 042).** Alloy states byte-identity over
  the trace, so a `hand-edit` during an invalid configuration falsifies it and
  the claim's missing scoping is visible. Quint accumulates
  `refusalMutated` from **one operation's own before/after pair**
  (`isRefusal(o) and wPost.tree.bytes != wPre.tree.bytes`), so the world's write
  is never attributable to a refusal. Quint is **silently correct** and the
  catalogue's wording is never put under pressure.
- **`SY-10.b`'s visible stop has no member of the outcome set (entry 041).**
  Quint discharges the obligation with a history flag, `not(hist.silentPark)`,
  and so never has to write down what the stop *returns*.
- **`SY-01.b` rests on kernel lock release, which no `EN-` row grants (entry
  040).** The catalogue half replays and holds in both. The other half —
  *"two separate mutations are made unsatisfiable by the model's own facts"* —
  does **not** replay: Quint's `inv_fail_MUT_SY_01b_the_lease_survives_a_death`
  fires. What Alloy recorded as a property of the claim is a property of
  **Alloy's encoding of a lock** as a relation constrained by a `fact`.

The rule underneath all three: **a property stated over one action's own
before/after pair cannot discover that its claim was quantified too widely; a
property stated over the trace can.** That is the sharpest available form of
H5's prediction, and it is about statement shape rather than about temporal
operators versus guarded actions — which is a *correction* to H5's framing, not
a confirmation of it.
*M1* `alloy-only`, confirmed by replay rather than assumed · *M2* `structure` ·
*M3* n/a · *M4* `none`.

**THE OUTCOME, recorded in place by `quint-statement-shape-k61`, and the rule
above needed a second axis.**

The leaf swept all twenty-five `SY-` properties rather than the two named here,
classified each as stated over the trace, over one operation's pair, or over a
single state, and put the table in
`models/system/README.md`. **Twelve pair rows
survive the sweep and each is the right grain**: their subject genuinely is one
grove operation, and widening them would assert something about the operator
that no filesystem-hosted tool can promise — the same mistake `SY-04.b`'s
literal wording makes, in the other direction. So *pair* is not a defect; it is
a defect exactly when the claim's subject is wider than one operation.

**The rule as stated above misses `SY-10.b`, and that is a correction rather
than a caveat.** `SY-04.b` was stated at the wrong grain. `SY-10.b` was stated
at the **right** grain — its subject genuinely is one admission's return — and
discharged by the wrong thing: `not(hist.silentPark)` read
`parked = contended and not(GEN_TIMEOUT)`, which is **the dial spelled
backwards**. A property discharged by a restatement of the model's own
configuration is unfalsifiable at any grain, and no trace/pair classification
detects it. The sweep therefore carries two questions: *is the property stated
at the grain of its claim's subject*, and *is it discharged by an OBSERVATION or
by a restatement of a dial*. `SY-10.b` was the only failure of the second.

- **`SY-04.b` is restated over the trace and is now narrowed TWICE, each
  narrowing with a control that fires under `base`'s own constants with no dial
  moved.** The first is the replay this finding said was owed: a hand edit moves
  a byte under an invalid configuration and grove did nothing
  (`inv_fail_MUT_SY_04b_the_operator_writes_under_an_invalid_configuration`),
  which is entry 042's counterexample in Quint. **The second is new and is this
  column's own.** `outcomeOn` gates transitions on `d.configValidated` — the
  driver's RECORDED VERDICT — where the layout gate three lines below reads
  `w.layoutOk` live, so a `configChange` between the validation and the
  transition leaves the validation standing as a **licence** and the transition
  writes the tree under an invalid configuration. Eleven states, retained and
  reproducible; the catalogue has an obligation for exactly this shape —
  `SY-03`, *a preflight is never a licence* — and states it for the **layout
  only**. **Neither could have been reached by the pair form at any sample
  count**: the offending step in the second is APPLIED and `refusalMutated`
  looked only at refusals, and the offending step in the first is the
  operator's and `refusalMutated` looked only at grove's.
- **`SY-10.b` is restated over what the admission RETURNS**, quantified over
  every contended admission in the trace, and the model now has to name the
  stop. It names `RGenContended`, **declared as an addition to the closed
  refusal set** exactly as `RConfigInvalid` was, because entry 041 is right that
  the set has no member for a contention timeout and `REpochStale` is
  `SY-10.a`'s reason for a different fact. The isolating control is
  `mutant_borrowed_stop_reason`: the admission still stops, still stops visibly,
  still parks nobody, and the only thing wrong is the NAME — which the old flag
  could not have asked about, and under which `SY-10.a` stays green.
- **A second trace conjunct for `SY-10.b`, over the wait map, was tried and
  WITHDRAWN, and what it found is a third result.** It fails in `base`, and not
  for `SY-10.b`'s reason: `contendGen` is `SY-11.b`'s **apparatus**, building a
  generation wait on purpose so the cycle claim has an edge to be checked over.
  The model's `WGen` carries two senses — the park `SY-10.b` forbids and the
  graph edge `SY-11.b` needs — and no predicate over `waits` separates them,
  including the narrowing to an exclusive hold. **One obligation's instrument
  constructs the state another obligation forbids, and neither noticed, because
  `SY-10.b` was stated over its own admission.** Both probes are in the README
  with their exact commands.
- **`SY-01.b` needed nothing**, as this finding said. It is listed so the sweep
  is over all three.

`models/run.sh --scope lifecycle --family quint` is **green with coverage
asserted at this revision: 93 commands, 25 of 25 cells, exit 0, 4m27s wall**
(86 and 3m44s before the restatements; the seven added commands are the two
restated properties' controls and their asserted-green neighbours).

*Claim affected:* `SY-04.b`, `SY-10.b`, `SY-03`, `SY-11.b`, and the closed
refusal set. *Durable correction:* `models/system/lifecycle.qnt`,
`models/system/lifecycle-controls.qnt`, `models/system/README.md`, and this
entry. **Two catalogue findings are added to `formal-synthesis-k16`'s
disposition list** — whether `SY-04.b` owes `SY-03`'s licence prohibition for
the configuration, and whether the closed refusal set owes a contention member —
and **one is about the models rather than the catalogue**: the `SY-10.b` /
`SY-11.b` collision over `WGen`. *M1* — the `SY-04.b` licence finding is
`quint-only`, reached after the barrier came down and so **not** a measurement
of either column's independent reach · *M2* `structure` · *M3* **2** — the
retained trace names the transition and the gate is two lines of `outcomeOn` ·
*M4* — not assessed here; the model's configuration gate is an abstraction of a
product path this leaf did not open.

> **[disposed by `lifecycle-scope-k72`]** **`SY-04.b` OWES IT, AND THIS COLUMN'S GATE WAS THE DEFECT —
`M4` IS NOW ASSESSED AND IT IS *none*.** The product path this leaf did not open
is the one that settles it: `src/loop_driver.rs` is the sole caller of
`SessionConfig::load` and calls it TWICE an iteration, once before the tree
mutation and once before the launch, which
`docs/adr/complete-session-configuration.md` states as *validated in full —
before every tree mutation and again before every launch*. So no product defect
stands behind the finding; the licence was this model's, licensed by the
catalogue's silence. `lifecycle.als` read the configuration LIVE and was green on
the stronger claim throughout — **the two columns answered one silence in
opposite directions and both stayed green**, which is entry 048's `FN-13` shape
in an obligation no enumeration had flagged. `outcomeOn` now reads
`w.configValid` beside the recorded verdict and the retained counterexample is
`mutant_config_licence`, isolating and asserted so. Record:
`docs/adr/a-lifecycle-claim-says-what-it-is-over.md`.

---

**6. `reap` — the two columns resolved one silence in opposite directions, and
the product settles it.**

Entry 046's finding 4 is that the catalogue never says which lifecycle actions
are gated on the root's classification, and that gating `reap` makes `SY-05.a`
**unwitnessable**: the teardown commits, the root goes `Absent`, `reap` refuses
`RootAbsent`, and the loop never scaffolds the fresh grove a missing task root
is supposed to MEAN. Quint gates it — `reapOp` runs through `outcomeOn`, the
gate classifier — and the runner reported `wit_SY_05a … never reached`.

**Alloy's `doReap` reads no tree at all**: its guard is the lease, the role, the
launch generation and `some World.running`, and nothing else. So the two columns
read one silence and filled it in opposite directions, which is the strongest
evidence available that the silence is real — and neither column could have
produced that evidence alone. Alloy made the right choice and learned nothing;
Quint made the wrong one and its **witness obligation** turned the mistake into
the finding. That is the witness discipline paying for itself in the one shape
nobody predicted: *a witness that never lands is how a model reports its own
wrong guess about an underdetermined spec.*

Entry 046 left the product question open — "the one finding here that **has** a
pre-fix behaviour to write against **if the shipped driver shares the gate**".
**It does not.** `src/loop_driver.rs` reaps by polling `try_wait` and the signal
file, hands off through `complete_post_reap_epoch_handoff`, and branches on
`Relaunch` / `Done` / `None`; no root classification is read anywhere on that
path. So the catalogue gap is real, and there is **no product defect behind it**
— `M4 = none`, and it falsifies **H7** by H7's own terms, which entry 046
anticipated for every finding but this one.
*Claim affected:* the `Actions` table, `SY-05.a`, `SY-09`. *Durable correction:*
the catalogue. *M1* `both`, by opposite resolutions · *M2* `ordering` ·
*M3* **3** · *M4* `none`.

---

**The product replay, and the one place an abstract counterexample is already
shipped behaviour.**

Entry 044's finding (a) — *an ordinary operation meeting a `PartialScaffold`
root has no member of the closed refusal set, and `FormatLegacy` is the one
`TT-20` explicitly forbids* — is not abstract. Four fixtures, a real
`.grove/` scaffolded by `grove-llm root-init` and then cut back to each
interruption window:

| fixture | window | stray file | `grove-llm pick` reports |
|---|---|---|---|
| A | charter only | yes | `…/.grove/FORMAT; this is a legacy tree and must be migrated` |
| B | charter only | no | the same |
| C | charter + leaf | yes | the same |
| D | charter + leaf | no | the same |

**Fixture D is an exact partial scaffold and is reported `Legacy`.** So the
shipped ordinary read path does not implement `PartialScaffold` at all; only the
driver's own recovery does, in `tree_lifecycle.rs`, which additionally **refuses**
with *"ambiguous partial root scaffold … exact fresh-tree content is mixed with
unexpected entries"* — a fail-closed outcome the catalogue's state table has no
member for, and a better answer than either model gives.

Reading the two together bounds entry 044's finding rather than confirming it
whole. The recovery's `unexpected`-entry branch splits: with scaffold-owned
content present it **refuses**, and with none present it returns `Ok(false)` and
the root falls through to the legacy path. So the window in which a stray entry
sends an interrupted `root-init` to `Legacy` rather than to `PartialScaffold` is
the interval **after the charter and before the leaf** — which
`create_root_unlocked` and `complete_scaffold` leave unguarded on purpose
("between phase one and phase two nothing holds the tree"). Two shipped tests
corroborate both halves:
`extra_task_structure_makes_an_exact_partial_scaffold_ambiguous` asserts the
refusal and that nothing is mutated, and
`an_untouched_root_brief_does_not_hide_a_legacy_v2_tree` asserts the
fall-through, deliberately, for a charter-only root.

**What that does and does not establish.** It establishes that `TT-20`'s
prohibition — never `Legacy` — is false of shipped behaviour, in a window the
code creates on purpose. It does **not** establish harm: the legacy path refuses
or migrates and does not silently complete somebody's scaffold, and whether the
window wants a third classification, a guard across the two phases, or nothing
at all is `formal-synthesis-k16`'s to disposition. The counterexample is
product-relevant; the severity is not this leaf's to assign.
*M1* `quint-only`, product-confirmed · *M2* `refusal` · *M3* **3** · *M4* — the
fixture above, for `formal-synthesis-k16`'s seam work.

> **[disposed by `routing-and-prose-k73`]** Split between two owners.
> `TT-20`'s prohibition, and whether the window wants a third classification, a
> guard across the two phases, or nothing, is **`task-tree-scope-k70`**'s
> (items 13 and 20 of the node's disposition table); the derived-test/seam half
> of *M4* is **`handoff-audit-k66`**'s, with the other seam work.
>
> **[disposed by `task-tree-scope-k70`]** **A third classification**, and the
> fixture table above is what decided it: the catalogue had one state for a
> witnessless root where `recover_partial_root_init_unlocked` has three.
> `PartialScaffold` gained a class — `Exact` completes, `Ambiguous` refuses — and
> the closed refusal set gained `ScaffoldIncomplete(class)` to report it, one
> parameterised member on the `Reserved(class)` → `WitnessPending(class)`
> precedent. A guard across the two phases was **refused**: `EN-06` grants only
> that cooperating processes are serialized, and the actor here is `EN-13`'s
> non-cooperating writer, so a guard buys nothing against the writer that
> produces the counterexample. Treating the charter as proof was **refused** for
> a sharper reason — `an_untouched_root_brief_does_not_hide_a_legacy_v2_tree` is
> a deliberate shipped test, and its failure mode is worse than the one being
> fixed.
>
> **Severity, which this entry left explicitly unassigned: a diagnostic defect,
> not a safety one, and it gets worse rather than better.** Nothing is silently
> completed in the surviving window. But once the approved breaking change
> removes migration, `Legacy` fails closed and the operator is told to migrate a
> tree that is not legacy, by a command that no longer exists, about a directory
> Grove created and then failed to recognise. The repair that closes the window
> is one reordering — make root initialisation's first write a root-init-exclusive
> one — and `tree_format::write_current_last` already validates and reuses a
> pre-existing temporary, so the code anticipates it. Product-facing, therefore
> **`handoff-audit-k66`**'s, beside the other four diagnostic questions.

---

**The runner obligation entry 044 owed forward, discharged, and the answer is
negative.** `task-tree-k56` repaired the runner's reverse-coverage direction
under the barrier, could not run `--family alloy`, and named re-running it as
this leaf's first obligation. Every `check`/`run` in the three `.als` files was
resolved against the manifest the runner reads out of the catalogue:
**129 obligations, 0 commands naming an obligation the catalogue does not
define, 0 naming none, 0 claim-level citations.** The 29 remaining commands are
`EN_`-prefixed controls, one vacuity witness, and the `witness_finding_` this
session added, all legitimately exempt — re-run after the correction, so the
figure is this revision's. The
check binds Alloy and Alloy passes it — which is the result, null and worth
having, because the alternative reading was that a check written blind had been
written wrong.

**What was NOT replayed, and why that is a result rather than an omission.**
Rule 3 of the independence protocol says *"not attempted" is not one of the
outcomes*, so the largest class of unreplayed evidence is accounted for here
rather than left out.

`crates/grove-finish/models/README.md` retains **twenty-one** counterexamples
and says of them that "twenty of the twenty-one are about the model or the
catalogue rather than about the protocol". Of those twenty, **twelve are one
shape**: a claim about tree *shape* falsified by Alloy's free initial state —
`FN-09.b`'s preparing witness with something inside it, `FN-12.a`'s manifest
half-written by no step, `FN-22.d`'s hand-edited manifest, `FN-31.b`'s
hand-edited marker pair, and so on. The README states the rule they taught as
*a shape claim under a free initial state must be restated over the transition
relation*, and records meeting it at five separate grains.

**None of the twelve has a Quint analogue, and the reason is structural: an
executable model has no free initial state.** `finish.qnt` and `lifecycle.qnt`
each open with an `action init` that constructs one state; every later state is
reached by a transition. So the entire class of defect is **unreachable in
Quint by construction** — which is why Alloy's finish column retained twenty-one
counterexamples and Quint's retained few, and why that count is not a measure of
either tool's power. It measures how each family gets its non-protocol states:
Alloy posits them and must then fence off the ones its protocol cannot build,
Quint builds them and must then work to reach the rare ones. The costs those two
strategies pay show up in different columns of this experiment —
Alloy's as twelve retained counterexamples, Quint's as four
`scenario_` instances written for no reason but sampling — and reading either
column's total as a finding count double-counts a modelling strategy.

Recorded as **inexpressible in the other formalism, with the reason**, which is
rule 3's second branch, and it disposes of twelve of the twenty at once.

**Missed.**

- **The correction is verified over its exact blast radius, not by a full
  family run, and the distinction is worth stating precisely.** `step`'s
  widening is visible only to a command whose scope predicate does not pin
  `always no inFlight`. Two predicates do pin it — `CurrentRootThroughout` and
  `Guarding` — and between them they cover **83 of the file's 104 commands**,
  which therefore cannot see the change at all. The remaining **22 were each
  run, and all 22 pass**: `TT-17` – `TT-20`, `TT-24.a`/`TT-24.b`, their
  witnesses, the retained replay, and the seven `EN_08`/`EN_11`/`EN_14`
  controls — including both inverted forms, which still behave (the `EN-14`
  premise-break finds its counterexample; the six exercise-removals find none).
  That is a **complete** verification of the change rather than a sample, and it
  is a stronger claim than a suite run would have made about the same edit.
  What it is **not** is a green `models/run.sh --family alloy`: that costs hours
  at these bounds — the run started for it reached 14 commands in seven minutes
  — and a session is not a good unit for it. The finish and lifecycle `.als`
  files are byte-unchanged by this session.
- **No Quint model was corrected by THIS session, and one needed to be.**
  Finding 5 says Quint's `SY-04.b` and `SY-10.b` are stated in shapes that
  cannot fail the way Alloy's did. Restating them is a real change to a green
  column and belongs to a session that can re-run it. **`quint-statement-shape-k61`
  was cut for it and has done it**; the outcome is recorded inside finding 5
  above, and it found a counterexample of its own on the first use of the rule.
  `formal-synthesis-k16` inherits the two catalogue findings it produced, not
  the restatement.
- **`TT-24.c` is a filled cell with no falsifier.** The Alloy column declared
  `TT-24.c` and `TT-24.d` out-of-bounds; Quint answered both. Reading Quint's,
  `inv_TT_24c` is a transcription of `gateOutcome`'s own branch
  (`isTransaction(t)` → `BlockedO(OwnershipConflict)`), and **no control kills
  it** — the task-tree controls file mutates `TT-20`, `TT-05`, `TT-12`, `TT-04`,
  `TT-24.d`, `TT-21.a` and `TT-23.b`, and not this. So the coverage matrix
  scores a transcription above an honest declaration, and **the instrument
  rewards the cheaper move.** Whether the fix is a control, a gap, or restating
  `TT-24.c`/`TT-24.d` as `FN-` obligations is `formal-synthesis-k16`'s.
- **Entry 044's `TT-23` finding is an overlap the Alloy column did not write
  down.** `TT_23b`'s own comment says an already-terminal entry "is skipped
  silently, which is what makes the re-run idempotent" — which is exactly the
  requirement entry 044 says is "implied by two claims and stated by neither".
  Alloy made the choice and left it in a comment; Quint noticed it was unstated
  and recorded it with a mutation. Under the material-finding rule's clause 3 —
  both operands recorded at discovery time — only Quint's version scores, and
  the discovery is the same one. **An unrecorded modelling choice and a recorded
  catalogue finding are the same event, and only one of them counts.**

**Cost.** One session, and the shape of the cost is the interesting part.
**The replay itself is reading**: findings 1, 4, 5 and 6 are two records or two
model files held side by side, and no solver was involved in any of them. The
solver's whole contribution was to make three readings falsifiable — the
tautology check, the widened-world probe, and the mutation control — and those
cost roughly fifteen minutes of Alloy between them, plus about ten for the
blast-radius verification. What was **not** cheap, and is the transferable
number: establishing that the first probe's *no instance* was a modelling
exclusion rather than a bound took three narrowing probes, and the inert
mutation under finding 3 cost a run to tell from a survivor. Both are the same
tax — **a negative result from a bounded tool is ambiguous until you spend a run
disambiguating it**, and a replay session pays that tax on every finding it
carries across. The product fixtures cost minutes and settled two questions the
models could not.
**Ninety per cent of this leaf's value came from reading, and the solver's whole
contribution was to make three of those readings falsifiable.**

**Counterfactual.** Findings 1, 4 and 6 are unreachable from inside either
column by construction — they are statements about a pair. Findings 2 and 3
needed one column's counterexample carried to the other; a careful adversarial
read of `task-tree.als` alone would plausibly have found the tautology, and
`review-prototype-task-tree-k55` is where that read happened for the Quint
column and no equivalent ran for Alloy, which is the asymmetry finding 1 is
about wearing different clothes. Finding 5 needed both files open at once.

**Verdict.** **Replay is not a comparison step; it is an instrument, and it
found things neither column's own reviewer could.** Its unit is not the finding
but the pair *(what one column concluded, what the other concluded about the
same sentence)*, and its two productive shapes are **opposite resolutions of one
silence** (`FN-13`, `reap`) and **a check that cannot fail in the direction the
other column's counterexample points** (`TT-20`). Both are invisible to a
mutation matrix, an assumption table and a witness obligation alike, because all
three ask a single model about itself. Schedule replay before synthesis, not as
a formality after it — and freeze the shared ancestor before the first model,
not after the fourth edit.

**What a green run here does not prove.** Nothing here re-checks either column.
The Alloy corrections are verified by the individual commands cited and by the
family run recorded in the model README, not by a full suite inside this entry.
The M1 reclassifications in finding 4 are read off the entries and the model
files; they are as good as those records and no better. The product replay is
four fixtures and a reading of two functions — it establishes that the window
exists, not that nothing else does. And finding 1 bounds every count in this
experiment: the unique/overlap figures are asymmetric by construction in the
Alloy direction, and no re-tagging in this entry repairs the four catalogue
edits that made them so.

**Derived tests.**

| finding | test |
|---|---|
| the `PartialScaffold` window | scaffold a root, interrupt **after the charter and before the leaf**, drop one foreign file, and assert the root is not classified `Legacy`. Red today. |
| the ambiguity refusal | **already covered** by `extra_task_structure_makes_an_exact_partial_scaffold_ambiguous`. Nothing to write; what is owed is a *claim*, since this is shipped behaviour the catalogue's state table has no member for. |

The other five record `M4 = none` and each falsifies **H7** by the terms H7 sets
for itself — which now holds for every material finding in the finish and
lifecycle scopes and for all but one in the task tree.

# Experiment 2 — pre-registration

**Written before any Experiment 2 model exists.** That is the whole value of the
section: a comparison decided after the results are in is a story, and this log's
standing complaint is that "Quint was helpful here" is worth nothing. Everything
below — hypotheses, counting rules, controls, and what would *falsify* each
prediction — is fixed now, at jj change `uwuvxpkowmpumtukrknzxqptvpklmlwp`
(commit `149994afa5f3a8d6fd04178b60754e085dfb960f`), 2026-08-24, before
`design-model-contract-k5` writes the claim catalogue.

Registered by `experiment-baseline-k4`. Nothing in Experiment 1 is revised;
entries 001–025, H1–H3 and the distillation stand as written.

**Revised once, before any Experiment 2 model existed.**
`experiment-baseline-k29` reviewed this section against the artifacts and
`experiment-baseline-k30` applied the result: the *material finding* rule, the
falsifiers of H4, H5, H6, H8, H9 and H10, and the M3, M5, M6, M7 and M8
counting rules were corrected because they did not decide their own outcomes.
The revision is legitimate **only** because of where it sits — the review was
inserted *ahead of* `design-model-contract-k5`, so no claim catalogue, no model
and no result existed to tune the measures towards; there was nothing to see.
That window closes when `design-model-contract-k5` becomes selectable.
**As corrected, this pre-registration is fit to bind Experiment 2**, and it is
now fixed: any later change to a hypothesis, a falsifier or a counting rule is
a post-hoc amendment and must be recorded as one, in place, naming what was
already known when it was made.

## What is being compared, and what is not

The subject is grove's own modular redesign: **task-tree semantics**, the
**finish/recovery protocol** described by `TODO.finish_process.md`, and the
**end-to-end lifecycle** joining session completion, tree exhaustion, finish,
interruption and recovery. Alloy 6 and Quint each cover all three.

This is a different subject from Experiment 1 in three ways that bound what the
two experiments can be pooled into, and they are stated now so a later synthesis
does not pool them anyway:

1. **The system is stateful across process deaths.** Experiment 1's subject was
   a tree algebra and its filesystem application. Experiment 2's includes crash
   recovery, a VCS boundary, and three transactions nested inside one another.
2. **The implementation already exists and is green.** Experiment 1 modelled
   ahead of code. Here 1,210 tests pass over 31,423 lines of `src/` before a
   line of model is written, so H2 ("the model leads profitably") **cannot** be
   retested — the counterfactual is unavailable. Any finding is a finding
   against *shipped* behaviour, which is a different and generally harder bar.
3. **Alloy 6 is required to be temporal.** Experiment 1 used Alloy for static
   relational structure and Quint for behaviour, which is precisely what H1
   predicted. Experiment 2 deliberately puts both tools on behavioural questions,
   so H1 is not being re-confirmed here — it is being *stressed*.

Baseline facts about the system under study — versions, pre-existing suite
state, the observable contract, and the repository's shape at this revision —
are **not** repeated here. They live in
[`docs/preservation-baseline.md`](preservation-baseline.md), which is the
contract this experiment must not perturb; §1 and §2 there are the versions and
green-suite record this pre-registration would otherwise have to carry.

## Independence protocol

The comparison is worthless if the second model is written by reading the first.
Three rules, fixed now — and a fourth, added at entry 048 because the first
three named the models and forgot the document they descend from:

1. **The claim catalogue is tool-neutral.** `design-model-contract-k5` defines
   state, actions, observations, environment assumptions, stable and transient
   states, refusal outcomes and recovery obligations in prose that encodes
   neither tool's syntax. Both models are written from it.
2. **Neither model reads the other before both are green.** Replay is a
   deliberate later step, not a running cross-check. A finding carried across
   during construction is recorded as such and excluded from the unique/overlap
   count.
3. **Replay is one-directional per finding and recorded either way.** Every
   unique counterexample is replayed in the other formalism, or recorded as
   inexpressible **with the reason**. "Not attempted" is not one of the outcomes.

4. **The shared ancestor is frozen before the first model, not after the
   fourth edit.** Added at entry 048, which found the rule missing. Rules 1–3
   name the *models* and say nothing about the document both are written from,
   and `docs/specs/semantic-contract.md` was edited by four Alloy sessions
   before the first Quint model existed — including a rewrite of *Entry* and a
   fix to `TT-01.b`, which are two of entry 026's three findings. The Quint
   column therefore read a corrected catalogue and could not rediscover them, so
   their `alloy-only` tags are true and uninformative. **An independence barrier
   over two models is not a barrier while their common ancestor is writable.**
   A catalogue finding made before the freeze is recorded and NOT fixed, exactly
   as the barrier already requires of a model finding; entries 026 – 033 predate
   this rule and every M1 tag on them is qualified by it.

The rules are recorded as *intended*, and each entry says whether they held.
Entry 017's lesson is the reason for that last clause: an arm's isolation
conditions are the first thing a later reader doubts.

## Hypotheses

Each carries the observation that would **falsify** it, because a prediction
with no falsifier is a description.

- **H4 — the blind spots differ.** A scope is **mutually discriminating** when,
  within that scope, Alloy 6 holds at least one `alloy-only` material finding
  *and* Quint holds at least one `quint-only` material finding — M1 tags,
  assigned before replay. H4 predicts **at least two of the three scopes are
  mutually discriminating**. *Falsified if* **fewer than two** are. That is the
  exact complement, and it subsumes what the first draft of this falsifier left
  unsorted: subset-in-every-scope, empty symmetric difference, and the case of
  exactly one discriminating scope all land as falsification.

- **H5 — temporal Alloy and executable Quint fail differently on the same
  question.** Quint's guarded actions and random simulation will reach
  *interruption-and-recovery ordering* defects that Alloy 6's bounded temporal
  traces do not; Alloy 6's temporal operators will reach *eventuality* claims
  ("recovery always terminates in one of the two operator-restorable exits")
  that bounded random simulation cannot establish. Those are **two directional
  predictions**, so both must hold. *Falsified if* **either** of them fails:
  if no `quint-only` finding carries the M2 class `interruption` or `ordering`,
  or if no `alloy-only` finding carries the M2 class `eventuality`, or if either
  class lands in the opposite formalism. "Neither tool reached a class the other
  missed" is one way to fail this, not the definition of failing it.

- **H6 — counterexample usefulness is a property of the tool, not the finding.**
  A counterexample's cost-to-act-on is more strongly determined by which tool
  produced it than by which defect it exposes. *Spread* and *between* need a
  statistic, and it is fixed here rather than after the scores exist. The
  population is the **paired** findings: those tagged `both` under M1, each of
  which therefore carries **two** M3 scores, one per formalism (M3 is recorded
  per `(finding, formalism)`, not once per finding). Let `d_i = M3_alloy(i) −
  M3_quint(i)` over the paired findings.
  - **between-tool effect** = `|median(d_i)|`;
  - **within-tool spread** = `max(MAD_alloy, MAD_quint)`, each the median
    absolute deviation of that tool's M3 scores across the same paired findings.

  *Supported if* the between-tool effect **strictly exceeds** the within-tool
  spread. *Falsified if* it is **strictly less**. An exact tie, or **fewer than
  five paired findings**, records H6 **inconclusive** — declared now, so that a
  thin result cannot be rescued by choosing a different statistic afterwards.

- **H7 — findings convert to tests.** Every material finding yields at least one
  executable Rust test that fails against the pre-fix implementation. *Falsified
  by any finding that cannot* — and such a finding is more interesting than the
  ones that can, so it is recorded prominently rather than dropped.

- **H8 — placement is a real trade-off, not a preference.** Component-local
  models (beside `grove-task-tree`, beside `grove-finish`) will be cheaper
  **per checked claim**; the defects that actually motivate the redesign will
  appear only in the system-level lifecycle model. Cheapness is measured as
  **authoring hours ÷ checked claims** — the operand the hypothesis names —
  aggregated over the component-local model files and over `models/system/`
  separately. *Falsified if* the system model produces no material finding the
  component models did not, **or** if component-local hours-per-checked-claim is
  not strictly lower than the system model's. Findings-per-hour is retained
  under M5 as a descriptive figure and is **not** an operand of this test.

- **H9 — synchronization is the dominant hidden cost.** *Total modelling
  effort* is fixed here as the denominator: **M5 authoring hours + M6
  synchronization hours + M7 tool-wrangling hours**, summed over every model
  file in both families. H9 predicts that M6 **exceeds the authoring hours of
  each family taken separately** and is **at least 30%** of total modelling
  effort. *Falsified if* M6 is below 30% of that total, **or** if M6 does not
  exceed `max(authoring_alloy, authoring_quint)`, **or** if tool-wrangling
  hours reach **twice** M6 — the "dwarfed by M7" clause, given a number so that
  it is decidable from the logged figures alone.

- **H10 — a green run's silence is the main hazard.** More Experiment 2 hours
  will be lost to a check that passed for the wrong reason than to a check that
  failed. Both operands are recorded by M8: the **summed standing hours of
  false-confidence incidents** and the **summed hours spent on genuine failing
  checks** — the second was missing from the first draft, which left the
  comparison one-sided. *Falsified if* the false-confidence sum is **less than
  or equal to** the genuine-failure sum. An incident with no recorded duration
  is counted as zero rather than dropped.

H4, H5 and H8 are the load-bearing ones: the brief keeps **both** model families
regardless of the result, so no hypothesis here decides whether a tool is used.
They decide what the eventual routing advice says.

## Measures

Counting rules fixed now, because the tempting time to define "material finding"
is after seeing which tool produced more of them.

**A *material finding*** is defined by the **source** of the defect and the
**durable consequence** of correcting it, not by a judgement about what a reader
would act on. "A reader would act on it" was the first attempt and it does not
sort the borderlines below, because acting on a bounds tweak is still acting.
The operational rule, in three clauses:

1. A finding is material **only when** its correction lands in the tool-neutral
   claim catalogue, in the requirements or design, in shipped behaviour, in
   durable documentation, or in a Rust test — and **survives outside** model
   syntax, runner settings, and bounds. The test is whether the correction is
   still there when both model families are deleted.
2. A model-only syntax or transcription error, and any bounds or trace-limit
   change, is **M7 tooling cost** — *unless* it also invalidates a recorded
   tool-neutral claim. In that case the **invalidated claim is the material
   finding** and the bounds change is its evidence, recorded as both.
3. Both operands are recorded **at discovery time**, before M1/M3 scoring: the
   affected tool-neutral claim, and the durable correction. **A classification
   carrying neither is not countable as a material finding** — it is an
   observation, and it goes in the entry prose.

The three borderlines the review of this pre-registration raised are resolved
here rather than left to the session that meets them:

| borderline | result |
|---|---|
| a bound that turned out too small | **M7.** Nothing outside the runner changes. If that bound previously produced a **believed green**, it is *also* an **M8** false-confidence incident. It becomes material only via clause 2 — when a tool-neutral claim was recorded on the strength of the too-small run. |
| a predicate that was ambiguous until a model forced a choice | **M7**, unless the ambiguity **originated in** the tool-neutral catalogue or design and the choice **forces a correction there**. Ambiguity resolved only inside one model's syntax is that model's cost. |
| shipped behaviour the model says is fine, but a catalogue required to describe it never described it | **Material.** The catalogue must change, and that change survives both models. This case may legitimately record **`M4 = none`** — there is no defect to write a failing test against — and each such finding **falsifies H7** by the terms H7 sets for itself. |


| # | measure | how it is counted | which brief item |
|---|---|---|---|
| **M1** | unique vs overlapping findings | each material finding is tagged `alloy-only`, `quint-only`, or `both`; `both` requires that each model reached it *before* replay | unique vs overlapping |
| **M2** | Alloy 6 temporal vs Quint actions | per scope, each finding is tagged with the class it belongs to — `ordering`, `interruption`, `eventuality`, `structure`, `refusal` — and the tool that reached it first | temporal vs action modelling |
| **M3** | counterexample usefulness | a 0–3 ordinal recorded at the moment of reading, before any fix: 0 = tool says "violated", no usable trace; 1 = trace readable, defect inferable with effort; 2 = trace names the transition; 3 = trace transcribes directly into a failing test. Recorded per **`(finding, formalism)`**, so a finding both tools reached independently carries **two** scores — the paired population H6 needs | counterexample quality |
| **M4** | derived tests | per finding: the test file and name added, or `none` plus why. The pre-fix red/post-fix green pair is the evidence | test derivation |
| **M5** | placement | per model file: which crate owns it, **checked claims**, **authoring hours**, findings produced, and wall-clock to green. The H8 comparison is **authoring hours ÷ checked claims**, aggregated over component-local files and over `models/system/` separately; findings-per-hour is retained as a descriptive figure and is not an H8 operand | component-local vs system-level |
| **M6** | synchronization burden | wall-clock spent making one model agree with the other after a change to either, logged as its own line item and never folded into authoring time. It is the numerator of H9's share; the denominator is M5 authoring + M6 + M7 wrangling, summed over both families | synchronization burden |
| **M7** | state-space and tooling cost | per model: bounds or trace limits, solver/backend, wall-clock and CPU-seconds of the full run, plus every hour lost to the tool rather than to the problem — **logged as `wrangling` hours, distinct from run wall-clock**, since H9 compares the former. Model-only syntax and transcription errors and bounds/trace-limit changes land here by clause 2 of the material-finding rule | state-space / tooling cost |
| **M8** | false-confidence incidents, **and their control arm** | each occasion a check was believed to pass while proving nothing, with how it was caught and **how many hours it stood** — *and*, on the same ledger, **each genuine failing check with the hours spent on it**. H10 compares the two sums, so recording only the first half makes it untestable. A duration left unrecorded counts as zero | false-confidence hazards |

M7 has a baseline to beat and a warning attached. Experiment 1's suites at this
revision cost **17s wall for Alloy** (20 commands) and **3m 29s wall / 2527s CPU
for Quint** (148 claims, one instance at six times the sample budget for a
witness landing in ~0.07% of traces). Experiment 2's subject has recovery states
and a VCS boundary; if its Quint run does not cost materially more than that, the
suspicion is that it is not modelling the interesting states.

## Named false-confidence hazards, and the control for each

Pre-registered so that finding one later is a confirmed prediction rather than an
excuse. Each is a way this experiment could report a green result that means
nothing.

| hazard | why it is live | control |
|---|---|---|
| **The dead tool** — a run that never started reports what a run that found nothing reports | live on the measurement machine *right now*: `java` on `PATH` is Corretto 16, below Alloy 6's floor, and only `run-alloy.sh`'s own JDK probe makes the suite green (`preservation-baseline.md` §1) | the repository runner aborts on launch-failure output rather than recording it as a result, and the suite is required to fail when a tool is removed |
| **The absent suite** — a model that is not run at all | entry 003's failure mode, and it has cost this workstream three times | one runner executes every model; a model file no runner reaches is a runner defect, asserted as such |
| **The vacuous invariant** — a property that holds because its antecedent is never reached | a bounded random simulation reaching an interruption state 0.07% of the time is one witness away from reaching it never | every invariant has a paired witness proving its situation is reachable; an invariant with no reachable witness is not a claim |
| **The unfalsifiable green** — no mutation control | Experiment 1's repeated lesson (entries 006, 008, 011) | one mutation per reported obligation: break the mechanism, watch the specific claim fail, restore |
| **Agreement mistaken for proof** — the two models agree because they share an error inherited from one catalogue | the catalogue is the single common ancestor of both models, by design | the catalogue's own assumptions are enumerated as a list, and each is mutated in at least one model |
| **The scope trap** — a bound too small to reach the defect | recovery needs at least two interleaved failures | every `check` records the bound at which it was run *and* the bound at which its witness first appears |
| **The idealisation** — a detail omitted from the model is where the bug lives | the finish protocol's real hazards are `renameat2`, symlinks and partial `jj commit` | each model's deliberate omissions are listed as refusal classes, and each listed omission is checked against the shipped code by hand |

## Recording protocol

Experiment 2 entries continue the same numbering (**026** onward) in the
`## Entries` section above, and keep the six fields — Situation, Formalism,
Caught / missed, Cost, Counterfactual, Verdict — that
*What makes an entry useful* defines. The node brief requires exactly those, so
nothing changes there.

Four additions, and they are additions rather than replacements:

1. Every entry states which **scope** it belongs to (task-tree, finish/recovery,
   lifecycle) and whether the model is component-local or system-level.
2. Every material finding carries its **M1 tag, M2 class and M3 score**, recorded
   before the fix.
3. Every entry records whether the **independence protocol held**, and names any
   breach.
4. Every entry names the model's **tool version, bounds or trace limits,
   solver/backend, fairness assumptions, abstractions, deliberate omissions, and
   what a green run does not prove** — the node brief's requirement, restated
   here as an entry field so it cannot be satisfied only in the model header.

**Misses are recorded with the same care as hits.** A log that contains only wins
produces a skill that only knows how to succeed, and Experiment 1's most-cited
result (H3, at entry 017) is a hypothesis that was *not* supported.

## What this pre-registration deliberately does not promise

- **It does not decide whether either formalism is worth it.** Both model
  families are required by the node brief regardless of the outcome. This section
  fixes what gets measured, not what gets used.
- **It cannot re-test H2.** The implementation is already written and green; the
  model-leads-implementation counterfactual is gone for this subject.
- **It is not a plan.** Which models exist, in which order, is
  `formal-modeling-k1`'s decomposition, not this section's.
- **It has no control arm.** There is no prose-only arm as at entry 017, because
  the shipped implementation *is* the prose-driven arm and its defects are
  already the thing being hunted. Any claim that a model beat prose here is
  confounded by the whole v1.0.0 → v19.3.0 history of this code being read,
  reviewed and revised; say so rather than implying a comparison was run.

# Experiment 2 — synthesis

**This section answers the pre-registration above, clause by clause, and nothing
else.** Every hypothesis is decided by the falsifier it fixed for itself before
any model existed; every measure is aggregated from the entries that recorded it
and reported as unavailable where they did not. Written by
`experiment-synthesis-k62`, the first child of `formal-synthesis-k16`, which was
decomposed because the phase gate is more than one session's work.

**It is a synthesis, not an amendment.** The pre-registration is closed. Nothing
below changes a hypothesis, a falsifier or a counting rule; where a rule turned
out not to decide what it was meant to decide, that is reported as a **result
about the instrument** and the rule is still applied as written.

**The conclusions are local to this experiment.** The subject is one repository,
one pair of tools at two pinned versions, and one team of sessions working under
one methodology. Nothing here is a claim about Quint, Alloy 6 or formal methods
in general, and the places where a general-sounding sentence is warranted are
marked as *transferable* explicitly.

## The runs this synthesis stands on

Re-run at this revision, from the repository root, by this session. **The
provenance column is not decoration**: a synthesis that quotes a producing
session's run line and calls it a re-run has measured nothing.

| cell | commands | coverage | result | provenance |
|---|---|---|---|---|
| `--scope task-tree --family quint` | 111 | 43 complete, 0 declared gaps, 0 empty, of 43 | exit 0 | this session |
| `--scope finish --family alloy` | 180 | 61 complete, 0 gaps, 0 empty, of 61; Q4 matrix asserted | exit 0 | this session |
| `--scope finish --family quint` | 228 | 61 complete, 0 gaps, 0 empty, of 61; Q4 matrix asserted | exit 0 | this session |
| `--scope lifecycle --family alloy` | 73 | 25 complete, 0 gaps, 0 empty, of 25 | exit 0 | this session |
| `--scope lifecycle --family quint` | 93 | 25 complete, 0 gaps, 0 empty, of 25 | exit 0 | this session |
| `--scope task-tree --family alloy` | 104 | 41 complete, **2 declared gaps** (`TT-24.c`, `TT-24.d`) | exit 0 | measured inside the whole-repository run |

**And the whole repository, in one invocation: `models/run.sh` — 791 commands,
256 cells complete, 2 declared gaps, 0 empty, of 258. Exit 0.** Both delegated
`docs/ordinal-fs-tree/models/` runners ran (20 Alloy commands and 148 Quint),
which is the positive control: those suites are known green, so a repository run
reporting them clean while finding nothing anywhere else would be reporting a
broken instrument. Q4's removal matrix is asserted in both directions per family
— alloy 10 of 10 rows, 2 `none`, 1 abstracted; quint 10 of 10 rows, 3 `none`, 1
abstracted.

**Its only blemish is the placement problem, and the runner surfaces it
itself.** The coverage section prints exactly two lines:

```text
  TT-24.c    alloy:gap quint:ok
  TT-24.d    alloy:gap quint:ok
```

That is entry 048's observation rendered by the instrument rather than argued in
prose — *the coverage matrix scores a transcription above an honest
declaration*, since Quint's `inv_TT_24c` fills a cell with a transcription of
`gateOutcome`'s own branch that no control kills, while Alloy's honest
`out-of-bounds` declaration reads as the gap. It is
`obligation-placement-k63`'s to settle.

**The task-tree Alloy file is 104 commands here against the 103 its README
records.** The extra one is entry 048's retained replay,
`witness_finding_a_world_write_during_an_open_scaffold_reaches_legacy`, added
when that session corrected `step`. The README's figure predates it.

**No wall-clock figure is taken from these runs.** They were executed
concurrently, and for part of the session the host was also compiling an
unrelated package with sixteen `cc1` processes; load averages passed 140 on a
16-core machine. Command counts, coverage matrices and exit status are immune to
that. Timings are not, so every timing cited anywhere below is quoted from the
producing session's README and labelled as such.

**129 obligations across the three scopes** — `TT` 43, `FN` 61, `SY` 25 — and
the matrix is `(family, obligation)`, so the phase's unit of coverage is 258
cells and not 129.

**The whole-repository run is the phase's own instrument and is reported
separately**, in `models/README.md`, because a bare
`models/run.sh` additionally asserts coverage over the *whole* catalogue in one
invocation and delegates to the two `docs/ordinal-fs-tree/models/` runners — a
positive control the per-cell runs do not carry.

**The cost asymmetry between the two families is real and is not about the
tools.** Alloy's task-tree cell is 6888 s of CPU for 103 commands; Quint's is
1209 s for 111. Alloy's finish cell is 14 m 33 s for 180; Quint's is 4 m 25 s for
228. Reading either ratio as "tool X is faster" would be wrong in both
directions at once: the task-tree Quint model unrolls a tree walk to `MAX_DEPTH`
on every transition and the finish one has no tree at all, while the Alloy
task-tree file is the only one in the corpus carrying `Int` — positions and keys
— and the bitwidth arithmetic dominates it. That last claim is enumerated rather
than inferred: across all four `.als` files in the repository,
`crates/grove-task-tree/models/task-tree.als` declares an integer scope on
**104** commands and `finish.als`, `lifecycle.als` and
`docs/ordinal-fs-tree/models/structure.als` declare **none**. **Run cost in this
corpus tracks the data structure in the model, not the family.**

## What the ledger supports, and where it is not there

The pre-registration fixed eight measures. **Two were kept as specified (M1,
M4), one is a structural zero (M6), and five were kept unevenly (M2, M3, M5,
M7, M8).** Saying so is a precondition for reading any figure below — a
synthesis that averages over a partly-empty ledger reports a number where it has
a gap.

| measure | state of the ledger |
|---|---|
| **M1** unique vs overlapping | **kept**, and corrected once by entry 048 finding 4. Every material finding carries a tag. |
| **M2** class | **kept for all but nine.** The Alloy finish column numbers twelve findings in its README; findings **4 – 12** live in entries 034 – 039, which record them in prose with no `M2`/`M3` tag at all. Every other material finding in the experiment carries a class. |
| **M3** counterexample usefulness | **same gap as M2**, and it is the gap that costs most, because M3 is H6's whole operand. |
| **M4** derived tests | **kept**, and its result is uniform: see H7. |
| **M5** authoring hours ÷ checked claims | **kept as an hours-per-obligation figure in nine entries of twenty-three** — 026 – 030 (Alloy, task-tree) and 040 – 043 (Alloy, lifecycle). Entries 044 and 047 give a session-level estimate instead. **Not recorded at all:** the whole Alloy finish column (031 – 039), and the Quint finish and lifecycle columns. |
| **M6** synchronization | **recorded as `0` wherever it is recorded**, and the reason is structural rather than a lapse — see H9. |
| **M7** state-space and tooling | **run wall-clock kept everywhere; `wrangling` hours kept in six entries.** The two are different columns and H9 compares the second. |
| **M8** false-confidence incidents | **incidents kept; durations kept in a mixture of hours and "rounds"; the control arm — genuine failing checks with hours — is almost empty.** See H10. |

**The M2/M3 gap has one cause and it is worth naming, because it is a lesson
about a recording protocol rather than about either tool.** The
pre-registration's recording protocol says *every material finding carries its
M1 tag, M2 class and M3 score, recorded before the fix*. Entries 031 – 033 do
this in a blockquote form; entries 034 – 039 switched to numbering findings in
the model README (*"a fourth finding"*, *"a seventh, an eighth and a ninth"*) and
carried the prose across into the entry without the tag line. Nothing detected
it: the runner checks obligations, not entries, and each slice's own reviewer
was checking the model. **A per-finding recording obligation with no instrument
behind it decays into the shape of whatever the previous entry looked like**,
and it decayed at exactly the boundary where one leaf handed off to the next.

## The M1 census, after entry 048's corrections

Entry 048 changed two things about this census, and neither is optional.

1. **Six findings tagged `quint-only (pending replay)` are `both`** — reached
   independently by each column before either read the other (048, finding 4).
2. **Findings whose correction had already been written into
   `docs/specs/semantic-contract.md` before the first Quint model existed are
   not measurements of Alloy's unique reach.** Four Alloy sessions edited the
   shared catalogue; the Quint column read the corrected text and could not
   rediscover what had been removed from its search space. Their `alloy-only`
   tags are true and uninformative, and H4's test must be computed over findings
   **reachable by both columns** or it scores commit order.

Each column's own reach: **Alloy 29** (3 task-tree, 12 finish, 14 lifecycle) and
**Quint 17** (6, 6, 5). Replay found **seven** pairs, so the two columns between
them hold **39 distinct** findings; entry 048's own findings about the pair and
the instruments bring the experiment to roughly **45**.

**The finish column has two counting surfaces and they disagree by two, which is
the recording gap showing up as a number.** Its README numbers twelve findings.
Entries 031 – 033 additionally carry two M1-tagged items the numbering does not
include — the rule that *every "never" claim about tree shape is false under a
free initial state unless restated over the transition relation*, and a
correction to the corpus's own cost model. Both land in durable documentation, so
the material-finding rule counts them and the README's numbering does not. **12
is used above because it is the column's own count**; 14 is defensible and
changes no verdict, since every hypothesis that reads these numbers reads them
for non-emptiness rather than for magnitude.

| scope | `alloy-only`, reachable by both | `quint-only` | `both` | excluded as unreachable-by-both |
|---|---|---|---|---|
| task tree (`TT`) | **0** | 6 | 0 | 3 (entry 026, fixed into the catalogue by `names-k33`) |
| finish (`FN`) | **8** | 3 | 3 | 1 (entry 031's `FN-05.a`, fixed by `entry-k39`) |
| lifecycle (`SY`) | **10** | 1 | 4 | 0 |

Raw, uncorrected, the census would read `alloy-only` 29, `quint-only` 17,
`both` **0** — the two columns sharing nothing at all. That figure is an
artifact of the tagging discipline (each column tagging its own findings
`X-only (pending replay)` in good faith) and it is what entry 048 was cut to
repair. **A pre-replay M1 census is not a measurement; it is a pair of
self-reports.**

## The hypotheses, each decided by its own falsifier

| # | prediction | verdict |
|---|---|---|
| **H4** | the blind spots differ — at least two scopes mutually discriminating | **supported** |
| **H5** | temporal Alloy and executable Quint fail differently on the same question | **falsified**, and the correct axis identified |
| **H6** | counterexample usefulness is a property of the tool | **inconclusive**, by the pre-registered thin-result rule |
| **H7** | findings convert to tests | **falsified**, and almost universally |
| **H8** | placement is a real trade-off | **supported on the one family the ledger can compute** |
| **H9** | synchronization is the dominant hidden cost | **falsified**, and the reason is the protocol rather than the tools |
| **H10** | a green run's silence is the main hazard | **supported**, but decided by the pre-registration's zero-rule rather than by a measurement |

### H4 — the blind spots differ. **Supported.**

Two of the three scopes are mutually discriminating on the corrected census:
**finish** (8 `alloy-only`, 3 `quint-only`) and **lifecycle** (10 `alloy-only`,
1 `quint-only`). The prediction was *at least two of three*; the falsifier was
*fewer than two*. Two.

**Lifecycle passes on a single finding, and it is worth naming which.** Four of
entry 046's five findings became `both` at replay; the survivor is finding 5,
*`SY-03` says "no later gate" without saying what a gate is*. H4's test is
non-emptiness in both directions, so one is enough — but a reader should know
the margin is one finding wide, and that entry 048 discharged rule 3 for the
non-overlaps **collectively** (*"six are overlaps"*, with `reap` making a
seventh) rather than recording a per-finding reason for each of the ten that
were not.

**The scope that fails is the informative one.** Task-tree is not mutually
discriminating because its `alloy-only` count, computed over findings reachable
by both columns, is **zero** — every Alloy task-tree finding had been written
into the catalogue before the Quint column read it. So the one scope where H4
fails, fails for a reason that is not about blind spots at all. Had the
independence protocol carried its fourth rule from the start, task-tree would
have been the third discriminating scope or the honest absence of one; as run,
it can be neither.

**What the support is worth.** The `alloy-only` counts are inflated by the
Alloy column having nine finish slices and four lifecycle slices to Quint's one
apiece, and by six of the nine finish slices recording findings the counting
rule can still count but the class rules cannot. H4 is a claim about
*non-emptiness in both directions*, which is robust to all of that; a claim
about the *ratio* would not be, and none is made.

### H5 — temporal Alloy and executable Quint fail differently. **Falsified.**

H5 made two directional predictions and required both. The second fails twice
over.

- **Quint reaches interruption/ordering defects Alloy does not: HELD.** Entry
  044's two sharpest findings — `PartialScaffold` against a foreign write, and
  bulk-mark convergence against `AlreadyTerminal` — are both `quint-only` and
  both classed `interruption`.
- **Alloy reaches eventuality claims bounded simulation cannot: FAILED.** The
  corpus contains exactly two findings classed `eventuality`, at entries 046 and
  047. **Both are `quint-only`.** No `alloy-only` finding anywhere carries the
  class. H5's falsifier fires on both of its own clauses at once: *no
  `alloy-only` finding carries the M2 class `eventuality`*, **and** *the class
  lands in the opposite formalism*.

**And the reason is not that Alloy cannot do eventualities. It is that the
catalogue both columns were written from contains none, deliberately**, which is
checkable at the source rather than inferred from the tags. `SY-13` is the one
claim that reads like a liveness property, and
`docs/specs/semantic-contract.md` states in its own
words that it is **existential reachability and deliberately not a liveness
property**: *"Stating it as 'the loop will reach one' would need a fairness or
admission premise the models have no grounds to grant."* The deliberate-omissions
table then removes the other candidate — *clocks, timeouts and retry counts* are
omitted because *a bounded handoff wait is a liveness property of the
implementation, not of the protocol*. Entry 045 records the same for the finish
scope (*"No eventuality is checked"* — `FN-23`'s idempotence and `FN-21.a`'s
resumption are stated as safety), and no model in the corpus assumes fairness
anywhere.

**So the arm of H5 that depended on eventualities had nothing to run on, and the
decision that removed it was made in the shared ancestor before either model
existed.** That is a property of the subject and of the catalogue's own
modelling choices, not of Alloy 6 — and it is exactly the confound a
pre-registration exists to expose rather than to hide. H5 is falsified on the
ledger as kept; what it actually measured is that **a catalogue written to be
checkable without fairness cannot discriminate two tools on their handling of
fairness.**

**One honesty note on the ledger behind this.** Seven of the Alloy finish
column's twelve findings (entries 034 – 039) carry no `M2` tag at all, so they
cannot carry the class either. The catalogue check above is what makes the
verdict safe against that gap: there is no eventuality claim for an untagged
finding to have been about.

**The axis H5 should have named, supplied by entry 048 finding 5.** What
separated the two columns was not temporal operators versus guarded actions. It
was **the shape a property is stated in**:

> A property stated over **one action's own before/after pair** cannot discover
> that its claim was quantified too widely. A property stated over **the trace**
> can.

Quint's `SY-04.b` accumulated `refusalMutated` from one operation's pair and was
*silently correct*; Alloy stated the same claim over the trace and the
catalogue's missing scoping became visible as a counterexample. Neither is a
tool property: `quint-statement-shape-k61` restated the property over the trace
in Quint and it immediately found a counterexample **of its own** that no Alloy
command had — a validated configuration standing as a licence across a
`configChange`. **The same tool, the same model, a different statement shape,
and a finding appears.**

That leaf also added a second axis the rule above misses. `SY-10.b` was stated
at the *right* grain and discharged by `not(hist.silentPark)` — a restatement of
the model's own dial spelled backwards. **A property discharged by a
configuration rather than by an observation is unfalsifiable at any grain**, and
no trace/pair classification detects it. Twelve of the twenty-five `SY-`
properties are pair-shaped and each is the right grain; *pair* is a defect only
when the claim's subject is wider than one operation.

**Transferable, and it is this experiment's most reusable result:** when
comparing two behavioural formalisms, control for statement shape and for what
discharges the property, or the comparison measures the modeller.

### H6 — counterexample usefulness is a property of the tool. **Inconclusive.**

H6 needs the **paired** population: findings tagged `both`, each carrying an M3
score per formalism. The corrected census has **seven** paired findings, which is
already above the pre-registered floor of five — but **none of the seven carries
two M3 scores**. Three cannot even in principle: their Alloy counterparts are
retained counterexamples in `crates/grove-finish/models/README.md` rather than
entry-recorded findings, and a retained counterexample carries no M3. For the
rest, entry 048 records the re-tagging itself with a single score or `M3 n/a`.
The usable paired population is therefore **zero**.

`inconclusive` was pre-registered for "fewer than five paired findings", and it
is recorded here rather than rescued with a different statistic — which is what
the clause was fixed in advance to prevent.

**What made it uncomputable is worth more than the verdict.** M3 is scored *at
the moment of reading, before any fix*, per `(finding, formalism)`. A finding
only becomes paired at **replay**, which is many sessions later and by
construction after both fixes. So the operand H6 needs and the moment H6's
population is determined **cannot both be satisfied by a protocol that forbids
cross-reading until both columns are green**. H6 is not thin by accident; it is
incompatible with independence rule 2 as written. A future experiment wanting
H6 must have each column score M3 for *every* finding it reaches, whether or not
it believes the finding unique — which costs nothing and is the only way the
scores exist when the pairing arrives.

### H7 — findings convert to tests. **Falsified.**

The falsifier is *any* material finding that yields no executable Rust test
failing against the pre-fix implementation. There are many, and the pattern is
uniform: **`M4 = none` is the modal value across every entry that records
M4** — entries 031 – 033 and 040 – 048 alike — and the
reason is nearly always the same one the pre-registration's third borderline
anticipated — *shipped behaviour the model says is fine, but a catalogue
required to describe it never described it*. There is no defect to write a test
against; the correction is a catalogue row.

Counting the exceptions is quicker than counting the falsifications. Findings
that **did** derive a failing test:

- entry 044, `PartialScaffold` vs a foreign write, and bulk-mark convergence —
  two, in the black-box binaries;
- entry 045, five tests across `tests/finish_lifecycle.rs` and
  `src/finish_cleanup/auxiliary/tests.rs`;
- entry 048, the `PartialScaffold` window, **red today**.

Every other material finding in the experiment records `M4 = none`.

**This is the single most important result for the phases that follow.** A
formal phase run against an implementation that already exists and is green
produces, overwhelmingly, **corrections to the specification rather than defects
in the code** — and H7 was written on the assumption that a finding worth having
is a finding a test can catch. On this subject that assumption is false about
four findings in five. The value the models delivered is almost entirely in
`docs/specs/semantic-contract.md`, and the phase that consumes it is
documentation, not implementation.

### H8 — placement is a real trade-off. **Supported, on one family.**

Both conjuncts hold, computed as pre-registered — *authoring hours ÷ checked
claims*, aggregated separately:

| arm | entries | hours | obligations | h/obligation |
|---|---|---|---|---|
| component-local (Alloy, task-tree) | 026 – 030 | 11.83 | 41 | **0.289** |
| system-level (Alloy, lifecycle) | 040 – 043 | 9.33 | 25 | **0.373** |

**Both arms are complete sums rather than partial ones**, which is checkable and
was checked: the component-local slices' obligation counts are 16 + 11 + 4 + 6 +
4 = **41**, and entry 030's two declared gaps make **43** — the whole `TT-`
scope. The system-level slices' are 6 + 5 + 6 + 8 = **25** — the whole `SY-`
scope. Neither aggregate is missing a slice.

Component-local is strictly lower, which is the operand H8 names. And the system
model produced material findings the component models did not — fifteen distinct
ones, about the joint rather than about either component: the two situations the closed
outcome and refusal sets cannot name, the licence a validated configuration
becomes, `SY-13`'s sinks, and the `reap` gate.

**Three narrowings, each of which the reader needs.**

1. **This is one family's arm.** The Alloy finish column (031 – 039) and every
   Quint column recorded no authoring hours, so "component-local" here means
   *the Alloy task-tree file* and nothing else. Entry 026 also folded **1 h of
   runner work** into its 3 h, and the runner is not a model file; excluding it
   gives 10.83 h over 41 obligations = **0.264**, which is lower still. The
   verdict is unchanged in either reading, which is the only reason it is
   reported.
2. **Entry 043's warning is applied rather than ignored.** The four lifecycle
   points — 0.42, 0.60, 0.46, 0.14 — are a **declining marginal-cost curve
   inside one file**, not four samples of system-level cost: by the fourth slice
   eight obligations needed three transitions and four fields, riding on
   machinery three earlier sessions paid for. The component-local arm has the
   same structure. Comparing **first slice to first slice**, as entry 043 asked:
   **0.19 (entry 026) against 0.42 (entry 040)** — the same direction, roughly
   twice the ratio. The verdict does not depend on which counting rule is used,
   which is the only reason it is reported at all.
3. **Neither figure is a per-obligation estimate for a new scope.** Both arms
   mix the cost of *building* a model with the cost of *adding claims to one*.

### H9 — synchronization is the dominant hidden cost. **Falsified.**

Two of the three falsifying clauses fire.

- **M6 is `0` for the entire construction phase**, recorded as such in every
  entry that records it (026 – 030, 042, 043). Not a lapse: the Alloy column had
  no sibling to synchronize with, and the Quint column was **forbidden** to read
  it. Synchronization was structurally unavailable until entry 048.
- **Post-barrier, M6 is one replay session plus one restatement session** —
  entry 048's two corrections to `task-tree.als` and
  `quint-statement-shape-k61`'s two restatements in `lifecycle.qnt`. Entry 048
  prices its own solver time at ~25 minutes; generously bounding the pair at two
  sessions puts M6 at roughly **4 h**.
- Against a denominator of M5 (≥ 22 h recorded, certainly higher) + M6 + M7
  wrangling (≈ 4.2 h recorded), M6's share is **≈ 10%**, below the 30% floor.
  And M6 does not exceed `max(authoring_alloy ≈ 21 h, authoring_quint)`.

**The finding under the falsification.** H9 predicted the cost of *keeping two
models in agreement*. This experiment never paid it, because the independence
protocol replaced continuous synchronization with **one deliberate replay**, and
that is a different and much cheaper operation — entry 048: *"ninety per cent of
this leaf's value came from reading, and the solver's whole contribution was to
make three of those readings falsifiable."*

**But it is not free, and the tax is nameable.** Entry 048: *a negative result
from a bounded tool is ambiguous until you spend a run disambiguating it*, and a
replay session pays that on every finding it carries across — three narrowing
probes to establish that one *no instance* was a modelling exclusion rather than
a bound, and one run to tell an inert mutation from a surviving one.

**Transferable:** running two formalisms independently and replaying once is
cheaper than maintaining them in step, and it buys a measurement that
maintaining them in step destroys. H9's premise — that two models must be kept
in agreement as they are written — is a description of a workflow this
experiment deliberately did not run.

### H10 — a green run's silence is the main hazard. **Supported, by the zero-rule.**

| arm | recorded |
|---|---|
| false-confidence incidents, standing hours | 026 ≈ 0.5 h · 031 ≈ 0.33 h · 040 ≈ 1.0 h · 041 ≈ 0.7 h · 045 ≈ 0.25 h — **≈ 2.8 h** |
| genuine failing checks, hours spent | **nothing recorded anywhere** — counted as **0** |

`2.8 > 0`, so H10 is supported on the ledger as kept. **It is supported by the
pre-registration's own rule that an unrecorded duration counts as zero, not by a
comparison**, and that is stated plainly because the alternative is to present a
default as a result. The control arm was specified precisely — M8 was revised
before any model existed *because* it was one-sided — and it was then almost
never populated. Entry 046 lists five genuine failing checks with no hours
against any of them.

**The uncounted half is larger than the counted half, and it points the same
way.** The 2.8 h above omits every incident recorded in *rounds* or as *the
whole producing session*: entry 048's `TT-20` tautology **stood for the life of
the file**, through five slices, a mutation matrix, an assumption table and a
retained-counterexample list, none of which could reach it. And two entire
review-plus-integration chains — `task-tree-k55`/`k56` and `system-k59`/`k60` —
exist because a column that reported its own incidents shipped more: `k55` found
**three false greens in the runner**, and `k59` found **four further
false-confidence incidents that had survived the producing session** — inside
six findings of which four were invisible to any run of the suite itself. Those
are four sessions whose entire cost is the price of green runs that meant less
than they appeared to. Counted in sessions rather than in
hours, the false-confidence arm dominates by a wide margin.

**Transferable, and it is the discipline this experiment would keep above all
others:** the unit worth reviewing is not the model and not the finding, it is
**the pair `(claim, the control that would falsify it)`**. Four of `system-k59`'s
six findings were invisible to the suite because a suite cannot ask whether a
mutation kills anything besides its target, whether a measurement measures its
subject, or whether a quoted number can be re-run.

## Which formalism caught what, stated as shapes rather than as counts

The census says *how many*. What a reader can act on is *which shape of
question each family was in a position to answer*, and the two families differ
by one structural property that no hypothesis predicted.

**Alloy posits states; Quint builds them.** `finish.als` retains **twenty-one**
counterexamples, of which the README says twenty are about the model or the
catalogue rather than about the protocol — and **twelve are a single shape**: a
claim about tree *shape* falsified by Alloy's free initial state. A preparing
witness with something already inside it, a manifest half-written by no step, a
hand-edited marker pair. Their lesson is one rule met at five separate grains:
*a shape claim under a free initial state must be restated over the transition
relation.*

**None of the twelve has a Quint analogue, and the reason is structural rather
than a difference in power.** `finish.qnt` and `lifecycle.qnt` each open with an
`action init` that constructs one state; every later state is reached by a
transition. **The entire class is unreachable in Quint by construction.**

So each family pays for its non-protocol states in a different currency:

| | how non-protocol states arise | what it costs | where the cost shows up |
|---|---|---|---|
| **Alloy 6** | posited by the solver from a free initial state | fencing off the states the protocol cannot build | twelve retained counterexamples in one file |
| **Quint** | built by transitions from one `init` | reaching the rare ones | twelve `scenario_` instances in the finish column, four in task-tree |

**Reading either column's retained-counterexample total as a finding count
double-counts a modelling strategy.** That is the single most likely misreading
of this experiment's raw numbers, and it is why the census above counts
*material findings* and not counterexamples.

**Quint's own productive shape is the witness obligation, and it paid off in a
way nobody predicted.** The catalogue never says which lifecycle actions are
gated on the root's classification. Quint gated `reap`; Alloy's `doReap` reads
no tree at all. The Quint model's `wit_SY_05a` then **never landed**, and the
unlanded witness is what turned a wrong guess into a finding. *A witness that
never lands is how a model reports its own wrong guess about an underdetermined
specification* — and the shipped driver settles it: `src/loop_driver.rs` reads
no root classification on the reap path, so the catalogue gap is real and there
is no product defect behind it.

**And the pair produced two findings neither column could have produced alone**,
which is the strongest available argument for running two:

- **Two opposite resolutions of one silence.** `FN-13`'s stated witness
  conflicts with *Outcomes*' rule that a caller who has already mutated is owed
  `Blocked(OwnershipConflict)`. Alloy followed the catalogue and **refused**;
  Quint followed the three-contexts rule and **blocked**. Two independent
  readings of one document, both reasonable, incompatible. `reap` is the same
  shape. **An opposite resolution is stronger evidence that a specification is
  underdetermined than either column's own account of it.**
- **A check that cannot fail in the direction the other column's counterexample
  points.** `TT-20`'s fourth conjunct was a theorem of `rootState`'s own body —
  green with no protocol premise at all, at its own bound. Entry 044's
  counterexample is a tree that *stops being* a partial scaffold, so it never
  entered the antecedent. **A third vacuity class**, beside *a bound too small*
  and *a state reached and empty*: **an antecedent that excludes exactly the
  counterexample the claim is about.** No mutation matrix, assumption table or
  witness obligation can reach it, because each of those asks whether a check
  *fails*.

## What neither formalism established

Recorded with the care the pre-registration demands of misses, because these are
what the documentation and implementation phases inherit as *unmodelled*.

- **No eventuality is checked anywhere in the experiment.** The catalogue's
  claims are safety claims; `FN-23`'s idempotence and `FN-21.a`'s resumption
  read as liveness and are stated as safety. Any "recovery always terminates"
  reading of these models is unsupported.
- **Nothing is model-checked past shallow depth.** Quint's task-tree column is
  bounded randomized simulation and nothing more — Apalache is reachable only
  after splitting the controls into a second file and restating `gapless`, and
  even then the full instance exhausts a 24 GB heap past `--max-steps=3`.
  Quint's finish column model-checks to depth 4; the lifecycle column's `verify`
  completes at `--max-steps=4`, **which does not reach a scaffold**. Alloy's
  `check` is exhaustive only within each command's own scope.
- **A lane-blind model passes every `FN-` check.** `EN-16`'s collapse control is
  measured: with the lane collapsed to `GitL`, every `FN-` property stays green
  and only the named witnesses stop landing. The three lanes are demonstrably
  *reachable*; the claims *differing* under them is not established.
- **Native jj's partial-commit hazard is unmodelled** — the deletion staying in
  the change while the unselected witness moves into a *successor* revision
  needs a notion of successor revisions no file here has. Entry 033 calls this
  the sharpest thing not to read a green run as covering, and this synthesis
  repeats it rather than softening it.
- **The marker-replacement protocol's byte layout** is a deliberate catalogue
  omission; `EN-01` *grants* same-directory rename atomicity rather than the
  model establishing it.
- **Disposal's removal is not shown re-enterable *within* a step.** `EN-03`
  gives the shipped removal one interruption point per entry; a defect needing a
  partial recursive deletion is outside every green above.
- **`FN-21.b`'s in-tree-witness condition is checked at a coarser grain than the
  shipped lease-owned reaper uses.** The model errs towards declining, so a
  shipped reaper that proceeds where the model declines satisfies every check.
- **The four revalidation points are not shown to be the right four.** A
  protocol with a third filesystem handoff would need a fifth and sixth, and
  nothing would notice.
- **The experiment has no control arm and cannot acquire one.** The shipped
  implementation *is* the prose-driven arm, and any claim that a model beat
  prose here is confounded by the whole v1.0.0 → v19.3.0 history of this code
  being read, reviewed and revised. **H2 — the model leads profitably — is not
  retested and cannot be**, on a subject whose implementation was green before
  the first model was written.

## Cost, counterfactual, verdict — for the experiment as a whole

**Cost.** Twenty-three entries (026 – 048) across **more** sessions than that —
three of the Quint columns were each revised by a `review-prototype` and an
`integrate-review-prototype` leaf, and two more leaves were cut downstream — over
two model families, 129 obligations, 258 `(family, obligation)` cells and
approximately 30 000 lines of model and model documentation. Authoring hours are
recorded for nine slices and total **21.2 h**; the true total is materially
higher and unrecoverable. Run cost at this revision is dominated by one file:
Alloy's task-tree cell at 6888 s CPU against every other cell's minutes.
**Wrangling — hours lost to the tool rather than to the problem — is ≈ 4.2 h
recorded**, and its single largest item is entry 026's 3 h of performance
cliffs, output-format discovery and a dead end in which every check ran at three
steps before the unconstrained initial state was noticed.

**Counterfactual.** Stated per shape rather than as one claim, because the
answer differs sharply:

- **Available by careful reading**: most of the catalogue's textual
  contradictions — `TT-17` against `PartialScaffold`, `TT-15.a` against
  `TT-24.b`, `FN-25.b`'s literal wording. A sufficiently adversarial reader with
  the catalogue open finds these. What the models supplied was the *forcing
  function* that made someone read that closely, which is real but is not a
  claim about formalism.
- **Not available by reading**: every finding that needs an interleaving.
  Entry 043 is explicit — the `doTreeOp` defect is missing a conjunct in a
  predicate 400 lines from the claim that wanted it, and its three sibling
  transitions carry the conjunct correctly, so *a diff review sees three right
  answers and no fourth question*.
- **Not available from either column alone**: the pair findings above.
- **Not available from any model**: the two questions the product fixtures
  settled in minutes — whether `reap` reads a tree in the shipped driver, and
  which window of an interrupted `root-init` actually reaches `Legacy`.

**Verdict — three sentences, and the third is the one to carry.**

1. **On this subject the two formalisms were complementary in the finish and
   lifecycle scopes and indistinguishable in the task-tree scope**, and the
   task-tree result is an artifact of a writable shared ancestor rather than a
   fact about the tools.
2. **The value delivered was overwhelmingly specification correction rather
   than defect detection** — `M4 = none` is the modal outcome — which is what a
   formal phase should be expected to deliver against an implementation that is
   already shipped and green.
3. **The most productive single instrument in the whole experiment was neither
   tool.** It was the **pair `(claim, the control that would falsify it)`**,
   applied adversarially by a fresh context: `system-k59` found six real defects
   in a suite that was green, coverage-asserted and had already published its
   own false-confidence ledger, and four of the six were invisible to any run of
   that suite.

## The combined workflow this subject would actually use

H4, H5 and H8 were declared load-bearing because *they decide what the eventual
routing advice says*. This is that advice, bounded to subjects like this one — a
stateful protocol with crash recovery and a VCS boundary, modelled against an
implementation that already exists.

1. **Write the tool-neutral catalogue first, and freeze it before the first
   model.** This is independence rule 4, and it was added at entry 048 *after*
   four sessions had already edited the shared ancestor. The cost of learning it
   late is that one of three scopes can no longer be scored. **An independence
   barrier over two models is not a barrier while their common ancestor is
   writable.**
2. **Run the two families independently and replay once. Do not maintain them in
   step.** H9's predicted synchronization cost never materialised because the
   protocol replaced it with one deliberate replay, and the replay is where the
   two findings neither column could reach came from. Budget the replay's own
   tax honestly: **a negative result from a bounded tool is ambiguous until you
   spend a run disambiguating it**, and a replay pays that per finding carried
   across.
3. **Route by statement shape, not by tool.** Before choosing a formalism, ask
   two questions of the claim, both of which cut across both tools:
   - *Is the claim's subject wider than one operation?* If so, state it over the
     trace. A property stated over one action's own before/after pair cannot
     discover that its claim was quantified too widely.
   - *Is the property discharged by an observation, or by a restatement of the
     model's own configuration?* The second is unfalsifiable at any grain and no
     shape classification detects it.
4. **Then route by where non-protocol states come from.** For a claim about
   *shape* that must hold from arbitrary starting states, a free initial state
   is the cheap instrument and Alloy has one. For a claim about whether a rare
   interleaving is *reachable at all*, a constructed initial state plus a
   witness obligation is the cheap instrument and Quint has one — and an
   unlanded witness is a first-class result, not a failure to measure.
5. **Every property carries a control that would falsify it, and the
   `(claim, control)` pair is what a fresh context reviews.** This produced more
   than either tool did. A suite cannot ask whether a mutation kills anything
   besides its target, whether a measurement measures its subject, or whether a
   quoted number can be re-run.
6. **Score M3 for every finding at reading time**, not only for findings
   believed unique — otherwise the paired population exists only after the
   scores can no longer be taken, which is what made H6 uncomputable here.
7. **Reach for a product fixture before reaching for a wider bound.** Four
   fixtures and a reading of two functions settled two questions the models could
   not, in minutes.
8. **Check whether your subject has any eventuality claims before paying for a
   temporal tool on that ground.** This one had none, and half of H5 had nothing
   to run on.

## Concrete changes to design, tests and documentation

Named here, executed by this node's siblings. This leaf decides no catalogue
question; the list is the hand-off, and every item names its owner.

**To `obligation-placement-k63` — the placement shape, at six recorded
instances.** An obligation whose subject spans two component scopes currently
has no home rule, and the consequences are already visible in three different
ways: `TT-24.c` and `TT-24.d` are declared out-of-bounds by the task-tree Alloy
column and answered by the Quint one; Q4 row 6 in the finish column is *decided
by a mutation and cited to an obligation no check in that directory can name*;
and `SY-06.b`, `SY-05.b` and `SY-14`'s operator exit are imported into the
lifecycle scope and checked in neither. This is what *model-to-crate ownership*
means for the crate boundary the root brief approves, so it is settled before
the disposition that would otherwise settle it case by case.

  **Outcome, recorded here because this hand-off named it as owed.** The rule is
  [`obligations-follow-context-not-artifact`](adr/obligations-follow-context-not-artifact.md):
  an obligation belongs to the scope that can execute its context, ordered by the
  approved crate dependency direction, and a clause a scope cannot check stays in
  place only as a declared **cross-scope citation**. Of the six instances, **two
  moved** — `TT-24.c` and `TT-24.d` are retired to `FN-32` and `FN-21.c` — and
  four are declared in place. **Two measurements fell out of the move and belong
  to this record rather than to the ADR.** First, `TT-24.c` was *false as
  literally worded*: `finish.als` **refuses** at its antecedent
  (`FN_10b_content_the_discard_cannot_classify_fails_closed` requires
  `Sys.res' in Refused`) while `finish.qnt` **blocks** at the same step, and both
  are green against `FN-10.b`, whose text says only *fails closed*. That is a
  seventh `both`-tagged opposite-resolution pair, found by attempting a
  re-statement rather than by either column, and it goes to
  `catalogue-disposition-k64`. Second, the transcription this hand-off names is
  now visible to the instrument: `models/run.sh` reports a **contested cell** —
  one family answering what another declared out of reach — with whether the
  answering family carries a control. It is reported, never fatal, and after this
  leaf there are none.

**To `catalogue-disposition-k64` — every inherited catalogue finding.**
Ninety-seven sites across eleven files name `formal-synthesis-k16` and hand it a
disposition. The classes, so the child can size its own decomposition:

- **Closed-set additions.** `RRolledBack`, `RConfigInvalid`, `RGenContended`,
  `ONotEntered`, `Stopped` — each declared by a model as an addition the closed
  refusal or outcome set has no member for. **Each of these changes the runner's
  manifest**, so each cascades into commands in both families.
- **Claims false as literally worded.** `FN-25.b`, `SY-13` on the
  `Legacy`/`Foreign`/`Malformed` sinks, `TT-17` against `PartialScaffold`,
  `TT-15.a` against `TT-24.b` and `TT-18`.
- **Underdetermined claims the two columns resolved in opposite directions.**
  `FN-13`'s refuse-or-block, and whether `reap` is gated on the root's
  classification. These are the sharpest items on the list precisely because
  neither reading is unreasonable.
- **Scoping.** Whether `SY-04.b` owes `SY-03`'s licence prohibition for the
  configuration as well as for the layout; `SY-14.b`/`SY-04.b` against
  `release-lease`, which touches no tree; `EN-11`'s controls column
  mis-attributing `TT-24.b` to `hand-edit` where the dependency is `EN-13`.
- **State-table completeness.** The shipped *ambiguous partial root scaffold*
  refusal is a fail-closed outcome the catalogue's state table has no member
  for, and it is a better answer than either model gives.
- **Prose, manifest-neutral.** The catalogue's *`models/run.sh` is the one
  repository runner* section says **three** obligations where Q4's own paragraph
  makes **four**.
- **Model-only, not catalogue.** The `SY-10.b` / `SY-11.b` collision over
  `WGen`: one obligation's apparatus constructs the state another forbids.
  `TT-24.c`'s Quint invariant is a transcription of `gateOutcome`'s own branch
  with no control — a control, a declared gap, or a re-scoping, and the third is
  `obligation-placement-k63`'s.

> **[disposed by `lifecycle-scope-k72`, closing `catalogue-disposition-k64`]**
> **The whole inherited set is disposed and the node is closed.** The enumeration
> ran at 93 sites (97 here, less the four `obligation-placement-k63` retired with
> `TT-24.c`/`TT-24.d`), classified into 24 decided items, 11 routed and 23 prose;
> the node split by scope behind a vocabulary child, and the four children are
> `routing-and-prose-k73`, `closed-set-additions-k74`, `task-tree-scope-k70`,
> `finish-scope-k71` (reviewed by `finish-scope-k75`, integrated by
> `finish-scope-k76`) and `lifecycle-scope-k72`.
>
> **What the catalogue's closed sets and counts became.** Refusal reasons
> **17 → 21** (`DeletionNotCommitted`, `ConfigurationInvalid`,
> `GenerationContended`, `ScaffoldIncomplete(class)`); outcomes **6 → 6**;
> blocked diagnoses **2 → 2**; task-root states gained `Reserved(Quarantined)`
> and split the witnessless root into `PartialScaffold(Exact)` and
> `PartialScaffold(Ambiguous)`; obligations **128 → 130** (`FN-29` gained `.a`
> and `.b`; `TT-17` split), which `models/run.sh --list` prints. §*Actions*
> gained the `validate-config` row it had always been short.
>
> **Two of the classes above resolved against their own headline.**
> `ONotEntered` and `Stopped` are **not** closed-set additions — the first is a
> call that was never made and the second was a level error, a missing REASON
> read as a missing outcome. And the *Scoping* bullet's `release-lease` finding
> is not a scoping question at all: it is the catalogue having used *lifecycle
> transition* for two sets with no member in common, which the two columns
> instantiated differently while both stayed green.
>
> **Five records carry the decisions**:
> [`a-refusal-leaves-nothing-standing`](adr/a-refusal-leaves-nothing-standing.md),
> [`a-witnessless-root-refuses-what-it-cannot-account-for`](adr/a-witnessless-root-refuses-what-it-cannot-account-for.md),
> `success-is-proved-by-the-ticket-not-the-tree` (retired at
> `delete-finish-transaction-k8`, with the transaction whose success it defined),
> [`a-closed-partition-is-over-outcomes-not-states`](adr/a-closed-partition-is-over-outcomes-not-states.md)
> and
> [`a-lifecycle-claim-says-what-it-is-over`](adr/a-lifecycle-claim-says-what-it-is-over.md).
>
> **The closing sweep, with the control that matters.** The subject sweep finds
> `formal-synthesis-k16` and `catalogue-disposition-k64` **only in this log** and
> in no live artifact; the positive control (`cross-model-replay-k15`) finds its
> own sites; the negative control finds none; and the **cross-tree** control —
> every NODE handle finds zero live sites while every LEAF handle finds its own —
> is the one `routing-and-prose-k73` learned to run, because a clean subject
> sweep hides a decomposition that has re-pointed at a directory instead of an
> owner.
>
> **The counts are deliberately not quoted here, and that is a finding about the
> instrument rather than reticence.** `formal-synthesis-k99` — the invented
> handle three sessions have used as the negative control — went from 0 sites to
> 1 the moment a durable record said it found 0. **A control handle, once named
> in a durable artifact, stops being a valid negative control**, and the same
> applies to every count in a sweep whose own report is inside the swept tree:
> writing *31 sites* makes it 32. The next sweep must invent a handle that
> appears nowhere (this one used `lifecycle-scope-k98`, 0) and must read its
> counts as *of the tree including this paragraph*.

**To `finish-verdicts-k65` — `TODO.finish_process.md` Q1 – Q4.** The catalogue
pre-committed the evidence that classifies each, so these are read rather than
argued. What is already on the record, and what is missing, is set out in this
session's running log; the verdicts are that child's.

> **[answered by `finish-verdicts-k65`, revised by `finish-verdicts-k78`]**
> `k65` answered all four **keep**; `finish-verdicts-k77` attacked that reading
> and `k78` integrated the result, leaving **Q2 and Q3 `keep` and Q1 and Q4's
> three cleanup rows `defer`** — the criterion `k65` declared mis-typed was
> completed rather than abandoned, and completing it returns `delete/replace` for
> a protocol `EN-03` says cannot be built, which is the defect measured rather
> than argued. `TODO.finish_process.md` is deleted: the questions, the
> dispositions, the cost table and the four binding constraints are now
> `docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`, itself
> retired at `delete-finish-transaction-k8` with the layer it kept.
> **Every earlier entry in this log that names `TODO.finish_process.md` names it
> as it stood when that entry was written and is left alone; this annotation is
> the one pointer forward.** The finding worth carrying past the verdicts: Q1's
> and Q2's pre-registered `delete/replace` criteria are each stated over a
> **counterfactual-capability** control, which measures whether a cheaper
> protocol would be *admissible* — so as pre-registered neither question could
> ever have returned `delete/replace` against a world where `EN-03` and `EN-05`
> hold, however green the run. A green counterfactual says a claim set does not
> depend on a mechanism; it does not say the mechanism can go.

**To `handoff-audit-k66` — reachability of the durable artifacts, and three
recorded documentation gaps.** This session checked all six model files against
the seven fields the node brief requires — tool version, bounds or trace limits,
solver/backend, fairness assumptions, abstractions, deliberate omissions, and
*what a green run does not prove*. **All three Alloy halves carry all seven.**
The three Quint halves are missing the same two or three each, and every one of
them is a gap of **placement rather than of substance**:

- **No explicit fairness statement in any Quint half.** Each Alloy half has one
  (`crates/grove-finish/models/README.md`: *"Fairness. None assumed, and none
  needed"*). The Quint halves state it nowhere, though entry 044 does — *"No
  fairness assumptions: nothing here is a liveness claim"* — and the catalogue
  settles it for the whole experiment at `SY-13`.
- **Tool version stated only incidentally** in the task-tree and finish Quint
  halves — `quint verify 0.32.0` appears inside a *Verification* paragraph
  rather than as a stated field. `models/system/README.md` gets this right, with
  `quint 0.32.0` on its run line beside the samples, depth and seed.
- **No *what a green run does not prove* section in the task-tree Quint half.**
  Its *Narrowings and qualifications, each declared* carries much of the
  material and entry 044's *Missed* carries the rest, so the
  pre-registration's recording obligation is met at the entry level.

The Alloy task-tree half also states its own version of that section as a bolded
paragraph inside *The mutation matrix* rather than as a heading — findable by
reading, not by scanning, which is the same weakness one degree milder.

**Tests — eight, and every one of them belongs to the implementation phase.**
Two from entry 044 (the `PartialScaffold`/foreign-write classification, and
bulk-mark convergence across an interruption), five from entry 045 (four in
`tests/finish_lifecycle.rs`, one in `src/finish_cleanup/auxiliary/tests.rs`), and
one from entry 048 (the `PartialScaffold` window — **red today**). Against the
roughly forty-five material findings this experiment produced, eight derived
tests is H7's falsification stated as a number.

**One test that must not be written yet.** The mid-flight block — an ordinary
mutation that has already applied an effect and whose next create is no longer
licensed — has no derived test *on purpose*: what it asks for is a decision the
catalogue has not made, and a test written first would encode the model's
least-wrong choice as though it were the contract.

## What this synthesis does not claim

- **Nothing about Quint or Alloy 6 in general.** One subject, one pair of
  pinned versions, one team, one methodology.
- **No comparison against prose was run**, and none can be: the shipped
  implementation is the prose arm, and its whole revision history confounds the
  comparison. H2 is untestable on this subject and is not tested.
- **The M-derived figures are as good as the ledger**, which is partly empty:
  no authoring hours for the Alloy finish column or for any Quint column beyond
  one estimate, no `M2`/`M3` tags for seven of the twelve Alloy finish findings,
  and an M8 control arm with no hours in it at all.
- **The finding counts are not a fair race.** The Alloy column ran nine finish
  slices and four lifecycle slices to Quint's one apiece, and the entry-level
  counting rule scores *what was written down as a finding*, not what was
  reached. Entry 048 has the worked example: `TT-23`'s idempotence requirement
  is a choice Alloy made and left in a code comment, and Quint recorded as a
  catalogue finding with a mutation behind it. **An unrecorded modelling choice
  and a recorded catalogue finding are the same event, and only one of them
  counts.**
- **Neither column is re-checked here.** This section re-ran the matrix and read
  the entries; it verified no model's reasoning.

---

## Distillation, second pass — entries 044 – 048 and the synthesis

`model-led-development-k94` carried this log's second half into the same skill,
`plugins/linkuistics/skills/model-led-development/`. The first pass above covers
entries 001 – 025 and stops there. The log stays as it is; what changed is that
the skill's citation keys now come in **three** forms, so a reader who doubts a
rule knows which file to open:

| key | what it names |
|---|---|
| `[003]`, `[044]` | a numbered entry in this log |
| `[synthesis]` | this file's closing sections, from *What is being compared, and what is not* to the end |
| `[c1]` – `[c5]` | the five candidate lessons adjudicated in [`candidate-lessons.md`](candidate-lessons.md) §§ 1 – 5 |

**Why this pass has a different shape from the first, and it is not a
convenience.** The first pass could row the log entry by entry because each entry
was one session's account of its own work, and nothing had re-opened it. This
half's evidence *had* been re-opened: [`candidate-lessons.md`](candidate-lessons.md)
went back to the model files, checked the producing sessions' accounts against
what the models actually say, and **weakened or falsified three of the six claims
those sessions believed** — including one this very log states as a slogan.
Landing the entries' own wording would therefore have shipped two claims this
campaign disproved. So the adjudication is the authority for every rule below and
the entries are cited where they carry the mechanism, not where they carry the
conclusion.

**Entries 026 – 043 have no rows of their own.** They are the Alloy columns, and
every rule this pass takes from them arrives through `[c1]` – `[c5]`, which
enumerate their obligations by name and cite the session and commit that found
each. Giving them rows here would restate an enumeration that already exists in a
more checkable form, and the enumeration is the thing a later reader should open.

**No session after the producers appended an entry**, and that is a fact about
this log rather than about the campaign. Entries 044 – 048 were all written by
the sessions that built or ran the models they describe — 044, 045 and 046 one
per model column, 047 a prototype experiment that is no column, and 048 the
cross-model replay whose subject is the pair. Every leaf that afterwards *scoped*, *reviewed*, *integrated* or
adjudicated a model recorded its work in a task body or a commit message, and
reached this file — where it reached it at all — only as an in-place
`[disposed by …]` annotation on somebody else's entry. Some of the campaign's
sharpest evidence never reached it in any form: neither `task-tree-scope-k70`'s
`FOCUS = 4` dead control nor `finish-scope-k76`'s fourteen-state refutation of a
declared impossibility appears anywhere in this file, and both are load-bearing
for rules the skill now states. `[c1]` – `[c5]` is where they are enumerated,
with a session handle and commit for each, and that is why the adjudication
rather than this log is the authority for this pass.

**So the instruction
[`keeping-a-log.md`](../plugins/linkuistics/skills/model-led-development/references/keeping-a-log.md)
then carried — every session that reaches for a formalism appends an entry
before it finishes — was kept by the sessions that *built* or *ran* models and
not by the sessions that *checked* them.** The recovery cost was one whole leaf re-opening model
files to rebuild what those entries would have carried, and it is the reason that
file now states the obligation over the work rather than over the role.

### Where each entry landed

| entry | where it landed | what did not survive, and why |
|---|---|---|
| 044 | the aggregate it contributes to the routing-economics correction — its share of the four-in-five *no derived test* ratio, which the skill states as the ratio and not entry by entry | its two derived tests **and** its `ONE_SNAPSHOT` dial with `mutant_two_listings`, neither of which is quoted anywhere in the skill: the correction landed the ratio rather than an exception list, and no worked example of a satisfied-by-construction dial was carried. Recorded here as a silent omission caught at review rather than a considered one — the material is still available to a later pass. Also the six catalogue findings themselves, which are grove's design and not routing evidence. **And its two structural choices — every action total, the interpreter as a state machine — deliberately.** The skill already carries both from `[003]`, and this entry reached them independently on a different subject. That is corroboration of an existing rule; it does not make the rule stronger in a reader's hands, so it is recorded here rather than added there |
| 045 | the starved-witness arithmetic — `(1/k)^20` over a twenty-step transaction, and every claim past the commit green on a witness that never landed — with the search-dial remedy that removes no behaviour; *quote a model-checked result with its depth*, on the eleven-step path a depth-4 check does not reach; and the budget rule in its narrowed form — *58 extra invariants were nearly free* (373 s at 3 invariants and depth 3 against 377 s at 61 and depth 4), read as a bound on the property count rather than as an isolation of depth, since that pair moves both operands and 445 s followed from three constants widening the state at no extra depth | its five derived tests, which the routing-economics correction carries only inside its aggregate ratio — the skill quotes no exception list; the six findings and three observations about the finish protocol, which are grove's; and its M8 runner incident, which is `[048]`'s in a sharper form |
| 046 | through `[c1]` and `[c3]`: `driverStep`'s forced teardown, which checked a whole scope over a world in which a finish session cannot decline, and `mutant_block_clears` firing through a defect in the sweep — the first of the three wrong-mechanism controls behind the isolating rule | *a deterministic loop satisfies twenty-three of its own claims by being written down*, and the search-dial-is-a-shape observation. Both are instances of traps the skill already states — satisfied-by-construction, and the sampling note — and neither adds a move a reader does not already have |
| 047 | one line: it is the corpus's clearest instance of *sampled, not proved*, and its cheapest instrument was a vocabulary enumeration rather than a model, which the skill already routes as *enumerate, then classify* `[025]` | almost all of it, on purpose. It is an experiment rather than a column: a throwaway module, nine assumption dials, two retained counterexamples about one library's root lifecycle. Its M8 incident — the same command at the same budget returning `[ok]` on one run and not the next — is real and is **not** landed as a rule, because one observation of a tool's nondeterminism at one pinned version is a bug report, not routing evidence |
| 048 | *an independence barrier over two models is not a barrier while their common ancestor is writable* and *a pre-replay census is not a measurement but a pair of self-reports*, both in *What this evidence does not support*; and finding 5's statement-shape result, which is the largest single rule this pass added — see below | its two corrections to `task-tree.als`, the `FN-13` pair and the `doReap` product replay, which are grove's. Its own verdict — *replay is not a comparison step, it is an instrument* — **was first declined and then landed at review**: the ground for declining it was that a reader with one model has no second column, and that ground does not reach a reader who has arrived at the two-formalism comparison route, who by construction holds two. It is now a row of its own in `references/routing-table.md`, conditioned on holding two families |
| *synthesis* | the routing economics in *How much will a model be worth to this work?* — four findings in five yielding no test, so the consuming phase is documentation rather than implementation; the two per-obligation figures with their three narrowings; the ≈ 4.2 h of wrangling; and *run cost tracks the data structure in the model, not the family*. In *Writing a model that can be read*: statement shape, and *a property discharged by a configuration is unfalsifiable at any grain*. In *What this evidence does not support*: the not-a-fair-race caveat and the frozen-ancestor rule. In `references/keeping-a-log.md`: the ledger decay, H6's *score every finding on every measure*, and — evidenced by this log's own producer-only shape rather than by the synthesis — *the sessions that check a model are the ones that stop appending* | H5's verdict as a verdict, H9 and H10, *the combined workflow this subject would actually use*, and *concrete changes to design, tests and documentation*. The first three decide a hypothesis about **these two tools on this one subject**, and the routing table sorts by the shape of the question rather than by a head-to-head; the last two are grove's design work. What was taken from H5 is the axis it identified, not the race it lost |

### What this pass added that no bullet of § 3A asked for

[`results-of-formal-methods-trial.md`](results-of-formal-methods-trial.md) § 3A
proposed seven changes to the skill and this pass landed all seven. It landed
**five more** — the fifth added by the integration of this pass's review — listed
here because an addition outside a proposal is exactly the kind of thing a later
reader should be able to find and dispute. § 3A is a
reading of this log, and a reading is not exhaustive over what it reads.

1. **State a property at the grain of its subject, not at the grain of an
   action** — and *a property discharged by a configuration rather than by an
   observation is unfalsifiable at any grain*. From entry 048 finding 5 by way
   of H5, which the synthesis calls **"this experiment's most reusable result"**
   and demonstrates with the same tool, the same model and only the statement
   shape changed. § 3A does not mention it. Dropping a result the log names as
   its most reusable, silently, would be the failure this table exists to
   prevent.
2. **The starved-witness arithmetic and its search-dial remedy**, from entry
   045: over a twenty-step transaction with every environment action enabled,
   `(1/k)^20`, and every claim past the commit green on a witness that never
   landed. It is a *generator* of empty greens, which is the skill's central
   subject, and the entry quantifies it.
3. **Quote a model-checked result with its depth, and treat the property count
   as cheap** — entry 045 again, 373 s at 3 invariants and depth 3 against 377 s
   at 61 invariants and depth 4, over a protocol whose shortest settled path is
   eleven steps. The pair bounds the marginal cost of properties; it does not
   isolate depth, since it moves depth too and the same model took 445 s when
   three added constants widened its state.
4. **`keeping-a-log.md`'s *the sessions that check a model are the ones that
   stop appending***, evidenced by this log's own shape — see *Entries 026 – 043
   have no rows of their own*, above. It is the one rule here whose evidence is
   the artifact being distilled rather than anything inside it.
5. **Replay as the instrument for a reader holding two families**, from entry
   048's own verdict by way of the synthesis's combined workflow. Added by
   `model-led-development-k98` while integrating `model-led-development-k96`,
   which observed that the ground for declining it — a reader with one model has
   no second column — cannot reach a reader who has arrived at this pass's own
   two-formalism comparison route.

### What did not survive from § 3A's own wording

One thing, and it is a narrowing rather than a refusal. § 3A asks for *the module
sweep rule* — run every module against every claim the model has. That landed,
**with the cost `[c4]` attaches to it and § 3A's bullet mentions only in
passing**: the obligation is to *run the sweep and record what it says*, and the
one repair this corpus costed needed a new constant in every module instantiation
(2 in one file, 37 in another) and was cut as its own task and then abandoned. An
unbounded obligation in a skill is one a session declines silently, so the bound
is part of the rule rather than a caveat on it.

### What this distillation owes Grove — checked, not assumed

[`grove-binds-without-the-plugin`](adr/grove-binds-without-the-plugin.md) asks
one question of every rule that lands in the plugin rather than in `content/`:
*does the plugin's absence change what a Grove session **writes**, or only how
well?* Answered per rule rather than in aggregate, because the ADR's failure mode
is a silent dependency and an aggregate answer is one.

**Every rule this pass landed defers cleanly, and the reason is the same for all
of them.** Each is an obligation on a session that runs a model suite — a control
shown to kill for the reason claimed, a narrowed module's reachability, a claim's
statement grain, a module sweep, a search dial, a ledger instrument, a
per-obligation cost figure, and the replay route added at integration, which
presupposes two model families. **Grove's methodology runs no model suites**, so
there is no Grove session that could violate one, and the plugin's absence
changes nothing a Grove session writes.

Two checks rather than one assertion:

- **`content/` cites this skill nowhere.** Its `linkuistics:` citations are
  `decision-records`, `codebase-design` and `using-jujutsu`, in `ADR-FORMAT.md`,
  `SPEC-FORMAT.md`, `references/grove.md` and `references/commit.md`. So no
  existing citation gains an obligation from this pass.
- **Therefore no row moves.** `docs/specs/corpus-rule-ownership.md`'s plugin
  deferral policy is unchanged, and `tests/plugin_fallback.rs` — which asserts
  both that each citation states what binds and that the enumeration is
  exhaustive — has nothing new to enumerate.

**The one rule from this campaign that *does* bind on Grove already has.** *A
control that has never been seen to fail is not a control* went into
`content/references/execute.md` by way of `methodology-changes-k91`, together
with the provenance discipline and the review-chain repair — see
[`candidate-lessons.md`](candidate-lessons.md) § *What binds, and what only gets
written down*. It is named here so that a reader comparing the two corpora does
not read this section as a claim that the campaign bound nothing.

### What this pass could not verify

Same limit the first pass recorded, and it has not moved: **the skill was not
pressure-tested against a no-skill control.** `linkuistics:authoring-conventions`
exempts a reference and routing guide from that, and this pass is entirely
reference material added to an existing one — but the exemption is a judgement,
not a measurement.

**One limit is this pass's own.** Nothing here was re-measured and no model was
re-run, by instruction: the adjudication in
[`candidate-lessons.md`](candidate-lessons.md) had already re-opened every model
file it cites. So every figure above is quoted, and what stands behind each rule
is a citation rather than an instrument. Where a citation is wrong, the rule
above it is wrong with it, and the three cases in which this tree was found
misreporting itself are enumerated in that document's *Where the tree misreports
itself*.
