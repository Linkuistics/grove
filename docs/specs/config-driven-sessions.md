# config-driven-sessions

## Problem

Before this design, Grove reconstructed one session command from several
independent sources: a repository marker and local stamp selected a primary
harness; task bodies and environment variables could reroute a leaf; more
variables selected a model; and harness-specific code appended naming and
sandbox arguments. The driver therefore knew too much about the programs it
launched while a reader could not see the complete launch policy in one place.

Lifecycle had the same split ownership. Human-facing subcommands, a routing
peek, and the launched agent each performed part of initialization, migration,
selection, retirement, or finishing. That made restart behavior harder to
state and permitted the leaf used to route a session to differ from the leaf
the session later adopted.

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

Immediately before spawn, Grove removes any inherited `GROVE_SIGNAL_FILE`, the
retired `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` handles, and the shipped
internal failure-injection seams, then grants only the fresh signal path for
this launch. Membership in that scrub list is "a descendant could act on this
value", which is why removed session-target metadata is absent from it: nothing
reads it, so leaking it grants nothing.
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
repository or basename. That recorded root device is also the **Workspace layout
preflight**'s operand, so acquisition both creates the control directory and
proves it usable before anything else runs.

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
in-tree witness and recovery protocol; migration, promotion, and finish each
carry one.
Single-path renames rely on filesystem atomicity, and no operation gains a
power-loss guarantee merely by sharing this lock.

### Workspace layout preflight

Teardown ends in one atomic same-filesystem rename of the whole `.grove/` root
into the workspace-control directory, with no copy or working-tree-sibling
fallback. A workspace that cannot supply that rename target is unfinishable for
its whole life, so the capability is validated as a **layout precondition**
rather than discovered at the finish gate, following the [supported workspace
layouts](../adr/supported-workspace-layouts.md) decision.

Immediately after creating the control directory and pinning the working-tree
root, and before configuration validation or any `.grove/` observation, the
driver compares the filesystem device of that created control directory with the
recorded root device. Equal devices pass. The two properties the quarantine needs
are established differently: **untracked** follows structurally from the resolver
placing controls only inside `.jj/` or the canonical per-worktree Git directory,
never a working-tree sibling, and is not measured; **same device** is contingent
on the operator's layout and is what this comparison proves.

Layout support follows from whether resolution stays inside the working tree.
`<workspace>/.jj/grove/` and a `.git/` **directory** keep it there, in every
native, secondary, and colocated jj shape and in a plain checkout; a `.git`
**file** — a linked worktree or a submodule — sends it to the main repository or
superproject, and is the only family whose devices can differ. Grove nevertheless
measures every layout rather than trusting that classification, because a
symlinked `.git` or `.jj` marker, or a control directory that is its own mount
point, leaves the working tree without changing the marker's kind.

A failure is a resumable no-mutation stop with the same standing as a
`grove-llm` version skew: no `.grove/` is created, an existing tree stays
byte-identical, no Grove-authored revision exists, and rerunning bare `grove`
after repairing the layout continues normally. Its diagnostic is distinct from
the unwritable-control-directory failure and names the working-tree root and its
device, the resolved control directory and its device, the marker that produced
the resolution — including a `.git` file's gitdir target, since that is what left
the working tree — and the two remedies: place the linked worktree on the main
repository's filesystem, or use a workspace whose administration directory is
in-root.

Acquisition is the sufficient gate as well as the earliest one, because root
initialization, migration, selection, finish allocation, and a later driver's
transaction recovery all run behind the lease. No second lifecycle command,
durable capability marker, or user-facing flag is introduced, and ambient
`grove-llm` tree verbs gain no layout check — they allocate no quarantine, and
`finish-commit`, which does, preflights independently.

That independence is required, not redundant. This check compares **proxies**:
the rename moves `.grove/` into the control directory's `grove/` child, and at
acquisition neither operand need exist, so a `.grove/` that is itself a mount
point passes here and is correctly refused at finish. Layout is also **mutable**
while the lease is held — `git worktree repair`, a rewritten gitfile, a relocated
main repository, or a changed bind mount all alter the answer, and the lease pins
the root's identity rather than the destination's device. And `finish-commit` is
**separately invocable**, including by an operator retrying a blocked
transaction, so it can attest nothing about which driver validated what.
Consuming this startup fact at the teardown gate would be exactly the stale
disposition the transaction's revalidate-at-every-gate rule rejects. The
preflight is therefore an early warning that weakens no finish-time check.

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

