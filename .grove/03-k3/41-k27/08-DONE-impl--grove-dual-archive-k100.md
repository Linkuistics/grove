# grove-dual-archive-k100

## Goal

Ship both caller-asset sets in Grove's archives and formula. Prove the
installed layout against each explicit Ravel build.

## Context

- The "Packaging and rollout" section and the "Installed distribution" and "No
  native mutation" rows of the delivery specification.
- `scripts/release-build.sh`, `scripts/templates/grove.rb.tmpl`,
  `scripts/grove-installed-smoke.sh` and the asset gate of `docs/RELEASING.md`.
- Tap states A to D of `docs/specs/coordinated-release.md`.

## Done when

- The archive layout and its manifest carry both asset sets under
  `libexec/ravel-policy`. They carry no skill payload, private worker or skill
  resolver.
- The formula keeps its unversioned Ravel dependency. Its caveats show both
  configuration formats, both setup routes and the `ravel@2` prefix until
  promotion.
- The formula test and the installed smoke check the TypeScript set against the
  1.1 build. They check the Rhai set against the 2.0 build through an explicit
  prefix.
- An installed upgrade from a 22.8 layout changes no old or foreign native
  entry, link, mode, pointer or byte. An active old reader keeps working.
- `release:test` and the release preparation checks pass. The asset gate of
  `docs/RELEASING.md` names both sets.

## Notes

No tap mutation and no publication here. The fresh-install and upgrade
rehearsal of states A to D needs the `ravel@2` formula, so rollout owns it.
Record which state each check in this leaf models.

Controls: an archive without the Rhai set must fail the manifest check. A
caveat that names only `policy.ts` must fail the formula test. Watch each fail
once.

## Decisions (running log)

**Builds.** On 2026-11-03 this session installed the 2.0 build from
`Ravel.skill-directory-delivery` at output commit `4f831feb4d0f`, with an empty
working copy, by `task install PREFIX=<scratch>`. Its `bin/ravel` SHA-256 was
`151635d25169e6a76c7eed0a1de64af661b3d77554c9bbef88ab85a59afb823d`. The 1.1 keg
`bin/ravel` SHA-256 was
`c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. Both match
the node brief. The bookmark has not moved.

**Archive.** Earlier leaves put the Rhai set in `archive_manifest`, and the
build copies the whole asset tree. This leaf adds the named controls only.

**Installed smoke interface.** `grove-installed-smoke.sh GROVE_PREFIX VERSION
RAVEL_1_1_PREFIX [RAVEL_2_0_PREFIX]`. The position names the runtime, because
the pre-cut 2.0 build reports `ravel 1.1.0`. `-` in the third position skips
the 1.1 lane, for a host whose only Ravel is 2.x. The floor callers keep the
three-argument form, and k101 owns their 2.0 lane. Each lane runs in its own
HOME. The 1.1 prefix must hold the private worker, and the 2.0 prefix must not.

**Formula seam.** The formula's `ravel_builds` lists each installed Ravel and
its runtime: the dependency, by its version output, and
`linkuistics/taps/ravel@2` when the tap has it and it is installed. The real
`brew test` therefore checks the Rhai set in states C and D. The release
formula test overrides `ravel_builds` with the two explicit builds. It also
checks the default list on this host, which is state A, where `ravel@2` raises
`TapFormulaUnavailableError` and the list is the 1.1 dependency only.

**Installed smoke and formula results.** On the native archive of the working
copy, both lanes passed with both fronts, and the formula test passed with the
two explicit builds. Controls, each seen to fail once: swapped prefixes fail
the worker layout check. A renamed `with_provenance` in the installed
`provenance.rhai` stops the 2.0 lane with Ravel's refusal, while the 1.1 lane
passes. A template whose caveats drop the `config.rhai` copy fails the formula
test by name. The template digest was `a8bebe9f…` before and after.

**Manifest control.** The built archive, repacked without its seven Rhai
files, fails `assert_archive` with one `missing` line for each. The unchanged
archive passes. `release.test.sh` now names both sets, keeps that control and
refuses a skill payload, a skill resolver and a second formula dependency.

**Upgrade check.** `scripts/grove-installed-upgrade.sh`, run as `task
release:upgrade`, copies the installed 22.8.0 keg into a scratch Cellar. The
real 22.8 startup wrote the legacy layout into a scratch HOME, with a snapshot,
`current` and the `~/.agents/skills` links. The check adds foreign entries,
aliases, a killed startup's leftover and distinct modes. A 22.8 session stays
open and reads a skill through a legacy link. The check then moves the opt and
bin links to the new keg, as Homebrew does. The new Grove runs one lifecycle, a
start with no tree and a refused launch. A refused launch exits 0 after Grove
stops the loop, so the check reads Ravel's refusal and the stopped loop. All
286 entries of the HOME, the old keg and the foreign tree stayed the same, and
the 22.8 session read the same bytes and completed. Controls, each seen to
fail: a new front that changes a mode, repoints `current`, writes through a
foreign link or writes into the old keg each gives one diff line.

**Release task.** The installed verification after publication takes its lanes
from the dependency's version. A 1.x dependency adds an installed `ravel@2`
lane, and a 2.x dependency runs the 2.0 lane alone. Task's shell gave the
expected arguments for all four combinations in a scratch Taskfile.

**Tap states modelled.** The formula test and the installed smoke model state
A, with the unversioned 1.1 dependency, and the opt-in 2.0 prefix of states B
and C. The upgrade check models a Grove-only upgrade from A to C. No check here
uses a published `ravel@2` formula or promotion, so states B to D with real
formulae stay with rollout.

**Gate.** `mise exec bun@1.4.3 -- bash scripts/check.sh` with
`GROVE_TEST_RAVEL` set to the 1.1 keg passed all 20 principal checks, and the
diff digest was the same before and after. A first run with Homebrew's Bun
1.4.4 failed the pinned typecheck, as earlier leaves recorded. It also caught
`GROVE_PREFIX` in the usage comment of the installed smoke as an unclassified
`GROVE_*` name. The comment now uses lowercase placeholders.

**Handed forward.** k101 owns the 2.0 lane of the target and floor smoke. The
three-argument calls in `release-smoke-target.sh` and
`grove-floor-native-cases.sh` still run the 1.1 lane only. The pre-existing
semicolons in the published v22.8.0 changelog section are not this leaf's
change.
