# config-driven-sessions

## Problem

Grove currently reconstructs one session command from several independent
sources: a repository marker and local stamp select a primary harness; task
bodies and environment variables can reroute a leaf; more variables select a
model; and harness-specific code appends naming and sandbox arguments.
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
resolves and version-checks its sibling `grove-llm`, validates configuration
before any task-tree mutation, brings the tree to a runnable current shape,
performs one authoritative pick, reloads configuration, and launches the
selected kind. The selected leaf's stable handle is embedded in `${prompt}` as
the session mandate. A launched session resolves that handle and never picks
again.

The resulting flow is:

```text
bare grove
  -> provision embedded methodology independently of launch policy
  -> acquire this working tree's driver lease
  -> resolve the sibling grove-llm and reject version skew
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

`grove --help` and `grove --version` stop before this flow: they provision
nothing, discover no repository, and acquire no lease. On the bare lifecycle
path, provisioning precedes ownership so a refused second driver still receives
the independently delivered methodology.

## Configuration file

### KDL shape

The document is a flat set of nineteen top-level nodes. A node name is the
session kind and its sole positional argument is a KDL string containing the
complete command template. Nodes have no properties and no child blocks.
Comments and insignificant ordering are permitted.

```kdl
requirements "claude --model opus ${prompt}"
review-requirements "grove-codex-review ${worktree} ${prompt}"
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

finish "claude --model opus ${prompt}"
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
| `${prompt}` | The embedded `continue.md` launcher plus the selected leaf's stable-handle mandate, as one argument. Required exactly once. |
| `${session_name}` | `<repo-basename>: <grove-name> grove`, as one argument. Optional. |
| `${worktree}` | The absolute root of the working tree that owns `.grove/`, as one argument. Optional. |
| `${repo}` | The absolute root of the main repository: the default jj workspace root, or the parent of Git's common directory. Optional. |

Every substitution occupies a complete parsed word. Embedded substitutions,
unknown `${...}` names, a substitution in word zero, and more than one use of
the same substitution are errors. `${prompt}` may appear at any nonzero argv
position; it is not required to be last. Each optional substitution appears
zero or one time.

The first parsed word is a non-empty literal executable or script name. “Word
zero” means exactly that first shell-split word: in `env MODE=review runner
...`, it is `env`, while the assignment and `runner` are ordinary later
arguments. Grove passes those words directly with the working tree as the
current directory. Shell variables, command substitutions, redirections,
pipelines, globs, aliases, and tilde expansion are not interpreted. A user who
needs those behaviors places them in an executable wrapper and configures that
wrapper as word zero.

An unquoted `#` reached between shell words would make the word splitter treat
the rest of that line as a comment. Grove rejects that form rather than silently
launching a truncated command. Quote or escape the `#` to pass it literally;
an unquoted `#` inside an existing word, such as `tag#1`, is already literal.

Substitution values are argv values, not text splices, so spaces or shell
metacharacters in repository paths, session names, and prompts never change
argument boundaries.

Grove adds no hidden harness-specific arguments or environment values. The
configured command owns executable choice, harness, model, reasoning effort,
approval and sandbox policy, session-name flags, and repository grants. Grove
still owns its temporary loop-control channel, child lifecycle, current
directory, and the generated prompt. Those are orchestration, not user launch
policy.

Immediately before spawn, Grove removes any inherited `GROVE_SIGNAL_FILE`,
retired `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID`, and stale
`GROVE_SESSION_TARGET`, then grants only the fresh signal path for this launch.
It does not remove or override `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR`, or
a repository-local `core.worktree` for the opaque configured command. The
configured owner expresses any such Git policy with literal `env` arguments or
an executable wrapper; the driver's working-tree current directory is not a
promise to rewrite the command's repository context.

### Validation and diagnostics

Configuration is all-or-nothing. A successful load proves:

- the exact path exists, is readable, and parses as KDL;
- every required kind occurs exactly once;
- no unknown top-level node, property, child block, or extra argument exists;
- every node's sole argument is a string;
- every template shell-splits successfully and obeys the executable,
  substitution, and `${prompt}` rules above, including a specific missing-
  `${prompt}` error.

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
later in an `env` invocation remain opaque by design. The foreground wait keeps
the child's exit status and elapsed time. When a session ends without a
completion signal, the driver reports both. A nonzero exit additionally names
the selected kind, configured word zero, and config path as a likely configured-
command failure. This diagnostic applies regardless of how quickly the process
failed; a time threshold would hide slow failures without making an intentional
zero-status exit clearer.

