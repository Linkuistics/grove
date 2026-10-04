# major-release-k21

## Goal

Prepare the major release this grove ends in (D8), and its cutover on this
machine:

- **The release notes.** `## Unreleased` opens with what an owner must do.
  - An owner policy that reads `repo` or `session_name` refuses until it is
    edited, and the sample shows the `.jj/repo` derivation.
  - `run` supervises instead of exec'ing.
  - A session ends its run with `harness-dispatch exit`, and `grove-llm
    complete` is gone.
  - `grove run` launches through dispatch, with a run record.
  - The skills and the binaries must be upgraded together.
- **Every live grove on the machine.** `docs/RELEASING.md` says what a release
  that changes the session's signal contract owes them. Their v22 drivers'
  sessions end with `complete`, which the new `grove-llm` lacks, so their
  drivers are stopped before publishing and restarted under the new pair.
- **This grove's own finish.** It knows how it ends under the v22 driver once
  `task release:major` has installed the new pair, and this leaf writes that
  into the root brief's Notes, where the finish session reads it.

## Context

- `docs/RELEASING.md`: *Prepare the release*, and *A release that changes how
  a tree is read meets every grove on the machine*, which is the precedent
  section's shape. `CHANGELOG.md`'s rules for `## Unreleased`.
- `CLAUDE.md`'s finish sequence (integration, then `task release:major`, then
  the final signal) and P5 (c)–(f) in `supervised-dispatch-k4`'s running log.
- Facts to verify, not assume:
  - whether `brew upgrade grove` removes the v22 keg, which takes the v22
    `grove-llm` with it;
  - whether a copy of the v22 `grove-llm` preserved outside the keg still
    sends `complete --done` to the v22 driver;
  - that a v22 driver mid-run finds its `harness-dispatch` from its own
    canonical executable path, which a removed keg invalidates.

## Done when

- `## Unreleased` carries the owner-facing framing above, consistent with the
  entries each leaf already logged.
- `docs/RELEASING.md` has the live-grove cutover procedure for a release that
  changes the signal contract.
- The root brief's Notes tell this grove's finish exactly how it ends after the
  release installs the new pair. Each step in them has been checked against the
  v22 binaries.
- A D1/D2 edit of the owner's installed policy is drafted for the owner. It
  must select under both v22, which still passes the parameters, and the new
  pair, which passes none. It is applied only with the owner's confirmation.
- An attended trial of the built pair, with a real harness in a scratch grove,
  has been offered to the owner and either run or declined on record. Use a
  scratch prefix, never the Homebrew one.
- `bash scripts/check.sh` passes.

## Notes

- Do not run the release here. The finish session runs it, with the owner's
  confirmation, by the sequence in `CLAUDE.md`.
- The trial exists because every live grove on the machine is the release's
  first user. The seams use fake harnesses only, by agreement (D9), so no
  automated test has watched a real interactive harness under nested job
  control, or a real harness sandbox writing its exit channel under `.jj/grove/`.

## Decisions (running log)

- K21.1 — This leaf prepares the major release only. Publication, integration
  and installation remain the finish session's authorized sequence. The final
  signal of this leaf remains installed v22 `grove-llm complete`.
- K21.2 — The installed personal policy already implements D1/D2: it ignores
  parameters and derives secondary-workspace grants from `request.cwd/.jj/repo`.
  Its SHA-256 is `5f54fa07177ebf911fc2fb4057bc5232ccd25b247297afa3c01781fee421feba`.
  Read-only inspection selects `gpt-6.1-sol` / high with the correct main-repo
  grant under both installed v22 with all three parameters and the built
  candidate without parameters. No personal policy edit is needed or applied;
  the release guide supplies the migration draft and dual-version procedure for
  policies that still read them, preserving owner routing choices.
- K21.3 — Homebrew 7.0.7's `upgrade.rb` invokes formula cleanup after upgrade;
  `cleanup.rb` removes eligible old kegs, and the official FAQ documents that
  behavior. Preserve a real copy of the v22 helper outside the keg before
  release rather than depend on disabled cleanup or a symlink.
