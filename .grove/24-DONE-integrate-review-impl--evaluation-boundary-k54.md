# evaluation-boundary-k54

**Integrates:** evaluation-boundary-k53

## Goal

Triage the committed review of `evaluation-boundary-k27`, apply the findings
that hold against the implementation and its contract, and verify the
integrated result before Grove's dispatch integration consumes this boundary.

## Context

Read `evaluation-boundary-k53` by its stable handle and committed artifact.
Its findings are review evidence, not this task's charter; independently
confirm each one and record accepted and rejected dispositions.

The producer changes are `11ea27e0` (`selection-cancellation-k28`),
`14dc2b95` (`signal-transparent-handoff-k29`) and `f70f1270`
(`ambient-authority-k30`, closing `evaluation-boundary-k27`). The area contract
is `docs/specs/harness-selection-and-execution.md`; recorded runtime evidence
is `docs/design/harness-selection-and-execution/runtime-evidence.md`.
Use the existing command seam and repository Taskfile for post-fix
verification. Distinguish source-derived review scenarios from controls that
have actually run on the pinned compiled runtime.

This step sits directly before `grove-dispatch-k31`. Keep package entry
resolution in its existing `package-entry-resolution-k52` work item unless
independent triage establishes a necessary dependency.

## Done when

- Every finding in `evaluation-boundary-k53` has a reasoned disposition, with
  accepted findings addressed and rejected findings explained.
- Required post-fix verification passes, with executable evidence for the
  accepted failure scenarios and accurate usage and runtime documentation.
- Any substantial remaining redesign or review work is externalized under
  the integration skill; this task is retired and committed as one change.

## Notes

No production or test changes were made by the review. Its committed report
contains all findings and the limitations of its inspection evidence.

## Decisions (running log)

**Findings read from the review's commit.** `46c30aad` holds the committed
report of `evaluation-boundary-k53`: two findings, F1 and F2, and eight
examined doubts recorded as sound. The doubts need no disposition; the
findings are triaged below against the pinned compiled runtime built from
`f70f1270`'s sources (worker `ad1b0271bbc3…`, Bun 1.4.2, macOS arm64), and
against the `bun-v1.4.2` source archive, fetched from GitHub's tag archive.

**F1 accepted — a real issue, fixed in the artifact.** Reproduced through the
unchanged front. A 6336-byte explicit policy, inspected with a fresh HOME,
left `Library/Caches/bun/@t@/<input hash>.pile` under that HOME: the front's
worker writes the owner's shared Bun cache, a side effect nothing documents.
With the entry's output changed from `check: "good"` to `check: "evil"` and its
output hash zeroed, the same file, with the same reported `sha256`, selected
`evil`; a fresh HOME selected `good`. So inspection and the run record would
name the admitted file's digest while other code ran. Source
(`src/jsc/RuntimeTranspilerCache.rs`, `really_get_cache_dir`): an empty or `0`
`BUN_RUNTIME_TRANSPILER_CACHE_PATH` disables the cache and is consulted before
`XDG_CACHE_HOME` and HOME; `MINIMUM_CACHE_SIZE` is 4 KiB; a zero output hash is
never checked. It is the only disable switch, since `IS_DISABLED` is set only
when that lookup finds no directory. Fix: the front sets
`BUN_RUNTIME_TRANSPILER_CACHE_PATH=0` in every worker environment as a setting
of its own. `BUN_*` is never granted, so no caller value can replace it, and a
granted `XDG_CACHE_HOME` is consulted only after it. Disabling rather than a
private cache: a policy is small, so transpiling it each time costs little,
and a private cache would be a new store to own.
Removing HOME would also disable it but breaks the documented base set.

**F2 accepted — a real issue, fixed in the artifact.** Reproduced through the
unchanged front, each ungranted run the control. With a directory symlink
`linked -> ../lib/real` and `dep` present in both parents, `--policy-env
NODE_PRESERVE_SYMLINKS` changed a helper's bare `dep` from the real parent's
to the link's, with no change to any import. A symlinked *file* does not
fire: `resolver.rs`'s branch governs a directory's real path. With
`NODE_CHANNEL_FD=3` granted, a module calling `process.send` when it exists
made the front refuse with `protocol_error`, a frame of 2065856105 bytes
(`{"vi`), five runs of five: Bun wrote its own IPC bytes into the private
channel. Also seen: a `select` policy calling `process.on("message")` ended
`worker_failed`; with `NODE_CHANNEL_SERIALIZATION_MODE=advanced`, a routes
policy doing the same drew Bun's version packet as a malformed frame. Each
fails closed, which is why P2 rather than P1. Fix: refuse
`NODE_PRESERVE_SYMLINKS` beside `NODE_OPTIONS` and `NODE_PATH`, and every
`NODE_CHANNEL_*` name as the class Node and Bun read the inherited IPC
channel from.

