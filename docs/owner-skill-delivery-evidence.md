# Owner skill delivery evidence

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
| Claude Code plugin cache | The [plugin loading reference](https://code.claude.com/docs/en/plugins/loading#cleanup-of-previous-versions), read on 2026-11-02. An update or uninstall writes `.orphaned_at` into the earlier version directory, and a background cleanup removes it 14 days later. A running session keeps the plugin set that it loaded until `/reload-plugins` or a new session. |
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
