# ravel-release-channel-k305

## Goal

Carry one explicit release channel through Ravel's existing release tasks,
so that the 2.0.0 cut publishes only `ravel@2` and leaves unversioned `ravel`
at 1.1.0.

## Context

- Ravel `docs/specs/release-channels.md`, sections "One release, two formula
  channels", "Release interface" and "Recovery and existing processes".
- Ravel `Taskfile.yml` (the tasks `release`, `release:patch|minor|major`,
  `release:doctor` and `release:lifecycle:test`) and `scripts/release-*.sh`.
- `scripts/release-formula.test.rb` already derives a keg-only `RavelAT2`
  class for its test. `scripts/templates/ravel.rb.tmpl` renders `ravel.rb`
  only.
- Grove `docs/coordinated-release-evidence.md`, section "Source evidence":
  the baseline step reads `Formula/ravel.rb`, rendering emits `ravel.rb`,
  publication copies to that formula, installation upgrades `ravel` and
  verification selects the shared-prefix command. Renaming one file is not
  sufficient.

## Done when

- The release tasks accept `CHANNEL=versioned-2` or `CHANNEL=unversioned`
  and refuse any other value before a mutation.
- For a 2.0.0 cut, omission of `CHANNEL` refuses. Omission never publishes
  unversioned `ravel`.
- The task owns the mapping of channel to formula file, class, `keg_only`
  line and prefix. A caller gives one argument.
- Preparation verifies the baseline against unversioned `ravel` for the first
  2.0.0 cut, and against `ravel@2` for a later 2.x cut before promotion. The
  incomplete-baseline refusal stays.
- After promotion, `CHANNEL=versioned-2` refuses because unversioned `ravel`
  has major 2.
- Each resume mode, `--tap-only` included, keeps the selected channel. No
  resume mode defaults to unversioned.
- Installation and verification of `ravel@2` use the versioned prefix and its
  `bin/ravel`, never the command on `PATH`. The installed verification reads
  the keg's installation record.
- The release evidence records the channel, the tag's commit, the asset
  digests, the book commit, the tap commit and the installed result.
- Lifecycle fixtures cover baseline against target channel, resume exits,
  wrong-channel refusal, the refusal of `versioned-2` after promotion and an
  unchanged unversioned formula during a versioned release. Each control is
  seen to fail against a deliberate mutation before it counts.
- `release-channels.md` no longer says that implementation remains ahead for
  this part. `task check`, `task test` and `task release:lifecycle:test` pass.
- Nothing is cut, tagged or published, and the tap is unchanged.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.
Chapter 11's prose belongs to the book pipeline that follows. Record the
exact command lines in the commit description or in the release evidence.
The drafting session then does not reconstruct them.

This tooling publishes outside the machine on its first real use. If an
adversarial read is earned, cut `review-impl` with `leaf-insert` before
`ravel-book-inside-k307`.

## Decisions (running log)

- Every release task and every publication script names its channel. The
  Taskfile requires `CHANNEL` with an enum, so omission refuses for every
  version and never only for 2.0.0. The scripts take `--channel` as an
  argument and never read a `CHANNEL` environment value. Task itself also
  reads environment values as variables; that is an accepted trade-off.
- One sourced file, `scripts/release-channel.sh`, owns the mapping from the
  channel to the formula name, file, class, `keg_only` line and prefix. It
  also owns the admission rules against the target version and the tap's
  unversioned formula.
- Admission: `versioned-2` publishes only major 2 and refuses once
  `Formula/ravel.rb` has major 2. `unversioned` refuses a major-2 target while
  `Formula/ravel.rb` has a lower major, because `release:promote` alone moves
  unversioned `ravel` to 2. The baseline formula is `ravel@2.rb` only for
  `versioned-2` over a major-2 baseline; otherwise it is `ravel.rb`.
- One template renders both classes from `@CLASS@` and an `@KEG_ONLY@` line.
  `release-formula.sh` always renders and tests both classes through
  Homebrew's own install/test methods. On the release path it keeps only the
  selected formula in `target/dist`.
- Installation and verification move out of the Taskfile into
  `scripts/release-install.sh --channel`. It verifies the keg path, the opt
  prefix, keg-only state and the keg's own `bin/ravel` and installation
  record. For `versioned-2` it also verifies that unversioned `ravel` is
  unchanged. Field names come from Homebrew 7.0.13's `formula.rb` `to_hash`.
- `scripts/release-evidence.sh --channel` writes
  `target/release-evidence/vX.Y.Z.json` after installation and before `live`.
  It records the channel, the tag's commit, the public asset digests, the
  Pages commit that last changed `vX.Y.Z/`, the tap commit that last changed
  the selected formula and the installed result.
- `--check-tap` stays channel-free. Every other publication mode, including
  `--book-only`, requires `--channel`, so no resume mode has a default.
- The unchanged formula test failed on this machine: the shared default
  skill directory now holds sources, so its exact-prompt launches gained
  skill text. Its `HOME` now sets `"sources": []`, as the skill-discovery
  specification directs. The private-record check uses a separate `HOME`.
- The Ravel commit is `ba3298f2c705` on `skill-directory-delivery`, pushed.
  Its description holds the exact command lines for the cut and each resume.
- Review earned: the first real use publishes outside the machine.
  `ravel-release-channel-k325` sits before the first book draft and carries
  the doubts that the fixtures cannot settle.
