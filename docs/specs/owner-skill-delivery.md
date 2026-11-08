# Owner skill delivery through Ravel

This design moves the owner's installed skill collection from native harness
installations to Ravel's directory delivery. Homebrew installs the ten sources;
native registrations retain capabilities that a skill read cannot supply.

The owner is the only user of Grove and Ravel, and this is the only running
grove. The switch therefore uses an owner checklist, a narrow process check,
one backup, ordinary writes and one restore command. It requires an interval
without other writers. It does not preserve concurrent edits or resume a
partly applied switch. [Concurrent writers](#concurrent-writers) states that
trade-off and the obligations that remain.

The [evidence document](../owner-skill-delivery-evidence.md) records the
measurements and their limits. Historical recovery experiments there describe
the earlier design, not the present contract. The
[baseline](../owner-skill-inventory.tsv) classifies installed entries.

Ravel's directory contract owns the feature itself. The
[coordinated release design](coordinated-release.md) owns publication order,
promotion and recovery. The [Grove delivery design](codex-skill-provisioning.md)
owns Grove's behaviour and the general cleanup procedure for other users.
[Homebrew installs skills and Ravel delivers them](../adr/homebrew-installs-skills-and-ravel-delivers-them.md)
records the installation decision. This document owns the owner's source set,
native residue and ordered cutover on this machine.

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

### Four simplifications

| Concern | Retained contract and removal | Why the owner's facts permit the change |
| --- | --- | --- |
| Processes and jobs | [The owner checklist](#the-owners-checklist) replaces broad program, job and login-item enumeration; the narrow harness/driver check remains. | One owner controls the accounts and launches involved. That owner can pause their tools and relaunch mechanisms; sole ownership alone does not establish that they are paused. |
| Writes | [Serial writes](#concurrent-writers), one backup and final comparison replace exchange, kept-copy detection, held links and merge. | No other writer is required during this operation. Finish waits and the owner makes no settings action. Overwritten concurrent edits are deliberately outside the guarantee. |
| Recovery | [Full restore](#rollback) replaces partial inverses, state classification, backup program copies and pending activation. | The only executor can restore the whole native layout before retrying; there is no concurrent actor's change to merge. Crashes still require retained data and explicit restore. |
| Dry run | [One disposable execution](#the-rehearsed-plan) and content-identified inputs replace digest-bound admission records. | The same executor controls the prepared tools and reviews their identities before the real attempt. Changed inputs need another dry run; a stored token adds no observation. |

### Terms

- **Deactivation** takes an entry out of native discovery and keeps its bytes
  in the backup. For a plugin, it disables the plugin without updating or
  uninstalling it. Its registered cache directory stays in place.
- **Purge** erases retained bytes after their readers end and the owner closes
  the rollback window. Deactivation precedes purge.
- **The switch** activates directory delivery in the real HOME and deactivates
  every superseded native entry.
- **The switch set** comprises the complete settings and registration files,
  and the file-system entries, that the switch changes. The plan lists them.
- **The plan** is the prepared description of those changes, the expected
  native and switched layouts, accepted source versions, commands and probes.
- **The program** runs the plan or restores its backup. It exposes no merge,
  partial inverse or continuation entry.
- **The backup** is one complete, verified copy of every item the attempt can
  change, including its absence and metadata. It precedes the first write.
- **The native return** restores the backup's native layout, with Ravel's
  explicit off value after promotion.
- **Native residue** is the set of native capabilities that stays active.
- **A keg source** is one source directory installed at
  `share/ravel/skills/<source-id>` and linked into Ravel's default directory.
- **An accepted version** is a formula version whose sources passed staging,
  with an accepted `resourceDigest` for each source.
- **The candidate keg** is Grove's pre-release keg in a private prefix, built
  by the production formula's install method.
- **The off value** is
  `"skills": {"schemaVersion": 1, "sources": []}`. Ravel 2.0 then delivers
  no source even while the kegs are installed.

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
  the ten. No other formula installs a source there. The switch
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
its source verification requires.

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
captures nothing from them. After promotion, the backed-up switch writes the
off value, as
[The native settings inside finish](#the-native-settings-inside-finish) states.
Source verification resolves in a disposable HOME. The switch captures
nothing from the real settings until it writes the switched value. An unpublished edit reaches a launch only through
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

After the switch, each owner-selected name has one active copy. Ravel supplies
the guidance; native-only `guardrail` keeps its sole capability plugin.

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

Deactivation keeps every native byte in the backup or its registered cache.
Claude Code's plugin caches stay where the platform keeps them. Grove's Codex
snapshots stay under `~/.agents/.grove`. A backed-up link preserves its target.

A Claude Code session reads absolute plugin-cache paths, so disabling a plugin
leaves its reads intact. A Codex or Pi session reads through link paths, and
the switch deactivates those links. Each such session must end before the switch.

Claude Code 2.1.319 uses both age and reader evidence for orphan cleanup.
An update or an uninstall writes `.orphaned_at` into the earlier version
directory. Cleanup considers that directory after the marker is older than
14 days. Before deletion, it reads `.in_use/<pid>` to identify live sessions.
It keeps a directory with a live reader marker. It also keeps the directory
when reading a marker or querying liveness reports an error.

<!-- tc: depart synonym-rotation because "check" names Claude Code's liveness operation and "verify" names a cutover action -->
The [executable evidence](../owner-skill-delivery-evidence.md#finding-for-the-owner-design)
establishes this behaviour for 2.1.319 only. The official loading reference
does not document the reader check.

Disabling a plugin is neither an update nor an uninstall, so it writes no
orphan marker. A running session also keeps the plugin set that it loaded
until a reload.

The reader check changes no cutover rule. Codex and Pi write no such marker,
and another Claude Code version can change the check. A missing marker
therefore proves nothing about those readers. The design keeps these four
rules for reasons independent of that check:

| Rule retained | Reason |
| --- | --- |
| Freeze automatic marketplace updates from preparation until purge | The freeze preserves the accepted plugin bytes and registrations through the switch and the native rollback window. |
| Keep each version that a live reader uses registered and current | This avoids orphaning the absolute read path and preserves the native return. A backup elsewhere cannot preserve that path. |
| Record and recheck the reader evidence | Named paths, registrations and the absence of orphan markers establish the retained resources. Reader markers cannot establish all three. |
| Keep the purge conditions | Ended sessions, the driver's reap, unchanged retained resources and the owner's closed rollback window govern every retained resource. |

Nothing updates or uninstalls a superseded plugin before purge. The official
marketplace updates automatically by default, so the freeze covers every
plugin that finish loaded, not only Grove's.

After finish, the retired listing names no plugin, so no later installation restores a version
directory that is gone. The frozen registration is therefore the whole native
return, as [Rollback](#rollback) states.

The writing hooks plugin and native guardrail plugin load in place through
their `opt` paths. Claude Code also keeps a cache copy of each, and no session
reads it. Homebrew's cleanup of an earlier keg therefore breaks no plugin path.

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
the candidate keg stay until the switch writes its receipt, because source
verification can need them.

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

Until finish launches, preparation leaves native discovery active. It may
install formulae, prepare `config.rhai`, register the personal marketplace and
install `guardrail` disabled. It cannot write the Ravel off value: remaining
Ravel 1.1 launches refuse that key.

Finish launches through this driver's original Ravel 1.1.0 and `policy.ts`
into Claude Code. It loads the finish skill and spine from their absolute
plugin-cache paths. After compatible Grove publication, installed verification
and Ravel promotion, finish runs the switch. The
[finish decision](../adr/finish-promotes-after-the-grove-release.md) owns that
placement and its single confirmation.

The switch keeps finish's loaded versions registered and current, disables
superseded plugins, and updates or uninstalls none of them. Finish rereads its
guidance after the probes. After this driver's reap, a new driver resolves the
compatible pair and delivers the keg sources through `config.rhai`.

Keep the full original 1.1.0 keg and every inherited ending command through
the final signal. Keep Homebrew cleanup protection through this driver's reap,
as the coordinated release design requires.

### The owner's checklist

The owner establishes a quiet interval before the program runs and keeps it
through its probes, receipt or restore. Sole ownership makes this an owner
operation; it does not prove the machine is already quiet.

The checklist names these obligations:

1. End other Claude Code, Codex and Pi sessions, their desktop applications and
   background servers, and any other Grove or Ravel invocation. End finish's
   delegates and background work, including delegates hosted inside the same
   harness process. No new session starts except a probe owned by the program.
2. Pause `mise dot watch`, herdr, Orca configuration writers,
   codebase-memory installers and updaters, and each other tool that can
   rewrite the plan's files or regenerate native skills. Do not run
   `mac-bootstrap`, `mise bootstrap`, plugin installation or update,
   Homebrew upgrades, or configuration synchronisation during the interval.
3. Prevent relaunch by the owner's login items, launch agents and schedules.
   Include the Codex app-server daemon, `herdr-core` and
   `dev.mise.mise-history` where installed. Inspect the account's Login
   Items, its LaunchAgents and scheduler configuration. A read-only
   `launchctl print gui/<uid>/<label>` can identify a loaded user job's
   arguments. Do this in the real account; changing HOME does not isolate
   launchd. The program enumerates no jobs and grants no privilege.
4. Leave finish's own Claude Code settings unchanged: no `/model`,
   `/config`, plugin action or other user settings action until the program
   returns. Finish waits on that one program and invokes no mutating MCP
   operation. MCP server processes may stay only when their server mode does
   not write the switch set. Staging measures that assumption. Stop or
   reconfigure a server that does write it before proceeding.
5. Keep automatic marketplace updates frozen for every plugin finish loaded.
   Keep the tested Claude Code version until the switch. Confirm its version
   before the dry run and real attempt. Keep the old driver keg protected.
6. Review the prepared plan, accepted staging results and dry-run result.
   Confirm that the machine still has the planned native layout and that the
   background tools remain paused.

The finish handoff gives the owner the current app, job and schedule names and
how to pause and restore each through its owning interface. These are checklist
entries, not process-pattern exemptions. The owner can inspect account-owned
items without privileges. If an item cannot be inspected or stopped, or an
unknown writer appears, the checklist is incomplete and the switch waits.
No automatic review of job labels or login items substitutes for that fact.

A harmless child such as Claude Code's `caffeinate` needs no exception. The
program checks only other Claude Code, Codex, Grove and Ravel processes.
It identifies finish's exact harness and driver ancestors by their live
ancestry, together with its own process. It excludes no other matching
descendant. A delegate harness or app-server therefore stops the attempt.
It repeats this check before the first write and before restore. During
probes, only the program's recorded probe processes may appear; the program
waits for their exit before its final check. Pi and non-harness writers remain
checklist responsibilities. The program signals none of them.

### The native settings inside finish

After promotion, an absent `skills` key enables Ravel's default sources.
The native return must therefore use the off value. Before the first real
write, the backup records both the original settings and the native return
value. The program then adds the off value by an ordinary write, preserving
the other keys. An existing off value is acceptable. Any other pre-existing
`skills` value stops the attempt before a write.

This is a deliberate exception to byte-for-byte restoration: when the backup
records no `skills` key, restore adds the off value. Every other backed-up
setting returns to its recorded value. Both the dry run and the real backup
make the exception visible. Restoring the absent key after promotion would
enable Ravel sources beside the restored native skills.

A stop after promotion but before the program starts can also leave default
delivery enabled beside native skills. Until the native return or successful
switch is verified, launch no new default session. Run the prepared operation
through the still-running finish session, or the owner's recovery entry.
Do not run Ravel 1.1 again with these shared 2.0 settings.

### The hooks installation inside finish

The backup also precedes the hooks installation. It covers the Claude Code
settings, plugin installation registry, marketplace registry, and any other
configuration file that the tested commands change. The plan enumerates those
writes. It does not back up and replace whole plugin caches or sign-in stores.

The program registers only `writing-hooks` from Grove's released `opt` path
and installs `technical-communication-hooks@writing-hooks` if absent. It
then writes the plugin's disabled setting. An already installed plugin is
acceptable only from that expected marketplace and origin. The personal keg
already supplies `guardrail` installed and disabled. The switch later
enables both plugins while it disables superseded plugins.

No command updates or uninstalls a loaded plugin, refreshes `linkuistics`,
or refreshes all marketplaces. Keep the frozen versions, registrations and
absolute guidance paths. Compare them before and after the installation and
probes. An orphan marker, missing registration or unreadable guidance fails
the switch.

Claude Code's commands can write and replace settings. Run them serially
while finish waits. After each command, read the settings again. Compute the
next write from that current file, preserving keys that the command added.
Restore and stop on a failed command or unexpected configuration change.

#### Claude Code's own writes

The existing measurement found that Claude Code's install commands replace
`settings.json` and can drop ACLs and extended attributes. Sole ownership
does not remove that behaviour. The simplified design removes held links and
inode checks because the checklist excludes other settings writers.

Before a command, refuse metadata that the tested command would lose: an ACL
or an extended attribute other than `com.apple.provenance`. The backup still
preserves all original metadata. Ordinary program writes preserve that
metadata. The dry run tests the command's resulting files, plugin origin,
settings and metadata using the version held for the real attempt.

A signed-in staging session with the owner's MCP servers must remain idle
through the installation without an unsolicited settings write or change
to a loaded registration. If that experiment fails, this design's quiet
interval assumption is false: resolve the writer before cutover. An unchanged
final file alone does not prove that no intermediate write occurred.

### The switch inside finish

The program exposes three operations: a read-only plan report, the switch,
and `restore --backup <directory>`. A new attempt uses a new backup directory.
No operation selects a continuation from a run-state table.

The switch takes these steps in order:

1. Review the completed checklist. Validate the complete plan and command
   inputs. Refuse unexpected native entries, ambiguous classifications,
   incomplete accepted results, or paths that escape the intended HOME.
   Run the narrow process check. No write to the switch set has occurred.
2. Verify the installed compatible Grove release, promoted Ravel, formula
   declarations and accepted versions. Resolve the drafted switched settings
   in a disposable HOME. Require exactly the ten accepted source IDs and
   their `resourceDigest` values. If a released Grove source differs,
   rerun its affected staging trials in the retained staging HOME and accept
   the new result before proceeding. Its HOME and `CODEX_HOME` are its own.
   An absent staging HOME or candidate keg stops the attempt.
3. Record finish's loaded cache paths, registrations and current guidance
   reads. Finish supplies paths that a program cannot infer from a process.
   Compare unverified built-in inventory rows with finish's own skill list.
4. Create and verify one complete backup, outside all discovery and inventory
   roots and on the volume of HOME. Include every planned settings and
   registration file, each native entry to deactivate, original absences,
   mode, ACL and extended attributes, the plan and command-input identities,
   accepted source evidence and reader evidence. Copy entries without following
   their links into external targets. Verify the copy against the originals
   before publishing the backup directory. An incomplete backup authorizes
   no mutation. Recheck the narrow process check and the planned originals.
5. Write the native off value, then install the disabled writing hooks plugin.
   The preceding sections specify both actions. On any failure after the
   backup, run restore and stop.
6. Deactivate superseded file-system entries using the verified backup to
   retain their bytes. Disable superseded plugins and enable the two native
   capability plugins. Write the switched Ravel settings last: they name the
   mandatory file and omit `sources`, so the default directory supplies the
   ten sources. Use ordinary writes and reread each result. Keep unrelated
   keys from the latest serial reading. No atomic exchange, held link or
   displaced-copy protocol is used.
7. Run the real-HOME source and harness probes. Resolve through promoted
   Ravel; launch one Claude Code session and one Codex session outside every
   Grove workspace. Remove Grove loop-control variables, Claude Code location
   variables, `CODEX_HOME` and every `RAVEL_*` name from their environment.
   The same environment rule applies to plan commands; staging trials use
   their explicitly separate HOME and Codex configuration. Wait for all
   probes to end. On failure or interruption, stop and reap only children
   the program launched before restore can write. Never signal another
   session. If an owned writer cannot be reaped, retain the backup and
   report recovery incomplete until it ends.
8. Run `mac-check`. Compare the complete expected switched configuration,
   registrations, native-entry inventory, source digests and reader evidence
   with the plan and the probes' results. Reread finish's guidance by its
   recorded path. An unexpected switch result, outside entry or metadata
   change is failure and restores the native return. An unrelated
   `mac-check` area is recorded and reported under the machine contract;
   a failure of its switch area prevents success. For example, an unrelated
   backup-disk review does not invalidate the measured skill delivery.
9. Write the successful receipt and copy it into the maintained manual.
   Verify both copies and make one final comparison before returning success.
   The manual entry says the receipt records an observation and stays valid
   only while the present layout agrees. A failed receipt or manual write
   restores the native return and withdraws any success entry created by this
   attempt. Include that manual destination in the backup's write set.

The probes must observe the mandatory read before the first action, an unnamed
craft skill before its relevant action, and a named Grove kind skill from the
core snapshot. Use a kind other than finish and ask for one quoted line.
They read `using-testanyware`, run its command reference and doctor, observe
Claude Code's hooks, and list no superseded native skill. Resolution must give
all accepted source digests. The probes also establish that finish's loaded
paths remain registered, readable and without orphan markers.

The receipt names the activated Grove and Ravel versions and paths, the
configuration digests, each source's keg path, formula version and snapshot
digest, accepted reruns, command-input identities, probe results, final
comparison and `mac-check` output. It lists the backup, staging HOME,
candidate keg, frozen cache versions and old binaries with their purge
conditions. No pending-activation record or copy of the program in each
backup is required.

### Concurrent writers

The program assumes the owner has stopped other writers for the entire
operation, including restore. Ordinary writes and whole-file restore may
overwrite an intervening edit. A final comparison detects a different final
layout; it cannot detect an edit that was overwritten or later returned to
the same bytes. This is an accepted loss of the earlier concurrent-write
guarantee, justified by the owner's chosen quiet interval, not by a claim that
one user has no background processes.

The checklist owns non-harness writers, jobs, login items, Pi, settings actions
inside finish and in-process delegates. The narrow process check owns other
Claude Code, Codex, Grove and Ravel processes. The serial program owns its
commands, probes and final comparisons. A detected violation restores the
program's changes and stops; it does not attempt a merge.

Two inventory rules remain independent of concurrency:

- Compare each ordinary child of `~/.claude/skills` and
  `~/.codex/skills`, before and after probes. Skip only the exact
  platform-managed subtrees `~/.claude/skills/synced` and
  `~/.codex/skills/.system`. Do not skip their parent roots. Also compare
  `~/.agents/skills`, Pi entries and the other baseline roots. A new
  superseded sibling, nested copy or link is unexpected and fails acceptance.
  Probe changes inside the two excluded subtrees alone do not fail it.
- Accept enabled `guardrail@personal` only from the personal keg's
  registered marketplace, containing only the native guardrail skill.
  It is a classified native capability. An additional skill or wrong origin
  fails inventory. Other ambiguous or unclassified entries still stop.

Credential refreshes and known platform reader markers follow the baseline
instrument's existing exclusions. Every exclusion names its exact subtree or
field; none suppresses ordinary siblings. No new blanket exclusion is allowed
because a probe writes an incidental file.

The design compared three arrangements. Keeping the old exchange and merge
protocol preserves more concurrent edits but requires records, copies and
recovery branches for writers the owner can stop. A handwritten cutover removes
the program but spreads backup, restore and verification obligations across
finish's commands. The chosen serial program keeps those obligations together
and accepts the quiet-interval requirement. Reopen that choice if another grove
or user shares the configuration, or a required writer cannot be paused.

### Resuming the switch

There is no automatic resumption. A caught failure after the first write runs
restore, verifies native delivery and exits unsuccessfully. An interruption
before the backup is complete changed no switch item. An interruption after
the backup is complete requires the same restore command before another
attempt; never continue the partly switched layout or infer success from a
prefix of completed writes.

A killed program, power loss or terminated finish cannot run cleanup while it
is dead. The owner retains the backup location and the prepared restore command
before mutation. After restart, the owner first re-establishes the checklist,
runs restore, and verifies the native return. If restore itself was interrupted,
run that same restore again. It replaces from the unchanged backup, so repetition
has the same target. No run log or program copy selects a recovery route.

A restore failure keeps the backup and reports the failed paths. It admits no
new switch and no success signal. Repair the I/O or permissions problem, then
run restore again. This covers an interrupted hook registration as well as
partial settings writes and native-entry deactivation.

Finish reports an unsuccessful attempt and its backup path. It never sends a
success signal for incomplete activation. It may retry after the cause is
repaired and the native return is verified. If it must end, use its original
force-stop command. After an unsuccessful finish, the owner runs only the
prepared restore or a fresh switch; teardown, releases and promotion are not
repeated. A failed released-core trial can require a Grove patch through its
owning release procedure. Ravel is neither cut nor promoted again.

### The rehearsed plan

Preparation runs the real plan once against a disposable copy of the relevant
HOME layout. This is an execution of the ordinary writes, hook commands,
probes, final comparison and restore, with isolated configuration and explicit
trial tools. A read-only listing is not the dry run. Copy no credentials;
reuse sign-in only through the staging contract. The dry run must not contact
the real account's launchd, stop its jobs or change its configuration.

Keep a readable result with the expected changes, probe results, restore
comparison and limitations. Identify the plan, program, each invoked script,
local imported helper and command input by its content digest. Resolve script
paths to the files actually executed. Also record interpreter, harness, Ravel
and external tool paths and versions. Record the substituted prefixes and
environment. A fixture or stub result is marked as such and supplies no
real-harness acceptance.

This resolves unbound command files without recreating rehearsal admission.
There is no rehearsal registry, durable admission token, program check for a
matching rehearsal digest, or program archive in every backup. The owner
reviews the readable result and keeps its prepared inputs unchanged. A changed
plan, program, helper, command input or tool version needs another dry run.
Record the identities before and after the dry run so its input did not change
during measurement. Finish compares these inputs with the real attempt's
prepared inputs; a mismatch requires a new dry run. This is a review of actual
inputs, not a program lookup of a passing record.

The prepared restore program stays available with the plan until the rollback
window closes. A software repair must retain compatibility with existing
backup data or preserve that restore entry. The backup contains restoration
data and its format, not a runnable copy of the program. Retain the evidence
document, plan and prepared tools across finish's task-tree teardown.

### Rollback

One `restore --backup <directory>` command serves caught failure, crash recovery
and the owner's later return to native delivery. It validates the complete
backup, its content identities and confined destination paths before writing.
It restores whole backed-up
files, metadata and entries, including original absence, rather than inferring
which parts completed. It removes only files and success entries that this
attempt created and that the plan names. Unrelated entries outside the write
set are reported and left alone.

Restore first writes directory delivery's off value using the backed-up
settings, then restores native registrations, settings and entries, and
finishes by enforcing the native return's Ravel settings. It never needs to
parse a partly written live settings file. It does not run a plugin uninstall
or delete a cache. Newly installed cache bytes may stay unregistered until
purge; every configuration mutation still returns to the native layout.
It verifies each restored item against the backup and runs the native probe.
That probe requires promoted Ravel to report `disabled`, native discovery
and a native Grove kind-skill read on both harnesses, and finish's recorded
guidance to remain readable when finish still runs. Any unexpected outside
entry is reported; failed verification keeps recovery incomplete.

The original settings file may have been absent. Even then, restore leaves a
Ravel settings file holding the off value. This is the only restoration
exception. No restore reverses the published releases, promotion, formula
installations, mise declarations or immutable Ravel snapshots.

A later owner restore requires the same quiet interval, unchanged backup,
frozen native registrations and retained cache paths. After purge, that native
return is unavailable. Repair the Ravel delivery route instead.

### The recovery model

The old Quint model describes cells, exchanges, outside writes and partial
continuations that this design removes. Its success is not evidence for the
simplified switch. Remove it and its task and claim-specific controls with the
implementation change; preserve the earlier measurements as historical evidence.

The current contract is checked at the program's existing disposable-HOME seam:
backup completeness before mutation, the full switch, failures and hard
interruptions at write boundaries, repeatable restore, native probes and final
layout comparisons. These are filesystem and command effects which the earlier
abstract model did not establish. The design claims no lossless concurrent
writes and no automatic crash recovery, so it needs no replacement recovery
state machine or model.

### Purge

Purge is outside this grove's success condition. It requires ended readers,
this driver's reap, unchanged retained resources and the owner's closed rollback
window. Keep automatic updates frozen until the retained native return is no
longer needed. Keep the full old Ravel keg protected until its last driver ends.

The receipt lists the backup, native cache versions and registrations, Grove's
Codex snapshot store, staging HOME and candidate keg, each with its condition.
A small read-only purge report can compare those retained identities and list
the conditions that only the owner can establish. It needs no kept-copy flags,
merge state or open-run classification.

Use the platform's commands to remove its registrations. Never erase plugin
caches by hand. The version-specific orphan-cleanup observation in
[Resource lifetime](#resource-lifetime) changes none of these conditions.

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
| 6 | Preparation | Prepare installed sources and native residue, freeze updates, complete the owner checklist and dry-run the real plan in a disposable copy of HOME | Accepted inputs are identified by content; switch and full restore pass without changing the real HOME |
| 7 | Finish | After promotion, back up before any write, switch, probe and compare; restore on failure | Both harnesses pass, finish keeps its guidance and the receipt is in the manual |
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

The existing program entry point with disposable HOME layouts remains the switch
seam. Real staging and cutover probes remain the acceptance seam for harness
behaviour. No fixture proves a real installation or activation.

- Inventory controls introduce an unclassified entry, a dangling entry and a
  changed resource; each must be reported. Credential rotation and documented
  reader markers must not change a skill's identity.
- Add a superseded skill beside each excluded platform subtree, at both native
  roots, and through a link. Each must fail the before and after comparison.
  A change confined to `synced` or `.system` must pass. Native guardrail
  from its sole-skill personal plugin must pass; an extra skill or wrong
  origin must fail.
- A fixture of another Claude Code, Codex, Grove or Ravel process must stop
  the attempt. Exact finish ancestors are allowed; another descendant
  harness and a background app-server are not. A harmless `caffeinate`
  child needs no exemption. Record which checklist obligations a fixture
  cannot establish.
- Run the full real plan once in the disposable copy, including hook
  commands, probes and restore. Change one invoked helper without changing
  its path; the recorded content identity must differ. Check the complete
  command-input list, input stability, substitutions and tool versions.
  No rehearsal registry may be required by the real program.
- Stop backup creation before publication. No switch item may have changed.
  Verify content, link text, absence, mode, ACL and extended attributes in
  the complete backup; fail a copy or metadata read and require no mutation.
- Inject caught failures before and after each real write, during hook
  registration and installation, during deactivation, probes, comparisons,
  receipt and manual writes. Each must restore the native return and exit
  unsuccessfully. Kill the program at those boundaries too; the next action
  must be explicit restore, not forward continuation.
- Interrupt restore and repeat the same command. Its final layout must equal
  the backed-up native layout with only the Ravel off-value exception. Test
  initially absent settings and an existing settings file with unrelated
  keys. Simulated permission or I/O failure must preserve the backup, name
  the failed paths and prevent success.
- Test Claude Code command failure and an unexpected file change. Metadata
  that its commands lose must stop before the command. Ordinary writes and
  restore preserve backed-up metadata. No held-link or inode-change
  condition is required.
- Measure hook installation beside an idle signed-in Claude Code session
  with the owner's MCP servers. Observe settings writes during the commands;
  compare loaded plugin versions and registrations before and after, ignoring
  only documented reader markers. An unsolicited settings write, changed
  loaded registration or orphan marker fails the quiet-interval assumption.
  Record the tested and held Claude Code version.
- Source controls require off settings to resolve as `disabled`, the
  switched settings to resolve exactly ten sources, and an eleventh source
  or one-byte digest change to stop acceptance. A changed released Grove
  source requires its staging reruns; a missing retained staging HOME stops
  them. Trial commands must see its isolated HOME and Codex tree.
- Real staging proves configuration isolation before discovery trials,
  mandatory and implicit reads, relative resources, delegates, compaction,
  stale-copy coexistence, missing-source refusal, offline resolution, hooks,
  native guardrail, TestAnyware VM use, Quint and codebase-memory MCP use.
  A failed case requires repair or an explicit owner scope decision.
- Both real-HOME probes use the promoted pair outside Grove workspaces with
  location and loop-control variables removed. Wrong configuration selection,
  missing native exclusion, wrong source digests, unreadable finish guidance,
  or a lost loaded registration must fail and restore native delivery.
- Change a switched setting, add an outside native entry or alter a
  registration during probes. The final comparison must fail and no success
  receipt may remain. A final comparison cannot detect overwritten
  concurrent edits; the design makes no such claim.
- `mac-check` reports native delivery, a currently verified switched layout,
  or a layout needing restore/review. No receipt alone establishes current
  success. A partial layout or stale receipt must require review. Without
  prepared evidence it reports that the switch is not prepared, rather than
  claiming a verified native state. Formula drift retains its separate area
  and controls.
- Purge reporting lists retained resources and manual reader conditions.
  A changed retained resource or an unverified switch blocks purge. It
  interprets no old recovery state.
- Only finish, or the owner after unsuccessful finish, runs the real switch.
  The receipt and maintained manual identify actual activation, the backup
  and every retained resource. AI tooling status reads that current result.

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
