# trellis — fork provenance

`crates/trellis/` is grove's **hard fork** of [zellij](https://github.com/zellij-org/zellij),
vendored as the [[trellis framework]] (provisional name) per **ADR-0020**.

## Forked from

- **Upstream:** https://github.com/zellij-org/zellij
- **Tag:** `v0.44.3`
- **Commit:** `55a2121b73dce4be624cda425a960e893000777c`
- **Vendored:** 2026-06-03, by `git archive v0.44.3` (tracked tree only — no `.git`,
  no untracked files).
- **License:** MIT (preserved in-tree at `crates/trellis/LICENSE.md`, mirrored to
  `content/LICENSES/zellij.LICENSE`). Copyright (c) 2020 Zellij contributors.

## Fork policy (ADR-0020 §3)

This is a **hard fork, not a tracking fork**. We **own** this source: no rebase
cadence, no adopting upstream code wholesale.

**No upstream-compatibility obligation.** We owe upstream zellij *nothing* —
not config compatibility, not layout/keybind compatibility, not behavioural
parity, not API stability. trellis exists to be the **best fit-for-purpose
substrate for grove**, so we change, delete, or rewrite whatever serves that
goal. Upstream-divergent edits are expected and encouraged, not a cost to
minimise; the only reason we still record provenance and divergences below is to
keep CVE hand-porting auditable, **not** to preserve a path back to upstream.

- Upstream is watched **only as a read-only CVE / advisory feed** (RustSec +
  GitHub security advisories on zellij and its dependency tree).
- Relevant security fixes are **assessed and hand-patched**, never rebased.
- Behavioural upstream fixes/features are ours to notice and **hand-port** if
  wanted — purely at our discretion, never an obligation.

## Local edits to the vendored tree

Record every deliberate divergence from the upstream `v0.44.3` tree here, so the
fork's drift from upstream stays auditable for CVE hand-porting.

- **Removed `rust-toolchain.toml`** (was pinning channel `1.92.0`): the fork
  builds with grove's workspace toolchain, not upstream's pinned channel.

### Bring-up: build + minimal rebrand (leaf 020, ADR-0020)

- **`Cargo.toml`: dropped `plugins_from_target` from the default features**
  (`default = ["vendored_curl", "web_server_capability"]`). Upstream's debug
  builds default to loading the builtin plugin wasm from
  `target/wasm32-wasip1/debug/`, which assumes a `cargo xtask build` step. grove
  consumes zellij as a finished multiplexer and does not author its default
  plugins, so a plain `cargo build` now embeds the committed
  `assets/plugins/*.wasm` and produces a launchable binary with no wasm
  toolchain. The feature is still defined for opt-in.
- **`zellij-utils/assets/layouts/default.kdl`: no bars.** Default startup layout
  is a single bare pane; the tab-bar / status-bar plugin panes are removed.
  grove drives its own chrome natively (110+).
- **`zellij-utils/assets/layouts/default.swap.kdl`: no bars.** The `ui`
  tab_template no longer injects the tab-bar / status-bar panes, so auto-layout
  swaps stay bars-free; the responsive `max_panes` geometry is unchanged.
- **`zellij-utils/assets/config/default.kdl`: `default_mode "locked"`.** grove
  panes are harness / shell surfaces; keystrokes pass through and grove drives
  navigation natively. The locked-mode `Ctrl g` unlock binding is retained.
