# Standalone invocations

`grove run` runs one task kind without creating or discovering a grove,
project, jj workspace or driver lease. It may be called by a task inside a
running grove. The owner's harness-dispatch policy selects the command for the
kind, as it does for a lifecycle session, and Grove reads no configuration of
its own. It does not integrate AgentAnyware.

## Selection and launch

The policy, the owner's settings and the record store are personal files that a
confined process must not read, so the command is selected before confinement
and outside it. Grove runs `harness-dispatch run --confine` in the staged
working directory, with the kind and the invocation's prompt, and dispatch
selects, records the run, and confines and supervises the harness it spawns.
[Harness selection and execution](harness-selection-and-execution.md#grove-integration)
owns what is passed and what comes back, and its
[confinement](harness-selection-and-execution.md#confinement) owns the sandbox.
Selection runs in Grove's own environment, so a grant or bound the owner set
for the policy applies. The confined harness receives a small environment and
the run's identity: a standalone invocation has a run record. A refused,
cancelled or timed-out selection launches nothing and publishes nothing.

## Isolation contract

Each invocation has a private temporary directory. Inputs are copied into its
working directory; declared output files are exported only after successful
completion. Duplicate artifact names, symlinks and existing output destinations
are refused. All outputs are validated before any are published. Publication
must not overwrite a destination created while the task was running; any partial
publication is reported explicitly.

Filesystem confinement is mandatory and is dispatch's. Installed system runtime
resources, the selected executable and dispatch's own executable are readable;
additional runtime files such as harness credentials require explicit
read-only grants, which Grove passes on as `--runtime-read`. The parent project
is not a runtime resource. Writes are confined to the working directory and the
private run directory dispatch keeps for the harness's temporary files and its
exit channel. Failure to establish confinement prevents launch. The selected
harness's permission flags cannot disable this outer boundary.

The invocation receives neither its parent's launch directory nor repository
selectors or terminal/multiplexer capabilities. Its exit channel is its own run's
and distinct from any enclosing session's. The harness acknowledges its work with
`harness-dispatch exit`, named by its canonical path in the prompt, which ends
only that invocation; tree verbs are unavailable in this context. Grove publishes
only on the exit-signal ending with dispatch's exit 0: a missing
acknowledgement, a harness that fails after acknowledging, an abnormal exit or a
cancellation cannot publish outputs as success.

Grove runs dispatch noninteractively, in its own POSIX session with no
inherited input or terminal streams, and its output, the harness's included,
goes to the invocation's transcript. Dispatch owns the harness's cancellation
and kills its process group at once, and Grove forwards its own cancellation to
dispatch and waits for it, so nested cancellation finishes inside an outer
supervisor's grace. Both have ended before outputs are considered. Processes
intentionally escaping their process group require stronger backend
containment; no process-tree guarantee may be claimed merely from a
process-group kill.

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
- The policy selects outside the sandbox; the harness inside it receives a
  recorded run ID and cannot read the personal policy, the owner settings or
  the record store.
- The file that runs is the one dispatch resolved, whatever else of that name
  PATH or Grove's own directory holds.
- A nested exit signal and nested cancellation leave the enclosing session's run
  intact.
- The harness cannot consume caller stdin or obtain its terminal/mux handles.
- Real sandbox probes deny reads and writes to an unrelated project while
  permitting staged artifacts and explicitly granted runtime reads.
- Harness exit without the exit signal, a failure after it, cancellation,
  output symlinks, missing outputs and destination races fail visibly.
- Existing interactive Grove job control and session-epoch checks remain green.