Failure to spawn is a driver error and bare `grove` exits nonzero. Once the
configured command has spawned, ending without a completion signal is a
successful stop of the driver loop, so bare `grove` exits zero even when the
child status is nonzero; the preserved status and launch-identity diagnostic
carry that distinction to wrappers and operators.

The driver reads and validates the whole file immediately before every launch.
It also validates before root initialization, migration or migration recovery,
and finish-leaf materialization. When one of those mutations occurs, the driver
loads the file again before launch rather than reusing the pre-mutation value.
An invalid pre-mutation read leaves the corresponding rootless, legacy,
current, or pending-migration tree byte-identical. If the file becomes invalid
after a successful mutation and before the second read, that completed mutation
remains as resumable tree state (and migration may already have its focused
commit), while no session launches. There is no cache across loop iterations.

## Session kinds live in filenames

Every current tree carries a positive format witness at `.grove/FORMAT` whose
exact contents, including the terminating LF, are:

```text
session-kinds-v1
```

Root initialization and legacy migration write it only after the complete
current tree is ready. They create a same-directory temporary file and atomically
rename it to `FORMAT`, so a process interruption exposes the old value or the
complete new value, never a torn marker; ordered power-loss durability remains
out of scope. Before interpreting absence as legacy, bare `grove` first recovers
a migration witness or an exact partial root-init scaffold. Ordinary current-
format readers require the known value, while an unknown value stops with an
upgrade diagnostic. This marker is format metadata inside the task tree, not
lifecycle state. It makes “already current” independent of slug text: a legacy
`01-design-notes-k3.md` can no longer masquerade as a kind-bearing current leaf
merely because `design` is a valid kind.

The current leaf grammar is:

```text
NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md
```

After the optional outcome infix, the parser separates exactly one member of
the closed kind set. The set maintains a non-prefix invariant: no kind label
followed by `-` prefixes another kind label. Without that invariant, rendering a
shorter kind plus a slug could produce the same bytes as a longer kind plus a
different slug, so longest matching alone could not preserve identity. The
position remains mutable and the `<slug>-k<key>` handle remains stable; kind is
routing metadata, not identity. Node directory names remain kind-free.

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
per-leaf harness metadata.

`finish` is driver-reserved. `leaf-add`, `leaf-insert`, `leaf-decompose`, and
`leaf-add-chain` reject `--kind finish`; `leaf-add-pair` has fixed research
kinds. Given an existing finish leaf, `leaf-retire`, `leaf-prune`,
`leaf-decompose`, and `leaf-promote-chain` reject it as an operand. A normal
`leaf-insert <finish> <slug> --kind <non-finish>` is permitted and sequences the
new work before teardown; ordinary `leaf-add` may also append later work because
finish selection cannot starve it.

Adding a session kind is a breaking configuration-schema and filename-grammar
change: the CLI, embedded methodology, examples, and all complete personal
configs must move together.

## Authoritative selection and mandate

The driver performs exactly one authoritative pick per loop iteration, after
any required lifecycle mutation. That pick returns the selected leaf's absolute
path, stable handle, and filename kind from one guarded tree read. It is not a
routing forecast and is not repeated immediately before spawn. The only
eligibility rule beyond terminal-state filtering is driver-owned `finish`: if
any non-finish leaf is live, the walk skips finish leaves and returns the first
live non-finish leaf in depth-first pre-order; a finish leaf becomes eligible
only when it is the sole live work. `grove-llm pick` uses the same rule so its
diagnostic answer never disagrees with the driver. More than one live finish
leaf is malformed and stops selection rather than making eligibility depend on
which duplicate is encountered first.

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

