# Every normative rule has one owner, and a mirror is a condition only

A rule in the embedded corpus is filed by its **load predicate**, not by its
topic. Let `Bound(R)` be the set of session kinds that must obey R; R's canonical
source is the narrowest file every session in `Bound(R)` already opens, and no
other file states R.

The only permitted restatement is a **condition** in `content/SKILL.md` — one
sentence saying that a situation exists calling for something other than what
this session is doing, and naming the file with the procedure. A condition may
not carry a test, a threshold, a list, or a procedure. Any third statement of a
rule is a defect, and a second statement in a *procedure* file is a defect even
while the two agree.

`docs/specs/corpus-rule-ownership.md` carries the placement function, the
inventory of every normative rule with its owner, and the per-rule enforcement.

## Why filing by topic cannot work

The corpus has four filing axes — loop step, session kind, artifact format, and
habit — and a rule usually belongs to all of them. The placement rule for an
`integrate-review-*` step governs Decompose, binds the five `review-*` kinds,
concerns positions among siblings, and is a judgement a session exercises. Filed
by topic it goes to every file owning one of those axes, and it did: six
full-length statements across `SKILL.md`, `references/decompose.md`,
`driving.md`, `TASK-FORMAT.md`, `CONTEXT.md` and a spec.

The failure is not that this is long. It is that nothing decides which one is
right when they disagree, and two of them already do. `references/design.md`
raises an ADR for a decision "hard to reverse, surprising, **or** a real
trade-off" while `grilling.md` requires all three; `references/requirements.md`
states the grilling procedure as unconditional in one paragraph and as
three-questions-gated in another. `references/planning.md` states the
working-increments rule **twice in its own body**, with `references/execute.md`
stating it a third time. A file that duplicates itself is what an absent
placement rule looks like at its clearest.

Load predicate is the axis that has one answer, because it is a fact about the
runtime rather than a judgement about subject matter. `src/prompt.rs` fixes each
kind's static path — the guaranteed core, `SKILL.md`, and `reference_file(kind)`
— so "the narrowest file every bound session opens" is computed, not argued.

## What the rule buys beyond de-duplication

**The embedding boundary stops being a second rule.** Only `content/` is
provisioned into harness skill directories, so a `docs/` path is reachable to a
session inside this repository and to no session anywhere else. *Normative
material stays embedded* is then the placement function's fourth case read
backwards: a rule may move to `docs/` iff `Bound(R)` is empty. There is no
separate boundary to remember and no judgement about whether a given move
crosses it.

**Per-kind rules lose their `SKILL.md` mirror entirely.** The driver resolves the
kind before the session exists and names that kind's reference file in
`${prompt}`, so the session performs no selection and there is nothing to trigger
it into. This is most of what `SKILL.md` carries today, and it is why the file
shrinks without anything being deleted from the corpus.

**Paraphrase becomes visibly illegal.** "ADRs are raised sparingly" reads as a
condition and is a *looser test*. Naming the file instead is what a mirror is
allowed to be, and it is what would have prevented the AND/OR split.

## The residue, named rather than argued away

A session sometimes opens a second file where a self-contained one would have
sufficed. That is the trade, and it is paid deliberately: the alternative is what
the corpus has now, where the second read is avoided and the two copies drift.
The `if`/`then` asymmetry [the skill delivers the
methodology](skill-delivers-the-methodology.md) settles applies here one channel
further in — a withheld procedure costs a lookup the session knows to make, while
a withheld *condition* yields an unasked question. So conditions may be mirrored
and procedures may not, which is exactly the asymmetry, applied to files instead
of channels.

That record decides what crosses the `${prompt}`/skill boundary under the
too-late test; this one decides what goes where **inside** the corpus. They share
the asymmetry and nothing else, and they reopen on different conditions.

## Considered options

- **Make each file self-contained, so a session never needs a second read.**
  Rejected because it *is* the current design and it produced the six-way
  restatement and both contradictions. Self-containment is a real property, but
  it is bought with duplication, and duplication in a corpus nothing validates
  (constraint 3) has no mechanism that keeps the copies equal. Reopen only if the
  corpus gains a generator, so that one source can *emit* the copies — which
  would make self-containment free rather than merely desirable.
- **Enforce ownership with in-corpus markers and a gate**, as the mandate's 140
  unit markers and build gate did. Rejected: that machinery was scaffolding for
  the delivery rewrite, it did its work, and it was deleted along with the
  classification it enforced. What a marker can decide is whether a partition is
  total; whether a page of conditions is *right* was never gateable, and the
  inventory's own record concedes as much. Enforcement is therefore per rule, by
  the instrument that fits it — a behavioural eval for conduct, a phrase-scoped
  sweep for distinctive wording, a budget for a loaded path. Reopen only if
  someone shows a checkable property a universal parser could decide that these
  cannot.
- **File by topic and nominate an owner per topic by hand.** Rejected because it
  answers the symptom and leaves the cause: a rule with four topics still needs a
  judgement per rule, made again by every future session, with no way to check
  the answer. The load predicate is the axis on which the question has one
  computable answer. Reopen never — a hand-maintained owner list is what the
  inventory would degrade into if the function were dropped.
- **Let the specs under `docs/` hold the normative statements and have
  `content/` cite them.** Rejected because it deletes the rules rather than
  rehoming them: `docs/` is not provisioned, so a session on any other repository
  cannot open the citation. This is the failure mode the embedding constraint
  exists to name, and the placement function makes it unreachable by
  construction. Reopen only if `docs/` ever ships inside the embed, which would
  make it a second `content/` and raise a different question.
- **Split the corpus by audience — one tree for sessions, one for
  contributors — and drop the four separations.** Rejected as the same decision
  under a coarser name: sessions and contributors are what `Bound(R)` non-empty
  and `Bound(R)` empty already distinguish, and the four separations are what
  fall out of the function rather than an extra scheme laid over it. Reopen if a
  third audience appears that is neither.
- **Keep `content/driving.md` as an embedded habits file.** Rejected on the
  function's own output rather than on its size: every normative rule in it has
  an owner elsewhere, two of them (`impl`'s source-citation and repo-claim
  disciplines) sit in a file no `impl` session is ever routed to, and what remains
  has `Bound(R)` empty. A file most of whose bytes no session's path reaches is
  the shape the *loaded path* measure exists to make visible. Reopen if a rule is
  found whose Bound is non-empty and whose narrowest owner is genuinely a habits
  file — which would mean the function had produced it, not that the file was
  kept.
