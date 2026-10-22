# Harness selection and execution — Grove caller contract

Grove launches independently installed Ravel through its public two-verb CLI.
Grove owns its task tree, prompts, ending instructions, result meanings, teardown
and provider receipts. Ravel routes and supervises the selected process. Then it
returns the opaque argument that `ravel end-session RESULT` published.

The [Ravel CLI](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md),
[executable specification](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/specs/session-execution.md),
[result transport](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-channel.md)
and [authority boundaries](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/authority.md)
own the executable behaviour. This document owns Grove's use of that interface.
The [visual views](../design/harness-selection-and-execution/README.md) show the
same caller boundary. Ravel injects no instructions and interprets no result
token.

<a id="problem"></a>
## Ownership

The owner decides which command, model and effort performs a kind. Grove decides
which leaf needs work. The selection stays in personal policy. This makes the
router usable by independent callers and keeps model choices out of Grove
releases ([policy ownership](../adr/harness-selection-is-owned-by-policy.md)).

<a id="package-boundary"></a>
## Independent packages

| Owner | Contract |
|---|---|
| Grove | Selects one leaf, composes its mandate and holds launch admission. Runs Ravel as a job, captures its stdout result and interprets it after the reap. |
| Ravel | Evaluates owner policy, supervises the selected process and isolates the enclosing ending context. Returns a published argument after cleanup. |
| Owner policy | Constructs every selected command and its provider, model and effort labels, with any caller-specific routing or review rules. |
| Grove caller assets | Read the task and Creator conventions. Optionally wrap selection with caller-owned provider evidence through ordinary SDK imports. |

Grove resolves the first executable regular `ravel` in an **absolute PATH
directory**. It canonicalises that path once and probes it with `--version`.
Grove supports stable `ravel 1.x`, from 1.0.0. A relative or empty PATH entry
adds no fallback to the current directory. An incompatible, malformed or
unsuccessful probe refuses with an upgrade remedy. Grove never falls through to
another installation.

The probe trusts the program on the owner's PATH. A version string is not
authentication. Canonicalisation fixes the path selection, not an inode.

Grove keeps its local `keyed-launch` runner for its own jobs. Ravel keeps its own
runner, front and compiled worker. No Rust dependency, private worker protocol,
shared version or shared installation prefix crosses the project boundary
([supervision decision](../adr/dispatch-supervises-the-harness.md)).

<a id="command-interface"></a>
## Public invocation

Grove builds an argument vector directly. It joins each value to its flag, so a
leading dash stays data. It uses the already resolved Ravel path for the launch,
for confinement and for the quoted ending command in the prompt.

| Caller | Public inputs |
|---|---|
| Lifecycle | `start-session --kind=KIND --prompt=MANDATE --exit-dir=LAUNCH_DIR --task-file=TASK_FILE --task-id=HANDLE` in the working-tree root. |
| Standalone | `start-session --kind=KIND --prompt=PROMPT --exit-dir=WORK_DIR --confine`, plus explicit repeated `--runtime-read=FILE`, in a private staged directory. |
| Managed session | The exact canonical command of the launch prompt, equivalent to `ravel end-session grove-continue` or `ravel end-session grove-force-stop`. It has no session ID or target path. |

`--exit-dir` belongs to the mechanics of channel allocation and confinement. It
reaches no policy. Task identity is caller data. Grove supplies no `--param`, no
policy or configuration path, no record path and no private ending-file
argument. Ravel's generic `--param NAME=VALUE` is available to other callers,
without task-tree meaning.

Grove scaffolding and tree verbs consult no policy and
admit any well-formed kind. An unrouted kind refuses only when its leaf
launches.

<a id="policy-and-choice"></a>
<a id="sample"></a>
<a id="policy-authority"></a>
<a id="owner-settings"></a>
<a id="imports-alias"></a>
## Owner policy and caller assets

The personal `~/.config/ravel/policy.ts` selects the command. Ravel's settings at
`~/.config/ravel/settings.json` supply bounds and explicit environment grants.
`.ravel-choice` selects among the options of the owner's policy, when that
policy calls the SDK helper. Grove reads none of those files itself.

The public SDK is `ravel/sdk`. Grove's [readable caller assets](../../assets/ravel-policy/README.md)
ship under its own `libexec/ravel-policy`. Personal policy imports them
explicitly. They are ordinary files, not embedded Ravel modules.

The owner reads
and copies the sample explicitly and keeps an existing policy. Neither Grove nor
Ravel installs it automatically. Ravel has no init or inspection verb. The
[configure-grove skill](../../plugins/grove/skills/configure-grove/SKILL.md)
owns the workflow for the owner. The Ravel specification owns import
resolution.

