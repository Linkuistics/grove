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
