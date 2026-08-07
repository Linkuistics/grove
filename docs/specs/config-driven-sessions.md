# config-driven-sessions

## Problem

Grove currently reconstructs one session command from several independent
sources: a repository marker and local stamp select a primary harness; task
bodies and environment variables can reroute a leaf; more variables select a
model; and harness-specific code appends naming, sandbox, and Herdr arguments.
The driver therefore knows too much about the programs it launches while a
reader cannot see the complete launch policy in one place.

Lifecycle has the same split ownership. Human-facing subcommands, a routing
peek, and the launched agent each perform part of initialization, migration,
selection, retirement, or finishing. That makes restart behavior harder to
state and permits the leaf used to route a session to differ from the leaf the
session later adopts.

## Solution

One personal file, `~/.config/grove/config.kdl`, is the entirety of user launch
policy. It maps every session kind to one complete command-template string.
Grove parses that string into arguments, expands only its documented
substitutions, and executes the resulting command directly. It does not identify
the harness, infer a model, add harness-specific arguments, or execute a shell.
This follows the [complete session configuration](../adr/complete-session-configuration.md)
decision.

One bare `grove` command owns the whole lifecycle. On every iteration it
validates configuration before any task-tree mutation, brings the tree to a
runnable current shape, performs one authoritative pick, reloads configuration,
and launches the selected kind. The selected leaf's stable handle is embedded
in `${prompt}` as the session mandate. A launched session resolves that handle
and never picks again.

The resulting flow is:

```text
bare grove
  -> provision embedded methodology independently of launch policy
  -> load and fully validate ~/.config/grove/config.kdl
  -> recover or perform at most one required lifecycle transition
       absent tree  -> create root brief + requirements leaf
       legacy tree  -> migrate layout and session-kind filenames, then commit
       empty tree   -> append or reuse the driver-owned finish leaf
       live tree    -> no mutation
  -> authoritative depth-first pick
  -> reload and fully validate configuration
  -> expand the selected kind's template with a mandate-bearing prompt
  -> spawn that argv directly as the foreground child
  -> relaunch only after an explicit completion signal
```

## Configuration file

### KDL shape

The document is a flat set of nineteen top-level nodes. A node name is the
session kind and its sole positional argument is a KDL string containing the
complete command template. Nodes have no properties and no child blocks.
Comments and insignificant ordering are permitted.

```kdl
requirements "env HERDR_AGENT=claude claude ${herdr_settings} --model opus ${prompt}"
review-requirements "env HERDR_AGENT=codex grove-codex-review ${worktree} ${prompt}"
integrate-review-requirements "grove-claude --session ${session_name} ${prompt}"

design "grove-claude --session ${session_name} ${prompt}"
review-design "grove-codex-review ${repo} ${prompt}"
integrate-review-design "grove-claude --session ${session_name} ${prompt}"

planning "grove-claude --session ${session_name} ${prompt}"
review-planning "grove-codex-review ${repo} ${prompt}"
integrate-review-planning "grove-claude --session ${session_name} ${prompt}"

prototype "grove-claude --session ${session_name} ${prompt}"
review-prototype "grove-codex-review ${repo} ${prompt}"
integrate-review-prototype "grove-claude --session ${session_name} ${prompt}"

impl "grove-claude --session ${session_name} ${prompt}"
review-impl "grove-codex-review ${repo} ${prompt}"
integrate-review-impl "grove-claude --session ${session_name} ${prompt}"

research-a "grove-claude --session ${session_name} ${prompt}"
research-b "grove-codex-research ${repo} ${prompt}"
combine-research "grove-claude --session ${session_name} ${prompt}"

finish "env HERDR_AGENT=claude claude ${herdr_settings} --model opus ${prompt}"
```

The wrapper names are illustrative. A configured wrapper must `exec` its
underlying harness so Grove continues to own the real foreground child.

The exact required node names are:

```text
requirements
review-requirements
integrate-review-requirements
design
review-design
integrate-review-design
planning
review-planning
integrate-review-planning
prototype
review-prototype
integrate-review-prototype
impl
review-impl
integrate-review-impl
research-a
research-b
combine-research
finish
```

Direct kind nodes are deliberate. A generic repeated node such as
`session kind="design" command="..."` states the kind twice and makes duplicate
diagnostics less local. Defaults, families, profiles, and inheritance are
excluded because they would recreate a precedence lattice and make a target
incomplete when read in isolation.

### Command-template grammar

