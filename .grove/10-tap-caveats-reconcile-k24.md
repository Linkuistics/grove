# tap-caveats-reconcile-k24

**Kind:** impl

## Goal

Correct the `linkuistics-herdr` Homebrew formula's `caveats`, which tell the user
the opposite of a decided ADR.

## Context

Surfaced by `herdr-notes-reverify-k17`. `Formula/linkuistics-herdr.rb` in
`~/Development/homebrew-taps` currently says of the authority patch:

> Offered upstream as a `fix:` PR from the `authority-fix` branch; drop it from
> the carry if that merges.

and closes with:

> Remove this formula and reinstall stock `herdr` once both patches are upstream.

ADR *herdr-optional-ui* records that offering the patch upstream was **considered
and rejected**, and that the carry is **permanent** — nothing is expected to end
it except upstream reaching the same separation independently. `brew info
linkuistics-herdr` prints these caveats, so the stalest possible account of the
decision is the one a user is most likely to read.

Everything else in the formula verified correct: the pinned revision matches
`ui-layout` HEAD, the version follows the `-linkuistics.<seq>` scheme, and the
`ZIG` override points at the `zig@0.15` keg exactly as the fork-maintenance spec
requires.

## Done when

- The caveats describe a permanent carry, and name what would actually end it
  (upstream separating session identity from lifecycle state on its own), rather
  than a pending PR.
- The `ui.layout` half of the caveats is checked against the same bar — it is
  described as "two `feat:` commits, unsubmitted upstream", which may or may not
  still be the intent now that upstreaming is closed as a policy.
- Committed in `homebrew-taps`, not here. No version bump: caveats do not change
  the build, so this must not force a reinstall.

## Notes

Low priority and independent of everything else in this grove — it is a
correctness fix to user-facing text, not a behaviour change.

**Do not** read this leaf as an opening to reconsider upstreaming. The decision
is made; this is only about the formula agreeing with it.