`.grove/` task-root absence is the complete fresh-tree discriminator. The driver
does not consult VCS history, an abandoned signal channel, or unlocked
lease/epoch bytes to infer that a missing task root used to be a completed grove.
A teardown commit can prove that Grove deleted an earlier tree, but it cannot
prove whether the present bare invocation means "recover that finish" or "start
another grove"; distinguishing those intents would require another command,
prompt, or durable marker. Thus a bare invocation after a successfully committed
teardown is a legitimate new grove and may allocate `plan-k1` again. The new
driver nonce and session epoch, not globally unique task keys, reject an old
cooperating session's `grove-llm` operations against the new task tree.

### Existing live tree

A current-format tree with a live leaf needs no lifecycle mutation. The driver
picks the first live leaf in depth-first pre-order, reloads config, expands that
kind's target, and launches. Completion signaling is unchanged: `relaunch`
continues with a fresh iteration, `done` stops cleanly, and an absent signal
stops while preserving and reporting the configured child's status and elapsed
time. There is no finish-only inference on the no-signal path: even if the child
successfully committed `.grove/` deletion before it exited, the driver does not
replace the missing disposition with `done`.

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
working-tree lock, recovers or rejects any pending transaction, re-resolves the
same live finish handle, and revalidates that no non-finish leaf is live before
starting teardown. If work appeared after launch, it names that work and leaves
the tree byte-identical; the session exits without a completion signal so the
next driver iteration selects the new work. On success the helper has committed
deletion of `.grove/` under a message naming `finish-k<key>` plus the active
finish-attempt identity and has removed the task root from the working tree.

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

#### Pre-commit transaction and recovery

Finish teardown is one fail-closed transaction, following the [task-tree
transactions fail closed](../adr/task-tree-transactions-fail-closed.md)
decision. The transaction keeps `.grove/` present until the repository has
proven the exact scoped deletion commit and the atomic quarantine handoff has
completed. Disposal after that handoff is cleanup-only and may finish later.
At no pre-commit interruption point may the working tree expose task-root
absence, because the next bare invocation would correctly classify that shape
as a fresh grove.

After the ordinary live-finish and no-other-work checks, repository and
quarantine validation that requires no tree mutation runs first. The expected
tracked deletion fingerprint must be non-empty, the reserved witness prefix
must be absent from the starting repository state, and the workspace-control
directory must provide an untracked same-device rename target. This repeats no
earlier result: the **Workspace layout preflight** compares the working-tree root
against the control directory before `.grove/` need exist, while this comparison
is against the exact rename operands, and the layout may have changed since.
The helper opens
`.grove/` itself as a no-follow directory, compares that descriptor's identity
with the `.grove` entry in the locked working-tree root, and retains it for
descriptor-relative transaction operations; a symlink or any non-directory task
root is refused before mutation. A wholly untracked task tree, including one ignored before it
was ever recorded, has no deletion a focused finish commit can record, so finish
refuses with the live tree unchanged and tells the operator to record it before
retrying. A validation failure leaves the task tree and Git index byte-identical
and creates no Grove-authored repository revision; jj's ordinary read-side
operation/snapshot bookkeeping is not promoted to a stronger byte-identity
claim. The helper then creates two reserved directories inside the task root, in
this order:

```text
.grove/PREPARING-FINISH-<finish-handle>-<attempt-identity>/
.grove/FINISHING-<finish-handle>/
```

The first exists only while the witness is being built. It is created **before**
the repository adapters prepare anything, so every auxiliary they may write —
notably a colocated Git-index backup — is already owned on disk by a named handle
and attempt rather than by an anonymous in-flight process. Both prefixes are
reserved, and every ordinary reader and mutator refuses either. Recovery
discards a preparing witness by aborting that repository preparation, and fails
closed on any content it cannot classify as its own; because publication
precedes evacuation, a preparing witness never holds an evacuated entry.

