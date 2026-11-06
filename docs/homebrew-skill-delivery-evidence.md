# Homebrew skill delivery evidence

`homebrew-delivery-design-k216` read these facts on 2026-11-06. It then
amended the delivery designs. The
[delivery specification](specs/codex-skill-provisioning.md) and the
[owner delivery design](specs/owner-skill-delivery.md) hold the decisions. So
do the [external pack contract](specs/external-pack-contract.md) and the
decision record
[Homebrew installs skills and Ravel delivers them](adr/homebrew-installs-skills-and-ravel-delivers-them.md).
No reading changed the owner's HOME, Homebrew prefix or harness configuration.

## Subjects

| Subject | Value |
| --- | --- |
| Homebrew | 7.0.12-96-ga9d4352, at `$(brew --repository)` on this machine |
| Claude Code | 2.1.319 |
| Codex | codex-cli 0.166.0 |
| mise | 2026.11.1 |
| Ravel specification | `docs/specs/skill-discovery.md` of the Ravel workspace `skill-directory-delivery`, commit `74099516d459` |
| WritingSkills | `main` at `9f7d516be7d9a06611ec0f7dca8bebe033724ed4`, the source of the hooks plugin in the measurement |
| Measurement script | [`evaluations/homebrew-skill-delivery/keg-lifetime.sh`](evaluations/homebrew-skill-delivery/keg-lifetime.sh), SHA-256 `dac3eaa426f16161090ae0eabf85c5ab923c60b53f22b62a49a0f94ffeba42cc` |

## Homebrew's cleaner and link order

`Library/Homebrew/cleaner.rb` gives these facts:

- `clean_dir` runs on `bin`, `sbin` and `lib` only. It sets mode 0555 on each
  file that is executable or starts with `#!`, and mode 0444 on each other
  file.
- `prune` walks the whole prefix except `libexec` and each `skip_clean` path.
  It removes empty directories and links that do not resolve.
- `rewrite_shebangs` acts only for a formula with a Node or Perl shebang
  detector. A skill formula has neither.

Grove 22.8.0's keg agrees. `bin/grove` and `bin/grove-llm` have mode 0555. The
files of `libexec/ravel-policy` keep mode 0644.

`Library/Homebrew/formula_installer.rb` installs the formula, runs the cleaner
and then runs `finish`. Only `finish` calls `link`, and `link` calls
`Keg#optlink`. `optlink` in `keg.rb` removes the `opt` link and then creates
it. The `opt` link therefore names a complete, cleaned keg, or for a moment
nothing.

`mise bootstrap packages where` prints the `opt` path of an installed formula,
"a path ... that keeps pointing at the installed version across upgrades". The
owner's `~/.config/mise/config.toml` records that mise cannot evaluate a
formula without a `sha256`. `mise bootstrap packages status --json` reports
`requested_version` and `installed_version` for each formula.

## Ravel's capture

The Ravel specification states:

- "A source path can itself be a symlink. Ravel canonicalises the source root
  once, then applies the boundary to every captured entry."
- `originPath` is "the canonical absolute source directory at capture".
- The snapshot manifest holds "the relative paths, the entry types, the
  normalised executable bits, the symlink targets and the content digests of
  the regular files". No write bit and no owner enter it.
- Capture detects a change of entry, type, identity, size or content during
  its scan, and refuses as `skill_source_changed`. A missing source refuses as
  `skill_source_missing`.

## The hook plugins across a keg change

The script builds a keg-shaped tree in a scratch directory:
`Cellar/grove/1/libexec`, with the writing source at `skills/writing` and the
two marketplace files at `.claude-plugin/marketplace.json` and
`.agents/plugins/marketplace.json`. The link `opt/grove` names that keg. Each
harness registers the marketplace by the `opt` path in a scratch home. The
script then writes `Cellar/grove/2`, moves the link to it and removes
`Cellar/grove/1`, as `brew upgrade` and its cleanup do.

| Reading | Claude Code 2.1.319 | Codex 0.166.0 |
| --- | --- | --- |
| Stored marketplace path | The `opt` path, as given, in `known_marketplaces.json` and in `extraKnownMarketplaces` of `settings.json` | The canonical path `Cellar/grove/1/libexec` in `config.toml` |
| Installation | A copy in `plugins/cache/writing-hooks/technical-communication-hooks/unknown` | A copy in `plugins/cache/writing-hooks/technical-communication/local` |
| `settings.json` after `plugin install` | `enabledPlugins` holds `true` for the plugin. The command has no option that installs a plugin disabled | Not applicable |
| After the keg change | `claude plugin list` reads the plugin from the `opt` path. The cache copy keeps the earlier bytes | `codex plugin list` fails: "marketplace root does not contain a supported manifest". The cache copy keeps the earlier bytes |
| Repair | None needed | `codex plugin marketplace add` with the `opt` path stores the new canonical path. `codex plugin add` then copies the new bytes |

