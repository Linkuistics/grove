# release-doctor-toolchain-gap-k27

**Kind:** impl

## Goal

Make `scripts/release-doctor.sh` fail on the toolchain misconfiguration that
actually breaks a release build, instead of passing while the build dies.

## Context

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

## Notes

Cheap and self-contained; sequenced last because it costs a release only one
re-run once you know the fix, and this grove's live leaves all gate the status
surface.