That authority split also governs `leaf-promote-chain`. The command accepts the
named live producer after its normal producer-kind, relationship, parent-shape,
and transaction checks, but it does not recompute pick or require the producer
to be the current walk result. The prompt mandate is an agent-side authorization
the tree command cannot observe; a second pick would reject the legitimate
launch-window insertion case above while proving nothing about what the session
was mandated to do. Completed-shape and pending-transaction recovery remain
idempotent by stable producer identity. This is workflow discipline, not a
security capability: the command trusts the explicit producer argument. The
[one-live-driver-per-working-tree](../adr/one-live-driver-per-working-tree.md)
decision excludes concurrent drivers and stale sessions without restoring
hidden target metadata.

`grove-llm kind --with-harness --json`, structured routing peeks,
`GROVE_SESSION_TARGET`, producer target receipts, and diversity warnings have no
role in this flow and are removed.

## Single-command lifecycle

Bare `grove` is the only human lifecycle action. `do`, `migrate`, and `retire`
subcommands and lifecycle flags such as `--harness` and `--no-launch` are
removed. Standard `--help` and `--version` metadata remain; any other argument
is an error. `grove-llm` remains the agent-facing deterministic tree interface.

### Toolchain and serialization

On every iteration the driver resolves `grove-llm` beside its own current
executable, falling back to `PATH` only when no sibling file exists. There is no
user override. Before configuration validation or tree mutation it runs that
exact binary's version check and stops on a missing, malformed, or different
version. Version skew is a resumable no-mutation stop.

### Process ownership and session epochs

The bare lifecycle path provisions embedded content first, then resolves the
working tree and acquires its **driver lease** before configuration validation
or any `.grove/` observation or mutation. The
repository adapter derives a control directory from the closest on-disk VCS
marker for that exact workspace, without invoking VCS discovery or honoring
repository-selection environment. A `.jj/` beside `.git` wins. For native,
secondary, and colocated jj it uses `<canonical-workspace-root>/.jj/grove/`
without following `.jj/repo` to the shared repository store or consulting the
default workspace. For plain Git, a `.git/` directory maps to
`<canonical-.git>/grove/`; a linked-worktree or submodule `.git` file is parsed
as the standard `gitdir:` pointer, resolved relative to the file, canonicalized,
and given its own `grove/` child. It never uses Git's common directory.
`GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR`, and the other Git discovery
variables cannot redirect this resolver. Nothing is created in the tracked
working copy, and neither `TMPDIR` nor another ambient path chooses the
coordination namespace. A native jj workspace, a colocated jj workspace, a
plain Git checkout, and a linked Git worktree therefore each get one workspace-
scoped control directory.

The same isolation applies to lifecycle VCS children. Before migration
inspection, recovery, or commit, the driver scrubs stale Grove controls and
repository selectors. Jujutsu then resolves from the authoritative workspace
current directory; Git additionally receives `GIT_WORK_TREE` for the leased
working tree, which overrides a conflicting local `core.worktree`. These
internal commands are orchestration and do not inherit the configured
foreground command's repository policy.

The driver opens the working-tree root, records its filesystem device and inode,
and opens the fixed lease file in that control directory. Acquisition takes a
nonblocking exclusive advisory lock, then compares `fstat` of the locked
descriptor with `stat` of the path. A mismatch means the path was replaced
between open and lock; the module closes it and retries up to eight times before
returning a control-file-replaced error. The open root descriptor pins the
worktree identity. Symlink and relative-path aliases contend on one lease, while
different worktrees or workspaces remain independent even when they share a
repository or basename.

Contention is not a tree operation to queue. A second driver exits nonzero
immediately, names the canonical working tree, says the existing driver must
stop, and leaves the existing driver as owner. The owner holds the root and
lease descriptors until the loop has handled its terminal
signal/no-signal/error disposition. Before every
lifecycle transition and foreground launch it re-stats the lease path against
the held descriptor; loss stops the loop visibly before more work. Normal
return, panic, and process death close the descriptor and release the kernel
lock. Leftover bytes carry no ownership and require no PID cleanup.

The locked lease record contains the working-tree identity and a fresh 128-bit
driver nonce read from the operating system's cryptographic randomness source.
The nonce is generated once per driver process and is never derived from a PID,
wall clock, address, hasher default, or task value. Every descriptor opened by
the protocol is close-on-exec, so an opaque configured command cannot pass
ownership to a descendant.