Inside it the helper writes a manifest naming the handle and the active session
epoch's opaque
128-bit launch nonce as the finish-attempt identity, plus a repository-start
anchor, the exact tracked deletion fingerprint expected from that anchor, and
every ordinary root entry's type, recursive digest, and recovery location. The
digest is a canonical no-follow SHA-256 Merkle digest over length-delimited
records:
directories hash their raw-name-byte-ordered child records; each record includes
the root-relative path bytes, entry type, and applicable mode bits; regular files
add their bytes and symlinks add the link-target bytes. Any other filesystem
entry type is rejected before mutation with its path. The digest detects missing,
substituted, or altered evacuated content without dereferencing a symlink or
leaving directory meaning implementation-defined. The start anchor is Git's
`HEAD`; for jj it is the exact preflight working-copy commit ID, its change
identity, and its parent commit identities. The expected deletion
fingerprint is independent of the generated finish leaf: that leaf is normally
working-tree-only and need never have existed in the starting VCS revision.
Preparation rejects a reserved-name collision or second witness before mutation
and writes a ready marker last, then publishes the witness by renaming the
preparing directory to `FINISHING-<finish-handle>/` in one atomic step; an
incomplete witness contains no moved source and is discardable. Once ready, the
helper revalidates that the root holds exactly the manifest's entries plus the
witness, and that each still matches its recorded digest, before it evacuates
every other `.grove/` entry beneath the witness by same-filesystem rename
without following symlinks.
The root now contains only the witness. Every ordinary tree reader and mutator
refuses this shape, and the witness retains the only copies of the finish leaf,
brief, format marker, terminal tree, and foreign root entries. Recovery is
selected before format parsing, liveness, migration, or root initialization.

Evacuation and rollback each move one entry at a time, so an interruption leaves
an arbitrary prefix moved. Recovery therefore locates every manifest entry rather
than matching the root against the two extreme shapes: each entry must sit in
exactly one of the task root and the recovery tree, resident entries must still
match their recorded digest, and anything else — an entry in both places, an
entry in neither, an unrecorded recovery-tree entry, or a foreign root entry —
is fail-closed and named. A moved prefix is an ordinary recoverable state, not a
malformed tree, and the union of the two places is always the recorded tree.

The tree transaction calls one deep repository seam that hides the Git, native
jj, and colocated-jj adapters. Its result has three factual dispositions:

- **Committed** — the exact immediate Git result derived from the recorded
  `HEAD`, or the exact jj partial-commit result derived from the recorded
  working-copy change and parents, is named by the requested finish handle and
  carries the manifest's finish-attempt identity, has the expected tracked
  deletion fingerprint, touches no path outside `.grove/`, and leaves no tracked
  `.grove/`; any required success-index activation is complete. The witness, not
  the commit parent, proves the live finish leaf that authorized teardown.
- **Not committed** — the exact commit is absent, Git `HEAD` still equals the
  recorded start or jj's current working-copy commit itself still has the
  recorded change identity at the recorded parents, and every repository-side
  staging/index mutation has been restored, so restoring the task tree is safe.
- **Recovery pending** — the adapter cannot yet prove either complete success or
  a restored pre-commit repository state. It retains its auxiliary recovery
  material and the tree keeps the blocking witness.

The seam does not infer this disposition solely from command exit status. After
a failed, interrupted, or lost commit result it checks the manifest anchor,
message, expected tree delta, and exact immediate Git `HEAD` or jj committed
parent that the requested transaction could have produced. Finding that commit
crosses the boundary permanently: recovery never restores the old task tree.
Not finding it permits rollback only after the adapter positively proves the
starting repository topology still holds and restores its original index state.
A different new revision, a rewritten message or tree, or any other topology
change is **Recovery pending**, not evidence that no commit occurred.

