# ravel-two-publication-k313

## Goal

Cut Ravel 2.0.0 once and publish it through `ravel@2` only, then verify the
installed prefix and the live book. This makes tap state B.

## Context

- Grove `docs/specs/coordinated-release.md`, step 4 of "Publication ownership
  and order" and "Recovery boundaries".
- Ravel book chapter 11 on `main`, with its channel and recovery commands.
- Grove `docs/legacy-driver-transition.md`, section "Rehearsal beside a Ravel
  2.0 prefix", for the verification of the old ending path.

## Done when

- Before the cut, a fresh read shows no `v2.0.0` tag, no 2.0.0 release and no
  `ravel@2.rb` in the tap. Otherwise the recovery table governs.
- From Ravel's default workspace, one run of
  `CHANNEL=versioned-2 HOMEBREW_NO_INSTALL_CLEANUP=1 task release:major`
  completed, or a partial run was resumed under the recovery table. No second
  bump happened.
- The tag `v2.0.0`, the GitHub release with the three archives, the
  versioned book on the Pages branch and `Formula/ravel@2.rb` exist.
  `Formula/ravel.rb` is unchanged at 1.1.0.
- `brew test ravel@2` passes. Its prefix's `bin/ravel --version` reports
  2.0.0. Its installation record names `$(brew --prefix)/share/ravel/skills`.
  The command `ravel` on `PATH` still reports 1.1.0.
- The live book shows 2.0.0 and its packaging guidance.
- `/opt/homebrew/Cellar/ravel/1.1.0` is complete. Its `bin/ravel` reports
  1.1.0 and refuses `end-session` outside a session with no result.
- The receipts are in Ravel's release evidence and in a new section of Grove
  `docs/coordinated-release-evidence.md`. They name the channel, the tag
  commit, the book commit, the tap commit and the installed result. They give
  the SHA-256 of each asset as measured and as GitHub reports it.

## Notes

The root charter makes this publication a deliverable of this grove. Before
the run, state the version, the channel and the four publications. Do not ask
again for authority that the charter gives.

Set `RAVEL_TAP_DIR=/Users/antony/Development/Delivery/homebrew-taps`.
Never run `brew cleanup` or `brew autoremove`. Never uninstall or reinstall
`ravel`.
A step can fail where the book does not cover its recovery. Then stop. Ask the
owner, and give the trade-off, a recommendation and the evidence.

ravel-main-landing-k312 pushed Ravel `main` at `12df84bebf4a`. It left the
default workspace on that commit with an empty working copy. A Grove session
inherits `RAVEL_EXIT_FILE` and `GROVE_LAUNCH_DIR`. Remove both from the
environment of each Ravel task, for example with `env -u`. Then no test
inherits the session's ending channel.

## Decisions (running log)

- 2026-11-12, pre-cut read: Ravel `main` and `main@origin` are
  `12df84bebf4a`, and the default workspace has an empty working copy. The
  manifests say 1.1.0. Neither the local tags nor origin's tags hold
  `v2.0.0`. GitHub has no 2.0.0 release, and v1.1.0 is the latest. The tap
  is clean at `12b53f8c449d`, and its `main` agrees with origin. It holds
  `ravel.rb` at 1.1.0 and `ravel-lite.rb`, with no `ravel@2.rb`. The 1.1.0
  keg has 14 files. Its `bin/ravel` reports 1.1.0, and `end-session`
  refuses with exit 1 outside a session. `task release:doctor` met every
  prerequisite. The recovery table therefore does not apply, and the cut
  runs once.
- No release step runs `release:lifecycle:test`, so the exported
  `HOMEBREW_NO_INSTALL_CLEANUP=1` cannot meet that suite's mutant gap.
- 2026-11-12: one run of `CHANNEL=versioned-2 HOMEBREW_NO_INSTALL_CLEANUP=1
  task release:major` started from Ravel's default workspace, with
  `RAVEL_EXIT_FILE` and `GROVE_LAUNCH_DIR` removed. Preparation passed, and
  `task check` passed 713 tests with 48 skipped. The local cut made the
  annotated tag `v2.0.0` (object `d2f74e6d6343`) at the release commit
  `ed6cedbf9a14`. From here a failure uses the recovery table, not a bump.
- 2026-11-12: the run stopped at `scripts/release-evidence.sh` with
  "permission denied", exit 127, after the install passed. Git records the
  script with mode `100644`, and the Taskfile has no `bash` prefix for it.
  Recovery row "Release evidence failed" applied: the resume ran the script
  through `bash` at the release commit, then `release-book.sh live v2.0.0`.
  Both passed. No second cut. The tag is public, so the repair does not
  serve this cut. It went to the new leaf
  ravel-evidence-step-invocation-k332, before release-readiness-checks-k321.
  Promotion starts `release-promote.sh` through `bash` and reads only the
  evidence file, so it is not affected.
- 2026-11-12, verification: the public assets, GitHub's digests and
  `target/dist` agree. The tap's only change is `Formula/ravel@2.rb`.
  `brew test ravel@2` passed again. The 1.1.0 keg inventory is identical to
  the one before the cut. The receipt is the section "Ravel 2.0.0
  publication receipt" of Grove `docs/coordinated-release-evidence.md`.
  `docs/ravel-v2.0.0-release-evidence.json` is a byte copy of Ravel's
  `target/release-evidence/v2.0.0.json`. The run logs are beside the
  original.
