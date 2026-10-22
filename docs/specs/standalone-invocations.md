# Standalone invocations

`grove run` runs one task kind without creating or discovering a grove,
project, jj workspace or driver lease. A task inside a running grove can call
it. The owner's Ravel policy selects the command for the kind, as it does for a
lifecycle session, and Grove reads no configuration of its own. It does not
integrate AgentAnyware.

## Selection and launch

The owner policy and settings are personal files that the confined process
cannot read. Before staging, Grove resolves independent Ravel once from absolute
PATH entries and checks its version. The public `start-session --confine`
selects outside the sandbox and supervises inside it. The unchanged caller
prompt supplies the exact canonical `end-session grove-continue` and
`end-session grove-force-stop` commands and explains their meanings.

Grove sends the kind, the prompt, the exit directory and the canonical
`--runtime-read` grants. It sends no task identity and no settings override.
Ravel's
[public execution contract](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/specs/session-execution.md)
owns confinement and cleanup. Ravel returns only an opaque stdout result after
the reap. It supplies no private ending report, persistent store or run
identity.

Selection keeps the owner's bounds and grants. A refused, cancelled or
timed-out selection launches no managed process and publishes no outputs.

## Isolation contract

Each invocation has a private temporary directory. Grove copies the inputs into
its working directory. It exports the declared output files only after a
successful completion. Grove refuses duplicate artifact names, symlinks and
existing output destinations. It checks all outputs before it publishes any.

Publication must not overwrite a destination that something created while the
task ran. Grove reports any partial publication explicitly.

Filesystem confinement is mandatory and belongs to Ravel. The installed system
runtime resources, the selected executable and Ravel's own executable are
readable. Additional runtime files, such as harness credentials, need explicit
read-only grants. Grove passes these on as `--runtime-read`. The parent project
is not a runtime resource.

Ravel confines writes to the working directory and to the private run directory.
Ravel keeps that directory for the temporary files of the harness and for its
exit channel. A failure to establish confinement prevents the launch. The
permission flags of the selected harness cannot disable this outer boundary.

The invocation receives neither its parent's launch directory nor repository
selectors, nor terminal or multiplexer capabilities. Its exit channel belongs to
its own run and is distinct from the channel of any enclosing session. The
harness acknowledges its work with the canonical
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

Grove runs Ravel noninteractively in its own POSIX session, with no inherited
input or terminal streams. It holds the public stdout separately. Managed output
and diagnostics on stderr go to the transcript of the invocation. Ravel owns the
cancellation of the harness and kills its process group at once. Grove forwards
its own cancellation to Ravel and waits for it. So a nested cancellation
finishes inside the grace of an outer supervisor.

Before Grove considers the outputs, Ravel and the harness are gone. So is the
process group of the harness. Ravel kills what remains of that group when the
harness exits. It returns the public result only once the group is gone. So a
writer that the harness left in its group cannot change an output after Grove
checks it.

Processes that intentionally escape their process group need a
stronger containment in the backend. No guarantee about the process tree may
rest on a kill of the process group alone.

An uncatchable death of Grove can leave its detached Ravel and harness running.
Grove then publishes nothing. The cancellation grace bounds ordinary nested runs
only when Ravel's cleanup finishes within that grace. A failure of the
transcript relay also prevents publication and keeps the log for inspection,
even when the harness acknowledged successfully.

## Visibility

Every invocation announces its kind and log location, streams a readable
transcript through its caller and reports its final status. The supervisor owns
the display connection. The confined child cannot control the parent terminal
or multiplexer. A supported existing multiplexer can provide a dedicated view. A
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
- The policy selects outside the sandbox. The harness inside it receives its own
  enclosing context and cannot read the personal policy or the owner settings.
- The file that runs is the one that Ravel resolved, whatever else of that name
  PATH or Grove's own directory holds.
- Nested completion, force-stop and cancellation leave the run of the enclosing
  session intact.
- The harness cannot consume the caller's stdin or obtain its terminal or
  multiplexer handles.
- Real sandbox probes deny reads and writes to an unrelated project. They permit
  the staged artifacts and the explicitly granted runtime reads.
- Force-stop, a harness exit without a result, malformed or unsupported results,
  a supervision failure, cancellation, output symlinks, missing outputs and
  destination races fail visibly.
- A writer that stays in the group of the harness after a clean acknowledgement
  stops before any output publishes.
- The existing interactive Grove job control and the session-epoch checks stay
  green.
