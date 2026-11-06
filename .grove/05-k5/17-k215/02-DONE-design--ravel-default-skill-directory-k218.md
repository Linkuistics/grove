# ravel-default-skill-directory-k218

## Goal

Design Ravel 2.0.0's default skill directory, so that `brew install` and
`brew upgrade` are the whole skill setup. Settle its effect on the owner
switch before the plan's rehearsal.

## Context

- The human wrote: "My goal is to have nothing more to do beyond homebrew
  install/upgrade". The human then chose "Ravel default in 2.0.0" over
  explicit settings. The running log of `homebrew-delivery-design-k216` holds
  both, with the readings behind them.
- Homebrew cannot write a link or a setting in HOME. Each skill formula
  therefore installs its sources at `share/ravel/skills/<source-id>`, and
  Homebrew links them into `$(brew --prefix)/share/ravel/skills/`. Ravel reads
  that directory when the settings name no sources.
- `docs/specs/codex-skill-provisioning.md`, "The keg layout", states the
  layout. It leaves the contract to Ravel's specification.
- `docs/specs/owner-skill-delivery.md`, "Open question: the default
  directory", states the conflict with the switch. Three rules read an absent
  `skills` key as delivery off: preparation, the inverse and the native probe.
- Ravel's specification is `docs/specs/skill-discovery.md` on Ravel's
  `skill-directory-delivery` line. Its protected-path rule and
  `crates/ravel/src/skills/mod.rs` refuse a source inside `~/.config/ravel`.
- `docs/homebrew-skill-delivery-evidence.md` holds the Homebrew and Ravel
  readings. Ravel 2.0.0 has no cut yet, and this grove owns its release.

## Done when

- Ravel's specification states the default skill directory. It answers each
  question below, and its own decision records change as their test demands.
- The open question of the owner design is settled in that design. A change to
  a recovery rule changes `docs/models/owner-switch.qnt` first, and
  `task model:owner-switch` stays green.
- Grove's delivery design and owner design cite the settled contract and
  restate none of it.
- A review chain is judged for this design and the amendments of
  `homebrew-delivery-design-k216` together. A chain that earns its place is cut
  with `leaf-insert` before `homebrew-delivery-plan-k217`.

## Notes

Questions for the design:

- **Finding the directory.** How Ravel finds the Homebrew prefix: from its own
  canonical executable, from a value that its formula records, or otherwise.
  The keg-only `ravel@2` and the Linux prefix must work. A Ravel installed
  outside Homebrew needs a stated behaviour.
- **Roots.** How Ravel reads each source's roots without settings: a
  convention such as `skills/` and `craft/`, or a descriptor in each source.
  The third-party root `quint-llm-kit-plugin/skills` is a case.
- **Entries.** The source ID of an entry is its directory name. State what a
  malformed name, a dangling link, a file, a dot entry and an empty directory
  do.
- **Settings.** Whether a `skills` key replaces the default or adds to it. How
  a mandatory file names a default source, and how a user turns the default
  off.
- **Schema and routes.** Whether default delivery changes what schema 4 or
  "delivery disabled" means. Grove's "native route on 2.0" depends on the
  answer.
- **The owner switch.** How the native state turns delivery off after
  promotion, what the inverse writes, and what the native probe observes.
  Finish's own launch stays on Ravel 1.1, which refuses a 2.0 key.
- **Grove's checks.** `release:formula:test`, `release:smoke` and
  `directory_route.rs` resolve the keg sources from the default directory.

The default directory is a narrow slice of Linkuistics/Ravel#4. The universal
provider and the configuration of hooks stay there.

## Decisions (running log)

**Evidence (2026-11-06).** `brew.sh` of Homebrew 7.0.12 sets
`HOMEBREW_CELLAR` to `${HOMEBREW_REPOSITORY}/Cellar` when that directory
exists, and to `${HOMEBREW_PREFIX}/Cellar` otherwise. A keg's path therefore
does not always lie below the prefix. `install.rb` `install_formula` unlinks
the current keg before `formula_installer.install`, and `finish` links the new
keg after it. A source is therefore absent from the shared prefix for the
whole installation of an upgrade, not for a moment.

Ravel 2.0's archive
already installs `share/ravel/examples` and `share/ravel/notices` beside
`bin/ravel`. No SKILL.md lies outside `skills/` and `craft/` in any of the ten
sources. The quint skills name no file outside `quint-llm-kit-plugin`.

