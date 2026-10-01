# worker-directory-chain-k63

**Reviews:** worker-directory-chain-k61

## Goal

An adversarial, inspection-only read of `worker-directory-chain-k61`: the
worker's move to `/` before it loads anything, the owner-only start directory,
and the tests and documents that claim no directory above the worker's start
directory takes part in an evaluation. Produce findings, not fixes.

## Context

k61 was offered three repairs and took none of them. Its running log says why:
Bun moves the resolver's directory with `process.chdir`, so the worker can
start in the private directory and resolve from `/`. That choice was made
without the human, on the reasoning that no stated control is weakened.
`package-json-autoloading-k62` runs next and rests on it, so attack it first.

Read the spec's `#policy-authority` and the firing-configuration table under
`#test-seams`, *The worker's directory* in the runtime evidence,
`docs/adr/policy-evaluation-precedes-process-replacement.md`, and k61's
running log. The code is the statement above the embedded table in
`worker/src/main.ts`, the directory creation in `evaluate` in `src/worker.rs`,
the `unmoved` probe in `scripts/dispatch.sh`, and three tests: the package
case and the tripwires in `tests/hostile.rs`, the start-directory case in
`tests/worker.rs`, and the root-directory case in `tests/authority.rs`.

Specific doubts k61 could not settle from inside:

- **Half the argument is seen, not read.** That dotenv and bunfig are read
  only from the directory a process starts in rests on one probe result on one
  host. k61 read `run_env_loader` and did not trace its caller on a compiled
  executable's start path. Read the bun-v1.4.2 source for anything the runtime
  reads from the current directory lazily, after start. For any such file the
  private empty directory is no longer a control, and only a compile switch is.
- **Nothing may resolve before the move.** The move is a statement in the
  worker's main module. Check what Bun does with the start directory before
  that statement runs, with the switches as shipped and with package.json
  autoloading on, which k62 is about to enable. k61 saw a 4 GiB `package.json`
  in TMPDIR leave a selection unaffected in an experimental build, and no test
  holds that.
- **`/` is not empty and cannot hold a fixture.** A container image can carry
  `/node_modules`, `/package.json`, `/.env` or `/bunfig.toml`, and a process
  running as root can write there. k61 says the first two are already above
  every module in a file, and tests the last two only through a policy that
  moves into a hostile directory. Decide whether that stands for `/`.
- **The policy-facing change.** A policy's `process.cwd()` is now `/` and its
  relative writes fail or land there. Check the shipped examples, the SDK and
  the documents for anything that assumed the private directory.
- **The firing configuration departs from k61's done-when**, which named the
  shipped worker started under the planted directory. The `unmoved` probe
  replaces it. Check that the probe differs from the shipped build by the move
  alone, and that the case's three arms prove what its name says.
- **The two changed tripwires.** The cwd tsconfig tripwire's entry now moves
  the worker back into the caller's cwd. Check that it still watches the class
  it reports, and that the new moved-into tripwire could fail.
- **The mode.** The start directory is created `0o700`. Check the claim that
  no window exists, and what a restrictive umask or an unwritable TMPDIR does.
- **Linux is unmeasured for the control.** The installed smoke test runs no
  hostile fixture.

## Done when

- Each doubt above has a finding or a stated reason there is none, with the
  source or the observation behind it.
- The spec, README, ADR and runtime evidence have been read against the code
  and the tests, and every statement that is not true of them is a finding.
- Findings are recorded in this file. If any is actionable, an
  `integrate-review-impl` leaf is inserted ahead of
  `package-json-autoloading-k62`.

## Notes

Inspection only. A probe build or a scratch experiment is reading, not fixing.

## Decisions (running log)

**Review boundary and evidence.** Review producer commit
`76bce0611ef8f7dd577d38e8ccac1738f885b444` against its parent and the current
source. The working copy was empty at entry. No implementation or test code
is changed, and no test, build, lint or format command is run. The producer's
recorded check and three-target smoke results are historical evidence, not
results independently rerun here. Tier 2 graph verification found no dispatch
symbols; coverage generation `2026-09-29T11:18:53Z` reports the package files
as `not_tracked` and the documents as excluded. The review therefore uses
direct source throughout this package and its cited documents.

**The startup argument holds for the main VM, but not every later VM.** Bun
1.4.2's `RunCommand::boot_standalone` loads bunfig and configures defines
before `Run::start` evaluates the main module. The shipped flags disable the
dotenv directory traversal. Later, however, a native `Worker` creates a fresh
VM and a fresh default-file ledger, applies the standalone flags, and calls
`configure_defines` again. With dotenv autoloading enabled it reads the
process's directory at that later time. This is a source-derived finding,
not a claim to have executed a new probe. The shipped switch remains a
control; the private start directory does not independently control this
path. Record this as F1 and hand the distinction to integration.

**The move retains root resolution.** `/node_modules` remains reachable for
modules with no file location, as the runtime evidence itself acknowledges.
The spec, README and changelog nevertheless exclude all ancestors of the
start directory, including `/`, and TMPDIR may itself be `/`. Record their
overbroad guarantee as F2. Root resolution was already part of normal
file-based resolution; its continued existence alone is not evidence of a
newly introduced code-execution vulnerability.

