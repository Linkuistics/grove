# ravel-release-promote-k306

## Goal

Deliver `release:promote VERSION=2.0.0`, the repeatable task that publishes
an already complete `ravel@2` release as unversioned `ravel`. Finish runs it
after the compatible Grove release and its installed verification.

## Context

- Ravel `docs/specs/release-channels.md`, sections "Promotion without another
  release" and "Recovery and existing processes".
- Grove `docs/specs/coordinated-release.md`, sections "Compatible tap
  states" (state D) and "Recovery boundaries" (row "Promotion partial").
- Grove `docs/adr/finish-promotes-after-the-grove-release.md`.
- The channel work of ravel-release-channel-k305.

## Done when

- The task cuts no version, rebuilds no archive, creates no tag or release and
  rewrites no book.
- Before any tap mutation, it verifies the public tag, the exact asset
  digests, the book identity, the published `ravel@2` formula and the
  consumer readiness receipt. Missing or mismatched evidence refuses with a
  remedy.
- The consumer readiness receipt has a documented shape. Grove's finish can
  write it from its release and installed verification. Ravel's runtime reads
  no Grove version table.
- One tap commit renders unversioned `ravel.rb` from the verified assets and
  marks `ravel@2` deprecated in favour of `ravel`. It changes nothing else in
  `ravel@2.rb`. The commit is pushed through jj.
- Installation upgrades unversioned `ravel` with `HOMEBREW_NO_INSTALL_CLEANUP=1`
  and verifies the selected prefix. The old 1.1.0 keg stays.
- A retry verifies and keeps a matching formula, tap commit or installation.
  It reports a mismatch and does not overwrite it.
- Fixtures cover unchanged archives, the deprecation, idempotent retries,
  preserved old prefixes, refusal before the readiness receipt and refusal of
  a mismatched archive. Each control is seen to fail against a deliberate
  mutation before it counts.
- `task check`, `task test` and `task release:lifecycle:test` pass. Nothing
  is published.

## Notes

Work in `Ravel.skill-directory-delivery` on `skill-directory-delivery`.

The real upgrade rehearsal with Homebrew, the tap states and a HOME that
holds only `policy.ts` is the later leaf `promotion-upgrade-rehearsal` of the
parent node. This leaf delivers the task and its fixtures. Write the exact
command line, the receipt shape and an example receipt where the finish
handoff can cite them.

If an adversarial read is earned, cut `review-impl` with `leaf-insert` before
`ravel-book-inside-k307`.

## Decisions (running log)

- The cut's release evidence, `target/release-evidence/vX.Y.Z.json`, is the
  pinned identity. Promotion verifies public state against it: the public
  tag's commit, each downloaded asset's SHA-256 and the Pages commit that
  last changed `vX.Y.Z/`. `EVIDENCE=FILE` overrides the path.
- The published `ravel@2.rb` is verified by content. Promotion renders both
  classes from the downloaded assets with the template at the tag's commit.
  The tap's `ravel@2.rb` must equal the versioned rendering, or the
  deprecated form after promotion. It checks no tap commit ancestry, so a
  scratch tap in the later rehearsal works.
- Homebrew 7.0.13 requires `date:` in `deprecate!`, and `deprecate!` follows
  `keg_only` in its component order. The line is `deprecate! date:
  "<tag date, UTC>", because: :versioned_formula, replacement_formula:
  "ravel"`. The tag's date keeps a retry's rendering byte-identical.
  `release-channel.sh` owns the line and its insertion.
- The readiness receipt is JSON at `READINESS=FILE`, by default
  `target/release-evidence/vX.Y.Z.readiness.json`. It binds the consumer
  release to the tag commit and the three digests. Promotion also checks
  that the tap's consumer formula has the receipt's consumer version. It
  interprets no route name and holds no Grove version table.
- Promotion requires a jj tap, because its one commit is pushed through jj.
  It keeps a promotion on the remote and pushes one local promotion commit.
  It commits and pushes over versioned state B. It reports and refuses any
  other state.
- Installation reuses `release-install.sh`, which gains `--version` for a
  run away from the tag. It also gains a check for every channel that each
  keg present before the mutation is still present and executable. A
  repeated `brew upgrade` of a current formula only warns, so a matching
  installation is kept.
- Promotion writes `target/release-evidence/vX.Y.Z.promotion.json`.
- The fixtures extend `release-channel.test.sh` with a promotion section and
  mutants, because they reuse its archives, evidence and fakes.
- Review earned: the first real use runs inside finish after `finish-commit`,
  pushes to the public tap and moves the default command.
  `ravel-release-promote-k326` sits before the first book draft and carries
  the doubts that the fixtures cannot settle. This leaf spends no in-session
  reviewer.
- The Ravel commit is `3f249355cd0a` on `skill-directory-delivery`, pushed.
  Its description holds the command lines. `task check`, 713 tests, `task
  release:formula` and `task release:lifecycle:test` with 41 mutants passed.
  The real tap stayed at `12b53f8c449d`, and Homebrew held only `ravel` 1.1.0.
