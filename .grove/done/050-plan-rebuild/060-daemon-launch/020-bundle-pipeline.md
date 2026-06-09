# 020-bundle-pipeline

**Kind:** work

## Goal

Ship the stock rmux daemon+CLI inside grove's release: build the `rmux` binary from
the pinned crate (version-locked to the linked `rmux-sdk`) for every target, stage it
beside `grove`/`grove-llm`, install it via the Homebrew formula, and assert the
version match in `release-doctor`. Implements ADR-0030 §1–§2.

## Context

The release pipeline (`scripts/release-{doctor,build,publish}.sh` +
`scripts/templates/grove.rb.tmpl`) builds per-target tarballs containing
`grove` + `grove-llm` + LICENSE + README and renders a Homebrew formula. Targets:
`aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu`
(Linux via `cargo zigbuild`, glibc floor 2.17). The `rmux` binary is not in grove's
cargo dep tree, so it must be built from source as a release step.

rmux is pure-Rust with **no openssl/curl** (the `web` feature gates only pure-Rust
crypto), so the macOS→Linux zigbuild cross is plain. Build `--no-default-features` to
drop the web-share server grove doesn't use (deferred to `rmux-web`).

## Done when

- **A `scripts/`-local build input** pins the `rmux` crate as its sole dependency
  (e.g. `scripts/rmux-daemon/Cargo.toml` + its own `Cargo.lock`), buildable via the
  same `cargo zigbuild --release --target <t>[.<glibc>] --no-default-features` path
  `release-build.sh` already uses for grove. (For native macOS, plain
  `cargo build --release --target …` mirrors the existing darwin arm.)
- **`release-build.sh` builds `rmux` per target** and stages the resulting binary
  into `grove-v<ver>-<target>/` alongside `grove`/`grove-llm` (same `cp`-into-stage
  pattern; keep the load-bearing `|| return 1` discipline if built inside a command
  substitution).
- **The version is derived from `Cargo.lock`**: the rmux build pin equals the
  `rmux-sdk` version resolved in grove's `Cargo.lock` (read via `cargo metadata` or a
  lockfile grep). No hand-maintained constant.
- **`release-doctor.sh` asserts the match** — the rmux-daemon build pin == the locked
  `rmux-sdk` version — and checks any new prereq the rmux build needs (likely none
  beyond the existing zig/zigbuild/rust-targets; confirm rmux builds with the current
  toolchain). Fails the punch list on mismatch.
- **The formula installs three binaries**: `grove.rb.tmpl` →
  `bin.install "grove", "grove-llm", "rmux"`, and the `test do` block asserts
  `rmux --version` (alongside the existing grove/grove-llm version assertions).
- A `release-doctor.sh` run passes; a dry `release-build.sh` (or at least the rmux
  build step) produces a working `rmux` per target; `rmux --version` matches the
  locked SDK version.

## Notes

- This is orthogonal to `010-launch-wiring` (build-time bundling vs runtime
  resolution); 010 should land first so grove already knows to find the sibling.
- Keep the rmux build's `Cargo.lock` committed for reproducibility.
- If the rmux build ever needs a system lib (it should not — verified no openssl/curl),
  revisit the zigbuild `--features` the way trellis once needed `vendored_curl`.
- `publish.sh` likely needs no change (it ships whatever `build.sh` staged) — confirm.