The disposition is not an unguarded enum that the tree caller may act on later.
The deep repository seam retains the outcome context through the filesystem
handoff and revalidates immediately before and after it. On **Not committed**,
the transaction restores manifest entries while retaining the witness, snapshots
or refreshes repository state, and removes the witness only when Git still
equals the recorded `HEAD` or jj reproduces the exact recorded preflight commit
ID. If the post-restore check changes, the root remains blocked by the witness
and the observed state becomes **Recovery pending**. On **Committed**, it
revalidates the attempt-bound exact result, renames the whole root to quarantine,
and revalidates again before disposal; a changed result atomically renames the
quarantine back to `.grove/` and remains blocked, reporting both the changed
result and the quarantine when that return itself cannot complete. Direct
repository or filesystem mutation after the final successful gate is outside
cooperating Grove guarantees, but any change observed at either gate is never
acted on through a stale disposition.

`Recovery pending` is fail-closed, not automatic history repair. Its diagnostic
names the artifact that holds the blocked transaction, the recorded exact
anchor, the observed topology, and the two
admissible ways to make retry decidable: preserve divergent work elsewhere and
restore the exact recorded start so the next recovery can roll the tree back,
or make the exact handle-and-attempt-named teardown result the immediate result
so recovery can finish forward. That artifact is normally the in-tree witness;
after a handoff whose restoration could not return the tree it is the
quarantine, because naming an absent task-root path would send the operator to
bytes that are not there. The operator then reruns bare `grove` or the
still-confirmed session's same `finish-commit`; Grove never resets, rebases, or edits the witness on the
operator's behalf. Direct VCS commits that track the witness or change topology
while the transaction is blocked enter this same procedure. This is the
documented recovery exit for non-cooperating repository mutations, not a fourth
repository disposition or durable state outside `.grove/`.

The VCS adapters implement the same disposition as follows:

- **Plain Git** first preserves the complete existing index. After evacuation,
  `git add -A` and `git commit --only` select `.grove/` while excluding the exact
  `FINISHING-<finish-handle>/` witness. Its message names the stable handle and
  opaque finish-attempt identity. The internal commit uses the same empty
  workspace-control hooks path migration uses, disabling all user Git hooks: arbitrary hook
  side effects cannot be rolled back from an index image and therefore cannot
  coexist with the promise to preserve unrelated working-tree bytes. Signing
  and repository failures remain visible. A staging or commit failure restores
  the index and proves `HEAD` is still the manifest anchor before returning
  **Not committed**. Failure of either proof returns **Recovery pending**.
  Unrelated staged and working-tree entries never enter the deletion commit.
  The adapter requires the witness to remain absent from the index and every
  candidate committed tree; if a direct broad commit tracks it, recovery is
  pending until the operator restores one of the two provable topologies.
- **Native jj** records exact preflight working-copy commit ID `C0`, change
  identity `W`, and parent set `P`, then commits the `.grove/` deletion fileset
  while excluding the exact witness. A partial `jj commit` keeps the selected
  deletion in the commit with change identity `W` and parents `P`, and moves
  unrelated changes plus the witness into a new successor working-copy commit.
  **Committed** requires that attempt-bound exact handle-named parent of the
  current successor. **Not committed** requires that exact result to be absent
  from the current repository view and the current working-copy commit itself to
  have identity `W` at `P`; after tree restoration its snapshot must reproduce
  `C0` before witness removal. Merely finding `W` somewhere in history is never
  sufficient for either rollback or success, so the predicates remain disjoint
  after `jj edit`, rewrite, or workspace-topology changes.
- **Colocated jj** copies the user's complete Git index before any jj command
  can preflight-snapshot or export the working copy. It then snapshots, records
  `C0`, `W`, and `P`, and prepares the `.grove/`-free success index before
  invoking the same partial-commit protocol as native jj. Preparation or an
  uncommitted command failure restores the pre-snapshot index before it may
  return **Not committed**. After the exact committed parent exists, recovery activates the
  success index; an activation failure is **Recovery pending**, never grounds to
  roll history or the tree back. The staged blob for every unrelated path
  remains exactly the user's pre-finish blob even when its working-copy content
  differs.

On **Not committed**, the tree transaction restores every entry the recovery tree
holds to its original path, verifies the complete current-format live finish
tree, and only then removes the witness. Restoration is therefore the same
operation over whatever remains, so retrying an interrupted rollback finishes it.
A reported failure leaves the same finish leaf selectable for retry. If tree
rollback fails — an occupied destination, a restored entry that does not match
its digest, or a repository change observed before or after restoration — it
returns an actionable diagnostic naming the exact witness and keeps the tree
blocked with the unrestored copies beneath it.

