# signal-contract-k18

## Goal

Retire the old signal contract and move the methodology onto the new one:

- **Removals.** `grove-llm complete` goes, with no stub. Grove's code stops
  using `GROVE_SIGNAL_FILE`; the scrub lists keep whatever names they must
  still remove. The runner's token goes.
- **The shipped methodology states the new ending.**
  - The spine's `references/driver.md` and every kind skill that names the old
    verb.
  - `grove-finish`: its teardown ends with `grove-llm record-teardown` and then
    `harness-dispatch exit`, and its other rows end with the exit signal.
  - `configure-grove` and its policy reference: never grant
    `GROVE_LAUNCH_DIR`.
  - The plugin README.
  - The conformance rows in `plugins/grove/conformance/rules.tsv`.
  - The Codex-provisioned skills.

## Context

- The spec's *Ending a session* and *The runner*, decision 9's signalling
  contract, and decision 7's surface (`Ended` has `signalled`, not a token).
- `crates/grove-llm/tests/instructed_verbs.rs` asserts that the shipped skills
  instruct no `grove-llm` verb the CLI lacks.
  `crates/grove-llm/tests/removed_surface.rs`,
  `session_kind_guidance.rs` and `plugins/grove/conformance.sh` and its test
  read the same prose.
- `crates/grove/src/provision.rs` provisions the bundled Codex-compatible
  skills.

## Done when

- An enumerate-then-classify sweep finds nothing that still names `grove-llm
  complete`, `GROVE_SIGNAL_FILE` or the token, except where it states the
  retirement or is a historical record. It covers the shipped skills, the
  conformance rows, the prompt and the code. It follows `references/execute.md`,
  and its controls are seen to fail on a deliberately wrong subject.
- The `instructed_verbs` test passes, as do the conformance suite and the plugin
  install test.
- The `keyed-launch`, `grove-llm` and `grove-loop` books follow (P2).
- `## Unreleased` records that `complete` is gone and how a session ends its
  run.
- `bash scripts/check.sh` passes.

## Notes

- **These plugin edits do not reach this grove's own sessions.** Their skills
  come from the marketplace pinned at v22 until the release, and their v22
  prompt still says `grove-llm complete`. Keep using the installed `grove-llm`
  for this session's own tree verbs and for its own ending (P5).
- The cargo guard keeps `GROVE_SIGNAL_FILE` for as long as this grove runs
  (root brief), so do not remove it here.
- Decision 9 records the finish compound residue: a `finish` session whose
  skill is unread meets the ordinary default. Keep the prompt's sentence shaped
  as decision 9 states, with the kind's own ending as its object.
- This leaf's retirement closes `launch-cutover-k15`. Run that node's close
  and name your run on the pre-cut review `launch-cutover-k19`.
