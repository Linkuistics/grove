# promotion-upgrade-rehearsal-k318 — brief

## Goal

Rehearse fresh installs and upgrades through tap states A to D with real
Homebrew behaviour. Decide before finish whether promotion can preserve the
Grove and Ravel pair.

## Context

- `docs/specs/coordinated-release.md`, "Compatible tap states" and
  "Acceptance and limits".
- Ravel `docs/specs/release-channels.md`, "Promotion without another release",
  and `release:promote` from ravel-release-promote-k306.
- `docs/legacy-driver-transition.md`, "Rehearsal beside a Ravel 2.0 prefix",
  and `task legacy:rehearsal`.

## Done when

- The rehearsal uses a scratch tap and a scratch Homebrew environment. It
  never changes the real tap, the real prefix or the owner's HOME.
- State C uses a candidate Grove formula built from unreleased archives of the
  working result. State D uses the promotion task against the scratch tap and
  the published 2.0.0 assets.
- Fresh installs of A to D, and upgrades from A and from C, give a usable
  pair. One upgrade is a Ravel-only upgrade with an outdated Grove. The
  rehearsal records what Homebrew does with the outdated dependent.
- Each upgrade includes a HOME that holds only `policy.ts`. It shows the
  documented `config_format_retired` refusal and remedy. It then shows a
  working launch after the conversion to `config.rhai`.
- After each package mutation, the old 1.1.0 front reports 1.1.0 and refuses
  `end-session` outside a session.
- Controls fail on a wrong channel, a `versioned-2` cut after promotion,
  mismatched archives, silent schema-3 reuse, a removed retained resource and
  promotion before compatible Grove. Each control is seen to fail.
- A committed record states the commands, the results and the decision:
  proceed to promotion in finish, or stay in state C. A stay in C carries the
  packaging repair as live leaves.

## Notes

Ravel `docs/specs/release-channels.md`, sections "Promotion interface" and
"Consumer readiness receipt", give the exact command line, the receipt
shape, the receipt command and the example receipt
`docs/specs/release-channels-readiness.example.json`.
`RAVEL_TAP_DIR` selects a scratch tap. `release-install.sh` installs from
`linkuistics/taps`, so a scratch Homebrew must tap the scratch clone under
that name.

ravel-release-promote-k328 replaced the retained-keg check. Since Ravel
`5196906162d4`, `release-install.sh` records a manifest of each earlier
`ravel` and `ravel@2` keg before the Homebrew mutation and refuses any later
change. Homebrew's `INSTALL_RECEIPT.json` is compared by presence only. This
rehearsal is the first real Homebrew upgrade under that check. Record whether
an upgrade changes any other file of a retained keg. The deprecation date is
now the tag commit's date at UTC−12.

This leaf may prove bigger than one session. If so, decompose it and do only
the first child.

## Decomposition

The first session decomposed the leaf on 2026-11-12. Each child adds its
section to one record, `docs/promotion-upgrade-rehearsal-record.md`, and
extends one rehearsal instrument and its Taskfile task.

1. scratch-homebrew-rehearsal-k333 builds the instrument. It makes a private
   Homebrew, a scratch HOME and a scratch tap with a scratch `origin`. It
   rehearses tap states A and B and the upgrade from A to B. It checks the old
   1.1.0 front after each mutation and the real prefix, tap and HOME.
2. candidate-state-rehearsal-k334 adds state C. It renders the candidate
   Grove formula from unreleased archives of the working result. It rehearses
   fresh C and the upgrades from A and B to C. Each uses native setup and
   directory setup through `ravel@2`.
3. promotion-state-rehearsal-k335 adds state D through `release:promote`
   against the scratch tap. It rehearses fresh D and the upgrades from A and
   from C. One upgrade changes only Ravel beside an outdated Grove. Each
   upgrade uses a HOME that holds only `policy.ts`. It records the
   retained-keg manifest result.
4. promotion-controls-decision-k336 runs the six controls and sees each fail.
   It completes the record with the decision. Finish either promotes, or
   the grove stays in state C with the packaging repair as live leaves.

## Pointers

- `docs/evaluations/hook-adapters/projected-keg.sh` holds the private
  Homebrew pattern: a `--shared` clone of the owner's Homebrew, its
  portable-ruby links, a read-only `bin/brew` and an `env -i` wrapper.
- `scripts/homebrew-cleanup-control.sh` shows Homebrew's periodic cleanup in a
  scratch prefix.
- `scripts/grove-tap-states.sh` models A to D in a Homebrew-shaped prefix
  without Homebrew's upgrade logic. This node uses real Homebrew.