On **Committed**, the transaction first verifies repository and colocated-index
cleanup, then atomically renames the whole `.grove/` root — witness, manifest,
and evacuated tree intact — to a collision-resistant quarantine in the
workspace's VCS-administration control directory. Preflight proves that cleanup
directory and the worktree are on the same filesystem before any tree mutation
or commit; if the workspace layout cannot supply an untracked same-device
quarantine, finish refuses with the live tree unchanged rather than falling back
to a trackable worktree sibling or non-atomic copy. Rename failure leaves the
original blocking witness and is idempotently recoverable from exact commit
proof. Rename success is the one namespace transition to task-root absence; only
after it may descriptor-rooted, no-follow recursive disposal begin. Disposal
unlinks a symlink as an entry and never traverses its target. Interruption or
disposal failure therefore leaves a complete post-commit quarantine rather than
an empty or partially deleted `.grove/`. The quarantine is cleanup garbage,
never a finish receipt or lifecycle input. Its exact path is reported and best-effort disposal
may be retried, but a proven commit plus absent task root remains successful
teardown.

A later bare driver validates configuration before touching this state. Under
the universal lock it rolls an uncommitted witness back, exposing the finish
leaf to a fresh HITL session and therefore a fresh confirmation. If the exact
commit is already proven, it completes forward cleanup; the resulting absent
root then follows the ordinary **Fresh tree** contract in that invocation, as
specified by the post-commit restart semantics. A still-running, already
confirmed finish session may instead retry `finish-commit`; exact proof returns
success without a second commit, allowing it to invoke `complete --done` last.
If neither repository predicate holds, the driver stops on **Recovery pending**
with the operator procedure above; it does not launch, select, or silently park
on an uncheckable witness.

Auxiliary Git-index backups or success images live in the workspace's VCS
administration directory because they must not enter jj's working-copy commit.
They are keyed to and valid only while the in-tree finish witness exists. Their
unlocked bytes never classify a rootless invocation.

Changing an auxiliary's artifact identity is itself a small transaction: a state
document records both inodes, the exchange is atomic, and recovery re-derives its
phase from the recorded identities. Each side of the exchange is staged under a
freshly drawn name inside that auxiliary's own reserved role-and-attempt
namespace, and every staged entry carries that namespace in its name, so an
interruption before the state document is durable leaves an entry attributable to
the attempt that created it rather than an anonymous temporary. Nothing on an
ordinary path removes an entry Grove did not create. The namespace also bounds,
rather than proves, ownership: a writer able to rewrite the state document in
place can make it agree with itself, so recovery validates that both staged names
lie inside the namespace and refuses any other. That writer already owns the
administration directory outright, so the residual redirection confers nothing.
While a replacement state document is present the auxiliary is mid-transition, so
disposing or activating through a pre-replacement snapshot fails closed with the
state document named; only recovery, which settles the replacement first, retires
the marker. A post-commit cleanup
quarantine likewise carries no workflow meaning after the atomic task-root
rename. `finish-commit` owns immediate best-effort no-follow disposal; a later
driver, after it owns the lease and has invalidated the previous epoch, reaps
quarantines and auxiliaries carrying Grove's valid cleanup-manifest marker and
having no matching in-tree witness. It attributes an auxiliary by reading that
marker alone, before recovering it: recovery settles any replacement in flight,
which is a mutation, and an owned auxiliary is settled by its witness's own
lifecycle recovery instead. While a witness is live, a marker the sweep cannot
attribute is therefore neither reaped nor reported as an orphan — the refusal
that names it comes from that recovery, so no diagnostic claims untouched state
on a path where the sweep had already mutated. Reaping never changes lifecycle
classification or turns cleanup bytes into a receipt; a persistent filesystem
error is reported and retried on the next owned invocation.

