# 100-framework-fork-bringup

**Kind:** work (foundation; spike-then-build — **decompose if needed** when picked)

## Goal

Hard-fork zellij into grove's cargo workspace as the **[[trellis framework]]**
crate (provisional name `trellis`), establish the **one-way framework↔grove seam**,
do a **minimal rebrand + MIT attribution**, and get it **building on the dev
platform**. This is the foundation the rest of the fork path (110–150) sits on.

## Context

- **ADR-0020** is the spine: deep hard fork, grove-first, extract-later. grove
  becomes a based-on-zellij codebase; the non-grove parts are modularised as the
  publishable [[trellis framework]] (extracted to its own repo *later*).
- **Hard fork, not tracking:** vendor zellij's source into the workspace and **own
  it** — no rebase, no upstream merge. Record the source commit/tag we forked from
  (provenance) but expect to diverge freely.
- **Seam:** `trellis` is a workspace crate that **must not depend on the grove
  crate** (compiler-enforced one-way dependency — this is what keeps grove-isms out
  of the framework and makes a later clean extraction possible).
- **License:** zellij is **MIT** — confirm by reading its actual `LICENSE` at fork
  time; preserve the copyright notice + MIT text (e.g. `LICENSES/zellij.LICENSE`),
  consistent with how this repo already vendors third-party licenses.
- **Rebrand (minimal here):** strip zellij branding from chrome/startup we control;
  set grove's defaults *in source* rather than via the old bundled-config
  `clear-defaults`/bars gymnastics (ADR-0020 makes those unnecessary). Full
  rebrand/public-naming is deferred to the extraction leaf.
- **Hosting-API shape is build-discovered here** (ADR-0020 Notes): library-you-link
  (`run(app)`, app owns `main`) vs runtime-you-plug-into. Spike both against the
  real zellij client/server internals; **library-idiomatic is preferred** but the
  call is empirical. Whatever lands is the seam 110 builds the MVP API on.

## Done when

- The forked zellij source lives in the workspace as the `trellis` crate, with the
  one-way seam in place (a deliberate grove→trellis dependency compiles; a
  trellis→grove dependency does **not**).
- zellij's MIT license + attribution are preserved in-tree; provenance (forked-from
  commit) recorded.
- `cargo build` produces a working binary that launches the forked mux on the dev
  platform (a bare shell pane is enough — native grove surfaces are 110+).
- The chosen hosting-API direction (library vs runtime) is recorded with the
  evidence, so 110 can build the MVP host API against it.

## Notes

- **Smallest thing that earns its place** (constraint 4): get it *building and
  launchable*, not feature-complete. Per-platform builds, GraphQL, and the
  observability API are explicitly **later/lazy** (ADR-0020 §7) — do not pull them
  forward here.
- The built 010–080 code (proxy seam, WASM nav, `zellij action` driving) is **not**
  ported here — it is superseded by ADR-0020 and stays in `done/`+history. 110
  rebuilds those concerns natively.
- This leaf is large; if the fork bring-up and the hosting-API spike each want a
  session, `grove-llm leaf-decompose` it when picked.
