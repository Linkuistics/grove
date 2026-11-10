# skill-keg-link-layout-k290


## Goal

Plan the smallest working increments that implement and publish the revised
skill-source layout before owner-pack-checkouts-k93 and the final switch.
Keep formula installation and upgrades as the whole skill setup in either
installation order.



## Context

- skill-keg-link-layout-k274 amends "The keg layout" in
  `docs/specs/codex-skill-provisioning.md`. Read its focused design review
  and any integration before planning.
- `docs/homebrew-skill-delivery-evidence.md`, "Directory-link layout
  experiment", records the actual Homebrew/Ravel seam, its rerun task and
  its limits. The source is `docs/design/skill-keg-link-layout/reproduce.rb`.
- Grove's formula template is `scripts/templates/grove.rb.tmpl`. The tap
  owns personal, superpowers, quint-llm-kit and testanyware. TestAnyware's
  release process must preserve the formula fix on later releases.
- `release:formula:test` and `tap:formula:test` exercise actual formula
  installation in owned prefixes. Their existing source-origin assertions
  and archive-to-keg comparisons assume real directories under share.
  `grove-tap-states.sh` constructs links itself and cannot establish actual
  Homebrew conflict behaviour. Reuse the real Keg seam for that obligation.

## Done when

- Concrete leaves before k93 own all five formulae, publication by new
  revisions or releases, and each formula producer's future releases.
  The four non-Grove formulae can publish before finish; Grove's revised
  template reaches the installed keg through finish's existing release.
- The plan covers actual formula output followed by actual Keg linking:
  both fresh installation orders, an initially absent share/ravel, duplicate
  source refusal, mixed-layout migration, upgrade, uninstall and removal of
  the old keg. Preserve source IDs, bytes, modes, links and accepted digests.
- The plan reconciles archive versus installed paths, formula tests,
  candidate-keg construction, origin assertions, inventories, hook
  registrations and prepared input comparisons with private payloads.
  It names the applicable Linux packaging checks as well as macOS evidence.
- k93 consumes the four revised installed formulae and the source-layout
  report. The finish handoff receives final Grove installation and shared
  default-directory acceptance with the published Ravel. A direct versioned
  path is not that acceptance.

## Notes

This design changes packaging, not source content or Ravel's link policy.
Use the specified `libexec/skill-sources` payload path; the installed-smoke
seam deliberately rejects the retired `libexec/ravel` and `libexec/skills`
layouts. Preserve those checks.
No automatic relink of another formula belongs in post-install hooks.
Keep original running-driver binaries. Do not replace the coordinated
release or finish sequences, and do not bump an already partially cut release.