Grove first applies POSIX shell-word splitting to the KDL string. This parsing
provides familiar quoting and escaping but does not invoke a shell. After
splitting, Grove validates and expands these whole-word substitutions:

| Substitution | Expansion |
|---|---|
| `${prompt}` | The complete embedded Grove workflow plus the selected leaf's stable-handle mandate, as one argument. Required exactly once. |
| `${session_name}` | `<repo-basename>: <grove-name> grove`, as one argument. Optional. |
| `${worktree}` | The absolute root of the working tree that owns `.grove/`, as one argument. Optional. |
| `${repo}` | The absolute root of the main repository: the default jj workspace root, or the parent of Git's common directory. Optional. |
| `${herdr_settings}` | Outside a Herdr pane, zero arguments. Inside one, `--settings` and Grove's inline turn-hook JSON as two arguments. Optional. |

Every substitution occupies a complete parsed word. Embedded substitutions,
unknown `${...}` names, a substitution in word zero, and more than one use of
the same substitution are errors. `${prompt}` may appear at any nonzero argv
position; it is not required to be last. Each optional substitution appears
zero or one time.

The first parsed word is a non-empty literal executable or script name. Grove
passes the remaining words directly to it with the working tree as the current
directory. Shell variables, command substitutions, redirections, pipelines,
globs, aliases, and tilde expansion are not interpreted. A user who needs those
behaviors places them in an executable wrapper and configures that wrapper as
word zero.

Substitution values are argv values, not text splices, so spaces or shell
metacharacters in repository paths, session names, prompts, and JSON never
change argument boundaries. `${herdr_settings}` is the only substitution whose
arity is not one; requiring it to occupy a whole word makes its zero-or-two
behavior unambiguous.

Grove adds no hidden harness-specific arguments or environment values. The
configured command owns executable choice, harness, model, reasoning effort,
approval and sandbox policy, session-name flags, repository grants, and
`HERDR_AGENT`. Grove still owns its temporary loop-control channel, child
lifecycle, current directory, and the generated prompt. Those are orchestration,
not user launch policy.

### Validation and diagnostics

Configuration is all-or-nothing. A successful load proves:

- the exact path exists, is readable, and parses as KDL;
- every required kind occurs exactly once;
- no unknown top-level node, property, child block, or extra argument exists;
- every node's sole argument is a string;
- every template shell-splits successfully and obeys the executable,
  substitution, and `${prompt}` rules above.

A syntax error reports the exact config path and KDL source span. Once syntax is
available, schema and template validation is aggregate rather than first-error:
one diagnostic names every missing kind in canonical order, every unknown kind,
every duplicate with all source locations, every malformed node, and every
invalid template with its kind and span. A missing file names the exact path and
the complete required kind set. No diagnostic silently fills a target or falls
back to another kind.

Validation does not attempt to identify the configured program or understand
its arguments. Failure to resolve or spawn the selected literal executable is a
launch error naming the selected kind and executable; wrappers and commands
later in an `env` invocation remain opaque by design.

The driver reads and validates the whole file immediately before every launch.
It also validates before root initialization, migration or migration recovery,
and finish-leaf materialization. When one of those mutations occurs, the driver
loads the file again before launch rather than reusing the pre-mutation value.
An edit therefore affects the next session, while a missing or invalid config
leaves a rootless, legacy, current, or pending-migration tree byte-identical.
There is no cache across loop iterations.

## Session kinds live in filenames

The current leaf grammar is:

```text
NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md
```

The parser matches the longest member of the closed kind set after the optional
outcome infix. This is necessary because `review-design` and `design`, for
example, can both prefix a slug. The position remains mutable and the
`<slug>-k<key>` handle remains stable; kind is routing metadata, not identity.
Node directory names remain kind-free.

Every positioned, keyed Markdown filename is task-shaped and must contain a
known kind, whether it is live, `DONE`, or `ABANDONED`. An absent or unknown kind
is a malformed tree and stops reads and mutations with the path and canonical
kind set. Other foreign files remain ignored. There is no read-side degradation
to `impl` after migration.

Task bodies no longer carry `**Kind:**`, `**Harness:**`, or
`**Producer launch:**`. Stable `**Reviews:**` and `**Integrates:**`
relationships remain because they describe artifact composition rather than
launch policy. Grow verbs write the chosen kind in the filename. The research
pair produces `research-a`, `research-b`, and `combine-research`; it needs no
per-leaf harness metadata. `finish` is driver-reserved and is refused by every
generic grow, retire, and prune operation.

