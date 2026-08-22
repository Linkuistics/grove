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
which names `Report::paths()`'s contract, a thing no model holds. Counted by
reading, as the `crate-k7` brief requires; a sum of two labels cannot see the
third.

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

- **The asymmetry that favours arm A, stated in the leaf and confirmed.** Arm A
  ran under a diffuse awareness that a model-versus-prose experiment existed
  somewhere in the subtree (the `crate-k7` brief says so without naming the
  probe); arm B ran under none. More decisively, **the measure is arm A's own
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
| out of scope — "the model says it excludes this; what does that oblige?" | none — the exclusion list is the worklist | 011: three of five interpreter defects sat in the three domains `operations.qnt`'s handoff names as excluded (strings, bytes, the filesystem). The exclusions were written two leaves before the defects; reading them as a list of what prose must now state would have caught all three at introduction |
| enforceable obligation — "the type system cannot forbid this, but can the library check it?" | none — ask before writing it down as an assumption | 011: six of seven `EntryName` obligations are genuinely uncheckable; the seventh costs one string test at two boundaries and turns an escape from the locked tree into a refusal with recovery advice. 002's counterfactual asks whether the *language* forbids it; this is the question after that one |
| already arranged — "did the model check this, or make it true?" | neither — a free question | 003: subtree preservation is true by construction of the state shape, and a model that satisfies an invariant by construction is indistinguishable from one that verified it |
| a message derived from a model — "may this refusal say what it says?" | none — name the carried value behind each clause | 013: `wit_insertIntoAGap`'s predicate `a.at < maxOrdIn` discriminates the case and does not characterise it; the comment beside it did, and the refusal's message transcribed the comment. One clause, no value behind it, one wrong error message for every hand-edited level with a leading hole |
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
| universal — "does this hold for all inputs, not just those a checker reached?" | Lean *(untested)* | — |
| model or prose at the implementation site? | ask instead whether the prose has already been through a model | 017: the pre-registered probe's prose arm shifted highest-first for the right reason, delegated the sequential destination check correctly, and refused the gap with a carried span — violating zero model claims. Every point it got right is one `operations.qnt` had already corrected in `ARCHITECTURE.md` (003, 013). H3 as stated is not supported; a model spent upstream and banked in the document is |
| a suite as a measure of a second implementation — "can the other arm fail this?" | have it written by a session that implements neither, from the claims alone | 017: fixing the *claims* before both arms was not enough. All four failures landed on literal message substrings the scoring arm had authored; the `assert_eq!` on each refusal's value passed. A test is a claim plus an assertion, and only the first predated the arms |
