# Harness selection and execution — Grove caller contract

Grove launches independently installed Ravel through its public two-verb CLI.
Grove owns its task tree, prompts, ending instructions, result meanings, teardown
and provider receipts. Ravel routes and supervises the selected process, then
returns the opaque argument published by `ravel end-session RESULT`.

The [Ravel CLI](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/session-cli.md),
[executable specification](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/specs/session-execution.md),
[result transport](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/session-channel.md)
and [authority boundaries](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/authority.md)
own executable behavior. This document owns Grove's use of that interface;
[visual views](../design/harness-selection-and-execution/README.md) show the same
caller boundary. Ravel injects no instructions and interprets no result token.

<a id="problem"></a>
## Ownership

The owner decides which command, model and effort performs a kind. Grove decides
which leaf needs work. Keeping that selection in personal policy makes the
router usable by independent callers and keeps model choices out of Grove
releases ([policy ownership](../adr/harness-selection-is-owned-by-policy.md)).

<a id="package-boundary"></a>
## Independent packages

| Owner | Contract |
|---|---|
| Grove | Select one leaf, compose its mandate, hold launch admission, run Ravel as a job, capture its stdout result and interpret it after reap. |
| Ravel | Evaluate owner policy, supervise the selected process, isolate enclosing ending context and return a published argument after cleanup. |
| Owner policy | Construct every selected command and its provider/model/effort labels, with any caller-specific routing or review rules. |
| Grove caller assets | Read task/Creator conventions and optionally wrap selection with caller-owned provider evidence through ordinary SDK imports. |

Grove resolves the first executable regular `ravel` in an **absolute PATH
directory**, canonicalizes it once and probes that path with `--version`.
Stable `ravel 0.1.x`, beginning with 0.1.0, is supported. Relative/empty PATH
entries introduce no cwd fallback. An incompatible, malformed or unsuccessful
probe refuses with an upgrade remedy; Grove never falls through to another
installation. Probing trusts the owner's PATH program; a version string is not
authentication, and canonicalization fixes path selection rather than an inode.

Grove keeps its local `keyed-launch` runner for its own jobs. Ravel keeps its own
runner, front and compiled worker. No Rust dependency, private worker protocol,
shared version or shared installation prefix crosses the project boundary
([supervision decision](../adr/dispatch-supervises-the-harness.md)).

<a id="command-interface"></a>
## Public invocation

Grove builds an argument vector directly, with each value joined to its flag so
that a leading dash stays data. It uses the already resolved Ravel path for
launch, confinement and the prompt's quoted ending command.

| Caller | Public inputs |
|---|---|
| Lifecycle | `start-session --kind=KIND --prompt=MANDATE --exit-dir=LAUNCH_DIR --task-file=TASK_FILE --task-id=HANDLE` in the working-tree root. |
| Standalone | `start-session --kind=KIND --prompt=PROMPT --exit-dir=WORK_DIR --confine`, plus explicit repeated `--runtime-read=FILE`, in a private staged directory. |
| Managed session | The launch prompt's exact canonical command, equivalent to `ravel end-session grove-continue` or `ravel end-session grove-force-stop`; no session ID or target path. |

`--exit-dir` is channel allocation/confinement mechanics and reaches no policy.
Task identity is caller data. Grove supplies no `--param`, policy/config path,
record path or private ending-file argument. Ravel's generic `--param NAME=VALUE`
is available to other callers without task-tree meaning. Grove scaffolding and
tree verbs consult no policy and admit any well-formed kind; an unrouted kind
refuses only when its leaf launches.

<a id="policy-and-choice"></a>
<a id="sample"></a>
<a id="policy-authority"></a>
<a id="owner-settings"></a>
<a id="imports-alias"></a>
## Owner policy and caller assets

Personal `~/.config/ravel/policy.ts` selects the command. Ravel's settings at
`~/.config/ravel/settings.json` supply bounds and explicit environment grants;
`.ravel-choice` selects among options the owner's policy offers when that policy
calls the SDK helper. Grove reads none of those files itself.

The public SDK is `ravel/sdk`. Grove's [readable caller assets](../../assets/ravel-policy/README.md)
ship under its own `libexec/ravel-policy` and are imported explicitly from
personal policy. They are ordinary files, not embedded Ravel modules. The owner
reads and copies the sample explicitly, preserving an existing policy; neither
Grove nor Ravel installs it automatically. Ravel has no init or inspection verb.
The [configure-grove skill](../../plugins/grove/skills/configure-grove/SKILL.md)
owns the owner-facing workflow; the Ravel specification owns import resolution.