The sweep is over cleanup manifests, never over the reserved namespaces. A
staged entry a death stranded before its state document names it is
*attributable* — its name says which role and attempt drew it — but attribution
is not proof of authorship, and the two are not the same claim. Grove's own
substitution refusal is what makes the difference reachable: when
`replace_artifact_from`'s post-copy identity check finds an inode it can no
longer identify, it declines to unlink it and reports, deliberately leaving a
*foreign* regular file at a shape-valid staged name. That is byte-for-byte the
same shape as an abandoned staged copy, so a namespace sweep would delete
exactly the bytes the refusal preserved. Grove therefore leaves both, along with
the colocated index filter's own private staging directory, and accepts a bounded
leak — an index-sized copy per death inside one of those windows — rather than a
removal it cannot prove. What it does instead is narrow the windows: the index
filter's staging directory is released explicitly once the replacement holds its
own staged copy and before the first publication boundary, because an owner that
only runs while the process unwinds is no owner at a boundary whose whole purpose
is to end the process.

This transaction promises process-interruption consistency, not ordered
power-loss durability. It issues no `fsync` protocol.

#### Crash and retry semantics

This post-commit contract begins only after the transaction proves the deletion
commit. Validation, evacuation, index preparation, staging, an uncommitted
commit failure, and their rollback are owned by the pre-commit transaction
above. In particular, `.grove/` absence alone does not prove that its deletion
was committed.

- When `finish-commit` returns success, the confirmed session invokes
  `complete --done` last.
- If the calling finish session loses the helper result, a retry does not trust
  `.grove/` task-root absence. Through the repository seam it requires the
  immediate VCS result to prove the exact handle-named, attempt-bound,
  `.grove/`-scoped teardown commit. `finish-commit` requires the still-active
  session epoch and reads the same opaque attempt identity from that launch
  nonce. Because successful cleanup may already have disposed the manifest,
  this retry proof is self-contained: the commit's own
  parent/result delta only deletes `.grove/`, its message names the requested
  handle and this launch's attempt identity exactly, and the result leaves no
  tracked task root. It never requires the generated finish leaf in the parent.
  Git checks `HEAD`, while native and colocated jj check the committed parent of
  the successor working-copy commit. A match returns idempotent success, after
  which the already-confirmed session invokes `complete --done`.
  No match is a refusal and never signals `done`. This is narrow command-outcome
  verification under the current finish invocation, not a rootless-driver
  lifecycle discriminator. The rootless proof is reached only when no task root
  exists and only for the launch attempt that authored the commit. A new grove's
  epoch necessarily has a different attempt identity, so even an external
  reset and root removal cannot make an older reused-handle teardown satisfy the
  new confirmed session.
- If the configured session exits without a completion signal after the commit,
  the live driver follows the ordinary no-signal path: report status and elapsed
  time, then stop. It does not infer a human decision from task-root absence.
- If the driver dies before observing `complete --done`, a replacement driver
  must first invalidate the old epoch. An operation admitted under that epoch
  may delay handoff; an orphan that holds the shared guard to the 30-second bound
  makes this replacement stop `blocked` without creating a task tree. After the
  guard releases, a later bare invocation invalidates the old epoch and signal
  channel, follows **Fresh tree**, and starts a new grove; it does not recover or
  replay the previous finish.
- Once the driver observes `done`, it stops cleanly and no restart is part of the
  completed lifecycle.

This leaves no dedicated durable finish receipt outside `.grove/`. The existing
teardown commit is audit history rather than driver workflow state; only a retry
of the handle-named teardown command may verify its own immediately preceding
result there. Stable handles are identities only within one task tree. Reuse
after reinitialization is therefore intentional and safe for cooperating
processes through epoch rotation.

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
  Finish uses the same only/path form with the exact
  `.grove/FINISHING-<finish-handle>` witness excluded after the ordinary root
  entries have been evacuated beneath it. Git records deletions at their
  original paths while the witness remains untracked and recoverable. The
  complete prior index is restored on an uncommitted failure and discarded only
  after exact commit proof. Both commits disable Git hooks through one shared
  empty internal hooks path, so preservation of unrelated working-tree bytes
  does not depend on arbitrary user hook behavior. Pre-existing staged entries
  outside `.grove/` remain staged and absent from either commit. A valid Grove
  cannot reach finish in an unborn repository without first recording either
  its migration or a task commit; an externally hand-constructed terminal tree
  that has never existed in `HEAD` is refused before evacuation because there is
  no non-empty deletion for a focused finish commit to record. The witness must
  remain untracked; a direct broad commit that tracks it is an operator-recovery
  case, not a candidate teardown result. The handle-plus-attempt commit message
  makes rootless proof specific to the active finish launch without becoming a
  later driver's lifecycle receipt.
