# scratch-homebrew-rehearsal-k333

## Goal

Build the promotion rehearsal instrument with real Homebrew in a scratch
environment. Rehearse tap states A and B and the upgrade from A to B.

## Context

- The node brief, `_promotion-upgrade-rehearsal.md`, and its Pointers.
- `docs/specs/coordinated-release.md`, "Compatible tap states", rows A and B.
- Ravel `docs/specs/release-channels.md`: the first `ravel@2` installation
  uses `--skip-link`.

## Done when

- One instrument and one Taskfile task make a private Homebrew, a scratch
  HOME and a scratch tap. The scratch tap is tapped as `linkuistics/taps`
  and has a scratch bare `origin`. The instrument never writes to the real
  tap, the real prefix or the owner's HOME, and it shows that with digests
  before and after.
- Fresh A, fresh B and the upgrade from A to B each give a usable pair. The
  Homebrew formula tests of `grove` and `ravel` pass, and Grove selects Ravel
  1.1.0 by default.
- After each package mutation, the 1.1.0 front of the scratch prefix and the
  real 1.1.0 front report 1.1.0 and refuse `end-session` outside a session.
- `docs/promotion-upgrade-rehearsal-record.md` holds the commands, the
  results and the Homebrew output of the upgrade.

## Decisions (running log)

- The leaf decomposed into four impl children on 2026-11-12. The node brief
  states the split. The rehearsal needs a real Homebrew, four tap states, a
  promotion run and six controls, which exceed one session.
- The instrument is one Bash script, `scripts/promotion-rehearsal.sh`, with
  `task promotion:rehearsal`. It follows the private Homebrew pattern of
  `docs/evaluations/hook-adapters/projected-keg.sh`. Later children add
  scenarios to it.
- The scratch tap is a bare clone of the real tap's Git history. Its `main`
  names the state: A is `12b53f8c449d`, the commit before `ravel@2`, and B is
  `f7f9b9b0bc4d`. A scenario moves state by a fetch and fast-forward of the
  tapped clone, which is the tap step of `brew update`. It does not run
  `brew update`, because that moves the private Homebrew itself.
- The scratch HOME copies the owner's `brew.env`, and each mutation sets
  `HOMEBREW_NO_INSTALL_CLEANUP=1`. The rehearsal measures the owner's path.
- Fresh B installs `ravel@2` first with `--skip-link` on an empty prefix. The
  upgrade from A to B installs it with Ravel 1.1.0 present, without the flag.
  The two scenarios cover both installation cases of row B.
- Every old-front check runs under `env -i`. Ravel 1.1.0 ends a session
  through the inherited `RAVEL_EXIT_FILE`, so a check with the inherited
  environment could end this live session.
- Homebrew 7.0.13 refuses a dependency from an untrusted tap, and tapping
  trusts nothing. Every new user's documented install fails at every tap
  state. The rehearsal records the refusal in fresh-a and then runs
  `brew trust --tap linkuistics/taps`, as Homebrew's message directs. The
  documentation fix is a new concern, so homebrew-tap-trust-docs-k337 owns it,
  before cross-repo-docs-reconciliation-k319.
