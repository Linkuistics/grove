# 010-vendor-and-seam

**Kind:** work (foundation)

## Goal

Vendor zellij **0.44.3** into grove's cargo workspace as the **[[trellis
framework]]** (provisional crate name `trellis`), by **plain copy** (source tree
minus its `.git`), and establish the **compiler-enforced one-way seam**:
`grove → trellis` compiles, `trellis → grove` must not. Get the vendored
crates **compiling inside grove's workspace** (launch/rebrand is 020).

## Context

- See the node `BRIEF.md` for the ADR-0020 spine (deep hard fork, grove-first,
  extract-later) and the full Goal/Done-when carried from the original leaf.
- **Vendoring = plain copy + recorded SHA** (user decision, this session): copy
  zellij 0.44.3's source into the workspace, record the forked-from tag/commit
  in a provenance file; **no** git-subtree, **no** upstream tracking (ADR-0020 §3
  — upstream is a read-only CVE feed only).
- zellij is itself a multi-crate workspace (`zellij` bin, `zellij-client`,
  `zellij-server`, `zellij-utils`, `zellij-tile`, `zellij-tile-utils`,
  `default-plugins/*`, …). Decide how those nest under grove's workspace
  (path deps; nested vs flattened members) as part of the integration.
- The existing `crates/grove-nav` already pins `zellij-tile = "0.44"`; keep an
  eye on whether the vendored `zellij-tile` should now be the source of truth.
- **License:** zellij is MIT — confirm by reading its actual `LICENSE` at fork
  time; preserve the copyright notice + MIT text (e.g. `LICENSES/zellij.LICENSE`),
  consistent with how this repo vendors third-party licenses.

## Done when

- zellij 0.44.3 source lives in the workspace as the `trellis` crate(s),
  integrated so `cargo build` resolves them (full launch is 020).
- The one-way seam holds: a deliberate `grove → trellis` dependency compiles;
  a `trellis → grove` dependency does **not** (demonstrate/record how it's
  enforced — workspace structure, no path-dep back-edge).
- MIT license + attribution preserved in-tree; provenance (forked-from
  tag/commit) recorded in a tracked file.

## Notes

- **Smallest thing that earns its place** (constraint 4): just integrate +
  compile. Launching the mux and rebrand are 020; the hosting-API spike is 030.
- Do **not** port the built 010–080 code (proxy seam, WASM nav, `zellij action`
  driving) — superseded by ADR-0020, stays in `done/`.