- K21.4 — Preserved installed v22 `grove-llm` at `.jj/v22-cutover/grove-llm`.
  Source and copy share SHA-256
  `4e752bda6c091be019d39169cfa1f099f9704ff03ba75faa7581a84359851b25`;
  `otool -L` reports only system libraries. With all ambient launch controls
  scrubbed, the relocated helper's `complete --done --signal-file <scratch>`
  wrote exactly `done\n`. No live signal was written.
- K21.5 — The owner accepted the offered attended scratch trial. The candidate
  prefix contains built binaries, worker and skills, with no Homebrew install.
  First real Codex run `7206ef93-9529-4284-a4e0-3a716a5c1bfd` reached its folder
  trust dialog and exited unsignalled when cancelled; dispatch recorded
  `harness_exit` and Grove stopped. Starting bare Grove also provisioned the
  candidate skills into the shared HOME; restored the installed v22 bundle
  immediately with a deliberately unmapped scratch launch, checked the v22
  finish skill's completion instructions, and isolated the retry in temporary
  HOME with private configuration/credential copies. The release procedure now
  explicitly isolates skill provisioning as well as binary lookup.
- K21.6 — Verified the installed v22 pair in a copied scratch keg with a fake
  harness that deletes that copied keg, then invokes the preserved helper with
  its live scratch epoch and original signal environment. The helper was
  admitted, wrote its finish disposition, and the running driver printed
  `grove: grove finished — loop complete.` and exited 0. Its launch lookup is
  pinned by `v22.0.0:crates/grove/src/dispatch.rs`, which canonicalizes the driver
  once; `loop_driver.rs` retains that sibling dispatch path for subsequent
  launches. Preserve the helper and finish the old loop instead of relaunching
  after keg cleanup. Root Notes now give this finish the exact absolute helper
  path and final action after the major release.
- K21.7 — The isolated attended trial completed through real Codex 0.160.0
  using the unchanged owner impl policy (`gpt-6.1-sol`, high, full access).
  Run `c7545713-c88b-4310-81d0-0e54043adb01` wrote its scratch cwd/run ID and
  launch paths, confirmed no legacy channel, passed `task check`, retired k1
  and sealed commit `ae0ef60f`, then sent dispatch exit. Its dispatch record
  confirms execution, `exit_signal`, SIGTERM and 151571 ms. Grove relaunched
  `observe-relaunch-k2` as run `64617cb4-9f53-47e3-9e94-da999c1fe376`; real Codex
  printed `TRIAL_RELAUNCH_OBSERVED` and remained at its prompt. A typed Ctrl-C
  exited that harness cleanly (code 0, `harness_exit`, 85141 ms), and Grove
  stopped with k2 still live. Both records carry empty parameters and the
  expected policy digest. Terminal control returned after both harnesses.
  This trial covers the real interactive harness, signalling, relaunch and
  unsignalled stop; it does not claim an OS sandbox, forced KILL, or descendants
  that leave the supervised group. The agreed fake-harness seams retain those
  separate checks. Scratch prefix/workspace/records remain outside Homebrew;
  private credential and configuration copies are removed after the runs stop.
- K21.8 — Repeated the installed-v22 live probe with both its copied keg and
  `.grove/` removed before the preserved helper ran. Active epoch admission and
  `complete --done` still succeeded; the old driver reported loop completion
  and exited 0. This matches the finish's teardown-before-install sequence.
- K21.9 — `task check` invoked `bash scripts/check.sh` and exited 0: all 12
  principal checks passed, including the complete workspace suite and final
  validation of all six walkthrough books. SHA-256 captured for all 1926
  tracked files before/after that gate matched individually, including source,
  manifests, fixtures, scripts, books, release notes and task notes. Only this
  result annotation and retirement/sealing follow the measurement. The leaf's
  deliverables are complete; the ordinary tree has no remaining producer work.
  Root Notes retain the release/ending contract, and the driver-owned finish
  still owns authorization, teardown, integration and the major release. This
  leaf publishes nothing and changes no installed binary or personal policy.
