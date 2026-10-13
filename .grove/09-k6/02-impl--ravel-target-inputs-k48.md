# ravel-target-inputs-k48


## Goal
Supply validated, independent Ravel layouts for all Grove delivery test targets.

## Context

Read k11's terminal handoff and Ravel docs/publication-evidence.md for the exact
0.1.0 immutable URLs and SHA-256 pins. Reuse existing archive/native/container/
CPU-floor/glibc/QEMU seams. This is the expand step: retain existing delivery
until k49 can switch its consumers atomically.

## Done when

- Reusable tasks obtain each supported target's exact Ravel archive, verify digest, target and full manifest before extraction/invocation, and expose separate-prefix layouts. Missing/wrong archives fail without skips or host Bun/Node installation.
- Real released layouts and tamper/wrong-target/missing-input controls pass, including Linux/QEMU. Keep result/context boundaries public and versions independent from Grove.
- Document the interface k49 uses for installed smoke tests and preserve complete frozen logs. No release, installation changes or original-driver mutation.

## Notes