**Other `NODE_*` names classified, not excluded.** Enumerated every `NODE_*`
environment read in the archive (`env_var.rs` registrations, dotenv-map
reads, `process.env` reads in `src/js`), then classified each.
`NODE_COMPILE_CACHE` is disabled in standalone executables
(`NodeCompileCache.rs`, `init_from_env_once`). `NODE_PRESERVE_SYMLINKS_MAIN` is
read only when resolving a `bun run` target (`run_command.rs`), never on the
standalone path. `NODE_REPL_EXTERNAL_MODULE` is read only by the REPL.
`NODE_UNIQUE_ID` puts `node:cluster` in worker mode, but it can talk only
over an adopted channel, which `NODE_CHANNEL_*` now excludes. The rest
(`NODE_ENV`, `NODE_DEBUG*`, `NODE_NO_WARNINGS`, `NODE_PENDING_DEPRECATION`,
`NODE_TLS_REJECT_UNAUTHORIZED`, `NODE_EXTRA_CA_CERTS`, `NODE_USE_SYSTEM_CA`,
`NODE_USE_ENV_PROXY`, `NODE_V8_COVERAGE`, `NODE_PORT`, `NODE_TEST_WORKER_ID`,
`NODE_CLUSTER_SCHED_POLICY`) set values, diagnostics, trust or network
behavior for trusted policy and load no code. None of the non-`BUN_*`,
non-`NODE_*` names in `env_var.rs` loads code in the worker either: the
`XDG_CONFIG_HOME` bunfig path sits behind the compiled no-bunfig guard, and
`DYLD_ROOT_PATH` is already a `DYLD_*` name.

**Controls, each seen to fail against a mutated front.** Two command-seam
tests in `tests/hostile.rs` carry the accepted findings, each class inert
through the front beside the shipped worker started directly, where it
fires. `the_runtime_transpiler_cache_…` has the worker cache the policy under
HOME, alters the entry's output and zeroes its hash. The same file then routes
to `poisoned`, from HOME and from a copy under `XDG_CACHE_HOME`. The front
writes no entry, and beside both caches it selects `admitted`, with the XDG
one granted. `node_resolver_and_channel_variables_…` uses the
directory-symlink helper, which routes `link` with `NODE_PRESERVE_SYMLINKS`
and `real` without it. Its `process.send` module reaches the driver as Bun's
`{"vi` rather than the policy frame. Through the front, the caller's values
are inert and each grant refuses. `tests/support/direct.rs` now reports an
over-bound length as a `malformed` frame without reading on, as the front
does, instead of waiting on a two-gigabyte body.

Mutations: with the host setting dropped, the cache test failed at its
first assertion, a `.pile` under HOME. With that assertion also disabled,
the front arm failed, selecting `poisoned`. With both new exclusions
removed, the grant loop failed, exit 3 where 2 was expected. With the base
set admitting `NODE_PRESERVE_SYMLINKS`, the front's inert arm routed `link`.
Admitting `NODE_CHANNEL_FD` made it refuse with `protocol_error` for
2065856105 bytes. Each mutation was reverted, and the source diffs clean
against its pre-mutation copy.

**Contract and documentation reconciled.** The spec's policy-authority
paragraph now states the host setting and the two new exclusions. Its
authority seam row and firing-configuration table now carry both classes, and
its summary names the setting. The crate README and `--policy-env` help list
the exclusions, the README explains the cache setting, and the runtime
evidence adds both rows and the before-fix observations. One sentence in
`docs/adr/policy-evaluation-precedes-process-replacement.md` records the cache
as a second startup input that only an environment set before start
controls. The decision is unchanged, and that is its rationale. The root
brief's handoff facts for later leaves name `environment::HOST` and the new
classes. The no-grant inspection line now says the worker has only those
variables *of yours*, since it also holds the host's own. No reviewer was
spent: each fix is covered by a command-seam test seen to fail under
mutation, and no new redesign remains to externalise.

**Verified.** `task check` passed after the last edit, with all 12 principal
checks, clippy at deny, fmt, the probe builds and every `harness-dispatch`
suite. The two new hostile tests, the updated environment and authority tests
and the unit exclusion test each ran unfiltered. The digest of the whole
working-copy diff was the same before and after the run. The worker's
build identity is unchanged (`ad1b0271bbc3…`). The fix is the front's, and
changes neither the worker, the installed layout nor a native dependency, so
the brief's per-target installed-smoke rerun is not owed. Linux remains unmeasured for these
hostile controls, as for the others in the runtime evidence.
