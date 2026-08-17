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

## Decisions (running log)

- **What "behavioural" means here was open, and is now decided and recorded.** The
  inventory's `test` column calls these rows behavioural without naming an
  instrument, and the ADR set said only "a behavioural eval for conduct". A
  `cargo test` cannot watch a session behave, so the instrument is the *delivery*
  half: the rule is present and reachable on the composed loaded path of every
  kind it binds. Raised as `docs/adr/behavioural-coverage-asserts-delivery.md`
  (all three of the AND test hold: the whole net and `loaded-path-budgets-k10`'s
  seam rest on it, "behavioural eval" invites the model-in-the-loop reading, and
  repeatability-versus-observing-conduct is a real trade-off). The spec's
  behavioural-eval seam now cites it rather than restating it.
- **The loaded path is composed from `src/prompt.rs`, not enumerated.** Static
  half: the guaranteed core, `SKILL.md`, `reference_file(kind)`. Conditional
  half: the transitive closure of the corpus files those name by path. Two
  findings fell out of building it, and both are load-bearing for
  `skill-router-k4` and `loaded-path-budgets-k10`:
  - **`SKILL.md`'s routing table is a selector, not an edge.** It names all ten
    reference files, so reading it as ten edges puts every family's discipline on
    every kind's path and the per-kind measure collapses. The walk excises that
    one section; `the_routing_table_is_a_selector_rather_than_an_edge` is the
    control.
  - **`content/SIGNAL.md` is named by no corpus file at all.** It reaches a
    session only because the core inlines its bytes, which is what makes
    `signal-is-the-last-action` provably an eighteen-kind rule and
    `finish-three-endings` provably a one-kind rule. Both are asserted absent
    from the other's path, so the split is checked from both sides.
- **No site pinning.** Eight leaves rehome rules on purpose, so an assertion that
  a rule is stated *in a named file* would fail on the intended change and report
  nothing about the unintended one. Sites are diagnostic output on failure only.
- **The anti-tautology control is a per-rule near-miss, not an ablation.**
  "Delete every paragraph the matcher matches, then assert it fails" is circular.
  Each of the twelve rules instead carries a plausible rewrite that keeps the
  subject and drops the force, asserted to be on-topic and insufficient — which
  is what shows the check is sensitive to the load-bearing clause.
- **No CHANGELOG entry.** The file's own rule is that a decision earns its entry
  when its behaviour lands; this commit adds a suite and a record and changes
  nothing the binary does.
- **`CONTEXT-MAP.md`'s record-ownership list was three records stale** — the two
  corpus ADRs and `corpus-rule-ownership` itself were never added, and its "the
  one spec" clause had become false. Reconciled in the same commit, because the
  map's own rule obliged the new ADR to join that list and adding to a list beside
  a false claim is worse than either.

## Findings for the rewrite leaves

- **`tests/retire_guidance.rs` will go red at `skill-router-k4`, correctly.** It
  asserts `SKILL.md` literally contains "an agent never prunes on its own", which
  the canonical trigger sentence 13 does not carry — that wording moves to
  `references/retire.md`. Left untouched here: it belongs to the leaf that rewrites
  those files, and the new suite is the instrument that survives the move
  (`pruning-is-hitl` is satisfied from either file). The same exposure exists for
  any exact-phrase pin on a file a rewrite leaf owns.
- **Baseline recorded below.** Nothing in `content/` was edited by this leaf.

## Baseline

`cargo test` on the corpus as it stands: **944 passing, 0 failing** (across the
unit suite and every integration binary), `cargo clippy --all-targets` clean,
`cargo fmt --check` clean. `tests/lifecycle_invariants.rs` contributes 10 tests
covering 12 inventory rules across the 8 required areas. Red-run verified by
hand: deleting `SKILL.md`'s pruning-is-HITL paragraph fails
`every_lifecycle_invariant_is_on_the_loaded_path_of_every_kind_it_binds` with one
complaint naming the rule, the area, all nineteen bound kinds, the unmet claim,
and where the rule's other claim still lives; `content/SKILL.md` was restored
byte-exactly afterwards.
