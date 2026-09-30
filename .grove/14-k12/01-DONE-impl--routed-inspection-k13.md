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

## Decisions (running log)

**Package layout.** Everything lives under `crates/harness-dispatch/`: the Rust
front in `src/`, the TypeScript worker in `worker/src/`, the SDK source in
`worker/sdk/`, the type-check fixtures in `worker/typecheck/`, and the worker
build script in `scripts/worker.sh`. `worker/package.json` and `worker/bun.lock`
pin the development-only type checker (TypeScript 7.0.2, `@types/bun` 1.4.2);
the compiled worker imports nothing from `node_modules`. The Taskfile's
`dispatch:*` tasks are thin wrappers over that script and cargo. An extraction
moves the directory and the tasks.

**The worker builds only through the Taskfile, and the front knows which source
it must match.** A `build.rs` that ran Bun would make the Rust package need Bun
to build, and the node brief says the worker builds only through Taskfile tasks.
So the route is an explicit task prerequisite: `task dispatch:worker` compiles
it, and `scripts/check.sh` runs that before `cargo test`. Staleness is caught by
identity rather than by ordering. `build.rs` (no Bun, only hashing) digests the
worker's source set and embeds it; the build script computes the same digest and
`--define`s it into the worker, which reports it in its first frame. A missing
worker, or one built from other source or another package version, refuses with
exit 5 and names `task dispatch:worker`, so a bare `cargo test` after editing
TypeScript fails loudly instead of testing stale code. No test is skipped.

**Installed relative layout: `<real front dir>/../libexec/harness-dispatch/`.**
The worker is `harness-dispatch-policy` there, with the SDK's declarations and
readable source in `sdk/`. That is Homebrew's convention for private
executables, keeps the worker off PATH, and in a checkout resolves to
`target/libexec/harness-dispatch/` for both cargo profiles. The front
canonicalizes `current_exe()` first, so a symlink to it still finds the worker.
Today's archives are flat, so `release-layout-k17` must add `bin/` and
`libexec/` to them, or change this one constant.

**Compiled Bun 1.4.2 workers do not auto-install.** A probe on this host had a
policy import `is-odd` with no `node_modules`. Plain `bun` fetched it from npm
(the positive control), while the four-switch compiled worker refused with
`ERR_MODULE_NOT_FOUND` and created no cache. A command-seam test pins the
refusal.

**The worker snapshots; the front validates and routes.** The worker imports
the entry and returns its `policy` export as JSON, replacing any function,
bigint, symbol, Map or Set with a `{"$harnessDispatch": "<type>"}` marker so a
function where a model string belongs is refused as a function rather than
dropped or stringified. It also refuses a non-plain object, whose prototype
methods would otherwise vanish unseen. Rust is the one validator: every refusal
carries a `policy.catalog[1].provider`-style location, and Rust resolves the
route. For `routes` that is the whole evaluation. `computed-selection-k21` adds
the worker-side `select` call and validates its result against the same
snapshot.

**Shape rules this leaf fixed, now in the spec.** The export is a plain
object. `model` and `effort` are nonempty, like `id`, `provider` and `program`.
A slot argument is exactly `{ slot: "<name>" }` over the spec's seven names.
Argument semantics (prompt exactly once, `runId` refused) stay with
`harness-exec-k14`. `schemaVersion` is checked before unknown fields, so a
future version reports as `unsupported_version`, not as unknown fields.
Unknown fields refuse at the top level, in candidates and in slot objects.

**Later forms are refused by name, hidden from help.** `--prompt`,
`--prompt-file`, `--task-file`, `--task-id`, `--context`, `--choice`,
`--policy-env`, `--timeout-ms`, `--context-bytes`, `--state-dir`, `run` and
`record` parse and then refuse with `unsupported_input`, exit 2. `select` and
`loadContext` refuse with `unsupported_form`, exit 3. Each owning leaf replaces
its refusal.

