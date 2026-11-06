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

## Claude Code's writes of the settings file

`hooks-installation-writes-k222` read these facts on 2026-11-06. The
[owner delivery design](specs/owner-skill-delivery.md#claude-codes-own-writes)
holds the decisions. The script
[`evaluations/hooks-installation-writes/settings-writes.sh`](evaluations/hooks-installation-writes/settings-writes.sh)
reproduces each reading except the read-only observation of the kept
processes. Each harness command ran with `HOME` and `CLAUDE_CONFIG_DIR` in a
scratch directory. No command signed in, and no measurement used the real
HOME.

| Subject | Value |
| --- | --- |
| Machine | macOS 27.0.1, Apple M4 Max |
| Claude Code | 2.1.319, SHA-256 `1f53587e92dee8e089fa349188a4cda64fa34ace6e7b0ca2ee278ddaa3cc2ebd` |
| codebase-memory-mcp | 0.10.6, SHA-256 `6fca5159065547135c491dc79a8c0697fa5868afc62c51c69e6edf39d60ab98e` |
| WritingSkills | `main` at `9f7d516be7d9a06611ec0f7dca8bebe033724ed4`, the source of the hooks plugin |
| `settings-writes.sh` | SHA-256 `b4cd60b9620518838bfdcc43eaead35879ea2304ad3ca1e1aba2a1900b87b9e3` |
| `race.py` | SHA-256 `d6cc5bd90dee328b87cbc178a453fb53ea350a3f5f83574c1bae09fc6277fe09` |
| `dirwatch.py` | SHA-256 `a9aaca129613e80093f24e175013fd2766c0a7cd115780e4837772e7492451ad` |
| `mcpserve.py` | SHA-256 `fb23bb3ca43f01b707cf0e3f6b32e76780dcc394fa5e8f43261027efd3e667eb` |

The run of record took the digests of these subjects before and after. No
digest changed.

### The write method

The script ran `claude plugin marketplace add`, `claude plugin install` and
`claude plugin disable`, each from the state before it.

- Each command writes `settings.json` once. It writes
  `settings.json.tmp.<pid>.<hex>` in `~/.claude` and renames that file onto
  the path. The inode at the path changes.
- A hard link to the earlier inode keeps the earlier bytes.
- The new file keeps the mode. It has no ACL and no extended attribute
  except `com.apple.provenance`, although the earlier file had both.
- The command writes the top-level keys in its own order.
- A command rewrites the file also when each value is already present.
  Three cases showed it, and in each the inode changed and the content
  stayed the same:
  - `marketplace add` with `extraKnownMarketplaces` already present.
  - `marketplace add` with the marketplace already registered.
  - `plugin install` with the plugin's `enabledPlugins` cell already `true`.
<!-- tc: depart synonym-rotation because "deny-delete" names a macOS ACL permission, not an action that this text describes -->
- With a deny-delete ACL on the file, `plugin install` exits 1. The file
  keeps its inode and its content, so the command makes no fallback write.
- `plugin install --marketplace` renames a file onto the path twice.
- `plugin list --json` and `plugin marketplace list --json` leave the inode
  and the modification time unchanged.
- When `settings.json` is a link, the command writes its temporary file
  beside the link's target and renames it onto the target. The link stays.

### The interval between the read and the rename

`race.py` restores the state before a command and holds a hard link to the
live file. It starts the command, and after a delay it adds one key to the
file. It writes the key either by a rename of a new file or in place. Each
command and each method ran at 46 delays from 0 to 135 ms.

The lead of a run is the time of the command's rename, less the time of the
outside write. The table counts where the key ended, and it gives the leads
of the runs whose key was missing from the live file.

| Command | Method | Live file only | Held inode only | Both | Neither | Missing from the live file at leads | In the live file at leads from |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `marketplace add` | In place | 12 | 17 | 17 | 0 | 0.5 to 36.4 ms | 40.6 ms |
| `marketplace add` | Rename | 31 | 0 | 0 | 15 | 1.6 to 33.3 ms | 38.0 ms |
| `plugin install` | In place | 15 | 12 | 19 | 0 | 1.7 to 30.3 ms | 36.8 ms |
| `plugin install` | Rename | 34 | 0 | 0 | 12 | 1.2 to 30.8 ms | 36.7 ms |
| `plugin disable` | In place | 14 | 14 | 18 | 0 | 1.8 to 32.6 ms | 42.9 ms |
| `plugin disable` | Rename | 36 | 0 | 0 | 10 | 2.5 to 30.8 ms | 36.3 ms |

The table gives five facts:

- A command makes its write from a read 30 to 38 ms before its rename. For `marketplace add`, that read lies between 36.4 and 38.0 ms
  before the rename. For `plugin install`, it lies between 30.8 and 36.7 ms,
  and for `plugin disable` between 32.6 and 36.3 ms.
- A write in place inside that interval lands in the earlier inode. The held
  link keeps it, and the live file lacks it.
- A rename by another program inside that interval is lost. Neither file
  holds the key.
- A write in place before the read is in both files.
- A write after the rename stays in the live file.

Two exploration runs of the same race on the same day gave the same pattern.

The run of record ran on an idle machine. The table therefore gives no upper
bound for a command that the scheduler pauses between its read and its
rename.

### What `plugin disable` changes

The script digested each file under `~/.claude` except `settings.json` and
`backups`, before and after `plugin disable`. None of the 24 files changed,
and `installed_plugins.json` and `known_marketplaces.json` are among them. In
`settings.json`, only the plugin's `enabledPlugins` cell changed, from `true`
to `false`.

### Two routes that keep a command out of the real file

- **`--scope local`.** The user `settings.json` kept its inode. The command
  wrote `.claude/settings.local.json` in its working directory. The
  installation record holds `"scope": "local"` and a `projectPath`.
  `plugin list --json` reports the plugin enabled in that directory and
  disabled in another one.
- **A private configuration directory.** `CLAUDE_CONFIG_DIR` named a
  directory that holds a copy of `settings.json` and a link to the real
  `plugins` directory. The real `settings.json` did not change. The
  installation record names an `installPath` inside the private directory.

### The kept processes

- Grove's sources under `crates/*/src` and the Rust sources of Ravel's tag
  `v1.1.0` name no `.claude` path. The same pattern finds `.claude` paths in
  Grove's tests, and the same search finds `.config` paths in Ravel's
  sources.
- codebase-memory-mcp 0.10.6 ran in server mode for 20 seconds with a
  scratch HOME. It answered `initialize`, `tools/list` and one call of
  `list_projects`. The inode, the size, the modification time and the digest
  of the scratch `settings.json` did not change. Its help text names
  `install`, `uninstall`, `update` and `config` as the commands that change
  configuration.
- Claude Code's executable holds about 40 calls that write the user
  settings. They set keys such as `model`, `fastMode`,
  `permissions.defaultMode`, the voice settings, `pluginConfigs` and `env`.
  Those keys belong to settings actions such as `/model`, `/config` and plugin
  configuration. The reading does not show which call can run without such
  an action.
- A read-only observation of the real HOME: the modification time of
  `~/.claude/settings.json` was 2026-10-11 12:15. No program replaced or
  changed the file in the 26 days before this reading, through daily
  sessions with these MCP servers.

### Readings without a result

- **A named pipe at the path.** Each command refused it: "Failed to read raw
  settings from …/settings.json: Error: Not a regular file (device, FIFO, or
  socket)". The pipe could therefore not time the commands' reads, and the
  race took its place.
- **codebase-memory-mcp's installer.** A first scratch reading ran
  `codebase-memory-mcp install` with a scratch HOME. It wrote files only
  there. It printed "Stopping active CBM sessions and operations for install"
  and stopped every running codebase-memory-mcp server on the machine,
  including the server of the session that read these facts. Its hook
  registration failed on the scratch file, so its write method stays unread.
  The script runs only the server mode.
- **A running Claude Code session.** A scratch HOME has no sign-in, so no
  reading shows whether an open session writes `settings.json` while a
  command changes it. The owner design gives that question a staging seam.
