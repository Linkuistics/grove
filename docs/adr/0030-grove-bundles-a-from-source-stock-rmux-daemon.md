# 30. grove bundles a from-source, version-locked stock rmux daemon; the TUI redirects to it via one env var

- Status: **accepted**
- Date: 2026-06-09
- Deciders: Antony Blakey (with grove `rmux-substrate` 050-plan-rebuild/060-daemon-launch)
- Builds on / closes: ADR-0028 (rmux substrate) — whose Consequences said "bundling
  the daemon binary is 050/060's job"; this is that job. ADR-0029 (rendered history
  via stock `rmux capture-pane`) — both the daemon-spawn and the capture shell-out
  resolve the *same* binary the *same* way, so bundling one binary serves both.
  ADR-0027 (singleton fleet session) — the persistence posture below is the no-cwd
  singleton seen from the daemon-lifecycle side.

## Context

After ADR-0028 grove is a ratatui app that drives an rmux daemon over `rmux-sdk`;
after ADR-0029 it also shells out to the stock `rmux capture-pane` CLI for
rendered-history. grove's own build pulls only the `rmux-sdk` + `ratatui-rmux`
*crates* — the `rmux` *binary* (daemon + CLI, one executable) is **not** in grove's
dependency tree. So a user who installs grove but has never installed rmux has no
daemon: `Rmux::connect_or_start` would fail to spawn one. For `grove tui` to be a
walk-away binary, grove must ship the daemon.

**The seam is one binary, one env var.** The SDK spawns the daemon via
`Command::new(daemon_binary())`, where `daemon_binary()` is `$RMUX_SDK_DAEMON_BINARY`
else bare `rmux` on `PATH` (`rmux-sdk-0.5.0/src/handles/rmux/connect.rs:177`).
`src/tui/editor.rs:73` (`rmux_binary()`) resolves the *same* env var for the
`capture-pane` shell-out (ADR-0029 D-D). So daemon-spawn and capture hit one binary,
redirected by one process env var — grove points both at a bundled `rmux` by setting
`RMUX_SDK_DAEMON_BINARY` once at startup. No SDK fork, no second code path.

## Decision

**grove builds the stock `rmux` binary from source, version-locked to the linked
SDK, bundles it beside its own binaries, and the TUI redirects to it via
`RMUX_SDK_DAEMON_BINARY`.**

**1. Build from source for every release target (not vendor upstream prebuilts).**
`release-build` builds the stock `rmux` binary from the pinned `rmux` crate for each
of grove's targets, through the same `cargo zigbuild` path grove already uses, and
stages it beside `grove`/`grove-llm`; the Homebrew formula installs all three into
`bin/`. Chosen over vendoring upstream's prebuilt release assets and over a Homebrew
`depends_on`. Why source-build-all:

- **Uniformity + coverage.** Upstream rmux v0.5.0 ships prebuilts for
  `aarch64-apple-darwin` and `x86_64-unknown-linux-gnu` but **not**
  `aarch64-unknown-linux-gnu`, a grove release target. Source-building all three
  closes that gap with one mechanism instead of a prebuilt/source hybrid.
- **Provenance + no upstream-asset coupling.** No trust in downloaded binaries, no
  dependence on upstream's asset naming or per-target coverage.
- **The cross-compile is plain.** rmux has **no openssl/curl anywhere** — its `web`
  feature gates only pure-Rust crypto (base64/getrandom/hkdf/httparse/
  rmux-web-crypto/serde/sha1/sha2/subtle/toml/zeroize). The isahc/openssl hazard that
  forced trellis's `vendored_curl` does not recur.

Build `rmux` **`--no-default-features`**: `default = ["web"]` pulls
`rmux-server/web`, the HTTP web-share server grove **defers to the `rmux-web`
grove**. grove's daemon needs (ptys, sessions, panes, `capture-pane` IPC) are all in
the non-web core. *Rejected — Homebrew `depends_on`:* covers only the brew channel,
leaving tarball / direct-download / `cargo install` users with no daemon.

**2. The daemon version is derived from `Cargo.lock` and asserted at release.** The
rmux workspace publishes all crates in lockstep (rmux ⇔ rmux-sdk ⇔ ratatui-rmux at
one version), and the daemon must speak the SDK's wire protocol. So the daemon is
built at exactly the version grove's `rmux-sdk` resolves to in `Cargo.lock`, and
`release-doctor` asserts the daemon-build pin equals that locked version. Single
source of truth = `Cargo.lock`; a caret bump of `rmux-sdk` carries the daemon with
it, and a mismatch fails the release rather than shipping silently. *Rejected —
hand-maintained `=0.5.0` pins:* a constant that drifts from the resolved version.

**3. The TUI resolves the bundled daemon as a sibling of its own executable and sets
`RMUX_SDK_DAEMON_BINARY` if unset; a user override wins.** Installed grove puts all
three binaries in one `bin/`, so the bundled `rmux` is
`current_exe().parent()/rmux`. At `grove tui` startup grove sets
`RMUX_SDK_DAEMON_BINARY` to that path **when the file exists and the var is unset** —
one set covers both the SDK daemon-spawn and the `capture-pane` shell-out.
Precedence: **user override → sibling-of-exe → bare `rmux` on `PATH`** (PATH catches
dev builds and `cargo install grove`). The set must happen in the **synchronous
`grove tui` entry, before the tokio runtime + input-reader thread spawn** — Rust 2024
makes `std::env::set_var` unsound once threads exist.

**4. The daemon + `grove-fleet` session persist indefinitely; no grove-driven
teardown now.** The detached singleton session (ADR-0027, `CreateOrReuse`) and its
daemon outlive each `grove tui` exit — agents keep running, the next launch
reattaches them live. grove provides no kill/restart affordance at interim parity;
stale panes from retired groves are harmless (a gone grove is simply not opened;
panes are re-pointed by `(grove, role)` key). Detach/reattach UX and daemon-lifecycle
controls are deferred to the **`rmux-web`** grove.

## Consequences

- **`grove tui` is a walk-away binary on every channel** (Homebrew + tarball +
  direct download): the daemon is physically present beside grove, version-matched by
  construction, and the TUI finds it with zero config. `cargo install grove` and dev
  builds fall back to a `PATH` `rmux`.
- **The release pipeline gains a from-source rmux build step** (a `scripts/`-local
  pinned manifest, `cargo zigbuild --no-default-features` per target) and a
  `release-doctor` version-match assertion. The formula installs three binaries and
  tests `rmux --version`. (Implemented in `060/020-bundle-pipeline`.)
- **grove's `grove tui` entry sets one process env var before the runtime starts**
  (`060/010-launch-wiring`); both the SDK and the ADR-0029 capture path inherit it.
- **No fork, no patch to rebase, no upstream PR** — the from-source build is of the
  *stock* crate at a pinned version; ADR-0029's zero-forks thesis holds.
- **Fleet/multi-repo (ADR-0025/0027) is untouched** by the substrate change: repo
  discovery is below the presentation seam; rmux owns only the ptys behind panes.
- **A future `rmux` bump is a one-line version change** that flows through
  `Cargo.lock` to the daemon build automatically; the asserted match is the guard.
- **If `rmux-web` ever wants web-share**, it revisits the `--no-default-features`
  choice (re-enabling `web` for the bundled daemon); flagged there, not here.
