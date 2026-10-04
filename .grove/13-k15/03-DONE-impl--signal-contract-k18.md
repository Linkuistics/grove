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

## Implementation plan

1. Assert the removed CLI verb is refused and retirement guidance names dispatch's exit.
2. Remove `complete`, its channel admission compatibility, and the runner payload surface; preserve launch-directory admission and scrub guards.
3. Update shipped skills, conformance, Codex bundle evidence, and the three changed crates' source-exact books.
4. Classify the full candidate sweep with failing controls, run the principal checks, retire this leaf, close `launch-cutover-k15`, name this run on its review, and seal one jj change.

## Decisions (running log)

- S1: Follow the approved appearance-only dispatch exit and separate Grove teardown record (module decisions 7 and 9). No compatibility stub for `complete` or fallback admission through the retired channel remains. Scrub lists and the live v22 cargo guard retain retired authority names.
- S2: The existing `launch-cutover-k19` review owns the node's adversarial review; this producer dispatches implementers only. The installed v22 binary remains this session's tree and completion interface.

- S3: Remove the obsolete `grove-llm` dev dependency on `keyed-launch` with its payload round-trip tests. CLI refusal, retirement guidance, live-epoch admission, and bundle byte equality provide the replacement boundary evidence.
- S4: Keep the finish prompt's final action object as the kind's own ending. Shipped finish guidance adds `record-teardown` before dispatch exit rather than replacing the universal exit action; recording waits for all authorized finish work.

## Sweep evidence

- Enumerated identifiers over `crates/`, `plugins/grove/`, `testing/` and `.cargo/`, then classified the retired names. Current Grove production occurrences of `GROVE_SIGNAL_FILE` are the loop and standalone scrub lists; the driver-lease submodules retain only fixture `env_remove` guards. Other test occurrences assert scrubbing, cargo isolation, ignored legacy input or removal. The dispatch environment test treats the name as caller-owned/grantable, not as dispatch authority. The cargo guard explicitly describes the installed v22 exception.
- Enumerated all `grove-llm` command words in the shipped skills; none names the retired verb. The same extraction finds a deliberately wrong multiline `complete` instruction and the design's explicit retirement record. Runner source identifiers retain `signalled` and contain no `Token` or `token`; the identical check rejects a deliberately reintroduced payload type/field. The CLI regression was observed failing before removal, then passing after it. Conformance's deliberately missing teardown-before-exit subject fails.
- Markdown outside the shipped corpus and changed books remains owned by the already-scheduled `current-state-docs-k20`, including the root and dispatch READMEs, usage/release guides and `CLAUDE.md`. No claim of a completed repository-wide documentation migration is made here.

- S5: The first frozen principal run failed only the quoted-launch test: it conflated dispatch run mechanics with the policy-input set. Keep checking the ordered policy inputs and permit precisely the optional `--exit-dir`/`--ending-file` suffix. Selection-only examples remain admissible during the declared documentation migration. All other eleven principal checks passed, all six books were valid, and before/after digests of repository subjects were identical.
- S6: The second frozen run passed the corrected driver suite but reached a stale CLI-help smoke assertion requiring `complete`. Replace it with `record-teardown` and explicitly forbid the retired command. A follow-up sweep of command-shaped test mentions also corrected the retirement reminder's stale diagnostic and stdout assertion. All other eleven principal checks passed again, all books remained valid, and all 1,925 repository-subject digests were unchanged.
- S7: The third frozen run reached dispatch's inspect-help assertion, which still required a warning against the retired signal variable. Pin the live launch-directory warning instead. Enumerating remaining dispatch, runner and viewer test mentions classifies the legacy names as environment scrubbing/authority isolation or the intentional caller-owned generic grant test. Again the other eleven principal checks passed, with unchanged repository-subject digests.

## Verification and node close

- The complete locked workspace suite passed after the final help correction. The final `task check` (`bash scripts/check.sh`) then passed all 12 principal checks, including plugin installation, shipped conformance and its mutation controls, release fixtures, dispatch typechecks/probes, workspace tests and all six books. Before/after SHA-256 digests matched for all 1,925 repository subjects: source, tests, fixtures, manifests, scripts, documentation, plugins, cargo guards and task notes.
- Final book validation has no deferred ranges: `grove-llm` reconstructs 4 roots/915 lines, `grove-loop` 15 roots/14,093 lines and `keyed-launch` 7 roots/2,309 lines. The other three books also pass.
- `launch-cutover-k15`'s Done when holds across k16, k17 and this contract leaf. The current 36-case real-driver PTY suite covers all three endings, rotated epoch admission, launch cleanup, unread abandoned cleanup, dispatch death and terminal recovery, background drivers, driver TERM and creator cases. The prompt, CLI, bundled Codex skills and shipped methodology agree on appearance-only dispatch exit and the separate teardown record.
- The approved ADRs already describe the delivered launch identity and supervision contract; no new decision or contradictory ADR remains to reconcile. Promote the completed node's remaining context to the root brief. Its scheduled k19 review remains next, and k20/k21 remain live, so the cascade closes this node only. After verification only retirement, creator naming and task/brief bookkeeping change before the focused jj commit.