<a id="bounded-context"></a>
<a id="dynamic-dispatch"></a>
## Selection and context

Ravel passes the supplied prompt unchanged to policy. Its request, parameters
and optional measured context are generic caller data. There is no run lookup.
Grove's adapter reads only the supplied task file through the public context
host. It returns a refusal for invalid relationships.

An owner's policy can
compute from kind, prompt or context, but Grove ships no route to a deciding
agent. The owner constructs commands from choices that their code controls.

Selection runs outside the confinement of the managed process, with owner grants
and bounds. A refused selection launches no process and leaves the leaf live.
Fix the named policy, choice or Creator evidence and rerun Grove. No inspection
reservation or fallback selection takes part.

<a id="execution-contract"></a>
<a id="supervision"></a>
<a id="group-cleanup"></a>
## Jobs and authority

Grove makes Ravel a foreground job through its local runner. Ravel makes the
selected process its own job. Each supervisor owns the terminal handoff,
restoration and group cleanup of its direct child. Ravel's
[process decision](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/adr/the-launched-child-is-a-job.md)
owns its side. Grove's [local runner decision](../adr/the-launched-child-is-a-job.md)
owns the side of the caller.

Processes that leave a managed group are outside
that guarantee of group cleanup. An uncatchable death of a supervisor can orphan
a process.

Grove scrubs enclosing session authority and provider-evidence authority before
probes and launches. Then it supplies lifecycle launch admission only to the
foreground session. Standalone helpers inherit no tree authority. Ravel isolates
policy from completion controls and gives the managed process a fresh enclosing
context. Neither inherited context nor same-user receipts authenticate hostile
local code.

<a id="exit-signal"></a>
<a id="run-ending"></a>
## Ending and returned result

Grove's lifecycle results are exactly `grove-continue` and `grove-force-stop`.
Ravel's first publication wins. Ravel returns it without a newline after the
cleanup of the managed process. Managed output and diagnostics are separate
from the result on stdout.

A published argument can succeed even when the
managed process later exits nonzero. A natural exit without a result supplies no
completion acknowledgement. A Ravel cancellation, or a failure of supervision or
cleanup, gives no successful result.

Grove captures stdout in a held unlinked file. It reads the supported value after
it reaps its own Ravel job. It neither reads an ending file nor looks up a run.

The result meanings belong to Grove. Continue advances ordinary work. Force-stop
stops without completion and leaves the remaining live work resumable. Ravel
adds no verb or interpretation rule. Standalone currently needs continuation to
publish outputs. Force-stop cannot acknowledge standalone completion.

<a id="grove-integration"></a>
<a id="lifecycle-launch"></a>
## Lifecycle interpretation

After the reap, Grove restores the terminal and invalidates session epoch
admission. Only then does it interpret the launch. A failed invalidation reads no
disposition and launches no next session. Once that gate passes, the driver
interprets in this order:

1. Driver interruption stops the loop with its signal.
2. A successful Ravel status with exactly `grove-force-stop` stops without
   completion. This holds even when the launch recorded teardown or a supervisor
   group survives. The CLI exits 1 and reports force-stop. It does not retire
   the live leaf.
3. Otherwise this launch's explicit teardown record finishes the loop, whatever
   the result, the supervisor status or a surviving group.
4. A surviving Ravel group stops the loop before continuation.
5. A successful Ravel status and exactly `grove-continue` advance to the next
   pick.
6. Anything else stops. A rerun of Grove resumes the remaining tree.

Only a successful supervisor status permits Grove to read the result. A failed or
interrupted supervisor cannot override teardown with bytes on stdout. A valid
force-stop overrides the completion disposition. It does not restore a tree that
the finish already tore down. Ordinary stops keep their existing CLI exit 0
behaviour. Grove re-raises a driver interruption as its original signal.

A finish session records teardown with `grove-llm record-teardown` after the
confirmed finish cycle. It does so before it executes the ending command of its
prompt as its last action. A missing `.grove/` alone attests no confirmed
teardown.

**Original live v22 sessions keep their launch contract.** Installed Grove 22.1.0
and its original dispatch executable drive the extraction. Such sessions still
end with `harness-dispatch exit`. They use the original admission and the
Creator-run convention. For a finish, they record teardown before that signal.

Never put a new Ravel command into a session that is already running. The
[legacy-driver transition](../legacy-driver-transition.md) keeps that complete
installation through the final finish signal.

<a id="confinement"></a>
## Standalone interpretation