The optional Herdr tree viewer depends on this filename grammar as well as the
node-directory grammar. It parses the same longest known kind before the slug
and never opens a task body merely to render kind. Adding a session kind is
therefore a breaking configuration-schema and viewer-grammar change: the CLI,
embedded methodology, viewer, examples, and all complete personal configs must
move together.

## Authoritative selection and mandate

The driver performs exactly one authoritative pick per loop iteration, after
any required lifecycle mutation. That pick returns the selected leaf's absolute
path, stable handle, and filename kind from one guarded tree read. It is not a
routing forecast and is not repeated immediately before spawn.

The driver selects the matching configuration target and constructs `${prompt}`
with the stable handle as an explicit mandate. A configured command receives no
hidden leaf environment variable. At session Bootstrap, the agent:

1. resolves the mandated handle with `grove-llm resolve`;
2. rejects a missing, ambiguous, terminal, or non-leaf result;
3. reads the glossary, cited ADRs, brief chain, and resolved task; and
4. executes that task without calling `grove-llm pick`.

A leaf inserted earlier during the launch window does not preempt a session
already mandated to another stable handle; it becomes the next iteration's
work. A mandate that was removed or made terminal before Bootstrap causes the
session to stop without a completion signal, leaving a restart to derive the
new fact from the tree. The prompt-visible mandate, not checkout state or an
inherited environment value, is also the review-ownership predicate specified
by [Grove owns escalated review](../adr/grove-owns-escalated-review.md).

`grove-llm kind --with-harness --json`, structured routing peeks,
`GROVE_SESSION_TARGET`, producer target receipts, and diversity warnings have no
role in this flow and are removed.

## Single-command lifecycle

Bare `grove` is the only human lifecycle action. `do`, `migrate`, and `retire`
subcommands and lifecycle flags such as `--harness` and `--no-launch` are
removed. Standard `--help` and `--version` metadata remain; any other argument
is an error. `grove-llm` remains the agent-facing deterministic tree interface.

### Fresh tree

When `.grove/` is absent, after full config validation the driver creates the
root `BRIEF.md` and `01-requirements-plan-k1.md`. The leaf body is the ordinary
`plan-k1` requirements task with no kind marker. Creation is working-tree only;
the first requirements session's focused commit folds in the scaffold.

The driver then picks `plan-k1`, reloads config, and launches the
`requirements` target with that handle as its mandate. A launch failure leaves a
real resumable leaf rather than a brief-only tree or hidden bootstrap state.

### Existing live tree

A current-format tree with a live leaf needs no lifecycle mutation. The driver
picks the first live leaf in depth-first pre-order, reloads config, expands that
kind's target, and launches. Completion signaling is unchanged: `relaunch`
continues with a fresh iteration, `done` stops cleanly, and an absent signal
stops without changing tree state.

### Finish leaf

When the lifecycle liveness read finds no live leaf, after full config
validation the driver appends a root leaf at the next position and with
`max(tree key) + 1`:

```text
NN-finish-finish-k<key>.md
```

Its stable handle is `finish-k<key>`. A finish leaf left live by an earlier
decline makes the liveness read non-empty and is therefore reused without any
allocation. The ordinary authoritative pick then selects the existing or newly
created leaf, and the `finish` configuration target launches a HITL session.

The finish session proposes the complete finish cycle and waits for explicit
human confirmation. Declining or exiting writes no completion signal and leaves
the same live finish leaf for a later bare `grove`. On confirmation it promotes
durable brief material, commits deletion of `.grove/` under a message naming
`finish-k<key>`, and runs `grove-llm complete --done` last.

The finish leaf is working-tree-only infrastructure for that deletion. It is
never separately committed or retired: its addition and deletion cancel while
the tracked task tree is removed in the focused finish commit. Generic terminal
verbs reject it so an accidental `DONE` cannot make teardown look complete.

## Legacy migration

Migration is automatic inside bare `grove`; there is no human-facing migrate
command. One migration converts both older directory layouts and the old
task-body kind format into the current tree, then records one focused commit.
An already-current tree is a no-op.

### Accepted inputs and mapping

Migration accepts the original `NNN-slug/` tree, the v1 flat dotted-decimal
tree, and the v2 directory tree whose leaves lack filename kinds. It processes
live, `DONE`, and `ABANDONED` leaves identically. For each legacy task body:

- one known `**Kind:**` value supplies the filename kind;
- an absent kind uses the former read default, `impl`;
- `work`, `review-work`, and `integrate-review-work` map to their `impl`
  spellings;