The same control directory contains one stable **session epoch** file. The
driver rewrites it under an exclusive guard at exactly three points:

1. immediately after acquiring the lease, write an inactive record carrying the
   new nonce and working-tree identity, invalidating a crashed predecessor;
2. immediately before each spawn, write an active record containing that nonce,
   identity, and the launch's freshly drawn absolute `GROVE_SIGNAL_FILE`; and
3. immediately after reaping the child, write an inactive record before reading
   or acting on the completion signal.

Every shared or exclusive epoch acquisition uses the same open, lock,
`fstat`/`stat`, bounded-retry sequence as lease acquisition before reading or
writing the record. This closes replacement between open and lock; deliberate
unlink/recreate outside that acquisition window remains unsupported repository-
control mutation. Each exclusive guard is a distinct lexical scope. It is
released before another epoch acquisition, every Tree access operation, and
foreground spawn; a guard-owning value cannot be reused for a second
acquisition. This removes the self-deadlock where one process opens a second
descriptor and blocks against its own earlier lock.

The signal file lives beside the controls and receives a fresh, independent
128-bit OS-random suffix for every launch. Its path supersedes the former
`signal_file_path` deterministic-stability contract: Grove intentionally draws
a new path for each launch and retries a path already present before spawn
rather than clearing or truncating another channel.
Allocation chooses an absent name but creates no file; the foreground session
materializes the channel only when it signals. A spawn failure therefore leaves
no signal artifact to clean up.
The current driver removes the file only after post-reap epoch invalidation and
signal interpretation. After a new driver owns the lease and has exclusively
installed its inactive epoch, it removes abandoned `signal-*` files from crashed
launches. With no durable tombstone, a cleaned path can be redrawn after restart;
the accepted collision bound is at most one in `2^128` per independent draw,
not literal non-reuse. The same bound applies to two driver processes drawing
the same nonce. This statistical freshness is the explicit trade-off for no
durable grove generation. `GROVE_SIGNAL_FILE` remains the only loop-control value
exported to the configured command; no target, kind, model, nonce, or generation
value is exported.

When `GROVE_SIGNAL_FILE` is present, every agent-facing `grove-llm` operation
that reads or mutates the task tree, plus `complete`, first acquires a shared
guard on the epoch file and retains it through the whole operation. It verifies
the exact signal path, driver nonce, and working-tree identity. To observe lease
liveness it opens and identity-revalidates the lease path, reads the matching
lease record, and tries `LOCK_EX|LOCK_NB` on that separate descriptor. Success
means no driver owns the lease: the probe is closed immediately, releasing the
lock, and the command fails stale. `EWOULDBLOCK` plus a matching nonce is the
advisory live result. If the driver dies after this probe, the shared epoch guard
still prevents a replacement driver from invalidating the record until the
already-admitted operation returns.

A cwd that resolves to another working tree receives a wrong-worktree
diagnostic naming both roots. A missing, inactive, unlocked, malformed, or
mismatched epoch receives a stale-session diagnostic. Both refuse before tree
access or completion signaling. Pure `grove-llm --version` is explicitly
exempt. A command invoked manually without `GROVE_SIGNAL_FILE` remains an
ordinary human/diagnostic tree command.

Every epoch lock acquisition first tries without blocking. On contention it
prints one diagnostic naming the operation and lock mode, then waits for a fixed
internal 30-second handoff bound. Timeout returns an error without tree access,
epoch rewrite, or signal interpretation. The four acquisition sites are:

- driver acquisition: exclusive, normally waiting only for an operation admitted
  before the predecessor died;
- pre-spawn activation: exclusive, with contention treated as an invariant or
  orphan failure rather than silently hanging;
- post-reap invalidation: exclusive, possibly waiting for a descendant that
  outlived the reaped foreground child; and
- ambient agent operations: shared, possibly waiting for a brief driver rewrite.

Acquiring the shared epoch guard is the admission boundary. A command admitted
before exclusive invalidation may finish even when its lease probe overlaps a
replacement driver's acquisition; its guard prevents that replacement from
installing the next epoch or touching the tree. Calls that begin after exclusive
invalidation see the new inactive record or nonce and fail. If an orphan retains
a shared guard after its parent is SIGKILLed, post-reap
invalidation waits once and then stops the loop `blocked` at the bound. It does
not consume the completion signal or start another session. Driver exit releases
the lease; a replacement driver waits for any already-admitted operation before
installing its inactive epoch, after which new calls from that orphan fail. This
turns the former silent between-session park into a bounded, diagnosable stop
without weakening crash handoff.

