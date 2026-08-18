# corpus-split-k26

**Reviews:** corpus-split-k6

## Goal

An adversarial read of the corpus split, against one question the producer could
not answer about itself: **which normative rule is now unreachable?**

## Context

`corpus-split-k6` deleted `content/driving.md`, cut `TASK-FORMAT.md` from 3,012
to 935 words, rewrote `ADR-FORMAT.md` and `grilling.md`, and moved rules into
`CONTEXT-FORMAT.md` and `references/execute.md`/`retire.md`. `content/` went from
~23,500 words at this grove's start to 14,698.

This leaf's mistakes are the least visible in the workstream. A rule quietly
stranded outside every loaded path fails **no test that exists** until
`loaded-path-budgets-k10` builds the reachability walk — `tests/rule_ownership.rs`
proves single-source for the wordings it names and says so, and its own module
doc records that a paraphrase reads clean. So a green suite is not evidence for
the claim this review is asked to attack.

Read `docs/specs/corpus-rule-ownership.md` (the inventory, the relocation table,
the trigger sentences) and the two ADRs it cites before starting.

## Done when

Each of these has been checked against the corpus as it stands, and every finding
is anchored to `path:line`:

- **Unreachable rules.** For every rule whose owner is not `SKILL.md` or a kind
  reference file, some file on a loaded path literally names that owner's path.
  `ADR-FORMAT.md`, `SPEC-FORMAT.md` and `CONTEXT-FORMAT.md` are reached from
  `SKILL.md`'s sentences 20–24; `TASK-FORMAT.md` and `BRIEF-FORMAT.md` from
  `references/decompose.md`; `grilling.md` from `references/requirements.md`.
  Confirm each edge is a real sentence naming a real path, not an inferred one.
- **Rules deleted with the file that argued them.** The producer walked
  `driving.md`'s relocation table row by row and swept for each rule's new owner.
  That is a *list*, and the spec's own audit rule says a list is complete only as
  far as the list — twice already a row was missing and the table authorised a
  deletion that would have taken a live rule with it. Read the deleted
  `content/driving.md` (`git show HEAD~1:content/driving.md`) and the deleted
  sections of `content/TASK-FORMAT.md` and `content/grilling.md` **sentence by
  sentence**, and resolve every imperative to a row or report it.
  One such gap was found and repaired during the session — `glossary-is-only-a-glossary`'s
  *not a spec, not a scratch pad* clause existed only in `grilling.md` — which is
  evidence the sweep can miss one, not that it missed only one.
- **Rules stated twice.** `tests/rule_ownership.rs` now claims single-source
  outright, having shed every `transient` / `restated` declaration. Attack that:
  a paraphrase is invisible to it, and the rows it does not name are unswept.
  `references/execute.md` lost its records paragraph in the same commit that
  landed `ADR-FORMAT.md`'s and `SPEC-FORMAT.md`'s statements — check nothing else
  restates either.
- **Attribution.** `grilling.md` and `CONTEXT-FORMAT.md` carry provenance
  comments recording deliberate divergences from upstream. Rules moved between
  them; check each header still describes the file it heads, and that
  `grilling.md`'s `<what-to-do>` block is byte-identical to its state before this
  commit.
- **`docs/USAGE.md`.** Two operator-facing sections landed there. Check they are
  genuinely unusable by a session (a session *is* the LLM), and that neither
  duplicates a rule `references/execute.md` or `grilling.md` states.

## Notes

- Inspection only. Do not edit; findings only, and cut an
  `integrate-review-impl` leaf only if there is something worth acting on.
- `content/SIGNAL.md` and `content/SIGNAL-FINISH.md` are out of scope for the
  whole workstream; confirm they are untouched rather than reviewing them.
