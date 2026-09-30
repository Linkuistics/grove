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
