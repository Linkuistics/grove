# formalism-skill-k38

## Goal

Distil `docs/formalism-findings.md` into a `linkuistics` skill: **which formalism
suits which question, and how a checked model drives an implementation**. This is
the second experiment's deliverable, and the root brief is explicit that it is a
deliverable rather than a by-product.

It runs last in the whole grove, after every modelling and implementation leaf,
which is why it sits at the root after the `grove-flip-k28` node rather than
inside it. `pick` descends a node in place, so the flip is fully explored before
this leaf is ever returned.

## Context

- `docs/formalism-findings.md` — the corpus. Entries 004–019 came out of
  `crate-k7`; the flip appends whatever it appends. The load-bearing field is the
  **counterfactual**: what would have caught this earlier or more cheaply.
- The root brief's *Findings accumulate; the skill distils* — the argument for why
  this artifact could not be written continuously, and therefore what it is for.
- `docs/ordinal-fs-tree/models/` — `structure.als`, `operations.qnt`, and the two
  runners `run-alloy.sh` / `run-quint.sh`. Both runners exist because both tools
  report *found nothing* with exit code 0, and that cost was paid twice. That is
  itself one of the skill's lessons.
- `docs/ordinal-fs-tree/ARCHITECTURE.md`, *The models* and *What the behavioural
  model adds* — including what each model records that it does **not** establish.
- `plugins/linkuistics/skills/` — the plugin this skill joins, and its existing
  members for shape and voice: `decision-records`, `codebase-design`,
  `doubt-driven-development`, `authoring-conventions`.
- The `superpowers:writing-skills` skill, for how a skill is built and verified.

## Done when

- A skill exists under `plugins/linkuistics/skills/`, with the description written
  so it triggers on the questions it answers rather than on the tool names.
- It covers both halves the root brief names: **choosing** a formalism per
  question — structural questions go to Alloy, which finds counterexample
  *structures*; behavioural questions go to Quint, which finds counterexample
  *traces* — and **using** a checked model to lead an implementation, including
  the rule that where a model and a test disagree the model wins and the test
  changes.
- Every claim in it is traceable to at least one findings entry. A claim with no
  entry behind it is a generalisation from nothing, which is precisely what the
  accumulate-then-distil schedule exists to prevent.
- The findings entries that did **not** make it into the skill are accounted for —
  a one-line note on why is enough. Silent omission and considered omission look
  identical afterwards.
- The root brief's *Done when* holds for the method half of the workstream.

## Notes

**Why this leaf was cut before its inputs existed.** The fog-or-ticket test asks
whether the question can be stated precisely *now*, not whether it can be
answered. It can: the root brief's own sentence states it. The root brief had
kept it on the horizon because its scope depended on how many formalisms were used
and what they taught — and `crate-k7` answered that: two formalisms, sixteen
entries, both models re-runnable and both recording their own misses. What
increment 2 can still add is entries, not a third formalism, so the scope is now
knowable and the leaf is a leaf. `references/decompose.md` is explicit that a
precisely-stateable question earns a leaf immediately, even blocked and
unanswerable today.

**Two lessons the corpus carries that a reader of the models alone would miss**,
and which are the sort of thing this skill exists to hold:

- **A model whose result cannot be read is not a checked model.** Both tools
  report *found nothing* with exit code 0. Both runners exist for that reason, and
  the cost was paid twice before it was noticed.
- **Read what a model says it does not establish, before trusting it.** Two of
  these matter downstream even now: a rename carrying its subtree is assumed
  rather than checked, and walk *order* is unmodelled, so `by_key`'s tie-break on
  a duplicate-key tree rests on prose.

**The counterfactual field is the skill's spine.** An entry that says what went
wrong is a war story; an entry that says what would have caught it earlier or more
cheaply is a rule. Prefer the entries that have one, and where the skill states a
rule, it should be a rule some counterfactual actually asked for.

**Prose survives alongside models, with a demoted job** — a checked model
guarantees consistency with itself and never that the right properties were
stated. That is one of the findings and it is also this skill's own honesty
clause: the skill is prose about how to write specifications, and nothing checks
it.
