# Standalone invocations

`grove run` runs one task kind without creating a grove, project, jj
workspace or driver lease. It gives the invocation none of them. A task inside a
running grove can call it. The owner's Ravel policy selects the command for the
kind, as it does for a lifecycle session, and Grove reads no configuration of
its own. It does not integrate AgentAnyware.

## The kind

An explicit kind is the first positional argument. Grove sends it to the policy
unchanged, inside a grove and outside one. One positional argument is always
the kind. A run without a kind therefore takes its prompt from `--prompt-file`.

A run without a kind takes its kind from the directory that it runs in. A grove
is active when the closest ancestor of that directory that holds `.jj/` also
holds `.grove/`. The lifecycle `grove` drives that tree from the same
directory. The walk reads the filesystem only. It runs no jj command and does
not follow the repository pointer of a secondary workspace. The human chose
this test in `standalone-default-kind-k131`.

- **Outside an active grove**, the kind is `standalone`. This includes a
  directory with no `.jj/` above it, and a workspace with no `.grove/`.
- **Inside an active grove**, the kind is that of the next live leaf, in the
  pre-order walk of `grove-llm pick`. The run gains no mandate and no tree
  authority. The human chose this outcome in `standalone-default-kind-k131`.
- **An active grove with no live leaf**, or a tree that the reader refuses,
  gives no kind. Grove refuses before selection. The refusal names the grove
  root and tells the caller to give an explicit kind.

The kind `standalone` requires no skill. It exists so that the owner's policy
can select a harness, a model and an effort for standalone work. The owner's
policy must route `standalone` before a run without a kind can launch outside a
grove. An unrouted kind is Ravel's refusal, and nothing launches.

The prompt is the same for every kind. It holds the caller's task, the staged
files and the two ending commands. It holds no load instruction and no
missing-guidance rule.

## Selection and launch

Before staging, Grove resolves independent Ravel once from absolute PATH
entries and checks its version. The public `start-session` selects the command
and supervises it. The unchanged caller prompt supplies the exact canonical
`end-session grove-continue` and `end-session grove-force-stop` commands and
explains their meanings.

Grove sends the kind, the prompt and the exit directory. It sends no
`--confine`, no `--runtime-read`, no task identity and no settings override.
Ravel 1.1 keeps `--confine`, and Ravel 2.0 refuses it. Grove's request works on
both majors, because it holds neither flag. The request is the same for both.

