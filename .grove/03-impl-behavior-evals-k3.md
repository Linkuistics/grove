# behavior-evals-k3

## Goal

Build the behavioural coverage for the Grove lifecycle invariants this refactor
must not change — the safety net the corpus rewrites in leaves 04–06 fall into.
Written against **current** behaviour, so it is green before those rewrites and
must stay green after.

## Context

The requirement is tests "proving behavior rather than only prose shape". The
existing suite is already good at generating claims from sources that move with
the product (`tests/session_kind_guidance.rs` is the model — the kind set comes
from `Kind::ALL`, flags come from clap's model, every sweep carries a control that
shows it can fail). Extend that culture; do not start a parallel one.

Cover these invariants:

- **No second pick.** The driver's mandate is authoritative; `grove-llm pick`
  stays a diagnostic. A session must not re-dispatch itself.
- **No VCS reprobe.** The driver's stated VCS is definitive, and a harness banner
  disagreeing does not win.
- **Stale launch.** A handle resolving to nothing, or to a `DONE` / `ABANDONED`
  leaf, is a stale launch — stop, not work to redo.
- **Decomposition boundary.** Work not serving the leaf's stated goal goes to the
  tree; inline continuation holds only while the work fits one focused session.
- **Human-only pruning.** An agent never prunes on its own.
- **Retire → commit → complete.** Retire precedes commit so the rename lands in
  it; `complete` is the last action.
- **Review budget.** A picked plain producer may materialise at most one
  in-session reviewer across the whole leaf; the predicate is in
  `docs/specs/doubt-grove-review-mechanics.md`.
- **All three finish-signal outcomes** — teardown (`--done`, stop), reopening
  (`complete`, relaunch), declining (no signal, stop, leaf still live).

The **requirements interview threshold** is deliberately *not* here: it is a
contradiction being resolved, so its test lands with the fix in
`kind-references-k5` rather than red across three leaves.

## Done when

- Each invariant above has coverage that fails when the rule is removed from the
  loaded path, and a control demonstrating the check can fail.
- The checks read the **loaded path** for a kind — the guaranteed core plus that
  kind's reference file, as `src/prompt.rs` composes it — rather than grepping the
  whole corpus, so a rule surviving only in a file no session opens still fails.
- The suite is green on the corpus as it stands today, and the leaf records that
  fact as the baseline the rewrites are measured against.
- Nothing in `content/` is edited by this leaf.

## Notes

- Read `rule-ownership-k2`'s inventory first — its **behavioural tests** column
  names what each rule's coverage should be, and its **load predicate** column
  tells you which kinds' paths to assert against.
- What "behavioural" can mean here is a real design question the inventory should
  have settled: deterministic assertions over the composed load path are cheap and
  repeatable; model-in-the-loop evaluations are neither, and are not obviously
  affordable in this suite. If the inventory left it open, decide it here, record
  the reasoning, and prefer the deterministic form — the goal is a net that runs
  on every `cargo test`, not an occasional evaluation.
- Do not weaken an invariant to make a test pass. If the corpus genuinely does not
  carry one of these rules on the relevant loaded path today, that is a finding:
  record it and cut work for it rather than asserting the weaker fact.
