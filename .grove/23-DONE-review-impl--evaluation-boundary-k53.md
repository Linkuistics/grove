# evaluation-boundary-k53

**Reviews:** evaluation-boundary-k27

## Goal

An adversarial, inspection-only read of the whole `evaluation-boundary-k27`
node: signal cancellation during selection (`selection-cancellation-k28`), the
signal-transparent handoff (`signal-transparent-handoff-k29`), and the proof
that ambient repository and environment inputs stay inert, with the
`--policy-env` grant (`ambient-authority-k30`). Produce findings, not fixes.

## Context

The node brief names the risk. Signal races, the ordering around the
linearization point, pre-`main` SIGPIPE capture and environment authority are
claims the compiler cannot check. A mistake here silently ends Grove sessions,
or grants completion authority to code that should not hold it. Every
mechanism was seen to fail against a mutated front or build (each leaf's
running log lists the mutations), but a mutation shows only that a test
notices the break it was written for. The review's job is the break nobody
wrote a test for. `grove-dispatch-k31` launches Grove sessions through this
code next, under the controlling PTY.

## What to doubt

- **The handled window.** `choose` installs the INT, TERM and HUP handlers
  after the inputs, the entry, the state directory and the worker's location
  are settled, and they stay installed across the record commit. Is there a
  point from the worker's fork to the final check where a signal is swallowed
  and a harness still runs, or a worker survives the front? Consider a signal
  during `Handlers::install` itself, between the reap and the commit, and one
  that arrives while a run lookup waits on the store's lock.
- **The linearization point.** After the commit the handled signals are
  blocked in the main thread, and the final check reads the handler's note and
  `sigpending`. The drain threads were spawned with those signals blocked.
  Does any other thread exist then, from std, SQLite or a dependency, that
  could take a process-directed signal after the check and before exec? Is
  the not-executed append ever reported as recorded when it was not?
- **Entry signal state.** The pre-`main` initializer records the mask and
  every signal's disposition, and the pre-exec hook reinstates them after
  `do_exec` resets SIGPIPE. Is `.init_array` / `__mod_init_func` guaranteed to
  run before std's `init` on both platforms, for a PIE and a static-pie? Is
  the hook's work async-signal-safe? Does reinstating *every* signal's
  disposition ever hand the harness something the caller did not have, such
  as a signal whose disposition the kernel reports but that cannot be set?
- **The deadline beside the handlers.** `selection-deadline-k44`'s bound must
  hold unweakened. Can a signal handled with `SA_RESTART` extend any wait past
  the deadline, or can a cancellation and a timeout race to a wrong exit?
- **Environment authority.** The worker gets HOME, PATH, TMPDIR, LANG and
  `LC_*` plus exact grants, and `BUN_*`, `NODE_OPTIONS`, `NODE_PATH`, `LD_*`,
  `DYLD_*` and `HARNESS_DISPATCH_*` are never granted. Is that exclusion list
  complete for what Bun 1.4.2 itself reads at startup? Look at the variables
  Bun's source consults before the worker's first line: `NODE_*` beyond the
  two, `JSC_*` options, `TZ`, `UV_*`, `SSL_CERT_*`, proxy variables. Is any of
  them a way to run code or change which code loads? Does a base variable,
  such as a TMPDIR or HOME the caller controls, give an ambient input reach
  that the private cwd was meant to deny? Is reporting whether a granted name
  is set a leak worth caring about?
- **The probe builds' faithfulness.** Each firing configuration drives a probe
  build directly, through a test-side protocol driver (`tests/support/direct.rs`),
  because a probe's identity is one no front accepts. Is the driver faithful
  enough that "fires under the probe" and "inert through the front" differ in
  the one control and nothing else? The `unregistered` probe is selected by a
  `HARNESS_DISPATCH_PROBE` define that the shipped build sets empty: can the
  shipped worker ever take the probe's branch? Can a probe build reach an
  archive by any route but the worker's own path, which the installed smoke
  test would refuse?
