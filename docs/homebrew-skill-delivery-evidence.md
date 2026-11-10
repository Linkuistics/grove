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

## The hook adapters from a keg

`grove-hook-adapters-k230` read each harness's registration from a Grove keg
on 2026-11-06.
[`evaluations/hook-adapters/keg-registration.sh`](evaluations/hook-adapters/keg-registration.sh)
ran it, and
[`keg-registration.out`](evaluations/hook-adapters/keg-registration.out)
holds its output, with the scratch directory shown as `S`.

| Subject | Value |
| --- | --- |
| Archive | `grove-v22.8.0-aarch64-apple-darwin.tar.xz` of the working copy, SHA-256 `5bc4c972f3f4…`. It holds both marketplace files at its root and the writing source at pack commit `4cafb0fd8e72` |
| Homebrew | A `--shared` clone of the owner's repository at `86f1f0c2fbb2`, with its own prefix in the scratch directory |
| Formula | A disposable `grove` in a scratch tap. Its `install` block is that of `scripts/templates/grove.rb.tmpl`. Versions 1 and 2 add the line "Probe keg: one" or "Probe keg: two" to `always-on/instruction.md` |
| Harnesses | Claude Code 2.1.319, Codex 0.166.0 and Pi 0.84.2, each in its own scratch HOME |
| Script | SHA-256 `d1cf20208320…`, with the same digests of the script, the archive, the template and both marketplace files before and after the run. The template's caveats changed after the run. Its `install` block, the only part that the script reads, is unchanged |
| Output | SHA-256 `3ebe90f02572…` |

Each harness ran the documented commands verbatim. The private Homebrew came
first on `PATH`, so `$(brew --prefix)` named the scratch prefix. Claude Code
ran without a sign-in, and its first two hooks run before the refusal. Pi ran
without a provider. Codex linked only the owner's `auth.json`, as in the
[common loading acceptance](common-loading-acceptance-record.md), and each
session sent one short prompt.

| Reading | Claude Code | Codex | Pi |
| --- | --- | --- | --- |
| Registration | The marketplace stores the `opt` path. `plugin details`: skills 0, hooks 6 | The marketplace stores `Cellar/grove/1`. The plugin is a copy in the Codex cache | The link names `share/ravel/skills/writing/...` in the prefix and resolves into `Cellar/grove/1` |
| Session at version 1 | SessionStart gave the instruction with "Probe keg: one". UserPromptSubmit gave the reminder. The init event names the plugin at the `opt` path and no `technical-communication` skill | No hook ran. The session file holds no instruction, and the reply log holds nothing | The extension wrote a start record with the source `startup` |
| Hook review | Not applicable | The interactive start showed "Hooks need review: 6 hooks are new or changed". "Trust all and continue" wrote six `trusted_hash` values and one `trusted_content_hash` | Not applicable |
| Session after the review | Not applicable | The hooks ran. The session file holds "Probe keg: one", and the reply log holds a start and a reply | Not applicable |

`brew upgrade` installed version 2, and its cleanup removed `Cellar/grove/1`.
`brew cleanup` then left only `Cellar/grove/2`, and `opt/grove` named it.

| Reading after the upgrade | Claude Code | Codex | Pi |
| --- | --- | --- | --- |
| Listing | `plugin list` reads the plugin from the `opt` path | `plugin list` fails: "marketplace root does not contain a supported manifest" | The link resolves into `Cellar/grove/2` |
| Session before any repair | SessionStart gave "Probe keg: two", from the new keg | The hooks ran from the cache copy. The session file holds "Probe keg: one" | The extension wrote a second start record |
| Repair | None needed | `marketplace add` and `plugin add` copied version 2 into the cache | None needed |
| Session after the repair | Not applicable | No hook ran | Not applicable |
| Second review | Not applicable | The interactive start showed the same review for six hooks. "Trust all and continue" accepted them | Not applicable |
| Session after the second review | Not applicable | The hooks ran. The session file holds "Probe keg: two" | Not applicable |