The lock order is fixed: a driver takes the lease, then a separately scoped
exclusive epoch guard, releases it, and only then enters task-tree operations;
an ambient `grove-llm` command takes a shared epoch guard before the Tree access
lock. The driver never waits for an epoch guard while holding a Tree access
guard. The lease serializes loop lifetimes, the epoch serializes launch
authority, and the Tree access lock serializes individual tree observations and
mutations.

Stable handles remain scoped to the live task tree and carry no persistent
grove-generation suffix. Across finish deletion and later root initialization,
even if a new tree reuses `plan-k1`, the old session cannot resolve, mutate, or
complete **through `grove-llm`** because its epoch is inactive and its random
signal path and driver nonce no longer match, subject to the accepted 128-bit
collision bound. The protocol does not prevent a stale process
from directly editing files, committing, or writing a known signal path outside
the agent CLI; it is workflow consistency among cooperating processes, not
authentication. Similarly, external deletion or replacement of files in the
VCS administration area is repository-control corruption, not a supported
concurrent operation; no survival or next-transition detection guarantee is
made for unlink/recreate outside an acquisition window. The fixed lease and
epoch bytes are untracked
and meaningful only while locked, so `.grove/` remains the only durable workflow
state. This follows the
[one-live-driver-per-working-tree](../adr/one-live-driver-per-working-tree.md)
decision.

Every cooperating tree reader and mutator serializes on one advisory lock over
an open descriptor for the working-tree root, which exists before `.grove/` and
survives its deletion. Readers hold it shared; root initialization, migration,
finish allocation or deletion, and every ordinary tree mutation hold it
exclusive. A contended command prints one waiting diagnostic and then waits.
This replaces the narrower `.grove/`-descriptor lock: separate lifecycle and
tree locks would require a global acquisition order and still leave root
creation and finish deletion outside the invariant. Lock descriptors are close-
on-exec. A driver releases its read guard immediately after copying the selected
path, handle, and kind, before the second config read or foreground spawn. This
is the Tree access read guard, not the driver lease: the session can acquire an
exclusive tree-mutation guard while the driver continues to own the loop, and
cannot inherit a lock that outlives the driver-side operation.

The lock supplies live-process serialization, not crash atomicity. Each multi-
path operation that promises process-interruption recovery still needs its own
in-tree witness and recovery protocol; migration and promotion retain theirs.
Single-path renames rely on filesystem atomicity, and no operation gains a
power-loss guarantee merely by sharing this lock.

### Fresh tree

When `.grove/` is absent, after full config validation and while holding the
exclusive working-tree lock, the driver creates the root `BRIEF.md`,
`01-requirements-plan-k1.md`, and finally the format witness. The leaf body is
the ordinary `plan-k1` requirements task with no body kind marker. Creation is
working-tree only; the first requirements session's focused commit folds in the
scaffold. If the process stops partway through, the next invocation recognizes
only an exact subset of this deterministic scaffold *before* general missing-
marker classification, completes it under the same lock, and atomically replaces
`FORMAT` last; any other unmarked entries route to migration or an ambiguity
diagnostic rather than being overwritten.

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
validation and under the exclusive tree lock the driver appends a root leaf at
the next position and with `max(tree key) + 1`:

```text
NN-finish-finish-k<key>.md
```

Its stable handle is `finish-k<key>`. A finish leaf left live by an earlier
decline is reused without any allocation. If later work appears anywhere in the
tree, authoritative selection skips the finish leaf until that work is
terminal; when finish is again the sole live leaf, the same stable handle is
selected. The `finish` configuration target then launches a HITL session.

