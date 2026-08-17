# skill-router-k20

**Reviews:** skill-router-k4

## Goal

Adversarially read the rewritten `content/SKILL.md` — 3,152 words down to 796 —
and the four rule statements that rewrite pushed into `references/commit.md` and
`references/retire.md`. Findings only; the fixes belong to an integration step
you cut only if you have findings worth acting on.

## The specific doubt

**Which retained rule got quietly weakened into a pointer?** A 75% reduction of
the file every session reads is exactly the artifact a review chain exists for,
and compression is the failure mode: a rule survives *by name* in a 12-word
trigger while the obligation it carried does not survive at all. The register's
own grammar makes this hard to see — a trigger is *supposed* to carry no test,
threshold or branch — so the question is not whether a sentence is short but
whether a session holding only that sentence would still **ask**.

Read each of the 26 trigger sentences against the inventory row it stands for
(`docs/specs/corpus-rule-ownership.md`, *The trigger sentences*, and the row's
own text in the inventory), and against the superseded `SKILL.md` in this leaf's
producer commit. Three shapes to hunt:

- a sentence whose **situation is narrower than the rule's** — it fires for some
  sessions that meet the rule and not others, so the rest never look;
- a sentence naming an owner file that **does not state the rule**, which is the
  reachability failure the design exists to prevent, not a wording quibble
  (`grep` the owner; several rules still sit in `driving.md` pending
  `loop-step-references-k11` and `corpus-split-k6`, which is expected — what is
  *not* expected is a rule stated nowhere in `content/`);
- an **`own` row that lost a clause**: the spine's *just-in-time, not few*, the
  HITL mark's *predicts, does not permit*, the bootstrap order's *nothing else by
  reflex*, `no-second-pick`'s *the mandate wins*.

## Also in scope

- The four rules this leaf moved because `SKILL.md` was their only statement in
  `content/` — `one-focused-commit`'s scope and the Retire-first reason
  (`references/commit.md`), `node-close-is-implicit` with *the close asks the
  human nothing*, and `pruning-is-hitl`'s *an agent never prunes on its own*
  (`references/retire.md`). Did each land whole, and did any of them import the
  **condition** into a procedure register? The spec's new paragraph on that is
  itself reviewable.
- The budget assertion in `tests/methodology.rs`. Is it the check the design
  asked for — a 900-word ceiling, exactly 26 triggers, each ≤25 words, eight
  `own` rows by name — or can it pass on a file that dropped a rule? Its
  `OWN_ROWS` phrases are the part most likely to be either too loose or so tight
  that a legitimate rewording fails.
- The five test files this leaf repointed (`commit_guidance`,
  `composition_guidance`, `lifecycle_invariants`, `prompt`, `retire_guidance`).
  Each moved a claim from `SKILL.md` to the file that now owns the rule.
  Did any of them get *weakened* in the move rather than relocated?

## Out of scope

`content/SIGNAL.md` and `content/SIGNAL-FINISH.md` (byte-frozen), `src/prompt.rs`,
and the rules later leaves will move — a rule still stated only in `driving.md`
is `loop-step-references-k11`'s or `corpus-split-k6`'s work, not a defect here.

## Done when

Every finding is anchored to `path:line`, classified by severity, and stated as
what a session would do wrong — not as a preference about prose. A review that
finds nothing worth acting on cuts no integration step.
