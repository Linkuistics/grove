# promotion-state-rehearsal-k335

## Goal

Add tap state D to the promotion rehearsal instrument through Ravel's
`release:promote` against the scratch tap. Rehearse fresh D and the upgrades
from A and from C.

## Context

- The node brief, `_promotion-upgrade-rehearsal.md`, its Notes on the
  readiness receipt and the retained-keg manifest, and the instrument and
  record of k333 and k334.
- Ravel `docs/specs/release-channels.md`, "Promotion interface" and
  "Consumer readiness receipt", and `scripts/release-promote.sh`.
- `docs/specs/coordinated-release.md`, row D and the paragraphs after the
  table.

## Done when

- The readiness receipt names the candidate Grove of state C. The promotion
  task runs with `RAVEL_TAP_DIR` on the scratch tap and the private brew first
  on PATH. It uses the published 2.0.0 assets and pushes only to the scratch
  `origin`.
- Fresh D gives a usable pair and the `config.rhai` sample.
- The upgrade from A includes a Ravel-only upgrade with Grove 22.8.0
  installed. The record states what Homebrew does with the outdated dependent.
- The upgrade from C gives a usable pair.
- Each upgrade includes a HOME that holds only `policy.ts`. It shows
  `config_format_retired` with exit 3 and the documented remedy, then a
  working launch after the conversion to `config.rhai`.
- The old 1.1.0 front reports 1.1.0 and refuses `end-session` outside a
  session after each package mutation.
- The record states whether an upgrade changed any file of a retained keg
  other than `INSTALL_RECEIPT.json`, and the deprecation date rendered.