<a id="bounded-context"></a>
<a id="dynamic-dispatch"></a>
## Selection and context

Ravel passes the supplied prompt unchanged to policy. Its request, parameters
and optional measured context are generic caller data; there is no run lookup.
Grove's adapter reads only the supplied task file through the public context
host and returns a refusal for invalid relationships. An owner's policy may
compute from kind, prompt or context, but Grove ships no deciding-agent route.
The owner constructs commands from choices their code controls.

Selection runs outside managed-process confinement, with owner grants and
bounds. A refused selection starts no process and leaves the leaf live.
Correct the named policy, choice or Creator evidence and rerun Grove; no
inspection reservation or fallback selection participates.

<a id="execution-contract"></a>
<a id="supervision"></a>
<a id="group-cleanup"></a>
## Jobs and authority

Grove makes Ravel a foreground job through its local runner; Ravel makes the
selected process its own job. Each supervisor owns its direct child's terminal
handoff, restoration and group cleanup. Ravel's
[process decision](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/adr/the-launched-child-is-a-job.md)
owns its side; Grove's [local runner decision](../adr/the-launched-child-is-a-job.md)
owns the caller's side. Processes that leave a managed group are outside that
group-cleanup guarantee; an uncatchable supervisor death can orphan a process.

Grove scrubs enclosing session and provider-evidence authority before probes
and launches. Lifecycle launch admission is then supplied only to the foreground
session; standalone helpers inherit no tree authority. Ravel isolates policy
from completion controls and gives the managed process fresh enclosing context.
Neither inherited context nor same-user receipts authenticate hostile local code.

<a id="exit-signal"></a>
<a id="run-ending"></a>
## Ending and returned result

Grove's lifecycle results are exactly `grove-continue` and `grove-force-stop`. Ravel's
first publication wins and is returned without a newline after managed-process
cleanup; managed output and diagnostics are separate from result stdout. A
published argument may succeed even when the managed process later exits
nonzero. Natural exit without a result supplies no completion acknowledgement.
Ravel cancellation or supervision/cleanup failure yields no successful result.

Grove captures stdout in a held unlinked file and reads the supported value after
its own Ravel job is reaped. It neither reads an ending file nor looks up a run.
Result meanings belong to Grove. Continue advances ordinary work; force-stop
stops without completion and leaves remaining live work resumable. Ravel adds
no verb or interpretation rule. Standalone currently requires continuation to
publish outputs; force-stop cannot acknowledge standalone completion.

<a id="grove-integration"></a>
<a id="lifecycle-launch"></a>
## Lifecycle interpretation

After reap Grove restores the terminal and invalidates session epoch admission
before interpreting the launch. Failed invalidation reads no disposition and
launches no next session. Once that gate passes, the driver interprets in order:

1. Driver interruption stops the loop with its signal.
2. Successful Ravel status with exactly `grove-force-stop` stops without
   completion, even when teardown is recorded or a supervisor group survives.
   The CLI exits 1 and reports force-stop. It does not retire the live leaf.
3. This launch's explicit teardown record otherwise finishes the loop,
   irrespective of result, supervisor status or surviving group.
4. A surviving Ravel group stops the loop before continuation.
5. Successful Ravel status and exactly `grove-continue` advance to the next pick.
6. Anything else stops; rerunning Grove resumes the remaining tree.

Only successful supervisor status permits reading the result; a failed or
interrupted supervisor cannot override teardown with bytes on stdout. A valid
force-stop overrides the completion disposition but does not restore a tree
already torn down. Ordinary stops retain their existing CLI exit 0 behavior;
driver interruption is re-raised as its original signal.

A finish session records teardown with `grove-llm record-teardown` after the
confirmed finish cycle, before executing its prompt's ending command as its
last action. Missing `.grove/` alone attests no confirmed teardown.

**Original live v22 sessions keep their launch contract.** The extraction is
driven by installed Grove 22.1.0 and its original dispatch executable. Such
sessions still end with `harness-dispatch exit`, use the original admission and
Creator-run convention, and record teardown before that signal for finish.
Never substitute a new Ravel command into an already running session. The
[legacy-driver transition](../legacy-driver-transition.md) preserves that complete
installation through the final finish signal.