- **The unfired classes.** A HOME bunfig and a cwd tsconfig are reported as
  having no firing configuration, with a tripwire test. Is either actually
  reachable in some build or placement the tripwire does not try, such as
  `XDG_CONFIG_HOME` for bunfig, or a tsconfig in a parent of the entry?
- **Structured output.** A policy that floods both streams, with text shaped
  like a protocol frame and like a report, leaves `--json` one document and
  the protocol its own. Is there a stream the policy can reach that is not
  captured: a descriptor above 3 it opens itself, `/dev/tty`, or the channel
  at descriptor 3?

## Pointers

- Spec: `docs/specs/harness-selection-and-execution.md`, sections
  `#execution-contract`, `#policy-authority`, `#diagnostics`, the lifecycle
  and authority rows of `#test-seams`, and the firing-configuration table.
- Evidence: `docs/design/harness-selection-and-execution/runtime-evidence.md`,
  *Ambient authority*.
- Decisions and the mutations seen to fire: the running logs of
  `selection-cancellation-k28`, `signal-transparent-handoff-k29` and
  `ambient-authority-k30`.
- Code: `crates/harness-dispatch/src/cancellation.rs`, `signal_state.rs`,
  `run.rs`, `worker.rs`, `environment.rs`, `choice.rs` and `cli.rs`;
  `worker/src/main.ts`; `scripts/dispatch.sh` (`probes`).
- Tests: `crates/harness-dispatch/tests/cancellation.rs`, `handoff.rs`,
  `deadline.rs`, `hostile.rs`, `environment.rs`, `authority.rs` and
  `worker.rs`, with `tests/support/direct.rs`, `hold.rs`, `probe.rs` and
  `signal-probe.c`.

## Done when

- Every doubt above has been read against the code and the tests, and each
  finding names its file, its line and a failure scenario, or the doubt is
  recorded as examined and found sound.
- A review with findings worth acting on cuts its `integrate-review-impl` leaf
  where `pick` reaches it next.

## Decisions (running log)

**Review boundary.** The producer consists of `11ea27e0`
(`selection-cancellation-k28`), `14dc2b95`
(`signal-transparent-handoff-k29`) and `f70f1270`
(`ambient-authority-k30`, closing `evaluation-boundary-k27`). The reviewed
range is `59898d76..f70f1270`, and this session started on an empty working
change whose parent was `f70f1270`. No implementation or test file was changed,
and no test, build, lint or format command was run. Recorded verification in
the producer leaves is evidence from those leaves, not a fresh verification
claim by this review.

**Discovery evidence.** Tier 2 was attempted against graph project
`Users-antony-Development-grove.new-tool-for-harness-selection-and-execution`.
Its ready, fast generation is `2026-09-29T11:18:53Z`, predating this crate.
The crate-scoped symbol search returned no symbols and no further page;
coverage checks for the relied-on source, tests, scripts and documents returned
`freshness: not_tracked`, with `docs/` and root `scripts/` excluded. The
material conclusions therefore use direct source reads. This is a bounded
review of the named node, not an exhaustive graph audit or a claim that no
other issues exist.

**Signal and deadline doubts settled.** The handler lifetime, shielded drain
threads, final block-and-check, entry-state restoration and post-reap
cancellation checks agree with the contract and their recorded controls.
No actionable defect was found in those paths. See the individual dispositions
below for limits on platform and timing evidence.

**Ambient-cache doubt settled.** The four compiled autoload controls do not
disable Bun's runtime transpiler cache. Its default HOME-backed lookup supplies
executable output for external policy imports. This is F1; it is separate from
Bun's package-install cache and Node's standalone-disabled compile cache.

**Grant-filter doubt settled.** Bun 1.4.2 reads more consequential `NODE_*`
names than the filter refuses. Resolver behavior and adoption of a runtime IPC
descriptor are F2. Plain `JSC_*` is not a corresponding finding: Bun disables
JSC's ordinary environment-option scan and reads `BUN_JSC_*`, which the filter
already excludes. Presence reporting intentionally discloses only the set bit
of an explicitly granted name.