The finish session proposes the complete finish cycle and waits for explicit
human confirmation. Declining or exiting writes no completion signal and leaves
the same live finish leaf for a later bare `grove`. On confirmation it promotes
durable brief material, runs `grove-llm finish-commit <finish-handle>`, and then
runs `grove-llm complete --done` last. `finish-commit` reacquires the exclusive
working-tree lock, rejects any pending transaction, re-resolves the same live
finish handle, and revalidates that no non-finish leaf is live before deleting
or committing anything. If work appeared after launch, it names that work and
leaves the tree byte-identical; the session exits without a completion signal so
the next driver iteration selects the new work. On success the helper commits
deletion of `.grove/` under a message naming `finish-k<key>`.

The helper cannot attest that a human spoke through an opaque configured
command. Explicit confirmation remains a finish-session obligation enforced by
the embedded Grove methodology; `finish-commit` is the deterministic
last-moment tree/VCS guard, not a security boundary or substitute for the HITL
contract.

No commit is made *for* the finish leaf and it is never retired. Its addition
and deletion cancel from the final tree in the focused finish commit. Jujutsu
may snapshot the live finish leaf in an intermediate working-copy commit, and a
broad Git task commit may pick it up; neither changes the contract, because the
successful deletion commit removes the whole tree and version control retains
the intermediate history. Generic terminal verbs reject finish so an accidental
`DONE` cannot make teardown look complete.

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
  map to `research-a` and `research-b` respectively;
- a standalone legacy `research` maps to `research-a`. A lone current
  `research-a` is legal; the kind names one configured research discipline, not
  structural membership in a pair.

An unambiguous vendor pair is one brief-less node whose first three task
children, in position order, are `research`, `research`, and
`combine-research`. Terminal outcomes do not alter this classification. Extra
foreign files are ignored; an extra task-shaped child or nested task node makes
the pair structurally ambiguous. An empty or repeated kind marker, an unknown
kind, or an ambiguous pair stops migration and names the exact paths rather
than guessing a configured target.

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
exposes an intermediate layout as current. The format witness lands last and is
part of the same focused migration commit.

A tree without `.grove/FORMAT` is legacy regardless of whether a legacy slug
begins with a current kind token. Migration parses those leaves through the
legacy grammar and their body markers; it never first strips a filename prefix
that happens to spell `design`, `review-impl`, or another current kind. A tree
with the known marker is wholly current and body kind/harness/receipt markers
are malformed remnants. A tree with an unknown marker value is a newer or
foreign format and stops without mutation. Foreign entries that are not task-
shaped remain byte-identical at their existing relative paths; any collision
between them and a planned current path stops before mutation.

### Fail-closed transaction and recovery

Migration holds the exclusive working-tree lock from source validation through
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

- In plain Git, migration stages with
  `git add -A -- .grove ':(exclude).grove/MIGRATING-session-kinds'`, then
  commits with
  `git commit --only -m <message> -- .grove ':(exclude).grove/MIGRATING-session-kinds'`.
  Finish runs `git add -A -- .grove`, then
  `git commit --only -m <message> -- .grove` after deletion. Git's only/path
  mode includes tracked deletions under an absent
  working-tree directory and works for an initial migration commit with no
  `HEAD`; tests exercise both facts. Pre-existing staged entries outside
  `.grove/` remain staged and absent from either commit. A valid Grove cannot
  reach finish in an unborn repository without first recording either its
  migration or a task commit; an externally hand-constructed terminal tree that
  has never existed in `HEAD` is refused because there is no deletion for a
  focused finish commit to record.
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
- hidden model flags, Codex grants, session-name arguments, and other
  harness-specific argument or environment injection.

Internal loop-control environment is not a compatibility surface. The driver
continues to scrub ambient control variables from non-foreground child commands
and grants only its own signal path to the real foreground session.
`GROVE_SIGNAL_FILE` remains that internal capability. Test suites inject tool
paths, clocks, and kill-grace durations through internal module seams rather
than supported `GROVE_LLM_BIN` or `GROVE_KILL_GRACE*` process configuration.

The configuration schema has no partial compatibility mode. Adding or renaming
a session kind requires a release note and a simultaneous edit to every complete
personal config. Older config fails with the exact missing/unknown kinds before
tree mutation. Older trees are supported only through the automatic migration
above; once migrated, no dual-format reader remains.

