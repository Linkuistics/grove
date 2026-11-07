# Owner skill delivery through Ravel

This design moves the owner's installed skill collection from native harness
installations to Ravel's directory delivery. Its focused design review is
integrated. Ravel later narrowed delivery to local directories, and mise
checkouts replaced the earlier policy of Git sources. Three focused reviews of
that change are also integrated.

`homebrew-delivery-design-k216` then replaced the mise checkouts with Homebrew
formulae, as the decision record
[Homebrew installs skills and Ravel delivers them](../adr/homebrew-installs-skills-and-ravel-delivers-them.md)
states. [Skill formulae](#skill-formulae) holds the owner's setup. Finish now
installs the writing hooks plugin before the switch changes a cell, as
[The hooks installation inside finish](#the-hooks-installation-inside-finish)
states. The installation adds no state and no inverse to the switch.

`ravel-default-skill-directory-k218` then settled Ravel's default skill
directory. Ravel 2.0 delivers the keg sources when the settings name no
`sources`, so the native state now holds `"sources": []`. Finish writes it
after promotion, as
[The native settings inside finish](#the-native-settings-inside-finish) states.

`native-settings-recovery-k221` then moved both of these preludes into step 3
of the switch. They run only while no run is open, so each run of the switch
reaches its row of the state table. The native settings now write a settings
record before their exchange, so detection covers that write too.

`hooks-installation-writes-k222` then measured how Claude Code writes
`settings.json`. The hooks installation now disables its plugin by the
program's own exchange. Each of its Claude Code commands runs under a held
link, and one accepted case remains, as
[Claude Code's own writes](#claude-codes-own-writes) states.

The leaf `owner-switch-recovery-k87` then rebuilt the switch's recovery on
cells, a run record and detection of outside writes. A Quint model supports
that recovery, and its focused review is ahead. The design changes no
installation, setting or file on the owner's machine.
The [evidence document](../owner-skill-delivery-evidence.md) records the
inputs, and [the baseline](../owner-skill-inventory.tsv) classifies each
installed entry.

Ravel's directory contract owns the feature itself. The
[coordinated release design](coordinated-release.md) owns publication order,
promotion and recovery. The [Grove delivery design](codex-skill-provisioning.md)
owns Grove's behaviour and the general cleanup procedure for other users. This
document owns the owner's source set, the native capabilities that stay, and the
ordered cutover on this machine.

## Problem

Four routes deliver skills on this machine today, and they disagree:

- **Claude Code** installs marketplace plugins. The Linkuistics marketplace
  follows `Linkuistics/grove` main with automatic updates. Third-party
  marketplaces and the personal skill directory add more entries.
- **Codex** receives Grove 22.8's release snapshot through links in both of
  its user roots. Older superpowers copies and links into a Quint clone sit
  beside them.
- **Pi** has manual links to the earlier checkout path. Each link dangles.
- **Tools** install their own skills and hooks: codebase-memory-mcp, herdr,
  Orca, and a terminal-browser cask that no longer exists.

The same skill therefore has different bytes in different harnesses, and some
names appear twice. An earlier repository move broke every Pi link without a
report. The planned pack moves will move every owner source again.

The owner's target is Ravel insertion of the directories of every
owner-selected skill. Four facts constrain the transition:

1. Ravel delivers readable directories and ordered mandatory reads. It installs
   no hook, command, agent definition, MCP server or tool.
2. This driver resolved Ravel 1.1.0 once, at its canonical path. That version
   cannot insert directories. Every remaining session of this grove loads its skills
   natively. The owner policy routes `finish` to Claude Code.
3. Grove 22.8 rewrites its Codex links at each Codex launch. Only compatible
   Grove, which finish releases, stops that writer.
4. Ravel 1.1 and Ravel 2.0 read the same `~/.config/ravel/settings.json`. The
   file does not exist today. A 1.1 launch refuses settings that hold keys it
   does not know.

## Solution

1. Classify every baseline entry by its owner and by what it supplies. Only
   owner-selected guidance moves to Ravel.
2. Deliver that guidance from ten local Ravel sources, with one mandatory file.
   Homebrew installs each source at a published commit, and mise declares the
   formulae. Ravel finds the sources in its default skill directory.
3. Keep every native capability that a read cannot supply. That covers hooks,
   commands, agent definitions, MCP and LSP servers, tools, platform skills and
   one Claude Code skill.
4. Prove the replacement on both harnesses in a staging HOME before finish.
5. Inside finish, after Grove's release and Ravel's promotion, switch the real
   HOME in one reversible operation. Verify both harnesses, and roll back on
   failure.
6. Purge retained bytes only after this driver's reap, on reader evidence.

The classification view (`#diagram-owner-classes`), the cutover view
(`#diagram-owner-cutover`) and the final-session view
(`#diagram-owner-final-session`) in the
[visual views](../design/harness-selection-and-execution/README.md) show the
same design.

## Decisions

### Terms

- **Deactivation** takes an entry out of native discovery and keeps its bytes.
  For a Claude Code plugin, it disables the plugin. For a file-system entry, it
  moves the entry into the retained tree of the cutover backup.
- **Purge** erases retained bytes. Removal of a native installation is
  deactivation followed, later, by purge.
- **The switch** is the one operation that activates directory delivery in the
  real HOME and deactivates every superseded entry.
- **The switch set** is each item that the switch changes. That is each moved
  file-system entry, each plugin whose state changes, and the `skills` key of
  the Ravel settings.
- **A cell** is one value that the switch changes: one key of a shared file,
  or one file-system entry. The plan records its earlier and switched values.
- **A part** is one atomic step of the switch. A file part exchanges one file
  and changes each of its cells together. An entry part renames one entry.
- **The plan** is the prepared manifest, the ordered parts with the values of
  each cell, and the inverse that follows from them. Its digest identifies it.
- **The program** is the prepared command that runs a plan: its switch, its
  inverse and the reading of its state.
- **A kept copy** is the content that an exchange or a Claude Code command
  displaced. After an exchange, it stays under the name of its stage file,
  in the backup or in a settings record. After a command, it stays under a
  held link.
- **A prelude** is one of two steps that step 3 of the switch runs while no
  run is open. They are the native settings and the hooks installation. A
  prelude leaves each of its cells at its earlier value, or it stops.
- **A settings record** holds the digests of one write of a prelude. For an
  exchange, it also holds the stage file, and after the exchange the name of
  the stage file holds the kept copy. For a Claude Code command, it holds a
  held link. A settings record holds no plan and no run record, so it is not
  a backup.
- **A held link** is a hard link in a settings record. It names the inode
  at the path of `~/.claude/settings.json` before a Claude Code command.
  While that inode stays at the path, the link holds the live file and no
  kept copy. After a rename displaces the inode, the link holds a kept copy.
- **A kept process** is a process that the session which runs the switch
  cannot end without ending itself. The process listing of step 1 excludes
  only kept processes.
- **Native residue** is the set of native entries that stay after the switch.
- **A keg source** is the copy of one source directory in a keg, at
  `share/ravel/skills/<source-id>`. Homebrew links it into Ravel's default
  skill directory, and Ravel captures it there as a `local` source.
- **An accepted version** is a formula version whose keg sources passed the
  staging trials. Each of its sources has an accepted `resourceDigest`.
- **The candidate keg** is Grove's keg before Grove's release. It is the
  production formula's install method over an unreleased archive, in a private
  prefix.
- **The off value** is the Ravel settings value
  `"skills": {"schemaVersion": 1, "sources": []}`. With it, Ravel 2.0 delivers
  no source, although keg sources are installed.

### Classes and dispositions

| Class | Baseline members | Disposition |
| --- | --- | --- |
| Owner pack | The `grove`, `linkuistics` and `testanyware` plugins and their skills. Grove's Codex snapshot links. | Ravel source from its formula's keg. Deactivate the native entries in the switch. |
| Selected third-party pack | Superpowers 7.0.0 and quint-llm-kit 1.0.0, with their Codex copies and links | Ravel source from a Linkuistics formula at the upstream commit. Deactivate the native entries in the switch. |
| Stale copy | The earlier `grove` copy in `~/.claude/skills`. The superpowers copies in `~/.codex/skills`. | Deactivate in the switch. |
| Dangling entry | `terminal-browser` links in three roots. The Pi skill links and the Pi technical-communication extension link. | Deactivate in the switch. No reader can read through them. |
| Tool-managed | codebase-memory skill copies, agent definitions and hooks. herdr and Orca hooks. The `testanyware`, `quint` and `codebase-memory-mcp` executables. | Unchanged. Their installers own them. |
| Capability-only | Enabled plugins without skills: Context7, Playwright, commit-commands, feature-dev, the explanatory output style and Swift LSP. All MCP entries. | Unchanged. |
| Platform-managed | Claude Code built-in skills, account-synced skills, Codex system skills, and the skills of Codex's bundled, runtime and remote-installed plugins | Unchanged. They are outside the owner's control and are no Ravel source. |
| Disabled third-party | Claude Code plugins that the owner disabled, with their skills | Outside the active collection. Unchanged. |

Two owner-pack skills have their own route:

- **`guardrail`** stays a native Claude Code skill. A read cannot install its
  frontmatter hook or its `/guardrail` command. It stays outside every Ravel
  search root.
- **`technical-communication`** splits. Ravel delivers its guidance. A
  skill-free native plugin keeps its always-on hooks and their scripts.

Pi is outside the automatic contract, because no owner policy route selects
it. The switch deactivates its dangling entries and gives it no replacement.
If the owner reinstalls the terminal-browser cask, its installer owns those
links again.

### Ravel sources

| Source ID | Formula and repository | Search roots | Delivered skills |
| --- | --- | --- | --- |
| `grove-core` | `grove`, from `Linkuistics/grove` | `skills` | Spine, `configure-grove`, finish, and the planning chain |
| `requirements` | `grove`, from RequirementsSkills | `skills` | The requirements chain |
| `design` | `grove`, from DesignSkills | `skills` and `craft` | The design chain, `decision-records`, `codebase-design`, `cli-tool-design` and `model-led-development` |
| `engineering` | `grove`, from EngineeringSkills | `skills` and `craft` | The impl and prototype chains, the coding styles, `using-codebase-memory`, `simplify-project` and `doubt-driven-development` |
| `research` | `grove`, from ResearchSkills | `skills` | The research pair, the combiner and `research-family` |
| `writing` | `grove`, from WritingSkills | `skills` and `craft` | The editorial kinds, `technical-communication` and `writing-code-walkthroughs` |
| `personal` | `personal`, from PersonalSkills | `craft` | `machine-environment`, `using-jujutsu` and `authoring-conventions` |
| `testanyware` | `testanyware`, from TestAnyware | `skills` | `using-testanyware` |
| `superpowers` | `superpowers`, from `obra/superpowers` | `skills` | All fifteen superpowers skills |
| `quint` | `quint-llm-kit`, from `quint-co/quint-llm-kit` | `skills` | `quint-lang`, `quint-modeling` and `quint-execute-spec` |

The source path of each row is
`$(brew --prefix)/share/ravel/skills/<source-id>`, in Ravel's default skill
directory. The
[external pack contract](external-pack-contract.md#layout) owns the layout of
each pack's source directory. The
[keg layout](codex-skill-provisioning.md#the-keg-layout) owns where each
formula puts a source, and the identity of a keg source. Ravel's skill
discovery specification, in its section "Default skill directory", owns how
Ravel reads the directory and each source's roots.

These rules bind the owner's sources:

- The real settings name no source, so the default skill directory supplies
  the ten. No other formula installs a source there. Step 6 of the switch
  compares the resolved IDs with the ten.
- Where trial settings name a source, they name a candidate keg path or a path
  in the default directory, never a versioned keg path. Homebrew's cleanup
  erases an earlier keg.
- Keep native-only skills and plugin-only trees outside every source.

The first commits of the two third-party formulae are known: superpowers
`bb92a77741419a4ab5f06e711a283343f1ada0c3` and quint-llm-kit
`cc75369f741af7d490936f82002c2d28e3b3d78d`. These are the commits of the
installed plugins. The formula author reads both again before the formulae
publish.

### Skill formulae

Homebrew installs each source, and Ravel reads the keg. Neither Ravel nor Grove
fetches a source. The [keg layout](codex-skill-provisioning.md#the-keg-layout)
states where each formula puts its sources, their stable paths and their
identity. This section states the owner's setup.

The owner's `~/.config/mise/config.toml` declares each skill formula in
`[bootstrap.packages]`, and mise runs Homebrew. No `[bootstrap.repos]` entry
names a skill source.

| Declaration | Sources |
| --- | --- |
| `"brew:linkuistics/taps/grove" = "latest"` | `grove-core`, `requirements`, `design`, `engineering`, `research` and `writing` |
| `"brew:linkuistics/taps/personal" = "latest"` | `personal`, and the `guardrail` plugin |
| `"brew:linkuistics/taps/testanyware" = "latest"` | `testanyware` |
| `"brew:linkuistics/taps/superpowers" = "latest"` | `superpowers` |
| `"brew:linkuistics/taps/quint-llm-kit" = "latest"` | `quint` |

The owner runs these commands. Each names the HOME with `-C`, as
`mac-bootstrap` does, so that a project's `mise.toml` cannot change an entry.

| Purpose | Command | Effect |
| --- | --- | --- |
| Installation on a new machine | `mise bootstrap --adopt AntonyBlakey/mise-setup` | Restores the tracked files first, the Ravel settings included. Then its packages part installs the formulae. Until that part installs `superpowers`, a launch refuses as `skill_source_missing`, because the mandatory file names that source |
| Installation on a configured machine | `mac-bootstrap`, which runs `mise -C "$HOME" bootstrap` | Installs each missing formula among the other parts |
| Installation of the packages only | `mise -C "$HOME" bootstrap packages apply` | Installs each missing formula, and upgrades none |
| Advance | `brew update`, then `brew upgrade <formula>` | Installs the tap's current version in a new keg, then links it |
| Report of drift | `mac-check` | Reports each skill formula's installed version against the tap's current version |

A formula version names a commit. For Grove and TestAnyware it is a release
tag. For the other three it is the commit of the formula's source archive.

The tap publishes only accepted versions. Grove's release task and TestAnyware's
release publish theirs. The owner edits the other three formulae after their
staging trials. The tap's current version is therefore the accepted version.

Homebrew installs only the tap's current version. A rebuilt machine therefore
installs the current versions, not earlier ones.

`mac-check` reads `mise -C "$HOME" bootstrap packages status --json` for each
declaration and its installed version. It reads `brew info --json=v2` for the
tap's current version. A skill formula that is not installed, or a status that
it cannot read, is a configuration error. An installed version other than the
tap's current version is drift.

Grove's keg holds core. The delivered spine and the installed Grove therefore
come from one release, and no separate core pin exists.

During this cutover, the staging trials read Grove's six sources from the
candidate keg. Grove's real keg holds them only after Grove's release, inside
finish. The candidate is the last published commit of this workstream's line
that changes `plugins/grove` or `plugins/packs.json`. A later change needs a
new candidate keg and a rerun of the trials that read the changed sources.

A committed evidence document records the candidate commit and the accepted
`resourceDigest` of each of Grove's six sources. Before finish, rollout builds
the candidate keg of the final line and shows that each digest is unchanged.
`task owner:candidate-keg` builds a candidate keg and prints each digest. Its
`ACCEPTED` variable names an earlier digest file, and a different digest fails
the task.
After the release, the switch compares the digests of the installed keg, as
its step 4 states.

The staging trials read the four other formulae from their installed kegs.
Installing a formula changes no HOME and no discovery, so those installations
can precede the staging trials. After the Ravel cut, rollout resolves every
source again with the published archives and records the same digests. Before
the switch changes anything, each installed keg source must give its accepted
digest.

Nobody edits a keg or runs a tool inside it. Homebrew leaves `libexec` files
writable, and Ravel would capture an edit. Development happens in the working
copy of each pack under `~/Development/AgenticOrchestration`.

An advance unlinks the earlier keg, installs the new keg and links it, as
Ravel's specification states. A capture therefore reads one complete keg, or
it finds the source absent. Each Homebrew installation or upgrade of a skill
formula is an advance. That includes a plain `brew upgrade` of every formula.

A launch during an advance delivers no source of the advancing formula, and
the result depends on the formula:

- For `superpowers`, the launch refuses, because the mandatory file names that
  source.
- For `grove`, the session finds no kind skill, so it reports and force-stops.
- For `personal`, `testanyware` and `quint-llm-kit`, the session lacks their
  craft skills, and nothing reports it.

The owner therefore runs an advance while no Grove loop launches. An advance
needs no other interval without captures.

Ravel 2.0 captures each keg source at each `ravel start-session` and each
`ravel skills resolve` whose settings name no `sources`. It captures a named
source when the settings name it. A Ravel 1.1 launch captures nothing, and the
off value captures nothing either. A running session reads its retained
snapshot until it runs Ravel again.

Before promotion, only Ravel 1.1 reads the real HOME's settings, so preparation
captures nothing from them. After promotion, step 3 of the switch writes the
off value, as
[The native settings inside finish](#the-native-settings-inside-finish) states.
Steps 4 and 6 resolve in a disposable HOME. The switch therefore captures
nothing from the real settings until its step 9 writes the switched value. An unpublished edit reaches a launch only through
trial settings in a staging HOME. Those settings can name a development
working copy as a `local` source.

A launch behaves as follows:

- Ravel captures each keg source at each launch. It reuses the published
  snapshot while the files stay the same. Each keg version therefore adds at
  most one snapshot.
- A formula that is not installed supplies no source, and the launch delivers
  the others. For `superpowers`, the launch refuses as `skill_source_missing`,
  exit 3, before Ravel launches any worker. For `grove`, the session
  force-stops. `mac-check` reports each missing formula. The owner runs
  `mise -C "$HOME" bootstrap packages apply`, then reruns Grove. No launch
  substitutes native skills or another version.
- Ravel reads no version, so a keg at another version delivers its files
  without a report. `mac-check` reports that state.

After the first installation, the route needs no network except for an
advance:

- A launch needs no network, because Ravel reads the kegs on disk.
- An advance fails without a network, because Homebrew downloads before it
  changes a keg. The earlier keg keeps launching, and `mac-check` reports
  drift until an advance succeeds.
- A missing formula stays missing until a network is available.

The policy costs the automatic updates that the Claude Code marketplace gives
today. An unpublished edit cannot reach a default launch until its formula
advances. In return, both harnesses read identical bytes, launches need no
network, and each keg version has one snapshot. A third-party remote can
rewrite its history. When a formula's commit leaves its remote, a rebuilt
machine cannot install that formula, and the owner chooses a new commit. An
existing keg keeps working.

The design rejected these alternatives:

- **Development working copies as default sources.** Each launch would capture
  uncommitted or conflicted jj states. Ravel retains every distinct capture and
  has no collector, so storage would grow with each edit.
- **Formulae that follow a branch.** Nothing would record which commit a keg
  holds, and mise cannot evaluate a formula without a `sha256`.
- **mise checkouts in `[bootstrap.repos]`.** They are specific to mise, and the
  human chose Homebrew, which mise runs. skill-marketplace-decision-k201
  records that choice.
- **The Claude Code marketplace clones.** Claude Code updates and erases them,
  and Codex delivery would then depend on Claude Code.

### Mandatory guidance

The default mandatory list holds one file: superpowers
`skills/using-superpowers/SKILL.md`.

- It replaces the superpowers plugin's `SessionStart` hook. That hook injects
  the same text into every Claude Code session today.
- Through Ravel, the file also reaches Codex launches. That is a deliberate
  change, because Codex today has older copies without that text.
- The file is 3,192 bytes, well under the historical 64 KiB bound. Its
  harness references are relative reads inside the same source.

The always-on instruction of `technical-communication` is not mandatory. The
[off-file decision](https://github.com/Linkuistics/WritingSkills/blob/main/docs/adr/the-off-file-is-the-only-switch.md) makes one file
the only switch for every always-on layer. Ravel reads a mandatory file
unconditionally, so it would ignore that switch. No Grove or personal guidance
is mandatory, because Grove's mandate names the kind skill. The explanatory
output style is a harness style rather than guidance. It stays a native hook.

### Native residue

| Capability | Route after the switch |
| --- | --- |
| Always-on technical communication on Claude Code | The skill-free plugin `technical-communication-hooks@writing-hooks` from Grove's keg. Claude Code runs it in place through the `opt` path. Its root is the skill directory, as for the Codex hooks plugin. |
| Always-on technical communication on Codex | Absent today, and the switch keeps it absent. Installing the existing Codex hooks plugin is a separate owner choice. |
| `guardrail` | `guardrail@personal` from the personal keg. It holds only `guardrail` |
| Superpowers bootstrap text | The Ravel mandatory file. The superpowers plugin is disabled. |
| Explanatory output style, Context7, Playwright, commit-commands, feature-dev and Swift LSP | The same enabled plugins, which hold no skills |
| codebase-memory MCP server, hooks, agent definitions and skill copies | Unchanged. Its own updater rewrites them. |
| herdr and Orca hooks | Unchanged. Their applications own them. |
| `quint-kb` MCP server | Unchanged. It runs from the Quint clone, which stays. |
| TestAnyware command, application and configuration | Unchanged installations. The skill reaches them by ordinary command execution. |

The repository moves and homebrew-skill-delivery-k215 supply three packaging
changes before the switch:

- WritingSkills offers the technical-communication hooks as a Claude Code
  plugin without a `skills/` directory. A trial must show that Claude Code runs
  the hooks and lists no native `technical-communication` skill. If Claude Code
  lists the root `SKILL.md` as a skill, repair the layout before the switch.
- PersonalSkills offers a Claude Code plugin that holds only `guardrail`.
- Grove's keg carries the marketplace file `writing-hooks`, and the personal
  keg carries the marketplace file `personal`. The delivery specification's
  [Hook adapters](codex-skill-provisioning.md#hook-adapters) states both. No
  marketplace lists a pack.

The design rejected user hooks that point into a working copy. Such hooks run
uncommitted code, and a layout change breaks them without a report. The Pi
extension link shows that failure. A snapshot path changes with each keg
version, so a hook that names a snapshot breaks at each advance. A keg's marketplace file and
its plugin come from one keg. A layout change in a later keg therefore moves
both together.

### Name collisions and stale copies

After the switch, each owner-selected name has one active copy, which Ravel
supplies.

- The switch deactivates both Codex user roots' copies of the Grove snapshot
  entries together.
- The switch deactivates the stale `grove` copy and the older superpowers
  copies. Before then, one staging trial measures coexistence with a stale
  native copy.
- Guidance names such as `superpowers:brainstorming` or
  `linkuistics:decision-records` stay name requests. Ravel's loading rule finds
  them by their bare names, so no alias table exists.
- `codebase-memory`, which its tool owns, and `using-codebase-memory` have
  different names and overlapping triggers. Claude Code disables the tool copy
  through `skillOverrides`, and Codex shows both. The switch keeps both states.
- The remaining same-name pairs are platform skills in separate harnesses,
  such as `skill-creator` and `pdf`. No Ravel source holds those names.

### Resource lifetime

Ravel snapshots are immutable and retained. Each keg version has one snapshot
for each source. An advance installs a new keg and changes no snapshot. A
running session keeps reading the snapshot of its launch.

Deactivation keeps every native byte. Claude Code's plugin caches stay where the
platform keeps them. Grove's Codex snapshots stay under `~/.agents/.grove`. A
moved link keeps its target.

A Claude Code session reads absolute plugin-cache paths, so disabling a plugin
leaves its reads intact. A Codex or Pi session reads through link paths, and
the switch moves those links. Each such session must end before the switch.

Claude Code's cleanup rule is time-based, not reader-based. An update or an
uninstall writes `.orphaned_at` into the earlier version directory. A background
cleanup erases that directory 14 days later and ignores readers. Disabling a
plugin is neither an update nor an uninstall, so it writes no marker. A running
session also keeps the plugin set that it loaded until a reload.

The cutover therefore keeps each version that a live reader uses registered
and current. Automatic updates of the earlier marketplace stay frozen from the
preparation step onward. Nothing updates or uninstalls a superseded plugin
before purge. The official marketplace updates automatically by default, so
this rule covers every plugin that finish loaded, not only Grove's. A backup
copy elsewhere does not protect an orphaned absolute path.

After finish, the retired listing names no plugin, so no later installation restores a version
directory that is gone. The frozen registration is therefore the whole native
return, as [Rollback](#rollback) states.

The two skill-free plugins load in place through their `opt` paths. Claude
Code also keeps a cache copy of each, and no session reads it. Homebrew's
cleanup of an earlier keg therefore breaks no plugin path.

### Settings isolation and the staging HOME

All trials before finish run in a staging HOME. It holds its own Ravel
configuration and settings with the ten sources and the mandatory file. Its
harness configuration is a prototype of the native residue: the skill-free
plugins are enabled, and no superseded skill is installed. Its Claude Code
registers `writing-hooks` from the candidate keg and `personal` from the
installed personal keg. The staging HOME copies no credential.

The staging settings name each of the ten sources in `sources`, because
Ravel's default skill directory is that of the real Homebrew prefix. Explicit
`sources` replace the default. They name Grove's six sources in the candidate
keg and the four other sources in their installed kegs. They hold the mandatory
file of the drafted real settings, which name no source. The staging HOME and
the candidate keg stay until the switch writes its receipt, because step 4 of
the switch can need them.

Each harness reuses only its sign-in store from the real HOME. Sign-in and
harness configuration have different owners, so no link may join them.

- **Claude Code** on macOS finds its sign-in in the login keychain through
  `HOME`. The staging HOME links `Library/Keychains` only. Its `.claude`
  directory and `.claude.json` belong to the staging HOME. This holds only for
  the default locations. `CLAUDE_CONFIG_DIR` moves the configuration, the
  plugins and the Keychain entry of the sign-in.
  `CLAUDE_SECURESTORAGE_CONFIG_DIR` moves only the sign-in store.
  `CLAUDE_CODE_PLUGIN_CACHE_DIR`, `CLAUDE_CODE_PLUGIN_DIRS` and
  `CLAUDE_CODE_PLUGIN_SEED_DIR` add plugin locations. A staging launch inherits
  none of these five variables, which Ravel's sandbox recipe also refuses.
- **Codex** keeps its configuration, hooks, agents, plugins, skills and sign-in
  file in one `.codex` tree. Ravel's smoke recipe links that whole tree. That
  link exposes the real configuration, hooks, plugins and superseded skills,
  and a staging configuration write would change the real files. The staging
  HOME therefore holds its own `.codex` tree, and each staging launch sets
  `CODEX_HOME` to it. Only the sign-in file `auth.json` comes from the real
  HOME, through a link to that one file.

The first staging trial proves this isolation before any other trial runs:

1. List every link in the staging HOME. The list holds `Library/Keychains` and
   `.codex/auth.json` only.
2. Show that `$CODEX_HOME/skills` and the staging `~/.agents/skills` hold no
   superseded entry, and that neither harness lists a superseded skill.
3. Record the effective configuration directory of each harness and every
   `-c` override of the staging launches. No inherited `CODEX_HOME` may point
   into the real HOME. Show that a staging launch refuses each of the five
   Claude Code variables above when the environment holds it.
4. Write the staging harness configuration and run one session on each
   harness. Digest the real configuration files, hook files, plugin registries
   and skill roots before and after. No digest may change.
5. Run one Codex session that refreshes its token. Then show that the real
   HOME's `codex login status` still succeeds and that `auth.json` is still a
   link.

If a refresh replaces the link with a file, the trial stops. A replaced link
could leave the real sign-in with a revoked token. The owner then chooses a
separate sign-in inside the staging `CODEX_HOME` instead of the reused file.

The real HOME's Ravel settings gain a `skills` key only inside finish, after
promotion. The native settings write the off value, and the switch writes the
switched value. By then, finish's own launch was this driver's last 1.1
settings read. Each write requires evidence that no other 1.1 driver can
launch.

### The final session and new launches

- **Until finish launches**, the native setup does not change. Preparation may
  install the skill formulae, register the personal keg's marketplace and
  install `guardrail` disabled. It may also write `config.rhai` beside
  `policy.ts`. Each of those changes leaves native discovery unchanged.
- **Finish** launches through this driver, Ravel 1.1.0 and `policy.ts` into
  Claude Code. It loads `grove-finish` and the spine from the enabled Grove
  plugin's cache path. Its lazy reads use that absolute path.
- **The switch** runs inside finish, right after promotion. Its steps 1 and
  2 list the processes.
- **The native settings** run in step 3 of the switch while no run is open,
  as [The native settings inside finish](#the-native-settings-inside-finish)
  states. They write the off value, so a 2.0 launch delivers no keg source
  before the switch changes the `skills` cell.
- **The hooks installation** runs next in step 3, as
  [The hooks installation inside finish](#the-hooks-installation-inside-finish)
  states. It installs one disabled plugin and changes no loaded plugin.
- **The rest of the switch** disables the Grove plugin but neither updates nor
  uninstalls it. The version directory keeps its registration and receives no
  orphan marker, so finish can still read its own guidance.
- **After the switch**, a session that the owner opens directly sees only the
  native residue.
- **After this driver's reap**, a new driver resolves compatible Grove and the
  promoted Ravel 2.0.0. Each of its launches reads `config.rhai`, captures the
  keg sources and receives the directories.

Finish keeps its original ending command and the full 1.1.0 keg, under the
coordinated release design.

### The native settings inside finish

Ravel 2.0 reads its default skill directory when the settings name no
`sources`. Ravel's skill discovery specification states that rule, and that
`"sources": []` turns the default off. After finish's release and promotion, a
2.0 launch without a `skills` key would therefore deliver every keg source
before the switch. The native state therefore holds the off value.

Preparation cannot write the off value, because this driver's 1.1 launches
refuse a 2.0 key. Step 3 of the switch writes it, after promotion. The program
runs the native settings only while no run is open, as
[Resuming the switch](#resuming-the-switch) states. The program must also be
rehearsed, as [The rehearsed plan](#the-rehearsed-plan) states. Each write is a
write interval, so the program lists the processes of step 1 again before it.

The native settings take these items in order:

1. Read the Ravel settings file. If it holds the off value as `skills`,
   change nothing.
2. If the file is absent, create it with the off value by an exclusive rename.
3. If the file exists without `skills`, write a new settings record. Then add
   the off value by the atomic exchange of switch step 9. The exchange keeps
   every other key. Compare the displaced content with the digest in the
   record. A difference is a stop.
4. In any other state, stop and ask the owner. Change nothing.

A settings record lies beside the backups, on the volume of the real HOME and
outside each tree that the baseline instrument sweeps. It holds the digest of
the content that item 1 read and the digest of the stage file. Each digest
covers the content, the mode, the ACL and the extended attributes, as in step 9.
The program writes the record under a temporary name. It renames the record
into place before the exchange, and no record replaces an earlier one.

The stage file waits in the record, and after the exchange its name holds the
kept copy. That copy takes part in detection like each kept copy of the
switch, as [Concurrent writers](#concurrent-writers) states. A write between
the read and the exchange flags it. A later write through a descriptor that
was open before the exchange flags it too. Each later run reports a flagged
copy in step 3, and step 8 stops while it is flagged.

`mac-check` reports a flagged copy of a settings record also before the first
backup exists. The creation of an absent file displaces nothing, so it writes
no record.

Each write is one atomic rename or exchange, and the record is complete before
the exchange. An interruption therefore leaves the key absent or the off
value, and the next run goes back to step 1 of the switch. An interruption
between the exchange and its comparison leaves a kept copy. Each later run
then compares that copy with the record. A failed write is a stop before the
switch changes a cell.

Another program can erase the `skills` key while no run is open. The native
settings cannot tell that erasure from the absent key before finish. They add
the off value again and list the value that they replaced. That is the
accepted case of [Concurrent writers](#concurrent-writers), because absence is
the earlier value of this prelude.

A `skills` key that another program sets to any other value reads as a changed
cell in step 3. A classification never accepts another earlier value for the
`skills` cell, as [Resuming the switch](#resuming-the-switch) states. The
owner restores the off value by hand instead, or erases the key for the next
run.

The prepared plan records the off value as the earlier value of the `skills`
cell. The switched value names only the mandatory file, so the default skill
directory supplies the ten sources. The inverse restores the off value by the
generic cell rule of [Rollback](#rollback). The native probe therefore still
reads `disabled`.

A finish that ends between promotion and the native settings leaves the key
absent. A 2.0 launch in the real HOME then delivers the keg sources beside the
native installations. The next run of the switch writes the off value in its
step 3, because no run is open yet.
[The recovery model](#the-recovery-model) covers the gate, the exchange and
the kept copy of the record.

### The hooks installation inside finish

Grove's keg holds the marketplace file `writing-hooks` only after Grove's
release, which runs inside finish. Step 3 of the switch therefore installs
`technical-communication-hooks` after the native settings. The same two
conditions apply: no run is open, and the program is rehearsed. The program
lists the processes of step 1 again before the installation.

The hooks installation takes these items in order:

1. If Claude Code has the plugin installed and disabled from
   `writing-hooks`, registered at `$(brew --prefix)/opt/grove`, change
   nothing.
2. If the plugin is absent, register that marketplace unless it is already
   registered at that path. Then run
   `claude plugin install technical-communication-hooks@writing-hooks`.
   Each of these two commands runs under a held link, as
   [Claude Code's own writes](#claude-codes-own-writes) states. Then disable
   the plugin.
3. If the plugin is installed and enabled from that marketplace, only
   disable the plugin.
4. In any other state, stop and ask the owner. Change nothing.

The program reads these states with `claude plugin list --json` and
`claude plugin marketplace list --json`. Neither command writes
`settings.json`, as the
[Homebrew evidence](../homebrew-skill-delivery-evidence.md#the-write-method)
measured.

The disabling reads the plugin's `enabledPlugins` cell again. If the cell
holds `true`, the program writes a settings record. It then writes `false`
by the atomic exchange of switch step 9, and compares the displaced content
with the record. A difference is a stop.

If the cell holds `false`, another program disabled the plugin first, and the
program writes nothing. `claude plugin disable` changes only this cell, as the
[Homebrew evidence](../homebrew-skill-delivery-evidence.md#what-plugin-disable-changes)
measured. The exchange therefore has the same effect, and detection covers
it.

Each command names the new marketplace or its plugin. No command refreshes or
updates `linkuistics`. No command runs `claude plugin marketplace update`
without a name, because that form refreshes every listing. The frozen
registration that holds finish's own guidance therefore stays as it was.

The prepared plan expects this installation. It records `false` as the earlier
value of the plugin's `enabledPlugins` cell. It also records the plugin's row
as an entry outside the switch set. Step 9 enables the plugin as it enables
`guardrail`.

The rehearsal installs the plugin from the candidate keg in its
disposable layouts, so the plan and the row are known before finish. The row's
digest covers Claude Code's cache copy of the plugin. That copy holds the bytes
of the `writing` source at its accepted digest.

A failed command is a stop before the switch changes a cell. An interruption
between the installation and the disabling leaves the plugin enabled while no
run is open. Item 3 of the next run disables it. The enabled value is the
switched value of the plugin's cell, which a classification never accepts. No
other route therefore reaches the native state. When another program enabled
the plugin, the run lists the value that it replaced, as for any switched
value.

The installation adds no state and no inverse. Its settings records follow
the rules of the native settings' record.
[The recovery model](#the-recovery-model) covers its place in step 3, the
held link, the disabling and the resumption of item 3.

#### Claude Code's own writes

`claude plugin marketplace add` and `claude plugin install` write
`~/.claude/settings.json`, a file of the switch set. No option keeps a
command out of that file and keeps the meaning of the installation. The
[Homebrew evidence](../homebrew-skill-delivery-evidence.md#claude-codes-writes-of-the-settings-file)
measured how Claude Code 2.1.319 writes the file:

- Each command writes a temporary file beside `settings.json` and renames it
  onto the path, once. A hard link to the earlier inode keeps the earlier
  bytes.
- On an idle machine, a command made its write from a read 30 to 38 ms
  before its rename. A write in place inside that interval lands in the
  earlier inode. A rename by another program inside that interval is lost.
- A command rewrites the file also when each value is already present.
- The new file keeps the mode. It loses each ACL and each extended attribute
  except `com.apple.provenance`.

The program therefore runs each command in these steps:

1. Write a settings record under a temporary name. It holds a held link to
   the live inode. Read that inode once, through the held link. The record
   holds the digest of that reading's content, mode, ACL and extended
   attributes. Rename the record into place. No record replaces an earlier
   one.
2. Stop if that reading has an ACL or an extended attribute other than
   `com.apple.provenance`. The command would drop it.
3. Read the plugin's state again. Run the command only if the state still
   needs it.
4. Run the command.
5. Stop unless the inode at the path differs from the held inode. A command
   that replaced nothing makes the held link prove nothing.
6. Compare the held copy with the digest in the record. A difference flags
   the copy, and the program stops.

Step 2 uses the same reading that the record's digest covers. Another
program can add an ACL or an extended attribute in place after that reading.
The attribute then lands in the held inode, and step 6 flags the held copy.
A stop that used an earlier reading would miss an attribute added before the
digest. The command would drop it, and the held copy would equal its record.

Step 3 decides from the state after the held link exists. Another program
can write a third value into the plugin's cell before the link. Step 3 then
reads that value and stops at item 4. Otherwise the command would replace
the value, and the held copy would equal its record.

The held copy then follows each rule of a kept copy in
[Concurrent writers](#concurrent-writers). Each later run reports it while it
is flagged, and step 8 stops. `mac-check` reports it too, also before the
first backup. Purge covers it.

A write in place before the command's read is in the live file and in the
held copy. It flags the copy, although no write is lost, and the owner's
merge then adds nothing. The design accepts that report.

A held link whose inode is still at the path holds no kept copy. A stop or
a crash before the command's rename leaves such a link. Step 3 of the next
run erases it before the preludes. Erasing the link loses no content,
because the path still names the inode.

**The accepted case.** Another program can replace `settings.json` by a
rename after the held link and before the command's rename. The command's
rename then replaces that program's inode, which has no other name. Each
write that the inode holds and the command's read missed is lost, and no
record reports it. Each ACL and each extended attribute of that inode,
except `com.apple.provenance`, is lost in the same way.

No mechanism outside Claude Code can keep that inode, so the design accepts
this case. Its exposure and its evidence are these:

- **The interval.** Each run of a command exposes one interval, from its
  read to its rename. No mechanism outside Claude Code bounds that interval.
  A command that the scheduler pauses keeps its interval open for the whole
  pause. On an idle machine, each command made its write from a read 30 to
  38 ms before its rename. That duration is evidence, not a bound.
- **The repeated runs.** A run of the program runs each command at most
  once, and only while the plugin is absent. No durable count limits the
  runs of a command. A run that stops after the registration, or after a
  held link and before a command's rename, leaves the plugin absent. The
  next run then runs the installation again. It runs the registration again
  unless the marketplace is registered at that path. An outside
  uninstallation, which [Concurrent writers](#concurrent-writers) accepts,
  also makes a later run install the plugin again.
- **The programs that can rename the file.** The exclusion keeps only the
  kept processes. Grove's sources and Ravel 1.1.0's sources name no
  `.claude` path. codebase-memory-mcp 0.10.6 wrote nothing in server mode,
  and its help names the commands that change its configuration. A server
  runs none of them. The finish session's own Claude Code process writes
  the user settings by the same rename, after a settings action such as
  `/model` or `/config`. Its agent waits on the program while the commands
  run, so a settings action can come only from its user.
- **The frequency.** A read-only observation found that no program changed
  the owner's `settings.json` in the 26 days before 2026-11-06.

The acceptance therefore rests on the programs that can rename the file. It
does not rest on the duration of an interval or on the number of intervals.
A timeout that ended a command before its rename would bound the interval.
The design does not use one. A command also writes its installation record,
so a timeout can end it between its writes, in a state that no measurement
has read.

Before the hooks installation, finish tells the owner to change no setting
in the finish session until the switch reports. A staging seam measures the
finish session's own process while the commands run, as the
[test seams](#test-seams) state.

The design rejected six alternatives. The evidence document holds the
measurement behind each reason:

- **The exclusion alone.** A write in place inside the interval, or a write
  through a descriptor after the rename, would then be lost with no report.
- **`--scope project` or `--scope local`.** The command then writes the
  settings of one project, and the installation record holds that project's
  path. The plugin is disabled in every other directory.
- **A private configuration directory.** With `CLAUDE_CONFIG_DIR` at a
  directory that links the real plugin store, the installation record names
  an `installPath` inside the private directory.
- **Cells that the program writes first.** A command rewrites the file also
  when each value is already present, so the interval stays.
- **A link at the path during each command.** Claude Code writes through the
  link to its target. The finish session's own process would race the
  command on that target in the same way.
- **A directory watch for other renames.** A watch cannot see each inode at
  the path, because two renames inside one wake read as one change.

### The switch inside finish

Finish runs the prepared and rehearsed switch after promotion and the verified
Grove/Ravel pair. One program runs each step that reads or changes the switch
set. Its steps are:

1. List live Claude Code, Codex, Pi, Grove and Ravel processes. Include
   desktop applications and their background servers. Also include each
   program that writes a file of the switch set, such as herdr, Orca, mise
   and `codebase-memory-mcp`. List each enabled launchd job, login item and
   schedule that can launch one of these programs. Exclude only the kept
   processes that [Concurrent writers](#concurrent-writers) names. Count
   each other descendant of the session that runs the switch as another
   process.
2. If any other listed process or job remains, stop and ask the owner. Do not
   signal another session.
3. Erase each held link whose inode is still at the path, as
   [Claude Code's own writes](#claude-codes-own-writes) states. Read the run
   record of the latest complete backup. While its run is open, run no
   prelude. Otherwise run
   [the native settings](#the-native-settings-inside-finish), then
   [the hooks installation](#the-hooks-installation-inside-finish). Then rerun
   the baseline instrument. Find the state of the switch set, as
   [Resuming the switch](#resuming-the-switch) states, and continue as its
   table states. The instrument's `UNCLASSIFIED`, `UNASSIGNED`, `UNCOVERED`
   and `NEEDS-REVIEW` rows count as changed entries. Compare the built-in
   rows, whose digest says `unverified`, with finish's own skill listing.
   Report each flagged kept copy, as
   [Concurrent writers](#concurrent-writers) states.
4. Stop unless the installed Grove is the latest published Grove release and
   its `opt` link reaches it. That is finish's release, or a later patch
   release that a repair needed. Write the drafted real Ravel settings into a
   disposable HOME, and run `ravel skills resolve --json` there with the
   promoted `ravel`. Compare the digest of each of Grove's six sources with its
   accepted digest in the committed staging evidence. For each source whose
   digest differs, rerun the staging trials that read it with the installed
   keg. They run in the retained staging HOME, under the environment rule of
   step 10. Stop if one fails. The digest of that rerun becomes the accepted
   digest of that source.
5. Run `mise -C "$HOME" bootstrap packages status --json`. Stop unless each
   skill formula is declared and installed. Stop unless each installed version
   equals its accepted version in the staging evidence. For Grove, the
   accepted version is the release that step 4 admitted.
6. Resolve the drafted real settings again in the disposable HOME of step 4.
   Those settings name no source, so the promoted `ravel` reads the real
   default skill directory. Stop unless the resolved source IDs are exactly
   the ten of [Ravel sources](#ravel-sources). Stop unless each source's
   `resourceDigest` equals its accepted digest.
7. Record the reader evidence. Name each plugin cache version directory that
   finish loaded and its registration in `installed_plugins.json`. Stop if any
   of those directories holds `.orphaned_at`.
8. Stop unless a passing rehearsal record names the prepared plan and the
   program, as [The rehearsed plan](#the-rehearsed-plan) states. Stop while a
   kept copy is flagged. Then write a new backup on the volume of the real
   HOME, outside each tree that the baseline instrument sweeps. It holds the
   plan, a copy of the program and the evidence of steps 4, 6 and 7. Write it
   under a temporary name, and rename it into place as the last action. That
   rename makes the backup complete, and it is the record "switch begun". No
   backup replaces an earlier one.
9. Apply the parts of the plan in their order, in one write interval of the
   program. The parts move each superseded file-system entry and change the
   Ravel `skills` key from the off value to its switched value. They also
   disable the superseded plugins and enable the two skill-free plugins. The entries come first, then the Claude Code settings, then
   the Ravel settings. For each part, the program does these four actions:
   1. Read the part. Each of its cells must hold its earlier value.
   2. For a file, write a stage file in the backup. It holds the live
      content with the switched cells, and the live file's mode, ACL and
      extended attributes. Record the digest of what the run read and the
      digest of the stage file.
   3. Exchange the stage file with the live file. Move an entry into the
      backup's retained tree at its original relative path. The native
      settings created the Ravel settings file, so the switch creates no
      file.
   4. Compare the displaced content with its recorded digest. For an entry,
      compare the moved entry with its recorded change identity.

   After the last part, read each cell again. Each must hold its switched
   value, and no kept copy may be flagged. A failed read, comparison or final
   reading is a conflict. On a conflict, the program applies the inverse to the
   parts that it changed. [Concurrent writers](#concurrent-writers) states
   why this detects each write by another program.
10. Verify with the rehearsed probes. They resolve the sources through the
   promoted `ravel` and the real HOME's configuration. Then they launch one
   Claude Code session and one Codex session by that route. Each probe runs
   outside every Grove workspace. Its environment holds none of Grove's loop
   control variables, which the
   [harness specification](harness-selection-and-execution.md#environment)
   lists.
11. On any failed probe, apply the inverse operation. Then verify the native
   route with the native probe of [Rollback](#rollback).
12. Run `mac-check`.
13. Read each cell again. Each must hold its switched value, and each must
   hold the value that it held before the probes. A changed cell counts
   as a failed probe. A flagged kept copy is a stop. Then add the receipt to
   the run record, with the result of `mac-check` and the cell values that
   this step read.
14. Compare each entry outside the switch set with the manifest of the
   backup's plan. A changed entry counts as a failed probe, and the run does
   not copy the receipt. Otherwise copy the receipt into the maintained
   manual. The entry says that the receipt describes the layout only while
   `mac-check` reports the switch complete.

Steps 1 to 8 change no cell of the plan, except the preludes and an inverse
that step 3 resumes. The preludes run only while no run is open, and each
leaves its cells at their earlier values or stops. A stop in steps 1 to 8
therefore leaves native delivery working with the published Grove and the
promoted Ravel.

A complete inverse of step 9 or step 11 restores the native layout, and the
native probe verifies it. An inverse can also stop part of the way. A session
can also end inside any of steps 8 to 14. In each case the next run finds the
state from the layout and the run record, as
[Resuming the switch](#resuming-the-switch) states.

When a step stops or an inverse runs, finish continues as follows. A failed
write of the receipt is a stop.

1. Find the state of the switch set again, as step 3 does.
2. Write a pending-activation record into the maintained manual. Name the
   published Grove version and its release commit, and the promoted Ravel path
   and version. Add the installed formula versions, the output of
   `mise -C "$HOME" bootstrap packages status --json` and the output of the
   failed step or probe.
3. In the record, name each backup and the state of the switch set. For a
   drifted or changed state, list each cell with its value. Also list each
   changed entry outside the switch set and each flagged kept copy.
4. Report the failure, and ask the owner to repair its cause.
5. Run the switch again from step 1. Its step 3 runs the preludes only while
   no run is open.
6. If the session must end before the switch succeeds, end it with the stop
   command of its launch prompt. Never send the success signal.

After such an end, or after finish ends without a result, the owner runs the
same prepared switch from step 1, outside any Grove session. Its step 3 runs
the preludes while no run is open, so the owner runs no separate prelude.

That run tears down and promotes nothing, and it releases nothing except a
patch release that a repair needs. One Claude Code session runs it and takes
finish's place in steps 1, 3 and 7. A continuation of an open backup runs the
program copy in that backup. The receipt replaces the pending-activation
record.

A failed trial in step 4, or a failed probe of a Grove kind skill, can need a
change to the released core skills. The owner then publishes a Grove patch
release through `task release:patch`. Step 4 of the next run reads that
release. No repair cuts or promotes Ravel again.

Each probe must observe these results:

- The mandatory file is read before the task's first action.
- A craft skill is read before the action it governs, although the task does
  not name it.
- A Grove kind skill that the task names is read from the `grove-core`
  snapshot. The kind is not `finish`, and the task asks only for one quoted
  line.
- `using-testanyware` is read, with a command reference that it names, such as
  the output of `testanyware llm-instructions`. The skill has no supporting
  file. `testanyware doctor` runs.
- On Claude Code, the technical-communication hooks run.
- Native discovery lists no superseded skill.
- The resolution gives each source its accepted `resourceDigest`.

After the probes launch, finish reads one of its own guidance files again by
its recorded cache path. A failed read counts as a failed probe. The receipt
records the result of `mac-check`. A failure there does not reverse a switch
whose probes passed. Finish reports it, and the owner repairs it under the
machine contract.

The receipt records the activated default path: the Grove version, the Ravel
path and version, and the digests of `config.rhai` and the settings. It also
records each source's keg path, formula version and snapshot digest, and each
trial that step 4 ran again. It records the
digests of the plan and the program, the cell values of step 13 and the
reader evidence. It names each backup location and each retained byte set
with its purge condition. The
[finish decision](../adr/finish-promotes-after-the-grove-release.md) records
why this runs inside finish.

### Concurrent writers

No file-system call compares a file and replaces it in one step. Two
mechanisms therefore protect the switch set. Detection protects each write by
another program. The exclusion protects readers, captures and progress.

**Cells.** The switch reads and changes cells only. A write to another key of
a shared file is not a conflict, because the exchange keeps that key. The
switch changes a cell only from its earlier value. The inverse changes a cell
only from its switched value, except for the entry case that
[Rollback](#rollback) states. Each run lists every value that it replaces.

**Detection.** Each changed file is published by an atomic exchange. One call
puts the stage file at the path and keeps the displaced content under the
stage name. On macOS, that call is `renamex_np` with `RENAME_SWAP`. The
displaced content becomes a kept copy.

- Before the exchange, the run records the digest of what it read. That
  digest covers the content, the mode, the ACL and the extended attributes.
  The run also records the digest of the stage file.
- Until its merge, a kept copy is flagged when it holds neither recorded
  content. A write before the exchange is in the displaced content, so it
  flags the copy.
- A writer can hold a descriptor that it opened before the exchange. Its
  later write lands in the kept copy and flags it then.
- A write after the exchange reaches the live file and stays there.
- Each run reports every flagged kept copy in step 3, at the final reading of
  step 9 and in step 13. That covers the copies of each backup and of each
  settings record. `mac-check` reports them too, also before the first
  backup.
- The owner merges a flagged copy into the live file. The merge applies the
  difference between the copy and its last recorded content. An erased key
  is part of that difference. The owner then records the merge with the
  digest that the copy holds. From then on, that digest is the only recorded
  content of the copy. Any other content flags the copy again, also a return
  to the content that the run read or to the stage content.
- No run erases a kept copy before purge. A copy that holds its stage
  content was never exchanged, and it holds no outside write.
- The exchange gives the path the stage file's metadata. The stage file
  therefore takes the live file's mode, ACL and extended attributes.
- An exclusive rename moves an entry and overwrites nothing. A changed
  identity after the move is a conflict, and the inverse moves the entry back.
  The inverse moves an entry only to an absent original path, as
  [Rollback](#rollback) states.
- The switch creates no file. The native settings create an absent Ravel
  settings file by an exclusive rename, while no run is open. The rename
  fails if another program created the file first, and that failure is a
  stop.
- The native settings exchange an existing Ravel settings file only after a
  settings record holds the two digests, as
  [The native settings inside finish](#the-native-settings-inside-finish)
  states. Their kept copy follows each rule above.
- Each Claude Code command of the hooks installation runs under a held link,
  as [Claude Code's own writes](#claude-codes-own-writes) states. A write in
  place inside the command's interval lands in the held copy and flags it.
  So does a later write through a descriptor. The disabling uses the
  exchange.

One case stays outside detection. Another program can write a cell's earlier
or switched value, and a run cannot tell that write from its own. The switch
can then replace an earlier value, or a resumed inverse a switched value.
The run's list reports each replaced value. The design accepts this case.

For a prelude cell, absence is the prelude's earlier value. A prelude can
therefore replace an erased `skills` key or an uninstalled hooks plugin, and
its list reports that value too.

A second case stays outside detection. Another program's rename during a
Claude Code command can lose a write, as
[Claude Code's own writes](#claude-codes-own-writes) states. The design
accepts that case on its evidence. Nothing bounds the duration of a
command's interval or the number of its runs.

[The recovery model](#the-recovery-model) finds no other lost
write. It lets outside programs write at every step, with no exclusion.

**Exclusion.** The switch still needs an interval without other actors:

- Codex and Pi sessions read through the link paths that the switch moves.
- A session that launches inside a write interval sees neither delivery.
- Grove 22.8 rewrites its Codex links at each Codex launch.
- A running writer makes a conflict likely, and a conflict stops the switch.

A write interval is one run of step 9, of one inverse or of one prelude. The
program lists the processes of step 1 before each interval. A remaining process or job
keeps the interval closed.

The exclusion keeps only the processes that the session cannot end without
ending itself. These are the **kept processes**:

- The harness process of the session that runs the switch.
- The driver chain that launched that session. Inside finish, that chain is
  this driver and its Ravel 1.1.0 process.
- The MCP servers of that harness process.
- The program and its shell.

Each other descendant of the session counts as another process. Examples are
a delegate harness session, a background command, a probe session and a
Ravel or Grove command. Before the session runs the program, it ends each
delegate and background task of its own. That rule includes a delegate
inside the harness process, which no process listing shows. One program
command then runs each interval, so no agent action, harness hook or probe
runs inside it.

Detection covers the writes of each kept process, except in the accepted
case of [Claude Code's own writes](#claude-codes-own-writes). Exclusion does
not protect
a kept process that reads. The harness process reads its own guidance by the
cache path that step 7 records. A kept MCP server that read a moved entry
would lose that read. The design accepts this case, because the session
cannot end its servers. The reader control of the [test seams](#test-seams)
tests the exclusion apart from detection.

The design rejected three alternatives:

- **An exemption by staging evidence.** A descendant of finish that wrote no
  switch-set file in staging was cleared. A descendant that never ran there
  passed by its absence.
- **A swap back of displaced content.** A second exchange could restore an
  outside write at once. A crash between its two renames can leave the write
  in a copy that records it as stage content. No run would then report it.
- **A comparison of whole shared files.** An owner edit or a tool's hook
  update would make a file drifted. `configure-grove` also edits the Ravel
  settings.

### Resuming the switch

Step 3 of each run reads the run record first. While the latest complete
backup's run is open, it runs no prelude. The layout then holds each prelude
cell at its earlier, switched or third value, and the table's row decides. A
prelude would stop at a switched value or replace a third value, and the open
run would never reach its row.

Otherwise step 3 runs the preludes before it finds the state. With no open
run, the table reads only the native or the changed state. A prelude stops
only at a value that also makes the state changed, and that row stops too.

Step 3 of each run finds the state from three inputs: the layout, the run
record of the latest complete backup and a plan. It keeps an incomplete
backup aside and reads nothing from it. It never reads the pending-activation
record.

The run record holds four records, each a file that the program creates by
an exclusive rename:

- "Switch begun" is the completion of the backup in step 8.
- "Receipt" follows step 13.
- "Inverse begun" precedes the first change of an inverse.
- "Inverse ended" follows an inverse after which no cell holds its switched
  value.

A backup's run is open until its last record is "inverse ended". While the
latest complete backup's run is open, its plan applies. Otherwise the
prepared plan applies. In every state, step 3 also compares each entry
outside the switch set with the manifest of the plan that applies. The table
states what a changed entry does. Step 14 compares the entries again before
it copies a receipt.

A part is switched when each of its cells holds its switched value. It is
earlier when each holds its earlier value. Any other value of a cell is a
third value. Each part is atomic, and the inverse takes the parts in reverse
order. An interruption therefore leaves the switched parts as a prefix of the
plan's order.

| State | Run record and layout | Next steps |
| --- | --- | --- |
| Complete | An open run whose last record is "receipt", and each cell switched | If the maintained manual lacks the receipt, run step 14. A changed entry outside the switch set then counts as a failed probe. Change nothing else |
| Active | An open run whose last record is "switch begun", and each cell switched | Run step 6, the probes of step 10 and steps 12 to 14. A stop in step 6 or a changed entry outside the switch set counts as a failed probe. The receipt uses the evidence in the backup |
| Interrupted | An open run whose last record is "switch begun" or "inverse begun", the switched parts a prefix and the other parts earlier, the active state excepted | Add "inverse begun" if it is not the last record. Resume the inverse. Then find the state again |
| Drifted | An open run with a third value or with switched parts that are not a prefix. Also an open run whose last record is "receipt" while a cell is not switched | Change nothing. Stop and report each cell with its value, and each changed entry outside the switch set |
| Native | No open run, and each cell and entry matches the prepared plan | If a backup exists, run the native probe of [Rollback](#rollback), and stop if it fails. Then go on with step 4 |
| Changed | No open run, and a cell or entry differs from the prepared plan | Stop for classification |

Exactly one row holds for each layout and run record. The recovery model
shows it.

The layout decides the state. A receipt only records the switched layout that
the probes saw. "Complete" also needs that layout now, so an outside write
after the receipt changes the state without any withdrawal record. No receipt
therefore certifies a layout that is not switched.

The run record tells an interruption from an outside write. Only an open run
can read as interrupted, and only a prefix pattern. One ambiguity remains.
While a run is open, another program can give a cell the value that an
interruption would leave. The resumed inverse then resets that cell, and its
list reports it.

From a drifted state, the owner runs the inverse of
[Rollback](#rollback) or restores a cell by hand. The inverse resets each
switched cell and leaves each third value, except for the entry case of
[Rollback](#rollback). After it, no run is open.

A changed state stops for the owner's classification. The owner prepares a
new plan, which records each accepted value as an earlier value. A cell never
accepts its switched value. A prelude cell never accepts another earlier
value either: the next run's prelude, or the owner by hand, restores that
value first. The new plan needs its own rehearsal, as
[The rehearsed plan](#the-rehearsed-plan) states. In finish, a classification
is a stop.

A prepared plan that differs from an open run's plan is reported. The owner
applies it only after the inverse of the open run.

In the active state, a failed probe leads to the inverse of step 11. Release,
promotion and teardown are outside each of these continuations.

### The rehearsed plan

The real HOME receives only a rehearsed switch. Each rehearsal of order 6 runs
one program on one plan in disposable layouts. Its record names the digests
of the plan and of the program. It also records the result of each control
of the [test seams](#test-seams). Step 8 stops unless a passing record names
the prepared plan and the program.

A backup keeps a copy of the program that wrote it. Each later continuation
of that backup runs that copy: the active continuation, a resumed inverse and
the owner's inverse. A changed program therefore applies only to a new
backup.

The program runs a prelude only when a passing rehearsal record names that
program. A prelude applies no plan, so this rule asks for no plan's record.

A changed plan or a changed program needs a new rehearsal. The design keeps
no reuse by kind of change, because a record of digests cannot decide such a
reuse. A rehearsal is a reproducible task in disposable layouts.

The program is `scripts/owner_switch.py`. Its docstring states the plan's
format and the rehearsal record that step 8 reads. `task
owner:switch:rehearse` rehearses it on a fixture plan.

The same rule binds finish and the owner's run after finish. Inside finish, a
plan without a passing record ends finish without success, because the
[finish decision](../adr/finish-promotes-after-the-grove-release.md) lets
finish run only a rehearsed operation. The owner rehearses the plan after
finish and then runs the switch from step 1. A rehearsal runs outside the
real HOME and changes no backup. The native probe tests native delivery only,
so it never admits a plan.

### Rollback

Inside finish, rollback is the inverse of the switch. The program runs it in
its own write interval, as [Concurrent writers](#concurrent-writers) states:

- It adds "inverse begun" to the run record before it changes a cell.
- It takes the parts in reverse order. After a conflict or a failed probe,
  it inverts only the parts that its run changed.
- In each file, it sets each cell that holds its switched value back to its
  earlier value and keeps every other key. It publishes the file by the
  atomic exchange and compares the displaced content.
- It leaves each cell that holds a third value, and it reports that cell.
- The Ravel `skills` key is a cell like the others. The inverse sets it from
  its switched value back to the off value, so Ravel 2.0 delivers no source.
- It moves each entry at its retained path back by an exclusive rename, if
  the original path is absent. It moves an entry of any change identity.
  That entry is the only third value that the inverse changes. A rename
  keeps the entry whole, so the move loses no write. A changed identity is
  then a third value at the original path.
- An entry at the original path is a third value of that cell, also while
  the retained entry exists. The inverse leaves both entries and reports the
  cell. The owner then keeps one of the two entries at the original path
  and moves the other outside native discovery. An entry that stays in the
  backup stays there until purge.
- Another program can create the original path after the inverse reads it.
  The rename then fails and changes neither path. The inverse stops as at a
  conflict, and the next run reads the occupied path as a third value.
- A conflict stops the inverse and leaves the run record as it is. The next
  run resumes the inverse. A rename that fails for another cause, such as a
  missing parent directory, is a conflict too. Each later run stops at it
  until the owner repairs that cause.
- It adds "inverse ended" only when no cell holds its switched value.

The native probe verifies the result on both harnesses. It observes these
results:

- `ravel skills resolve --json` with the promoted `ravel` reports `disabled`,
  because the settings hold the off value.
- Native discovery on each harness lists the superseded skills again.
- One session on each harness, launched as in step 10, reads a Grove kind
  skill natively. The kind is not `finish`, and the task asks only for one
  quoted line.
- Inside finish, finish reads one of its own guidance files again by its
  recorded cache path.

After finish, the owner can apply the same inverse while the backup and the
plugin caches exist. The owner runs the program copy of the latest backup,
which lists the processes of step 1 first. The owner also adds a line to the
maintained manual.

The inverse is the only return to native delivery. It needs the latest backup
and the plugin caches of the frozen `linkuistics` registration. After finish,
no remote marketplace lists a pack, so no later installation restores native
skills. After purge, no native return exists. The owner then repairs the Ravel
route: the formulae, the settings or Ravel itself. A session that its user
opens directly can still read the folders that `ravel skills resolve` lists.

No rollback erases Ravel snapshots. Rollback keeps the skill formulae, their
mise declarations and the two skill-free plugins, installed and disabled. No
harness discovers skills in a keg.

### The recovery model

The Quint model [`owner-switch.qnt`](../models/owner-switch.qnt) covers the
switch, the inverse, interruptions, outside writes, classifications and
receipts. It also covers each run's entry through steps 1 to 3, with the gate
and both preludes. The hooks installation's command reads the file and later
renames a new file onto the path, under a held link.

`task model:owner-switch` runs each of its claims. Its worlds let outside
programs set cells, add and erase other keys and write into kept copies at
every step. A write to the Claude Code settings file lands in
place or by a rename. The worlds also let a program change an entry outside
the switch set, and they let a crash end a run at every step. No world has an
exclusion.

In six design worlds and five mutant worlds, the first state comes before the
first entry's preludes, with no `skills` key and no hooks plugin. In the
others, the first state is the layout that those preludes leave. Each entry in each world still passes the
gate and the preludes.

Six properties hold in each of the fourteen design worlds:

1. No write by another program is lost without a report. The accepted case
   is a cell write of the cell's earlier or switched value. For a prelude
   cell, absence is also an earlier value. The second accepted case follows
   another program's rename during a Claude Code command. The command's
   rename can then destroy a write, and the model accepts that loss.
2. Exactly one row of the state table holds. A run reads an interruption only
   while a run is open, and a settled state only while none is.
3. No receipt records a cell that is not switched. Each receipt records the
   cells that the probes saw, and "complete" needs each cell switched now.
4. No exchange applies a plan that has no rehearsal record.
5. No run copies a receipt into the maintained manual with a changed entry
   outside the switch set.
6. Each entry reaches the row of the table that holds. A prelude refuses a
   value only where that row stops, and never a value that a prelude itself
   left. While no run is open, an interrupted prelude can leave a value in a
   prelude cell. No entry reads the table with such a value.

Each of thirteen mutant worlds lacks one mechanism, and it reaches the violation
that the mechanism prevents. The same violation stays unreached in each
design world.

| Missing mechanism | Violated property |
| --- | --- |
| The recorded digest of the displaced content | 1 |
| The run record in the state | 2 |
| "Inverse ended" only after the inverse | 2 |
| The cell reading before the receipt | 3 |
| The rehearsal condition of step 8 | 4 |
| The inverse's rule that leaves a third value | 1 |
| The merge digest as the only recorded content of a merged copy | 1 |
| The comparison of step 14 | 5 |
| The gate of the preludes in step 3 | 6 |
| The settings record before the exchange of the native settings | 1 |
| The hooks installation in every entry | 6 |
| The disabling of an enabled plugin by the hooks installation | 6 |
| The held link before a Claude Code command | 1 |

The results are sampled, not proved. Each world ran at least 4,000 samples of
up to 60 steps. Before the preludes, a switch of two prelude cells and one
other part is 26 steps away from its receipt. Seven focused worlds
narrow when outside writes or crashes act:

- One reaches drift after a receipt, and the owner's inverse from it.
- One reaches a merged copy that returns to older content.
- One reaches the complete continuation after a crash between the receipt
  and step 14.
- One reaches the active continuation after a crash while the probes run.
- One reaches conflicts at the exchanges of the switch.
- One reaches the native settings' exchange, its gap before the comparison
  and a late write into the kept copy of its settings record.
- One reaches a write in place inside a command's interval and a late write
  into the held copy. It also reaches the accepted case and a crash between
  the held link and the rename.

The model file lists what the model does not establish. Metadata, the
program copy, the probes and readers belong to the rehearsal controls. So do
the registration command, the evidence of the accepted case and the creation
of an absent settings file.

**Entry parts.** The model treats each part as a file. Two rules settle an
entry part without a model:

- The program moves an entry only by an exclusive rename, and only to an
  absent path. No move overwrites an entry, and the program erases none.
- The cell of an entry part is the pair of its original and retained paths.
  Its earlier value is the entry with its recorded change identity at the
  original path, and nothing at the retained path. Its switched value is
  the reverse. Each other pair is a third value.
- The retained path lies in the backup of the open run. While no run is
  open, the cell is the original path alone. Its earlier value is then the
  entry that the plan records there.

Properties 1 to 4 then hold for entries:

1. No write is lost. An entry that another program creates stays at its
   path, because no rename overwrites it. A write into a moved entry moves
   with it and changes its change identity. Each later reading then
   reports a third value. The inverse moves a changed entry back. It leaves
   an entry whose original path is occupied. Purge requires each retained
   entry to keep its recorded change identity.
2. Each pair has exactly one value, so the state table applies unchanged.
   The entries come first in the plan, and each rename is atomic. An
   interruption therefore still leaves a prefix of switched parts.
3. A receipt reads each pair as it reads each cell.
4. No rename runs outside a rehearsed plan.

An occupied original path cannot make the inverse fail again and again. The
inverse renames only onto an absent original path. If another program
occupies that path first, the rename fails and changes neither path. The
next run reads the occupied path as a third value and leaves it. A rename
that fails for another cause stops each run until the owner repairs that
cause, as at any stopped step.

### Purge

Purge covers each backup's retained tree and kept copies, and the kept copy
of each settings record. It also covers Grove's Codex snapshot store, the
superseded Claude Code plugin registrations and the earlier marketplace
registration. Purge requires five conditions:

- No session that ran during the switch remains.
- This driver's group is reaped.
- No kept copy is flagged.
- Each retained entry keeps the change identity that its move recorded.
- The owner decides that the rollback window is closed.

Purge also ends the native return, as [Rollback](#rollback) states.

Purge is outside this grove's success condition, as the 1.1.0 keg cleanup is.
The receipt lists each item with its condition. The maintained manual carries
the procedure. The owner uninstalls platform registrations only through the
platform's own commands. Nobody erases a cache directory by hand. A Claude Code
uninstall marks the version orphaned, and the platform erases it 14 days later.

### Independent sessions

After the switch, a session that the owner opens directly with `claude`,
`codex` or `pi` sees only the native residue. It can receive the same
directories through the output of `ravel skills resolve`. It can also read the
keg sources by path. An advance then links the default directory to another
keg under that session. The root
requirements leave automatic configuration of such sessions outside scope.
This is the main daily consequence of the owner's chosen target.

## Requirements for the Ravel directory contract

The owner design depends on each item below. `ravel-rhai-skill-delivery-k34`
settled them, and the local-source contract `7a894c78` keeps them. Items 1, 2
and 5 describe local sources, after Ravel withdrew Git sources.

1. Admit at least ten sources, each with a narrow source directory. The bound
   is 16.
2. Admit a mandatory file in a third-party source, with relative supporting
   reads in the same source.
3. Expose the descriptor path and the snapshot roots in the request. The
   configuration script can then grant reads to a harness that needs explicit
   directories.
4. Preserve executable bits in snapshots. The technical-communication checker
   runs from its skill directory.
5. Resolve every source with no network and no Git executable.
6. Add a loading rule for user-invoked skills. Tell the agent not to load a
   skill whose metadata sets `disable-model-invocation: true` unless the caller
   names it. `authoring-conventions` is the owner's case.
7. State exactly how Ravel 1.1 treats a settings file that holds 2.0 keys. The
   switch's timing assumes that 1.1 refuses such a file.
8. Document a sandbox-home technique for actual harness trials that links
   only each harness's sign-in store. For Codex, it sets a separate
   `CODEX_HOME` and reuses only the sign-in file, never the whole `.codex`
   tree. It lists every linked path. The existing smoke recipe links the whole
   tree. That recipe does not satisfy this item, and k34 decides whether the
   smoke task adopts the narrower route.
9. Read the default skill directory when the settings name no `sources`, and
   deliver nothing with `"sources": []`. `ravel-default-skill-directory-k218`
   settled this item in the section "Default skill directory" of Ravel's
   specification.

## Cutover contract

`owner-ravel-skill-inventory-k38` grows the concrete leaves. Rollout orders them
with the coordinated release work.

| Order | Owner | Step | Done when |
| --- | --- | --- | --- |
| 1 | This design | Baseline, classification and contract | The baseline and this design are committed and reviewed |
| 2 | Repository moves | Move every owner-pack and selected third-party entry with its resources. Supply the three packaging changes. Publish each pack commit, record the five domain packs in Grove's record and publish this workstream's line. Publish the formulae of personal, superpowers and quint-llm-kit, and TestAnyware's release with its skill. | Each moved skill's content and resources match the baseline or a recorded change. Each source has a published commit, and each separate formula is in the tap |
| 3 | k38 | Re-inventory, compare per entry, classify each new entry | No entry stays unassigned |
| 4 | k38 leaves | Convert `policy.ts` to `config.rhai`. Draft the Ravel skill settings: the off value, the switched value with only the mandatory file, and the staging settings. Draft the `[bootstrap.packages]` declarations and the residue plan. | The drafts are committed. No draft launches a session before the candidate keg of order 5 exists |
| 5 | k38 leaves | Run the staging trials on both harnesses with the accepted feature build | The drafts pass under the staging HOME. Each trial below passes or carries a recorded scope decision. The evidence names each accepted formula version, the candidate commit and each accepted `resourceDigest` |
| 6 | k38 leaves | Prepare the real HOME without changing discovery. Declare the skill formulae in `[bootstrap.packages]` and install the four that published. Register the personal keg's marketplace and install `guardrail` disabled. Add the drift report to `mac-check`. Freeze automatic updates of the earlier marketplace. Rehearse the program on the plan in disposable layouts. The rehearsal covers the native settings, the hooks installation, the switch, the probes, the inverse and the controls of the test seams. | `mac-check` reports no drift for the four formulae. After each control, each cell holds its earlier or switched value, or the run reports it. Each outside write stays in the live file or in a flagged kept copy. After each interruption, the next run finds the state and reaches the native or the complete state. The rehearsal record names the plan, the program and the result of each control |
| 7 | Finish | After promotion, run the switch. Its step 3 writes the native settings and installs the writing hooks plugin | The probes pass and the receipt is in the manual |
| 8 | Owner | Purge retained bytes | The purge conditions hold |

The staging trials run three steps first, in this order:

1. Build the candidate keg from an unreleased archive of the candidate commit,
   with `task owner:candidate-keg`.
2. Write the staging settings with the paths of the candidate keg and of the
   four installed kegs.
3. Run the isolation trial of
   [Settings isolation and the staging HOME](#settings-isolation-and-the-staging-home).

Neither of the first two steps launches a harness, so the isolation trial is
still the first trial that launches a session. The trials use the accepted
feature build. After the Ravel cut, rollout resolves the sources again with the
published archives, as step 5 of the
[coordinated release design](coordinated-release.md) states.

The other trials cover implicit craft discovery and
the mandatory read order. They cover a Grove kind skill that the task names, a
read of the TestAnyware command reference and real VM tool use. They also
cover the Quint and codebase-memory MCP servers, the Claude Code hooks and
`/guardrail`. Each harness also runs one delegate, one compaction and one
stale-copy coexistence case.

One trial moves a source of the candidate keg away and observes
`skill_source_missing` before any worker. One trial launches without a network and observes the same
directories. A trial fails if native discovery lists a superseded skill.

One Claude Code trial disables the plugin that holds its own guidance and
launches a nested probe. It then reads its guidance by absolute path again.

Persistent setting changes follow the machine configuration contract. mise
tracks `~/.claude/settings.json`, `~/.codex/config.toml`, `~/.codex/hooks.json`
and `~/.config/ravel` in place. The `[bootstrap.packages]` declarations are in
the tracked `~/.config/mise/config.toml`. The kegs, the skill directories and
the plugin registry are not tracked files. The maintained manual is the
Obsidian `AI Dev Tooling` note. It still names the earlier
`~/Development/grove` checkout.

## Test seams

- The baseline instrument, rerun, compares entries one by one by their change
  identity. Its control root must show one unclassified entry and one dangling
  entry. Its fixture controls must show that each class of change changes its
  entry, and that a rotated credential does not.
- Each outcome of the drift report of `mac-check` has a fixture control. The
  outcomes are a missing formula, an unreadable status and an installed
  version other than the tap's current version.
- Staging trials and rehearsals use Ravel's public `start-session` and
  `skills resolve` with the actual harnesses. They read Grove's six sources
  from the candidate keg and the four other sources from their installed kegs.
- Inside finish, the hooks installation changes no plugin that finish loaded
  and writes no orphan marker. A staging measurement shows it. It digests each
  plugin version directory that a running session loaded, and each
  registration, before and after the installation's commands.
- Controls stop a run of the switch between the native settings and the hooks
  installation, and between the installation and the disabling. The next run
  from step 1 must install or disable the plugin and reach the native state.
- A control writes another key of `settings.json` in place between a held
  link and its command's rename. The program must stop with a flagged held
  copy, and the live file must lack the key. Each later run must report the
  copy, and step 8 must stop. Another control writes through a descriptor
  that it opened before the rename, after the comparison. Each later run and
  `mac-check` must report that copy, also before the first backup.
- A control stops the program between a held link and its command's rename.
  The next run from step 1 must erase the link, install the plugin and reach
  the native state.
- A control writes a third value into the plugin's cell before the held
  link. The hooks installation must stop at item 4 and run no command.
- A control gives `settings.json` an extended attribute other than
  `com.apple.provenance`, and another gives it an ACL. Each must stop the
  hooks installation before its first command. A stub command that writes
  the file in place must stop the program at its comparison of inodes.
- Controls add an extended attribute, and an ACL, in place just before a
  held link. Each must stop the program before its command. Other controls
  add them in place after the reading that the record's digest covers and
  before the command's rename. Each must stop the program with a flagged
  held copy, and each later run must report that copy.
- A staging measurement runs the hooks installation while a signed-in Claude
  Code session with the owner's MCP servers is open in the staging HOME. It
  watches `settings.json` during each command. Any replacement by another
  process counts against the evidence of the accepted case, and the owner
  decides the case again.
- The rehearsal reruns
  [`settings-writes.sh`](../evaluations/hooks-installation-writes/settings-writes.sh)
  with the Claude Code version that it uses, and its record names that
  version. One rename per command and an interval within the evidence's
  range must hold, or the design is reviewed again. The rerun reads the
  write method of that version. It bounds no interval of a later command in
  the real HOME.
- A control stops the program after the registration and before the
  installation's held link. The next run from step 1 must run no
  registration, install the plugin and reach the native state. The
  rehearsal record counts the commands that each run ran.
- Controls stop the native settings before and after each write, between the
  settings record and the exchange, and between the exchange and its
  comparison. They run with the settings file absent and with a file that
  holds another key. The next run from step 1 must find the key absent or the
  off value, and reach the native state.
- A write of another key between the native settings' read and their exchange
  must stop the native settings with a flagged kept copy. Another control
  makes the same write and stops the run inside the gap before the
  comparison. The next run must report the flagged copy, and step 8 must stop.
- A descriptor opened before the exchange of the native settings writes after
  the comparison. Each later run must report the flagged copy, and step 8
  must stop. `mac-check` must report the copy before the first backup exists.
- Two controls stop the switch at the crash points of an open run. One stops
  it after the last exchange of step 9 and before the receipt. The other stops
  it after the receipt and before step 14. The next run from step 1 must run
  no prelude, and it must reach the active or the complete continuation.
- A control gives the `skills` cell a third value before the switch. Step 3
  must find the changed state. After the owner restores the off value, the
  next run must find the native state. Another control erases the `skills`
  key after an inverse. The next run must add the off value, list the
  replaced value and find the native state.
- With the off value and installed keg sources, `ravel skills resolve --json`
  must report `disabled`. A control copy of the default skill directory with
  an eleventh source must stop step 6.
- `task model:owner-switch` runs the recovery model. Each claim must
  hold, each witness must be reached, and each mutant must reach its
  violation.
- The switch and its inverse have a dry run against the plan. Rehearsals use
  disposable layouts, and each rehearsal record names the plan, the program
  and the result of each control.
- Interruption controls stop the program before and after each part, between
  each exchange and its comparison, beside each record and inside the probes.
  The next run must find the interrupted, active or native state, and it must
  keep an incomplete backup aside.
- Fault points write or erase an unrelated key just before and just after
  each exchange. They act in step 9, in the inverse and in the disabling of
  the hooks installation. A write before an exchange must leave a flagged
  kept copy and stop the run. A write after it must stay in
  the live file. A descriptor opened before an exchange must flag the kept
  copy when it writes.
- A control merges a flagged copy and then changes that copy. The next run
  must flag it again. Another control merges a copy and then returns it,
  through a descriptor, to the content that the run read. The next run must
  flag that copy too. Step 8 and step 13 must stop while a copy is flagged.
- A control changes the mode and an extended attribute of a file between the
  read and the exchange. The comparison must find it. The published file must
  keep the live file's metadata.
- Cell controls give a switched cell a third value, before and after the
  receipt. The next run must find the drifted state and change nothing. The
  owner's inverse must reset the other cells and leave that cell.
- A control writes a cell's switched value while a run is open. The resumed
  inverse must reset it and list it. Another writes it after "inverse ended",
  and the next run must find the changed state.
- Controls change the content of a moved entry and retarget a moved link.
  Each runs before the switch and after it. Before the switch, the run must
  find the changed state. After it, the run must find the drifted state, and
  the owner's inverse must move the entry back.
- A control puts a new entry at the original path of a moved entry. The run
  must find the drifted state. The owner's inverse must leave both entries,
  report that cell and reset the other cells. The next run must find the
  changed state. Another control creates the original path after the
  inverse reads it. The rename must fail and change neither path, and the
  next run must find the drifted state.
- One control stops a run between steps 13 and 14, and the next run must copy
  the receipt. Two controls add an entry outside the switch set before step
  14: one inside a run, and one after such a stop. Each run must count the
  entry as a failed probe and apply the inverse. It must not copy the
  receipt. Another changes a cell after the receipt. `mac-check` must
  report the drifted state, and no run may copy the receipt into the manual.
- A changed plan or a changed program without a passing rehearsal record must
  stop step 8. A continuation must run the program copy of its backup.
- A writer that is not a kept process, or a job that can launch one, must
  stop the run at step 2. The same writer, launched later, must stop the
  next write interval before its first change.
- A reader control launches a delegate harness session and a background
  command from the session that runs the switch. Each reads a path that the
  plan moves, and neither writes. Each must stop the run at step 2. Each,
  launched later, must keep the next write interval closed.
- A control keg source that differs from its accepted source by one byte must
  stop step 6. In a control copy of Grove's keg, the same difference must make
  step 4 rerun the trials of that source.
- Only finish runs the switch against the real HOME, or the owner after a
  finish that ended without success.
- After the switch, the real-HOME probes, `mac-check` and the AI tooling status
  report read the result.

## Out of scope

- Automatic configuration of independently opened sessions. A native pointer
  skill that runs `ravel skills resolve` would reintroduce a native install.
  Reopen this item if the owner wants such sessions configured automatically.
- Installing Codex technical-communication hooks, which the owner never
  enabled.
- A replacement route for Pi.
- Harmonising the duplicate codebase-memory copies in Codex.
- Uninstalling disabled third-party plugins.
- Erasing platform-managed caches by hand.
- Automatic advances, such as a scheduled `brew upgrade`. Reopen this item if
  the owner wants a source to follow its branch.
- Installation routes other than Homebrew. Other users install the same
  formulae.
