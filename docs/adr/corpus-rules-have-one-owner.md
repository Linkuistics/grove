# Every normative rule has one owner

A rule in the methodology is filed by **when a session meets it**, not by what
it is about. The register that holds the rules is the `grove` plugin. That
plugin has a shared **spine** skill and one `grove-<kind>` skill for each kind.
Until `plugin-kind-skills-k17`, the register was the binary's embedded
`content/`. That move did not change the function below. It changed only the
names of its outputs.

Each rule records two facts. `Bound(R)` is the set of session kinds that must
obey it. `Occasion(R)` is a **non-empty set** of moments at which it applies.
The moments come from `orientation`, `launch`, `context`, `step:<loop step>`,
`artifact:<artifact>` and `none`. Placement is then an **ordered** decision, and
the first match wins:

1. `Bound(R) = ∅` or `Occasion(R) = {none}` → the rule is not normative. It
   leaves the skill set.
2. `Bound(R)` is one kind → that kind's own `grove-<kind>` skill. `Bound(R)` is
   one family → the family's single file in the spine. The families are the
   five reviews, the five integrations, the two research halves and the four
   editorial stages. Each member's skill directs a load of that file by name.
3. `artifact:A ∈ Occasion(R)` → the format file of A.
4. `launch ∈ Occasion(R)` → `references/driver.md`.
5. `context ∈ Occasion(R)` → `references/grove.md`.
6. `step:S ∈ Occasion(R)` → the spine's loop-step reference for the **earliest**
   such S in loop order.
7. `Occasion(R) = {orientation}` → the spine's `SKILL.md`.

No other file states R. The order **is** the tie-break: a rule with two
plausible homes has exactly one, and the rule that fires first decides it.

**`Occasion` is a set because real rules cross moments.** A session hands a
question back to a human at Execute, at Retire and at Finish. A record of only
one of the three, with precedence computed from it, moves the ambiguity into the
input instead of ending it. Several artifacts make R one row for each
artifact. Several steps make R one row, and the earliest step owns it. A
session must hold a rule from the first moment that the rule can apply.

The `artifact:` domain is exactly the set of artifacts that have a `*-FORMAT.md`
file, because rule 3 must name a format file and nothing else.

`context` and `launch` are twins. `references/grove.md` and
`references/driver.md` sit on either side of the loop, so no `step:S` reaches
either of them. Without a value of its own, a rule that belongs to one of them
would be a hand assignment that looks like a derivation.

**Reachability is an edge, and the check asserts both of its ends.** A kind's
static path is now its own skill's `SKILL.md` and the spine's `SKILL.md`. The
check reads that path off the shipped bytes, and no module of the binary fixes
it. Every other rule records the file whose sentence triggers it. Where that
file is **not** the owner, it must **actually name the owner's path**. A test
that asks only whether the recorded file can load certifies the failure that it
exists to catch.

**A rule that a condition inside its own owner triggers records no edge.** A
condition in the owner file reaches about half of the conditional rules. The
session has already opened that file, and the record names *which part
applies*, not what sent the session there.

If those records were edges, each one would be a self-loop, and the whole graph
would be cyclic. The acyclicity law would then forbid the ordinary case. The graph is therefore over **cross-file
transitions only**. A same-file record is an in-file condition that
reachability neither needs nor checks.

What the condition register may say about a rule that it does not own is a
separate decision: [a restatement declares its
class](restatement-declares-its-class.md).
`plugins/grove/conformance/rules.tsv` holds the inventory of every normative
rule, with its owner, class, load predicate and test.
`plugins/grove/conformance.sh` enforces it.

## Why filing by topic cannot work

The corpus has four filing axes: loop step, session kind, artifact format and
habit. A rule usually belongs to all of them. Take the placement rule for an
`integrate-review-*` step. It governs Decompose, binds the five `review-*`
kinds, concerns positions among siblings, and is a judgement that a session
makes. Filing by topic sends it to every file that owns one of those axes, and
that happened. Six full statements existed, in `SKILL.md`,
`references/decompose.md`, `driving.md`, `TASK-FORMAT.md`, `CONTEXT.md` and a
spec.

