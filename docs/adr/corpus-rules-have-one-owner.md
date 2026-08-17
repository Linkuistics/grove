# Every normative rule has one owner

A rule in the embedded corpus is filed by **when a session meets it**, not by
what it is about. Two facts are recorded per rule: `Bound(R)`, the set of session
kinds that must obey it, and `Occasion(R)`, a **non-empty set** of moments at
which it applies, drawn from `orientation`, `launch`, `context`,
`step:<loop step>`, `artifact:<artifact>`, and `none`. Placement is then an
**ordered** decision, first match winning:

1. `Bound(R) = ∅` or `Occasion(R) = {none}` → not normative; leaves `content/`.
2. `Bound(R)` is one kind or one family → that kind reference.
3. `artifact:A ∈ Occasion(R)` → A's format file.
4. `launch ∈ Occasion(R)` → `references/driver.md`.
5. `context ∈ Occasion(R)` → `references/grove.md`.
6. `step:S ∈ Occasion(R)` → the loop-step reference for the **earliest** such S
   in loop order.
7. `Occasion(R) = {orientation}` → `content/SKILL.md`.

No other file states R, and the ordering **is** the tie-break: a rule with two
plausible homes has exactly one, decided by which rule fires first.

**`Occasion` is a set because real rules cross moments.** Handing a question back
to a human happens at Execute, Retire and Finish; recording one of the three and
letting precedence compute from it moves the ambiguity into the input instead of
removing it. Several artifacts make R one row per artifact; several steps make R one
row owned by the earliest, because a rule must be held from the first moment it can
apply. The `artifact:` domain is exactly the artifacts a `*-FORMAT.md` file exists
for, since rule 3 has a format file to name and nothing else. `context` and `launch`
are twins: `references/grove.md` and `references/driver.md` sit either side of the
loop, so no `step:S` reaches either, and without a value of its own a rule belonging
to one of them is a hand-assignment wearing a derivation's clothes.

**Reachability is an edge, and both ends are asserted.** `src/prompt.rs` fixes
each kind's static path; every other rule records the file whose sentence triggers
it, and where that file is **not** the owner it must **actually name the owner's
path**. A test that asks only whether the recorded file can be loaded certifies
the failure it exists to catch.

**A rule triggered from inside its own owner records no edge.** Roughly half the
conditional rules are reached by a condition in the file that owns them — the
session has already opened it, and what the record names is *which part applies*,
not what sent it there. Reading those as edges makes every one of them a self-loop
and the graph uniformly cyclic, so the acyclicity law would forbid the ordinary
case. The graph is therefore over **cross-file transitions only**, and a
same-file record is an in-file condition that reachability neither needs nor
checks.

What the condition register may say about a rule it does not own is a separate
decision: [a restatement declares its
class](restatement-declares-its-class.md). `docs/specs/corpus-rule-ownership.md`
carries this function's derivation, the inventory of every normative rule with its
owner, class, load predicate and test, and the per-rule enforcement.

## Why filing by topic cannot work

The corpus has four filing axes — loop step, session kind, artifact format, and
habit — and a rule usually belongs to all of them. The placement rule for an
`integrate-review-*` step governs Decompose, binds the five `review-*` kinds,
concerns positions among siblings, and is a judgement a session exercises. Filed
by topic it goes to every file owning one of those axes, and it did: six
full-length statements across `SKILL.md`, `references/decompose.md`,
`driving.md`, `TASK-FORMAT.md`, `CONTEXT.md` and a spec.

The failure is not that this is long. It is that nothing decides which one is
right when they disagree, and two of them already did. As the corpus stood when
this record was written, `references/design.md` raised an ADR for a decision
"hard to reverse, surprising, **or** a real trade-off" while `grilling.md`
required all three; `references/requirements.md` stated the grilling procedure
as unconditional in one paragraph and as three-questions-gated in another; and
`references/planning.md` stated the working-increments rule **twice in its own
body**, with `references/execute.md` stating it a third time. A file that
duplicates itself is what an absent placement rule looks like at its clearest.

## What the pair buys, and what it does not claim

**The consequence is computed; the input is judged once.** Choosing
`Occasion(R)` is a judgement — but it is *one* judgement per rule, recorded in
the row, and checkable by a reader against a single question: does a session
actually meet this at that moment? Given the pair, the owner follows with no
further argument. That is strictly less judgement than filing by topic, which
demands a fresh, unrecorded and uncheckable choice among four axes from every
session that touches the rule. It is not the stronger claim that placement needs
no judgement at all; `Bound(R)` alone cannot decide a home, and a design that
says otherwise ends up smuggling the second input in unstated.

**The embedding boundary stops being a second rule.** Only `content/` is
provisioned into harness skill directories, so a `docs/` path is reachable to a
session inside this repository and to no session anywhere else. *Normative
material stays embedded* is then the function's first case read backwards: a rule
may move to `docs/` iff its `Bound` is empty or its `Occasion` is `none`. There
is no separate boundary to remember and no judgement about whether a given move
crosses it.

**Reachability becomes checkable rather than assumed.** `src/prompt.rs` fixes
each kind's static path — the guaranteed core, `SKILL.md`, and
`reference_file(kind)` — and nothing else can be static. Every other rule states
the file whose sentence triggers it, and the **cross-file** ones form a graph of
edges that must terminate at a static path. A rule whose chain does not terminate is
present in `content/` and deleted in effect, which is what `driving.md` does today
to `impl`'s source-citation and repo-claim disciplines: they sit in a file no
`impl` session is ever routed to.