**Diagnostics are drained, not yet bounded.** Two threads drain the worker's
stdout and stderr, and after reaping the front waits at most one second for a
descendant still holding them. The 256 KiB bound and its output-limit error
belong to `bounded-context-k22`. The 1 MiB frame cap is enforced now as a
`protocol_error`, because a length prefix needs some cap. k22 owns its
documented refusal.

**The worker's descriptor 3 is a socket pair end, and nothing else crosses.**
The child end is duplicated above the standard descriptors (a caller with
stdin closed would otherwise put it at 0), placed at 3 in `pre_exec`, and every
higher descriptor is marked close-on-exec. The first test run hung because the
original pair end was shadowed rather than dropped. The front then held both
ends, and a dead worker never read as EOF. The worker-failure tests caught it,
and `evaluate` now drops the original before spawning.

**Bun 1.4.2 leaves its compile template behind.** Every `bun build --compile`
leaves a read-only ~60 MB `.<hash>-00000000.bun-build` copy in its cwd,
whatever the output path. `dispatch.sh build` compiles from a throwaway
directory that a subshell trap removes.

**Release documents changed only where this leaf made them false.** Release step
1 runs `scripts/check.sh`, which now needs Bun, so `release-doctor.sh` checks
the pin (read from `dispatch.sh`, seen to fail with a wrong pin) and
`docs/RELEASING.md` names it. The new member inherits the version and carries
`release = false`. `cargo release config` resolves it exactly as it does
`keyed-launch`. RELEASING's *seven packages* heading is cited by source-exact
books, so its count is left to `release-layout-k17`, when the archives ship the
package. A note in that leaf's body says so.

**Usage documentation starts in the package.** `crates/harness-dispatch/README.md`
documents this leaf's surface: install, authority, the routes form, SDK types,
both inspection forms and the refusal table. It moves with an extraction.
Later leaves extend it, and `dispatch-documentation-k41` consolidates it.

**No in-session reviewer.** The node's scheduled `review-impl` names these
foundations as its doubts, so this producer's review is already scheduled.

**Done-when instruments.** The command-seam tests are in
`crates/harness-dispatch/tests/`.

- Member, version, lints and no Grove dependency: the manifest, and
  `cargo tree -p harness-dispatch` with paths stripped. The same command lists
  `grove-loop` for `grove-llm`.
- Worker location, symlinks and identity: `worker.rs`. It covers
  `a_symlink_to_the_front_still_finds_its_worker`, missing-worker decoys on
  PATH, in the cwd and in the environment (each seen to fire at the layout
  path), build, version and protocol mismatches beside an accepted-identity
  control, and protocol and crash failures. A hand-run staleness probe edited
  `main.ts`. The front refused the old worker with exit 5, and both digest
  implementations agreed before and after the edit.
- Embedded SDK and declarations:
  `the_type_checked_fixture_selects_through_the_embedded_sdk`, and
  `task dispatch:typecheck`. Its controls were seen to fail on a broken
  fixture, a stale `@ts-expect-error` and damaged declarations.
- Authority: `authority.rs`. It covers the hostile cwd and environment beside
  their explicit firing configuration, explicit relative `--config`,
  replacement without merge, a personal import of a repository entry, symlinked
  entries, and missing, directory, unset and relative HOME refusals.
- Worker isolation:
  `the_worker_runs_in_a_private_empty_directory_with_null_stdin_and_a_fresh_environment`.
  It checks the cwd, its emptiness and removal, the exact environment keys, null
  stdin, and descriptor 7 closed while the channel is open. For output capture,
  see the two `policy_output…` and `a_refusal_carries…` tests in `inspect.rs`.
- Validation, `select`, incomplete mapping, imports and no auto-install:
  `inspect.rs`, in the invalid-shape table and its valid twin,
  `an_unrouted_kind…`, `a_missing_relative_import…`, `a_missing_package…` and
  `an_entry_that_throws…`.
- Both report forms and later-form refusals: `inspect.rs`.
- Tasks and checks: `task dispatch:install PREFIX=<scratch>` installed a pair
  that inspected through a symlink. `task check` passed all 12 principal checks,
  with 1485 tests and none failing.
