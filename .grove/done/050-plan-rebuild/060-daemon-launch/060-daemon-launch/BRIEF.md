# 060-daemon-launch — brief

**Kind:** planning

## Goal

Make `grove tui` a **walk-away binary** on the **stock** rmux daemon: settle how
the daemon binary is bundled/shipped, how `grove tui` launches and connects to it,
session naming/persistence, and how the fleet singleton + multi-repo model
(ADR-0025/0027) realises under rmux. Grow work leaves for bundling + launch.

## Context

030-engine connects via `Rmux::connect_or_start` against **published rmux 0.5.0**
(SDK crates `rmux-sdk`/`ratatui-rmux` + the stock `rmux` daemon+CLI binary). For a
user who has not installed rmux, `connect_or_start` needs a daemon binary to spawn
— so grove must ship one.

**No fork (ADR-0029).** The original D7 plan to ship grove's *forked* rmux build is
dead — rendered-history capture already ships in stock rmux 0.5.0. grove bundles
the **stock published daemon+CLI**, with no patch to rebase and no forked build in
the release pipeline. This corrects the pre-decomposition leaf text.

## Areas to grill (questions, not answers)

- **Bundling the daemon.** Vendor the stock `rmux` binary into grove's
  distribution and point `connect_or_start` at it via `SDK_DAEMON_BINARY_ENV`?
  How does this interact with the manual release pipeline
  (`scripts/release-{doctor,build,publish}.sh`)? Per-platform binaries?
- **Version pinning.** grove pins `rmux-sdk`/`ratatui-rmux` crate versions *and*
  the bundled daemon+CLI binary version — they must match. How is the match
  enforced/verified at build time? (open-in-editor also shells out to the stock
  `rmux capture-pane` CLI — ADR-0029 — so the CLI binary is shipped too.)
