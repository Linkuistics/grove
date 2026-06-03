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

- Upstream is watched **only as a read-only CVE / advisory feed** (RustSec +
  GitHub security advisories on zellij and its dependency tree).
- Relevant security fixes are **assessed and hand-patched**, never rebased.
- Behavioural upstream fixes/features are ours to notice and **hand-port** if
  wanted.

## Local edits to the vendored tree

Record every deliberate divergence from the upstream `v0.44.3` tree here, so the
fork's drift from upstream stays auditable for CVE hand-porting.

- **Removed `rust-toolchain.toml`** (was pinning channel `1.92.0`): the fork
  builds with grove's workspace toolchain, not upstream's pinned channel.
