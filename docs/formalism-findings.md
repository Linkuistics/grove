# Formalism findings

A running log of what using formal models to drive design and implementation
actually costs and actually buys. It is the raw material for a `linkuistics`
skill on choosing and using a formalism; it is **not** that skill, and it should
not be written as if it were.

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

- **H1 — the split is real.** Structural questions ("is this shape coherent?")
  and behavioural ones ("does this operation preserve it?") want different
  tools, and a practitioner can tell which they are holding *before* choosing.
- **H2 — the model leads profitably.** Writing the specification before the
  implementation produces better code than writing it after, and the
  disagreements between the two are worth more than the time they cost.
- **H3 — a specification can drive implementation.** An LLM given a checked
  formal model produces a more faithful implementation than one given prose of
  equivalent length. Untested and, on current evidence, the least certain.

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

### Routing table (under construction)

Filled in from the entries above as evidence accumulates. Empty rows are honest;
guesses are not.

| kind of question | formalism | evidence |
|---|---|---|
| structural — "is this shape coherent, and can it even represent what I need?" | Alloy | 002: found two defects unprompted, sharpened two more, produced a tree satisfying every stated invariant with a subtree invisible to every traversal |
| reachability — "is every thing I must name reachable from the constructors I have?" | Alloy | 002: both blocking defects across entries 001 and 002 were this question; it is the one Alloy answers best |
| already guaranteed — "can the target language forbid this outright?" | none — check first | 002: two of eight structural claims were free from Rust's type system; modelling them taught nothing |
| stated-isomorphism — "are both directions written down?" | none — a free question | 002: the canonicity gap was half an isomorphism, and asking costs nothing |
| behavioural — "does this operation preserve the invariant, from any reachable state?" | Quint | 003: confirmed. All three invariants Alloy could not state were checkable, and six further defects came out of the same file |
| interruption — "what does a crash halfway through leave behind?" | Quint | 003: the ordering rule's real payoff, and `promote`'s unavoidable transient duplicate, are both invisible to every other method in this log |
| ordering — "does the order of these steps matter, and why?" | Quint | 003: the stated reason was wrong and the real one was unwritten; a model that stops halfway is the only thing that separates them |
| dead branch — "is this refusal reachable at all?" | either, via a reachability witness | 003: the occupancy refusal fires in 0% of traces on any tree the library builds. Sampled, so evidence and not proof |
| routing itself — "which of the two am I holding?" | count the states the property mentions | 003: one state → structural; two or more → behavioural. Mechanical, and applicable before choosing. Supersedes shape-versus-operation, which mis-sorts "no key is ever reissued" |
| already arranged — "did the model check this, or make it true?" | neither — a free question | 003: subtree preservation is true by construction of the state shape, and a model that satisfies an invariant by construction is indistinguishable from one that verified it |
| did it run at all? — "is this suite green, or dead?" | must-be-reached claims beside the must-hold ones | 003: a JVM too old made Alloy print nothing, which its runner read as thirteen unfired witnesses and seven holding checks. Only the witnesses distinguish the two, and every one failing at once is a signature no real defect produces |
| universal — "does this hold for all inputs, not just those a checker reached?" | Lean *(untested)* | — |
