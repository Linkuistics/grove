# Owner skill delivery through Ravel

This design moves the owner's installed skill collection from native harness
installations to Ravel's directory delivery. Its focused design review is
integrated. Ravel later narrowed delivery to local directories, and the section
[Pack checkouts](#pack-checkouts) replaced the earlier policy of Git sources.
Two focused reviews of that change are also integrated. The design changes no
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
   mise holds the checkout of each source at a published commit.
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
  file-system entry, each plugin whose state changes, and each changed
  settings or registry file. The Ravel settings file is in it.
- **Native residue** is the set of native entries that stay after the switch.
- **A pack checkout** is the local clone of one source repository. Ravel
  captures it as a `local` source.
- **A pin** is the published commit at which the owner holds a pack checkout.

### Classes and dispositions

| Class | Baseline members | Disposition |
| --- | --- | --- |
| Owner pack | The `grove`, `linkuistics` and `testanyware` plugins and their skills. Grove's Codex snapshot links. | Ravel source by destination repository. Deactivate the native entries in the switch. |
| Selected third-party pack | Superpowers 7.0.0 and quint-llm-kit 1.0.0, with their Codex copies and links | Ravel source at the upstream commit. Deactivate the native entries in the switch. |
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

| Source ID | Repository | Search roots | Delivered skills |
| --- | --- | --- | --- |
| `grove-core` | `Linkuistics/grove` | Core kind and spine directories | Spine, `configure-grove`, finish, and the planning chain |
| `requirements` | RequirementsSkills | Kind directory | The requirements chain |
| `design` | DesignSkills | Kind and craft directories | The design chain, `decision-records`, `codebase-design`, `cli-tool-design` and `model-led-development` |
| `engineering` | EngineeringSkills | Kind and craft directories | The impl and prototype chains, the coding styles, `using-codebase-memory`, `simplify-project` and `doubt-driven-development` |
| `research` | ResearchSkills | Kind directory | The research pair and the combiner |
| `writing` | WritingSkills | Kind and craft directories | The editorial kinds, `technical-communication` and `writing-code-walkthroughs` |
| `personal` | PersonalSkills | Craft directory | `machine-environment`, `using-jujutsu` and `authoring-conventions` |
| `testanyware` | TestAnyware | Its skill directory | `using-testanyware` |
| `superpowers` | `obra/superpowers` | `skills` | All fifteen superpowers skills |
| `quint` | `quint-co/quint-llm-kit` | `quint-llm-kit-plugin/skills` | `quint-lang`, `quint-modeling` and `quint-execute-spec` |

These rules bind the layout. The research-pack proof settles the exact paths.

- Configure one source for each repository.
- Set its source directory to the narrowest directory that holds the delivered
  skills. That directory also holds every resource they reference by relative
  path. Never use a repository root that holds build output. TestAnyware's root holds `target/`.
- Expose only Ravel-delivered skills through search roots. Keep native-only
  skills and plugin-only trees outside every root.
- Keep each relative symlink inside its source. Ravel refuses absolute and
  escaping links. The baseline found no link in any delivered skill.
- Use no Git attribute that changes the bytes of a checkout, such as a filter
  or an end-of-line conversion. Git's checkout configuration can still change
  the written files. The tree ID of a source directory therefore does not
  identify the files that Ravel captures, and
  [Pack checkouts](#pack-checkouts) states the identity that does.

The first pins are known: superpowers
`bb92a77741419a4ab5f06e711a283343f1ada0c3` and quint-llm-kit
`cc75369f741af7d490936f82002c2d28e3b3d78d`. These are the commits of the
installed plugins. Rollout reads both again before it writes the pins.

### Pack checkouts

Ravel reads each source from a local directory and records no revision. Its
decision record "Skill sources are local directories" states why. The section
"Sources outside Ravel" of its specification states what a tool outside Ravel
does for a source. Ravel and Grove require no particular tool. Other users can
make their checkouts with any tool. This section states the owner's choice.

On this machine, the `[bootstrap.repos]` part of mise owns every pack checkout.
The machine contract already gives machine setup to `mise bootstrap`, so no
other tool is necessary. The entries are in `~/.config/mise/config.toml`, which
mise tracks. Each entry names one checkout:

| Field | Value |
| --- | --- |
| Checkout path | `~/.local/share/skill-packs/<source-id>`, a plain Git clone |
| `url` | The HTTPS URL of the published remote |
| `ref` | The pin, as a full commit ID |

The Ravel source `path` is the checkout path or the narrowest subdirectory
that holds the delivered skills. Ravel expands no tilde, so the settings name
the absolute path, such as `/Users/antony/.local/share/skill-packs/grove-core`.
No harness discovers skills under `~/.local/share`, so a checkout adds no
native entry.

The design assumes that every pack remote is public, so a clone needs no
credential. The four existing remotes are public, and every owner-pack skill is
public today in `Linkuistics/grove`. A private pack needs a credential route in
each HOME that clones it, before its staging trial.

The owner runs these commands. Each names the HOME with `-C`, as `mac-bootstrap`
does, so that a project's `mise.toml` cannot change an entry.

| Purpose | Command | Effect |
| --- | --- | --- |
| Installation on a new machine | `mise bootstrap --adopt AntonyBlakey/mise-setup` | Restores the tracked files first, the Ravel settings included. Then it installs the packages and runs the repository part. Until that part completes, a launch refuses as `skill_source_missing` |
| Installation on a configured machine | `mac-bootstrap`, which runs `mise -C "$HOME" bootstrap` | Runs the repository part among the other parts |
| Installation of the checkouts only | `mise -C "$HOME" bootstrap repos apply` | Clones each missing checkout, and moves each other checkout to its pin |
| Advance of a pin | An edit of the `ref` in the tracked file, then `mise -C "$HOME" bootstrap repos apply` | Fetches, then moves the checkout to the new pin |
| Report of drift | `mise -C "$HOME" bootstrap repos status --missing` | Exits 1 when any checkout is missing, dirty, at another commit or has another origin |

A dirty or conflicting checkout stops `apply` before it changes any checkout.
It also stops a complete `mise bootstrap` run before its later parts.
`--skip-dirty` skips a dirty checkout, but not a conflicting one.

`mise bootstrap plan`, which `mac-check` runs today, ignores repositories.
`mac-check` therefore gains the status command. It reads the `--json` output,
so that it can tell drift from a configuration error.

The tracked file records the intended pin of each checkout. mise's local
history records each edit, and `mise dot sync` publishes it. The HEAD of each
checkout records the delivered commit, and the status command reports it as
`current_sha`. The two agree exactly when the state is `current`. Ravel's
descriptor records the content digest of each capture and its origin path. The
cutover receipt joins the source ID, the checkout path, the pin and the digest.

The `grove-core` pin follows Grove's releases. After each Grove release, the
owner advances it to the released commit. Without that advance, the installed
Grove and the delivered spine can differ.

During this cutover, the staging trials accept a candidate pin for
`grove-core`. The core layout reaches `main` only in finish, so the candidate
is a published commit of this workstream's line. Later leaves can still change
the core skills. The candidate is therefore the last commit of the line that
changes the `grove-core` source directory. A later change needs a new
candidate and a rerun of the trials that read `grove-core`.

A committed evidence document records the Git tree ID of the `grove-core`
source directory at the candidate, from `git rev-parse <commit>:<path>`. The
tree ID identifies the committed files, their executable bits and their links.
Before finish, rollout shows that the final line has the same ID. The switch
compares the ID again at the released commit, to decide whether the core
trials run again.

The tree ID does not identify the files that a checkout holds. With
`core.autocrlf=true`, Git writes CRLF line ends. With `core.symlinks=false`, it
writes a link as a regular file that holds the target. Neither changes the
tree ID, and `git status --porcelain --ignored` stays empty. The staging clones
and the real-HOME `apply` run under different Git configuration, so their
files can differ.

Ravel captures the written files. Its `resourceDigest` covers their relative
paths, types, executable bits, link targets and contents. The checkout path is
not part of it. The accepted identity of each source is therefore its
`resourceDigest`, from `ravel skills resolve --json` in the staging HOME.

After the Ravel cut, rollout records each accepted digest again with the
published archives. Before the switch changes anything, each real-HOME checkout
must give its accepted digest. An equal tree ID never admits a different
digest.

Nobody edits a pack checkout or runs a tool inside it. Development happens in
the working copy of each pack under `~/Development/AgenticOrchestration`.
`apply` refuses a checkout with local changes, and Ravel would capture those
changes. The status command does not see a file that Git ignores, such as
`__pycache__`, but Ravel captures it. In a clean checkout,
`git status --porcelain --ignored` prints nothing.

An advance needs an interval in which nothing captures the checkouts that it
changes. A capture during the change of a checkout refuses as
`skill_source_changed`. Ravel's capture is not atomic, so it can also retain a
mix of two commits.

Ravel captures a checkout at each `ravel start-session` and each
`ravel skills resolve` whose settings name it. A Grove launch, `grove run` and
the release notes of a release task reach Ravel through `start-session`. Any
harness session can run either command, also a session that Ravel launched. A
Ravel 1.1 launch captures nothing. Settings without a `skills` key capture
nothing either. A running session reads its retained snapshot until it runs
Ravel again.

Every `apply` and every complete `mise bootstrap` counts as an advance. A pin
that `mise dot sync` brought from another machine moves a checkout without a
local edit.

Before an advance, the owner lists the live Claude Code, Codex, Pi, Grove and
Ravel processes, as switch step 1 does. The owner ends each other harness
session and each Grove driver, or waits for it to end. Until `apply` ends, the
owner runs no Ravel command and no release task. A session can do the advance
and keep its own driver. That driver launches the next session only after this
session ends. The session runs no Ravel command until `apply` ends.

Preparation and the switch need no stop. The real HOME's settings name no skill
source until the switch writes them. Switch step 1 finds no other Ravel or
Grove process, and finish's own launch is a Ravel 1.1 launch.

An unpublished edit reaches a launch only through trial settings in a staging
HOME. Those settings can name a development working copy as a `local` source.

A launch behaves as follows:

- Ravel captures each checkout at each launch. It reuses the published snapshot
  while the files stay the same. Each pin that the owner applies therefore adds
  at most one snapshot, while the checkout holds only the files of its pin.
- A missing checkout, or a missing configured root, refuses as
  `skill_source_missing`, exit 3, before Ravel launches any worker. Grove's
  loop stops without a relaunch. The owner runs
  `mise -C "$HOME" bootstrap repos apply`, then reruns Grove. No launch substitutes native skills or another revision.
- Ravel reads no revision, so a checkout at another commit delivers its files
  without a report. The status command reports that state.

After the first installation, the route needs no network except for an advance:

- A launch needs no network, because Ravel reads the checkouts on disk.
- The status command works for commit refs. `apply` succeeds without a fetch
  when every checkout is at its pin.
- An advance fails without a network. `apply` fetches before it changes a
  checkout, also when the clone already holds the commit. The checkout keeps
  its earlier commit, and launches keep delivering it. The status command
  reports `differs` until an `apply` succeeds.
- A missing checkout stays missing until a network is available.

The policy costs the automatic updates that the Claude Code marketplace gives
today. An unpublished edit cannot reach a default launch until its pin
advances. In return, both harnesses read identical bytes, launches need no
network, and each pin has one snapshot. A rebuilt machine clones the same
commits from the remotes while each remote keeps them.

A third-party remote can rewrite its history. When a pin leaves its remote, a
fresh clone cannot reach it, and the owner chooses a new pin. An existing
checkout keeps working.

The design rejected these alternatives:

- **Development working copies as default sources.** Each launch would capture
  uncommitted or conflicted jj states. Ravel retains every distinct capture and
  has no collector, so storage would grow with each edit.
- **Checkouts that follow the default branch.** An entry without a `ref` reports
  `current` after the remote moves ahead. Nothing then records which commit a
  checkout holds.
- **A lock file and a synchronisation task in the marketplace repository.** It
  would do what mise does, and it would add a second owner of machine setup.
- **The Claude Code marketplace clones.** Claude Code updates and erases them,
  and Codex delivery would then depend on Claude Code.

The marketplace keeps its own revisions for native users. Rollout verifies that
the marketplace and the owner's pins name the same commits at the switch. Later
divergence is allowed when it is deliberate.

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
[off-file decision](../adr/the-off-file-is-the-only-switch.md) makes one file
the only switch for every always-on layer. Ravel reads a mandatory file
unconditionally, so it would ignore that switch. No Grove or personal guidance
is mandatory, because Grove's mandate names the kind skill. The explanatory
output style is a harness style rather than guidance. It stays a native hook.

### Native residue

| Capability | Route after the switch |
| --- | --- |
| Always-on technical communication on Claude Code | A skill-free Claude Code plugin from WritingSkills. Its root is the skill directory, as for the existing Codex hooks plugin. |
| Always-on technical communication on Codex | Absent today, and the switch keeps it absent. Installing the existing Codex hooks plugin is a separate owner choice. |
| `guardrail` | A Claude Code plugin from PersonalSkills that holds only `guardrail` |
| Superpowers bootstrap text | The Ravel mandatory file. The superpowers plugin is disabled. |
| Explanatory output style, Context7, Playwright, commit-commands, feature-dev and Swift LSP | The same enabled plugins, which hold no skills |
| codebase-memory MCP server, hooks, agent definitions and skill copies | Unchanged. Its own updater rewrites them. |
| herdr and Orca hooks | Unchanged. Their applications own them. |
| `quint-kb` MCP server | Unchanged. It runs from the Quint clone, which stays. |
| TestAnyware command, application and configuration | Unchanged installations. The skill reaches them by ordinary command execution. |

The repository moves must supply three packaging changes before the switch:

- WritingSkills offers the technical-communication hooks as a Claude Code
  plugin without a `skills/` directory. A trial must show that Claude Code runs
  the hooks and lists no native `technical-communication` skill. If Claude Code
  lists the root `SKILL.md` as a skill, repair the layout before the switch.
- PersonalSkills offers a Claude Code plugin that holds only `guardrail`.
- The new marketplace lists both plugins beside the ordinary pack plugins.
  Native users keep the full plugins, and Ravel users enable only these two.

The design rejected user hooks that point into a working copy. Such hooks run
uncommitted code, and a layout change breaks them without a report. The Pi
extension link shows that failure. A snapshot path changes with each pin, so a
hook that names a snapshot breaks at each pin advance. A hook that names a pack
checkout runs committed code. A layout change at a pin advance still breaks it
without a report.

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

Ravel snapshots are immutable and retained. Each source pin has one snapshot.
An advance changes a pack checkout but no snapshot. A running session keeps
reading the snapshot of its launch.

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
copy elsewhere does not protect an orphaned absolute path. A late rollback
installs from the marketplace when a needed version directory is gone.

### Settings isolation and the staging HOME

All trials before finish run in a staging HOME. It holds its own Ravel
configuration and settings with the ten sources and the mandatory file. Its
harness configuration is a prototype of the native residue: the skill-free
plugins are enabled, and no superseded skill is installed. The staging HOME
copies no credential.

The staging checkouts lie outside the staging HOME. A trial clones them with
the drafted `[bootstrap.repos]` entries under a separate checkout HOME. mise's
state and the symbolic links inside a pack repository therefore add no link to
the staging HOME.

The environment of the checkout run holds only these variables, as the
environment of the `task owner:checkouts` fixture does:

| Variable | Value |
| --- | --- |
| `PATH` | The directories of the selected `git` and `mise`, then `/usr/bin` and `/bin` |
| `HOME` | The checkout HOME |
| `XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_STATE_HOME` and `XDG_CACHE_HOME` | Directories inside the checkout HOME |
| `MISE_CONFIG_DIR`, `MISE_DATA_DIR`, `MISE_STATE_DIR` and `MISE_CACHE_DIR` | Directories inside the checkout HOME |
| `GIT_CONFIG_NOSYSTEM` | `1` |
| `GIT_CONFIG_GLOBAL` | The null device |
| `GIT_TERMINAL_PROMPT` | `0` |

No inherited Git configuration, credential program or SSH agent reaches the
run. The system configuration of both installed Git builds names
`osxkeychain`. The owner's global configuration is on the XDG path and names
Git Credential Manager. Each of these inherited variables would expose a
helper: `XDG_CONFIG_HOME`, `GIT_CONFIG_GLOBAL`, `GIT_CONFIG_COUNT`,
`GIT_CONFIG_PARAMETERS` and, without `GIT_CONFIG_NOSYSTEM`,
`GIT_CONFIG_SYSTEM`. `GIT_ASKPASS` and `SSH_ASKPASS` name further credential
programs, and `GIT_TERMINAL_PROMPT` does not disable them. These seven
variables are the credential routes.

A fixture control proves the allowlist before the first clone from a real
remote. It runs in a fixture HOME, and it never uses the real keychain:

1. Serve a local remote that asks for a credential.
2. Give a decoy helper that writes a marker file and returns no credential.
3. Build one case for each route. Its environment is the allowlist without
   the entry that blocks the route, with the route added.
4. Clone in each case through `mise -C <fixture HOME> bootstrap repos apply`.
   The marker must appear in each case.
5. Put every route into the parent environment, and build the allowlist from
   that environment.
6. Clone through that allowlist with the same command. The clone must fail,
   and no marker may appear.

A remote that needs a credential then fails the trial without a read of the
real keychain.

The staging settings name the staging checkouts by absolute path. They differ
from the drafted real settings only in those paths. The staging HOME and the
checkout HOME stay until the switch writes its receipt, because step 4 of the
switch can need them.

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

The real HOME's Ravel settings gain the skill configuration only inside the
switch. By then, finish's own launch was this driver's last 1.1 settings read.
The switch also requires evidence that no other 1.1 driver can launch.

### The final session and new launches

- **Until finish launches**, the native setup does not change. Preparation may
  install disabled plugins, add a marketplace and write `config.rhai` beside
  `policy.ts`. It may also add the `[bootstrap.repos]` entries and clone the
  pack checkouts. Each of those changes leaves native discovery unchanged.
- **Finish** launches through this driver, Ravel 1.1.0 and `policy.ts` into
  Claude Code. It loads `grove-finish` and the spine from the enabled Grove
  plugin's cache path. Its lazy reads use that absolute path.
- **The switch** runs inside finish, after promotion. It disables the Grove
  plugin but neither updates nor uninstalls it. The version directory keeps
  its registration and receives no orphan marker, so finish can still read its
  own guidance.
- **After the switch**, a session that the owner opens directly sees only the
  native residue.
- **After this driver's reap**, a new driver resolves compatible Grove and the
  promoted Ravel 2.0.0. Each of its launches reads `config.rhai`, captures the
  pack checkouts and receives the directories.

Finish keeps its original ending command and the full 1.1.0 keg, under the
coordinated release design.

### The switch inside finish

Finish runs the prepared and rehearsed switch after promotion and the verified
Grove/Ravel pair. Its steps are:

1. List live Claude Code, Codex, Pi, Grove and Ravel processes. Include
   desktop applications and their background servers. Also include each
   program that writes a file of the switch set, such as herdr, Orca and the
   codebase-memory updater. Exclude finish, its descendants and the driver
   chain that launched it.
2. If any other listed process remains, stop and ask the owner. Do not signal
   another session.
3. Rerun the baseline instrument. Then look for a complete backup of an
   earlier run. If one exists, find the state of its switch set first, as
   [Resuming the switch](#resuming-the-switch) states. Compare the change
   identity of each entry outside that switch set with the prepared manifest.
   Without a backup, that is every entry. Stop for classification on any new,
   missing or changed entry. Also stop on any `UNCLASSIFIED`,
   `UNASSIGNED`, `UNCOVERED` or `NEEDS-REVIEW` row. Compare the built-in rows,
   whose digest says `unverified`, with finish's own skill listing.
4. Read the Git tree ID of the `grove-core` source directory at the commit of
   the latest published Grove release. That is finish's release, or a later
   patch release that a repair needed. Compare the ID with the candidate's ID
   in the committed staging evidence. If the IDs are equal, the candidate's
   digest is the accepted `grove-core` digest. If they differ, rerun the
   staging trials that read `grove-core` with the released commit. They run
   in the retained staging HOME, under the environment rule of step 10. Stop
   if one fails. The digest of that rerun becomes the accepted `grove-core`
   digest.
5. Advance the `grove-core` pin to the released commit of step 4. Run
   `mise -C "$HOME" bootstrap repos apply`. Nothing captures a checkout
   meanwhile, as [Pack checkouts](#pack-checkouts) states.
6. Run `mise -C "$HOME" bootstrap repos status --missing`, and stop unless it
   exits 0. In each pack checkout, `git status --porcelain --ignored` must
   print nothing. Each pin in the tracked file must equal its accepted pin in
   the staging evidence. For `grove-core`, the accepted pin is the released
   commit that step 4 admitted. Then write the drafted real Ravel settings
   into a disposable HOME. Run `ravel skills resolve --json` there with the
   promoted `ravel`. Stop unless each source's `resourceDigest` equals its
   accepted digest.
7. Record the reader evidence. Name each plugin cache version directory that
   finish loaded and its registration in `installed_plugins.json`. Stop if any
   of those directories holds `.orphaned_at`.
8. Write a new backup on the volume of the real HOME, outside each tree that
   the baseline instrument sweeps. It holds copies of the switch set's files,
   the prepared manifest and the inverse operation. It also holds the switched
   state of each item, as the dry run computes it. The evidence of steps 4, 6
   and 7 goes into it too. Write it under a temporary name, and rename it into
   place as the last action. Only that rename makes the backup complete. No
   backup replaces an earlier one.
9. Apply the switch as one operation. Write the real-HOME Ravel skill
   settings. Disable the superseded plugins and enable the two skill-free
   plugins. Move each superseded file-system entry into the backup's retained
   tree at its original relative path. Write each changed file once, by a
   rename of its new content. Before each rename, compare the file with its
   copy. A file that differs is a failed part. If any part fails, apply the
   inverse operation.
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
13. Write the receipt into the backup, with the result of `mac-check`. Write
   it under a temporary name, then rename it into place.
14. Copy the receipt into the maintained manual.

Steps 1 to 8 change no native entry and no Ravel setting. A stop in those
steps therefore leaves native delivery working with the published Grove and
the promoted Ravel. A failed `apply` keeps the edited pin. The tracked file
then names the released commit, and the status command reports `differs`. No
launch captures the checkout, because the real HOME's settings name no skill
source then.

A complete inverse of step 9 or step 11 restores the same state, and the
native probe verifies it. The inverse can also fail part of the way. A session
can also end inside any of steps 8 to 14. In each case the next run finds the
state from the layout, as [Resuming the switch](#resuming-the-switch) states.

When a step stops or an inverse runs, finish continues as follows. A failed
write of the receipt is a stop.

1. Find the state of the switch set again, as step 3 does.
2. Write a pending-activation record into the maintained manual. Name the
   published Grove version and its release commit, and the promoted Ravel path
   and version. Add the edited pin, the output of `status --json` and the
   output of the failed step or probe.
3. In the record, name each backup and the state of the switch set. For a
   partial or drifted state, list each item with its state.
4. Report the failure, and ask the owner to repair its cause.
5. Run the switch again from step 1.
6. If the session must end before the switch succeeds, end it with the stop
   command of its launch prompt. Never send the success signal.

After such an end, or after finish ends without a result, the owner runs the
same prepared switch from step 1. The owner runs it outside any Grove session.
That run tears down and promotes nothing, and it releases nothing except a
patch release that a repair needs. One Claude Code session runs it and takes
finish's place in steps 1, 3 and 7. The receipt replaces the
pending-activation record.

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
- A TestAnyware resource is read, and `testanyware doctor` runs.
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
records each source's checkout path, pin and snapshot digest. For `grove-core`,
it records both tree IDs and each trial that step 4 ran again. It records the
reader evidence, each backup location and each retained byte set with its purge
condition. The [finish decision](../adr/finish-promotes-after-the-grove-release.md)
records why this runs inside finish.

### Resuming the switch

A run can stop, apply its inverse or end before its receipt. A session can also
end before it writes the pending-activation record. Step 3 of each later run
therefore reads the state from the layout and the latest complete backup, not
from that record. It keeps an incomplete backup aside and reads nothing from
it. Each complete backup holds the same earlier state, because step 8 runs only
when the switch set is native.

Each item of the switch set has an earlier state and a switched state:

- A moved entry is at its original path only, or at its retained path only.
- A plugin's enablement key in its harness settings holds its earlier value or
  its switched value.
- In a file that other programs also write, step 3 compares only the keys that
  the switch changes. A hook update by a tool therefore changes no state.
- The Ravel settings file holds its copied content or the switched content.
  When no such file existed before the switch, its absence is the earlier
  state.

An item in any other condition is in neither state. An entry at both paths is
an example. Step 9 and the inverse change each item in one atomic step. An
interruption alone therefore leaves each item in one of its two states.

| State of the switch set | Evidence | Next steps |
| --- | --- | --- |
| Complete | The latest complete backup holds a receipt | Copy the receipt into the maintained manual if the manual lacks it. Change nothing else |
| Active | Each item is in its switched state, and the backup holds no receipt | Do not run steps 4, 5, 7, 8 or 9 again. Run step 6, the probes of step 10 and steps 12 to 14. A stop in step 6 counts as a failed probe. The receipt uses the evidence in the backup |
| Native | Each item is in its earlier state | Run the native probe of [Rollback](#rollback). Stop if it fails. Otherwise the run goes on as without a backup |
| Partial | Each item is in one of its states, and the states differ | Resume the inverse from the latest complete backup. Then find the state again |
| Drifted | An item is in neither state | Resume the inverse for each item in its switched state. Then stop for classification |

After the owner classifies each drifted item, the native probe must pass. The
owner then prepares the manifest and the dry run again. A new complete backup
records the new earlier state, and the switch runs again from step 1. No item
is in its switched state then, so that backup holds a native state.

In the active state, a failed probe leads to the inverse of step 11. Release,
promotion and teardown are outside each of these continuations.

### Rollback

Inside finish, rollback is the inverse of the switch. It sets each changed key
back to its earlier value, and it writes each file once by a rename. It
restores the copied Ravel settings. When no such file existed before the
switch, it leaves no Ravel settings file. It moves each entry back. It changes only an item in its
switched state, so a second run resumes an incomplete one.

The native probe verifies the result on both harnesses. It observes these
results:

- `ravel skills resolve --json` with the promoted `ravel` reports `disabled`.
- Native discovery on each harness lists the superseded skills again.
- One session on each harness, launched as in step 10, reads a Grove kind
  skill natively. The kind is not `finish`, and the task asks only for one
  quoted line.
- Inside finish, finish reads one of its own guidance files again by its
  recorded cache path.

After finish, the owner can apply the same inverse operation while the backup
and the plugin caches exist. Otherwise, the supported return to native delivery
installs the current marketplace plugins. No rollback erases Ravel snapshots.
Rollback keeps the pack checkouts, their mise entries and the advanced
`grove-core` pin. No harness discovers skills in them.

### Purge

Purge covers each backup's retained tree, Grove's Codex snapshot store, the
superseded Claude Code plugin registrations and the earlier marketplace
registration. Purge requires three conditions:

- No session that ran during the switch remains.
- This driver's group is reaped.
- The owner decides that the rollback window is closed.

Purge is outside this grove's success condition, as the 1.1.0 keg cleanup is.
The receipt lists each item with its condition. The maintained manual carries
the procedure. The owner uninstalls platform registrations only through the
platform's own commands. Nobody erases a cache directory by hand. A Claude Code
uninstall marks the version orphaned, and the platform erases it 14 days later.

### Independent sessions

After the switch, a session that the owner opens directly with `claude`,
`codex` or `pi` sees only the native residue. It can receive the same
directories through the output of `ravel skills resolve`. It can also read the
pack checkouts by path. A pin advance then changes the files under that
session. The root
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

## Cutover contract

`owner-ravel-skill-inventory-k38` grows the concrete leaves. Rollout orders them
with the coordinated release work.

| Order | Owner | Step | Done when |
| --- | --- | --- | --- |
| 1 | This design | Baseline, classification and contract | The baseline and this design are committed and reviewed |
| 2 | Repository moves | Move every owner-pack and selected third-party entry with its resources. Supply the three packaging changes. Publish each pack repository on its remote. Publish this workstream's line, which holds the `grove-core` candidate. | Each moved skill's content and resources match the baseline or a recorded change. Each pack has a published commit for its pin |
| 3 | k38 | Re-inventory, compare per entry, classify each new entry | No entry stays unassigned |
| 4 | k38 leaves | Convert `policy.ts` to `config.rhai`. Draft the Ravel skill settings, the `[bootstrap.repos]` entries and the residue plan. | The drafts are committed. No draft launches a session before the clones of order 5 |
| 5 | k38 leaves | Run the staging trials on both harnesses with the accepted feature build | The drafts pass under the staging HOME. Each trial below passes or carries a recorded scope decision. The evidence names each accepted pin, each accepted `resourceDigest` and the tree ID of the `grove-core` candidate |
| 6 | k38 leaves | Prepare the real HOME without changing discovery. Add the `[bootstrap.repos]` entries, clone the pack checkouts with `mise -C "$HOME" bootstrap repos apply` and add the status command to `mac-check`. Freeze automatic updates of the earlier marketplace. Rehearse the switch, the probes and the inverse in disposable layouts. Interrupt the switch, the inverse and the receipt write in some of them. | The status command exits 0. The rehearsal restores every rehearsed layout byte for byte. After each interruption, the next run finds the state and reaches the native or the complete state |
| 7 | Finish | Run the switch after promotion | The probes pass and the receipt is in the manual |
| 8 | Owner | Purge retained bytes | The purge conditions hold |

The staging trials run three steps first, in this order:

1. Run the credential control of
   [Settings isolation and the staging HOME](#settings-isolation-and-the-staging-home).
2. Clone the pack checkouts from the remotes into the checkout HOME, with the
   drafted entries. Write the staging settings with those paths.
3. Run the isolation trial of that section.

The clones launch no harness, so the isolation trial is still the first trial
that launches a session. The trials use the accepted feature build. After the
Ravel cut, rollout verifies the pins again against the published archives, as
step 5 of the [coordinated release design](coordinated-release.md) states.

The other trials cover implicit craft discovery and
the mandatory read order. They cover a Grove kind skill that the task names, a
TestAnyware resource read and real VM tool use. They also
cover the Quint and codebase-memory MCP servers, the Claude Code hooks and
`/guardrail`. Each harness also runs one delegate, one compaction and one
stale-copy coexistence case.

One trial moves a pack checkout away and observes `skill_source_missing` before
any worker. One trial launches without a network and observes the same
directories. A trial fails if native discovery lists a superseded skill.

One Claude Code trial disables the plugin that holds its own guidance and
launches a nested probe. It then reads its guidance by absolute path again.

Persistent setting changes follow the machine configuration contract. mise
tracks `~/.claude/settings.json`, `~/.codex/config.toml`, `~/.codex/hooks.json`
and `~/.config/ravel` in place. The `[bootstrap.repos]` entries are in the
tracked `~/.config/mise/config.toml`. The pack checkouts, the skill directories
and the plugin registry are not tracked files. The maintained manual is the
Obsidian `AI Dev Tooling` note. It still names the earlier
`~/Development/grove` checkout.

## Test seams

- The baseline instrument, rerun, compares entries one by one by their change
  identity. Its control root must show one unclassified entry and one dangling
  entry. Its fixture controls must show that each class of change changes its
  entry, and that a rotated credential does not.
- `task owner:checkouts` measures the checkout route of mise in a fixture HOME
  with a fixture origin. Its `--mutate` run must fail.
- Staging trials use Ravel's public `start-session` and `skills resolve` with
  the actual harnesses. They clone the pack checkouts from the real remotes.
  A decoy-helper control must show a helper call through each inherited
  route, and none through the allowlist, before the first such clone.
- The switch and its inverse have a dry run against the manifest. Rehearsals
  use disposable layouts. Their controls stop a run inside step 8, inside step
  9, inside the inverse and before the receipt. The next run must find the
  native, partial and active states. It must keep the incomplete backup aside.
- Other rehearsal controls change a hook in a file of the switch set, and put
  an entry at both paths. A hook change between steps 8 and 9 must stop step 9.
  A hook change at another time must change no state. An entry at both paths
  must give the drifted state.
- A control clone with `core.autocrlf=true` keeps the tree ID of its pin. The
  digest comparison of step 6 must stop on it.
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
- Automatic pin advances, such as a scheduled `apply`. Reopen this item if the
  owner wants a pack to follow its branch.
- A checkout tool for other Ravel users. Ravel and Grove name none.