- `requirements`, `design`, `planning`, `prototype`, `impl`, their review and
  integration forms, and `combine-research` map directly;
- the first and second `research` children of an unambiguous legacy vendor pair
  map to `research-a` and `research-b` respectively.

An unambiguous vendor pair is one brief-less node whose first three task
children, in position order, are `research`, `research`, and
`combine-research`. Terminal outcomes do not alter this classification. Extra
foreign files are ignored; an extra task-shaped child or nested task node makes
the pair structurally ambiguous. A standalone `research`, an empty or repeated
kind marker, an unknown kind, or an ambiguous pair stops migration and names
the exact paths rather than guessing a configured target.

The established directory mapping remains deterministic. A v1 flat entity's
dotted position becomes its chain of per-level positions and its existing key is
preserved. In an original `NNN-slug/` tree, siblings sort by `NNN`, receive
gapless per-level positions, and receive fresh keys in depth-first pre-order;
a node brief receives the node's key before its descendants. The unkeyed root
brief remains at `.grove/BRIEF.md`. Existing `DONE` outcomes survive; original
formats that predate `ABANDONED` cannot synthesize one. Headers are rewritten to
the resulting position-free handles.

The destination filename receives the mapped kind. Migration removes every
`**Kind:**`, `**Harness:**`, and `**Producer launch:**` line while preserving all
other bytes, including `**Reviews:**` and `**Integrates:**`. Directory
migration and kind migration are planned together, so no successful invocation
exposes an intermediate layout as current.

A tree without a migration witness must be wholly legacy or wholly current. A
mixture of kind-bearing and body-kind leaves is diagnosed as an interrupted or
hand-edited conversion and is not guessed into either grammar. Foreign entries
that are not task-shaped remain byte-identical at their existing relative paths;
any collision between them and a planned current path stops before mutation.

### Fail-closed transaction and recovery

Migration holds the exclusive tree-access lock from source validation through
commit or reported failure. Before changing a source path it constructs a
complete destination tree and deterministic source/destination plan beneath a
reserved `.grove/MIGRATING-session-kinds/` witness. The witness contains the
untouched originals needed for rollback and the staged destination. Its
presence alone makes every other tree reader and mutator refuse; diagnostics
name the witness and instruct the operator to rerun bare `grove`.

Landing moves original root entries into the witness and destination entries
into their final positions according to that plan. Recovery infers progress
from each entry's source, staged, and final location; it neither reparses a
partially migrated root as a live tree nor allocates new keys. After config
validation, bare `grove` resumes that exact transaction before ordinary format
detection. A reported pre-commit failure attempts rollback. Failed rollback or
process interruption leaves the witness intact and the tree unwalkable.

Once every final path and body is verified, Grove commits only the `.grove/`
migration paths while excluding the witness, then removes the witness. A crash
after the commit but before witness removal is recovered by verifying the
committed final tree and removing the now-redundant witness. The transaction
promises process-interruption consistency, not power-loss durability; it adds no
ordered `fsync` protocol.

The migration commit message identifies the grove and migration, not a mutable
path. There is no work-item handle because the driver performs migration before
launching a task session.

### Scoped Git and Jujutsu commits

Migration and finish commits preserve unrelated user work.

- In plain Git, Grove stages final `.grove/` paths with an exclusion pathspec for
  any migration witness, then commits with the same explicit `.grove/` pathspec
  in only/path mode. Pre-existing staged entries outside that path remain staged
  and absent from the Grove commit.
- In Jujutsu, Grove commits a `.grove/` fileset, excluding any live transaction
  witness. Unrelated working-copy changes remain in the successor working-copy
  commit.

The same repository seam continues to resolve jj first, use filesystem renames
in jj-enabled workspaces, and use Git only in plain Git trees.

## Removed surfaces and compatibility

The design removes, rather than deprecates, these launch-policy surfaces:

- harness detection for launching, the harness stamp, `.grove-stamps/`, and
  per-invocation harness flags;
- all `GROVE_<KIND|FAMILY>_HARNESS` and `GROVE_*_MODEL` routing variables;
- user-settable Grove executable, skill-dir, and kill-grace overrides;
- leaf `**Harness:**` metadata and grow-verb harness flags;
- task-body kind markers and their read-side `impl` degradation;
- `grove do`, `grove migrate`, `grove retire`, and dry-run routing output;
- structured harness-routing peeks, target receipts,
  `GROVE_SESSION_TARGET`, and review diversity warnings;
