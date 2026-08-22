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
| routing itself — "which of the two am I holding?" | count the states the property mentions | 003: one state → structural; two or more → behavioural. Mechanical, and applicable before choosing. Supersedes shape-versus-operation, which mis-sorts "no key is ever reissued" |
| already arranged — "did the model check this, or make it true?" | neither — a free question | 003: subtree preservation is true by construction of the state shape, and a model that satisfies an invariant by construction is indistinguishable from one that verified it |
| did it run at all? — "is this suite green, or dead?" | must-be-reached claims beside the must-hold ones | 003: a JVM too old made Alloy print nothing, which its runner read as thirteen unfired witnesses and seven holding checks. Only the witnesses distinguish the two, and every one failing at once is a signature no real defect produces |
| deriving tests — "what do I actually run against the implementation?" | the model's must-be-reached witnesses | 004: four Alloy witnesses became executable broken-domain tests; not one `check` translated into anything runnable |
| grammar — "can two filenames name one entry?" | property-based testing *(untested)* | 004: both models hold no strings by design, so the largest piece of the implementation had no model coverage at all. The instrument that fits is `format(parse(f)) == f` over generated names, and it was not reached for |
| model-claim-to-test — "does this test still say what the claim said?" | quote the claim's predicate beside the assertion | 005: two tests named a claim and checked a weaker property — `v.seen = n` became string equality, `Malformed` became *not an entry* — and both drifted toward the property that was easier to observe |
| partial discharge — "the language forbids some of this obligation" | split it into two named obligations | 005: the qualification survived in the file that made the claim and in none of the three artifacts that repeated it; the unqualified half was a real defect |
| a written law nobody holds the code against | an adversarial reader briefed to attack one judgement | 005: five of six findings, including both trait-shape defects, which neither model could reach; the models supplied the law and a reader supplied the comparison |
| does this test test anything? — "the suite is green, but was it ever red?" | deliberate mutation of the implementation | 006: six breakages, each firing exactly the expected tests; the only evidence for two claims made in prose (dropping a guard releases the lock, a re-export stays a violation). Minutes, no tooling, any suite |
| order stability — "does this answer depend on the order the input arrived in?" | property-based testing over permutations *(untested)* | 006: `by_key`'s documented tie-break was machine-dependent on a hand-edited tree, because listing order is arbitrary and ordinal distinctness is only preserved. Found by composing two model premises; `walk(shuffle(l)) == walk(l)` is what would have found it directly, and is the second leaf running to name this instrument without using it |
| what do I have to get right unaided? | the model's own recorded misses | 006: the handoff block's *walk order is unmodelled* located every line of undefended prose in the leaf before any code existed. The mirror of 004's *witnesses are the test suite* |
| a case the platform makes untestable — "the branch exists and the host cannot reach it" | assert which fact is true on the host, never skip | 006: APFS refuses non-UTF-8 filenames, so a halting branch is unreachable on macOS; a skipped test reports what a passing one reports |
| universal — "does this hold for all inputs, not just those a checker reached?" | Lean *(untested)* | — |
