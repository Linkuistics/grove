# release-doctor-toolchain-gap-k27

**Kind:** impl

## Goal

Make the release path work without the operator remembering two undocumented
workarounds: `scripts/release-doctor.sh` should fail on the toolchain
misconfiguration that actually breaks a release build instead of passing while
the build dies, and `cargo release` should not refuse the detached HEAD that jj
colocation always leaves in place.

## Context — the second friction, found in `observe-mid-turn-live-k31`

`cargo release minor --execute` **cannot cut a release from this repo as
configured**:

```
error: cannot release from branch `HEAD` as it doesn't match `*`, `!HEAD`;
       either switch to an allowed branch or add this branch to `allow-branch`
```

jj colocation keeps git's HEAD **detached** — that is jj's normal resting state,
not a broken checkout — and cargo-release's default `allow-branch` is
`["*", "!HEAD"]`, which excludes exactly it. Passing `--allow-branch HEAD`
works, and the result is clean: git makes the detached commit and the tag, jj
imports it (`Reset the working copy parent to the new Git HEAD`), and
`jj bookmark set main -r <release-change>` puts the bookmark on it. No duplicate
commit, no attached-HEAD fight.

So the fix is one line in `release.toml` — `allow-branch = ["*", "HEAD"]` — plus
a comment saying *why* (this repo is always jj-colocated, so HEAD is always
detached). Same shape as the toolchain gap below: a workaround the operator must
remember every time is weaker than configuration that is correct by
construction.

Both frictions cost `observe-mid-turn-live-k31` a release cut. Neither is
recorded anywhere the next releaser would look — `release.toml`'s own usage
comment still says plain `cargo release minor --execute`, which does not work.

## Context — the toolchain gap

Found the hard way in `ship-release-k25`: the doctor printed
`✓ rustup target: aarch64-unknown-linux-gnu` and `all prerequisites met`, and
`release-build.sh` then died on that exact target with

```
error[E0463]: can't find crate for `std`
  = note: the `aarch64-unknown-linux-gnu` target may not be installed
```

Not a contradiction — the two ask **different tools**. The doctor asks *rustup*
what is installed; the build invokes whatever `cargo` resolves to on `PATH`. On
this machine `command -v cargo` is `/opt/homebrew/bin/cargo`, and Homebrew's
rustc knows nothing about rustup's installed targets. Exporting
`PATH="$HOME/.cargo/bin:$PATH"` fixed it and all three targets built.

This is the *same* trap `docs/specs/herdr-fork-maintenance.md` already records
under "rustup's cargo must win over Homebrew's" — written for building the herdr
fork, and it bites grove's own release just as hard. That spec is about a
repository we do not control, so the knowledge should not have to be borrowed
from it to release grove.

## Done when

- `release-doctor.sh` checks the toolchain the **build** will use, not only what
  rustup reports — e.g. resolve `command -v cargo` and fail (or warn loudly) when
  it is not rustup's shim, and/or ask the resolved `rustc` for its target list
  rather than asking `rustup`.
- The failure names the remediation, in the doctor's existing punch-list style,
  the way its other checks do.
- Whether the release scripts should *set* the PATH themselves rather than
  diagnose it is a live question — a diagnosis the operator must act on every
  time is weaker than a build that is correct by construction. Decide, and record
  which and why.
- Consider whether the herdr-fork spec should cite this rather than duplicating
  it, once the grove-side check exists.
- `cargo release` cuts a release from a jj-colocated checkout without a flag —
  `allow-branch` in `release.toml` admits detached HEAD, and the usage comment at
  the head of that file is corrected to whatever command actually works.

## Notes

Cheap and self-contained; sequenced last because it costs a release only one
re-run once you know the fix, and this grove's live leaves all gate the status
surface. Both frictions have now cost two releases (`ship-release-k25`,
`observe-mid-turn-live-k31`) rather than one — the toolchain gap has bitten
twice, so "one re-run" is a per-releaser cost, not a one-off.

The workarounds, for anyone cutting a release before this leaf lands:

```sh
export PATH="$HOME/.cargo/bin:$PATH"
cargo release minor --allow-branch HEAD --execute
```