- In Jujutsu, Grove commits a `.grove/` fileset excluding the exact live
  migration or finish transaction witness. Unrelated working-copy changes and
  the witness additions remain in the new successor working-copy commit until
  recovery removes the witness; the selected deletion stays in the prior change
  identity at its recorded parents and carries the handle-plus-attempt message.
  A colocated workspace backs up the user's Git index before jj's preflight
  snapshot, prepares its `.grove/`-free Git success index after that snapshot,
  and activates the success image only after the exact jj result is proven;
  native jj has no Git-index step.

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
driver-only finish creation, current pick, universal working-tree lock, and the
migration and finish transactions. The finish transaction's small interface is
the requested stable handle; it hides witness preparation, evacuation, rollback,
forward cleanup, and recovery ordering from the CLI and driver. The repository
module owns worktree/main-repo resolution and path/fileset-scoped commits. Its
internal finish seam consumes the transaction manifest and reports
**Committed**, **Not committed**, or **Recovery pending** after comparing the
manifest anchor and requested handle with Git's current `HEAD` topology or jj's
current working-copy/committed-parent topology. Git, native-jj,
colocated-index, and lost-result behavior stay behind that seam. The seam keeps
its outcome context through pre/post handoff revalidation instead of returning a
staleable bare enum, so tree lifecycle callers never reproduce VCS-specific
commit-boundary rules.

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
  when work appears after launch, handle-plus-attempt deletion commit naming, Jujutsu/Git
  intermediate finish snapshots, clean `--done` stop, and successful teardown
  followed by a configured child exiting without a signal: the driver preserves
  the real status/elapsed-time diagnostic and a later bare invocation launches a
  fresh requirements `plan-k1`;
- finish-transaction preparation and every evacuation/rollback/forward-cleanup
  transition boundary: before readiness the original tree is untouched; after
  readiness every reader and unrelated mutator refuses the exact witness; an
  uncommitted interruption restores a byte-identical live finish tree; rollback
  failure leaves the only copies under an actionable blocking witness;
  repository topology changes before or after rollback/forward handoff retain or
  atomically restore the witness instead of consuming a stale disposition;
  forward rename failure does the same, while interruption after the atomic root
  rename leaves a complete cleanup quarantine and no partial task root; exact
  commit proof always recovers forward and never resurrects the tree;
- reserved-name collision, foreign files, symlink entries without target
  traversal during evacuation and quarantine disposal, canonical recursive
  directory digests, special-file refusal, manifest/content tampering, and
  refusal of duplicate witnesses in either publication state before any source
  move; plus a symlinked `.grove/` refused as a non-directory before the
  transaction opens anything, and a replaced root refused by no-follow
  descriptor identity before mutation;
- workspace-control quarantine preflight, including same-device success and a
  cross-device refusal that leaves the live tree and repository untouched;
- the workspace layout preflight at lease acquisition: a cross-device linked Git
  worktree refused before configuration validation and before tree access, in
  both a rootless tree — where no `.grove/` is created — and one with an existing
  tree left byte-identical, with no Grove-authored revision and a diagnostic
  naming both paths, both devices, and the gitfile target; a symlinked `.git` or
  `.jj` marker onto another filesystem refused on the same path; same-device
  linked Git worktrees, plain checkouts, and native and colocated jj default and
  secondary workspaces all admitted and driving normally; the refusal
  distinguishable from the unwritable-control-directory refusal and resumable
  once the layout is repaired; and ambient `grove-llm` tree verbs unaffected;