Global skill provisioning survives but no longer selects or implies a launch
harness. The binary still sweeps embedded `content/` to each installed known
personal skill directory. That delivery registry is independent of the opaque
configured commands and never contributes arguments or target identity. The
single surviving launcher is `content/prompts/continue.md`, augmented with the
selected stable-handle mandate; `start.md` and `retire.md` disappear with their
human lifecycle verbs. The launcher deliberately remains small and tells the
target to use the provisioned Grove skill, so that provisioned methodology is a
prerequisite of every configured target. Because target identity is opaque,
Grove cannot prove that an arbitrary wrapper ultimately exposes the skill; a
target without it fails as a configured session and receives the exit-status
diagnostic rather than a guessed harness-specific provisioning action.

`leaf-add-pair` remains the structural one-call constructor, with the signature
`leaf-add-pair <parent> <stem>` and no harness flags. It emits `research-a`,
`research-b`, and `combine-research`; choosing materially independent commands
for the two research kinds is explicit configuration-owner policy, not a tree
invariant Grove can verify by comparing opaque templates.

## Module interfaces

The configuration module is deep. Its external interface loads one fixed file
into a complete kind-to-template map and expands one selected template from a
context containing prompt, session name, worktree, and repository root. It hides
KDL syntax handling, aggregate schema diagnostics, shell-word parsing,
placeholder validation, and argv construction. Callers cannot request a
default, family, harness, or model.

The loop driver owns lifecycle order and one selected-leaf value per iteration.
It asks the tree module to recover or perform the required transition, asks for
one pick, reloads configuration, expands one command, and owns the foreground
child. Its wait result preserves exit status and elapsed time. It does not
re-open the tree through a routing adapter.

The process-ownership module exposes a small interface: acquire one driver
lease for a resolved working tree, activate/invalidate one session epoch around
a spawn, and validate optional ambient session context while returning a guard
held through one `grove-llm` operation. It hides workspace-administration path
resolution, locked-path identity revalidation, filesystem-identity records,
OS-random nonces and signal names, bounded advisory-lock acquisition, lease
probing, epoch serialization, stale-signal cleanup, and crash handoff. Internal
dependencies supply the control-directory resolver, randomness source,
monotonic clock, wait policy, and a lock/filesystem backend to tests; explicit
post-open and post-lock barriers plus an event trace make race placement and
guard lifetime deterministic without exposing those hooks to callers. The
production backend is the real filesystem and advisory-lock implementation;
none of these dependencies is an external environment or configuration
interface. The loop driver and agent CLI use the same module, so no caller
reimplements the race-sensitive protocol.

The tree module owns the format witness, leaf grammar, finish eligibility,
driver-only finish creation, guarded finish commit, current pick, universal
working-tree lock, and migration transaction. The repository module owns
worktree/main-repo resolution and path/fileset-scoped commits.

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
  substitution diagnostics, including missing `${prompt}`, aggregation, and
  source spans;
- scalar substitution, paths with spaces, prompt in non-final position, literal
  `env` as word zero, and absence of shell evaluation;
- reloading between iterations and between a lifecycle mutation and launch;
- no mutation for a pre-mutation missing or invalid config in rootless, legacy,
  current, empty, and pending-migration trees; plus a post-mutation invalid edit
  that preserves the completed transition but launches nothing;
- sibling/PATH `grove-llm` resolution and fatal missing/malformed/version-skew
  checks before mutation;
- metadata-only `--help`/`--version`; provisioning before lease acquisition on
  the bare path; skill refresh on a refused second driver; and an unwritable
  workspace-administration control directory failing before configuration or
  tree access;
- fresh root creation and partial-scaffold recovery under the universal lock,
  atomic format replacement, one authoritative selection, mandate resolution,
  a launch-window insert, spawn failure and restart;
- finish allocation, reuse, decline followed by later work, finish eligibility,
  duplicate-finish refusal, per-verb reservation, `finish-commit` revalidation
  when work appears after launch, deletion commit naming, Jujutsu/Git
  intermediate finish snapshots, and clean `--done` stop;
- migration of every accepted layout, aliases, terminal leaves, missing kinds,
  standalone research, vendor pairs, format-marker creation, kind-prefixed
  legacy slugs, unknown marker values, metadata removal, relationships, every
  ambiguity, collision, interruption point, rollback failure, and post-commit
  witness recovery;
