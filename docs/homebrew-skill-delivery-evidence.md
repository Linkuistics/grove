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

These readings establish the earlier keg layout only, with real source
directories. [The hook adapters on the projected keg](#the-hook-adapters-on-the-projected-keg)
holds the readings of the current layout.

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
holds its output, with the scratch directory shown as `S`. These readings
establish the earlier keg layout only, in which the writing source was a real
directory under `share`.
[The hook adapters on the projected keg](#the-hook-adapters-on-the-projected-keg)
holds the readings of the current layout.

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

## Shared directory seam

`shared-directory-seam-k294` delivered the durable seam of
[Shared default directory acceptance](specs/codex-skill-provisioning.md#shared-default-directory-acceptance).
It answers finding L3. It links actual formula output with Homebrew's own
linker into private prefixes. Ravel then resolves each prefix's default skill
directory with no settings. No real keg, link, tap or HOME changed.

### Instrument

`scripts/shared-directory-seam.sh` prepares the inputs, and
`scripts/shared-directory-seam.rb` runs the cases under `brew ruby`. For each
case, the instrument substitutes Homebrew's prefix, Cellar, cache, Caskroom,
lock and linked-keg constants. Each case therefore owns a new prefix and HOME.
The instrument calls the install method, `Cleaner#clean`, `Keg#link`,
`Keg#unlink`, `Keg#optlink` and `Keg#uninstall`. It calls them in the order of
Homebrew's own installer, upgrade, cleanup and uninstall code at
`bc63375ac3a4`. `Keg#link` rolls back a conflict by unlinking the whole keg,
so the duplicate cases observe Homebrew's own rollback.

Ravel's keg comes from Ravel's formula install method. The run rendered the
template of the Ravel workspace `skill-directory-delivery`, whose working
copy is the lane's source commit `ff14cba826b0`. The keg holds `examples`,
`notices` and `installation.json` under `share/ravel`. The record names the
case's own `share/ravel/skills`, because the case substitutes
`HOMEBREW_PREFIX`. Each resolution runs the keg's executable. Ravel finds that
record beside the canonical executable.

After each successful link, the instrument compares the exact source IDs, the
roots and the digests with the accepted values. It compares each
`originPath` with the owning keg's `libexec/skill-sources/<id>`. It compares
every entry of each source through the prefix with the pristine payload, with
its type and executable bit. A refused duplicate is followed by a check of the
earlier owner's sources and a scan of the prefix. The scan finds no link into
the refused keg outside `opt`.

These facts limit the instrument:

- Ravel prints no build identity. The instrument identifies Ravel's archive by
  its SHA-256 in a lane of `scripts/ravel_target_inputs.py`, and checks that
  `bin/ravel` holds the lane's identity. It refuses an unrecorded archive.
- The instrument runs install methods, not `FormulaInstaller` with
  downloads, bottles and dependencies. It does not exercise bottle
  relocation, a post-install step or Homebrew's sandbox.
- A tap formula without a plain checkout in the caller gets accepted values
  from its pristine private payloads. Its own `tap:formula:test` compares those
  payloads with its plain checkout.
- The seam runs on macOS only. No Linux Homebrew run exists.

### The run of 2026-11-10

`task release:formula:test` ran on the native archive of the working copy
with the published 22.8.0 keg as `OLD`. It passed every earlier step, tap
states A to D included, and then the seam. `task tap:formula:test` for
`personal` passed with the seam for its formula. The records are
`target/shared-directory-seam/20261110T120220Z.json` and
`20261110T120248Z.json`. Both runs digested each subject before and after,
and no subject changed.

| Subject | SHA-256 or revision |
| --- | --- |
| Homebrew | 7.0.12-221-gbc63375, revision `bc63375ac3a4d6debb1ceb441ae6e22854f0909a` |
| `keg.rb` | `0faa3f4cbb42c6ae7aa344ca04b129573ed9dc783d72b549bed02497c881552d`, equal to the experiment's linker |
| `formula_installer.rb` | `a7bb4d48a4b61978a76220de09a3a8c8b0a35b96a19d846f1b1a5a512bc98eff` |
| `install.rb` | `7f4e3e73c76b1e02457a2ec8c506f576227e178e3653872db90dde2e19afcdb3` |
| `cleaner.rb` | `6eafe14a3edefd5f760d5ffbbd35277b9cfd64ca8f600a976ae88753c56c7838` |
| `cleanup.rb` | `697d2965d4b38b68371443e4ebdd3a4f55e7b64706ab44a54578b12c2df2852b` |
| `uninstall.rb` | `77535bf2c75275dae9d525551a580d8dc458151a6b134bef986923b345945023` |
| `extend/pathname.rb` | `601bfa46a7c188eb95c0dc52259ad8586e84dd1206d92e3c08d57c4514a46d58` |
| `scripts/shared-directory-seam.rb` | `a5a45c4a184c2cd97ca93f05168916ba4b671e2ba882ea43eee57be7864aa929` |
| `scripts/shared-directory-seam.sh` | `b34521b00eaffb66fad28f832637c0193317b3c5a2263b1e826256f97766608d` |
| `scripts/skill-formulae.json` | `cd46d8bf25b9226adc8f62d95a5b0ad16f565077ec7ce1ebcb77fc6ca29d584a` |
| Ravel archive | `289b4ac57ffd148255960507f02809034e8fb35fe97132813230116add804921`, lane 2.0, build identity `58d479ed058f…` |
| Rendered Ravel formula | `b5b22ba8b45c41b35f2f66f3ec87cce47cf4883e28e5bac6f838d236c2110746`, package version 1.1.0 |
| Grove archive | `5f6093a6e6bb7c5c0e8fbcccff8907c7fe329a68a823c6d1cc3b1161508a885c` |
| Rendered Grove formula | `20896774f6c1f491c3cb4fbda29fd04f49ffb82f47c871e1a7205ed6b79e491b`, package version 22.8.0, projected |
| Tap | `homebrew-taps` at `origin/main` `9ca2b5e37510`, working copy equal |

The four tap formulae were the same file in the working copy and on the
published tap. `scripts/skill-formulae.json` declares each one `direct`, so
each took part only in the failure control.

| Formula | Package version | Formula SHA-256 | Archive SHA-256 |
| --- | --- | --- | --- |
| `personal` | 2026-11-06 | `6073283203325e06…` | `ff55ec0adea4e899…` |
| `superpowers` | 7.0.0 | `924bf5cfabb109fc…` | `496b576d4a4a62c0…` |
| `quint-llm-kit` | 1.0.0 | `d4f50a75dc29ecf4…` | `152947a80275a1d8…` |
| `testanyware` | 2.8.0 | `9af063f4c7e16eca…` | `bda57d356222e6ef…` |

The fixture formulae were `seam-fixture` 1 (direct) and 2 (projected),
`seam-control` 1 (direct), and `seam-duplicate` 1 and 2 (projected). The
second version of `seam-duplicate` also ships `grove-core` in the release run,
and `seam-fixture` in the tap run. The records hold each fixture's digest.

### Case results

Each case passed in both runs. The release run's observations follow.

| Case | Observation |
| --- | --- |
| `failure-control-forward` | The four published formulae and two direct fixtures linked. `personal`, linked first, expanded into file links. Ravel refused with `skill_resource_invalid` |
| `failure-control-reverse` | The same kegs in reverse published order. `testanyware`, linked first, expanded. Ravel refused with `skill_resource_invalid` |
| `order-forward` | From a prefix without `share`: Grove, `seam-fixture` 2, then `seam-duplicate` 1. After each link, every source matched |
| `order-reverse` | The reverse order. After each link, every source matched |
| `ravel-first` | Ravel linked first and held `share/ravel`. The three skill kegs then linked. Every source matched, and each of Ravel's five shared files still reached Ravel's keg |
| `ravel-promotion` | The skill kegs linked beside the keg-only Ravel. Ravel then linked. All eight sources matched, and Ravel's files reached its keg |
| `upgrade-cleanup-uninstall` | The seam upgraded each keg to its next revision, such as Grove 22.8.0_1, and `Keg#uninstall` removed the old keg. All eight sources matched after each step. Each uninstall removed its sources. With no keg left, Ravel reported `disabled` |
| `mixed-layout-migration` | Grove and `seam-duplicate` 1 linked after the direct `seam-fixture` 1, which expanded. Ravel refused with `skill_resource_invalid`. After the upgrade of `seam-fixture` to the projected version 2, all eight sources matched |
| `duplicate-fresh` | `seam-duplicate` 2 conflicted at `share/ravel/skills/grove-core`. `Keg#link` unlinked the whole keg. Grove's six sources and `seam-fixture` matched. No source or executable entry of the refused keg remained |
| `duplicate-upgrade` | The upgrade from `seam-duplicate` 1 to 2 conflicted at the same entry. The new keg linked nothing, and the earlier keg stayed unlinked. The seven other sources matched. `seam-duplicate` and `bin/seam-duplicate` left the prefix |

The duplicate upgrade reproduces finding L5 on the upgraded formula. Every
source of the upgraded formula leaves the default directory, and its other
entries leave the prefix too.

### Controls

The failure control is part of every run. It failed as required in both runs
before the projected cases passed. Two mutation controls ran on scratch
copies of the instrument with the fixture formulae, and each failed as
intended:

- With the direct `seam-control` among the projected kegs, the first
  comparison refused its `originPath`, which was not a private payload.
- With `personal` declared `projected`, the layout check refused its keg. Its
  `share/ravel/skills/personal` was not a relative symlink.

An earlier run found one defect in `release-formula.test.sh`. Its renderer
matched `@SHA_[A-Z_]*@`, which leaves `@SHA_AARCH64_APPLE_DARWIN@` in place.
The release test reads no checksum, so the defect was hidden. The seam
compares each formula's checksum with its archive, so the renderer now
matches `@SHA_[A-Z0-9_]*@`.

### The design experiment

`task design:keg-link-layout:test` stays as a cheap failure control. It needs
no formula, download or tap. The seam replaces it as acceptance of formula
output.

### Remaining work

The cases with all five revised formulae cannot pass before the tap formulae
have the projected layout. `tap-skill-keg-projection-k295` changed three
`layout` declarations in `scripts/skill-formulae.json` with their formulae,
and `testanyware-keg-projection-k296` changed the fourth.
`skill-formula-publication-k298` runs the seam with all five. After its
push, it changes each `published` declaration to `projected`. Release
acceptance repeats the seam with the published Ravel archives, after their
lane is in `scripts/ravel_target_inputs.py`.

## Tap skill keg projection

`tap-skill-keg-projection-k295` revised the tap formulae `personal`,
`superpowers` and `quint-llm-kit` to [the keg layout](specs/codex-skill-provisioning.md#the-keg-layout)
on 2026-11-10. Each formula installs its source in
`libexec/skill-sources/<source-id>`. A relative directory symlink at
`share/ravel/skills/<source-id>` is the public alias. Each formula adds
`revision 1`. Its URL, `sha256` and upstream version stay the same.

The tap holds the three revisions in the local commit `c0e61c1f3309`, above
the published `main` `9ca2b5e37510`. Nothing pushed that commit.
`skill-formula-publication-k298` publishes it with the TestAnyware revision.

### The test

`scripts/tap-formula.test.sh` reads the layout of its formula from
`scripts/skill-formulae.json`. For a projected formula, it checks these
facts:

- The alias is a symlink to `../../../libexec/skill-sources/<source-id>`.
- The payload is a real directory, and the alias canonicalises to it.
- `libexec/skill-sources` holds exactly the declared sources.
- The payload holds the files of the plain checkout, each with its Git
  mode. Each regular file of the commit is a regular file of the payload.
  The payload holds no entry of another type.
- Ravel's `originPath` names the payload.

`testanyware` keeps the direct layout until its revision, and the direct
branch of the test keeps its earlier checks.

The test then compares the native-only payload with its accepted subject.
That subject is the keg of the formula on the tap's `origin/main`. The
formula's own install and test methods make that keg in a second private
prefix.

The comparison covers `libexec/plugins` and each marketplace file
at the keg root. For each entry, it compares the type, the permission
mode, and the link target or the SHA-256. Hook declarations lie in those
files. The test names each file that declares a hook. "Skill formulae" of
the [owner delivery design](specs/owner-skill-delivery.md#skill-formulae)
requires this comparison.

The seam of the test now includes each other declared formula of the tap,
as `release:formula:test` does. In these runs, the other two revisions took
part as projected subjects. `testanyware` took part as a direct formula, in
the failure control only. `scripts/tap-formula.test.rb` now prints the full
package version, so the keg directory carries the revision.

### The runs of 2026-11-10

`task tap:formula:test` ran once for each formula with the finished
instrument. Each run passed. The Ravel 2.0 prefix was the archive
`289b4ac57ffd…` of lane 2.0, build identity `58d479ed058f…`. The seam
rendered the Ravel template of the workspace `skill-directory-delivery` at
`ff14cba826b0`. Homebrew was 7.0.12-221-gbc63375. The tap's `origin/main`
was `9ca2b5e37510`, equal to the remote's `main`.

| Formula | Full package version | Formula SHA-256 | Source, roots and `resourceDigest` | Seam record |
| --- | --- | --- | --- | --- |
| `personal` | `2026-11-06_1` | `37b7b69e0e5b2e00…` | `personal`, `craft`, `dc1f26eafe9b7db6…` | `20261110T122051Z.json` |
| `superpowers` | `7.0.0_1` | `169a987607502430…` | `superpowers`, `skills`, `be1ce6e3c25064b8…` | `20261110T122123Z.json` |
| `quint-llm-kit` | `1.0.0_1` | `29cf32515afef6b1…` | `quint`, `skills`, `0a8b945fbc6732d5…` | `20261110T122156Z.json` |

The seam records lie in `target/shared-directory-seam/`. Each digest equals
the accepted value of the
[repository moves record](repository-moves-record.md). The commit, the
source directory and the roots are unchanged too. No source therefore needs
new staging trials.

The committed formula files have the SHA-256 values of
the measured files. The measured subjects were the formulae, the test and
its helper, the declarations, the seam, the Ravel template and the Ravel
archive. The SHA-256 of each subject was the same before and after the
three runs.

Each seam run passed its 10 cases. The failure control linked the four
published direct formulae and expanded. The migrations were `personal`
2026-11-06 to 2026-11-06_1, `superpowers` 7.0.0 to 7.0.0_1 and
`quint-llm-kit` 1.0.0 to 1.0.0_1. Each revision has a new package version,
so the seam accepted each pair.

For `personal`, 12 entries of `libexec/plugins` and
`.claude-plugin/marketplace.json` equal those of `personal` 2026-11-06. The
file that declares the hook is
`libexec/plugins/guardrail/skills/guardrail/SKILL.md`. Its SHA-256 is
`aca6380f08492a17…` in both kegs.

The documented commands registered the
revised keg's marketplace in an isolated Claude Code home and installed
`guardrail@personal`. `claude plugin validate` passed with one warning:
the `guardrail` manifest declares no version. The published keg has the
same manifest. This equal payload does not replace the `guardrail`
capability trial. `hook-adapter-keg-seam-k297` repeated that trial on the
revised keg, in
[The guardrail capability trial](#the-guardrail-capability-trial).

### Controls

Three controls ran with the finished checks, and each failed at its own
check:

| Control | Result |
| --- | --- |
| The published direct `personal` formula, against its `projected` declaration | The layout check refused it, because `share/ravel/skills/personal` was not a symlink |
| The revised `personal` formula in a scratch clone of the tap, with mode 0755 on `guardrail/PROVENANCE.md` | The payload comparison refused it and showed mode 644 against 755 |
| The same clone, with one line added to the hook-declaring `SKILL.md` | The payload comparison refused it and showed the two SHA-256 values |

An earlier run of `personal` passed before two changes to the test. One
change joined the list of native-only entries on one line in its message.
The other change added the tap's other formulae to the seam. The table
above reports only the runs after those changes.

## TestAnyware keg projection

`testanyware-keg-projection-k296` changed TestAnyware's formula template and
the tap's `testanyware` formula to
[the keg layout](specs/codex-skill-provisioning.md#the-keg-layout) on
2026-11-10. The release archive keeps the source at
`share/ravel/skills/testanyware`. The formula moves it to
`libexec/skill-sources/testanyware`, and `install_symlink` makes the relative
alias. The executables, `pkgshare` and the Linux library and venv steps did
not change.

### The template and its test

TestAnyware commit `cd2bcf5f35de` holds the template change and is on its
pushed `main`. It also updates the section "Agent skill" of TestAnyware's
README. The template renders no `revision`. TestAnyware's
`release-publish.sh` copies the rendered formula over the tap's formula, so
a release never carries the tap's revision forward. A comment in the
template states that rule.

`scripts/tests/release-pipeline-test.sh` now checks five facts in the
rendered formula:

- The install method moves the source into `libexec/skill-sources`.
- The install method makes the alias with `install_symlink`.
- The test method asserts the target of the alias.
- No line installs a real directory in `share/ravel/skills`.
- The formula has no `revision` line.

`task release:test` passed with 93 checks and no failure. Its control was
the template of TestAnyware's previous `main`, `1809a49068a2`. With that
template, the four layout checks failed. A rendering with `revision 1` failed
the revision check.

### The tap formula

The tap holds the revision in the local commit `12b53f8c449d`, above the
commit `c0e61c1f3309` of `tap-skill-keg-projection-k295`. The tap's `main`
stays at `9ca2b5e37510`, and nothing pushed either commit. The formula is the
new template rendered at 2.8.0 with the three `sha256` values of the
release, plus a comment and `revision 1`. The previous template, rendered the
same way, is byte-equal to the published formula. The two formulae therefore
differ only in the layout and the revision.

`scripts/skill-formulae.json` now declares the `projected` layout for
`testanyware`. Its `published` declaration stays `direct` until
`skill-formula-publication-k298` pushes the tap.

### The run of 2026-11-10

`task tap:formula:test` ran once with `SOURCES=testanyware=plugins/testanyware`
and the release commit `1809a49068a23d9d276913f3a90227dbefb98a82`. It passed.
The inputs were the same as in the runs of k295:

- The Ravel 2.0 prefix was the archive `289b4ac57ffd…` of lane 2.0, with the
  build identity `58d479ed058f…`.
- The seam rendered the Ravel template of the workspace
  `skill-directory-delivery` at `ff14cba826b0`.
- Homebrew was 7.0.12-221-gbc63375.
- The tap's `origin/main` was `9ca2b5e37510`.

| Formula | Full package version | Formula SHA-256 | Source, roots and `resourceDigest` | Seam record |
| --- | --- | --- | --- | --- |
| `testanyware` | `2.8.0_1` | `ad71af0f77f78965…` | `testanyware`, `skills`, `3e9d7929eddd8988…` | `20261110T123729Z.json` |

The digest equals the value that the
[repository moves record](repository-moves-record.md) gives for the 2.8.0
keg. The source therefore needs no new staging trials. The payload held the
files of `plugins/testanyware` with their Git modes.

The seam passed its 10 cases. All four tap formulae took part as projected
subjects. In the failure control, the published direct `testanyware` 2.8.0
expanded into file links, and Ravel refused the resolution. The migration was
2.8.0 to 2.8.0_1. That version is new, so the seam accepted the pair.

The measured subjects were the four tap formulae, the test and its helper,
the declarations, the seam, the Ravel template and the Ravel archive. The
SHA-256 of each subject was the same before and after the run.

### Control

The formula of the tap's `origin/main` ran against the new `projected`
declaration. The layout check refused it, because
`share/ravel/skills/testanyware` was not a symlink.

## The hook adapters on the projected keg

`hook-adapter-keg-seam-k297` repeated the seam "Hook adapters" and
"Native-only hook acceptance" of the
[delivery specification](specs/codex-skill-provisioning.md#test-seams) on
2026-11-10. The subjects were the revised formula output of
`grove-keg-projection-k293` and `tap-skill-keg-projection-k295`. The
earlier sections of this document read real source directories, so they
establish the earlier layout only.
[`evaluations/hook-adapters/projected-keg.sh`](evaluations/hook-adapters/projected-keg.sh)
ran the seam, and
[`projected-keg.out`](evaluations/hook-adapters/projected-keg.out) holds its
output. The output shows the scratch directory as `S` and the owner's
Homebrew prefix as `PREFIX`.

### Subjects and isolation

| Subject | Value |
| --- | --- |
| Grove archive | `grove-v22.8.0-aarch64-apple-darwin.tar.xz` of the working copy, SHA-256 `5f6093a6e6bb…`. The shared-directory seam measured the same archive. Its two marketplace files equal those of `assets/keg-root`, and each names `./libexec/skill-sources/writing/craft/technical-communication` |
| Grove formula | A disposable `grove` in a scratch tap. It takes the `SKILL_SOURCES` constant and the `install` block of `scripts/templates/grove.rb.tmpl`, SHA-256 `76cbd91d26de…`. Versions 1 and 2 add the line "Probe keg: one" or "Probe keg: two" to `always-on/instruction.md`, which the `SessionStart` hook prints |
| `personal` formula | The file of the tap working copy, verbatim, from the local commit `c0e61c1f3309`. SHA-256 `37b7b69e0e5b…`, full package version `2026-11-06_1` |
| Accepted `guardrail` subject | The owner's keg `PREFIX/Cellar/personal/2026-11-06`, read only. The staging capability trials loaded `guardrail` from it |
| Homebrew | A `--shared` clone of the owner's repository at `bc63375ac3a4`, with its own prefix in the scratch directory |
| Harnesses | Claude Code 2.1.322, Codex 0.167.0 and Pi 0.84.2 |
| Model of the `guardrail` sessions | `claude-sonnet-5-5` |
| Instrument | SHA-256 `ff5c5d001d88…`. The output is SHA-256 `60c64baff227…` |

Each harness ran in a HOME of its own. Each ran the documented commands
verbatim, with the private Homebrew first on `PATH`, so `$(brew --prefix)`
named the scratch prefix. The Claude Code HOME of the hook adapters had no
sign-in, so no request reached a model. Pi ran without a provider. Codex
linked only the owner's `auth.json`, and each `codex exec` sent one short
prompt. Each `guardrail` HOME linked only `Library/Keychains`, for the
sign-in.

The instrument digested each subject before and after the run, and no digest
changed. It also digested the owner's harness files, the owner's `opt` links,
the owner's kegs and the owner's default skill directory before and after
the run. No digest changed. The owner's Codex `auth.json` and the Claude Code
sign-in item in the Keychain are outside that check, because the sessions use
them.

### The hook adapters

| Reading | Claude Code | Codex | Pi |
| --- | --- | --- | --- |
| Keg layout | `share/ravel/skills/writing` in the keg is the alias `../../../libexec/skill-sources/writing`. The payload is a real directory. Homebrew linked the alias into the prefix | Same keg | Same keg |
| Registration | The marketplace stores the `opt` path. `plugin details`: skills 0, hooks 6 | The marketplace stores `Cellar/grove/1`. The cache copy equals the private payload in 31 entries, with paths, types, modes and SHA-256 values | The link names `share/ravel/skills/writing/...` in the prefix and resolves into `Cellar/grove/1/libexec/skill-sources/writing` |
| Session at version 1 | `SessionStart` gave "Probe keg: one", and `UserPromptSubmit` ran. The init event names the plugin at `opt/grove/libexec/skill-sources/writing/craft/technical-communication` and lists no `technical-communication` skill. The HOME holds no native skill directory | Before the review, no hook ran. After "Trust all and continue", six hooks were trusted, the session file holds "Probe keg: one", and the reply log holds a start and a reply | The extension wrote a start record with the source `startup` |

`brew upgrade` installed version 2, and its cleanup removed `Cellar/grove/1`.
Only `Cellar/grove/2` remained, and `opt/grove` named it.

| Reading after the upgrade | Claude Code | Codex | Pi |
| --- | --- | --- | --- |
| Listing | `plugin list` reads the plugin from the `opt` path of the private payload | `plugin list` fails: "marketplace root does not contain a supported manifest" | The link resolves into `Cellar/grove/2` |
| Session before any repair | `SessionStart` gave "Probe keg: two", from the new keg | The hooks ran from the cache copy. The session file holds "Probe keg: one" | The extension wrote a second start record |
| Repair | None needed | The documented `marketplace add` stored `Cellar/grove/2`, and `plugin add` copied version 2. The cache copy again equals the payload in 31 entries | None needed |
| Session after the repair | Not applicable | No hook ran until the interactive review. "Trust all and continue" accepted six changed hooks. The session file then holds "Probe keg: two" | Not applicable |

No harness refused the private payload or the public alias. Codex accepted
the `local` source at `./libexec/skill-sources/...` inside its marketplace
root. Each lifecycle observation equals that of the earlier layout.

### The `guardrail` payload

The revised keg holds `guardrail` at `libexec/plugins/guardrail`, as the
accepted keg does. The comparison covered `libexec/plugins` and
`.claude-plugin`:

- The 10 entries of `libexec/plugins` are equal in path, type, mode, link
  target and SHA-256.
- The one entry of `.claude-plugin`, the marketplace file, is equal in the
  same way.
- The hook declarations are equal. The only declaration is the `hooks:`
  block of `skills/guardrail/SKILL.md`, with two `PreToolUse` matchers. The
  plugin manifest declares no hooks, and no `hooks.json` exists.

### The `guardrail` capability trial

The trial repeats the two turns of the
[guardrail plugin trial](repository-moves-record.md#guardrail-plugin-trial).
The first turn is `/guardrail`. The second asks the agent to run
`rm -rf scratch-dir` once with the Bash tool. The allow rule permits that
command, so without the hook the command runs. The refusal assertion passes
only when a `PreToolUse` hook returns `ask` and `scratch-dir` remains after
the session. One verdict function judges both sessions.

The no-op control ran first. It registered a scratch copy of the revised keg
whose `guardrail-hook.sh` only exits 0.

| Reading | No-op control | Trial from the revised keg |
| --- | --- | --- |
| Registration | `marketplace add` of the scratch copy, then `plugin install guardrail@personal` | The documented commands. The marketplace stores the `opt` path. `plugin details`: skills 1, `guardrail`. Hooks 0, because the hook lies in the skill's frontmatter |
| Plugin in the init event | `S/noop-keg/libexec/plugins/guardrail` | `S/brew/opt/personal/libexec/plugins/guardrail` |
| Command in the session file | `/guardrail:guardrail` | `/guardrail:guardrail` |
| `PreToolUse` hook | One start and one response, with no `ask` | One start and one response, with `ask` and the reason "guardrail: recursive forced delete (rm -rf). Confirm you intend to run: rm -rf scratch-dir" |
| Tool result | `(Bash completed with no output)` | An error that carries the reason, and one `permission_denied` event |
| `scratch-dir` after the session | Absent | Present |
| Refusal assertion | **Fail**, as required | **Pass** |
| Session | `4370c622-a32e-4807-ae50-1921a9fecf28` | `66fcf5e8-23cf-4e6e-96f2-94cd6a17d477` |

The control ran its hook and failed the assertion, so the assertion detects
a hook that does not refuse. Claude Code ran `guardrail` in place from the
registered `opt` path of the revised keg, and its capability trial passed.

### Runs

The first full run passed every reading, and its control failed as required.
Three of its readings were defective. The owner files kept the real home
path, the slash-command check read the stream instead of the session file,
and each init event printed twice. After a change to those three readings,
the instrument ran again from an empty scratch directory. The tables above report only that second run.

### Limits

- Print mode only. Like the earlier trial, the readings show the hook's
  `ask` and the refusal in print mode, not the interactive confirmation.
- The upgrade case uses two fixture revisions of Grove's output with one
  changed line. It does not upgrade `personal`, because the `guardrail`
  payload is equal in both kegs.
- Each upgrade reading names one keg change. A running session across an
  upgrade is outside this seam, as in the earlier readings.
- macOS only. No Linux Homebrew run exists.

### Rerun

Run the seam from the repository root with an empty scratch directory. It
spends real Claude Code and Codex sessions, so `scripts/check.sh` does not
run it.

```sh
mise exec -- task seam:hook-adapters \
  ARCHIVE=target/archives/grove-v22.8.0-aarch64-apple-darwin.tar.xz \
  TAP=~/Development/Delivery/homebrew-taps SCRATCH="$(mktemp -d)"
```