**Handoff.** The two actionable findings earn `worker-directory-chain-k64`,
inserted as an `integrate-review-impl` immediately ahead of
`package-json-autoloading-k62`. Its charter points here and leaves triage to
that session. This session has no `HARNESS_DISPATCH_RUN_ID`; no creator run
is asserted. The review is a root leaf and closes no ancestor node.

## Findings

### F1 — P2: the private start directory is not an independent dotenv control for a later native Worker

Locations: `crates/harness-dispatch/tests/hostile.rs:843`,
`docs/specs/harness-selection-and-execution.md:334`,
`docs/specs/harness-selection-and-execution.md:1087`,
`docs/design/harness-selection-and-execution/runtime-evidence.md:467`,
`docs/adr/policy-evaluation-precedes-process-replacement.md:42`, and the
startup-only rationale at `crates/harness-dispatch/worker/src/main.ts:82`
and `crates/harness-dispatch/src/worker.rs:500`.

The new tripwire changes directory and immediately reads the main VM's
environment. That VM has already loaded its environment before the entry
was imported. It never creates a native `Worker`, which is a later runtime
operation that loads default environment files again. Thus the empty-start
directory arm of the dotenv square does not support the unqualified claim
that each control still holds alone throughout evaluation, and the
moved-into dotenv class has a source-identifiable candidate firing
configuration that the tripwire does not exercise.

The pinned runtime's source supplies the missing path:

