# Standalone invocations

`grove run` runs one task kind without creating or discovering a grove,
project, jj workspace or driver lease. It may be called by a task inside a
running grove. The owner's harness-dispatch policy selects the command for the
kind, as it does for a lifecycle session, and Grove reads no configuration of
its own. It does not integrate AgentAnyware.

## Selection

The policy, the owner's settings and the record store are personal files that a
confined process must not read, so the command is selected before confinement
and outside it. Grove asks harness-dispatch to inspect the kind, with the
invocation's prompt and parameters, and launches the command the inspection
reports. [Harness selection and execution](harness-selection-and-execution.md#grove-integration)
owns what is passed and what comes back. Selection runs in Grove's own
environment, so a grant or bound the owner set for the policy applies. The
confined harness receives the invocation's small environment, as before, and no
harness-dispatch run identity: a standalone invocation has no run record. A
refused selection launches nothing and publishes nothing.

## Isolation contract

Each invocation has a private temporary directory. Inputs are copied into its
working directory; declared output files are exported only after successful
completion. Duplicate artifact names, symlinks and existing output destinations
are refused. All outputs are validated before any are published. Publication
must not overwrite a destination created while the task was running; any partial
publication is reported explicitly.

Filesystem confinement is mandatory. Installed system runtime resources and the
selected executable are readable; additional runtime files such as harness
credentials require explicit read-only grants. The parent project is not a
runtime resource. Writes are confined to the invocation directory and its
dedicated completion channel. Failure to establish confinement prevents launch.
The selected harness's permission flags cannot disable this outer boundary.

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
- The policy selects outside the sandbox; the harness inside it cannot read the
  personal policy, the owner settings or the record store.
- Nested completion and cancellation leave the outer completion channel intact.
- The harness cannot consume caller stdin or obtain its terminal/mux handles.
- Real sandbox probes deny reads and writes to an unrelated project while
  permitting staged artifacts and explicitly granted runtime reads.
- Child exit without completion, bad tokens, nonzero exits, cancellation,
  output symlinks, missing outputs and destination races fail visibly.
- Existing interactive Grove job control and session-epoch checks remain green.
