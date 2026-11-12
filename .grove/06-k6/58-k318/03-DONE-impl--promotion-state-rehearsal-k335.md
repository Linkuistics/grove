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

## Decisions (running log)

- The candidate label changes from `22.9.0-rc.1` to the next minor, `22.9.0`,
  for every candidate scenario. Ravel's `verify_readiness` admits only an
  `X.Y.Z` `consumer.version`, and the tap's Grove formula must carry the same
  version. The `-rc.1` label therefore cannot reach state D. The real state C
  holds a released Grove `X.Y.Z`, so the release-shaped label is also the
  faithful one. The label stays in the scratch tap and the work directory, so
  it publishes and reserves no public version. The rule against reserving
  22.9.0 concerns public identities. The other reason of k334, that a later
  real 22.9.0 supersedes the candidate, concerns only a scratch prefix that
  the run discards. The full run repeats states A to C with the new label.
- The promotion runs as `task release:promote` in a scratch colocated jj
  clone of Ravel's `main`, `WORK/ravel`. A run in Ravel's own workspace
  would write `target/release-evidence/v2.0.0.promotion.json` there, which a
  later real promotion could misread. The clone's `origin` fetches from the
  real GitHub repository for the tag, Release and book checks. Its push URL
  is a path that does not exist.
- A `brew` wrapper first on `PATH` gives Ravel's scripts the scenario's
  private Homebrew with its own HOME. `release-install.sh` runs `brew update`.
  In Homebrew's default mode that moves the private Homebrew to the latest
  stable tag, `7.0.13`, away from the owner's commit. The owner's Homebrew
  has developer mode on, so the promotion scenario turns developer mode on
  first, and the record compares Homebrew's commit before and after.
- Grove 22.9.0 exits 0 when Ravel refuses a launch. It prints Ravel's
  diagnostic and remedy and `status exit status: 3`, and the leaf stays live.
  The instrument checks those lines, and also checks exit 3 from a direct
  `ravel start-session`.
- The first full run passed. A Ravel-only upgrade at D leaves Grove 22.8.0,
  because Homebrew upgrades only bottled dependents. Grove 22.8.0 then
  refuses Ravel 2.0.0 and names the wrong remedy. A side control also showed
  that Ravel 2.0.0 refuses a top-level `"sources": []`, which Grove's caveats
  suggest. Neither finding is this leaf's packaging. The new leaf
  promotion-upgrade-remedy-docs-k338, before cross-repo-docs-reconciliation-k319,
  owns both corrections. promotion-controls-decision-k336 weighs them in the
  decision.
