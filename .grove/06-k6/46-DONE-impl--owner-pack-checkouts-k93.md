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
- skill-keg-link-layout-k292 distinguishes source commits from full
  package versions. skill-formula-publication-k298 published the four
  revised formulae and holds their packaging acceptance records. Consume
  them with the shared-directory seam of shared-directory-seam-k294 and
  the hook readings of hook-adapter-keg-seam-k297.

## Done when

- `~/.config/mise/config.toml` declares each skill formula in
  `[bootstrap.packages]`. No `[bootstrap.repos]` entry names a skill source.
- Homebrew installed the formulae of personal, superpowers, quint-llm-kit and
  TestAnyware at their accepted commits. Each of those sources gives its
  accepted `resourceDigest`.
- Claude Code registers the personal keg's marketplace file, and holds
  `guardrail` installed and disabled.
- `mac-check` compares full installed and tap package versions, including
  revisions, using Homebrew when mise omits the revision. Source commit and
  digest comparisons stay separate. Fixtures distinguish old revisions,
  version drift, configuration errors and invalid source layouts.
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

## Decisions (running log)

- The real `~/.config/mise/config.toml` now declares `personal`,
  `superpowers` and `quint-llm-kit` beside `grove` and `testanyware`.
  `mise bootstrap packages use -g --no-install` wrote them. They are
  grouped with the other Linkuistics formulae. No `[bootstrap.repos]`
  table exists in either mise file.
- `brew update` moved Homebrew from 7.0.12-221 to 7.0.13-45 (`95d4938386e5`).
  The seven files that the seam digests are byte-identical, and Git shows no
  change to them or to `upgrade.rb`. `task seam:shared-directory` with the
  tap subjects then passed all 10 cases, and its failure control failed as
  required. Homebrew 7.0.13 is therefore accepted for the advance.
- The advance ran `brew upgrade` of `personal`, `superpowers`,
  `quint-llm-kit` and `testanyware` with `HOMEBREW_NO_AUTO_UPDATE`,
  `HOMEBREW_NO_INSTALL_CLEANUP` and `HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK`.
  Each now links its accepted revision. The earlier kegs stay, because no
  cleanup ran. Each prefix entry links the keg alias, which reaches the
  private payload. The installed formula files equal the acceptance records
  of k298, at tap `12b53f8c449d`.
- `task owner:sources:verify` is new. It resolves the real prefix's default
  directory with Ravel 2.0 in a disposable HOME and compares each source with
  the plan draft. The four formulae matched. Its digest, version, extra-ID
  and origin controls each failed as required. Finish can run it for all ten
  sources after Grove's release.
- Claude Code 2.1.322 registered the `personal` marketplace from
  `/opt/homebrew/opt/personal` and installed `guardrail@personal`, which was
  then disabled. In HOME, the "project" scope of `plugin disable` writes the
  same `~/.claude/settings.json`. The plugin reads from the keg's `opt`
  path. Its cache copy equals the keg payload. The installation also
  refreshed the auto-updating `linkuistics` marketplace. Its clone stayed at
  `238c3a839ab4`, and no other installed plugin changed. That refresh is a
  fact for owner-marketplace-freeze-k253.
- `mac-check` gained the area "Skill formulae" in
  `~/.local/lib/machine-config/check.py`, with fixture tests. It reports
  configuration errors, earlier revisions, version drift and invalid source
  layouts as separate classes. It checks the layout only at the tap's
  current version. On the real machine, its one message is that Grove
  22.8.0 has no keg layout, which Grove's release inside finish removes.
- The machine contract is the Obsidian notes. "Maintenance and Recovery"
  gained "Skill formulae", which states the advance rule, the launch
  results and why no process check is needed. "Machine Configuration",
  "New Machine Setup" and the `mac-bootstrap` comment point to it. The
  manual "AI Dev Tooling" records the `guardrail` registration. The
  `machine-environment` skill is unchanged, because an edit would change
  the accepted `personal` digest.
- The owner design now states why an advance needs no process check. The
  mise checkouts changed a source in place, and Homebrew links only a
  complete keg. Its drift-report row names the new area.
- The earlier lines of the Obsidian notes keep their existing writing-rule
  findings. This leaf corrected only the findings in its own text.
- `bash scripts/check.sh` passed all 18 checks on its second run. The first
  run failed only four `grove-tui::witnessed` tests with "session readiness
  timed out" after 36 to 63 seconds under the full suite. With
  `GROVE_TEST_RAVEL` set as `check.sh` sets it, the 10 tests of that file
  passed alone in under 5 seconds. This leaf changed no Rust code.
