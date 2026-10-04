# current-state-docs-k20

## Goal

Make the documents a reader meets describe the result as current state. These
are the documents no test ties to a source change, so the implementing leaves
did not have to move them (P2):

- `docs/ARCHITECTURE.md`: *Process ownership*, the harness-dispatch section,
  and the runner.
- `docs/USAGE.md` and its coverage inventory,
  `docs/specs/user-guide-coverage.md`. `complete` is gone and
  `record-teardown` arrives; the guide covers how a session ends its run and
  `grove run` through dispatch.
- The dispatch README, `crates/harness-dispatch/README.md`:
  - `run` supervises;
  - `exit` and the run flags (`--exit-dir`, `--ending-file`, `--confine`,
    `--runtime-read`);
  - the run ending and the end observation;
  - how a session ends its run;
  - never granting `GROVE_LAUNCH_DIR`.
- This repository's `CLAUDE.md`: the finish sequence's last step and the
  section on invoking `grove-llm` (the cargo guard, now on the new variables).
- `docs/RELEASING.md`'s mentions of the signal channel, such as the `env -u
  GROVE_SIGNAL_FILE` probe for live groves.
- Any other repository document that still describes the `exec` handoff,
  `complete`, `GROVE_SIGNAL_FILE`, the token or Grove-side confinement as
  current.

## Context

- `signal-contract-k18` updated the quoted-launch test to distinguish policy
  inputs from `--exit-dir` and `--ending-file`. Until this documentation leaf
  updates every quoted lifecycle example, it accepts the selection-only form
  or the full mechanics suffix. Update the root/run help examples, dispatch
  README and usage guide together, then require the full suffix in
  `the_grove_invocation_dispatch_s_help_shows_is_the_one_the_driver_makes`; do
  not keep the migration allowance after all quoted lifecycle examples move.

- `confined-run-k23`'s committed review, F6, points to decision 7 of
  `docs/specs/module-decomposition.md`. Reconcile its runner API listing with
  the shipped code as part of this document sweep; the integration leaf fixes
  the book-structure ledger, leaving the full module contract to this leaf.

- The spec's *Grove integration* closing paragraphs list what Grove's usage
  guide, the configure-grove skill and the dispatch README must explain, and
  each invocation they quote must be one `--help` carries or a suite makes.
- `docs/ARCHITECTURE.md`'s documentation-ownership table says which document
  owns which subject.
- The specs, ADRs, glossary and visual document were reworked by design.
  Change them only where implementation proved them wrong, and record why in
  the running log.

## Done when

- An enumerate-then-classify sweep (`references/execute.md`), with controls
  seen to fail, covers the repository's Markdown outside `.grove/`. It shows
  every remaining mention of the retired surface is a historical record or
  states the retirement.
- The user-guide coverage test and the repository's link and reference tests
  pass.
- `bash scripts/check.sh` passes.

## Notes

- **`CLAUDE.md` is read by this grove's own finish**, which runs under the v22
  driver and prompt (P5 e). Word the finish sequence's last step so it holds
  under both versions, for example by ending as the launch prompt directs. Or
  agree a wording with the note `major-release` writes for the finish. Never
  leave the finish told to run a verb that cannot end its run.
- `AGENTS.md` is a symlink to `CLAUDE.md`, so a single edit reaches Codex too.

## Decisions (running log)

- K20.1 — Reconcile prose against the shipped source and settled area spec,
  without changing launch behavior. The graph CLI refused startup because a
  pre-coordination/unverified generation is active; source fallback is required.
  Tighten the quoted-launch assertion first, update help and all quoted lifecycle
  examples together, reconcile the architecture/guide/dispatch README and runner
  contract, then enumerate and classify the remaining Markdown candidates. Use
  controlled dirty/clean sweeps, guide/reference tests, and `task check` before
  retirement and one focused jj commit. The finish instruction must defer to its
  launch prompt so the installed v22 driver can still end this meta-grove.

- K20.2 — The tightened quoted-launch test failed on the old root help example.
  All lifecycle examples now carry `--exit-dir` and `--ending-file`, with no
  selection-only allowance. Decision 7's runner sketch was wrong after the
  confinement increment: reconcile `NoninteractiveLaunch`, `FilesystemGrants`,
  `run_confined_observed`, transparent entry signals and group outcomes with the
  exports. No launch semantics changed. The retained book anchors/slice IDs stay
  stable while their prose describes teardown recording and appearance-only exit.

- K20.3 — Reconcile one more implementation-proved difference in decision 7: the
  lifecycle wrapper has a 10-second cancellation grace, while standalone has
  five seconds because confined cancellation kills the harness immediately.
  Also correct help's ordering: dispatch writes the ending file before waiting
  for the record-store append. Preserve historical source-ledger IDs, research,
  evaluations and versioned changelog entries with their explicit provenance;
  remove obsolete promises from the current Unreleased roll-up.

- K20.4 — The one bounded reviewer found valid contradictions in the architecture
  roll-up and the README's exit-status rule. Correct both against `run::exit`: an
  exit-signal ending returns 0 only after escalation or a successful natural
  exit; a natural failure after signalling keeps its status. Apply that same
  qualification to root help. The ending still remains `exit_signal`, so Grove
  lifecycle relaunch and standalone publication retain their distinct rules.

- K20.5 — All four bounded-review findings are valid and actionable, verified
  against source. In addition to K20.4, include directly delivered INT for
  transparent wrappers in decision 7 and correct dispatch's `keyed-launch`
  dependency in the architecture summary and overview table. These are prose
  corrections to explicit source facts, not redesign; no second review is needed.

- K20.6 — Enumerated every lexical token and inline code span from repository
  Markdown outside `.grove/` (including hidden paths, excluding build output and
  installed dependency trees), then classified authority variables, retired
  commands/types, process replacement and signalling/confinement prose. The
  remaining retired names belong to versioned changelog entries, the preservation
  baseline, dated research/evaluations/runtime evidence, explicit retirement or
  v22 cutover notes, historical ledger IDs, or source-exact legacy scrub/cleanup.
  `Watch::Signalled` remains live; generic kind/receipt/usage tokens and exec in
  wrappers or POSIX explanations are unrelated. Updated the summary layers and
  concept index as well as the detailed sections. The initial control exposed
  ripgrep reading inherited stdin; explicit `.` fixes the search surface. With
  identical flags, a scratch retired command was detected, its repair read clean,
  a live exit-command positive control was found, the preservation-baseline
  cross-tree control stayed dirty, and a deliberately broken pattern failed.
  Guide coverage (4 tests) and reference navigation (13 tests) pass.

- K20.7 — The source-structure briefs had retained pre-cutover line ranges,
  root sizes and slice/chapter totals. Reconcile their current ledgers with the
  already-updated walkthrough manifests (915 grove-llm lines; 2309 keyed-launch
  lines), retaining old measurement denominators only as explicitly historical
  evidence. Dispatch's settings paragraph also incorrectly included `exit`:
  `main` routes it directly to the settings-free exit verb. Correct that prose
  against the implementation and settled spec. Discard the in-progress checks
  that preceded these corrections; freeze the final project files and run the
  complete gate on that unchanged revision.

- K20.8 — The first completed gate passed 11 checks, including all six books,
  but failed dispatch's help-example presence test: its exact expected command
  still ended after `--prompt`, so it rejected the corrected lifecycle example.
  Reproduced that failure alone, then migrate this second existing assertion to
  the same full mechanics suffix. Root help's owner-settings summary also needs
  the settings-free `exit`/`init` exception already corrected in the README.
  Correct only these contract descriptions/assertions and rerun the required
  gate. Hashes confirmed all 1926 measured subjects stayed unchanged during the
  failed gate; it is not reported as a pass.

- K20.9 — Final `task check` (which runs `bash scripts/check.sh`) passes all
  12 principal checks: formatting, shellcheck, clippy, plugin installation,
  conformance and its controls, release preparation/helper/tasks, dispatch
  worker/probes/types, the locked workspace test suite, and all six walkthrough
  books in final mode. Guide coverage and reference navigation pass inside that
  run, as do both lifecycle-example assertions. All 1926 versioned file subjects
  have identical before/after hashes. The final sweep enumerates 399 Markdown
  paths, 25592 distinct lexical tokens, 50549 code spans and 55 surface-token
  candidates; the classification and dirty/clean/cross-tree controls in K20.6
  hold. All Done-when clauses are met. No live review leaf names this producer;
  `major-release-k21` remains live, so retirement closes no parent node. The
  settled design and ADRs need no further change: the runner sketch and source
  ledgers record the implementation-proved corrections above.
