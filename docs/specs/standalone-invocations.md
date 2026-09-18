# Standalone invocations

`grove run` runs one configured task kind without creating or discovering a
grove, project, jj workspace, driver lease, or local configuration delta. It may
be called by a task inside a running grove. It uses the existing named commands,
bindings, routes, parameters and personal profile selection in
`~/.config/grove/config.kdl`. It does not integrate AgentAnyware.

## Isolation contract

Each invocation has a private temporary directory. Inputs are copied into its
working directory; declared output files are exported only after successful
completion. Duplicate artifact names, symlinks and existing output destinations
are refused. All outputs are validated before any are published. Publication
must not overwrite a destination created while the task was running; any partial
publication is reported explicitly.

Filesystem confinement is mandatory. Installed system runtime resources and the
configured executable are readable; additional runtime files such as harness
credentials require explicit read-only grants. The parent project is not a
runtime resource. Writes are confined to the invocation directory and its
dedicated completion channel. Failure to establish confinement prevents launch.
The configured harness's permission flags cannot disable this outer boundary.

The invocation receives neither its parent's Grove completion authority nor
repository selectors or terminal/multiplexer capabilities. Its own completion
channel is distinct from a task-tree session's epoch channel. `grove-llm complete
--done` acknowledges only that invocation; other tree verbs are unavailable in
this context. A malformed token, missing acknowledgement, abnormal exit or
cancellation cannot publish outputs as success.

The child has its own POSIX session and no inherited input or terminal streams.
The supervisor captures output, owns cancellation, and cleans up its process
group before considering outputs. Nested cancellation must finish child cleanup
within the outer supervisor's grace. Processes intentionally escaping their
process group require stronger backend containment; no process-tree guarantee
may be claimed merely from a process-group kill.

## Visibility

Every invocation announces its kind and log location, streams a readable
transcript through its caller, and reports its final status. The supervisor owns
the display connection; the confined child cannot control the parent terminal
or mux. A supported existing multiplexer can provide a dedicated view. A full
interactive UI embedded in another harness requires that harness's integration
API and is outside this command's initial interface.

## Release notes

Release preparation supplies the previous release's changelog, commit
descriptions and source diff as explicit input artifacts. A configured
`release-notes` kind produces one Markdown body. The deterministic release task
validates and inserts it, commits the notes through jj, and continues its existing
checks, version cut, builds, publication and installation verification.

Existing Unreleased notes remain authoritative. No LLM invocation is necessary
when notes are already present. Tests substitute a deterministic harness and
must never depend on paid LLM calls.

## Verification obligations

- No jj invocation or local config read, including from a directory containing
  hostile `.grove.kdl` and repository selector variables.
- Nested completion and cancellation leave the outer completion channel intact.
- The harness cannot consume caller stdin or obtain its terminal/mux handles.
- Real sandbox probes deny reads and writes to an unrelated project while
  permitting staged artifacts and explicitly granted runtime reads.
- Child exit without completion, bad tokens, nonzero exits, cancellation,
  output symlinks, missing outputs and destination races fail visibly.
- Existing interactive Grove job control and session-epoch checks remain green.
