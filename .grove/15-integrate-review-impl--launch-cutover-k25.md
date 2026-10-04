# launch-cutover-k25

**Integrates:** launch-cutover-k19

## Goal

Triage `launch-cutover-k19`'s six findings against the source and apply those that
hold, so that `current-state-docs-k20` and `major-release-k21` describe an agreed
cutover. This session owns corrections to what `launch-cutover-k15` produced
(`launch-directory-k16`, `dispatch-ending-k17`, `signal-contract-k18`), not the
documents `current-state-docs-k20` owns.

## Context

- Read the findings from `launch-cutover-k19`'s committed review, not from this
  body. Classify each on evidence: a contract stated unclearly, a real issue, a
  visible trade-off, or noise.
- F1 is the one to start with: a shipped skill in `plugins/linkuistics` and the
  spec behind it still say `grove-llm complete`. The reader in
  `crates/grove-llm/tests/instructed_verbs.rs` is sound but scoped to
  `plugins/grove/skills`; settle whether to widen it to every shipped plugin.
  Sweep by enumerating, then classifying, as `references/execute.md` requires, and
  watch each control fail before crediting a clean read.
- F2 and F4 are test changes. F3 and F5 are a contract question each (the table's
  missing row; the legacy epoch reading). F6 is two sentences, in the skill and the
  prompt.
- A leaf that changes a crate's source updates that crate's walkthrough book in
  the same commit, and logs the change under `## Unreleased` in `CHANGELOG.md`.

## Done when

- Each finding is classified; each real one is fixed with a test, or accepted
  visibly with its reason in this leaf's log.
- No shipped skill instructs a `grove-llm` verb the CLI lacks, across every plugin
  this repository ships.
- `bash scripts/check.sh` passes.

## Notes

- This is a meta-grove: end the session with the installed v22 `grove-llm`,
  never `./target/debug/grove-llm`, as the root brief says.
- `current-state-docs-k20` still owns `docs/USAGE.md`, `docs/ARCHITECTURE.md`,
  the READMEs and `CLAUDE.md`; the review's "For the leaves after this one" lists
  them. Do not take them here.
- `major-release-k21` owns the live-grove cutover. The review notes that a running
  v22 driver whose session meets a v23 `grove-llm` cannot send `complete`; that is
  its question, not this leaf's.
