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

## Distillation — where each entry landed

`formalism-skill-k38` turned this log into
`plugins/linkuistics/skills/model-led-development/`. The log stays as it is: it
is the evidence, and the skill's citation keys (`[003]`, `[017]`, …) are these
entry numbers, so a reader who doubts a rule can come back here and check it.

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
Three rules, fixed now:

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
