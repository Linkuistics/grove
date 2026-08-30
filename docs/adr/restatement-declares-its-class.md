# A restatement in the condition register declares its class

The `grove` **spine** skill's `SKILL.md` is the methodology's **condition
register** — it carries *that a situation exists calling for something other than
what this session is doing* — and every reference file, format file and
`grove-<kind>` skill is a **procedure register**, carrying how to act once a rule
applies. The register was `content/SKILL.md`, inside the binary's embedded
corpus, until `plugin-kind-skills-k17`; the split below is unchanged by the move,
and only the file it names is.

A procedure register holds exactly one file per rule and never mirrors. The
condition register's relationship to a rule is one of **three declared classes**,
and every inventory row states which:

- **`own`** — the spine's `SKILL.md` is the canonical source. Legal only at
  `Occasion = orientation`, where the rule's whole content is its trigger and no
  procedure remains to defer. A procedure file may explain such a rule; none may
  restate it.
- **`trigger`** — **one sentence of at most 25 words**: the situation, a
  single-clause obligation, and the owner file's path. Never a threshold, a
  branch, an enumeration or steps. Required whenever a rule's load predicate
  fires from `SKILL.md`.
- **`none`** — no `SKILL.md` statement at all. Mandatory whenever the rule binds
  one kind or one family; otherwise legal only when the edge into the owner comes
  from some other conditional file.

Two rules may share one sentence **only** when they have the same situation and
the same owner file. Any statement of a rule outside its owner and its declared
class is a defect, and a second statement in a *procedure* file is a defect even
while the two agree.

[Every normative rule has one owner](corpus-rules-have-one-owner.md) decides
*which* file owns a rule; this record decides what, if anything, the condition
register may say about a rule it does not own.
`docs/specs/corpus-rule-ownership.md` carries the per-row classes, the canonical
trigger sentences and the arithmetic they measure to.

## Why one uniform mirror rule does not survive contact

The predecessor of this record treated `SKILL.md`'s relationship to a rule as one
thing — *a mirror is one sentence naming a file, carrying no test, threshold, list
or procedure* — and then, in the same document, permitted `SKILL.md` to carry the
seven-item spine list, the bootstrap order, the one-reviewer allowance, the
integration-placement test and the triage mapping. It forbade and permitted the
same shape, which means it decided nothing.

The requirements independently ask `SKILL.md` to *carry* the bootstrap order,
which a pointer cannot do. So the uniform rule was not merely inconsistent, it was
unsatisfiable: some rules genuinely have no procedure left to defer, and calling
their statement a mirror misdescribes them.

## What the classes buy

**Paraphrase becomes visibly illegal.** "ADRs are raised sparingly" reads as a
condition and *is* a looser test — it is a three-part AND test compressed into an
adverb. Under `trigger`, naming the file is what a sentence is allowed to be, and
the AND/OR contradiction that shipped in the corpus could not have been written.

**Per-kind rules lose their `SKILL.md` statement entirely.** The driver resolves
the kind before the session exists and names that kind's own `grove-<kind>` skill
in `${prompt}`, so the session performs no selection and there is nothing to
trigger it into. That is most of why the spine's `SKILL.md` is short without
anything having been deleted, and it is why the spine carries no list of kinds:
a kind exists **iff** a skill of that name exists.

**The word budget becomes arithmetic over a measured set.** With the classes
counted, `SKILL.md`'s size is the sum of its `own` bodies and its trigger
sentences, and both are writable and countable in advance rather than hoped for.
The predecessor's numbers disagreed with each other precisely because nothing had
been measured: the sentence count was derived by pairing rules that the grammar
forbids pairing.

**Sharing has a test rather than a habit.** Same situation and same owner file is
the whole of it, and it follows from the grammar: a sentence naming two paths needs
an enumeration, and a sentence covering two situations needs a branch. Without the
test, pairing is how a `trigger` grows back into a procedure one comma at a time.

## The residue: the `own` class

`own` concedes that some rules have no procedure to defer — the numbered spine, the
bootstrap order, one task is one session. For those the condition register *is* the
canonical source rather than a mirror of one. That is not a hole in the two-register
split; it is the split's degenerate case, and naming it is what stops `SKILL.md`
from being forbidden to carry a rule the requirements independently require it to
carry.

The class is bounded by its legality condition and not by taste: `Occasion` must be
`orientation`, and no procedure may remain. Rules that merely *feel* foundational —
the doubt budget, the triage mapping, the kind set — fail it, because each retains a
procedure or an enumeration that a reader would have to be handed anyway.

## Considered options

- **Keep the uniform "one sentence naming a file" mirror rule.** Rejected because
  it is the predecessor, and it contradicted itself in its own document while
  leaving a requirement unsatisfiable. Reopen only if the corpus loses every rule
  whose whole content is its trigger, which would make `own` empty and the uniform
  rule true by vacuity.
- **Allow no restatement at all — `SKILL.md` becomes a pure routing table.**
  Rejected because a session cannot look up a rule it does not know applies. The
  `if`/`then` asymmetry [the skill delivers the
  methodology](skill-delivers-the-methodology.md) settles applies here one channel
  further in: a withheld procedure costs a lookup the session knows to make, while
  a withheld *condition* yields an unasked question, which is grove's primary
  failure mode. Reopen if the driver ever resolves the situation as well as the
  kind, which would make the condition a runtime fact rather than a judgement.
- **Make each file self-contained, so a session never needs a second read.**
  Rejected because it *is* what the corpus had, and it produced a six-way
  restatement of one rule and two live contradictions. Self-containment is a real
  property, but it is bought with duplication, and duplication in a corpus nothing
  validates (constraint 3) has no mechanism keeping the copies equal. Reopen only
  if the corpus gains a generator, so one source can *emit* the copies — which
  would make self-containment free rather than merely desirable.
- **Let a `trigger` carry the test when the test is short.** Rejected because
  short is not the property that matters: "raised sparingly" is the shortest
  possible form of the ADR test and it is also the wrongest, and a session that
  reads a compressed test does not know it has read a compressed one. Reopen never
  — this is the specific failure the class exists to prevent.
- **Enforce the classes with in-corpus markers and a build gate**, as the
  mandate's 140 unit markers did. Rejected for the same reason the owner record
  rejects it: a marker can decide whether a partition is total, and whether a page
  of conditions is *right* was never gateable. Enforcement is per rule — a word
  budget over the measured sentence set, a reachability assertion over the edges,
  a phrase-scoped sweep for a distinctive wording.