`grove run KIND` stages the inputs. Its prompt supplies the output and
completion instructions. Ravel evaluates policy outside confinement and confines
only the managed process. Explicit runtime files are read-only. Policy and
settings stay outside the managed reads. A missing sandbox refuses, with no
unconfined fallback.

<!-- tc: depart synonym-rotation because confirmed names the human's approval of a finish, while check and validate name Grove's tests of a run -->
Grove logs and streams the sanitised managed output. It checks cancellation, a
surviving group and a successful Ravel status. Then it accepts exactly
`grove-continue`.

Only after that does it check the outputs and publish them through the held
descriptor of the work directory, without overwriting a destination. A refusal, an absent or unsupported result, or invalid outputs
publish nothing. An exact `grove-force-stop` reports failure and publishes no
outputs. The prompt supplies the canonical commands and both result meanings.
[Standalone invocations](standalone-invocations.md) owns that staging, output
and transcript contract.

<a id="identity-and-creator"></a>
<a id="review-policy"></a>
<a id="records-and-outcomes"></a>
## Caller-owned creator evidence

Ravel has no durable store, run identity, record or observation verb, or
historical lookup. Grove's opt-in provenance wrapper captures the evidence of
the selected provider in a canonical receipt. A finishing producer writes
`**Creator:** recorded <JSON>` from its own `GROVE_PROVIDER_EVIDENCE`. The line
goes directly under the review's `**Reviews:**` line.

Missing evidence requires
the owner's explicit `declared <provider>` form. A retry replaces the reference
only when it finishes the producer. A node's creator is the child that closes
it. Review retries read the committed reference.

The [provider evidence contract](../grove-provider-evidence.md) and the
[Creator decision](../adr/a-review-carries-its-creator-reference.md) own receipt
validation, association and policy refusal. The owner decides provider
diversity. Ravel enforces no rule for reviews. Current assets refuse legacy
`run <id>` references without opening old history. Only original live-v22
sessions use that form with their original policy. Existing dispatch history
stays with the old tool.

<a id="diagnostics"></a>
## Refusals and remedies

Ravel's public CLI diagnostics and exit codes belong to its specification.
Grove reports them and stops on an unsupported or missing result. A selection
refusal leaves the task live. The owner fixes an unrouted leaf in their policy,
never through an admission check in a tree verb. Grove's public loop outcome
and its process status are distinct. The [usage guide](../USAGE.md) shows
refusal recovery and interruption.

<a id="delivery"></a>
## Delivery

Homebrew's Grove formula depends on the independently versioned
`linkuistics/taps/ravel`. Grove archives contain its binaries and readable
caller assets. Archive users install Ravel separately. Keep the complete prefix
of each project. Grove assumes no matching Cellar version, no neighbouring
executable and no bundled worker.

The [release instructions](../RELEASING.md)
and the [delivery boundary](../grove-delivery-contraction.md) own Grove's
packaging. Ravel owns its [runtime evidence](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/runtime-evidence.md)
and its [published installation evidence](https://github.com/Linkuistics/Ravel/blob/3916a4a1d17fef925db7e9c730bd4c23d6df3ac8/docs/publication-evidence.md).

<a id="test-seams"></a>
## Caller acceptance

| Claim | Existing witness |
|---|---|
| Absolute PATH resolution, version refusal and independent prefixes | Resolver, activation and installed fixtures in `crates/grove/tests/` and `crates/grove/src/dispatch/ravel.rs`. |
| Literal public arguments and exact ending path | `crates/grove-loop/tests/session_command.rs` and real installed command fixtures. |
| Result capture, rejection and post-reap precedence | `crates/grove-loop/tests/session_result.rs`, installed result fixtures and loop-driver tests. |
| Lifecycle admission, cancellation and terminal recovery | Driver and epoch tests, installed lifecycle tests and local runner PTY tests. |
| Caller policy, context and provider evidence | `task ravel:review`, `task ravel:provenance` and strict typechecking of the installed SDK. |
| Confined staging and publication | Standalone tests and installed fixtures that use fake agents. |
| Independent delivery | Acceptance of the archive, formula and target in the delivery handoff. |

`task check` runs the repository gate, including all source books and plugin
conformance. Explicit installed fixture tasks cover the named ignored cases. The
[caller acceptance](../ravel-caller-surface-audit.md) and the
[delivery acceptance](../grove-delivery-contraction.md) record their bounded
runs. Ravel checks its private worker and runtime controls, and this document
does not duplicate them.

<a id="out-of-scope"></a>
## Boundary limits

There is no direct-command bypass, no persistent Ravel history API and no Grove
policy fallback. Grove adds no daemon, agent-server, dashboard or storage
protocol. Its task tree and methodology remain ordinary artifacts that the
caller owns.
