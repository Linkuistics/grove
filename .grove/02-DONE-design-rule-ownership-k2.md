# rule-ownership-k2

## Goal

Design the ownership and load-path architecture for the installed prompt corpus —
one canonical source per normative rule, resolvable load paths, and a clean
separation of policy from mechanics, format grammar and rationale. Every later
leaf in this grove executes against this design, so it is load-bearing and its
review step is expected.

## Context

The corpus today states rules in more than one place with no recorded owner, and
its only shape check is a 500-line ceiling whose own doc comment concedes it
"establishes nothing semantic". This leaf replaces that with an explicit map.

The **inventory** is the core deliverable. One row per normative rule:

| column | what it records |
|---|---|
| rule ID | a stable identifier prose and tests can both cite |
| canonical source | the one file that states the rule |
| permitted mirrors | where it may be restated, and why that restatement earns its place |
| load predicate | which sessions read it, and under what condition |
| behavioural tests | what proves a session actually holds it |

Four separations to design, given the requirement that they be distinct:

- **Policy** — `content/SKILL.md` and the ten kind references.
- **Mechanics** — `grove-llm` help text and the config schema. Command facts that
  change belong where they are generated from, not transcribed into prose.
- **Format grammar** — `content/TASK-FORMAT.md`.
- **Rationale and history** — non-normative docs under `docs/`.

The hard boundary: **normative operational material must remain embedded under
`content/` and explicitly reachable by an installed session.** A rule relocated to
a repo doc an installed session cannot open has been deleted, not rehomed. State
in the design how reachability is established for each relocation.

Two semantic contradictions to resolve deliberately, both already decided by the
requirements and needing only their canonical statement and load predicate here:

1. `requirements` **always** establishes *what* is wanted, but the full
   one-question-at-a-time grilling procedure runs **only** when three or more
   interdependent questions are open. Today `references/requirements.md` states
   both the always-form and the threshold, and `execute.md` states the always-form
   alone.
2. ADR creation uses the **narrower AND test** from `decision-records` (hard to
   reverse *and* surprising without context *and* the result of a real trade-off).
   Today `grilling.md` states the AND test and other files say "sparingly".

Also settle the **plugin fallback policy** here, since it is an ownership question
in exactly this sense: for each of the 14 `linkuistics:` deferrals across 7 files,
decide whether Grove owns a minimal local statement (preferred) or defers with
explicit provisioning verification. Leaf `plugin-fallback-k9` executes the answer.

## Done when

- A spec exists under `docs/specs/` carrying the inventory and the four
  separations, written to the membership test — a session on an unrelated future
  grove editing `content/` needs it.
- Every normative rule currently in `content/` appears as a row, with its
  canonical source named and its mirrors either justified or marked for removal.
- The load predicate for each rule is stated in terms a test can check, because
  `loaded-path-budgets-k10` and `behavior-evals-k3` both consume this column.
- Both contradictions have a single canonical statement, and the files that
  currently disagree are named as the edits later leaves must make.
- The fallback policy is decided per deferral, not as a blanket rule.
- The ADR *one canonical source per normative rule, with permitted mirrors* is
  raised if it clears the AND test on the design as it lands — in place, reworking
  the existing set rather than appended beside it.
- The design says explicitly which of leaves 04–06 it reshapes, if any, and cuts
  or re-cuts those leaves rather than leaving the difference for them to absorb.

## Notes

- `src/prompt.rs` is **not** in scope to change. Read it — it is the runtime that
  composes the guaranteed core and picks the reference file per kind — and design
  around it. Its three-part architecture is a fixed constraint.
- The corpus's shape is already argued in `docs/ARCHITECTURE.md`, *The corpus's
  shape, and the three alarms over it*. Reconcile that section with this design
  rather than writing a second account beside it.
- Prefer the ownership/load-path refactor over mechanisation in this design.
  Mechanising command facts is a later, selective move — say which facts qualify,
  do not build it here.
