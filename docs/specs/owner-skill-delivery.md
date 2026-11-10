# Owner skill delivery through Ravel

This design moves the owner's installed skill collection from native harness
installations to Ravel's directory delivery. Homebrew installs the ten sources.
Native registrations retain capabilities that a skill read cannot supply.

The owner is the only user of Grove and Ravel, and this is the only running
grove. The switch therefore uses an owner checklist, a narrow process inspection,
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
   HOME in one reversible operation. Observe both harnesses. On failure, restore native delivery.
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
| Processes and jobs | [The owner checklist](#the-owners-checklist) replaces broad program, job and login-item enumeration. The narrow harness/driver inspection remains. | One owner controls the accounts and launches involved. That owner can pause their tools and relaunch mechanisms. Sole ownership alone does not establish that they are paused. |
| Writes | [Serial writes](#concurrent-writers), one backup and final comparison replace exchange, kept-copy detection, held links and merge. | No other writer is required during this operation. Finish waits and the owner makes no settings action. Overwritten concurrent edits are deliberately outside the guarantee. |
| Recovery | [Full restore](#rollback) replaces partial inverses, state classification, backup program copies and pending activation. | The quiet interval permits whole-file recovery during the attempt. Later edits require review before restore. Crashes still require retained data and explicit restore. |
| Dry run | [One disposable execution](#the-rehearsed-plan) and content-identified inputs replace digest-bound admission records. | The same executor controls the prepared tools and reviews their identities before the real attempt. Changed inputs need another dry run. A stored token adds no observation. |

### Terms

- **Deactivation** takes an entry out of native discovery and keeps its bytes
  in the backup. For a plugin, it disables the plugin without updating or
  uninstalling it. Its registered cache directory stays in place.
- **Purge** removes retained bytes after their readers end and the owner closes
  the rollback window. Deactivation precedes purge.
- **The switch** activates directory delivery in the real HOME and deactivates
  every superseded native entry.
- **The switch set** comprises the complete settings and registration files,
  and the file-system entries, that the switch changes. The plan lists them.
  The maintained manual is an append-only record outside whole-file restore.
- **The plan** is the prepared description of those changes, the expected
  native and switched layouts, accepted source versions, commands and probes.
- **The program** runs the plan or restores its backup. It exposes no merge,
  partial inverse or continuation entry.
- **The backup** is one complete, verified copy of every item the attempt can
  change, including its absence and metadata. It precedes the first write.
- **The native return** restores the backup's native layout, with Ravel's
  explicit off value after promotion.
- **Delivery assertions** name the settings values, registrations, native entries,
  source identities and capabilities that establish the current delivery route.
  They do not prescribe unrelated settings keys or manual content.
- **Native residue** is the set of native capabilities that stays active.
- **A keg source** is one source directory stored privately in its keg and
  exposed through the directory symlink of the
  [keg layout](codex-skill-provisioning.md#the-keg-layout).
- **An accepted version** is a full Homebrew package version, including its
  revision, accepted under [Skill formulae](#skill-formulae). Its sources
  have accepted commits and `resourceDigest` values.
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
  removes an earlier keg.
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

A formula's upstream version names a commit. For Grove and TestAnyware it is
a release tag. For the other three it is the commit of the source archive.
Homebrew adds the formula revision to form the full package version, such as
`2026-11-06_1`. A revision can change packaging while keeping the commit.

The tap publishes only accepted versions. Grove's release task and TestAnyware's
release publish theirs. The owner edits the other three formulae after their
staging trials or the packaging-only acceptance below. The tap's current full
package version is therefore the accepted version.

A packaging-only revision may retain source staging trials when its commits,
roots and source digests equal the accepted values. Its acceptance record
names the earlier trials, the revised full package version and the formula
content. The revised formula must pass the delivery specification's
[shared-directory seam](codex-skill-provisioning.md#shared-default-directory-acceptance)
and applicable [hook seam](codex-skill-provisioning.md#test-seams).
Equal source digests alone establish no acceptance of changed packaging or
hook paths. A changed source digest requires that source's staging trials.

Retaining source trials does not retain acceptance of native-only plugins.
Compare each complete native-only payload, including files, modes, links and
hook declarations, with its accepted subject. A changed payload or hook
command requires its affected capability trials. This includes `guardrail`,
which lies outside the `personal` source digest.

Homebrew installs only the tap's current version. A rebuilt machine therefore
installs the current versions, not earlier ones.

`mac-check` reads `mise -C "$HOME" bootstrap packages status --json` for each
declaration and its installation state. It reads `brew info --json=v2` for
the installed and tap package versions, including revisions. A missing
formula or unreadable status is a configuration error. A different full
package version is drift, even when the source commit is unchanged.

The switch compares full package versions too. The plan's non-Grove
`accepted.versions` and Grove's admitted release identify those package
versions. Grove's binary version alone does not identify a formula revision.
If mise omits a revision, the comparison uses Homebrew's package identity.
It must distinguish the earlier revision from the accepted revision.

The report also checks the source entries against the
[keg layout](codex-skill-provisioning.md#the-keg-layout). A current formula
version with expanded file links is an invalid source layout, not version
drift. Report the affected source and formula, with the documented upgrade
or relink recovery. A report observes the state. It repairs no links.

During this cutover, real-machine preparation upgrades the four non-Grove
formulae to accepted revisions that supply the directory symlinks. Their
acceptance records bridge the earlier source trials to the new package
versions under the packaging-only rule above.

Preparation verifies them
through the shared default directory, even while the settings keep native
delivery active. After publication, finish installs Grove's revised formula.
It then verifies the ten default sources and their owning private payloads
before the switch.

Promotion also links Ravel's shared `share/ravel`
entries. The verification follows that link operation and any recovery from
a link conflict. These checks implement the layout's migration rule.
The versioned paths that staging used do not substitute for them.

Grove's keg holds core. The delivered spine and the installed Grove therefore
come from one release, and no separate core pin exists.

During this cutover, the staging trials read Grove's six sources from the
candidate keg. Grove's real keg holds them only after Grove's release, inside
finish. The candidate is a published commit of this workstream's line that
holds the production formula template. Its `plugins/grove` and
`plugins/packs.json` are equal to those of the last commit that changes
either path. A later change to either path needs a new candidate keg and a
rerun of the trials that read the changed sources. A commit earlier than the
template change builds the earlier keg layout, and `task owner:candidate-keg`
refuses it.

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

The hooks plugin of [Native residue](#native-residue) is the one tool that
runs inside a keg. Its hooks write nothing there, so the `writing` source
keeps its accepted digest through each session. The technical-communication
specification of WritingSkills owns that rule, and its script
`scripts/hook-writes.py` enforces it.

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
captures nothing from them. Immediately after promotion, finish establishes the
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
- **The Claude Code marketplace clones.** Claude Code updates and removes them,
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
| `quint-kb` MCP server | Unchanged. It runs from the Quint clone, which stays. Its Codex entry sets `default_tools_approval_mode = "approve"`, because Ravel launches Codex with the approval policy `never`. The maintained manual records that line. |
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
the guidance. Native-only `guardrail` keeps its sole capability plugin.

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

<!-- tc: depart synonym-rotation because "inspection" names Claude Code's liveness operation and "verify" names a cutover action -->
The [executable evidence](../owner-skill-delivery-evidence.md#finding-for-the-owner-design)
establishes this behaviour for 2.1.319 only. The official loading reference
does not document the reader inspection.

Disabling a plugin is neither an update nor an uninstall, so it writes no
orphan marker. A running session also keeps the plugin set that it loaded
until a reload. The [guidance probe case](../owner-skill-delivery-evidence.md#the-guidance-probe-case)
of the staging trials observed both with Claude Code 2.1.319.

The reader inspection changes no cutover rule. Codex and Pi write no such marker,
and another Claude Code version can change the inspection. A missing marker
therefore proves nothing about those readers. The design keeps these four
rules for reasons independent of that inspection:

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

The staging HOME lies inside the real HOME, so each staging launch runs in a
working directory outside the real HOME. Claude Code reads `.claude/skills` in
every parent folder of its working directory. A working directory in the real
HOME therefore reaches the real native skills, even when it lies in the
staging HOME. The staging launcher refuses such a working directory.

The first staging trial proves this isolation before any other trial runs:

1. List every link in the staging HOME. Before any session, the list holds
   `Library/Keychains` and `.codex/auth.json` only. After the sessions, only
   these two links lead out of the staging HOME. A tool can create a link that
   resolves inside the staging HOME, such as npm's `.bin` link of an `npx`
   package. Such a link joins no owner, so the trial lists it with its owner.
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
and Ravel promotion, finish establishes native delivery before switch preflight.
It then runs the switch. The
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

The operator establishes a quiet interval before any operation and keeps it through
probes, receipt or restore. Finish is the operator while it runs. After an unsuccessful
finish, the owner uses the replacement operator contract in [Resuming the switch](#resuming-the-switch).
Sole ownership does not establish that the machine is quiet.

1. End other Claude Code, Codex and Pi sessions, their desktop applications and background servers.
   Include Claude and ChatGPT, which can host tools independently of CLI sessions.
   End any other Grove or Ravel invocation and the operator's delegates and background work.
   Include delegates inside the same harness process. Launch no session except a program-owned probe.
2. Pause `mise dot watch`, herdr, Orca configuration writers, codebase-memory installers and updaters.
   Pause each other tool that can rewrite planned files or regenerate native skills.
   Run no bootstrap, plugin update, Homebrew upgrade or configuration synchronisation during the interval.
3. Prevent relaunch by the owner's login items, launch agents and schedules.
   Include the Codex app-server daemon, `herdr-core` and `dev.mise.mise-history` where installed.
   Inspect Login Items through System Settings and record any inspection limit.
   Inspect LaunchAgents and scheduler configuration. List loaded user jobs with `launchctl list`.
   A job can exist without a LaunchAgents file. Inspect a known job's arguments with `launchctl print gui/<uid>/<label>`.
   Use the real account. Changing HOME does not isolate launchd.
4. Leave the operator's Claude Code settings unchanged until the program returns.
   Invoke no `/model`, `/config`, plugin action, mutating MCP operation or in-process delegate.
   The operator waits on one program. MCP servers may stay only when staging establishes that they leave the switch set unchanged.
   Stop or reconfigure a server that writes it before proceeding.
5. Keep marketplace updates frozen for every plugin the operator loaded.
   Keep the tested Claude Code version and the original driver keg protected.
   Compare tool versions before the dry run and real attempt.
<!-- tc: depart synonym-rotation because "check" elsewhere names a process inspection, not a second verb for verify -->
6. Review the plan, staging evidence and reproducible input comparison from the dry run.
   Verify the planned native layout and that the background tools remain paused.
7. Pause Obsidian edits and synchronisation of the maintained manual through their owning interfaces.
   Include iCloud and edits from the owner's other devices.
   Resume them only after the appended entries agree with the operation's result.

The handoff names current apps, jobs and schedules and their pause and resume
instructions. These are owner actions, not program exemptions. No exhaustive
unprivileged login-item command is assumed. The recorded `sfltool dumpbtm`
trial required authorisation. If a required item cannot be inspected or paused,
the checklist remains incomplete. After promotion, the operator first establishes
native delivery through `native-off`, then repairs preparation.

#### The narrow process inspection

The program recognises Claude Code, Codex, Grove and Ravel by executable identity
and their measured launcher forms. Its built-in recogniser covers native executables,
versioned packaged binaries, Node launchers with their actual entry-script paths,
and Codex app-server and daemon subcommands. It compares executable basenames,
canonical paths and parsed arguments. A substring in unrelated arguments is
not recognition. Owner-supplied process regexes remain removed.

Preparation records actual installed paths, launch forms and versions for those
four tools. An unknown launcher form or unreadable executable identity requires
review before mutation. This boundary does not identify every possible writer.
Pi, Claude and ChatGPT desktop applications and non-harness writers remain checklist
obligations. A harmless `caffeinate` child needs no exemption.

The program admits only its own process and the operator's exact live harness
and driver ancestors. It grants no blanket descendant exemption. Another delegate
harness or existing app-server refuses the attempt. During probes, only recorded
probe-owned processes may appear. Inspection repeats before mutation, before
restore and after probe cleanup. It signals no unrelated process.

#### Probe process lifetime

Each probe launcher must establish a tested lifecycle for its harness and services,
including Codex daemons that outlive their parent. Preparation must measure a
fresh HOME without an existing daemon, normal exit, failure and interruption.
A probe's exit alone does not establish service termination.

Use a measured launch mode that prevents persistent services.
Alternatively, use a supported shutdown route for a service whose creation the launcher proves. Ownership requires
its process identity, creation time, executable and configuration HOME, with creation
evidence from that probe. Matching a path or finding a new PID alone is insufficient.
The program retains that evidence through reparenting and verifies service exit.

Before final process inspection or restore, the program ends its owned services and verifies their exit.
It reaps its child processes. A reparented service need not remain a child. PID reuse must never authorise a signal.

An existing daemon remains unrelated and refuses the attempt. If the measured
launcher cannot establish ownership or cleanup, no real switch is admitted.
An unexpected cleanup failure retains the backup and reports recovery incomplete.
The owner resolves that service through its owning interface before restore.

### The native settings inside finish

After promotion, an absent `skills` key enables Ravel's default sources beside
native skills. Finish therefore runs the prepared `native-off` operation immediately
after promotion, before source acceptance or any new default launch. It needs
no switch backup, accepted source digest or completed switch checklist. The owner
can run the same operation after a crash or a stop before backup creation.

`native-off` is a bounded safety operation, separate from switching or full restore.
It parses the current Ravel settings and preserves every unrelated key and metadata.
An absent file becomes a settings file with the off value. An absent `skills`
key gains that value. An existing off value stays unchanged.

Any other value or
malformed file requires review without replacement. It refuses other Ravel drivers
and requires the owner to pause writers of this one settings file.

Publish this one file through a complete temporary sibling and atomic replacement.
Never truncate the live settings. Copy and verify unrelated keys and existing
metadata before replacement, and synchronise the prepared file before publication.
Measure persistence on the target filesystem. An interruption leaves the original
or the complete off value available to the same idempotent command.
This is ordinary file publication, not exchange with a displaced-copy recovery protocol.

The prepared recovery instructions name its exact executable, HOME, settings path
and command before promotion. A write failure reports incomplete native delivery
and forbids new default launches. After a crash, rerun the same operation.
Promoted `ravel skills resolve` must report `disabled`, and native discovery must
remain intact. This route requires no source capture or harness launch to write
the off value. Native harness reads then verify the prepared recovery result.

This safety operation deliberately precedes the switch's full backup. It changes
only delivery's off key, with no native-entry deactivation or hook command.
The ordinary switch still backs up its entire write set before its first write.
Its backup now normally records the off value.

Restore also enforces that value
when older backup data records an absent key. This is the only settings exception
to whole-file restoration. Do not launch Ravel 1.1 against these shared 2.0 settings.

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
`settings.json` and can drop ACLs and extended attributes. They replace
`known_marketplaces.json` and `installed_plugins.json` in the same way, so
each rule below about lost metadata covers all three files. Sole ownership
does not prevent that behaviour. The simplified design omits held links and
inode inspections because the checklist excludes other settings writers.

Before a command, refuse metadata that the tested command would lose: an ACL
or an extended attribute other than `com.apple.provenance`. The backup still
preserves all original metadata. Ordinary program writes preserve that
metadata. The dry run tests the command's resulting files, plugin origin,
settings and metadata using the version held for the real attempt.

A signed-in staging session with the owner's MCP servers must remain idle through
installation, native deactivation, capability enabling, both probes and receipt completion.
Observe writes throughout that interval, including writes that a later write overwrites.
Measure the live parent harness and servers, not only command subprocesses.
The staging parent must use that staging HOME and the held harness version.

The disposable dry run alone cannot establish this parent-harness behaviour.
No unsolicited settings write or loaded-registration change is acceptable. If that experiment fails, this design's quiet
interval assumption is false: resolve the writer before cutover. An unchanged
final file alone does not prove that no intermediate write occurred.

### The switch inside finish

The program exposes a read-only plan report, `native-off`, the switch and
`restore --backup <directory>`. No operation selects a continuation from a run-state
table. A new attempt uses a new, explicitly named backup directory.

The switch takes these steps after verified native delivery:

1. Review the completed checklist, plan and dry-run input comparison.
   Refuse unexpected native entries, ambiguous classifications, incomplete acceptance or paths outside the intended HOME.
   Run the narrow process inspection. No switch-set write occurs here.
2. Verify the compatible Grove release, promoted Ravel, formula declarations and accepted versions.
   Resolve drafted switched settings in a disposable HOME. Require the ten accepted source IDs and their `resourceDigest` values.
   If a released source differs, stop in native delivery.
   Rerun affected staging trials, revise accepted inputs and repeat the dry run before a fresh attempt.
   An absent retained staging HOME or candidate keg stops that acceptance work.
3. Record the operator's loaded cache paths, registrations and current guidance reads.
   The operator supplies paths that the program cannot infer from processes.
   Compare unverified built-in inventory rows with that session's skill list.
   Retain the attempt instructions specified in [Resuming the switch](#resuming-the-switch) before mutation.
4. Create and verify one complete backup outside discovery and inventory roots, on HOME's volume.
   Include every planned settings and registration file and each native entry to deactivate.
   Include original absences, content, mode, ACL, extended attributes and link text.
   Include plan and command-input identities, source acceptance, reader evidence and attempt identity.
   Copy links without following their targets. Verify originals against the copy before publishing the backup directory.
   An incomplete backup authorises no switch mutation. Repeat process inspection and compare the planned originals again.
5. Verify the native off value, then install the disabled writing hooks plugin.
   Run commands serially under the hooks contract. On any failure after backup publication, restore and stop.
6. Deactivate superseded entries, disable superseded plugins and enable the two native capability plugins.
   Write switched Ravel settings last. They name the mandatory file and omit `sources`.
   Use ordinary writes and reread each result. Preserve unrelated keys from the latest serial reading.
7. Run real-HOME source and harness probes through promoted Ravel outside every Grove workspace.
   Clear loop-control variables, Claude Code location variables, `CODEX_HOME` and every `RAVEL_*` name from probe environments.
   Apply the same rule to plan commands. Staging commands use their explicitly isolated HOME and Codex tree.
   End probe-owned services through the measured lifecycle contract before final inspection or restore.
   Verify service exit and reap child processes.
   Never signal another session. If cleanup fails, retain the backup and report recovery incomplete.
8. Run `mac-check` and compare complete planned switch results, inventories, digests, metadata and reader evidence.
   Reread the operator's guidance by its recorded paths. Unexpected results require restore.
   Record unrelated `mac-check` findings under the machine contract. Its switch area must pass.
   Record complete post-switch file identities separately from delivery assertions for later restore eligibility.
9. Write the receipt and append its observation to the maintained manual.
   Verify both records and compare the final layout again. A failure restores native delivery and invalidates this attempt's receipt.
   Append a correction to any manual success observation. Never restore the whole manual or silently remove other notes.

The probes observe the mandatory read before action and an unnamed craft skill before its relevant action.
They observe a named Grove kind skill from the core snapshot. They
use a kind other than finish and quote one line. They read `using-testanyware`,
run its command reference and doctor, observe Claude Code's hooks and list no
superseded native skill. Resolution gives all accepted digests. The operator's
loaded paths stay registered, readable and without orphan markers.

The receipt names activated versions and paths, source and configuration identities,
command inputs, probe results, final comparison and `mac-check` output. It also
records delivery assertions, whole-file restore identities and the exact attempt
and backup path. It lists retained sources, staging HOME, candidate keg, frozen
cache versions and old binaries with their purge conditions.
Restore identities cover restore destinations, excluding the receipt's own bytes
and the append-only manual. No receipt digest refers to itself.

The manual is append-only evidence, not a restore destination. Pause its writers
through each append and readback, including recovery entries. Preserve its metadata
and unrelated content. A failed append prevents switch success. An unwritable
manual cannot prevent configuration recovery, but its pending correction remains
an explicit unresolved obligation. Each later native return appends a new observation.

### Concurrent writers

The program assumes the owner has stopped other writers for the entire
operation, including restore. Ordinary writes and whole-file restore may
overwrite an intervening edit. A final comparison detects a different final
layout. It cannot detect an edit that was overwritten or later returned to
the same bytes. The owner's chosen quiet interval justifies this accepted loss of the earlier concurrent-write guarantee.
It does not establish that one user has no background processes.

The checklist owns non-harness writers, jobs, login items, Pi, settings actions
inside finish and in-process delegates. The narrow process inspection owns other
Claude Code, Codex, Grove and Ravel processes. The serial program owns its
commands, probes and final comparisons. A detected violation restores the
program's changes and stops. It does not attempt a merge.

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
field. None suppresses ordinary siblings. No new blanket exclusion is allowed
because a probe writes an incidental file.

The design compared three arrangements. Keeping the old exchange and merge
protocol preserves more concurrent edits but requires records, copies and
recovery branches for writers the owner can stop. A handwritten cutover removes
the program but spreads backup, restore and verification obligations across
finish's commands. The chosen serial program keeps those obligations together
and accepts the quiet-interval requirement. Reopen that choice if another grove
or user shares the configuration, or a required writer cannot be paused.

### Resuming the switch

There is no automatic resumption. A caught failure during the quiet interval runs
restore, verifies native delivery and exits unsuccessfully. If no complete backup
exists, use `native-off` and repair preparation. An interruption after backup
publication requires explicit restore before a fresh attempt. Never infer success
from a prefix of completed writes.

Before mutation, preparation retains an immutable attempt note outside `.grove/`
and discovery roots. It names the unique attempt ID, canonical HOME, plan and tool
identities, exact backup destination and literal restore command. The operator
verifies that note and its path before invoking the switch. Its preparation record
survives finish's termination. The backup repeats those identities when complete.
The note records instructions, not progress, success or a recovery state.

Recovery verifies that the supplied backup matches that attempt note and HOME.
The owner selects the note for the interrupted invocation, never an inferred latest
backup. If that choice is uncertain, recovery refuses writes pending review.
No fresh attempt follows an unresolved earlier attempt. There is no general claim
that backup contents can identify a partly written live layout without operator evidence.

A killed program cannot restore while dead. After restart, the owner re-establishes
the checklist and invokes the retained command. If restore itself was interrupted,
repeat that command under the same uninterrupted quiet interval. Outside that
interval, apply the drift rule in [Rollback](#rollback) before overwriting files.
A restore failure preserves backup data and reports failed paths without success.

After finish ends, the owner may use one native Claude Code session as operator.
That session loads the retained instructions and guidance by absolute path and
records its own ancestor identities, cache paths and built-in listing.
Its receipt names that operator rather than claiming finish still runs.
Alternatively, the owner supplies equivalent reader evidence directly from a terminal.
No exemption for the old finish PID survives its death.

An unsuccessful finish uses its original force-stop command. A later recovery or
fresh switch does not repeat teardown, promotion or release. A released-core defect
can require a Grove patch through its owning procedure. Establish native delivery
before any release task that can launch a default session.

### The rehearsed plan

Preparation executes the real plan once against a disposable copy of the relevant
HOME layout. It executes ordinary writes, hook commands, probe cleanup, final
comparison and full restore. It also tests `native-off` without a switch backup.
A read-only listing is insufficient. Sign-in reuse follows the staging contract.
The trial changes no real configuration, launchd job or login item.

The readable result includes a machine-readable input table and a repeatable
read-only comparison command. Each row names a logical input, actual executed
path, content digest and role. Include the plan, program, invoked scripts, imported
local helpers, command inputs, interpreters and external tool versions.
Record identities before and after execution. A changed subject invalidates the run.

The result also contains per-probe outcomes, full restore comparison and limits.
A fixture or stub result supplies no real-harness acceptance.

The comparison command enumerates the real prepared inputs through the same
instrument and reports differences row by row. The operator runs it before each
real attempt and retains its result. After finish, the owner is that operator.
The switch consumes prepared values, not a passing token or rehearsal registry.
No program lookup of a stored admission record is required.

Only declared path substitutions are permitted. The table maps disposable HOME
to real HOME, the candidate Grove keg to the released keg, and `ravel@2` to
promoted Ravel. Compare the plan after replacing only those declared path fields
with their logical names. Every other plan value must match.

Prefix substitution
requires equal source and resource content and equal executable content and versions.
Record installed paths as well as logical names. Arbitrary text replacement is forbidden.

A changed plan, source digest, program, helper, command input or tool version needs
another dry run. A changed released source first needs its affected staging trials
and revised acceptance, then that dry run. Matching content at the declared release
prefix alone needs no new run. Missing inputs or unexplained differences stop
before switch mutation and leave verified native delivery available.

The restore program and compatible backup format remain available until purge.
Retain the plan, attempt notes, comparison instrument and result outside `.grove/`.
A repair must keep existing backups readable or preserve their restore entry.
Backups hold data and format information, not another executable program copy.

### Rollback

One `restore --backup <directory>` entry serves immediate failure, crash recovery
and an eligible later native return. It verifies complete backup contents, metadata,
confined destinations and the selected attempt's identity before writing.
It restores whole files and entries, including original absence. It runs no plugin
uninstall and removes no cache. Unrelated entries outside the write set stay unchanged.
The maintained manual remains append-only and is never a whole-file restore target.

During the same quiet interval, caught failure or an interrupted operation restores
from that attempt's unchanged backup. No concurrent edit is assumed or preserved.
The owner must establish that the interval remained quiet after a crash.
If that cannot be established, automatic whole-file restore refuses pending review.
An I/O failure retains the backup and names the paths requiring repair.

A later restore first compares every destination with the receipt's complete
post-switch identity, including unrelated keys inside switch-set files.
Any content, metadata, registration or absence drift refuses before the first write.
It reports differing paths and preserves the current files. Successful delivery
assertions alone cannot authorise whole-file restore.

The owner must review those
changes and prepare a revised native-return plan that preserves wanted edits.
This entry performs no merge. No `--force` bypass discards intervening changes.

Before an eligible later restore writes, retain the compared current layout in a
new recovery backup and its explicit instructions. If restore is interrupted,
that backup and its note identify the same quiet recovery interval and retry target.
This is another operation with one backup, not forward continuation of the switch.
Uncertain attribution or a broken quiet interval requires review without overwriting.

Restore writes Ravel's off value from backed-up settings first, then native
registrations, settings and entries. It enforces the off value at completion.
It never needs to parse a partly written live settings file. Original absence of
the `skills` key is the deliberate restoration exception. Releases, formulae, mise
declarations and immutable Ravel snapshots are outside restore.

Verification compares restored items against backup data and runs the native
probes with the measured service lifecycle. Promoted Ravel reports `disabled`.
Both harnesses discover native guidance and read a Grove kind skill.
The live operator's recorded guidance remains readable. Unexpected entries or
failed probes leave recovery incomplete.

Recovery invalidates the old activation
receipt and appends the outcome to the manual. It never rewrites past observations.

#### Current health and restore eligibility

The plan names delivery assertions separately from whole-file backup destinations.
These assertions cover the off or switched Ravel value, expected native exclusion,
capability-plugin values and origins, source digests and required reader resources.
Unrelated settings keys and free manual text are outside that projection.
This projection is for observation, not cell-wise inverse or merge.

`mac-check` evaluates those assertions and the current inventory and sources.
It can report healthy switched delivery while whole-file restore is ineligible.
It reports restore eligibility separately, with differing files. A changed model
preference or unrelated hook can invalidate restore eligibility without invalidating
delivery.

A changed skill source, capability or native entry can invalidate delivery.
Missing evidence means not prepared or review needed, never inferred success.
Purge applies the current delivery assertions and retained-resource identities,
not equality of every unrelated key with a day-zero file.

The program's read-only `status` entry supplies this report to `mac-check`.
The plan names native assertions beside the switched ones, and an assertion
can accept a list of values. The real plan before promotion can then accept
both an absent `skills` key and the off value.
Step 8 runs `mac-check` before the receipt exists. The status then evaluates
the running attempt as switched, but only while that switch is an ancestor
process of the status. A crashed attempt is no ancestor, so its layout needs
restore, even when every write completed.

After purge, the retained native return is unavailable. Repair the Ravel route
instead. Frozen native registrations and retained cache paths remain prerequisites
for any eligible native return before purge.

### The recovery model

The old Quint model described cells, exchanges, outside writes and partial
continuations that this design removes. Its success is not evidence for the
simplified switch. The repository no longer holds the model, its runner, its
task or its claim-specific controls. The
[evidence](../owner-skill-delivery-evidence.md#the-recovery-model) keeps the
earlier measurements as history.

The program's existing disposable-HOME seam tests the current contract.
It covers backup completeness before mutation, the full switch, failures and hard interruptions at write boundaries.
It also covers repeatable restore, native probes and final layout comparisons. These are filesystem and command effects which the earlier
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

Use the platform's commands to remove its registrations. Never remove plugin
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
| 6 | Preparation | Prepare installed sources and native residue, freeze updates, complete the owner checklist and dry-run the real plan in a disposable copy of HOME | Accepted inputs are identified by content. Switch and full restore pass without changing the real HOME |
| 7 | Finish | After promotion, establish native delivery with `native-off`. Then back up, switch, probe and compare. Restore on failure | Both harnesses pass, finish keeps its guidance and the receipt is in the manual |
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
  changed resource. Each must be reported. Credential rotation and documented
  reader markers must not change a skill's identity.
- Add a superseded skill beside each excluded platform subtree, at both native
  roots, and through a link. Each must fail the before and after comparison.
  A change confined to `synced` or `.system` must pass. Native guardrail
  from its sole-skill personal plugin must pass. A wrong origin must fail.
  An extra skill must fail, both in the directory that Claude Code loads
  and in the plugin-cache copy.
- A fixture of another Claude Code, Codex, Grove or Ravel process must stop
  the attempt. Exact finish ancestors are allowed. Another descendant
  harness and a background app-server are not. A harmless `caffeinate`
  child needs no exemption. Record which checklist obligations a fixture
  cannot establish.
- Cover native binaries, packaged versioned binaries, Node entry scripts and Codex daemon subcommands in recognition fixtures.
  An unrelated argument containing a tool name must not match. An unknown executable form requires review.
- Run Codex probes with no daemon present, then cover success, failure, interruption, reparenting and PID reuse.
  The measured launcher must leave no owned service before final inspection or restore.
  An existing daemon must refuse without a signal. Unprovable ownership blocks real activation.
- Exercise `native-off` after promotion without a switch backup and after each preflight refusal.
  Cover absent settings, absent key, an existing off value, malformed settings and write failure.
  Preserve unrelated keys and metadata. Resolve as `disabled` before any release task or default launch.
  Interrupt temporary-file creation and replacement. The live file must remain original or completely published, with unrelated preferences intact.
- Run the full real plan once in the disposable copy, including hook
  commands, probes and restore. Change one invoked helper without changing
  its path. The recorded content identity must differ. Inspect the complete
  command-input list, input stability, substitutions and tool versions.
  No rehearsal registry may be required by the real program.
- Compare each dry-run input with the real input through the retained comparison command.
  Equal content at declared replacement prefixes must pass. A changed helper at the same path must fail.
  Changed source acceptance requires staging and a new dry run. Unknown substitutions must fail.
- Stop backup creation before publication. No switch item may have changed.
  Verify content, link text, absence, mode, ACL and extended attributes in
  the complete backup. Fail a copy or metadata read and require no mutation.
- Inject caught failures before and after each real write, during hook
  registration and installation, during deactivation, probes, comparisons,
  receipt and manual writes. Each must restore the native return and exit
  unsuccessfully. Kill the program at those boundaries too. The next action
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
- Measure the complete switch beside an idle signed-in Claude Code parent with the owner's MCP servers.
  Include installation, deactivation, capability enabling, probes and receipt writes. Observe settings writes during the commands.
  Compare loaded plugin versions and registrations before and after, ignoring
  only documented reader markers. An unsolicited settings write, changed
  loaded registration or orphan marker fails the quiet-interval assumption.
  Record the tested and held Claude Code version.
- Interrupt before and after backup publication and recover through the retained attempt instructions.
  Mismatched HOME, attempt note or backup identity must refuse without writing.
  Exercise owner operation after finish ends, with new ancestry and reader evidence.
- Edit an unrelated model key, hook and manual note after success.
  Delivery health must remain accurate. Whole-file drift must refuse a later restore without changing any destination.
  An interrupted eligible later restore must retain its recovery backup and explicit retry instructions.
- Append a manual success observation, then fail final verification.
  Recovery must retain unrelated notes, invalidate the receipt and append a correction.
  A failed manual append must prevent success without preventing native configuration recovery.
- Source controls require off settings to resolve as `disabled`.
  The switched settings must resolve exactly ten sources.
  An eleventh source or one-byte digest change must stop acceptance. A changed released Grove
  source requires its staging reruns. A missing retained staging HOME stops
  them. Trial commands must see its isolated HOME and Codex tree.
- Real staging proves configuration isolation before discovery trials.
  It proves mandatory and implicit reads, relative resources, delegates, compaction and stale-copy coexistence.
  It proves missing-source refusal, offline resolution, hooks and native guardrail.
  It proves TestAnyware VM use, Quint and codebase-memory MCP use.
  A failed case requires repair or an explicit owner scope decision.
- Both real-HOME probes use the promoted pair outside Grove workspaces with
  location and loop-control variables removed. Wrong configuration selection,
  missing native exclusion, wrong source digests, unreadable finish guidance,
  or a lost loaded registration must fail and restore native delivery.
- Change a switched setting, add an outside native entry or change a
  registration during probes. The final comparison must fail and no success
  receipt may remain. A final comparison cannot detect overwritten
  concurrent edits. The design makes no such claim.
- `mac-check` reports native delivery, a currently verified switched layout,
  or a layout needing restore/review. No receipt alone establishes current
  success. Delivery assertions govern health. Whole-file drift separately governs restore eligibility.
  A partial layout or stale delivery evidence requires review. Without
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
