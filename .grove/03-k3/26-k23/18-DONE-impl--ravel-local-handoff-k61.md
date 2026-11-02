# ravel-local-handoff-k61

## Goal

Close the local stage with a reproducible matched temporary installation, a
reconciled documentation set and recorded commits. Pinned delivery and Grove's
dual-Ravel stage then use one known build as their base.

## Context

- The `## Integration handoff` and `## Settled inputs` sections of
  `ravel-local-delivery-k23`.
- Ravel `docs/skill-discovery-design-evidence.md`: "Feature line and stage
  handoff" and "Consumer handoff".
- Ravel `scripts/install.sh`, `scripts/installed-check.sh` and
  `scripts/installed-smoke.sh`.

## Done when

- The session fetches Ravel. When `main` moved, it rebases the line, reconciles
  upstream changes and reruns the affected checks. It records the exact base.
- The documentation set describes the delivered local subset and nothing more.
  That set is the specifications, `CONTEXT.md`, the book, the examples and
  `CHANGELOG.md`. It names the temporary Git refusal and the unfinished pinned
  stage. All book tasks pass, with `task book:integrity`.
- `task check`, `task test:cli`, `task harness:smoke:test` and
  `task test:native-linux` pass on the final commit.
- `task install PREFIX=<temporary prefix>` installs the feature build. The
  installed checks pass with an isolated `HOME`. The build reports
  `ravel 1.1.0`. The record names its commit, build identity, toolchain and
  platform.
- The temporary installed-pair smoke runs the installed 1.1.0 release and the
  feature build at the same time, with fake managed programs. The 1.1.0
  release runs a native-only launch under one temporary `HOME`. The feature
  build runs a directory launch under a second temporary `HOME`. A settings
  file with `skills` under the first `HOME` refuses in 1.1.0, exit 2. Neither
  run reads or writes the owner's real `HOME`. The installed 1.1 resources,
  settings and ending executables do not change.
- The Ravel evidence document records the base, the output commit, the build
  inputs and the platform facts of this stage. The Grove brief of
  `ravel-pinned-delivery-k25` and the running log name the same commit.
- Nobody pushes the line, and no release consumes it.

## Notes

When a producer of this stage cuts a `review-impl` leaf, it inserts that leaf
before this one with `grove-llm leaf-insert`. The review and its integration
then precede the handoff.

## Decisions (running log)

- Base: `jj git fetch` in `Ravel.skill-directory-delivery` reported no
  change. Local `main` and `main@origin` are both
  `24a545236185b0910ba6117fe8f9e8b2e2c8a748`, so the line needs no rebase.
  The line tip at entry is `bc737938a72e`, from
  ravel-skill-confinement-k69, with an empty working copy.
- Documentation audit: book chapter 6, `CHANGELOG.md` and the source already
  name the temporary Git refusal. Four statements claim more than the local
  subset. The status of `docs/specs/skill-discovery.md` says that
  implementation remains ahead. Its Git section, its `skills resolve`
  sentence on Git pins and its acceptance rows for Git do not mark the
  unfinished pinned stage. `docs/specs/configuration-evaluation.md` states a
  Git requirement for a cache miss without the refusal. The `CONTEXT.md`
  entry "Skill source" names Git without the refusal.
- Not in scope: book chapters 10 and 11 still describe the TypeScript worker
  and SDK. The design evidence records that as unfinished Rhai book work,
  which this line does not assume complete. This stage does not repair it.
- The installed-pair smoke becomes a Ravel script and task,
  `task smoke:pair OLD=<1.1.0 executable> NEW=<prefix>`. It is the
  "Version transition" acceptance row of the skill specification, and the
  pinned stage and rollout need the same check. It runs on macOS only,
  because its guard against reads of the real `HOME` uses `sandbox-exec`.
  A probe in this session showed that the guard denies a read of the real
  `~/.config/ravel/policy.ts` and admits writes outside the real `HOME`.
- Record shape: the build identity digests only the Rust sources, the
  manifest, the build script and the lockfile. The handoff therefore uses a
  build commit, measured, and a child commit that changes only the evidence
  document and names the build commit.