1. [`WebWorker::create` and `start_vm`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/web_worker.rs#L354)
   copy the parent's transform inputs and clone its environment loader, then
   construct a new VM. This is not the bundler's `for_worker` clone of
   already-configured defines.
2. [`Loader::clone_for_worker`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/dotenv/env_loader.rs#L598)
   resets `default_files_loaded` to empty. The new VM's transpiler is freshly
   initialized by
   [`init_runtime_state`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/runtime/jsc_hooks.rs#L443).
3. [`start_vm`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/jsc/web_worker.rs#L734)
   sets the environment behavior, reapplies the standalone compile flags,
   and calls `configure_defines` before the nested worker's entry executes.
   [`apply_standalone_runtime_flags`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bun.js.rs#L10)
   makes that behavior disabled in the shipped build, but loading-enabled
   in the `autoload` probe.
4. [`run_env_loader`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/bundler/transpiler.rs#L758)
   discovers files from the then-current `top_level_dir`.
   [`load_default_files`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/dotenv/env_loader.rs#L713)
   opens their names relative to the process's current directory. With no
   owner-directed move, that directory is now `/`, rather than the private
   empty directory the process started in.

A bounded case to establish in integration is an admitted policy that
creates and terminates a native `Worker` from an admitted absolute file,
and consumes the worker's report of the dotenv variables. For a writable
fixture, let the policy move into the planted directory before creating that
worker. Run the same policy and fixture with the shipped worker and the
autoload probe started in an empty directory. The source predicts the
probe's nested worker can see the planted variables while the shipped
switch prevents them. This review has not executed that case; distinguish
that prediction from k61's measured main-VM results.

The consequence is a lost independent control for this path, not a bypass
of the shipped dotenv-off switch. Starting a native worker is an explicit
policy operation, but its automatic environment-file loading is not an
explicit import or read of that file. Integration should establish the
behavior, distinguish main startup from later VM startup in the test and
documents, and reconcile k61's no-weakened-control reasoning with the
producer task's trade-off instruction. No implementation repair or human
decision is made by this review.

### F2 — P2: the public ancestor guarantee incorrectly includes the filesystem root

Locations: `docs/specs/harness-selection-and-execution.md:385`,
`crates/harness-dispatch/README.md:1222`,
`CHANGELOG.md:163`, and the analogous absolute claim at
`crates/harness-dispatch/src/worker.rs:9`.

The spec excludes every directory above the worker's start directory, and
the README and changelog exclude a package anywhere above it. But `/` is
such an ancestor. The worker moves to `/`, and Bun's
[`resolve_and_auto_install`](https://github.com/oven-sh/bun/blob/bun-v1.4.2/src/resolver/resolver.rs#L1286)
uses that current directory for an importer with no file location. Therefore
`/node_modules` remains a resolution source; with package.json autoloading
on, `/package.json` remains one too. A supported caller may also set TMPDIR
to `/`, directly contradicting the unqualified TMPDIR exclusion. A Linux
image may supply these files even when the invoking user cannot create them.

`runtime-evidence.md:489` already records this limitation correctly. Root
was also above file-based importers before the change, so its continued use
does not by itself introduce a new source of authority. Nevertheless the
public guarantee must exclude root explicitly and describe the move as
removing the non-root start-directory chain for modules with no file
location. Reconcile the README, spec, changelog and source description with
the evidence; k62 should inherit the qualified guarantee.

## Disposition of the mandated doubts

| Doubt | Finding or reason there is no additional finding |
|---|---|
| Startup files and lazy reads | F1. For the main VM, `RunCommand::boot_standalone` (`src/runtime/cli/run_command.rs:1110`) loads bunfig at 1133, applies the standalone flags at 1192 and calls `configure_defines` at 1194, before `Run::start` loads the entry at 1424. `configure_defines` is idempotent once defines are loaded. The later native-Worker path is the exception above. `Bun.Transpiler` explicitly disables environment loading (`src/runtime/api/JSTranspiler.rs:1017`). No later bunfig reload was found in the inspected ordinary-import or native-Worker startup paths; explicitly asking native APIs to build or load code remains a policy effect. |
| Resolution before the move, including package.json on | No additional finding for the shipped startup. The main module has static imports, so embedded modules and prefixed built-ins are loaded before its body, but they do not require ambient package discovery: `resolve_and_auto_install` handles built-ins before directory resolution and returns embedded graph matches before its top-level-directory fallback (`src/resolver/resolver.rs:1140`, `:1294`); embedded fetch is served directly from the graph (`src/runtime/jsc_hooks.rs:3742`). Standalone VM initialization suppresses auto-JSX directory discovery (`jsc_hooks.rs:519`, `VirtualMachine.rs:4104`), and the shipped dotenv behavior skips directory traversal. Enabling package.json alone does not enable that traversal. k61's package-on observations are consistent with these paths; k62 already owns the missing regression cases and must land them before enabling the switch. Read “before it loads anything” as before owner-policy loading; the embedded imports precede the move. |
| Files at `/` and the validity of a moved-into fixture | F2 for root packages and F1 for later dotenv loading. For main-VM dotenv/bunfig startup, the source trace explains why those root files are not autoloaded after the move. The existing moved-into probe is a limited main-VM observation; it is not a complete surrogate for every later runtime operation at root. No fixture was planted at `/`. |
| Policy-facing cwd change | No additional finding. The README at 1160 and spec at 331 state `/` and `request.cwd`, and `authority.rs:228` observes it. The SDK's `SelectionRequest.cwd` and measured-read contract retain caller cwd; `Session.read` resolves against `this.cwd` (`host.ts:228`). The shipped example sources use catalog computation and host reads/lookups, with no dependency on a private cwd or relative scratch writes. The final harness cwd is unaffected because only the child moves. The visible cost is documented; no sandbox for relative policy writes is promised. |
| The firing configuration and its three arms | No additional finding. `dispatch.sh:202` supplies the shipped switches; the `unmoved` probe uses exactly those switches and the same source, with its distinct identity and the `main.ts:87` branch skipping only the move. `hostile.rs:707` tests front refusal under inspect and run, probe loading, and directly-started shipped-worker refusal, for data, blob and registered virtual modules. It removes the firing sentinel between arms. The identity mismatch is intentional and the direct driver bypasses it; `a_probe_build_is_never_accepted_as_an_installations_worker` covers rejection by the front. This is an explicit, justified replacement for the original done-when's impossible moved-worker firing arm. |
| Changed tripwires | F1 for the moved-into dotenv arm. The cwd tsconfig arm still names the same class: an entry elsewhere asks for an alias after moving into caller cwd. `aliased_entry_moved_into` at 609 uses a dynamic import after the move, so it actually exercises resolution there. It is reported as an unfired class, not a passing control; the separate beside-entry case at 620 observes the tsconfig probe applying the fixture. No additional tsconfig finding. |
| Mode, restrictive umask and unwritable TMPDIR | No additional implementation finding. tempfile 3.27.0's `src/dir/imp/unix.rs:6` passes the requested permissions to `DirBuilder::mode` before creation, so there is no create-open-then-chmod window. Actual bits are `0o700 & !umask`: group/other bits cannot be added, but a mask removing owner bits may make the directory unusable. The spec's owner-only claim is about excluding other users, not guaranteeing exact owner bits. The stand-in test asserts exact 700 for its normal test environment. Creation failure in an unwritable TMPDIR and spawn/chdir failure in an inaccessible directory return worker-stage refusal before policy or harness launch (`worker.rs:506`, `:581`). This is a source-derived failure path, not a newly executed permissions experiment; the change does not establish safety against replacement by a hostile owner of the parent directory. |
| Linux measurement | No additional finding. The runtime evidence at 485 explicitly says hostile fixtures were not run on Linux. k61 records successful installed smoke runs for the move on all three targets, and makes no Linux hostile-fixture result claim. The source implements the same move and switches there; the platform-specific authority behavior remains unmeasured. |

## Review evidence and limits

The changed source, command-seam fixtures, build/probe construction, README,
authority and test-seam spec sections, worker ADR and runtime evidence were
read against the producer's committed diff. The Bun source archive already
present under `/tmp` was checked byte-for-byte against freshly retrieved
`bun-v1.4.2` raw source for the resolver, VM, transpiler, startup command,
native worker, dotenv loader, bunfig loader, runtime hooks and standalone
flag application. All comparisons matched. Source inspection supports the
two findings; it does not replace the paired integration's runtime checks.
No production or test changes, new runtime measurements, builds, tests,
linting or formatting were performed in this review.