- hidden model flags, Codex grants, session-name arguments, turn-hook arguments,
  and `HERDR_AGENT` injection.

Internal loop-control environment is not a compatibility surface. The driver
continues to scrub ambient control variables from non-foreground child commands
and grants only its own signal path to the real foreground session.

The configuration schema has no partial compatibility mode. Adding or renaming
a session kind requires a release note and a simultaneous edit to every complete
personal config. Older config fails with the exact missing/unknown kinds before
tree mutation. Older trees are supported only through the automatic migration
above; once migrated, no dual-format reader remains.

Global skill provisioning survives but no longer selects or implies a launch
harness. The binary still sweeps embedded `content/` to each installed known
personal skill directory. That delivery registry is independent of the opaque
configured commands and never contributes arguments or target identity.

## Module interfaces

The configuration module is deep. Its external interface loads one fixed file
into a complete kind-to-template map and expands one selected template from a
context containing prompt, session name, worktree, repository root, and optional
Herdr settings. It hides KDL syntax handling, aggregate schema diagnostics,
shell-word parsing, placeholder validation, and argv construction. Callers
cannot request a default, family, harness, or model.

The loop driver owns lifecycle order and one selected-leaf value per iteration.
It asks the tree module to recover or perform the required transition, asks for
one pick, reloads configuration, expands one command, and owns the foreground
child. It does not re-open the tree through a routing adapter.

The tree module owns the leaf grammar, driver-only finish creation, current
pick, and migration transaction. The repository module owns worktree/main-repo
resolution and path/fileset-scoped commits. The Herdr module may produce the two
settings arguments, but they cross into a session only when the visible
`${herdr_settings}` splice requests them.

Deleting the configuration module would scatter KDL validation, shell-word and
substitution rules, source diagnostics, and argv construction across every
lifecycle branch. Its small load/expand interface therefore earns the seam. No
harness abstraction replaces the removed registry: opaque command targets have
one production adapter, direct process execution, so another port would be
hypothetical indirection.

## Test seams

The primary behavioral seam is the bare `grove` process in isolated temporary
Git, native jj, and colocated jj worktrees. Tests provide an isolated home with a
real `config.kdl` and executable fake commands that record argv, cwd,
environment, prompt mandate, and completion behavior.

Through that seam, cover:

- all KDL syntax, shape, duplicate/unknown/missing-kind, shell-word, and
  substitution diagnostics, including aggregation and source spans;
- one-argument scalar substitution, zero/two-argument Herdr expansion, paths
  with spaces, prompt in non-final position, literal word zero, and absence of
  shell evaluation;
- reloading between iterations and between a lifecycle mutation and launch;
- no mutation for missing or invalid config in rootless, legacy, current,
  empty, and pending-migration trees;
- fresh root creation, one authoritative selection, mandate resolution, a
  launch-window insert, spawn failure and restart;
- finish allocation, reuse, decline, generic-terminal refusal, deletion commit
  naming, and clean `--done` stop;
- migration of every accepted layout, aliases, terminal leaves, missing kinds,
  vendor pairs, metadata removal, relationships, every ambiguity, collision,
  interruption point, rollback failure, and post-commit witness recovery;
- Git staged-change preservation and jj working-copy preservation for migration
  and finish commits;
- direct foreground ownership, no hidden argv/env injection, signal/no-signal
  outcomes, and explicit versus absent Herdr settings.

The `grove-llm` tree interface is the second seam. Exercise current filename
parsing, longest-kind matching, malformed task-shaped names, stable resolution,
pair generation, finish refusal, pick order, and migration refusal while a
witness exists. The Herdr renderer gets filename-only fixtures for all nineteen
kinds and both terminal infixes.

Internal unit tests may cover pure KDL/template and migration-plan functions,
but acceptance is stated only in observable process, tree, VCS, and argv terms;
tests do not reach through those interfaces to implementation state.

## Out of scope

- Choosing commands, models, reasoning effort, approval mode, or sandbox policy
  for the user, or creating/editing their personal config.
- Inferring the harness behind a configured executable or wrapper.
- Profiles, includes, defaults, family inheritance, repository-local config, or
  environment overrides.
- A shell command language inside configuration.
- Enforcing cross-harness or cross-model review diversity.
- Changing pick order, review relationships, promotion semantics, pruning
  authority, or completion-signal behavior.
- Power-loss durability or branch/bookmark/worktree integration.
