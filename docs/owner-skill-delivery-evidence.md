# Owner skill delivery evidence

The [owner specification](specs/owner-skill-delivery.md) describes the current
design. The exchange, held-copy, recovery-model and rehearsal-admission
measurements below are historical evidence for the earlier design. They do
not certify the simplified serial switch. [Single-owner switch design](#single-owner-switch-design)
states the present evidence boundary.

This document records the inputs to the
[owner skill delivery design](specs/owner-skill-delivery.md). The design
session only read the machine. It changed no installation, setting, plugin
registry or file outside this checkout, and it signalled no other session.
The integration of the design's focused review repaired the instrument and
regenerated the baseline on the same day. It also only read the machine. The
section [Review corrections](#review-corrections) records what changed.

After Ravel narrowed delivery to local directories, `owner-pack-checkouts-k79`
measured the owner's checkout tool in a fixture HOME. The section
[Pack checkouts](#pack-checkouts) records that measurement as history.
`skill-marketplace-decision-k201` replaced the checkouts with Homebrew
formulae, and `candidate-keg-k238` removed the measurement's script and task.

## Method

On 2026-11-02, the session walked every skill discovery root of Claude Code,
Codex and Pi. It also read both Claude Code plugin registries and each
installed plugin's components. The walk covered user hooks, agent definitions,
MCP entries, the owner policy and the tool installations that skills need.
The repaired instrument also walks Codex's installed plugin cache. A final
sweep finds every `SKILL.md` under `~/.claude`, `~/.codex`, `~/.agents` and
`~/.pi`. Each file must lie in a walked root or a classified container.

[`evaluations/owner-skill-delivery/inventory.py`](evaluations/owner-skill-delivery/inventory.py)
is the instrument. It writes [the baseline](owner-skill-inventory.tsv), one
row for each entry. Each row names the entry's harness, location, form,
frontmatter name, digest, file count, class, source owner, capabilities,
disposition and a note. Rows carry `~` for HOME.

The digest is the entry's change identity: the first 16 hexadecimal digits of a
SHA-256 over the state that defines the entry. The instrument's docstring
defines it for each class:

| Entry class | State in the digest |
| --- | --- |
| Skill tree | Sorted relative paths, each file's executable bit and content, and each file or directory link's target |
| Claude Code plugin, Codex installed plugin | The whole version directory |
| User hook | Its matcher and definition, and each file under HOME that its command names. One row exists for each hook. |
| MCP server | Its definition. The instrument replaces env and header values, and values of keys that name a token, secret, password or key, with their names. |
| Agent definition, Pi extension, skill override | Its content |
| Tool | The resolved executable, the bundle's `Info.plist` or the clone's revision and changed paths |
| Coverage row | The sorted `SKILL.md` paths of its container |

A rotated credential therefore does not change an MCP row. A digest holds no
raw value. A row that the instrument cannot observe says `NEEDS-REVIEW`.
Built-in Claude Code skills have no files. Their digest says `unverified`,
and a session's own skill listing is their only evidence. This session's
listing holds the same 18 built-in names.

| Artifact | SHA-256 |
| --- | --- |
| `docs/owner-skill-inventory.tsv` | `cbfac5b83548a3f434f34d2ac4e15afcc4004ea6b212a0beabff58b3de58a0d0` |
| `docs/evaluations/owner-skill-delivery/inventory.py` | `f3c2a86e838594c922b4e20622ff8b60e592cb200d21b3dc65b96ad7d1c43726` |
| `docs/evaluations/owner-skill-delivery/controls.py` | `6a63437b87d457a22a3ae11039393343806550f6abdbf0cd8715d7d63137418d` |

Two runs of the saved instrument produced byte-identical output, and the
committed baseline is that output. The measurement digested the instrument, the
controls script and both Claude Code settings and registry files before and
after the runs. It also digested both Codex configuration files and the
`mcpServers` object of `~/.claude.json`. No digest changed.

The instrument and the controls script of this table produced the baseline.
`owner-inventory-reader-marker-k244` later changed both of them. The section
"Reader markers in the plugin cache" records their current values and how a
comparison reads the baseline's Claude Code plugin rows.

### Controls

A clean read from a broken instrument looks the same as a clean read from a
working one. Four controls separate the two cases:

- **Mutation control.** A scratch root held an unknown skill directory and a
  dangling link. The instrument reported one `UNCLASSIFIED` row and one
  dangling row for that root.
- **Clean run.** Without the scratch root, no row said `UNCLASSIFIED`,
  `UNASSIGNED`, `UNCOVERED` or `NEEDS-REVIEW`.
- **Positive control.** The instrument found `using-testanyware` in all four
  places that hold it: the Claude Code plugin, both Codex user roots and the
  dangling Pi link.
- **Change controls.** [`controls.py`](evaluations/owner-skill-delivery/controls.py)
  builds a fixture HOME and applies one change in each case. The repaired
  instrument passed all 14 cases. They cover a changed hook command or script
  and a second hook with the same event and owner. They cover a changed MCP
  command or arguments with the same name. They also cover a lost executable bit, a
  retargeted directory link and changed agent, skill-override and
  remote-installed plugin content. The last two cases are a skill in an unknown
  Codex marketplace and a skill outside every walked root. A repeated run and
  a rotated MCP credential left the output unchanged.
- **Failing revision.** The reviewed instrument, SHA-256 `6138bacf…9792`,
  passed only the two unchanged cases. For each of the other 12 cases, its
  whole output stayed byte-identical, so its comparison gate would accept the
  change.

`task owner:inventory` prints the inventory, and `task owner:inventory:controls`
runs the change controls.

## Inputs

| Subject | Observation |
| --- | --- |
| Harnesses | Claude Code 2.1.316 and Codex CLI 0.166.0. Pi's settings record changelog version 0.84.2. |
| Grove and Ravel | Installed Grove 22.8.0 and Ravel 1.1.0. The driver's ending executable is `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`. |
| Linkuistics plugins | Installed version `238c3a839ab4`, the marketplace clone's commit `238c3a839ab4b30b101fbe3cc5212d66091df28a`. The marketplace follows `Linkuistics/grove` with automatic updates. |
| Superpowers | Plugin 7.0.0 at `bb92a77741419a4ab5f06e711a283343f1ada0c3`, from `https://github.com/obra/superpowers.git`. Fifteen skills and one `SessionStart` hook. |
| Quint | Plugin 1.0.0 at `cc75369f741af7d490936f82002c2d28e3b3d78d`. The clone `~/.quint-llm-kit/repo` is at the same commit, with a modified `package-lock.json`. `quint` 0.33.1. |
| TestAnyware | Homebrew `linkuistics/taps/testanyware` 2.7.0 and `/Applications/TestAnyware.app` 2.7.0. `testanyware doctor --json` passed its installation, configuration, directory and local-network checks. Its login-item check failed, which does not affect a skill read. |
| codebase-memory | `codebase-memory-mcp` 0.10.6. Its installer wrote skill copies, agent definitions, hooks and MCP entries in both harnesses. |
| mise | 2026.10.25. The owner's configuration has no `[bootstrap.repos]` entry. `mac-check` runs `mise bootstrap plan --detailed-exitcode`. |
| Remotes | On 2026-11-02, `gh repo view` reported `Linkuistics/grove`, `Linkuistics/TestAnyware`, `obra/superpowers` and `quint-co/quint-llm-kit` as public. The pack repositories that the moves create do not exist yet. |
| Owner policy | `~/.config/ravel/policy.ts` routes each kind to `claude` or `codex`. `finish` goes to Claude Code with `claude-opus-5-5`. No route selects Pi. |
| Ravel settings | `~/.config/ravel` holds only `policy.ts`. No `settings.json` exists. |
| Ravel contracts | The local-source contract `docs/specs/skill-discovery.md` at `7a894c78`, on the unpushed bookmark `skill-directory-delivery`. The historical directory contract at `b93bb927`. Ravel main's `docs/specs/configuration-evaluation.md` at `24a54523`. The sandbox-home recipe in `docs/specs/configuration-evaluation.md` and `scripts/harness-smoke.sh` at `2a77ffbda3b2510f0997c740c0642b18afaeece7` in `Ravel.coordinated-release-design`. |
| Codex home | The Codex 0.166.0 executable reads `CODEX_HOME` and canonicalises it. It names `.codex/config.toml`, `.codex/agents`, `.codex/hooks` and `.agents/skills` as user paths, and `auth.json` as its file sign-in store. The owner policy passes `-c` overrides only and sets no `CODEX_HOME`. |
| Claude Code plugin cache | The [plugin loading reference](https://code.claude.com/docs/en/plugins/loading#cleanup-of-previous-versions), read on 2026-11-02. An update or uninstall writes `.orphaned_at` into the earlier version directory. Cleanup can remove it after 14 days. The [executable evidence](#finding-for-the-owner-design) adds the live-reader check for 2.1.319. A running session keeps the plugin set that it loaded until `/reload-plugins` or a new session. |
| Claude Code configuration directory | The [environment reference](https://code.claude.com/docs/en/env-vars) and the [credential management section](https://code.claude.com/docs/en/authentication#credential-management), read on 2026-11-02 by `ravel-rhai-skill-delivery-k50`. `CLAUDE_CONFIG_DIR` replaces `~/.claude` for settings, history and plugins. It also moves the sign-in file and keys the macOS Keychain entry to that directory. Three `CLAUDE_CODE_PLUGIN_*` variables add plugin locations. No trial measured these statements. Ravel's sandbox recipe at `7a894c78` also names `CLAUDE_SECURESTORAGE_CONFIG_DIR`, which moves only the sign-in store. |

Ravel main keeps settings at `$HOME/.config/ravel/settings.json` for 2.0.0.
Its keys are `timeoutMs`, `contextBytes`, `configEnv`, `harnesses` and
`budgets`, and an unknown key refuses. The historical directory contract adds a
`skills` key and states that a 1.1 launch refuses shared settings that hold it.

## Discovery roots

| Harness | Root | Content at the baseline |
| --- | --- | --- |
| Claude Code | Plugin cache under `~/.claude/plugins/cache` | Enabled and disabled marketplace plugins, each at an immutable versioned path |
| Claude Code | `~/.claude/skills` | The tool's `codebase-memory` copy, a stale `grove` copy, account-synced skills and a dangling `terminal-browser` link |
| Claude Code | Built in | Skills that the harness supplies. The session's own skill listing is the only evidence for them. |
| Codex | `~/.codex/skills` | Grove-snapshot links, older superpowers copies, the tool's `codebase-memory` copy, a dangling `terminal-browser` link and system skills |
| Codex | `~/.agents/skills` | The same Grove-snapshot links, Quint links, the tool's `codebase-memory` copy and a dangling `terminal-browser` link |
| Codex | Installed plugin cache `~/.codex/plugins/cache` | Six `openai-bundled`, five `openai-primary-runtime` and six `openai-curated-remote` plugins. `config.toml` enables five bundled plugins and disables the other six local ones. The remote plugins have a remote install record and no `config.toml` entry. |
| Codex | Bundled and runtime marketplace sources | The catalogs that the bundled and runtime plugins install from |
| Pi | `~/.pi/agent/skills` and `extensions` | Links to `~/Development/grove`, which no longer exists, and tool extensions |

The Codex executable names the roots `codex-home-skills`, `user-agent-skills`,
`system-skills` and a plugin cache. Codex therefore reads both user roots. Its
handling of one name in both roots was not measured. The review session
observed that Codex exposes the remote `pages` plugin's `write-page` skill. The
remote plugins hold 30 skill files, 20 of them in `openai-templates`.

The coverage sweep classifies each other area that holds `SKILL.md` files.
Their layout identifies them as install sources or retained versions, not
discovery roots:

| Container | Content |
| --- | --- |
| `~/.claude/plugins/marketplaces` | Claude Code marketplace clones |
| Inactive versions in `~/.claude/plugins/cache` | Earlier Linkuistics versions and three uninstalled official plugins, each with an orphan marker |
| `~/.agents/.grove/snapshots` | Grove 22.8 snapshot store, which the Codex links target |
| `~/.codex/.tmp/plugins` | The `openai-curated` marketplace clone |
| `~/.codex/vendor_imports` | Codex's curated skill catalog |
| `~/.codex/plugins/.remote-plugin-install-staging` | One remote-install staging bundle |

## Baseline summary

The baseline has 444 entry rows.

| Class | Rows |
| --- | ---: |
| Owner pack | 137 |
| Platform-managed | 116 |
| Tool-managed | 53 |
| Dangling | 48 |
| Disabled third-party | 26 |
| Selected third-party pack | 24 |
| Stale copy | 15 |
| Capability-only | 14 |
| Not a discovery root | 11 |

No platform-managed or tool-managed skill has the name of a Ravel-delivered
skill.

These defects exist before any move:

- Each Pi skill link and the Pi technical-communication extension link dangle.
  They point at `~/Development/grove`, which the earlier checkout move left
  behind.
- `terminal-browser` links dangle in `~/.claude/skills`, `~/.codex/skills` and
  `~/.agents/skills`. The cask is absent from Homebrew.
- Both Codex user roots hold links to the same Grove snapshot entries.
- `~/.codex/skills` holds superpowers copies without provenance. Ten of them
  differ from superpowers 7.0.0, and four match it.
- `~/.claude/skills/grove` is an earlier Grove copy with retired signal files.
  Its name collides with the plugin's `grove` skill.
- Codex's snapshot copies of `technical-communication` and
  `machine-environment` differ from the Claude Code plugin's bytes.
- `guardrail` has no Codex link, as its `harnesses: [claude-code]` metadata
  directs.

## Capabilities beside the guidance

- The `linkuistics` Claude Code plugin carries the technical-communication
  hooks for six events. The same skill holds a Codex hooks plugin, but the
  owner's Codex configuration names no marketplace that offers it. Codex
  sessions therefore run without that always-on layer.
- `guardrail` installs a `PreToolUse` hook from its frontmatter and responds
  to `/guardrail`. Both need native Claude Code discovery.
- `authoring-conventions` and `guardrail` set `disable-model-invocation: true`.
- The superpowers `SessionStart` hook injects `using-superpowers` into each
  Claude Code session.
- Claude Code reads MCP entries for `quint-kb` and `codebase-memory-mcp`.
  Codex reads `node_repl`, `context7`, `openaiDeveloperDocs`, `quint-kb`,
  `computer-use` and `codebase-memory-mcp`.
- User hooks in both harnesses belong to Orca, herdr and codebase-memory-mcp.
  Each hook script names its installer. Claude Code has 21 user hooks and
  Codex has 12.
- `skillOverrides` in `~/.claude/settings.json` turns off the tool's
  `codebase-memory` skill.

## Machine contract

mise tracks `~/.claude/settings.json`, `~/.claude/hooks`, `~/.codex/config.toml`,
`~/.codex/hooks.json`, `~/.codex/AGENTS.md` and `~/.config/ravel` in place. It
does not track the skill roots, the plugin registries or `~/.pi`. The
maintained manual for this area is the Obsidian `Configuration/AI Dev Tooling`
note and its maintenance script. That note still names `~/Development/grove` as
the live Grove source.

## Pack checkouts

This section is history. The owner design now installs each source through
a Homebrew formula, as its section "Skill formulae" states.
`candidate-keg-k238` removed the script and its task `owner:checkouts`. Grove
commit `7b113a28a96621217c16154d4f44eba667236132` still holds both.

The design gave the pack checkouts to the `[bootstrap.repos]` part of mise.
The [mise repository reference](https://mise.jdx.dev/bootstrap/repos.html),
read on 2026-11-02, states these facts:

- Each entry is keyed by its checkout path, with a required `url` and an
  optional `ref`. A `ref` is a branch, a tag or a full commit ID.
- `filter`, `depth` and `submodules` apply only to the first clone. `depth`
  cannot be combined with a full commit ID.
- A branch or tag `ref` makes the status command run `git ls-remote`. A commit
  `ref` is examined locally.
- mise never removes a checkout. `unapply` leaves the directories.
- The reference does not mention jj.

The script `docs/evaluations/owner-skill-delivery/checkouts.py` measured the
behaviour that the design then stated. It built a fixture origin and a
fixture HOME, and it ran the installed mise with only isolated paths in its
environment. It moved the origin away to stand in for a machine without a
network. Each mise call named the fixture HOME with `-C`. The fixture hid the
owner's global and system Git configuration. `task owner:checkouts` ran it.

| Artifact | SHA-256 |
| --- | --- |
| `docs/evaluations/owner-skill-delivery/checkouts.py` | `349e2e233c3967f370c39174034a019cf84f2980758f4098c2827bc819cd3244` |

With mise 2026.10.25 and Git 2.56.0, all 33 cases matched. The script asserted
each observation in this table:

| Case | Observation |
| --- | --- |
| No checkout | The state is `missing`, and `status --missing` exits 1. `bootstrap plan --detailed-exitcode` exits 0 |
| `apply` with a commit `ref` | Clones the repository and checks out that commit with a detached HEAD. The state is `current`, and `status --missing` exits 0 |
| No network, `ref` unchanged | The state stays `current`. `apply` exits 0, and its verbose output shows no fetch |
| No network, `ref` moved to a commit that the clone holds | The state is `differs`, and `status --missing` exits 1. `apply` fetches first, exits 1 and leaves the checkout at its earlier commit |
| No network, `ref` moved to a commit that the clone lacks | The same result |
| Network again | `apply` moves the checkout to the new commit |
| Untracked file | The state is `dirty`. `status --missing` and `apply` exit 1 |
| File that `.gitignore` names | The state stays `current`, and `status --missing` exits 0. `git status --porcelain --ignored` lists the file |
| Another origin URL | The state is `conflict`. `status --missing` and `apply` exit 1 |
| No `ref` | `apply` clones the default branch. After the remote moves ahead, the state stays `current`. `update` moves the checkout forward |

A manual verbose run named the fetch command `git fetch --prune --tags origin`.

The `--mutate` run expects `apply` to succeed for the offline move to a held
commit. That run failed exactly that case, which shows that the comparison can
fail. The script digest was the same before and after both runs. Each run
removed its temporary directory.

Two further observations used the same fixture without the script:

- In a checkout that holds a colocated jj repository, the state stayed
  `current` and `apply` succeeded.
- The fixture configuration held two entries and a `bootstrap` task. One
  checkout had an untracked file, and the other was missing.
  `mise -C <home> bootstrap --only repos,task` exited 1 and named the local
  changes. It cloned neither checkout and ran no task. After the file was
  removed, the same run cloned the missing checkout and ran the task.

The `plan` case shows that the plan that `mac-check` runs cannot report a
missing checkout.

### Credential routes of a clone

On 2026-11-02, read-only `git config --show-origin --get-all credential.helper`
queries tested the clone environment of the staging trials. They used
Homebrew Git 2.56.0 and Apple Git 2.54.0, with an empty HOME and decoy files in
the scratch directory:

| Environment | Helper that Git reports |
| --- | --- |
| No `GIT_CONFIG_NOSYSTEM` | `osxkeychain`, from `/opt/homebrew/etc/gitconfig` or the Xcode `gitconfig` |
| `GIT_CONFIG_NOSYSTEM=1` and `GIT_TERMINAL_PROMPT=0`, with an inherited `XDG_CONFIG_HOME` | The decoy from `$XDG_CONFIG_HOME/git/config` |
| The same, with an inherited `GIT_CONFIG_GLOBAL` | The decoy from that file |
| The same, with an inherited `GIT_CONFIG_COUNT` | The decoy, with the origin `command line` |
| The allowlist of the fixture, with `GIT_CONFIG_PARAMETERS` added | The decoy, with the origin `command line` |
| The allowlist of the fixture, with `GIT_CONFIG_SYSTEM` added | None, because `GIT_CONFIG_NOSYSTEM=1` blocks it |
| The allowlist without `GIT_CONFIG_NOSYSTEM`, with `GIT_CONFIG_SYSTEM` added | The decoy from that file |
| The allowlist of the fixture | None, with exit 1 |

Both builds gave the same results. The owner's global configuration is
`~/.config/git/config`, on the XDG path, and names Git Credential Manager. The
current environment sets neither `XDG_CONFIG_HOME` nor `GIT_CONFIG_GLOBAL`.
These queries read configuration only. They show which helper Git would call,
not a call. The decoy-helper control of the staging trials observes the call.

### The released commit of `grove-core`

The v22.8.0 release commit `bacb255aba8` changed only `CHANGELOG.md`,
`Cargo.lock` and `Cargo.toml`. No release file is in a skill directory. The
tree ID of the `grove-core` source directory therefore changes between the
candidate and the released commit only if the integration of `main` changes
that directory.

### Checkout configuration and the tree ID

On 2026-11-02, `owner-pack-checkouts-k84` built a scratch repository with Git
2.56.0. Its `skills` directory held a text `SKILL.md`, a relative link and an
executable script. Two clones ran with the isolated configuration of the
fixture. The second clone added `core.autocrlf=true` and
`core.symlinks=false`:

| Clone | Tree ID of `skills` | `git status --porcelain --ignored` | Written files | Ravel `resourceDigest` |
| --- | --- | --- | --- | --- |
| Plain | `3a94b4217e46` | Empty | LF line ends, the link is a link | `f7a0ae858b65` |
| Converted | `3a94b4217e46` | Empty | CRLF line ends, the link is a regular file | `4eca6b6d32f7` |
| Plain, at another path | `3a94b4217e46` | Empty | As the first clone | `f7a0ae858b65` |

The digests came from `ravel skills resolve --json` of the local feature build
in `Ravel.skill-directory-delivery`, with a scratch HOME and an empty
environment. The build reports `ravel 1.1.0`. The first digits of its SHA-256
are `68297a769cd9`. The tree ID and the status therefore cannot tell the two
clones apart. The digest can, and it does not depend on the checkout path.

A read-only query of the owner's effective Git configuration found no
`core.autocrlf`, `core.eol`, `core.symlinks`, `core.attributesFile` or
`core.sparseCheckout`. The global file `~/.config/git/config` sets the LFS
filters. Today's owner checkouts are therefore not known to differ. The
design's check needs the digest, because the configuration can change.

### Publication by atomic exchange

On 2026-11-02, `owner-pack-checkouts-k86` called `renamex_np` through Python
on macOS 27.0.1, on an APFS volume, in a scratch directory:

| Call | Result |
| --- | --- |
| `RENAME_SWAP` of a live file and a prepared file | Exit 0. The path held the new content. The other name held the replaced content, with its original inode. |
| `RENAME_EXCL` onto an existing file | Refused with `EEXIST`. Neither file changed. |
| `RENAME_EXCL` onto an absent name | Exit 0. The file was created. |

One call can therefore publish a file of the switch set and keep the content
that it replaced. The section "Concurrent writers" of the design uses both
calls. The measurement used no file of the owner's HOME.

On the same day, `owner-switch-recovery-k87` repeated the setup with these
calls:

| Call | Result |
| --- | --- |
| A write through a descriptor that a writer opened before `RENAME_SWAP` | The write landed under the displaced name. The path did not receive it. |
| `RENAME_SWAP` of files with different modes and extended attributes | The path took the prepared file's mode and attributes. The displaced name kept the old ones. |
| `RENAME_EXCL` of a directory and of a dangling link into another directory | Exit 0 for each. The link kept its target. |
| `RENAME_EXCL` of a directory onto an existing directory | Refused with `EEXIST`. |
| `RENAME_SWAP` of two directories | Exit 0. Each name held the other directory. |

A late write therefore reaches a kept copy, and the design flags that copy.
The stage file must take the live file's metadata before the exchange.

### The recovery model

`owner-switch-recovery-k87` wrote the Quint model
[`models/owner-switch.qnt`](models/owner-switch.qnt) and its runner
[`models/run-owner-switch.sh`](models/run-owner-switch.sh). Its run of
2026-11-02 had the model digest `793ef9ef86a4`.

`owner-switch-recovery-k89` then changed the model for the findings of
`owner-switch-recovery-k88`. Outside programs and late descriptors now remove
keys, and a merged copy keeps only its merge digest as recorded content. One
layout flag stands for a changed entry outside the switch set, and step 14
compares it before the manual copy. The model has a fifth property and two
more mutants.

Three instrument changes came with it:

- A step picks a kept copy only from the copies that exist. Before, it picked
  an index from 0 to 40, and no merge happened while outside budget remained.
- The outside phase `idle` and the crash phase `receipt` serve two focused
  worlds. They hunt witnesses only.
- Two witnesses ignore a key that a later removal superseded. Without that,
  a legitimate removal would read as a loss.

The sampling therefore differs from k87's run, and no witness count compares
across the two runs.

On 2026-11-02, `task model:owner-switch` ran the changed files with Quint
0.33.1. The three subjects had the same digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `docs/models/owner-switch.qnt` | `4b3002d9d23d0ecd59202a7ed04e532c90dfd384d45d8055724305bea6b39840` |
| `docs/models/run-owner-switch.sh` | `66c9fcfb6065e602ba5e523f86eabc37937c226543e425f4896a214973c7596d` |
| `Taskfile.yml` | `2aec4fba957f23f586498606734917daa4c12410dedc788010f3023b9c65b69a` |

Each world ran 4,000 samples of up to 60 steps. The classification world ran
12,000, and the unchecked-receipt mutant ran 20,000. All ten invariants held
in each of the nine design worlds. Each mutant witness stayed unreached
there. These are the witness counts:

| World | Witness | Traces |
| --- | --- | --- |
| `crashes` | A complete switch, each after a crash | 1,778 |
| `crashes` | An interruption resumed to native | 2,962 |
| `crashes` | A failed probe inverted to native | 536 |
| `crashes` | The owner's inverse to native | 2,135 |
| `writers` | A complete switch with no crash | 1,585 |
| `writers` | An exchange conflict in a flagged copy | 92 |
| `writers` | A late write in a flagged copy | 113 |
| `writers` | A merged copy, then a complete switch | 96 |
| `writers` | A removal displaced into a flagged copy | 18 |
| `classify_world` | A second plan switched to complete | 6,176 |
| `coincidence` | A resumed inverse resets an outside switched value | 187 |
| `after_receipt` | Drift after a receipt | 1,859 |
| `during_run` | A conflict at a cell | 58 |
| `merge_return` | A merged copy returned to the content that the run read, and flagged | 48 |
| `entries` | The complete continuation copied its receipt | 523 |
| `entries` | The complete continuation found an outside entry, inverted, and stopped for classification | 448 |

Each mutant reached its violation:

| Mutant | Traces |
| --- | --- |
| No recorded digest of the displaced content | 175 of 4,000 |
| No run record in the state | 1,327 of 4,000 |
| "Inverse ended" before the inverse | 4,000 of 4,000 |
| No cell reading before the receipt | 155 of 20,000 |
| No rehearsal check | 1,234 of 4,000 |
| An inverse that resets a third value | 91 of 4,000 |
| No merge authority over a merged copy | 44 of 4,000 |
| No comparison in step 14 | 61 of 4,000 |

The sweep of every invariant against every mutant found violations only of
the properties that the table in the design names. The mutant without a run
record also broke the state table and the settled-state property, because it
reads states by another rule.

The results are sampled, not proved. The shortest path to a receipt has 15
steps, and the manual copy is the 16th, inside the 60-step bound.

#### The preludes in step 3

`native-settings-recovery-k221` then changed the model for findings F1 and F2
of `homebrew-delivery-design-k219`. Each run now enters through steps 1 to 3.
The gate skips the preludes while a run is open, and the two prelude cells
can be absent. The native settings write a settings record before their
exchange. The model has a sixth property, four more mutants and four more
design worlds.

Four instrument changes came with it:

- Each instance states where its first state lies: before the first entry's
  preludes, or where they leave the layout.
- The stop at the table acts only where no row continues.
- The outside phase `run` excludes the preludes again. The outside phase
  `prelude` and the crash phase `probe` are new.
- `wit_driftedDuringRun` stays only in `during_run`, and `both_preludes`
  carries the exchange conflict of `both`.

The sampling therefore differs from the run of 2026-11-02, and no witness count
compares across the two runs.

On 2026-11-06, `task model:owner-switch` ran the changed files with Quint
0.33.1. The three subjects had the same digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `docs/models/owner-switch.qnt` | `7df7fc51b9b81ca889e50d77f59c8aed370553c0b91f584910cf062f99c5974c` |
| `docs/models/run-owner-switch.sh` | `ff25f882894444b2f2d93438516ab501a1aeb766b4c6d1834d65f19840d17f3e` |
| `Taskfile.yml` | `05c3080b84d0443eabe075e83a4c01ce3d60a27e2ec4409e1e1ec93ee9d386f1` |

Each world ran 4,000 samples of up to 60 steps. The classification world and
the mutant that resets a third value ran 12,000, and the unchecked-receipt
mutant ran 20,000. All twelve invariants held in each of the thirteen design
worlds. Each mutant witness stayed unreached there. These are the counts of
the new and moved witnesses:

| World | Witness | Traces |
| --- | --- | --- |
| `native_settings` | A write before the native settings' exchange in a flagged copy of a settings record | 665 |
| `native_settings` | A late descriptor write in that copy, flagged | 1,134 |
| `native_settings` | A crash between that exchange and its comparison, with the copy flagged | 315 |
| `native_settings` | A merged copy, then a complete switch | 1,258 |
| `active_entry` | A crash while the probes ran, then an entry that resumed and completed | 1,476 |
| `prelude_crashes` | A crash in the native settings, then the native state at the table | 3,725 |
| `prelude_crashes` | A crash before the hooks installation, then the plugin installed and the native state | 561 |
| `prelude_crashes` | An interrupted installation disabled by the next entry | 299 |
| `entries` | The complete continuation through the gate copied its receipt | 534 |
| `both_preludes` | An exchange conflict in a flagged copy | 34 |
| `during_run` | Drift during a run | 2,681 |

Each mutant reached its violation:

| Mutant | Traces |
| --- | --- |
| No recorded digest of the displaced content | 940 of 4,000 |
| No run record in the state | 445 of 4,000 |
| "Inverse ended" before the inverse | 3,966 of 4,000 |
| No cell reading before the receipt | 161 of 20,000 |
| No rehearsal check | 503 of 4,000 |
| An inverse that resets a third value | 28 of 12,000 |
| No merge authority over a merged copy | 46 of 4,000 |
| No comparison in step 14 | 69 of 4,000 |
| No gate of the preludes | 1,996 of 4,000 |
| No settings record | 981 of 4,000 |
| No hooks installation after the first entry | 3,756 of 4,000 |
| No disabling of an enabled plugin | 117 of 4,000 |

The sweep of every invariant against every mutant found violations only of
the properties that the table in the design names, with two known extras. The
mutant without a run record also broke the state table and the settled-state
property. The mutant that writes "inverse ended" early also broke property 6.
It reads an open run as closed, so a prelude meets a value that its own write
left.

The rarest witness was a removal displaced into a flagged copy, in 12 of
4,000 traces of `writers`. From a first state before the preludes, the
shortest path to a receipt has 21 steps.

#### The held link of the hooks installation

`hooks-installation-writes-k222` then changed the model for Claude Code's own
writes. The hooks installation's command now reads the file and later renames
a new file onto the path, under a held link. The disabling is an exchange
after a settings record. An outside write to the file of part `HOOKS` lands
in place or by a rename.

The model has a second accepted case, one more mutant and one more design
world. That world, `hooks_commands`, runs 12,000 samples. The added steps change the sampling, so no witness count compares
with the run above.

On 2026-11-06, `task model:owner-switch` ran the changed files with Quint
0.33.1 in 6 minutes 8 seconds. The three subjects had the same digests before
and after the run:

| Subject | SHA-256 |
| --- | --- |
| `docs/models/owner-switch.qnt` | `a1fea03e76cebdec7cd115e002c851a128c9efc566195e0eca250e301331cb69` |
| `docs/models/run-owner-switch.sh` | `0fe7ef316e63ce1438b8d4a3193af35e09f6c2e9d99b699b1f9aa4ff0c71c2e1` |
| `Taskfile.yml` | `05c3080b84d0443eabe075e83a4c01ce3d60a27e2ec4409e1e1ec93ee9d386f1` |

All twelve invariants held in each of the fourteen design worlds. Each mutant
witness stayed unreached there, the new one included. The runner printed 241
passes and no failure. These are the counts of the new witnesses:

| World | Witness | Traces |
| --- | --- | --- |
| `hooks_commands` | A write in place inside a command's interval, in a flagged held copy | 32 of 12,000 |
| `hooks_commands` | A late descriptor write in a held copy, flagged | 42 of 12,000 |
| `hooks_commands` | The accepted case: a write lost after another program's rename during a command | 34 of 12,000 |
| `hooks_commands` | A crash between a held link and the rename, then the native state after the link's erasure | 415 of 12,000 |

The mutant without the held link reached its violation in 13 of 4,000 traces.
The sweep found that it violates only property 1. Each other mutant violated
the same properties as in the run above.

The rarest witness was that mutant's. The rarest design witness was a removal
displaced into a flagged copy, in 21 of 4,000 traces of `writers`. From a
first state before the preludes, the shortest path to a receipt now has 26
steps.

#### The comment of the review's integration

`native-settings-recovery-k224` changed one comment of the model, in its
list of what the model leaves to prose. No definition changed. On
2026-11-06, `task model:owner-switch` ran the changed file with Quint 0.33.1
in 6 minutes 5 seconds. The runner printed 241 passes and no failure. The
three subjects had the same digests before and after the run:

| Subject | SHA-256 |
| --- | --- |
| `docs/models/owner-switch.qnt` | `c90bbf9ab658518753630d8cd9226cd926f0ac1eaf6e70c5f200c934e60412c4` |
| `docs/models/run-owner-switch.sh` | `0fe7ef316e63ce1438b8d4a3193af35e09f6c2e9d99b699b1f9aa4ff0c71c2e1` |
| `Taskfile.yml` | `05c3080b84d0443eabe075e83a4c01ce3d60a27e2ec4409e1e1ec93ee9d386f1` |

## Not measured

- Claude Code's treatment of a root-level `SKILL.md` inside a skill-free
  plugin.
- The effect of disabling a plugin on a Claude Code session that is already
  running. The loading reference says that the session keeps its loaded set,
  but no trial observed it.
- Codex sign-in from a separate `CODEX_HOME` that reuses only the
  authentication file, including a token refresh.
- Which tree Codex reads for an enabled bundled plugin: the installed cache or
  the marketplace source. The baseline records both.
- Whether Claude Code under the owner policy reads snapshot paths outside its
  working directory without a permission prompt.
- Codex's handling of one skill name in both user roots.
- Pi's startup with a dangling extension link.
- Built-in Claude Code skills on disk. The baseline lists them from the
  session's skill listing.
- `mise bootstrap repos apply` against the real remotes.
- A decoy credential helper against a remote that asks for a credential.
- The size and capture time of a TestAnyware checkout subdirectory.
- Ravel's capture of a pack checkout that mise wrote.

The staging trials in the design's cutover contract own each of these
measurements before the switch.

The rehearsal of order 6 owns two more:

- Ravel 1.1 and Ravel 2.0 with a settings file that holds no key. The
  inverse leaves that file when the switch created it.
- An exchange and an exclusive rename between each live path of the switch
  set and the backup. Both calls need one volume.

Two measurements need the real HOME or a new machine, so no staging trial can
own them:

- A complete `mise bootstrap` run with repository entries. Packages and
  launchd agents act for the whole machine or account, not for one HOME.
- An adoption with repository entries on a new machine. The new-machine
  runbook in the maintained manual owns it.

## Review corrections

The focused review `owner-ravel-skill-delivery-k41` found three instrument
defects. Its integration `owner-ravel-skill-delivery-k48` repaired them and
regenerated the baseline.

- **Omitted Codex plugin cache.** The reviewed instrument read the Codex
  marketplace sources but not `~/.codex/plugins/cache`. The repair walks every
  marketplace, plugin and cached version there. A plugin in an unknown
  marketplace is `UNCLASSIFIED`. The coverage sweep reports any other
  `SKILL.md` as `UNCOVERED`.
- **Hidden hooks and capabilities.** The reviewed instrument kept one hook row
  for each event and guessed owner, without a digest. It hid three
  codebase-memory-mcp `SessionStart` hooks in Claude Code. MCP and agent rows
  held names only, tool rows held static text, and the tree digest left out
  executable bits and directory links. The repair gives every class the change
  identity above.
- **Static tool literals.** Tool rows now come from the installed paths,
  `Info.plist` and the Quint clone's revision. The codebase-memory-mcp binary
  has no versioned path, so its digest is its identity.

Per-entry comparison with the reviewed baseline found no added, removed or
reclassified skill row outside the classes above. The 30 earlier hook rows
became 33 rows with one hook each. The new rows are 54 installed Codex plugin
rows, one skill-override row and 11 coverage rows.

Every digest changed, because its definition changed. No class or disposition
changed. Other column
changes are the `orphan=no` mark and a file count on 26 Claude Code plugin rows,
and the renamed form of 15 marketplace-source rows. Six agent rows and four
tool rows gained a file count. Five tool rows and 14 capability notes have
fresh text.

The focused review `owner-pack-checkouts-k81` found one P1 and five P2 defects
in the pack-checkout revision. Its integration `owner-pack-checkouts-k82`
repaired the specification and its views:

- **An advance rule that excluded the switch.** The rule now forbids a capture
  of a changing checkout, not a running launch. `ravel skills resolve` is also
  a capture. A Ravel 1.1 launch and settings without a `skills` key capture
  nothing.
- **A weak clone environment.** The specification now gives the allowlist of
  the fixture and requires a decoy-helper control. The table under
  [Credential routes of a clone](#credential-routes-of-a-clone) records the
  routes.
- **No continuation after a failed apply.** Every stop before the native
  change keeps native delivery working. Finish writes a pending-activation
  record and runs the switch again. The owner can run it again after finish,
  with no release, promotion or teardown.
- **A core pin chosen after its verification.** Staging records the tree ID of
  the accepted `grove-core` candidate. The switch compares it with the released
  commit before any change. A probe and a staging trial read a Grove kind
  skill.
- **Clones after the isolation trial.** The credential control and the clones
  now precede the first harness launch.
- **A root-brief handoff that named a tool.** The integration leaf holds the
  repaired text for the replanning leaf.

One in-session reviewer then read the repaired text and found nine defects.
The integration repaired each of them:

- The `grove-core` candidate is the last core change of the line, and rollout
  verifies its tree ID before finish. A failed core trial after the release
  leads to a Grove patch release.
- A failure in the native change and an inverse after a failed probe share
  the continuation of a stop.
- Probes and core trials run outside every Grove workspace, without Grove's
  loop control variables. They cannot record finish's teardown.
- Every `apply` and every complete `mise bootstrap` counts as an advance. A
  process check precedes it.
- Each case of the credential control uses the allowlist of a fixture HOME as
  its base, adds one route and clones through mise.
- Smaller repairs define the owner-run switch and compare every pin with its
  accepted pin. They also move the draft check after the clones and forbid Git
  attributes that change the bytes of a checkout.

The focused review `owner-pack-checkouts-k83` found three P2 defects in that
repair. Its integration `owner-pack-checkouts-k84` repaired the specification
and its views:

- **An inverse that could not fail.** A rerun of the switch now reads the
  state of the switch set from the layout and the latest backup. A partial
  state resumes the inverse, and the native probe must pass before a retry.
  Each backup is new, and the pending-activation record names each backup.
- **Activation without its receipt.** The active state reruns the checks,
  the probes and the receipt, and it changes no item. The owner's run after
  finish is now named beside finish as a run against the real HOME.
- **A tree ID taken as the captured files.** The accepted identity of each
  source is its Ravel `resourceDigest`. The section
  [Checkout configuration and the tree ID](#checkout-configuration-and-the-tree-id)
  records why. Step 6 compares each real checkout with its accepted digest
  in a disposable HOME, and a probe compares it again.

One in-session reviewer then read the repaired resumption and found two P1,
seven P2 and two P3 defects. The integration repaired each of them:

- A backup is complete only after its final rename. Step 3 keeps an
  incomplete backup aside.
- A file that other programs also write is compared only at the keys that the
  switch changes. A drifted item goes to the owner's classification, then to
  a new native backup.
- Step 9 writes each file once by a rename, after a comparison with its copy.
  Step 1 also lists the programs that write those files.
- The absence of the Ravel settings file is its earlier state. An entry at
  both paths is in neither state.
- Step 4 reads the latest Grove release, so a repair's patch release reaches
  the next run.
- In the active state, a stop in step 6 counts as a failed probe.
- The section "Rollback" defines the native probe.
- `mac-check` runs before the receipt, which goes into the backup first. The
  backup lies outside each tree that the instrument sweeps.
- The coordinated recovery rows name each state and keep a Grove patch
  release apart from a Ravel cut.

The focused review `owner-pack-checkouts-k85` found one P1 and three P2
defects in that repair. Its integration `owner-pack-checkouts-k86` repaired
the specification and its views:

- **A comparison that cannot stop a writer.** Step 9 compared a file and
  then renamed it, so a write between the two was lost. Step 1 now lists the
  jobs that can launch a writer and counts the descendants of finish. One
  prepared command runs each write interval. An atomic exchange publishes
  each file, and the run compares the displaced content. A difference fails
  the part and keeps the update in the backup. The section
  [Publication by atomic exchange](#publication-by-atomic-exchange) records
  the call.
- **A receipt that outlived its layout.** The layout now decides the state,
  and a receipt only tells the complete state from the active one. Step 3
  and the inverse withdraw a receipt that no longer applies.
- **A moved entry identified by its path.** The state of a moved entry now
  also needs its recorded change identity. A changed entry or a retargeted
  link at either path is drifted.
- **A changed plan without a rehearsal.** Each rehearsal records the digests
  of its manifest and inverse. A changed plan reuses that evidence only for
  items of a rehearsed kind. Any other change needs a new rehearsal before
  step 8.

The in-session reviewer of `owner-pack-checkouts-k86` found three P1 and nine
P2 defects, R1 to R12, and eight P3 items. k86 judged them a redesign, and
`owner-switch-recovery-k87` rebuilt the recovery model with the model above.
Each item has this disposition:

| Item | Disposition |
| --- | --- |
| R1: one process check before the probes | Closed. The program lists the processes before each write interval, and no probe session may still run. Detection protects writes in any case. |
| R2: an exemption by absence | Closed. No exemption remains. The run's own process tree may write, and detection covers it. The staging descendant trial is gone. |
| R3: no exclusion for the owner's inverse | Closed. The owner's inverse runs the program copy, with the same listing and detection. |
| R4: a drifted file blocks the native return | Closed. The inverse resets each switched cell and leaves only third values. |
| R5: the Ravel settings compared whole | Closed. Only the `skills` key is a cell. |
| R6: a receipt without a second layout check | Closed. Step 13 reads each cell again and records the probed values. "Complete" also needs them now. |
| R7: states ignore a changed manifest | Closed. The state uses the open run's plan or the prepared plan. A changed entry stops the active state before its receipt. |
| R8: a classification leaves no record | Closed. A classification makes a new plan with the accepted values. |
| R9: an interruption looks like an outside write | Closed, with one accepted case. The run record and the prefix rule separate them. An outside write of a switched value during an open run is reset and reported. |
| R10: a digest record cannot decide reuse | Closed. A rehearsal names the plan and the program, and any change needs a new rehearsal. |
| R11: a rehearsal inside finish | Kept from k86. Finish stops on an unrehearsed plan, and the owner rehearses after finish. |
| R12: no withdrawal after a successful finish | Closed. No withdrawal record exists. The layout decides the state, and `mac-check` reports it. The manual's receipt applies only while `mac-check` reports the switch complete. |
| P3: mode and attributes lost | Closed. The stage file takes the live metadata, and the digest covers it. |
| P3: a stage file under the displaced name | Closed. A copy that holds its stage content holds no outside write. |
| P3: a failed verification | Closed. A failed final reading is a conflict, and the run inverts its parts. |
| P3: "steps 1 to 8 change nothing" | Closed. The text names the resumed inverse and the pin advance. |
| P3: "byte for byte" against the fault points | Closed. Order 6 now requires each cell value and each outside write to survive. |
| P3: step 1 or step 2 stops | Closed. The seams say step 2. |
| P3: drift seams need a backup | Closed. Before the switch, a changed entry gives the changed state. |
| P3: the switch's own shell | Closed. The run's own process tree includes the program's shell. |

The focused review `owner-switch-recovery-k88` found one P1, three P2 and
one P3 defect in that redesign. Its integration `owner-switch-recovery-k89`
repaired the specification, the model and the views:

- **A merged copy that returned to older content.** The reviewed predicate
  tested the content that the run read before the merge record. A copy that
  a descriptor returned to that content was not flagged. The merge digest is
  now the only recorded content of a merged copy, and the owner's merge
  applies the copy's difference, a removed key included. The model reaches
  the trace, and its mutant without the rule loses the removal.
- **Descendant readers inside the exclusion.** Step 1 exempted the whole
  descendant tree, so a delegate session could read a moved path. Step 1 now
  excludes only the kept processes: the harness process, its driver chain,
  its MCP servers and the program. Each other descendant counts as another
  process. The k87 text had also dropped the driver chain, which k86 named.
  A reader control tests the exclusion apart from detection.
- **A complete continuation without the outside comparison.** A run that
  stopped between steps 13 and 14 copied its receipt without a comparison
  of the entries outside the switch set. Step 14 now compares them first,
  and a changed entry counts as a failed probe. The model's fifth property
  and its mutant cover the rule.
- **An inverse onto an occupied path.** An exclusive rename cannot move a
  retained entry onto an occupied original path. The inverse now moves an
  entry only to an absent path. An occupied path is a third value, and the
  inverse leaves both entries. The design argues the entry case in full in
  place of a model extension.
- **Shared node identities in the cutover view.** The recovery nodes reused
  two identities of preparation nodes. They now have their own identities.

One in-session reviewer then read the argument for entry parts and found
four P1, two P2 and three P3 items. The integration repaired five of them:

- The inverse moves an entry of any change identity back. The inverse
  changes no other third value, and the cell rules now name that exception.
- The switch compares a moved entry with its recorded change identity, which
  the baseline instrument defines.
- Purge requires each retained entry to keep that change identity.
- The retained path belongs to the open run's backup. While no run is open,
  an entry cell is the original path alone.
- The drifted row names a receipt as the last record, as the model does.

The claim that an occupied path cannot repeat a failure is now limited to
that case. Another failed rename stops each run until the owner repairs it.
A parent path that another program replaced with a link was noise, because
the entry still lands at its original path. The resume rule for a moved-back
entry is the design's stated trade-off: a third value waits for the owner.

Row R2 of the table above records the whole process tree as exempt. The
repair of the descendant readers narrows that exemption to the kept
processes.

<!-- tc: off because this section preserves the completed review's findings as history -->

## Owner switch implementation review

`owner-switch-program-k242` inspected producer commit `7d69e659`, from base `27723524`. These findings are pending integration triage; they are not repairs or new test results.

### F1 — P1: gates can read a plan other than the rehearsed plan

`scripts/owner_switch.py:838-845`, `1426-1433`, `1853-1854` and `1900-1904`.
The controller loads the prepared plan once and keeps its bytes and digest.
It then passes the original pathname to each command. Each gate opens that
pathname again. Step 8 checks the digest of the controller's earlier bytes,
and the backup receives those earlier bytes, not what the gates read.

For example, start with a rehearsed plan A. After its initial load, change
the on-disk accepted digest of `superpowers` to the digest of a modified
installed source. Step 6 reads that new value and passes. Step 8 still
admits A's rehearsal, although A would have rejected that source. The
receipt can therefore certify a source that the rehearsed plan did not
accept. A file edit, or a plan command that edits its own plan, suffices.
The rule in "The rehearsed plan" requires a changed plan to have its own
passing record. Every gate needs the same admitted bytes, or a checked
binding to their digest; reopening an unchecked pathname does not supply it.

### F2 — P1: descendants of kept MCP servers bypass the process gate

`scripts/owner_switch.py:805-819` exempts every descendant of each matching
MCP server, as well as every descendant of the switch process. The contract
keeps the MCP servers themselves and counts each other session descendant
as another process. These are different sets.

A kept MCP server can have a running tool child that launches Codex, Grove
or a background reader of a link that step 9 moves. Even when the child's
arguments match `watch`, the `pid in kept` branch skips it. Step 9 or an
inverse can then open its write interval while that reader still runs.
The current delegate and background controls launch children of the fixture
harness, not children of a kept MCP server, so they do not cover this case.
This breaks "Concurrent writers", whose exclusion protects those readers
independently of the kept-copy mechanism.

### F3 — P1: probe and trial environments can select another harness HOME

`scripts/owner_switch.py:838-847`, `867-871`, `1215-1220` and `1566-1570`.
The five Claude Code location variables are removed only by `claude_env`,
which the installation prelude uses. Plan probes, native probes and the
step-4 trials instead inherit them through `command_env`. They also inherit
`CODEX_HOME` unchanged.

For example, run with `CODEX_HOME=/Users/antony/.codex` and a disposable
`--home`, or with `CLAUDE_CONFIG_DIR` naming an earlier staging directory.
A direct harness command in `probes` can then inspect or change that
directory instead of the switch's HOME. In the real switch it can report a
pass from the staging configuration while the real configuration fails.
This breaks the effective-directory and environment rules in "Settings
isolation and the staging HOME" and the real-HOME probes of step 10.
`Case.run` deliberately sets `CLAUDE_CONFIG_DIR`, but its fixture probe
reads JSON through `OWNER_SWITCH_HOME` and never tests a harness's effective
configuration directory, so the rehearsal misses this escape.

### F4 — P1: plan paths can escape a disposable HOME

`scripts/owner_switch.py:428-440` and `447-457`. The loader validates only
the schema token and entry ordering. `Layout` then joins plan paths to HOME
without validating that they are relative or remain inside it. An absolute
path discards HOME; `..` and symlinked parent directories can escape it too.

A plan entry whose path is `/Users/antony/.codex/skills/example` will name
the real entry even under `--home /tmp/rehearsal/home`. Given the matching
identity and an admitting record, step 9 moves that real entry into the
disposable backup. A `manual` path can similarly append a receipt outside
the selected HOME. The documented plan format says these paths are
HOME-relative, and the node's contract says disposable rehearsal leaves the
real HOME unchanged. Neither property is enforced. This concerns the
program's data paths, separately from the explicitly executable commands
that a plan author is already trusted to supply.

### F5 — P1: an incomplete plan can report complete without required checks

`scripts/owner_switch.py:1832-1837`, `1893-1904` and `1757-1787`. Missing
gate lists and probe lists default to empty lists, a missing baseline does
nothing, and a missing `mac_check` becomes `None`. The loader does not
require these fields or require the gate commands to invoke the built-in
checks. An empty command list is treated as a successful step.

For example, a plan with `gates: {}`, `probes: []` and no `mac_check`, whose
parts match the native layout and whose digest has an admitting record,
can exchange its parts and write `result: complete`. No installed-release,
formula, source or reader check has run, and neither harness was probed.
A rehearsal record proves that bytes were exercised; it does not make an
omitted obligation present. The fixed steps 4 to 7 and 10 to 13 of "The
switch inside finish" cannot be optional fields of a plan that the program
accepts as a complete switch. The fixture always supplies these fields,
so its passing path does not establish rejection of this input.

### F6 — P2: step 4 reruns staging trials in the switch HOME

`scripts/owner_switch.py:1566-1570` runs each trial with the inherited HOME
and `cwd=gate.home`. Under `run`, that HOME is the target of the switch;
inside finish it is the real HOME. The gate carries no retained staging
HOME and does not select one for these trials.

When Grove's installed `design` source differs by one byte, a trial that
launches its harness using HOME will therefore read the native real-HOME
configuration before step 9, instead of the retained staging configuration.
It can fail for the wrong reason or modify the real harness configuration
as preparation for its trial. Step 4 explicitly requires these reruns in
the retained staging HOME, under the isolation rules. A command wrapper
could supply that HOME, but neither the command contract nor the gate
requires or checks one. The fixture trial only appends a source and digest
to a log and returns 0, so it cannot detect the wrong subject.

### F7 — P2: reader evidence can become invalid before success

`scripts/owner_switch.py:1631-1667`, `1757-1787` and `1798-1818`.
Step 7 checks registrations and `.orphaned_at` once. The program never
checks those paths again before the receipt, and the active continuation
reruns only gate 6. The fixture plan's outside manifest does not include
the Grove or official-plugin reader cache directories.

For example, let gate 7 pass for the registered official-plugin directory.
An update after that gate, or between an active interruption and its
continuation, can orphan that directory and move its registration. The
switched cells and the manifest can still match, so the run copies a receipt
whose reader evidence describes the earlier registration and no marker.
The same case applies to a named directory that finish read. "Resource
lifetime" requires the versions that live readers use to remain registered
and current, and freezes updates of every plugin finish loaded. Gate 7 does
not verify that freeze or detect its violation during the remaining run.
The existing orphan control adds its marker before the first gate only.

### F8 — P2: the success receipt omits required activation and recovery evidence

`scripts/owner_switch.py:1783-1787`, `1798-1818` and `1904`.
The receipt consists of plan/program digests, probe results, `mac_check`,
cells and gate results. Step 5's formula evidence is discarded when the
backup is written. No code adds the digest of `config.rhai`, a digest of
the activated settings file, the other formula versions, the backup
locations or the retained byte sets and their purge conditions.

Consequently even the ordinary successful fixture run produces a manual
receipt without that information. Its plan digest and earlier gate output
do not record the actual activated configuration or give the owner a
complete cleanup inventory. The final paragraph of "The switch inside
finish" explicitly requires these fields. The receipt control checks only
the presence of gates 4, 6 and 7, so it accepts this incomplete evidence.

### F9 — P2: a full passing rehearsal need not measure Claude Code's writes

`scripts/owner_switch_rehearsal.py:1818-1837` and `Taskfile.yml:414-419`.
The normal task leaves `--settings-writes` off. `cmd_rehearse` treats a
missing rerun as passing and writes `passed: true` with
`settings_writes: null`. The switch's `passing_rehearsals` accepts that
record without any measured Claude Code version or write method.

For example, upgrade Claude Code to a version whose installation writes in
place, then run the default complete fixture rehearsal. Its stub still
renames like 2.1.319, and the resulting record admits the program's
preludes. The real command is rejected only after it has already written
in place; the held inode then supplies no saved earlier bytes. "Test
seams" requires the rehearsal to rerun `settings-writes.sh` with the
version it uses and record that version, or revisit the accepted case.
The producer's separately recorded measurement is useful evidence but is
not a requirement enforced by a newly passing record.

### Review evidence and limits

The graph project was
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-07T09:15:24Z`. Both pages of the bounded function search were read.
Exact source and graph snippets support the findings; an outbound trace of
`run_switch` supplied call candidates, with source controlling where the
graph attached an unrelated Rust `get` method. Coverage metadata matched
both Python files, the Taskfile and the specification, with no recorded
gap. The model's coverage metadata was not tracked, so its source was read
directly. This is task-directed verification, not a claim of graph
completeness.

The state classifier follows the six rows on the inspected inputs. In
particular, an open run skips the preludes, an active continuation obtains
admitted digests from its own backup's step-4 evidence, and a differing
program digest causes execution of the backup's program before the state
continuation. The model explicitly leaves readers, executable program
identity, real probes and metadata outside its checked abstraction; its
recorded green result does not establish those properties of Python.

No unsupported tool-response finding is asserted: Homebrew's revision is
included in the installed version comparison, mise must report `installed`,
and a Ravel document whose status is not `resolved` is rejected. The
findings above concern concrete unchecked inputs and transitions rather
than an assumed later tool format.

## Owner switch review integration

`owner-switch-program-k243` triaged the findings F1 to F9 above against the
program at `7d69e659`. A reading of the source reproduced each finding. Each
one is a real issue, and each repair stays inside the program, its rehearsal
and the existing rules of the specification. No repair changes a recovery
transition, so the model stays unchanged.

| Finding | Disposition | Repair | Guarding control |
| --- | --- | --- | --- |
| F1 | Real | A gate that the program launches stops unless the plan at `OWNER_SWITCH_PLAN` has the digest `OWNER_SWITCH_PLAN_DIGEST` | `plan-changed-after-load` |
| F2 | Real | The process gate keeps only a chain of processes that match the MCP patterns. Each other descendant of a server counts | `mcp-server-child`, `mcp-server-kept` |
| F3 | Real | Each command of a plan drops Claude Code's five location variables, `CODEX_HOME` and each `RAVEL_*` name | Each fixture command refuses them |
| F4 | Real | Each plan path is relative, normalised and free of `..`. Its parent folder must resolve inside HOME | `plan-paths-escape` |
| F5 | Real | The loader requires every field. Each step that runs commands needs one, and each gate list names the program's gate | `plan-incomplete` |
| F6 | Real | Each trial runs with `HOME` set to the plan's staging HOME and `CODEX_HOME` set to its `.codex` tree | `staging-home-missing`, the trial's own refusal |
| F7 | Real | Inside the probes, the run reads step 7's reader evidence again. A change counts as a failed probe | `reader-orphaned-in-probes`, `reader-moved-before-continuation` |
| F8 | Real | The receipt holds the activated configuration, each source, the trials, the readers, the backup and each retained byte set | `receipt-evidence` |
| F9 | Real | A passing record needs a passing rerun of `settings-writes.sh` and its version. The hooks installation runs a Claude Code command only with that version | `rehearsal-without-settings-writes`, `claude-version-differs` |

The model abstracts each probe as a pass or a failure. The reader rule of F7
therefore adds a cause of failure and no transition. Exclusion, the
environment and the receipt's fields are outside the model, as before.

Each repair has a mutant of the repaired program that lacks only that
repair. Each mutant failed its guarding control, and the positive control
`mcp-server-kept` passed against the F2 mutant. The F1 mutant completed the
switch with the changed source, which is the case that F1 states.

## Owner switch entries

`owner-switch-entries-k92` added four entries to the program, a switch row
to the owner's `mac-check` and their rehearsal controls. The entries are the
owner's inverse, the merge of a flagged copy, the purge check and the
pending-activation record. The specification's sections "Concurrent
writers", "Rollback", "Purge" and "Test seams" state each rule.

No recovery rule changed, so the model stayed unchanged. The merge follows
`ownerMerge` of the model: it applies the copy's difference from its last
recorded content, and a cell takes only a third value. The owner's inverse
is the inverse of "Rollback". The purge check, the run log and the
pending-activation record are outside the model, because no state finding
reads them.

Each record now keeps the bytes that its reading read. A merge needs those
bytes, because a digest cannot give a difference.

| Mechanism | Guarding control | Mutant that lacks it |
| --- | --- | --- |
| A merge removes a key that the copy lost | `merge-difference`, `merge-return-through-descriptor` | Failed both |
| A merge digest is the only recorded content of the copy | `merge-return-through-descriptor` | Failed |
| A cell takes only a value that no run writes there | `merge-third-cell` | Failed |
| The inverse leaves an occupied original path | `owner-inverse-occupied-path` | Failed |
| The owner's inverse runs the program copy of the latest backup | `owner-inverse-runs-backup-copy` | Failed |
| The record holds the mise status | `pending-after-step` | Failed |
| A receipt names each record that it replaces | `pending-after-inverse` | Failed |
| The purge check compares each retained entry's identity | `purge-check-retained-changed` | Failed |
| A stopped run keeps its command evidence | `pending-after-step` | Failed |
| `mac-check` reports a flagged copy | The three `mac-check-flagged-*` controls | Failed all three. Passed `mac-check-native` and `mac-check-complete` |

The `mac-check` controls run the owner's real command with HOME set to a
disposable HOME. That HOME links the checker's folder and holds copies of
the plan and the program in `<state>/prepared`. The checker's own tests in
`~/.local/lib/machine-config/tests` cover each state's fixed message, a path
that is not safe to print and a failing program.

On 2026-11-07, `task owner:switch:rehearse` passed 216 of 216 controls with
Python 3.15.1, on program `885171751f02` and plan `a1d5f409abbf`. Its rerun
of `settings-writes.sh` passed with Claude Code 2.1.319, so the record
passed. Each subject file was last changed before the run started. `task
model:owner-switch` reported "all claims hold". The model, its runner and
the Taskfile had the same digests before and after that run.

<!-- tc: on -->

## Re-inventory before the cutover

`owner-ravel-skill-inventory-k38` performed order 3 of the cutover contract
on 2026-11-07. It reran the baseline instrument in the real HOME and
compared each entry with [the baseline](owner-skill-inventory.tsv). The
instrument, the controls script and the baseline had the SHA-256 values of
"Method". Claude Code was 2.1.319 and Codex was 0.167.0.

### Controls of the rerun

- Two runs gave byte-identical output of 447 rows, with the SHA-256
  `b4f7d65ced0c…`. No row said `UNCLASSIFIED`, `UNASSIGNED`, `UNCOVERED` or
  `NEEDS-REVIEW`.
- A scratch root held an unknown skill and a dangling link. With
  `CONTROL_ROOT` set to it, the output held one `UNCLASSIFIED` row and one
  dangling row for that root.
- `task owner:inventory:controls` passed 14 of 14 controls.

The comparison keyed each row by its harness, location and entry. It
compared every field of the rows with the same key, and it listed each key
that only one side held.

### Differences and their reasons

No owner-pack, third-party-pack, stale-copy or dangling skill row changed.
Each one keeps the change identity that the section "Owner-pack entries of
the baseline" of the [moves record](repository-moves-record.md) maps to its
source and root. No new row has an owner-controlled class. The rows that
differ are these:

| Rows | Class | Difference | Reason |
| --- | --- | --- | --- |
| `grove`, `linkuistics` and `testanyware` of the `linkuistics` marketplace. `superpowers` and `quint-llm-kit`. `commit-commands`, `explanatory-output-style`, `feature-dev` and `swift-lsp` | Owner pack, third-party pack and capability-only | The digest only. The version and the file count stay the same | Each version directory holds `.in_use/<pid>`, and the PID is that of the running Claude Code session. The baseline held the marker of an earlier session. Without that folder, each count is one less, as in the baseline |
| `context7` and `playwright` | Capability-only | Version `a7ba5b93db0c` became `0412be76f50a` | The official marketplace updated them automatically |
| `github`, `serena`, `skill-creator` and `greptile` | Disabled third-party | New versions. `greptile` 1.3.1 adds the skills `login` and `review` | The same automatic updates. Each plugin stays disabled, so its skills stay outside the active collection |
| Five account-synced skills, `openai-docs` and the Codex bundled and remote plugins | Platform-managed | New digests or versions | Platform updates. No Ravel source holds these names |
| `testanyware` and `TestAnyware.app` | Tool-managed | 2.7.0 became 2.8.0 | Homebrew installed the release whose formula carries the `testanyware` source |
| The coverage rows of inactive cached versions | Not a discovery root | Fewer versions | Claude Code's 14-day cleanup of orphaned versions |
| The coverage row of the marketplace clones | Not a discovery root | 89 `SKILL.md` files in place of 82 | The official marketplace clone gained skills when it updated |

TestAnyware's keg already links `share/ravel/skills/testanyware` into
Ravel's default skill directory. The real HOME's `~/.config/ravel` holds only
`policy.ts`, so Ravel 1.1 delivers nothing from that directory.

### Consequences for the cutover

- The instrument's digest of a plugin version directory depends on the
  session that runs it. A later comparison would report each loaded plugin
  as changed. `owner-inventory-reader-marker-k244` repairs the digest.
- A probe session writes the same marker. The real plan's `outside`
  manifest therefore names no plugin version directory.
- The official marketplace updated enabled plugins in five days. The freeze
  of order 6 covers it as well as the `linkuistics` marketplace.
- Claude Code moved from 2.1.316 to 2.1.319 in the same five days. The
  rehearsal record names one Claude Code version, so that version must hold
  from the rehearsal to the switch.

## Reader markers in the plugin cache

`owner-inventory-reader-marker-k244` repaired the instrument on 2026-11-07.
Its controls and its rerun used Claude Code 2.1.319.

### The marker

Claude Code writes `.in_use/<pid>` into each plugin version directory that a
running session loaded. The marker holds the session's `pid` and
`procStart`. The session removes its own marker when it ends, and the empty
`.in_use` directory stays. The marker therefore identifies a reader, not the
plugin.

The Claude Code plugin row now leaves out the entry `.in_use` at the top of
the version directory, whatever its type. The file count leaves it out too.
A nested `.in_use` and `.orphaned_at` stay in the digest. The Codex
installed-plugin row does not change, because Codex writes no such marker.
Each other entry class keeps its digest.

Claude Code 2.1.319 uses the same rule for its own folder digest of managed
plugin releases. The settings schema in the executable states that this
digest leaves out `.git`, `.in_use`, `.orphaned_at`, `.claude-plugin-link`
and `.links_materialized` at the root.

### Controls of the repair

`task owner:inventory:controls` passed 19 of 19 controls. The fixture's
plugin version directory holds one marker of an earlier session, as the
baseline did. Five cases are new:

| Case | Expectation |
| --- | --- |
| Added marker | The whole output stays the same |
| Changed marker | The whole output stays the same |
| Removed marker, with the empty `.in_use` directory kept | The whole output stays the same |
| Changed plugin manifest | The plugin row changes |
| Nested `.in_use` entry under `.claude-plugin` | The plugin row changes |

Each new control failed against an instrument that it guards against:

| Instrument | Result | Failed cases |
| --- | --- | --- |
| The earlier instrument, SHA-256 `f3c2a86e…3726` of "Method" | 16 of 19 | The three marker cases. The plugin row changed in each |
| A mutant that leaves out `.in_use` at every depth | 18 of 19 | The nested entry. The whole output stayed the same |
| A mutant that leaves out each dot entry at the top | 17 of 19 | The changed manifest and the nested entry. The whole output stayed the same |

The two mutants existed only in a scratch directory. They are not part of the
repository.

| Artifact | SHA-256 |
| --- | --- |
| `docs/evaluations/owner-skill-delivery/inventory.py` | `b933ed455dc53af5c70ef9e2a7a3bc1389e32e413bbca8c4486d0c063c749ccc` |
| `docs/evaluations/owner-skill-delivery/controls.py` | `812fe247edead7c20570bd4a79840e99879d02feb90abc0f76665c836ee80a0b` |
| `docs/owner-skill-inventory-reference.tsv` | `7a16008e3213848f7e96ca2b0b00f7e2b9db43f2769334b79bccb4e5f4885f02` |
| `docs/owner-skill-inventory.tsv` | `cbfac5b83548a3f434f34d2ac4e15afcc4004ea6b212a0beabff58b3de58a0d0`, unchanged |

### Rerun in the real HOME

The repaired instrument ran twice in the real HOME, and the earlier
instrument ran once between the same subject digests. The subjects were both
instruments, the controls script, both Claude Code settings and registry
files, both Codex configuration files and the `mcpServers` object of
`~/.claude.json`. No digest changed.

- The two runs of the repaired instrument gave byte-identical output of 447
  lines: a header and 446 rows. No row said `UNCLASSIFIED`, `UNASSIGNED`, `UNCOVERED` or
  `NEEDS-REVIEW`.
- With `CONTROL_ROOT` set to a scratch root, the output held one
  `UNCLASSIFIED` row and one dangling row for that root.
- The output of the earlier instrument had the same keys. It differed in 11
  rows. Each was a Claude Code plugin row, and only its digest and file count
  differed. Each count was one more. These 11 version directories held one
  marker file each. The other 15 Claude Code plugin rows were the same. Their
  directories held an empty `.in_use` directory or none.

The repaired output is [the reference inventory](owner-skill-inventory-reference.tsv).
Each difference between the reference and the baseline is in a class of
"Differences and their reasons" above, or is the repair itself.

### How a comparison reads the baseline

The baseline stays unchanged, because the moves record and the design cite
its line numbers. Its Claude Code plugin rows include one marker of an
earlier session in the digest and in the file count. Nobody can reconstruct
the bytes of that marker. A later comparison therefore reads the rows in
this way:

- For a Claude Code plugin row, compare with the reference, not the
  baseline. The key, the version, the commit and the repaired digest must be
  equal, unless a recorded update explains the difference.
- For a Claude Code plugin row of the baseline, compare the version, the
  commit, the class and the disposition. Its digest16 is not comparable. Its
  file count is one more than the repaired count when a marker was present.
- For every other row, compare with the baseline as before. The reference
  shows that the repair did not change these rows.

### Finding for the owner design

Claude Code 2.1.319's orphan cleanup reads `.in_use`. The executable's
cleanup first finds an `.orphaned_at` older than 14 days. It then runs a
liveness check on the version directory before it removes it. That check
parses each `.in_use/<pid>` marker and treats a marker that it cannot read as
live. For a live session, the cleanup logs "Skipping orphan cleanup, in use
by live session" and keeps the directory. The
[plugin loading reference](https://code.claude.com/docs/en/plugins/loading#cleanup-of-previous-versions),
read on 2026-11-07, does not mention this check.

`owner-cleanup-reader-check-k256` inspected the installed executable again on
2026-11-07. `claude --version` reported `2.1.319 (Claude Code)`.
The resolved executable was
`~/.local/share/claude/versions/2.1.319`, with SHA-256
`1f53587e92dee8e089fa349188a4cda64fa34ace6e7b0ca2ee278ddaa3cc2ebd`.

The inspection found the age constant `P9n=1209600000`, which is 14 days
in milliseconds. The cleanup calls `Fw` before deletion. A live result
or an error returns without deletion. `Fw` reaches `JWt`, which reads the
markers, and `m1t`, which checks process liveness. An unreadable marker
counts as live. These minified names identify this executable only.

This was a static inspection. It invoked no cleanup and changed no cache.
It establishes the branch in this executable, without measuring a cleanup
run or promising the behaviour of another version.

The earlier design stated that cleanup ignored readers.
[Resource lifetime](specs/owner-skill-delivery.md#resource-lifetime) now states
the age and reader checks for 2.1.319. The freeze, versions registered and
current, reader evidence and purge conditions all stay. That section records
each rule's reason independently of the undocumented check. Codex and Pi
write no marker, and a later Claude Code version can change the check.
The existing switch and its recovery model therefore need no change.

## Policy conversion

`owner-policy-conversion-k245` converted the owner's Ravel policy to Rhai on
2026-11-07. This is order 4 of the
[cutover contract](specs/owner-skill-delivery.md). The owner approved the
candidate. The leaf then copied it to `~/.config/ravel/config.rhai`, beside
the unchanged `policy.ts`. No `settings.json` existed before or after the
copy, and the leaf wrote none.

| File | SHA-256 |
| --- | --- |
| `~/.config/ravel/policy.ts`, modified 2026-11-07 19:34 | `30a0e5fca7c1446999bd19289566fc078053a503c75f03ca7056e641f836f131` |
| `~/.config/ravel/config.rhai`, and the committed candidate [`owner-config.rhai`](evaluations/owner-skill-delivery/owner-config.rhai) | `bbde65efccbfd09e8ac84ab41e1322d35fa31b9726b8451466ce873dd22f88d8` |
| Ravel 1.1.0 keg, `bin/ravel` | `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed` |
| Accepted 2.0 build, `bin/ravel` from `target/ravel-archives-2.0` | `3a9a9129573e4d827a317f6b793ba990044080a31a7b667b64cc153a2f974a0b` |

The 2.0 notice reports build identity `58d479ed…`, which
`scripts/harness-trials/builds.json` names.

### The translation

The Rhai file keeps the routing table, both argument shapes and the
jj-workspace grant. `VERSION` is `harness-dispatch 3`. The conversion needs
no host capability that the accepted build lacks.

`ravel::read_text` replaces `statSync` and `readFileSync`. A missing `.jj/repo` gives `source_missing`, and a directory
gives `source_not_regular`. Both give no grant, as in `policy.ts`.
`ravel::path_resolve` and `ravel::path_parent` replace Node's `resolve` and
`dirname`.

The conversion adds no read grant for skill snapshots. Such a grant changes
the argument vectors when skill delivery is on.
`owner-staging-discovery-k249` measures whether Claude Code needs one.

Two edge cases differ by design. A `.jj/repo` that is not UTF-8 refuses under
Rhai, and Node decoded it with replacement characters. The refusal of an
unreadable `.jj/repo` carries the host's own error text.

### The equivalence run

`task owner:policy-equivalence OUT=<new directory>` runs
[`policy_equivalence.py`](evaluations/owner-skill-delivery/policy_equivalence.py).
Each build has its own private HOME, which holds only its own file. The PATH
holds fake `claude` and `codex` programs that record their arguments. No real
harness runs. The prompt holds quotes, a leading `--help`, a dollar sign and
non-ASCII characters.

The cases are each of the 24 kinds of `policy.ts` in five directories, and
three refusals. The directories are a plain directory, a primary jj workspace,
a secondary workspace that jj created, and two hand-written secondary
workspaces. One of these names its store by an absolute path. The other uses
a relative path with surrounding whitespace. The refusals are a kind that the
policy does not list, a whitespace-only `.jj/repo` and an unreadable one.

| Run | Rhai input | Result |
| --- | --- | --- |
| Candidate | `owner-config.rhai` | 123 passed, 0 failed |
| Installed copy | `~/.config/ravel/config.rhai` | 123 passed, 0 failed |
| Control: `copy-edit` at `medium` | Mutated copy | Only the five `copy-edit` rows failed |
| Control: grant one level too deep | Mutated copy | Only the 72 secondary-workspace rows failed |

In each routed case, both builds launched the same program with the same
argument vector. They reported the same provider, model, effort and reason.
In each refusal case, both refused with exit status 3, the same code and the
same remedy. Each refusal message was equal, except for the unreadable store.

A further check copied the real `~/.config/ravel` with both files into a
private HOME. There the 1.1.0 keg evaluated `policy.ts`, and the 2.0 build
evaluated `config.rhai`.

### Obligations that remain

A later change to `policy.ts` needs a new conversion and a new equivalence
run before the switch. The digest of `policy.ts` in the table above
identifies the converted version. Compare it with the live file before the
switch.

After this driver's reap, the owner does these steps:

- Remove `~/.config/ravel/policy.ts` when no 1.1 launch remains.
- Remove the inactive 1.1.0 keg with `brew cleanup ravel`, outside the
  finish session of its last driver.
- Write any new settings keys, which Ravel 1.1.0 refuses as
  `settings_invalid`. These can be the `harnesses` entries for Claude Code and
  Codex. The owner sets no `policyEnv`, so no `configEnv` conversion is
  necessary.

## Owner settings drafts

`owner-settings-drafts-k246` drafted the owner's Ravel skill settings and the
declarations of the skill formulae on 2026-11-07. This is order 4 of the
[cutover contract](specs/owner-skill-delivery.md#cutover-contract). The drafts
are in [`owner-cutover/`](owner-cutover). No draft is in the real HOME. The
real `~/.config/ravel/settings.json` was absent before and after the work.

| Draft | Content | Consumer |
| --- | --- | --- |
| [`settings-off.json`](owner-cutover/settings-off.json) | The off value. `sources` is empty, and `mandatory` is absent | Step 3 of the switch, the earlier value of the `skills` cell and the inverse |
| [`settings-switched.json`](owner-cutover/settings-switched.json) | The switched value. It names no `sources` and holds only the mandatory file, superpowers `skills/using-superpowers/SKILL.md` | Step 9 of the switch |
| [`settings-staging.json`](owner-cutover/settings-staging.json) | The staging settings. They name the ten sources in byte order of their IDs, with the roots of each source and the same mandatory file | The staging HOME |
| [`bootstrap-packages.toml`](owner-cutover/bootstrap-packages.toml) | The five `[bootstrap.packages]` declarations of "Skill formulae" | `owner-pack-checkouts-k93` |

Each settings draft is a complete settings file that holds only the `skills`
key. The cells of the switch plan take the value of that key.
`owner-switch-plan-draft-k247` and `owner-switch-plan-k254` consume the off
value and the switched value.

### Paths of the staging settings

Grove's six sources are in the candidate keg. The staging settings name each
of them through the link of the candidate's skill directory:

`/Users/antony/.local/state/owner-staging/candidate/brew/share/ravel/skills/<source-id>`

`task owner:candidate-keg` makes that link for each source when its `OUT` is
`/Users/antony/.local/state/owner-staging/candidate`.
`owner-staging-setup-k248` builds the keg with that `OUT`. The path is outside
every working copy and the scratchpad, and it holds no version.

The four other sources are in Ravel's default skill directory of the real
Homebrew prefix, at `/opt/homebrew/share/ravel/skills/<source-id>`. Homebrew
links each of them to its installed keg. No path of the staging settings names
a versioned keg.

The staging HOME does not contain a source, and no source contains it.
Otherwise Ravel refuses the source as `skill_access_unavailable`.

### The declarations

The owner's `~/.config/mise/config.toml` already declares
`brew:linkuistics/taps/grove` and `brew:linkuistics/taps/testanyware`.
`owner-pack-checkouts-k93` adds the three other declarations to that table.
The local tap holds each of the five formulae: `grove` 22.8.0, `personal`
2026-11-06, `testanyware` 2.8.0, `superpowers` 7.0.0 and `quint-llm-kit`
1.0.0. Of these, only `grove` and `testanyware` are installed.

### The check

`task owner:settings-drafts OUT=<new directory>` runs
[`settings_drafts.py`](evaluations/owner-skill-delivery/settings_drafts.py).
The script copies the accepted 2.0 build into the keg `Cellar/ravel@2/2.0.0`
of a fixture Homebrew prefix. The installation record of that keg names the
prefix's default skill directory. That directory links ten fixture sources,
one at each path of the staging settings, moved under `OUT`. Each case runs
`ravel skills resolve --json` under `env -i`, with a new HOME. The command
launches no harness, and the case PATH holds only `/usr/bin` and `/bin`.

| Case | Settings | Result |
| --- | --- | --- |
| `no-settings` | None | Exit 0, `resolved`, the ten sources in byte order |
| `off` | The off value | Exit 0, `disabled`. The HOME held no cache after the run |
| `switched` | The switched value | Exit 0, `resolved`, the ten default sources. The mandatory path lay in the superpowers snapshot and held the bytes of the fixture file |
| `staging` | The staging settings, with each path moved under `OUT` | Exit 0, `resolved`, the ten sources and the mandatory file |
| `staging-as-is` | The unchanged staging settings | Exit 3, `skill_source_missing` at `skills.sources[0].path`, the absent candidate path of `design`. The settings checks passed |
| Control | The off value with the mandatory reference | Exit 2, `settings_invalid` at `skills.mandatory` |
| Control | The switched value with an absent mandatory file | Exit 3, `skill_mandatory_invalid` at `skills.mandatory[0]` |
| Control | The staging settings with a repeated source | Exit 2, `settings_invalid` at `skills.sources[10].id` |

The script also requires the five declarations at `latest` and reads each
formula from the local tap with `brew info`, without an update.

The accepted build was `bin/ravel` with SHA-256 `3a9a9129573e…`, which
`scripts/harness-trials/builds.json` names. The drafts had these digests, which
were the same after the run:

| Draft | SHA-256 |
| --- | --- |
| `settings-off.json` | `6cdd20fd11675506d53584624ff8d8333c4f3c1f3d0c93b36c275865a7f306fa` |
| `settings-switched.json` | `944f2a245b8d60e6d61886015a15284657651788cbf4e232e85542ce6baa8743` |
| `settings-staging.json` | `d8a49f066310b0976cb4e13674c524d96d31b6aa0ba5bfc6923fc0b097623145` |
| `bootstrap-packages.toml` | `09dff6442b119698aa4ad6696f10b369dd9a61b9e3963046fd9ee324a1a308ee` |

Each check failed against a wrong draft in a scratch copy:

| Mutation | Cases that failed |
| --- | --- |
| The off value without `sources` | `off`, which resolved the ten sources and wrote a cache, and the first control |
| The switched value with another file as mandatory | `switched` |
| The staging settings with nine sources | `no-settings`, `switched` and `staging` |
| A misspelt formula name | The declarations, with Homebrew's refusal of that name only |

The fixture sources hold probe skills, not the published sources. The staging
trials resolve the published sources, as the cutover contract states. After
the Ravel cut, rollout repeats the off-value control with the published
`ravel@2` and the installed kegs.

## Owner switch plan draft

`owner-switch-plan-draft-k247` drafted the real switch plan on 2026-11-07.
Order 4 of the [cutover contract](specs/owner-skill-delivery.md#cutover-contract)
calls it the residue plan. The draft is not in the real HOME. The real
`~/.config/ravel/settings.json` and `~/.local/state/owner-switch` were absent
before and after the work.

| Artifact | Content |
| --- | --- |
| [`switch-plan-draft.json`](owner-cutover/switch-plan-draft.json) | The plan, schema `owner-switch-plan/1`, with every field of the format |
| [`switch-plan-parts.tsv`](owner-cutover/switch-plan-parts.tsv) | Each part and cell, with its line in [the reference inventory](owner-skill-inventory-reference.tsv), its class and its disposition |
| [`switch-plan-dry-run.txt`](owner-cutover/switch-plan-dry-run.txt) | The output of `run --dry-run` in the disposable HOME |
| [`switch-plan-processes.txt`](owner-cutover/switch-plan-processes.txt) | The real process tree and job listing against the plan's patterns |
| [`switch-plan-check.json`](owner-cutover/switch-plan-check.json) | The record of the check, with the result of each control |

`task owner:switch-plan:draft` runs
[`switch_plan_draft.py`](evaluations/owner-skill-delivery/switch_plan_draft.py)
and writes the plan and the table from the real HOME. It reads only. Two runs
gave the same bytes. `task owner:switch-plan:check OUT=<new directory>` admits
the draft and checks it.

### The parts

The script reads each child of five discovery roots and finds its row in the
reference inventory. A row of the class `owner-pack`, `third-party-pack`,
`stale-copy` or `dangling`, with `deactivate` as the first word of its
disposition, makes an entry part. A child without exactly one row stops the
script. Each entry carries the change identity that `owner_switch.py
identity` prints.

| Root | Entry parts | Children that stay |
| --- | --- | --- |
| `~/.claude/skills` | The stale `grove` copy and the dangling `terminal-browser` link | `codebase-memory`, and the account-synced folder `synced` |
| `~/.codex/skills` | 44 links into Grove's snapshot store, 14 superpowers copies and `terminal-browser` | `codebase-memory`, and Codex's system folder `.system` |
| `~/.agents/skills` | 44 links into Grove's snapshot store, 3 quint links and `terminal-browser` | `codebase-memory` |
| `~/.pi/agent/skills` | 44 dangling links into `~/Development/grove` | `codebase-memory` |
| `~/.pi/agent/extensions` | The dangling `technical-communication.ts` link | Five extensions of codebase-memory, herdr and Orca |

The 154 entry parts come first. Then the part of `.claude/settings.json`
changes seven cells of `enabledPlugins`:

- It disables `grove`, `linkuistics` and `testanyware` of the `linkuistics`
  marketplace, `superpowers` and `quint-llm-kit`. Each is `true` in the real
  HOME and in the reference inventory.
- It enables `technical-communication-hooks@writing-hooks` and
  `guardrail@personal`. Both keys are absent in the real HOME today. The
  draft's earlier value is `false`. The hooks prelude of step 3 writes that
  value, and owner-pack-checkouts-k93 installs `guardrail` disabled.

The last part changes the `skills` key of `.config/ravel/settings.json`. Its
earlier value is the `skills` value of `settings-off.json`. Its switched value
is the `skills` value of `settings-switched.json`. The program's check of the
drafted real settings accepted it, because it names no `sources`.

### The manifest outside the switch set

`outside` holds three path entries and five root manifests:

- Path entries: `.claude/skills/codebase-memory`,
  `.codex/skills/codebase-memory` and Grove's snapshot store
  `.agents/.grove`, which the moved links name.
- Root manifests: `.agents/skills`, `.pi/agent/skills`,
  `.pi/agent/extensions`, `.claude/agents` and `.codex/agents`. A root
  manifest also detects a new child of its root.

`.claude/skills` and `.codex/skills` have no root manifest. A platform writes
their folders `synced` and `.system`. A probe session can rewrite either
folder, and step 14 would then count a failed probe and run the inverse. The
draft names no plugin version directory, because each Claude Code session
writes `.in_use/<pid>` there. It also leaves out `~/.codex/hooks.json` and the
`~/.quint-llm-kit` clone. Their tools rewrite them, and the switch reads
neither.

### Processes and jobs

The harness pattern matches a process whose program is `claude`. The MCP
patterns match each of the owner's stdio servers of Claude Code, and each
launcher chain:

| Server | Processes |
| --- | --- |
| `quint-kb` | `node .../.quint-llm-kit/repo/mcp-servers/kb/dist/server.js` |
| `codebase-memory-mcp` | The server, and its child `codebase-memory-mcp --cbm-daemon-internal` |
| Playwright | `npm exec @playwright/mcp@latest`, or `npx @playwright/mcp@latest`, and the `node .../playwright-mcp` that it launches |

Context7 uses HTTP and launches no process. Each other child of a server
counts as another process.

<!-- tc: depart synonym-rotation because "kept" names the design's term "kept process", not a second word for a check -->
Finding for the owner design: Claude Code runs `caffeinate -i -t 300` as a
child of the harness process. The design's kept processes do not name it, so
step 2 would always stop. The draft keeps it through `session.mcp`, because
no other plan field keeps a child of the harness process. The section
"Concurrent writers" of the design should name it.

The watch patterns match the program of a command, not any substring. A path
such as `.../grove` in another argument therefore does not match. They match
Claude Code, Codex, Pi, Grove, Ravel, herdr, Orca, mise and
`codebase-memory-mcp`, and the packaged Node forms of the three harnesses.
They also match the applications Claude, ChatGPT, Herdr and Orca. ChatGPT
holds Codex's `node_repl` MCP server. `OrcaSlicer` does not match.

The job listing runs `launchctl list` and `crontab -l`. `sfltool dumpbtm`
waited for authorisation, so the listing cannot read login items without
privileges. That is a limit of step 1.

The listing ran from this session, which Grove 22.8 launched through Ravel
1.1.0. The program's own process gate ran from the same position and named
the same processes and jobs. At that time, step 2 would have stopped for
19 processes and 3 jobs:

- The owner's Codex app-server daemon, which reads the Codex roots.
- `herdr-core` and `mise dot watch`, which write files of the switch set.
  The job `dev.mise.mise-history` runs `mise dot watch`, and two jobs
  `dev.herdr.server.*` can launch herdr.
- A Ravel 1.1.0 session of another task and two Grove builds in `/private/tmp`.
- Eleven Codex daemons of earlier trials in disposable HOMEs.

The owner ends these processes and unloads the jobs before the switch. The
finish handoff of discovery-rollout-k12 names them.

### Commands

| Field | Value |
| --- | --- |
| `state` | `.local/state/owner-switch` |
| `manual` | `Library/Mobile Documents/iCloud~md~obsidian/Documents/Brain/Configuration/AI Dev Tooling.md`. Its folder resolves inside HOME |
| `baseline` | `$python -I .local/state/owner-switch/prepared/inventory.py` |
| `gates` | The program's own gate of each of steps 4 to 7 |
| `probes` | `owner_switch_probes.py resolve --expect switched`, and `session --route ravel` for Claude Code and for Codex |
| `native_probe` | `owner_switch_probes.py resolve --expect disabled`, and `session --route native` for Claude Code and for Codex |
| `mac_check` | `.local/bin/mac-check` |

Each command runs in HOME, so a path relative to HOME reaches the prepared
folder. owner-switch-plan-k254 copies `inventory.py` and the probe program
beside the plan and the program. The instrument reads TOML through
`tomllib`, so it needs Python 3.11 or later. It runs under `$python`, the
interpreter of the switch program.

No probe program exists yet. The draft names its interface, and
`owner-switch-probes` implements it before owner-switch-plan-k254. That leaf
can change the interface. k254 then changes the commands of the plan.

### Provisional accepted values

`accepted` holds provisional values, and owner-switch-plan-k254 replaces them
with the evidence of owner-staging-acceptance-k252:

- `sources`: the ten sources of the table "The ten Ravel sources" in the
  [moves record](repository-moves-record.md#the-ten-ravel-sources). The
  script reads that table, so each `resourceDigest` is the recorded value.
- `versions`: `personal` 2026-11-06, `testanyware` 2.8.0, `superpowers`
  7.0.0 and `quint-llm-kit` 1.0.0, each a formula of `linkuistics/taps`.
- `trials`: one command for each of Grove's six sources that prints a
  message and fails. A step 4 that must rerun a trial therefore stops.
- `staging`: `/Users/antony/.local/state/owner-staging/home` and
  `/Users/antony/.local/state/owner-staging/candidate`. owner-staging-setup-k248
  records the final paths.

### The check

The check builds a disposable HOME. It copies each entry that a part or an
`outside` item names from the real HOME, and each copy had the draft's change
identity. It writes both settings files with each cell at its earlier value.
The program's loader then admitted the draft, and `run --dry-run` reported the
state `native`. The output listed each of the 156 parts in the switch and in
the inverse, and it reported no passing rehearsal.

Each control must fail, and each did:

| Control | Result |
| --- | --- |
| The plan without `mac_check` | Exit 3, "the plan is incomplete" |
| A part path that is absolute | Exit 3, "is not relative to HOME" |
| A gate 4 list without the program's gate | Exit 3, "does not run the program's gate of step 4" |
| One added byte in a stale superpowers copy | The state `changed` |
| A new child of `.agents/skills` | The state `changed` |

Eighteen pattern cases state what each pattern list must match or must not
match. Three wider mutants failed them:

- A watch list of plain substrings failed seven cases.
- A job pattern without word boundaries failed two cases.
- An MCP pattern for any `node` failed one case.

| Subject | SHA-256 |
| --- | --- |
| `switch-plan-draft.json` | `c40451d63c33e1cb8800a80ddce6b8b4e21e534c4711ec5b8dd24a72249f0859` |
| `switch-plan-parts.tsv` | `ce2ad54ebdb49fe6ba82a6c1d60481cfb449587736d1e6024ff9830897377fd4` |
| `scripts/owner_switch.py`, unchanged | `885171751f02b62451090a4a41cc7b5145bb3aac29556d85b1dbcb262cc7884f` |
| `inventory.py`, unchanged | `b933ed455dc53af5c70ef9e2a7a3bc1389e32e413bbca8c4486d0c063c749ccc` |

The dry run reads no gate, no probe and no rehearsal record. The rehearsal of
owner-switch-rehearsal-k255 measures them on the completed plan.

## Owner switch plan draft review

`owner-switch-plan-draft-k258` inspected producer commit
`9490761a9099b656630d0dd05c49c9721fa3ec8b` against the owner design. The
findings below describe counterexamples from the source. They are not
results of running the switch. No production or test code was changed.

### D1 — P1: command paths do not identify the rehearsed programs

The commands at
[`switch_plan_draft.py:101`](evaluations/owner-skill-delivery/switch_plan_draft.py#L101)
and line 114 execute `owner_switch_probes.py` and `inventory.py` from the
real `<state>/prepared` directory. The plan records their paths but no
content identities. `load_plan` hashes the JSON only, `rehearsed` compares
the plan and main program digests, and a backup copies the main program.
`run_command_full` still opens the two external scripts at their live
paths. Those scripts are outside both `parts` and `outside`.

Consider a prepared plan that has passed rehearsal. Before its real run,
replace the probe script with a program that exits zero without observing
anything. Neither admitted digest changes, so step 8 still accepts the
record and step 10 reports successful commands. Replacing the inventory
script with one that emits no rows also bypasses the anomaly check. The
same substitutions affect a continuation of an existing backup: running
its saved main program does not restore either dependency. This breaks
the requirement that the real HOME receive the rehearsed operation and
that a continuation execute the operation preserved by its backup
([The rehearsed plan](specs/owner-skill-delivery.md#the-rehearsed-plan)).

### D2 — P1: job labels do not enumerate the programs that jobs can launch

The job command and patterns at
[`switch_plan_draft.py:96`](evaluations/owner-skill-delivery/switch_plan_draft.py#L96)
read only the current launchd listing and crontab text. A launchd listing
holds labels, PIDs and exit statuses; it supplies no program arguments.
The patterns also omit Pi. Login items are not read at all, and every
crontab error is suppressed and converted to success.

A loaded, idle job labelled `dev.personal.maintenance` can run
`/opt/homebrew/bin/mise dot watch`. Its listing line contains none of the
patterns. With no current writer process, the gate admits a write interval
although that job can start its writer during the interval. A Pi job
labelled `dev.pi.agent` is another missed case. A failure to read crontab
also looks like no schedule. These are gaps in the observation, not
evidence that those particular jobs are installed today. The recorded
three matched jobs do not establish the absence of others. Step 1 requires
every enabled job, login item and schedule that can launch a named program;
step 2 must stop while one remains
([The switch inside finish](specs/owner-skill-delivery.md#the-switch-inside-finish)).

### D3 — P2: omitting two whole roots also omits their ordinary children

The `outside` choices at
[`switch_plan_draft.py:67`](evaluations/owner-skill-delivery/switch_plan_draft.py#L67)
omit `.claude/skills` and `.codex/skills` because their platform containers
are mutable. That also omits the names of every other child not already
in `parts`. The control adds a child only to `.agents/skills`, a root that
does have a manifest.

For a concrete changed HOME, add `.codex/skills/quint-lang` as a link to
the retained Quint clone after drafting. That path is absent from the
draft's parts. `outside_changes` never reads it. The selected inventory's
`codex_cls` classifies it as a known third-party entry, with disposition
`deactivate; ravel:quint`, rather than emitting an anomaly. Step 3 can
therefore admit the native state while step 9 leaves a superseded native
copy active. The new copy depends on a later discovery probe to stop the
switch, and the manifest still cannot detect it at step 14. Changes inside
`synced` or `.system` do not justify ignoring their ordinary siblings.
This leaves a gap in the one-active-copy rule and the comparison of entries
outside the switch set
([Name collisions and stale copies](specs/owner-skill-delivery.md#name-collisions-and-stale-copies),
switch steps 3 and 14). D5 is a separate continuation failure, not a
replacement for this missing observation.

### D4 — P2: the MCP list grants a non-server a kept-process exemption

The pattern at
[`switch_plan_draft.py:86`](evaluations/owner-skill-delivery/switch_plan_draft.py#L86)
labels `caffeinate -i -t <digits>` as an MCP server. The producer's
committed process listing shows that this exempts the harness child
`caffeinate -i -t 300`. The program follows matching children recursively,
so the exemption is executable, not merely a comment about future work.

The design permits only the harness, its driver chain, actual MCP servers,
and the program and shell. It expressly counts every other descendant as
another process. `caffeinate` is not a server, and ending it need not end
the session. The draft therefore implements a different exclusion rule
from [Concurrent writers](specs/owner-skill-delivery.md#concurrent-writers).
The producer has acknowledged the difference, but a proposed design change
in a brief does not settle it. Nor does this one exemption establish a
rule for another helper child: an unmatched `/bin/sleep 300` child still
closes the interval. No harmful write by `caffeinate` is alleged; the
defect is the unresolved, load-bearing change to the exclusion contract.

### D5 — P1: the chosen inventory rejects the switched guardrail plugin

The plan's `baseline` at
[`switch_plan_draft.py:114`](evaluations/owner-skill-delivery/switch_plan_draft.py#L114)
names the unchanged inventory instrument. Its plugin-skill classification
at [`inventory.py:164`](evaluations/owner-skill-delivery/inventory.py#L164)
recognizes the old `linkuistics` marketplace, superpowers and Quint. For
another enabled plugin with a `skills` directory, it emits
`UNCLASSIFIED`. The personal plugin has `skills/guardrail/SKILL.md`.

After step 9 enables `guardrail@personal`, stop the program before the
receipt. On the next run, step 3 runs that inventory with guardrail enabled
and collects its unclassified row. The cells still describe the active
state, but `continue_after_switch` treats the collected row as an outside
change and calls `failed_probe`, which applies the inverse. A correct
activation thus fails its required active continuation solely because the
instrument does not recognize the intended native residue. A stop after
the receipt but before its copy likewise makes step 14 fail. The dry run
never executes the baseline, so its passing record does not cover this
case. This contradicts the guardrail disposition and the continuation
contract
([Native residue](specs/owner-skill-delivery.md#native-residue),
[Resuming the switch](specs/owner-skill-delivery.md#resuming-the-switch)).

### Other doubts and limits of this review

Both new plugin keys are absent in the real HOME at review time. Their
earlier values of `false` are nevertheless consistent: cutover order 6
requires `guardrail` installed and disabled, and the hooks prelude installs
and disables the other plugin before step 9. The disposable check writes
those expected values; it does not prove that preparation has occurred.

No false reversal caused by an ordinary read of the eight `outside` items
was established. The change identity hashes content, entry names, types,
executable bits and link targets; it omits timestamps and follows no link.
Reading the snapshot store or acquiring its empty lock therefore supplies
no demonstrated content change. The tool owns updates to codebase-memory
copies, agents and Pi adapters. An actual installer write can change an
identity, but step 14 currently requires that change to fail the probe.
No update or probe was launched here, so this review makes no claim that
the kept server or a normal probe necessarily rewrites one of those items.

The provisional accepted values, failing trial placeholders and absent
probe implementation are declared handoffs to later leaves. Their absence
is not a separate finding against this draft. D1 concerns what the eventual
commands identify, regardless of the implementation they receive.

This was a task-directed Verify review using graph project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-07T12:56:06Z`. The bounded symbol searches returned no further
pages. An inbound trace of `outside_changes` identified the state,
continuation and step-14 consumers; exact source controlled each claim.
Coverage reported no recorded gap in the cited code. TSV and text evidence
had untracked freshness metadata and were read directly. That signal is
not proof of completeness. The personal plugin's local layout, the selected
real-HOME keys, directory names and relevant launch-agent definitions were
also read without changing them.

The inspected plan and part table have SHA-256
`c40451d63c33e1cb8800a80ddce6b8b4e21e534c4711ec5b8dd24a72249f0859`
and `ce2ad54ebdb49fe6ba82a6c1d60481cfb449587736d1e6024ff9830897377fd4`.
They match the producer's recorded subjects. The main program and inventory
digests also match that record. The generator digest is
`9b13ac3122a74a5b640c418e26af4b4e0fbb482f7701ac8117109d0bd63b4b26`.
No test, build, lint, format, check task or switch operation was run by this
review. Integration owns fixes and executable verification of the cases.

## Owner switch plan draft integration

`owner-switch-plan-draft-k259` triaged the five findings of the review above
on 2026-11-08. It verified each finding against the source, and each was
real against the current design. It then prototyped a repair for each
finding in disposable layouts. The repairs passed their checks. The real
HOME stayed read only.

The human then stated two facts: this grove is the only grove that runs,
and the human is the only user of Grove and Ravel. The human asked for a
simpler switch on that basis, and chose to drop the repairs. The human chose
four simplifications: the process and job exclusion, the concurrent-writer
detection, the recovery state machine and the rehearsal admission.
`owner-switch-simplification-k260` redesigns the switch. Its review and a
planning leaf follow it, before the staging leaves.

| Finding | Disposition |
| --- | --- |
| D1, unbound command files | Real. Superseded by the redesign, which must still identify each file that a certified plan runs |
| D2, job labels only | Real. Superseded by the redesign, because an owner checklist replaces the automatic job listing |
| D3, two omitted roots | Real. Superseded by the redesign, which must still see a new sibling of a platform folder |
| D4, a helper as an MCP server | Real. Superseded by the redesign, because the automatic process listing goes |
| D5, the switched guardrail plugin | Real. Superseded by the redesign, which must still let the instrument accept an enabled `guardrail@personal` |

The section "Other doubts and limits of this review" raised no further
finding.

The prototypes measured three facts, which k260 carries in its task file:

- During a turn, Claude Code runs `caffeinate -i -t 300` as a child of its
  harness process. That child writes no file.
- launchd reads the account's own home, whatever HOME names. A job listing
  that reads `~/Library/LaunchAgents` through HOME misses the owner's jobs
  in a disposable layout.
- `launchctl print gui/<uid>/<label>` shows the program arguments of each
  loaded job. All 577 jobs took about 2 seconds.

## Single-owner switch design

owner-switch-simplification-k260 uses the human's sole-user and sole-grove
facts to replace four mechanisms. The specification now requires a quiet
interval, one complete backup before all writes, ordinary serial writes,
full restore on failure and one disposable execution of the real plan.
This is a design change only. No new switch or actual owner cutover is
measured by this section.

| Finding | Design disposition |
| --- | --- |
| D1 | Keep content identities for the plan, program, invoked scripts, local helpers and command inputs, with tool paths/versions and dry-run substitutions. A path alone certifies nothing. Remove runtime rehearsal admission; the owner reviews the result and keeps the prepared inputs unchanged. |
| D2 | Remove automatic job classification. The owner checklist names current writers and relaunch mechanisms, including mise, herdr, Orca and the codebase-memory updater. The narrow harness/driver process check does not establish their absence. |
| D3 | Compare ordinary children of both native skill roots. Exclude only `.claude/skills/synced` and `.codex/skills/.system`, not their parents or siblings. Probe changes inside those exact subtrees are expected. New superseded siblings still fail. |
| D4 | Remove MCP regex exemptions. The narrow process check excludes exact finish ancestors, not arbitrary matching descendants. Non-harness `caffeinate` needs no exemption. |
| D5 | Classify enabled `guardrail@personal` as the deliberate native exception only at the personal keg origin and with only the guardrail skill. Another skill or origin fails. |

The design keeps one active route per owner-selected name, finish's absolute
guidance paths, frozen current plugin registrations and the original Ravel
1.1.0 keg. Native restore explicitly turns Ravel sources off after promotion;
restoring an absent `skills` key would enable the default sources beside
native skills.

The source inspection used Verify-tier graph evidence for
`scripts/owner_switch.py`: `run_switch` calls the process check, preludes,
state lookup, rehearsal admission and backup, with partial continuation and
program re-execution. `held_command` implements the held-link, metadata and
inode checks. Both call directions for `run_switch` were read at depth one,
with tests included and no remaining page. Coverage at generation
`2026-11-08T03:42:48Z` reported no recorded issue for the program, rehearsal,
draft generator and inventory paths. This is best-effort coverage, not a
completeness proof. The relevant spec, live leaves and earlier review findings
were read directly.

The simplified program is not implemented yet. Its dry run must measure the
complete command write set, metadata and quiet-session assumption. A final
comparison can detect a different resulting layout; it cannot detect an edit
that an ordinary write already overwrote. A killed program cannot restore
while dead. The retained restore entry must work after restart, and a failed
restore must preserve its backup and prevent success. The new design makes
these limits explicit instead of applying the old model's claims to it.

The existing owner-switch-simplification-k261 review challenges the design.
The parent rollout brief gives k262 artifact and leaf consequences; no
implementation leaf or machine configuration was changed in this session.

The visual document's manifest and three changed diagram sources were served
at the existing localhost viewer and matched the working files. Browser
access was unavailable, so this session did not inspect their rendered
layout. The editable sources and stable discussion links remain available
for the scheduled review.
