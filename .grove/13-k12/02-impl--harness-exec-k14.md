# harness-exec-k14

## Goal

Make `harness-dispatch run` hand the selected joint choice to its harness. It
expands the candidate's argv from literals and whole-argument slots, resolves
the program, and replaces itself with the harness while preserving cwd,
descriptors and exit behavior. `inspect` shows the same expanded argv.

## Context

The contract is in the spec's `#command-interface`, `#policy-and-choice` (slots
and program resolution) and `#execution-contract` sections. Take only the plain
exec here. Handled signals, the linearization point and signal-state
transparency belong to `evaluation-boundary-k27`. The handoff record and the
`runId` slot belong to `dispatch-records-k23`, so `runId` is refused as not yet
supported.

## Done when

- `--prompt TEXT` or `--prompt-file PATH` is read once, preserving its bytes and
  trailing newlines. It must be valid UTF-8 with no NUL and at most 1 MiB. `run`
  requires exactly one of them. `inspect` renders a marked placeholder when
  neither is given. Terminal stdin is never read, and the prompt never reaches
  the worker.
- `--task-file` resolves against the original cwd. `--task-id` accepts an opaque
  UTF-8 identity of at most 1024 bytes. Both reach the policy request and their
  slots, and neither supplies kind or identity from anything else.
- Arguments are literals or slot objects: `prompt` (exactly once), `kind`,
  `taskFile`, `taskId`, `model` and `effort`. A slot fills one whole argument.
  An absent optional input cannot satisfy a used slot. There is no shell,
  splitting or interpolation.
- The program is an absolute path, a PATH name, or a relative path containing a
  separator, which resolves against the caller's cwd. Inspection reports the
  resolution. Only the selected program is checked. Not found exits 127 and
  unexecutable exits 126, and neither ever falls back to another candidate.
- `run` execs the harness in the caller's cwd with its descriptors and
  environment. Short choice diagnostics go to stderr, and stdout and stdin stay
  the harness's. An exec error reports errno with a remedy and exits nonzero.
- Command-seam tests show a fake harness receiving literal spaces, quotes, shell
  punctuation and newlines in the prompt and paths. It keeps its cwd and exits
  with its own code and signal.