The length is not the failure. The failure is that nothing decides which
statement is right when two disagree, and two of them already disagreed. When
this record was written, `references/design.md` raised an ADR for a decision
that was "hard to reverse, surprising, **or** a real trade-off".
`grilling.md` required all three. `references/requirements.md` stated the
grilling procedure as unconditional in one paragraph. It stated the same
procedure, gated on three questions, in another.

`references/planning.md` stated the working-increments rule **twice in its own
body**, and `references/execute.md` stated it a third time. A file that repeats
itself shows most clearly what an absent placement rule looks like.

## What the pair gives, and what it does not claim

**The consequence is computed, and the input is judged once.** The choice of
`Occasion(R)` is a judgement. It is *one* judgement for each rule, recorded in
the row. A reader can check it against a single question: does a session meet
this rule at that moment? Given the pair, the owner follows with no further
argument.

That is strictly less judgement than filing by topic. Filing by topic demands a
fresh choice among four axes from every session that touches the rule. Nobody
records that choice, and nobody can check it. The pair does not make the
stronger claim that placement needs no judgement at all. `Bound(R)` alone cannot
decide a home. A design that says otherwise ends up with the second input
hidden in it, unstated.

**The delivery boundary stops being a second rule.** A harness installs only
the plugin's skills. A `docs/` path is therefore reachable from a session inside
this repository and from no session anywhere else. The rule *normative material
ships in the skill set* is then the first case of the function, read backwards.
A rule may move to `docs/` if and only if its `Bound` is empty or its
`Occasion` is `none`. There is no separate boundary to remember, and nobody
judges whether a given move crosses it.

**Reachability becomes checkable instead of assumed.** A kind's static path is
its own `grove-<kind>` skill's `SKILL.md` and the spine's `SKILL.md`. Nothing
else can be static. Every other rule states the file whose sentence triggers
it. The **cross-file** records form a graph of edges, and each chain must end at
a static path.

That graph is the **composed loaded path**. `plugins/grove/conformance.sh`
computes it as the closure of what the shipped files actually name. A rule whose
chain does not reach a static path ships, but it is deleted in effect. When this
record was written, `content/driving.md` did that to two disciplines of `impl`:
source citation and repository claims. They were in a file that no `impl`
session ever reached.

**The edge is the assertion, not the file.** A record of *the file whose
sentence triggers R* has value only when a check finds that sentence. A row can name a loadable file that says nothing about R. That row
passes a loadability test and leaves R unreachable. A file can also own rows
while nothing anywhere points at it.

The graph therefore asserts two things. The source names the owner's path, and
every owner that is not static has an incoming edge. **The file carries the
property, not the rule.** Once a session has opened the owner, it has reached
every rule in that file. That is why an in-file record asserts nothing, and why
the check of incoming edges is stated for each owner file instead of each row.

## The residue, named instead of argued away

A session sometimes opens a second file where one self-contained file would
have been enough. That is the trade, and the corpus pays it deliberately. The
alternative is the earlier corpus, which avoided the second read and let the
two copies drift.

The `if`/`then` asymmetry applies here one channel further in. A withheld
procedure costs a lookup that the session knows to make. A withheld *condition*
yields a question that nobody asks. So conditions may be restated, and
procedures may not. That is exactly the asymmetry, applied to files instead of
channels. [A restatement declares its class](restatement-declares-its-class.md)
is where that permission has its limit.

That record decides what crosses the `${prompt}`/skill boundary under the
too-late test. This record decides what goes where **inside** the corpus. The
two records share the asymmetry and nothing else, and different conditions
reopen them.

The second residue is that a reader judges `Occasion(R)`, and nothing derives
it. A reader can check it only by asking whether a session really meets R at
that moment. A set narrows what a wrong answer can be. A rule that crosses
moments records all of them, so the failure is a *missing* member, not a silent
arbitrary choice. A rule with no honest occasion at all could still be placed
with a dishonest one. Nothing detects that, and nothing claims to.

## Considered options

