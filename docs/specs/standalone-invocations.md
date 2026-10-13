# Standalone invocations

`grove run` runs one task kind without creating or discovering a grove,
project, jj workspace or driver lease. It may be called by a task inside a
running grove. The owner's Ravel policy selects the command for the
kind, as it does for a lifecycle session, and Grove reads no configuration of
its own. It does not integrate AgentAnyware.

## Selection and launch

The owner policy/settings are personal files that the confined process cannot
read. Grove resolves and version-checks independent Ravel once from absolute
PATH entries before staging; public `start-session --confine` selects outside
the sandbox and supervises inside it. The unchanged caller prompt supplies the
exact canonical `end-session grove-continue` command. Grove sends kind, prompt,
exit directory and canonical `--runtime-read` grants, with no task identity or
settings override. Ravel's
[public execution contract](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/specs/session-execution.md)
owns confinement and cleanup. It returns only an opaque stdout result after
reap; it supplies no private ending report, persistent store or run identity.
Selection retains owner bounds/grants. Refused, cancelled or timed-out selection
launches no managed process and publishes no outputs.

## Isolation contract

Each invocation has a private temporary directory. Inputs are copied into its
working directory; declared output files are exported only after successful
completion. Duplicate artifact names, symlinks and existing output destinations
are refused. All outputs are validated before any are published. Publication
must not overwrite a destination created while the task was running; any partial
publication is reported explicitly.

Filesystem confinement is mandatory and is Ravel's. Installed system runtime
resources, the selected executable and Ravel's own executable are readable;
additional runtime files such as harness credentials require explicit
read-only grants, which Grove passes on as `--runtime-read`. The parent project
is not a runtime resource. Writes are confined to the working directory and the
private run directory Ravel keeps for the harness's temporary files and its
exit channel. Failure to establish confinement prevents launch. The selected
harness's permission flags cannot disable this outer boundary.

The invocation receives neither its parent's launch directory nor repository
selectors or terminal/multiplexer capabilities. Its exit channel is its own run's
and distinct from any enclosing session's. The harness acknowledges its work with
the canonical `ravel end-session grove-continue` command in its prompt, which
ends only that invocation and takes no session ID. Tree verbs are unavailable.
Grove publishes only after a successful Ravel exit and the exact supported
result. A missing/unsupported result, supervision failure, abnormal supervisor
exit or cancellation prevents success. Ravel's first publication wins: a managed
process exiting nonzero after publishing does not override a successfully reaped
result. Grove adds no contradictory private child-exit interpretation.

Grove runs Ravel noninteractively in its own POSIX session with no inherited
input or terminal streams. It holds public stdout separately; managed output
and diagnostics on stderr go to the invocation's transcript. Ravel owns the harness's cancellation
and kills its process group at once, and Grove forwards its own cancellation to
Ravel and waits for it, so nested cancellation finishes inside an outer
supervisor's grace. Both have ended before outputs are considered, and so has
the harness's process group: Ravel kills what remains of it when the harness
exits and returns the public result only once it is gone, so a writer the harness left
in its group cannot change an output after it is validated. Processes
intentionally escaping their process group require stronger backend
containment; no process-tree guarantee may be claimed merely from a
process-group kill.

Grove's uncatchable death can leave its detached Ravel and harness running;
it publishes nothing. The cancellation grace bounds ordinary nested runs only
when Ravel's cleanup finishes within that grace.
Transcript relay failure also prevents publication and retains the log for
inspection, even when the harness acknowledged successfully.

## Visibility

Every invocation announces its kind and log location, streams a readable
transcript through its caller, and reports its final status. The supervisor owns
the display connection; the confined child cannot control the parent terminal
or mux. A supported existing multiplexer can provide a dedicated view. A full
interactive UI embedded in another harness requires that harness's integration
API and is outside this command's initial interface.

## Release notes

Release preparation supplies the previous release's changelog, commit
descriptions and source diff as explicit input artifacts. A `release-notes`
kind the owner's policy routes produces one Markdown body. The deterministic release task
validates and inserts it, commits the notes through jj, and continues its existing
checks, version cut, builds, publication and installation verification.

Existing Unreleased notes remain authoritative. No LLM invocation is necessary
when notes are already present. Tests substitute a deterministic harness and
must never depend on paid LLM calls.

## Verification obligations

- No jj invocation, including from a directory holding repository selector
  variables, and a `.grove.kdl` or `config.kdl` there or in HOME changes nothing.
- The policy selects outside the sandbox; the harness inside it receives its own
  enclosing context and cannot read the personal policy or owner settings.
- The file that runs is the one Ravel resolved, whatever else of that name
  PATH or Grove's own directory holds.
- A nested result publication and nested cancellation leave the enclosing session's run
  intact.
- The harness cannot consume caller stdin or obtain its terminal/mux handles.
- Real sandbox probes deny reads and writes to an unrelated project while
  permitting staged artifacts and explicitly granted runtime reads.
- Harness exit without a result, supervision failure, cancellation,
  output symlinks, missing outputs and destination races fail visibly.
- A writer left in the harness's group after a clean acknowledgement is stopped
  before any output publishes.
- Existing interactive Grove job control and session-epoch checks remain green.