Ravel's
[public execution contract](https://github.com/Linkuistics/Ravel/blob/v1.1.0/docs/specs/session-execution.md)
owns cleanup. Ravel returns only an opaque stdout result after the reap. It
supplies no private ending report, persistent store or run identity.

`grove run` still accepts `--runtime-read FILE`, repeatable, and gives it no
effect. Grove prints one notice on stderr that names the flag, and it sends
nothing to Ravel for it. Grove does not check the file. The harness reads each
file that the account of the owner and the harness sandbox permit. The flag
stays so that existing commands keep working, and its help states that it has
no effect.

The version admission is that of the
[caller contract](harness-selection-and-execution.md#package-boundary): stable
1.x from 1.1.0 and stable 2.x from 2.0.0. On the native route on 1.1, Ravel
reads the owner's `~/.config/ravel/policy.ts`. On the native route on 2.0, it
reads `~/.config/ravel/config.rhai`. The standalone caller sends the same
public verbs and flags to both majors. The admission selects no route.

Selection keeps the owner's bounds and grants. A refused, cancelled or
timed-out selection launches no managed process and publishes no outputs.

## Isolation contract

Each invocation has a private temporary directory. Grove copies the inputs into
its working directory. It exports the declared output files only after a
successful completion. Grove refuses duplicate artifact names, symlinks and
existing output destinations. It checks all outputs before it publishes any.

Publication must not overwrite a destination that something created while the
task ran. Grove reports any partial publication explicitly.

Grove applies no filesystem confinement and requests none from Ravel. The
[harness sandbox](#the-harness-sandbox) is the only boundary of the agent.
Grove's prompt tells the agent to work only on the staged files. That
instruction is not a boundary. With its sandbox off, a harness can read and
write whatever the account of the owner can, the parent project included.

On the directory route, the agent reads the descriptor and the snapshots at
their absolute paths, as the
[skill-delivery design](codex-skill-provisioning.md#installation-and-bootstrap)
states. Ravel keeps each snapshot read-only by its file modes. Nothing stops
the agent from reading the source directories or the configuration of the
owner.

With injection disabled, the harness loads native guidance from its own roots
under HOME, as a lifecycle session does. The prompt names no guidance, so Grove
makes no promise that the agent reports missing guidance. If the agent runs the
force-stop command, Grove publishes nothing.

Grove passes the private `control` directory as `--exit-dir`. That directory
lies beside the working directory, inside the system temporary directory, and
Ravel allocates the exit channel beneath it. The harness sandbox must let the
ending command write there. Codex in `--sandbox workspace-write` writes in
`TMPDIR`, so its ending command reaches Ravel. The harness keeps its own
temporary files in the `TMPDIR` of the caller.

The invocation receives neither its parent's launch directory nor its terminal.
Grove removes the loop control variables from the environment of the job. It
also removes the four repository selectors `GIT_DIR`, `GIT_WORK_TREE`,
`GIT_COMMON_DIR` and `GIT_INDEX_FILE`. It removes the multiplexer variables
`TMUX`, `TMUX_PANE`, `ZELLIJ`, `ZELLIJ_SESSION_NAME` and `ZELLIJ_PANE_ID` too.
Otherwise the harness receives the environment of the caller, as a lifecycle
session does.

The removal of a multiplexer variable removes a pointer, not a capability. The
socket of a multiplexer stays reachable at its default path, unless the
harness sandbox denies it.

Its exit channel belongs to its own run and is distinct from the channel of
any enclosing session. The harness acknowledges its work with the canonical
`ravel end-session grove-continue` command in its prompt. That command ends only this invocation
and takes no session ID. Tree verbs are unavailable.

Grove publishes only after a successful Ravel exit and an exact
`grove-continue`. The `grove-force-stop` command of the prompt stops only this
invocation, reports failure and publishes no output, even if every required
output is valid. A missing or unsupported result, a supervision failure, an
abnormal supervisor exit or a cancellation prevents success. Ravel's first
publication wins. A managed process that exits nonzero after it publishes does
not override a result that Ravel reaped successfully. Grove adds no private
interpretation of the child's exit that contradicts this.

Grove runs Ravel as a detached job. Ravel leads its own POSIX session, with
standard input at end of file and no inherited descriptor above standard error.
Grove holds the public stdout separately. Managed output and diagnostics on
stderr go to the transcript of the invocation. `grove run` keeps the terminal, so
its relay writes from the foreground group and a typed Ctrl-C reaches Grove.

Grove sends a caught INT, TERM or HUP on to the `ravel` process and waits for it
to exit. Grove sends no KILL.

Ravel owns the cancellation of the harness. It sends the signal on to the process group of the harness, and it sends KILL
after its kill-grace of 5 seconds. A nested cancellation can therefore outlast
a shorter grace of an outer supervisor. Grove then publishes nothing, and the
nested Ravel finishes its cleanup after the outer supervisor returns. The
caller contract states the
[detached job](harness-selection-and-execution.md#execution-contract) and its
[signals](harness-selection-and-execution.md#interruption).

Before Grove considers the outputs, Ravel and the harness are gone. So is the
process group of the harness. Ravel kills what remains of that group when the
harness exits. It returns the public result only once the group is gone. So a
writer that the harness left in its group cannot change an output after Grove
checks it.

Processes that intentionally escape their process group need a
stronger containment in the backend. No guarantee about the process tree may
rest on a kill of the process group alone.

An uncatchable death of Grove can leave its detached Ravel and harness running.
Grove then publishes nothing. Grove sets no bound on its wait for Ravel. An
outer supervisor's grace bounds a nested run only when Ravel's cleanup finishes
within that grace. A failure of the
transcript relay also prevents publication and keeps the log for inspection,
even when the harness acknowledged successfully.

## The harness sandbox

The configuration script of the owner selects the harness sandbox of each
standalone kind. It selects the harness, the model and the effort in the same
way. The sandbox is part of the arguments of the selected command. Grove builds
no harness argument and reads no configuration of its own.

A standalone kind needs writes in the working directory and in the temporary
directory, and reads of its skill directories. Codex with `--sandbox
workspace-write` gives that. Claude Code gives it with its own sandbox
settings, its working directories and `--add-dir` for each snapshot root. A
selected command with its sandbox off gives no boundary, and Grove neither
detects nor refuses it.

Grove's own Codex helper for the `release-notes` kind runs Codex with
`--sandbox workspace-write`. It uses the Codex home of the owner, so it copies
no sign-in file.

## Visibility

Every invocation announces its kind and log location, streams a readable
transcript through its caller and reports its final status. The supervisor owns
the display connection. The child has no terminal of the parent and no
multiplexer variable. A supported existing multiplexer can provide a dedicated view. A
full interactive interface embedded in another harness needs the integration API
of that harness. It is outside the initial interface of this command.

## Release notes

Release preparation supplies the changelog of the previous release, the commit
descriptions and the source diff as explicit input artifacts. A `release-notes`
kind that the owner's policy routes produces one Markdown body. The
deterministic release task checks and inserts it, and commits the notes through
jj. It then continues its existing checks, version cut, builds, publication and
installation verification.

Existing Unreleased notes remain authoritative. No LLM invocation is necessary
when the notes are already present. Tests substitute a deterministic harness and
must never depend on paid LLM calls.

## Verification obligations

- No jj invocation, including from a directory that holds repository selector
  variables. A `.grove.kdl` or `config.kdl` there or in HOME changes nothing.
- A run without a kind routes `standalone` outside an active grove. The cases
  are a directory with no `.jj/` above it, a `.grove/` with no `.jj/` above
  it, and a workspace with no `.grove/`.
- A run without a kind, from a subdirectory of an active grove, routes the next
  live leaf's kind. An explicit kind there keeps its route.
- A run without a kind in an active grove with no live leaf refuses before
  selection, names the remedy and launches nothing.
- The policy selects the command. The harness receives its own enclosing
  context and no control variable of an enclosing session.
- Grove sends neither `--confine` nor `--runtime-read` to Ravel 1.1 or 2.0. A
  `--runtime-read` of `grove run` prints its notice and changes no request.
- The file that runs is the one that Ravel resolved, whatever else of that name
  PATH or Grove's own directory holds.
- Nested completion, force-stop and cancellation leave the run of the enclosing
  session intact.
- The harness cannot consume the caller's stdin or open its terminal. It
  receives no repository selector and no multiplexer variable.
- On the directory route of 2.0, the agent reads the injected skills outside
  cwd. A write to a snapshot file fails by its file mode.
- On 1.1 and on 2.0 with injection disabled, a scripted agent reads its native
  guidance under HOME. After its force-stop, nothing publishes and Grove
  launches the agent once. The fixture proves Grove's interpretation of a
  force-stop, not the behaviour of a real agent.
- Force-stop, a harness exit without a result, malformed or unsupported results,
  a supervision failure, cancellation, output symlinks, missing outputs and
  destination races fail visibly.
- A writer that stays in the group of the harness after a clean acknowledgement
  stops before any output publishes.
- A Ctrl-C typed on the terminal of `grove run` cancels the invocation and
  publishes nothing. The terminal stays with the group of `grove run`.
- The existing interactive Grove job control and the session-epoch checks stay
  green.
