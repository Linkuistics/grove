# routed-inspection-k13

## Goal

Create the `harness-dispatch` Rust package and its compiled policy worker.
`harness-dispatch inspect --kind K` then evaluates a static `routes` policy and
reports which configured candidate would run and why.

## Context

Read the node brief and the spec sections it names. Choose where the TypeScript
worker, the SDK declarations and the Taskfile tasks live, within the package
boundary, and record the choice in this leaf's running log. Settle how
`cargo test` and `task check` obtain a compiled worker built from the current
source. Two acceptable routes are a build step that invokes the pinned Bun, or
an explicit task prerequisite. Either way a missing worker fails loudly and no
test is skipped.

## Done when

- A new workspace member `harness-dispatch` exists, with its own binary. It has
  `version.workspace = true` and `[lints] workspace = true`, and no dependency
  on any Grove crate.
- A worker compiled with Bun 1.4.2 and all four no-autoload switches is found
  relative to the real front executable, following symlinks. The front process
  verifies the worker's protocol and build identity before sending a request.
  A mismatched or missing worker refuses with exit 5 and a remedy. No PATH, cwd
  or environment variable can substitute another worker.
- The worker registers `harness-dispatch/sdk` as an embedded virtual module
  before it imports the selected entry. The SDK exports the policy and
  candidate types a `routes` policy needs, and its declarations type-check.
- Policy authority follows the spec. The personal default is
  `~/.config/harness-dispatch/policy.ts`. An explicit `--config` resolves
  against the original cwd. There is no cwd search and no environment-selected
  entry. A missing, unreadable or invalid entry refuses, naming the path.
  Inspection states the resolved path and whether its authority is personal or
  explicit.
- The worker runs in a private empty directory, with null stdin and a fresh
  environment of HOME, a PATH snapshot, TMPDIR, LANG and LC_*. It speaks over a
  private framed channel. Worker stdout and stderr are captured and never
  interleaved with structured output.
- The policy shape is validated: `schemaVersion` 1, a nonempty `version`, the
  catalog shape with unique IDs, nonempty provider labels, model and effort
  strings, and exactly one of `routes` or `select`. `select` is refused as not
  yet supported by this release. An unrouted kind refuses as an incomplete
  mapping with exit 3; no default is invented. An entry whose import is
  missing, or which throws while loading, refuses and names the entry. No
  package is installed automatically.
- `inspect` prints the source, authority, policy version, candidate, provider,
  model, effort, reason and timing, as human text and as `--json` schema version
  1. Unknown input versions and fields are refused with their location.
- Taskfile tasks build and install the pair locally and run the package checks.
  `scripts/check.sh` includes those checks. Command-seam tests run with
  temporary policies and no Grove files or binary.

## Notes

Argv expansion and `run`, the selection deadline, the handoff record and the
choice contract are this node's later leaves. Inspection may omit argv until
`harness-exec-k14` adds it, but must not print a placeholder that implies
expansion happened.

This is the plan's largest leaf (review `harness-selection-and-execution-k42`,
finding F4). If it proves too big, decompose it along behavior, so that each
child still inspects something. For example, human-text inspection of a
route can come first and `--json` inspection with version refusals after it.
Do not split out a crate-only or protocol-only child: nothing could
demonstrate it.