- **Session naming + persistence.** ADR-0027 made the fleet session a singleton
  (`grove-fleet`, a constant since there is no cwd anchor). Under rmux: one
  detached session reused across `grove tui` restarts (`ensure_session`
  `CreateOrReuse`)? Does session persistence give detach/reattach for free (ties
  to the deferred `rmux-web` grove — flag, don't solve)?
- **Launch flow.** What `grove tui` does end-to-end: resolve fleet from config →
  `connect_or_start` (spawn bundled daemon if none) → `ensure_session` → run the
  draw loop. First-run UX when no daemon is running.
- **Fleet + multi-repo under rmux.** How `MultiRepoView` groves map onto rmux
  panes/sessions across repos (ADR-0025 manifest+scan, ADR-0027 no-cwd-anchor)
  — mostly survives below the seam; confirm nothing rmux-specific breaks it.

## Done when

The bundling, launch flow, session/persistence model, and version-pinning approach
are settled; work leaves are grown; any ADR-0027 amendment under rmux is recorded.

## Decisions (running log)

**The seam is one binary, one env var (bootstrap finding).** The SDK's
`connect_or_start` spawns the daemon via `Command::new(daemon_binary())` where
`daemon_binary()` = `$RMUX_SDK_DAEMON_BINARY` else bare `rmux` on PATH
(`rmux-sdk-0.5.0/src/handles/rmux/connect.rs:177`). `editor.rs:73` (`rmux_binary()`)
resolves the *same* env var for the `capture-pane` shell-out (ADR-0029). So
**daemon-spawn and capture hit one binary, redirected by one process env var** —
grove points both at a bundled `rmux` by setting `RMUX_SDK_DAEMON_BINARY` once at
startup. No SDK fork, no two paths. grove's own build pulls only the `rmux-sdk` +
`ratatui-rmux` *crates*; the `rmux` *binary* is absent from the dep tree, so it must
be shipped separately (this leaf's whole job).

**Q1 — Bundling mechanism: vendor upstream prebuilt into grove's tarball (A).**
`release-build` downloads the matching per-target `rmux` from the `helvesec/rmux`
v0.5.0 GitHub release, SHA-verifies it against the release's `SHA256SUMS`, stages it
beside `grove`/`grove-llm`; the formula installs all three into `bin/`. Chosen over
*B (brew `depends_on`)* — only covers the brew channel, leaving tarball / direct-
download / `cargo install` users with no daemon (fails the "walk-away binary" goal) —
and over *C (build rmux from source per target)* — cross-compiles foreign code
(zigbuild + possible system-lib needs, the isahc/openssl hazard the build script
warns of) for no benefit now that ADR-0029 retired the fork. A gives an exact
version match by construction and a SHA-verified, present-on-every-channel binary.
(User: "A".)

**Verified gap (informs Q2).** Upstream rmux v0.5.0 ships prebuilt tarballs for
`macos-aarch64` (= grove's `aarch64-apple-darwin`) and `linux-x86_64` (= grove's
`x86_64-unknown-linux-gnu`), plus macos-x86_64 / windows / a `SHA256SUMS`. It does
**not** ship `aarch64-unknown-linux-gnu` — grove's third release target. So A covers
two of three; the ARM-Linux target needs its own call (Q2).

**Q2 — REVISES Q1: build the stock `rmux` from source for ALL targets, not vendor
prebuilts.** Rather than mixing prebuilt-download (two targets) with a source-build
for the ARM-Linux gap, the user chose to **build the stock `rmux` binary from source
for every grove release target** — uniformity over the hybrid. (User: "Let's source
build all the platforms we need.") This supersedes Q1's "vendor prebuilt": grove no
longer downloads upstream release assets at all. Consequences, all favourable:
- **Version-pinning becomes near-automatic (folds Q3).** The `rmux` *crate* version
  is the pin; built in lockstep with the resolved `rmux-sdk`/`ratatui-rmux` versions
  — see Q3 for the single-source-of-truth mechanism.
- **No upstream-prebuilt dependency.** No reliance on upstream asset naming/coverage
  (the ARM-Linux gap simply vanishes — we build it like the rest), no SHA-download
  trust step; full source provenance.
- **Cross-compile is plain.** rmux has **no openssl/curl anywhere** (checked: the
  `web` feature gates only pure-Rust crypto — base64/getrandom/hkdf/httparse/
  rmux-web-crypto/serde/sha1/sha2/subtle/toml/zeroize). The isahc/openssl hazard that
  forced trellis's `vendored_curl` does **not** recur; the same zigbuild path grove
  already uses for its own Linux targets builds rmux cleanly.
- **Build `rmux --no-default-features`.** `rmux`'s `default = ["web"]` pulls
  `rmux-server/web` — the HTTP web-share server grove **defers to the `rmux-web`
  grove**. grove's daemon needs (ptys, sessions, panes, `capture-pane` IPC) are all
  in the non-web core, so drop web: leaner binary, fewer moving parts. (If `rmux-web`
  ever wants web-share, it revisits the feature set — flagged there, not here.)

**Q3 — Daemon version derived from `Cargo.lock`, asserted at release.** The bundled
`rmux` is built at exactly the version grove's `rmux-sdk` resolves to in `Cargo.lock`
(the rmux workspace publishes all crates in lockstep, so rmux ⇔ rmux-sdk ⇔
ratatui-rmux at one version). `release-doctor` asserts the daemon-build pin equals
the locked `rmux-sdk` version, so a caret bump can't ship a mismatched daemon. Chosen
over hand-maintained `=0.5.0` pins (a constant that silently drifts). Single source
of truth = `Cargo.lock`. **Build mechanism** (work-leaf detail, not grilled): a tiny
`scripts/`-local cargo manifest whose sole dep is the pinned `rmux` crate, built
through grove's existing `cargo zigbuild` path with `--no-default-features`, output
staged beside `grove`/`grove-llm`; the formula installs all three into `bin/`.

**Q4 — Launch wiring: resolve bundled `rmux` as sibling-of-exe, set
`RMUX_SDK_DAEMON_BINARY` if unset; user override wins.** Installed grove puts all
three binaries in one `bin/`, so the bundled `rmux` is a sibling of
`current_exe()`. At `grove tui` startup grove resolves `current_exe().parent()/rmux`
and, when present **and the var is unset**, sets `RMUX_SDK_DAEMON_BINARY` to it — one
set covers both the SDK daemon-spawn (`connect.rs:177`) and the `capture-pane`
shell-out (`editor.rs:73`). Precedence: **user override → sibling-of-exe → bare
`rmux` on PATH** (User: "user override wins"). PATH still catches dev builds
(`cargo run`) and `cargo install grove`. First-run is automatic: daemon not running →
SDK spawns the bundled `rmux` → connects.
- **Impl note (Rust-2024 footgun, for the work leaf):** `std::env::set_var` is
  `unsafe`/unsound once threads exist. Resolve-and-set must happen in the
  **synchronous `grove tui` entry, before the tokio runtime + input-reader thread
  spawn** — not inside `run_app`'s async body where the reader thread is already up.
- **First-run failure UX:** if no `rmux` resolves anywhere, `connect_or_start` errors
  ("connect or start rmux daemon"); surface a friendly "rmux not found — it ships
  with grove; reinstall, or set `RMUX_SDK_DAEMON_BINARY`" hint rather than the raw
  transport error. Minor; work-leaf detail.

**Q5 — Persist indefinitely, reattach, no teardown; fleet/multi-repo confirmed
intact.** The detached `grove-fleet` singleton (`CreateOrReuse`, ADR-0027) + daemon
**outlive each `grove tui` exit** — agents keep running, next launch reattaches them
live (the seed of detach/reattach). No grove-driven kill/restart now; stale panes
from retired groves are harmless (a gone grove just isn't opened; panes re-pointed by
`(grove, role)` key). **Fleet/multi-repo needs no work:** `run_app` resolves
`resolve_repo_roots` → `MultiRepoView::scan` → fs-watch re-scan below the seam; rmux
owns only the ptys behind panes. ADR-0025 (manifest+scan) and ADR-0027 (no-cwd-anchor
singleton) survive the substrate change untouched. Detach/reattach UX + daemon
lifecycle controls → deferred **`rmux-web`** grove. (User: "persist, no teardown.")

## Outcome — ADR + two work leaves

The model is settled, so this planning leaf decomposes into a node with two work
leaves and records the durable decision as **ADR-0030** (bundling/launch model;
ADR-0028's consequences already named "bundling the daemon binary is 050/060's job"
— this closes it). The two work leaves are orthogonal (Rust runtime vs shell release
pipeline), with different verification:

- **`060/010-launch-wiring`** (work) — grove resolves the bundled daemon
  (sibling-of-exe → `RMUX_SDK_DAEMON_BINARY`-if-unset → PATH; user override wins),
  set **before** the tokio runtime + reader thread spawn (Rust-2024 `set_var`
  safety), friendly first-run "rmux not found" error. Headless-testable resolver.
  Independent of the pipeline (works against a dev `rmux` on PATH) → do first.
- **`060/020-bundle-pipeline`** (work) — `release-build` builds stock `rmux` from a
  `scripts/`-local pinned manifest (`--no-default-features`) per-target via the
  existing `cargo zigbuild` path, stages it beside `grove`/`grove-llm`; the formula
  installs all three + a `rmux --version` test; `release-doctor` asserts the rmux
  build pin == the locked `rmux-sdk` version (Q3) and checks any new prereqs.

## Notes
