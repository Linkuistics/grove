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