These readings agree with the source of Codex at tag `rust-v0.166.0`.
`discover_handlers` in `codex-rs/hooks/src/engine/discovery.rs` adds a plugin
hook only when its `hooks.state` entry holds a `trusted_hash` that matches,
or when the session has `--dangerously-bypass-hook-trust`. The trust covers
the hook declaration and a fingerprint of the plugin content. A copy with
other bytes therefore makes each hook "modified". `codex exec` has no review
and reports no skipped hook. `startup_hooks_review.rs` in `codex-rs/tui`
gives the review at an interactive start, and its third choice says "hooks
won't run".

Two runs before the accepted run failed in the setup, not in a harness:

- The first run waited for the composer of the TUI. Codex draws the composer
  first and replaces it with the review about two seconds later, so the run
  exited before the review. The script now waits for the review itself.
- The second run reached the first review. A newer Codex release then made
  the next interactive start ask to update first, and that prompt's first
  choice runs `brew upgrade --cask codex`. The script now writes
  `check_for_update_on_startup = false` into the scratch configuration.

## Readings without a result

<!-- tc: depart synonym-rotation because SessionStart is the harness's event name, not a second verb for launch -->
- **A Codex session with a missing marketplace path.** A probe plugin with a
  Codex `SessionStart` hook wrote nothing, with the marketplace intact or
  missing. Codex sent unauthenticated requests, and the server refused each
  with status 401, so no account was used. The reading does not show whether a
  Codex session runs a cached plugin while its marketplace path is missing.
  [The hook adapters from a keg](#the-hook-adapters-from-a-keg) answers the
  question. Nobody had trusted the probe's hook, and Codex runs no untrusted hook.
- **A remote listing that drops its plugins.** Claude Code refused a
  `file://` marketplace source with "Invalid marketplace source format". The
  design cannot simulate a GitHub listing in a scratch home. The delivery
  specification gives the question a seam on the published bookmark.
  [A refresh of the retired listing](#a-refresh-of-the-retired-listing)
  answers the question through a Git `insteadOf` rule.

## The retired listing

A marketplace file with the name `linkuistics`, a description and an empty
`plugins` array passed `claude plugin validate` with one warning: "Marketplace
has no plugins defined". `claude plugin marketplace add` accepted it.

`native-marketplace-retirement-k235` wrote Grove's retired file. `claude
plugin validate` accepted it with the same single warning.

### A refresh of the retired listing

`native-marketplace-retirement-k235` measured the seam "Retired listing" on
2026-11-07.
[`evaluations/retired-listing/refresh.sh`](evaluations/retired-listing/refresh.sh)
ran it, and
[`refresh.out`](evaluations/retired-listing/refresh.out) holds its output,
with the scratch directory shown as `S`.

| Subject | Value |
| --- | --- |
| Claude Code | 2.1.319, in two scratch homes under `env -i`, without a sign-in |
| Listed commit | Grove `3cf8dfc702fb`. Its marketplace lists ten plugins, `grove` among them |
| Retired file | `.claude-plugin/marketplace.json` of this leaf, SHA-256 `a2d45eb52a61…` |
| Remote | A local bare repository. Its first commit has the tree of the listed commit. Its second commit changes only the marketplace file, to the retired file |
| Route to the remote | Each home's `.gitconfig` sends `https://github.com/Linkuistics/grove.git` and `git@github.com:Linkuistics/grove.git` to the bare repository. `claude plugin marketplace add Linkuistics/grove` cloned it, and the registration records the source `github` with the repository `Linkuistics/grove` |
| Script | SHA-256 `1a8e30ceb459…`, the same before and after the run. The retired file has the same digest before and after the run |
| Output | SHA-256 `2442c7fb557f…` |

Both homes added the marketplace at the first commit and installed
`grove@linkuistics`. Each session then listed the plugin and six `grove:`
skills in its init event. The bare repository then moved to the second
commit.

| Reading | Result |
| --- | --- |
| `claude plugin marketplace update linkuistics` | Exit 0, "Successfully updated marketplace". The clone is at the second commit. A second refresh also exits 0 |
| Installation records after the refresh | `installed_plugins.json` and `enabledPlugins` still hold `grove@linkuistics`. The plugin cache keeps its version directory |
| `claude plugin list` | `grove@linkuistics`, "✘ failed to load", with "Plugin grove not found in marketplace linkuistics". The command exits 0 |
| `claude plugin details grove@linkuistics` | Exit 1, "Plugin "grove@linkuistics" not found" |
| Session after the refresh | The init event lists no plugin of the marketplace and no `grove:` skill |
| `claude plugin update grove@linkuistics` | Exit 1, "Plugin "grove" not found". The records stay as they were |
| Interactive start after the refresh | The screen after 30 seconds shows no notice of the plugin |
| `claude plugin marketplace remove linkuistics` | Exit 0. It also uninstalled `grove@linkuistics`. `installed_plugins.json` and `enabledPlugins` are then empty. The plugin cache keeps its version directory |

A refresh therefore keeps the plugin installed and enabled, but the plugin no
longer loads. A session loses its skills, and only `claude plugin list` shows
the failure. The release notes tell a user to complete the first three steps
before the marketplace refreshes.

The second home, "auto", had `autoUpdate: true` for the marketplace. Claude
Code describes that field as "Whether to automatically update this
marketplace and its installed plugins on startup".

In this home, the clone stayed at the first commit after two headless
sessions. It also stayed there after an interactive start of 30 seconds, with
`lastUpdated` a week back. That start reached the prompt and added the
official marketplace. The plugin stayed enabled and loaded. The measurement
therefore shows no automatic refresh without a sign-in. It does not show what
a signed-in start does.

A read of the owner's own registration shows that a
signed-in start refreshes it. The section "For rollout" of
[The retired marketplace](repository-moves-record.md#the-retired-marketplace)
gives that reading.

Two parts of the run read no Grove behaviour. The interactive start adds
`claude-plugins-official` to each home, so `known_marketplaces.json` lists it
afterwards. An earlier version of the script ran the same readings without
the interactive starts, with the same results.

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

## Directory-link layout experiment

skill-keg-link-layout-k274 repaired the delivery spec's description of
Homebrew conflict repair. The
[layout contract](specs/codex-skill-provisioning.md#the-keg-layout) owns the
remedy, its formula owners and the migration. The
[conflict-repair view](http://127.0.0.1:52710/#diagram-keg-link-layout) shows
the failing and designed paths.

### Instrument and subjects

The [experiment source](design/skill-keg-link-layout/reproduce.rb) runs with
`brew ruby`. It calls Homebrew's actual `Keg#link` and `Keg#unlink`. It
substitutes only the prefix, Cellar, linked-keg record and lock directories.
Each case owns a new temporary prefix with two kegs and a separate HOME.
It clones the accepted Ravel build and gives that clone an installation
record pointing to the fixture's shared default skill directory. No real
Homebrew link or Ravel setting changes.

Each source has a skill, a distinct resource and a relative link that stays
inside the source. Reference digests come from resolution of the intact
payloads. The observations after linking resolve the shared prefix, not
direct keg paths. The proposed cases also inspect the directory entries,
their canonical payload paths and a regular skill file.

The successful measurement on 2026-11-10 used these subjects. The experiment
digests all three before and after its cases and refuses a changed subject.

| Subject | SHA-256 |
| --- | --- |
| Accepted Ravel executable, staged `ravel-2.0/bin/ravel` | `3a9a9129573e4d827a317f6b793ba990044080a31a7b667b64cc153a2f974a0b` |
| Installed Homebrew linker, `Library/Homebrew/keg.rb` | `0faa3f4cbb42c6ae7aa344ca04b129573ed9dc783d72b549bed02497c881552d` |
| Experiment | `abef8eed147cf1138cc93f8d9bac88dc36fe1e022bcc910985a113d28ba1bdd2` |

Homebrew reported `7.0.12-221-gbc63375`. Both `link_dir` and
`resolve_any_conflicts` matched the method bodies at
[`3ccfa1d07143`](https://github.com/Homebrew/brew/blob/3ccfa1d07143/Library/Homebrew/keg.rb)
byte for byte. The source's symlink branch precedes its directory branch,
and conflict repair uses `link_dir` with `:mkpath` for the displaced subtree.

### Observations

| Fixture operation | alpha then beta | beta then alpha |
| --- | --- | --- |
| First direct-directory source linked alone | Resolves with its reference digest | Resolves with its reference digest |
| Second direct-directory keg linked | First source expands into file links. `skill_resource_invalid` | First source expands into file links. `skill_resource_invalid` |
| Unlink and relink the displaced old keg | Both reference digests restored | Both reference digests restored |
| Both sources projected from private payloads | Both entries remain symlinks. Both reference digests retained | Both entries remain symlinks. Both reference digests retained |
| Upgrade first projected keg, then remove its old keg | Default entry reaches the replacement. Both digests retained | Default entry reaches the replacement. Both digests retained |
| Unlink the other projected keg | Remaining source retains its digest | Remaining source retains its digest |

The task exited 0 with all assertions passed and unchanged subjects. The
direct-directory cases are the failure control for the same linker,
fixtures and resolver used by the proposed layout. An initial exploratory
run reproduced the defect but could not remove Ravel's read-only scratch
snapshots. The retained instrument restores permissions on its own real
directories before cleanup. The successful run covered every case.

Rerun from the repository root, using an executable with directory delivery:

```sh
mise exec -- task design:keg-link-layout:test \
  RAVEL=/absolute/path/to/ravel-2.0/bin/ravel
```

### Interpretation and limits

The packaging owner can hide its payload location behind the existing shared
entry. This removes cross-formula repair from installation and preserves
Ravel's existing source boundary. The experiment verifies the actual filesystem
operation. A model of the same operation would not establish Homebrew's
behaviour.

This is a design experiment on macOS, not acceptance of published formulae.
It does not run their install methods, Homebrew's cleaner, downloads or
bottle relocation. It simulates upgrade with actual unlink/link operations
and explicit removal of the old fixture keg. Duplicate IDs, mixed-layout
migration and Linux packaging remain implementation acceptance obligations.

Changed canonical paths also require affected hook registrations and prepared
input comparisons to be reconciled. Rerun the seam after changes to Homebrew's
linker. A linker that descends through source symlinks would reopen this remedy.

The existing viewer served the revised manifest and diagram sources at port
52710. The browser access host was unavailable. The rendered layout and its
deep-link positioning remain unverified. Source and manifest inspection alone
do not establish visual rendering.

## Directory-link layout review

`skill-keg-link-layout-k291` inspected producer commit
`7e3673f3c8340c70f3675572550b981faaa36b55`. The subjects are the amended
section "The keg layout", the matching rules of the owner design, the
experiment above with its source, and the handoffs to
`owner-pack-checkouts-k93`, `skill-keg-link-layout-k290` and finish. The
review read Homebrew's installed `keg.rb`, `cleaner.rb`,
`formula_installer.rb` and `install.rb` at `bc63375ac3a4`. It read Ravel's
skill discovery specification and formula template at `ff14cba826b0`.

The review ran no test, build or experiment. Each finding states a defect, a
state that shows it and the rule that the state breaks. No finding proposes a
repair. P2 marks a contract or acceptance gap that planning would otherwise
inherit. P3 marks a consistency defect that leaves the remedy intact. No
finding is P1, because the review found no counterexample to the remedy.

### Claims that hold

Each linking claim of the amendment holds in the installed linker.
`Keg#link_dir` tests `src.symlink? || src.file?` before `src.directory?`, and
`Pathname#find` does not descend into a symlink. A source alias is therefore
one entry under the `:link` and the `:mkpath` strategies.
`Keg#unlink` removes a prefix link only when its lexical target equals the keg
entry. It therefore removes the link to the alias and nothing in the payload.
`libexec` is not a link directory, and the prune step of the cleaner skips
`libexec`.

An upgrade of an expanded old source repairs it. The unlink removes the file
links and the empty directories, and the link of the new keg then adds the
alias. The direct-layout cases of the experiment are a failure control on the
same linker, fixtures and resolver. The claims of the experiment stay inside
what its cases ran.

### L1 — P2: a revision-only formula has no accepted-version identity

The amendment gives each affected formula a new revision "even if its source
commit is unchanged". The version model of the owner design does not admit
that change. Its terms define an accepted version as "a formula version whose
sources passed staging". Its section "Skill formulae" states that a formula
version names a commit. It also states that the tap publishes only accepted
versions. Switch step 2 verifies "accepted versions".

The program encodes the same model. The `accepted.versions` of the plan draft
name `2026-11-06`, `2.8.0`, `7.0.0` and `1.0.0`. `gate_5` in
`scripts/owner_switch.py` compares each with the `installed_version` from
`mise bootstrap packages status --json`. The leaf k93 instead makes
`mac-check` compare "each source's installed commit against its accepted
commit".

Consider the state after k93 upgrades `personal` to revision 1. If mise
reports `2026-11-06_1`, gate 5 stops the switch until a plan names a version
that no staging trial read. If mise reports `2026-11-06`, gate 5 cannot tell
the old revision from the new one. The commit comparison sees no change in
either case, because a revision keeps the commit.

The state breaks the rule that the tap publishes only accepted versions. It
leaves k290 and `owner-switch-plan-k254` with no rule for the identity of a
revised version. The sentence "Their source commits and accepted digests may
remain unchanged" implies acceptance by digest, but no rule states it.

### L2 — P2: the plugin paths of the hook adapters cross the alias, and no reading covers it

Both marketplace files of the keg name
`./share/ravel/skills/writing/craft/technical-communication` as the plugin
source. Under the amendment, `share/ravel/skills/writing` is the relative
directory symlink of the formula. Each harness therefore reaches the plugin
root through a symlinked path component. The section "Hook adapters" of the
delivery spec is unchanged. It states that Codex "resolves a `local` source
only inside its marketplace root".

The readings of "The hook adapters from a keg" and "The hook plugins across a
keg change" both used real directories. The first used the current `install`
block of the template. The second script made `skills/writing` with
`mkdir -p` and named that path in both marketplace files. No reading shows how
Codex applies its root rule to a path through a directory symlink. No reading
shows how Codex copies such a plugin to its cache, or how Claude Code runs it
in place.

The design does not decide whether the marketplace files keep the alias path
or name the private payload. The limits above ask only that "affected hook
registrations" be "reconciled", and k290 repeats that word. Neither asks the
seam "Hook adapters" to run again on the projected layout. A harness that
refuses the path loses the writing hooks. That breaks the root outcome "Hooks
and plugin capabilities remain usable where required".

### L3 — P2: the acceptance of the layout belongs to no durable seam

The amendment states what acceptance must observe. It names the shared
default directory, every ID and digest, the owning private payload and
regular payload files. It names no seam that runs it. The test-seam table of
the delivery spec is unchanged. Its row "Installed distribution" points an
installation record at the `share/ravel/skills` of one keg. The amendment
itself says that a reading of that kind "cannot detect the original failure".

`task design:keg-link-layout:test` is the only instrument that links two kegs
into one prefix. It needs a `RAVEL` executable, and `scripts/check.sh` does
not run it. The limits above say "Rerun the seam after changes to Homebrew's
linker", but nothing detects such a change. The installed Homebrew is
`7.0.12-221-gbc63375`. Four commits after the cited `3ccfa1d07143` changed
`keg.rb`. Their subjects name dry-run output only, and the experiment reports
the cited methods unchanged.

The Done when of k290 lists the cases for this cutover. After `.grove/` ends,
the table of the spec is the only durable acceptance contract. The guarantee
rests on internal behaviour of Homebrew, not on a documented interface. A
later linker change can therefore reopen the defect. The layout report of
`mac-check` then shows it only when the owner runs that report.

### L4 — P3: the formula of Ravel is an unlisted co-owner of `share/ravel`

The ownership paragraph names five formula owners of the projection. A sixth
formula links into the same directory. The formula template of Ravel installs
`share/ravel/examples` and `share/ravel/notices`, and it writes
`share/ravel/installation.json` in the keg. The unversioned `ravel` is a
linked formula, and the Grove formula depends on it.

Two linking events follow from that. When a new machine installs Grove as
its first skill formula after promotion, Homebrew links the dependency Ravel
2.0 first. The keg of Ravel then holds the whole `share/ravel` link. Inside
finish, promotion links Ravel 2.0 beside the ten sources of the owner. Under
the old layout, promotion on a machine whose only skill formula is Grove would
expand all six Grove sources. The facts of
the amendment do not name that trigger.

The projection keeps each source whole in both events, by the code path that
the experiment ran. The `examples` and `notices` of Ravel become file links,
and no source changes. The experiment and the cases of k290 name only skill
kegs. No planned case therefore links the keg that finish links beside the
live sources.

### L5 — P3: a duplicate source ID unlinks the whole later keg

The spec states that "Two formulae that ship one source ID conflict at that
entry". The effect is wider than the entry. `Keg#link` handles a link error by
unlinking the whole keg that it was linking. `FormulaInstaller#link` then
reports "The formula built, but is not symlinked". In an upgrade, Homebrew
unlinked the earlier keg before the installation. `install.rb` relinks the
earlier keg only when the new version is not installed.

Consider a Grove upgrade while another formula owns one of the six Grove IDs.
The new Grove keg links nothing, and the earlier keg stays unlinked. All six
Grove sources leave the default directory, `grove-core` included. Each later
Grove session then force-stops for missing guidance. The sentence of the spec
and the case "duplicate source refusal" of k290 both describe a refusal at one
entry.

### L6 — P3: Grove documents still describe the earlier keg layout

The amendment changes where each formula puts a source. These current
documents still state the earlier layout:

- The section "The pack record and the release" of the same spec says "The
  formula installs `share/ravel/skills/` and the two dot directories at the
  keg root". That contradicts the amended table.
- The glossary entry "Keg source" in `plugins/CONTEXT.md` puts a keg source at
  `share/ravel/skills/<source-id>/`. The term of the owner design now puts it
  in private storage behind a symlink. The two definitions of one term
  disagree.
- `docs/RELEASING.md` says that "the formula installs the same shape into its
  keg". Its keg listing names `share/ravel/skills/grove-core/` and each pack
  as directories.
- `docs/ARCHITECTURE.md`, `plugins/grove/README.md` and the item "Grove's
  formula" of the external pack contract put each source at
  `share/ravel/skills/<source-id>/` of the keg.
- The ownership paragraph names the `testanyware` formula of the tap. The
  release of TestAnyware renders that formula from
  `scripts/templates/testanyware.rb.tmpl` in its own repository. A change to
  the file of the tap lasts only until the next TestAnyware release.
- The facts bullet on the cleaner justifies the payload by the prune step
  "under `share/`". The prune step skips `libexec`, so that reason no longer
  reaches the payload. The conclusion still holds.

The reconciliation list of k290 names formula tests, origin assertions,
inventories and registrations. It names none of these documents. The section
"Scope and remaining work" of the delivery spec requires implementation to
update "installation guidance, architecture, glossary" with the behaviour.

### L7 — P3: the documentation of Ravel shows the failing layout

The packaging rule is now only in the Grove delivery spec, which binds "Each
Linkuistics skill formula". The skill discovery specification of Ravel owns
the default skill directory. It says that packages "put their sources" there.
Its view `docs/design/skill-discovery/homebrew-directory.mmd` shows "Install
share/ravel/skills/ID in the keg", which is the layout that fails. Ravel
2.0.0 is not yet cut, and this grove executes that release.

A package that follows the published documentation of Ravel installs a real
source directory. The next conflict repair then expands it. Ravel then
refuses each resolution of the whole default directory, so one such package
blocks the sources of every other package too. The amendment names no
document of Ravel as a consumer.

### L8 — P3: the amendment adds hard prose findings to three documents

The text of the producer breaks the writing rules of the project. Before the
producer commit, the checker reported no hard finding in
`codex-skill-provisioning.md` or in this evidence document. After it, the
checker reports 5 and 13, and `owner-skill-delivery.md` went from 1 to 5.
Most are semicolons, and the others are long sentences and word rotations.
This review marked the section "Directory-link layout experiment" as pending
for the integration to convert, and changed none of its words.

## Directory-link layout integration

`skill-keg-link-layout-k292` integrated the findings from review commit
`b32012233495017dfa1c36c906d530b42965ceea`. It retained private payloads with
relative directory symlinks. The review found no counterexample to that
remedy. This integration repairs its contracts and acceptance obligations.
It installs and publishes no formula.

### Source verification

The integration read the specifications, the experiment and the queued
handoffs. It read the installed `Keg#link` and `Keg#unlink`, including
whole-keg rollback after a link error. The graph supplied `gate_5` from
`scripts/owner_switch.py`, which compares mise's `installed_version` with
the plan's accepted string. Its call lookup reported no inbound call.
That result establishes no absence of use, since the gate can be dispatched
through a function table.

The graph's full index generation was `2026-11-10T10:37:04Z`. Coverage
metadata matched each requested path. It recorded partial parsing for the
Mermaid sources, which the integration read in full. Later coverage for the
external-pack views used generation `2026-11-10T10:47:29Z`. Clean metadata
establishes no completeness beyond that recorded coverage.

### Dispositions

| Finding of skill-keg-link-layout-k291 | Classification | Result and owner |
| --- | --- | --- |
| L1 | Contract stated unclearly | The owner spec distinguishes full package versions from commits. Packaging-only acceptance can retain source trials after equal commits, roots and digests, plus the applicable packaging and hook seams. k290 assigns the program and fixture changes. k93 and k254 consume full revisions. |
| L2 | Real issue | Both writing marketplaces must name the private payload directly. The Hook adapters seam repeats registration, copying, execution, cleanup and trust on revised formula output. k290 assigns that work. The earlier readings establish the earlier layout only. |
| L3 | Real issue | The delivery spec adds Shared default directory acceptance. Formula validation runs it before publication, and release acceptance repeats it with published Ravel archives. Actual formula output and Homebrew operations replace a direct keg path as evidence of linking. |
| L4 | Real issue | The contract includes Ravel's shared ancestors and entries. Acceptance covers Ravel-first dependency installation and promotion beside installed sources. k290 assigns the fixtures, and k12 carries actual promotion acceptance to finish. |
| L5 | Real issue | A duplicate ID can leave every entry of the later keg unlinked. The upgrade can also leave the earlier keg unlinked. The contract requires ownership resolution and an intended linked keg before sessions resume. Acceptance covers the whole rollback. |
| L6 | Real issue | Current Grove guidance distinguishes archive directories, private installed payloads and public aliases. The cleaner rationale now names its `libexec` exclusion. k290 assigns TestAnyware's release template and tap output together. |
| L7 | Real issue | The delivery contract requires Ravel's packaging guidance to show intact sources behind directory symlinks. k290 assigns Ravel's spec and Homebrew directory view before the 2.0.0 cut. This session changes no Ravel file. |
| L8 | Real issue | Preparatory change `e287b474` converts affected Grove prose before substantive edits. The experiment retains its measured subjects, observations and scope. |

The repaired [keg layout](specs/codex-skill-provisioning.md#the-keg-layout)
owns packaging. The [owner formula contract](specs/owner-skill-delivery.md#skill-formulae)
owns revision acceptance. The
[shared-directory seam](specs/codex-skill-provisioning.md#shared-default-directory-acceptance)
owns repeated linking acceptance. The affected diagrams keep their existing IDs
and now include Ravel's participation, package revisions and whole-keg
rollback.

### Consequences for queued work

`skill-keg-link-layout-k290` assigns the implementation, formula publication
and cross-repository documentation work before `owner-pack-checkouts-k93`.
The plan must include the durable seams, not only this cutover's trials.
The current formulae and validation programs still implement the earlier
layout. The revised contracts are no claim that those implementations pass.

Reconciliation changes `plugins/grove/README.md`, which is inside the
`grove-core` source. Its digest therefore changes even though the skill
procedures do not. k290 must assign a new candidate keg and the affected core
staging trials. `owner-switch-plan-k254`, its input comparison and the dry run
consume that renewed evidence. Unchanged sources can use the packaging-only
bridge, but the changed core cannot.

One narrow reviewer challenged the revision-acceptance contract. It found an
unclear boundary: `guardrail` lies outside the personal source digest. An
unchanged source therefore cannot establish unchanged native-hook behaviour.
The owner contract now requires complete native-plugin payload comparisons
and renewed capability trials for changes. The hook seam names guardrail's
trial and a no-op control that must fail its refusal assertion. k290 assigns
that acceptance.

This clarification changes no source-boundary rule.

Finish receives the installed ten-source acceptance after Ravel promotion,
full package identities, revised hook registrations and current prepared
inputs. It preserves the original Ravel 1.1.0 keg through the final signal.
No formula link or real-HOME setting changed in this integration.

### Reference sweep and validation

The path-reference sweep covered Markdown, Mermaid and JSON across the
repository, including dot directories and the task tree. It excluded generated
`target` output and VCS internals. The same search found a known current
payload, a deliberately wrong direct-directory fixture and its later removal.
The retained references have these meanings:

| Surface | Disposition |
| --- | --- |
| Current specifications, architecture, glossary, release guidance and source-layout views | Describe private installed payloads and public aliases. Archive paths remain directories. |
| README, usage, configuration guidance and the installation decision record | Name the public default directory. That path remains valid. |
| Current marketplace assets, harness case inputs and unreleased release note | Still describe the implementation that exists. k290 assigns their change when the new implementation lands. |
| Staging settings, measured evidence, move records and finished task files | Preserve their recorded subjects and results. They establish no acceptance of the revised formula output. |
| Live planning and implementation leaves | Assign revised formula output, package acceptance, core staging and the finish handoff. |

`mise exec -- task check` exited 0. All 18 principal suites passed, including
1,344 workspace tests, nine documentation tests and five final walkthrough
validations. The workspace run skipped 29 tests. The result covers the current
implementation. It establishes no acceptance of the future formula output.

Both diagram manifests parsed. Their diagram IDs equal those of the review
commit, and every referenced source exists. The viewer served the revised
manifest. The browser host was unavailable, so rendered layouts and deep-link
positions remain unverified.

## Grove keg projection

`grove-keg-projection-k293` changed Grove's formula template to the
[keg layout](specs/codex-skill-provisioning.md#the-keg-layout). The archive
layout did not change. The formula moves each archive source into
`libexec/skill-sources/<source-id>`. `Pathname#install_symlink` then makes
the alias `share/ravel/skills/<source-id>`, relative to the real destination,
as `../../../libexec/skill-sources/<source-id>`. Both keg marketplace files
name `./libexec/skill-sources/writing/craft/technical-communication`. This
work published no formula and changed no real keg, link or HOME.

<!-- tc: depart synonym-rotation because "validate" occurs only in the command name claude plugin validate -->
### Checks

Each check below ran on the working copy with the 1.1 lane and the 2.0 lane
of `docs/ravel-target-inputs.md`. Each passed.

| Check | Result |
| --- | --- |
| `task release:formula:test`, with the published 22.8.0 keg as `OLD` | Homebrew's install method made the projected keg. Each alias was a relative symlink to its private source. Each regular file kept its manifest mode. Each `originPath` was the private source, and each digest equalled its plain checkout. `claude plugin validate` accepted the keg. Tap states A to D passed, fresh and along one upgrade |
| `task release:test` | The template check passed. Two controls each failed with the refusal of a real directory under `share`. One had the earlier install, and one had a real install beside the projection |
| `scripts/candidate-keg.sh` on the working-copy commit | The keg came from the template's install method. The aliases, the `brew link` links, the private origins and the digests passed |
| `task ravel:directory` | The 11 cases passed with fixture kegs in the projected layout |
| `bash scripts/check.sh` | All 18 principal checks passed |
| `task release:smoke` | The unchanged archive passed on `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu` and `x86_64-unknown-linux-gnu`. On each target, the 2.0 lane resolved the six archive sources |

Three more controls ran on scratch copies. Each failed as intended. The
earlier template made a keg that the layout comparison refused. A
marketplace file that named the public alias failed the marketplace check.
An absolute alias failed the installed smoke. A keg with real source
directories under `share` failed the link construction of the tap states.

### Linux scope

Only `task release:smoke` applies to Linux. It tests the archive layout on
each Linux target, and that layout is unchanged. The formula, its install
method, `brew link` and the tap states ran on macOS only. No Linux Homebrew
run exists, so Linux formula linking remains a stated limit.