- Documentation repairs, in the build commit: the status of
  `docs/specs/skill-discovery.md` now says that the local stage implements
  local sources. It says that a valid Git source refuses with status 3. The
  refusal has no stable code, and its remedy names the pinned stage. Its Git section, its
  `skills resolve` sentence and a note before the acceptance table name the
  cases that wait for the pinned stage. `configuration-evaluation.md`,
  `CONTEXT.md`, `CHANGELOG.md` and book chapter 10 gained the refusal or the
  new task. The fixtures `a_valid_git_source_refuses_the_launch_until_the_pinned_stage`
  and `a_valid_git_source_refuses_before_any_acquisition_without_a_settings_code`
  check status 3 and the order. The launch fixture uses an unparsable
  script, so the refusal precedes evaluation.
- The technical-communication findings that the hook reports in the skill
  specification and the changelog lie outside the changed lines and predate
  this change. Book chapter 10 has the same three STE problems before and
  after the change, at lines that the change does not touch.
- `scripts/installed-pair-smoke.sh` cleanup restores write access first,
  because retained snapshots are read-only. A first trial showed the
  failure of a plain `rm -rf`.
- Mutation controls of the pair smoke: each control is a mutated copy in
  the scratchpad, and each failed with its expected message.
  - The guard has no deny rule.
  - The directory launch runs under the real home. The guard hid the real
    configuration, and the launch refused as `config_missing`.
  - A file appears in the new prefix after the first digest.
  - The 1.1 settings hold no `skills`.
  - The script passes `originalPrompt` instead of `prompt`.
- Ravel build commit `6436e66fdb308e1c06ee683b6e1b7bc341fabe8b`, parent
  `bc737938a72e`. Measured on it, with an empty working copy that stayed
  unchanged: `task check` exit 0, 727 of 727 run tests passed, 48 skipped.
  `task test:cli` 14 of 14, 1 skipped. `task harness:smoke:test` exit 0. All
  seven book tasks exit 0, with 24 of 24 excerpt mutants killed.
  `task test:native-linux PLATFORM=arm64` exit 0, `failed=0`, 720 of 720,
  48 skipped, Ubuntu 26.04.1, glibc 2.43, kernel 7.0.14-linuxkit. No amd64
  run.
- Installed pair on that commit: `task install PREFIX=<scratchpad prefix>`
  exit 0 with the installed smoke direct and through a link. The build
  reports `ravel 1.1.0`, build identity
  `2ee59f33657690da4facfb7d055527cd3fd12268494dc03bb48e7c0e954bf312`,
  `bin/ravel` SHA-256 `456c0bb2…`. `task smoke:pair` with the Homebrew 1.1.0
  keg exit 0. The keg, the prefix and the real `~/.config/ravel` kept their
  digests, and the real `~/.cache/ravel` stayed absent. Toolchain Rust and
  Cargo 1.99.1, Rhai 1.27.0, aarch64 macOS 27.0.1, Darwin 27.0.0.
- Ravel output commit `1d50dae5d29ee5f54e00a2259d392b407ec6509d`, parent
  `6436e66fdb30`. It changes only `docs/skill-discovery-design-evidence.md`,
  which gained "Local stage record". The local bookmark
  `skill-directory-delivery` moved there through jj. `origin` has no ref for
  it, so nobody pushed it. The brief of `ravel-pinned-delivery-k25` names
  both commits and the build identity.
- No in-session reviewer was spent. The mutation controls cover the new
  script, and the documentation changes are status statements that the
  fixtures above check.
- Final commit `1d50dae5d29e`, with an empty working copy that stayed
  unchanged: `task check` exit 0, 727 of 727 run tests passed, 48 skipped.
  `task test:cli` 14 of 14, 1 skipped. `task harness:smoke:test` exit 0.
  `task test:native-linux PLATFORM=arm64` exit 0, `failed=0`, 720 of 720,
  48 skipped. The book tasks did not rerun, because the commit changes only
  the evidence record, which no book task reads.