**The edge is the assertion, not the file.** Recording *the file whose sentence
triggers R* is only worth something if that sentence is checked to exist: a row
naming a loadable file that says nothing about R passes a loadability test while
leaving R unreachable, and a file may own rows while nothing anywhere points at it.
So the graph asserts that the source names the owner's path, and that every
non-static owner has an incoming edge. **What carries the property is the file, not
the rule**: once a session has opened the owner, every rule in it is reached, which
is why an in-file record asserts nothing and why the incoming-edge check is stated
per owner file rather than per row.

## The residue, named rather than argued away

A session sometimes opens a second file where a self-contained one would have
sufficed. That is the trade, and it is paid deliberately: the alternative is what
the corpus has now, where the second read is avoided and the two copies drift.
The `if`/`then` asymmetry [the skill delivers the
methodology](skill-delivers-the-methodology.md) settles applies here one channel
further in — a withheld procedure costs a lookup the session knows to make, while
a withheld *condition* yields an unasked question. So conditions may be restated
and procedures may not, which is exactly the asymmetry, applied to files instead
of channels; [a restatement declares its
class](restatement-declares-its-class.md) is where that permission is bounded.

That record decides what crosses the `${prompt}`/skill boundary under the
too-late test; this one decides what goes where **inside** the corpus. They share
the asymmetry and nothing else, and they reopen on different conditions.

The second residue is that `Occasion(R)` is judged rather than derived, and a
reader can only check it by asking whether a session really meets R at that moment.
Recording it as a set narrows what a wrong answer can be — a rule that crosses
moments records all of them, so the failure mode is a *missing* member rather than a
silently arbitrary choice — but a rule with no honest occasion at all would still be
placeable by writing a dishonest one. Nothing detects that, and nothing pretends to.

## Considered options

- **Take `Bound(R)` alone, mapping "all nineteen kinds" to the loop-step
  reference for the step the rule governs.** Rejected because it is not a
  function of its stated input. *Which step does it govern* is a second input,
  and leaving it unstated produces two failures rather than one: the rule reads
  as judgement-free while still requiring a judgement, and the judgement is
  unrecorded, so two readers file the same rule differently and neither can be
  shown wrong. It also mislabels the load predicate — read literally, "the
  narrowest file every bound session already opens" sends every all-nineteen rule
  to `SKILL.md`, because loop-step references are not on any static path. Reopen
  only if `src/prompt.rs` ever puts the loop-step references on the static path,
  which would make `Bound` sufficient and this record's rules 4–7 redundant.
- **Keep `Occasion(R)` single-valued and record the moment a rule *mainly*
  applies.** Rejected because "mainly" is the unrecorded judgement this record
  exists to remove, one layer down: a rule triggered at three steps has no
  principal one, two readers pick differently, and neither can be shown wrong. The
  set form costs a tie-break rule and buys a checkable input. Reopen if a rule
  appears whose occasions are genuinely alternatives rather than a conjunction,
  which would make the set the wrong structure rather than an under-specified one.
- **Enforce ownership with in-corpus markers and a gate**, as the mandate's 140
  unit markers and build gate did. Rejected: that machinery was scaffolding for
  the delivery rewrite, it did its work, and it was deleted along with the
  classification it enforced. What a marker can decide is whether a partition is
  total; whether a page of conditions is *right* was never gateable, and the
  inventory's own record concedes as much. Enforcement is therefore per rule, by
  the instrument that fits it — a behavioural eval for conduct, a phrase-scoped
  sweep for distinctive wording, a budget for a loaded path, a reachability
  assertion over the `@` chains. Reopen only if someone shows a checkable
  property a universal parser could decide that these cannot.
- **File by topic and nominate an owner per topic by hand.** Rejected because it
  answers the symptom and leaves the cause: a rule with four topics still needs a
  judgement per rule, made again by every future session, with no way to check
  the answer. Reopen never — a hand-maintained owner list is what the inventory
  would degrade into if the function were dropped.
- **Let the specs under `docs/` hold the normative statements and have
  `content/` cite them.** Rejected because it deletes the rules rather than
  rehoming them: `docs/` is not provisioned, so a session on any other repository
  cannot open the citation. This is the failure mode the embedding constraint
  exists to name, and the placement function makes it unreachable by
  construction. Reopen only if `docs/` ever ships inside the embed, which would
  make it a second `content/` and raise a different question.
- **Split the corpus by audience — one tree for sessions, one for
  contributors — and drop the four separations.** Rejected as the same decision
  under a coarser name: sessions and contributors are what a non-empty and an
  empty `Bound` already distinguish, and the four separations are what fall out
  of the function rather than an extra scheme laid over it. Reopen if a third
  audience appears that is neither.
- **Keep `content/driving.md` as an embedded habits file.** Rejected on the
  function's output rather than on its size: every rule in it has a narrower
  owner under rules 2–6, and two of them (`impl`'s source-citation and repo-claim
  disciplines) sit in a file no `impl` session is ever routed to, which is the
  reachability failure inside `content/` rather than at its edge. A file most of
  whose bytes no session's path reaches is the shape the *loaded path* measure
  exists to make visible. **Its deletion is conditional, not immediate**: the
  spec's relocation table names every surviving imperative and its new owner, and
  the file goes only once each one is stated there and each `SKILL.md` sentence
  pointing at it has been repointed. Reopen if a rule is found whose owner is
  genuinely a habits file — which would mean the function had produced one, not
  that the file was kept.