**Finding the directory (2026-11-06).** Ravel's formula writes an
installation record at `share/ravel/installation.json` in Ravel's keg, with
`skillDirectory` set to `HOMEBREW_PREFIX/share/ravel/skills`. Ravel reads the
record two levels above its canonical executable. A keg-only `ravel@2` and a
Linux prefix record the same directory. Without a record, an installation has
no default directory.

A derivation from the executable path was rejected,
because the Cellar can lie inside Homebrew's repository. A call of `brew`, an
environment variable and a compiled list of prefixes were rejected too.

**Settings (2026-11-06).** `skills.sources` becomes optional. Without it, the
default directory supplies the sources. With it, the array names exactly the
sources, and Ravel reads no default directory. `"sources": []` turns the
default off. `mandatory` becomes optional, and a reference names a source of
the effective set.

The design rejected explicit sources beside the default ones. A repeated ID
would need a refusal or an override rule. No planned consumer needs one
configured source beside the installed ones.

**Roots (2026-11-06).** A default source has the roots `skills` and `craft`,
each where it exists as a directory, in that order. A descriptor beside each
entry was rejected for now. The root `.` was rejected, because it widens the
agent's listing to every tree of a source. The quint formula installs
`quint-llm-kit-plugin` as the source directory, so its root is `skills`.

**Entries (2026-11-06).** An entry whose name has a leading dot is ignored.
An entry with a source-ID name that resolves to a directory with a root is a
source. The new code `skill_directory_invalid`, stage `skills`, exit 3,
refuses each other entry. That covers a malformed name, a file, a dangling or
cyclic link, another object and a directory with neither root. An absent default
directory supplies no source and refuses nothing.

**Schema and routes (2026-11-06).** Schema 4, the descriptor, the request map
and `skills resolve` keep their shapes. Delivery is disabled when the
effective source set and the mandatory list are both empty, or without an
absolute HOME. The routing notice's `skills` object gains `defaultDirectory`.
The default directory belongs to the installation, so a separate HOME does not
isolate it, and only `"sources": []` does.

**The owner switch (2026-11-06).** The earlier value of the `skills` cell
becomes the off value `{"schemaVersion": 1, "sources": []}`. Finish writes it
by a guarded step after promotion and before the hooks installation. The
switched value names only the mandatory file. The inverse restores the off
value by the generic cell rule, and the native probe still expects
`disabled`.

Step 6 stops unless the resolved source IDs are the ten accepted IDs. The
model's cells are abstract and it holds no settings file, so
`owner-switch.qnt` needs no change. `task model:owner-switch` passed with the
model unchanged: every invariant held, every witness was reached and every
mutant reached its violation.

**Grove's checks (2026-11-06).** Each check copies the Ravel 2.0 build into a
private prefix and writes its installation record. The record names the keg's
`share/ravel/skills` or a fixture directory of links. The resolution runs in a
HOME without settings.

**Upgrade window (2026-11-06).** A launch during an upgrade's installation
resolves without the sources of the formula being upgraded, and nothing
refuses. A mandatory file in that source refuses. A Grove session then lacks
its spine and force-stops. The design accepts the window, and a required-ID
list was rejected.

**Review chain (2026-11-06).** The design earns one review chain. The owner
design runs its switch against the real HOME inside finish, and the Ravel
contract becomes a public 2.0 surface. `homebrew-delivery-design-k219`
(`review-design`) now sits before `homebrew-delivery-plan-k217`. It reviews
k216 and k218 together. No provider evidence was set, so it carries no
Creator line.

**Planning inputs (2026-11-06).** `homebrew-delivery-plan-k217` cuts these
from the settled design:

- Ravel code: optional `sources` and `mandatory`, the installation record, the
  listing of the default directory, `skill_directory_invalid` and the notice
  member `defaultDirectory`. The Ravel commit is `78d833d24364`.
- Ravel's formula channels write the record, and their installed tests read
  it. Ravel's book chapters on settings follow the specification.
- The quint-llm-kit formula installs `quint-llm-kit-plugin` as its source
  directory.
- Grove's checks copy the 2.0 build with a record, as the delivery
  specification's test seams state.
- Shipped guidance still says that the directory route needs `skills`:
  `plugins/grove/skills/configure-grove/SKILL.md` and the directory route of
  `docs/USAGE.md`. They change with the implementation of the default.
- The owner's drafts hold the off value, the switched value and the staging
  settings, and the rehearsal covers the native settings.