- **Take `Bound(R)` alone, and map "every kind" to the loop-step reference for
  the step that the rule governs.** The option read *all nineteen kinds* while
  the binary held a set of nineteen to quantify over. A kind exists **if and
  only if** a skill of that name exists, so nothing holds that set now. The
  option is restated over the quantifier that remains. It is rejected because
  it is not a function of its stated input.

  *Which step does it govern* is a second input. Leaving it unstated causes two
  failures, not one. The rule reads as free of judgement while it still needs a
  judgement. The judgement is not recorded, so two readers file the same rule
  differently and neither can be shown wrong. The option also mislabels the
  load predicate.

  Read literally, "the narrowest file every bound session already opens" sends
  every rule for every kind to `SKILL.md`, because loop-step references are on
  no static path. Reopen it only if the spine's `SKILL.md` ever states the
  loop-step procedures itself. That would put them on every static path, make
  `Bound` sufficient and make rules 4 to 7 of this record redundant.
- **Keep `Occasion(R)` single-valued, and record the moment at which a rule
  *mainly* applies.** It is rejected because "mainly" is the unrecorded
  judgement that this record exists to end, one layer down. A rule triggered
  at three steps has no principal one. Two readers choose differently, and
  neither can be shown wrong. The set form costs a tie-break rule and gives a
  checkable input. Reopen it if a rule appears whose occasions are true
  alternatives instead of a conjunction. The set would then be the wrong
  structure, not merely an under-specified one.
- **Enforce ownership with markers in the corpus and a gate**, as the 140 unit
  markers and the build gate of the mandate did. It is rejected. That machinery
  was scaffolding for the delivery rewrite. It did its work, and it was deleted
  with the classification that it enforced. A marker can decide whether a
  partition is total. Whether a page of conditions is *right* was never
  something a gate could decide, and the inventory's own record admits it.

  Enforcement is therefore for each rule, by the instrument that fits it. A
  behavioural evaluation fits conduct, and a sweep scoped to a phrase fits
  distinctive wording. A budget fits a loaded path, and a reachability
  assertion fits the `@` chains. Reopen it only if someone shows a checkable
  property that a universal parser could decide and these instruments cannot.
- **File by topic, and name an owner for each topic by hand.** It is rejected
  because it answers the symptom and leaves the cause. A rule with four topics
  still needs a judgement for each rule. Every future session makes that
  judgement again, with no way to check the answer. Reopen never. A
  hand-maintained owner list is what the inventory would become if the
  function were dropped.
- **Let the specs under `docs/` hold the normative statements, and let the skill
  set cite them.** It is rejected because it deletes the rules instead of
  moving them. Nothing installs `docs/`, so a session on any other repository
  cannot open the citation. That is the failure that the delivery constraint
  exists to name, and the placement function makes it unreachable by
  construction. Reopen it only if `docs/` ever ships inside the plugin. That
  would make it a second register and raise a different question.
- **Split the corpus by audience, with one tree for sessions and one for
  contributors, and drop the four separations.** It is rejected as the same
  decision under a coarser name. A non-empty `Bound` and an empty `Bound`
  already tell sessions and contributors apart. The four separations fall out
  of the function, and they are not an extra scheme laid over it. Reopen it if a
  third audience appears that is neither.
- **Keep `content/driving.md` as an embedded file of habits.** It is rejected on
  the function's output, not on the file's size. Every rule in it has a
  narrower owner under rules 2 to 6. Two of them, the source-citation and
  repository-claim disciplines of `impl`, sit in a file that no `impl` session
  ever reaches. That is the reachability failure inside `content/` instead of at
  its edge. A file whose bytes are mostly on no session's path is the shape that
  the *loaded path* measure exists to make visible.

  **Its deletion is conditional, not immediate.** The relocation table of the
  spec names every remaining imperative and its new owner. The file goes only
  when each of those is stated in its new owner, and each `SKILL.md` sentence
  that points at it points elsewhere. Reopen it if someone finds a rule whose
  owner is truly a file of habits. That would mean the function produced one,
  not that the file was kept.
