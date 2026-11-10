# owner-pack-checkouts-k93

## Goal

Install the owner's skill sources in the real HOME through Homebrew, at their
accepted commits. Cover each formula that publishes before finish. Make
`mac-check` report drift. Discovery stays native.

## Context

- owner-settings-drafts-k246 drafted the five declarations in
  `docs/owner-cutover/bootstrap-packages.toml`. The real
  `~/.config/mise/config.toml` already declares `grove` and `testanyware`,
  so add the other three to its `[bootstrap.packages]` table.
- skill-marketplace-decision-k201 replaced the mise checkouts with Homebrew
  installation. mise only declares the formulae, in `[bootstrap.packages]`.
  The root brief's "Skill installation" states the decisions.
- homebrew-delivery-design-k216 amends `docs/specs/owner-skill-delivery.md`.
  That document holds the contract and the real-HOME preparation of cutover
  order 6. The amended design gives this leaf its commands and its status
  report.
- Grove's formula carries core and the five domain packs. The personal pack,
  superpowers and quint-llm-kit have formulae of their own. TestAnyware's
  formula carries `using-testanyware`.
- owner-staging-acceptance-k252 records the accepted commits and the
  accepted `resourceDigest` of each source. Read them from the staging
  evidence.
- owner-staging-setup-k248 installed `personal`, `superpowers` and
  `quint-llm-kit` for the staging trials. This leaf declares them and
  verifies them at the accepted commits.
- skill-keg-link-layout-k274 amends "The keg layout" of
  `docs/specs/codex-skill-provisioning.md`. Consume its formula implementation
  and publication work before this leaf: private payloads, relative directory
  symlinks and revised formula versions. The two-keg seam and migration
  limits are in `docs/homebrew-skill-delivery-evidence.md`.

## Done when

- `~/.config/mise/config.toml` declares each skill formula in
  `[bootstrap.packages]`. No `[bootstrap.repos]` entry names a skill source.
- Homebrew installed the formulae of personal, superpowers, quint-llm-kit and
  TestAnyware at their accepted commits. Each of those sources gives its
  accepted `resourceDigest`.
- Claude Code registers the personal keg's marketplace file, and holds
  `guardrail` installed and disabled.
- `mac-check` reports each source's installed commit against its accepted
  commit. It tells drift from a configuration error, and a fixture control
  shows each outcome.
- Each non-Grove formula has the revised source layout. Resolution through
  the shared default directory gives its accepted IDs and digests, and each
  entry reaches the owning keg's private payload. `mac-check` distinguishes
  an invalid expanded layout from version drift and names its recovery.
- The machine contract and the `mac-bootstrap` guidance state that each
  Homebrew installation or upgrade of a skill formula is an advance. The
  amended owner design states why an advance needs no process check.
- Native discovery is unchanged. The real HOME's Ravel settings name no skill
  source, and no harness root holds a new entry.

## Notes

Grove's keg holds core and the five packs only after Grove's release, which
happens inside finish. The amended owner design states what the switch checks
in that keg after the release.

An advance links a complete new keg, so a capture reads one keg or refuses.
`homebrew-delivery-design-k216` therefore withdrew the process check before an
advance.

No advance may remove a running driver's binaries. Keep the original Ravel
1.1.0 keg and its cleanup protection. This leaf does not execute the switch.
Discovery stays native, and the simplified switch design changes nothing
else here.

Do not rely on the earlier TestAnyware relink as the permanent remedy.
Complete all four non-Grove formula upgrades before resuming default-source
launches. A remaining old real-directory source can still expand during
linking. No formula post-install hook should relink another formula, and no
step weakens Ravel's source boundary. A new canonical source path also needs
the affected registrations and prepared-input comparisons reconciled.

owner-ravel-skill-inventory-k38 put the policy conversion, the drafts and
the staging trials before this leaf. owner-marketplace-freeze-k253,
owner-switch-plan-k254 and owner-switch-rehearsal-k255 follow it, also in
order 6.