- the two preflights independent rather than one deferring to the other: a
  workspace that passes at acquisition and becomes cross-device before teardown
  is refused by `finish-commit` with the live tree unchanged and no Grove-authored
  revision; a `.grove/` that is its own mount point passes acquisition and is
  refused at finish; and an operator-invoked `finish-commit` retry performs its
  own comparison with no durable capability marker anywhere on disk;
- plain-Git validation, index-backup, staging, hook suppression,
  injected/signing commit, index-restore, unexpected-`HEAD`, and lost-result
  failures, including a wholly untracked tree (ignored before first record) and
  a direct broad commit that tracks the witness; native-jj partial-commit change/parent identity,
  exact preflight commit reproduction, `jj edit`/rewrite/workspace-topology
  changes, exact-result absence, disjoint committed/not-committed predicates,
  and lost-result failures; and colocated-jj pre-snapshot index backup ordering,
  success-index preparation, commit, success-index activation, and restore
  failures. Each uncommitted reported failure preserves unrelated work and
  positively proves the recorded repository anchor, each ambiguous result is
  classified by the exact handle-and-attempt-named scoped commit, and each unexpected
  repository state retains both its auxiliary material and the in-tree witness;
- bare-driver recovery of a pre-commit finish witness after full configuration
  validation, producing a selectable finish leaf and a fresh confirmation; plus
  committed-witness recovery completing teardown before the ordinary
  rootless/fresh transition, with no witness or index image becoming a finish
  receipt on its own; and `Recovery pending` diagnostics naming the recorded and
  observed topology plus both operator-restorable proof paths;
- lost `finish-commit` results in plain Git, native jj, and colocated jj: an
  exact immediate handle-and-attempt-named, `.grove/`-only deletion commit makes
  the same active launch's retry idempotently successful, while task-root
  absence without that proof never emits `done`; include a prior completed grove
  with reused handles followed by external root removal/reset so the older
  attempt cannot satisfy the new epoch;
- quarantine disposal that unlinks rather than follows symlinks, immediate
  best-effort cleanup, and later lease-owned reaping of orphaned internal
  auxiliaries/quarantines without using them for lifecycle classification;
- auxiliary artifact-identity replacement: interruption on either side of the
  state document, a clean same-attempt retry, both staged names refused outside
  the auxiliary's reserved role-and-attempt namespace, substituted or symlinked
  staged entries left byte-identical, and a synchronous mid-transition failure
  whose caller-held snapshot settles from disk before disposing — freeing every
  attempt-scoped name — while a substituted replacement still fails closed;
- every marker-rebind boundary reached from a real colocated-Jujutsu finish
  process through a test-prefixed seam that is scrubbed from launched sessions:
  each process death restarts to the exact original Git index, the unchanged
  working-copy commit, a live finish leaf, and no interpretable auxiliary state
  — only a death between the staging copy and the state document that would name
  it leaves an entry, unclaimed at a drawn name and deliberately not unlinked;
  each synchronous failure preserves the exact index bytes and retries cleanly
  inside the same driver launch's attempt;
- substitution at every entry a colocated-Jujutsu rebind owns — both
  auxiliaries' canonical pairs, the replacement state document, and either
  drawn staging entry — by a foreign regular file and by a symlink into the
  neighbouring `.git/` files: each restart refuses naming the witness and the
  auxiliary, leaves the substituted inode and every other entry exactly as
  found, and reaps nothing the live witness owns; while foreign entries at the
  derivable `.filtered` name and inside the reserved staging namespace survive a
  recovery that completes;
- a completion signal written after the successful deletion commit that no
  driver ever interprets, the child ordering its parent's death and reaping
  ahead of the write so the abandonment is deterministic rather than raced:
  replacement cleanup treats the abandoned channel as coordination rather than a
  finish receipt, then follows the bounded epoch handoff and fresh-root rules;
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
- the same orphaned-guard handoff after a successful finish deletion: the first
  replacement stops `blocked` without recreating `.grove/`, then an invocation
  after guard release invalidates the old epoch and launches the fresh tree;
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
- non-invocation of a mutating, rejecting user Git hook by both internal plain-
  Git commits, on success and on injected failure, with unrelated staged and
  working-tree bytes unchanged either way, and with signing failure still
  reaching the transaction;
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