- root-init, migration, finish, and ordinary mutator contention on the same
  working-tree lock; Tree access guard release before launch; close-on-exec
  descriptors; and successful session-side mutation without deadlock;
- immediate refusal of two bare drivers in one working tree before either can
  duplicate a launch; canonical-path aliases; independence of same-named
  worktrees; simultaneous drivers in default and secondary native-jj
  workspaces, default and secondary colocated-jj workspaces, and main and linked
  Git worktrees; alias contention inside every shape; identical lease resolution
  under differing `TMPDIR` and conflicting `GIT_DIR` / `GIT_WORK_TREE`; release
  on normal exit and forced process death; and driver/session descriptors absent
  after configured-command exec;
- lease and epoch open/lock/stat replacement races retrying and then failing
  visibly through deterministic post-open/post-lock barriers; no claim that
  deliberate unlink/recreate outside acquisition is survived; and no production
  dependency on temporary-directory cleanup behavior;
- black-box outcomes for the three epoch transitions and shared agent admission:
  one contention diagnostic naming mode and operation, no self-deadlock,
  no tree/launch overlap, and a bounded stop; the internal backend's event trace
  separately proves three writes, guard release before reacquisition, and the
  30-second timeout through an injected monotonic clock;
- a tree command admitted immediately before driver death completing before a
  replacement driver proceeds; calls begun after exclusive invalidation failing;
  the shared-guard/lease-transfer/read/probe interleaving admitting the old call
  but blocking replacement invalidation, followed by refusal of calls that
  begin after invalidation;
  an orphaned tree command outliving its SIGKILLed foreground parent causing a
  bounded stop rather than parking or relaunching the loop; and `grove-llm
  --version` succeeding against inactive or foreign epochs while task-tree verbs
  still refuse;
- 128-bit OS-random driver nonces generated once per process; an independent
  per-launch signal draw; occupied draws retried; no deterministic reuse; the
  accepted cross-restart collision bound recorded rather than asserted
  impossible; abandoned-signal cleanup only after exclusive crash handoff; an
  old completion signal having no effect on the new launch; and manual commands
  without loop context retaining their current behavior;
- finish deletion followed by root initialization in the same working-tree
  path, including reuse of `plan-k1`, where the old session cannot resolve,
  mutate, or complete through `grove-llm` and the newly launched session can;
- exact Git pathspec behavior for tracked deletion and unborn migration commits,
  staged-change preservation, malformed unborn-finish refusal, and jj working-
  copy preservation for migration and finish commits;
- direct foreground ownership, no hidden argv/env injection, signal/no-signal
  outcomes with exit status and elapsed time, nonzero configured-command
  diagnostics, and child termination on driver shutdown.

The `grove-llm` tree interface is the second seam. Exercise current filename
parsing, the kind-label non-prefix invariant, malformed task-shaped names,
stable resolution, pair generation without harness flags, per-verb finish
refusal, finish-skipping pick order, mandate-authorized promotion after a
launch-window insert, and migration refusal while a witness exists.

Internal unit tests may cover pure KDL/template and migration-plan functions and
the process-ownership backend's event trace. Acceptance remains stated in
observable process, tree, VCS, diagnostic, timing-bound, and argv outcomes; only
the internal race tests inspect protocol events and barriers.

## Out of scope

- Choosing commands, models, reasoning effort, approval mode, or sandbox policy
  for the user, or creating/editing their personal config.
- Inferring the harness behind a configured executable or wrapper.
- Profiles, includes, defaults, family inheritance, repository-local config, or
  environment overrides.
- A shell command language inside configuration.
- Enforcing cross-harness or cross-model review diversity.
- Treating driver/session leases as authentication against a caller that
  deliberately strips or forges its ambient loop-control context.
- Preventing a stale process from editing or committing files directly outside
  `grove-llm`, or surviving deliberate deletion/replacement of Grove controls in
  the VCS administration area; those are outside cooperative workflow ownership.
- Literal cross-process nonce or signal-path non-reuse; the design accepts the
  independently drawn 128-bit collision bound instead of durable tombstones.
- Changing non-finish depth-first order, review relationships, pruning
  authority, or completion-signal behavior.
- Power-loss durability or branch/bookmark/worktree integration.