<a id="confinement"></a>
## Standalone interpretation

`grove run KIND` stages inputs and supplies output/completion instructions in
its prompt. Ravel evaluates policy outside confinement and confines only the
managed process. Explicit runtime files are read-only; policy/settings stay
outside managed reads. A missing sandbox refuses with no unconfined fallback.

Grove logs and streams sanitized managed output, checks cancellation, surviving
group and successful Ravel status, then accepts exactly `grove-continue` before
validating/publishing outputs through the held work-directory descriptor without
overwriting destinations. Refusal, absent/unsupported result or invalid outputs
publish nothing. [Standalone invocations](standalone-invocations.md) owns that
staging, output and transcript contract.

<a id="identity-and-creator"></a>
<a id="review-policy"></a>
<a id="records-and-outcomes"></a>
## Caller-owned creator evidence

Ravel has no durable store, run identity, record/observation verb or historical
lookup. Grove's opt-in provenance wrapper captures selected-provider evidence
in a canonical receipt. A finishing producer writes `**Creator:** recorded <JSON>`
from its own `GROVE_PROVIDER_EVIDENCE` directly under the review's `**Reviews:**`
line; missing evidence requires the owner's explicit `declared <provider>` form.
A retry replaces the reference only when it finishes the producer. A node's
creator is the child that closes it. Review retries read the committed reference.

The [provider evidence contract](../grove-provider-evidence.md) and
[Creator decision](../adr/a-review-carries-its-creator-reference.md) own receipt
validation, association and policy refusal. The owner decides provider diversity;
Ravel enforces no review-specific rule. Current assets refuse legacy `run <id>`
references without opening old history. Only original live-v22 sessions use that
form with their original policy. Existing dispatch history stays with the old tool.

<a id="diagnostics"></a>
## Refusals and remedies

Ravel's public CLI diagnostics and exit codes belong to its specification.
Grove reports them and stops on an unsupported/missing result; selection refusal
leaves the task live. An unrouted leaf is remedied at its owner policy, never by
an admission check in a tree verb. Grove's public loop outcome and process status
are distinct; [usage](../USAGE.md) shows refusal recovery and interruption.

<a id="delivery"></a>
## Delivery

Homebrew's Grove formula depends on independently versioned
`linkuistics/taps/ravel`. Grove archives contain its binaries and readable caller
assets; archive users install Ravel separately. Preserve each project's complete
prefix. No matching Cellar version, neighboring executable or bundled worker is
assumed. [Release instructions](../RELEASING.md) and the
[delivery boundary](../grove-delivery-contraction.md) own Grove's packaging.
Ravel owns its [runtime evidence](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/runtime-evidence.md)
and [published installation evidence](https://github.com/Linkuistics/Ravel/blob/3916a4a1d17fef925db7e9c730bd4c23d6df3ac8/docs/publication-evidence.md).

<a id="test-seams"></a>
## Caller acceptance

| Claim | Existing witness |
|---|---|
| Absolute PATH resolution/version refusal and independent prefixes | Resolver, activation and installed fixtures in `crates/grove/tests/` and `crates/grove/src/dispatch/ravel.rs`. |
| Literal public arguments and exact ending path | `crates/grove-loop/tests/session_command.rs` and real installed command fixtures. |
| Result capture, rejection and post-reap precedence | `crates/grove-loop/tests/session_result.rs`, installed result fixtures and loop-driver tests. |
| Lifecycle admission, cancellation and terminal recovery | Driver/epoch tests, installed lifecycle and local runner PTY tests. |
| Caller policy/context/provider evidence | `task ravel:review`, `task ravel:provenance` and strict installed-SDK typechecking. |
| Confined staging and publication | Standalone tests and installed fixtures using fake agents. |
| Independent delivery | Archive/formula/target acceptance in the delivery handoff. |

`task check` runs the repository gate, including all source books and plugin
conformance. Explicit installed fixture tasks cover named ignored cases;
[caller acceptance](../ravel-caller-surface-audit.md) and
[delivery acceptance](../grove-delivery-contraction.md) record their bounded
runs. Private worker/runtime controls are checked in Ravel, not duplicated here.

<a id="out-of-scope"></a>
## Boundary limits

There is no direct-command bypass, persistent Ravel history API or Grove policy
fallback. Grove does not add daemon, agent-server, dashboard or storage protocols.
Its task tree and methodology remain caller-owned ordinary artifacts.