A second part of the script installs a probe plugin whose `SessionStart` hook
writes `CLAUDE_PLUGIN_ROOT` to a file. It runs `claude -p` without a
sign-in. The hook runs before the sign-in refusal, so no request reaches a
model. The hook received the `opt` path of the probe plugin. Claude Code
therefore runs a directory marketplace's plugin in place, through the path
that the owner registered.

The same session also showed that `codex plugin marketplace remove` leaves
the plugin enabled in `config.toml` and keeps its cache copy.

## Readings without a result

<!-- tc: depart synonym-rotation because SessionStart is the harness's event name, not a second verb for launch -->
- **A Codex session with a missing marketplace path.** A probe plugin with a
  Codex `SessionStart` hook wrote nothing, with the marketplace intact or
  missing. Codex sent unauthenticated requests, and the server refused each
  with status 401, so no account was used. The reading does not show whether a
  Codex session runs a cached plugin while its marketplace path is missing.
  The delivery specification gives that question a seam.
- **A remote listing that drops its plugins.** Claude Code refused a
  `file://` marketplace source with "Invalid marketplace source format". The
  design cannot simulate a GitHub listing in a scratch home. The delivery
  specification gives the question a seam on the published bookmark.

## The retired listing

A marketplace file with the name `linkuistics`, a description and an empty
`plugins` array passed `claude plugin validate` with one warning: "Marketplace
has no plugins defined". `claude plugin marketplace add` accepted it.

## Release cut

`release.toml` replaces one line of `CHANGELOG.md` at a cut and no other file.
No file in `plugins/grove` names a Grove version. A cut therefore leaves the
bytes of the `grove-core` source unchanged.

## A skill directory under the Ravel configuration

The human asked for `~/.config/ravel/skills/` as the installation directory.
Two readings decided against it.

**Ravel's protected paths.** `crates/ravel/src/skills/mod.rs` of the Ravel
workspace `skill-directory-delivery` protects `$HOME/.config/ravel`, the
settings file and the skill cache. It compares the canonical source path with
each protected path. A scratch HOME tested two sources with the release build
of that workspace. That build reports `ravel 1.1.0`, and its SHA-256 is
`af3d24502b493120284ed7ded79d1c13732540c144928558f09489b3829fde35`:

| Source at `~/.config/ravel/skills/<id>` | Result of `ravel skills resolve --json` |
| --- | --- |
| A link into a keg-shaped tree | Resolved. `originPath` named the versioned keg |
| A copied directory | Refused as `skill_access_unavailable`, exit 3: "overlaps the protected path" |

**Homebrew's sandbox.** `formula_installer.rb` runs `post_install` in a sandbox
that allows writes to the formula's logs, Xcode, the formula's keg and each
link directory of the prefix. The build step denies reads of HOME. No formula
can therefore write a link or a setting in HOME.

## The default directory

`Keg#resolve_any_conflicts` in `keg.rb` merges a directory that several kegs
link. A keg can link into a destination that is a directory link into another
keg. Homebrew then removes that link, makes a real directory and links the
other keg's entries into it. `share/ravel/skills` is not in `SHARE_PATHS`, so the
first keg links it as one directory link, and the second keg's link turns it
into a real directory.

## The contract of the default directory

`ravel-default-skill-directory-k218` read these facts on 2026-11-06, from
Homebrew 7.0.12-126-g86f1f0c. The section "Default skill directory" of Ravel's skill discovery specification and
Ravel's decision record "Installed skill sources are the default" hold the
decisions. The [owner delivery design](specs/owner-skill-delivery.md#the-native-settings-inside-finish)
holds their effect on the switch.

**The Cellar.** `Library/Homebrew/brew.sh` sets `HOMEBREW_CELLAR` to
`${HOMEBREW_REPOSITORY}/Cellar` when that directory exists. Otherwise it sets
`${HOMEBREW_PREFIX}/Cellar`. A keg therefore does not always lie below the
prefix. Ravel cannot derive the prefix from the canonical path of its own
executable, so its formula records the directory.

**The upgrade order.** `install_formula` in `Library/Homebrew/install.rb`
unlinks each current keg of the formula. It then calls
`formula_installer.install` and `formula_installer.finish`, and only `finish`
links the new keg. The entries of an upgraded formula are therefore absent
from the shared prefix for the whole installation of the new keg.

**Ravel's installed layout.** The release archive of Ravel 2.0 installs
`share/ravel/examples` and `share/ravel/notices` beside `bin/ravel`, as
`scripts/release-common.sh` of the Ravel workspace `skill-directory-delivery`
lists them. The installation record joins that `share/ravel` directory.

**The source layouts.** No source of the owner's ten holds a `SKILL.md`
outside its `skills` and `craft` directories. The six Grove sources and
`personal` are the source directories of their packs. `testanyware` is
`plugins/testanyware` of TestAnyware. `superpowers` 7.0.0 holds its skills in
`skills/`.

The skills of quint-llm-kit name no file outside
`quint-llm-kit-plugin`. Its one parent-relative reference,
`../../../quint-lang/guidelines/choreo.md`, stays inside `skills/`.