## Findings

Both findings are inferred from the committed front and the pinned Bun
`bun-v1.4.2` source, downloaded from the tagged upstream source archive and
read without executing it. The failure scenarios have not been dynamically
reproduced in this inspection-only session. Integration must independently
triage them and establish executable controls for findings it accepts.

### F1 — P1: Disable the ambient runtime transpiler cache before starting Bun

**Location:** `crates/harness-dispatch/src/worker.rs:465`, with
`crates/harness-dispatch/src/environment.rs:102` supplying HOME and
`environment.rs:64` admitting an exact `XDG_CACHE_HOME` grant.

The front clears the environment and passes the selected values, but supplies
no host-owned runtime-cache disable setting. In Bun's pinned source the runtime
transpiler cache is enabled and reads external file sources of at least 4 KiB.
Without `BUN_RUNTIME_TRANSPILER_CACHE_PATH`, it looks under `XDG_CACHE_HOME` if
present, otherwise under HOME (`Library/Caches/bun/@t@` on macOS,
`.bun/install/cache/@t@` on Linux). Cache entries retain input and feature
hashes but contain executable output; the output integrity hash is unkeyed and
can be zero. An empty cache-path setting or `0` disables this mechanism.
[Bun cache source](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/RuntimeTranspilerCache.rs#L603-L671),
[enabled feature](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bun_core/feature_flags.rs#L88).

**Failure scenario:** an admitted personal policy, or an ordinary imported
helper, is large enough to be cached. A writable ambient cache contains an
entry retaining that input's valid metadata but replacing its output with
JavaScript that imports repository code or changes the selected candidate.
For example, a caller's absolute HOME can name a repository-controlled home,
or an explicitly granted `XDG_CACHE_HOME` can name a shared cache. Bun accepts
the cache entry and uses its output as module source. The named policy's bytes
need not change, and the private empty cwd and four autoload switches do not
prevent it.
[Cache-output loading](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/RuntimeTranspilerCache.rs#L381-L538),
[external-import cache hit](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/jsc_hooks.rs#L2884-L2942).

This does not assume a hostile-code sandbox against an owner who intentionally
imports untrusted code. The gap is the host implicitly accepting executable
bytes from a cache the owner never named as policy. HOME reaches that cache
without any grant. Existing hostile fixtures cover dotenv, bunfig, tsconfig,
specifier shadows and runtime options, but contain no poisoned-cache control.
The claim that other ambient inputs stay inert is therefore too broad.

**Integration direction:** make cache disabling an immutable host setting
before Bun startup, or establish an equivalently private cache authority.
Keep the caller's cache-related variables from overriding it. Add a control
whose altered cached output really fires under the pinned compiled runtime,
then show that the ordinary front runs the admitted source instead. Reconcile
the environment description and runtime evidence with the resulting behavior.

### F2 — P2: Refuse Bun's resolver and inherited-IPC variables as grants

**Location:** `crates/harness-dispatch/src/environment.rs:113`.

The filter refuses only `NODE_OPTIONS` and `NODE_PATH`. It admits
`NODE_PRESERVE_SYMLINKS`, `NODE_CHANNEL_FD` and
`NODE_CHANNEL_SERIALIZATION_MODE`. The standalone startup path calls
`load_extra_env_and_source_code_printer`, which applies the first to the
resolver and records the latter pair as a pending IPC descriptor and mode.
These are runtime controls, not values merely delivered to policy.
[Standalone startup](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/cli/run_command.rs#L1205),
[environment handling](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/VirtualMachine.rs#L3690-L3735).

**Failure scenarios:** with `NODE_PRESERVE_SYMLINKS=1` granted, an ordinary
symlinked helper's bare dependency can resolve from the symlink's directory
instead of the real helper's directory, selecting a different module tree
without a change to the policy import. Bun's directory-resolution logic
explicitly branches on that option.
[Resolver source](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs#L6239-L6266).
With `NODE_CHANNEL_FD=3` granted, a reusable policy module that normally
attaches `process.on("message", ...)` or calls `process.send()` causes Bun to
adopt dispatch's descriptor 3 for its own IPC. Initialization writes Bun's
version packet and installs its socket consumer on the same descriptor that
the worker uses for its framed protocol. Selection can then refuse with a
protocol/worker failure or fail to receive its expected frame. Adoption is
lazy; the variable alone does not immediately seize the channel.
[IPC initialization](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/ipc_host.rs#L473-L593).

The default scrubbed environment excludes these names, so this requires an
explicit grant. Nevertheless, the grant boundary promises to reject runtime
code-loading and private-channel authority before policy evaluation; accepting
these grants creates precisely those effects. Existing environment tests
exercise the named exclusions but never these names. This is not the separate,
accepted ability of intentionally trusted policy to open descriptor 3 itself.

**Integration direction:** source-ground the additional exclusions, cover
their refusals even when unset, and retain positive controls for harmless exact
grants. Use a symlink-resolution control and a genuine inherited-IPC control
to confirm the effects on the pinned runtime. Update the usage, help and area
specification consistently. Do not infer that every `NODE_*` name is equally
dangerous; `NODE_COMPILE_CACHE` is explicitly disabled for standalone builds.

## Examined doubts

1. **Handled window — sound in the named lifecycle.**
   `cancellation.rs:97-123` installs handlers before any worker fork. A signal
   for an as-yet-uninstalled disposition retains its entry behavior, when
   there is still no worker to orphan; a handled signal is retained in the
   atomic note. Partial installation unwinds installed dispositions.
   `choice.rs` keeps the handlers through reap and resolution, and `run.rs:81-97`
   keeps them through the record commit and handoff notice. Lookup waits are
   clipped to the selection time left and the fixed store wait, then checked
   for deadline and cancellation. Cancellation and lock-wait tests have
   positive holds. Group signals reach policy descendants; front-only
   signalling does not promise to supervise arbitrary policy descendants,
   and the producer's control states that limit explicitly.

2. **Linearization — sound for the delivered front.**
   The only front-created background threads in this path are the stdout and
   stderr drains, both created inside `cancellation::shielded`; bundled SQLite
   has no configured sort-worker threads here. `block_and_check` blocks the
   installed signals and checks the note and pending set, excluding
   caller-blocked signals from cancellation. After the check they remain
   blocked until their entry dispositions and mask are reinstated. No other
   unblocked signal recipient was found in the inspected dependencies and
   path. `run.rs:145-163` attaches an append failure to `RunNote.unrecorded`;
   it never reports a successful not-executed append when that append failed.
   The handoff suite forces a signal after commit by stalling stderr and
   separately forces append failure. A handoff notice followed by a refusal
   is the documented JSON-lines `run` behavior.

3. **Entry signal state — sound for supported release targets.**
   `signal_state.rs:57-93` uses ELF `.init_array` or Mach-O
   `__mod_init_func`, before the C main that enters Rust's `lang_start` and
   std's SIGPIPE initialization. ELF dynamic and static-PIE startup both
   process executable initializers before main; this review did not build a
   static-PIE variant. The recorded native handoff and all-target installed
   smoke controls exercise default and ignored PIPE/HUP plus mask state.
   `run.rs:114` uses `Command::exec` in the front process, not a child after
   fork. `reinstate`'s success path uses atomics and signal APIs; allocation
   in an error branch is therefore not a post-fork deadlock here. KILL/STOP
   are explicitly skipped; unsupported numbers and glibc's reserved 32/33
   are not successfully queried and so are not reinstated. A failed
   initializer refuses before evaluation. No other queryable-but-unsettable
   signal was found on the supported libc/platform paths.
   [glibc dynamic/static initializer ordering](https://github.com/bminor/glibc/blob/glibc-2.39/csu/libc-start.c#L121-L190),
   [Rust 1.85 exec ordering](https://github.com/rust-lang/rust/blob/1.85.0/library/std/src/sys/pal/unix/process/process_unix.rs).

4. **Deadline and cancellation — sound within the stated bound.**
   Channel reads and writes use remaining-deadline slices; the atomic note
   is checked between waits. TERM and its cleanup grace precede KILL/reap,
   including policy loops, ignored TERM and exit handlers. Lookup lock waits
   consume the same budget. Post-conversation and post-reap checks make a
   received cancellation overrule worker failure or timeout. `SA_RESTART`
   does not remove the socket deadlines or SQLite's bounded busy wait. Store
   work after a bounded read and record commit has the already documented
   separate bounds; this review makes no new hard real-time guarantee.
   `deadline.rs`, `cancellation.rs`, `lookup.rs` and their producer logs cover
   those branches without relying on policy cooperation.

5. **Environment authority — F1 and F2.**
   HOME exposes the transpiler cache; exact grants admit consequential
   resolver/IPC variables. The base PATH snapshot and private temporary cwd
   give no additional implicit entry search in the inspected worker path.
   JSC's plain environment scan is disabled, while Bun's option namespace
   is `BUN_JSC_*` and already refused. TZ, thread-pool settings, trust-store
   variables and proxies may affect time, resource scheduling or network
   requests made by trusted policy; granting those effects is consistent
   with exact grants and did not reveal another code-loading or private-IPC
   injection path in this review. Set-bit reporting is an intentional,
   bounded disclosure to the invoking owner; values are not reported.

6. **Probe faithfulness — sound for the recorded firing configurations.**
   `tests/support/direct.rs` drives the compiled probes on descriptor 3 with
   the same hello/evaluate framing, selected entry and declared fixture
   environment. Its runtime cwd deliberately exposes the tested ambient
   fixture; the front instead uses its private cwd. This is part of the
   claimed control, not a driver-only way of importing fixture code. The
   early import effects being tested do not need later context/select
   exchanges. Probe identities carry a distinct prefix, refused by the
   front before evaluation. `HARNESS_DISPATCH_PROBE` is a compiled define,
   empty in the shipped build, never read from process environment.
   `scripts/release-common.sh:34-53` lists the exact archive contents;
   `scripts/release.test.sh` rejects extra probe files. A probe substituted
   for the expected worker path is caught by installed smoke's identity
   and real-dispatch checks. These controls do not authenticate an arbitrary
   attacker-replaced binary, which is outside this boundary's contract.

7. **Unfired classes — sound with the evidence's stated limits.**
   HOME bunfig and invocation-cwd tsconfig remain tripwires for the tested
   placements, not counted proofs. In Bun's config path the compiled
   no-bunfig guard precedes both HOME and `XDG_CONFIG_HOME` lookup. The
   no-tsconfig control guards resolver autoloading, including parent walks;
   the recorded entry-adjacent alias fixture supplies a firing control.
   The source adds no reason to treat the two unfired placements as proven
   firing classes. Linux ambient-fixture controls remain explicitly
   unmeasured in the runtime evidence; all-target installed smoke does not
   fill that gap. Package `main`/`exports` resolution is already owned by
   `package-entry-resolution-k52`, not a new finding here.

8. **Structured output — sound within the trusted-policy contract, except F2.**
   The inherited-descriptor sweep leaves only 0-3, even for a descriptor
   above the soft limit; fd 1/2 diagnostics are drained into bounded captures.
   Shaped protocol/report text on those streams cannot become a channel
   frame or an extra inspect document. A trusted policy can deliberately
   open `/dev/tty`, another output path or fd 3; preventing those deliberate
   effects would require the excluded hostile-code sandbox. The statement
   about one JSON document applies to `inspect`; `run` emits the documented
   handoff/refusal JSON lines on stderr. F2 is actionable because a granted
   runtime variable lets an ordinary IPC-using import claim the channel
   implicitly, not because fd 3 is unknowable to trusted code.

## Handoff

`evaluation-boundary-k54` is the adjacent integration step for
`evaluation-boundary-k53`, inserted ahead of the live `grove-dispatch-k31`
sibling.
The integration reads this committed report by handle and independently
triages it; no finding is promoted to a settled design decision by this review.
