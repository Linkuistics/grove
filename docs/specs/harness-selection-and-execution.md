# Harness selection and execution

This is the first-release design of **harness-dispatch**. It specifies new
behavior; the command is not implemented yet. After design review, the
[artifact identity and original creator](#identity-and-creator) and
[supplied review policy](#review-policy) sections, with the Grove scope slot
they rely on, are being redesigned and do not yet bind implementation planning.
The other sections include the review's repairs.
The [visual document](../design/harness-selection-and-execution/README.md) has
matching package, execution and provenance views.

<a id="problem"></a>
## Problem

A caller knows the work and its session kind. Its owner knows which harness,
model and reasoning effort should perform it. Encoding both in Grove couples
selection experiments to the task-tree driver and prevents independent use.

The [policy ownership decision](../adr/harness-selection-is-owned-by-policy.md)
defines that boundary. The [worker and handoff decision](../adr/policy-evaluation-precedes-process-replacement.md)
chooses its runtime. This specification owns the interface contracts below.
The existing [complete-command](../adr/complete-session-configuration.md),
[open-kind](../adr/a-kind-is-an-open-token.md),
[personal-authority](../adr/untracked-configuration-delta.md) and
[foreground-job](../adr/the-launched-child-is-a-job.md) decisions continue to
govern Grove.

<a id="package-boundary"></a>
## Package boundary

The public executable and independently buildable Rust package are named
`harness-dispatch`. They depend on no Grove domain, task-tree or jj package.
The installation also supplies a private `harness-dispatch-policy` executable:
TypeScript compiled with Bun, including its runtime and SDK. Its versioned
protocol is private to the matching package release. Neither a system Bun nor
a system Node installation participates in policy evaluation.

| Owner | Responsibility hidden behind its interface |
|---|---|
| Rust front process | Input validation, selected-policy authority, worker lifecycle and bounds, candidate/result validation, native argv expansion, durable records, final exec |
| Compiled policy worker | Load the selected TypeScript entry, assemble context, evaluate a static table or asynchronous selector, return serializable evidence and a candidate ID |
| Owner policy | Candidate catalog, model/effort values, context requirements, preferences and any review rule |
| Optional Grove adapter and example | Read supplied task/brief data and interpret `Reviews`; translate it to generic artifact references |
| Grove | Select one task, compose the prompt, supply authoritative caller data, own the foreground job and completion channel |

The SDK, the Grove adapter and the examples are embedded in the worker and
reached through fixed package specifiers (see
[runtime discovery](#policy-authority)). Their type declarations and readable
sources ship beside the worker. The Grove adapter is an explicit import, versioned
and built with the worker, not a dependency of ordinary dispatch. It uses file
data; it does not require a Grove binary for selection. It never runs a second
pick. The package owns no task-tree mutation. The adapter ships with the package
so that it always matches the embedded SDK. It depends only on the public SDK and
Grove's documented task conventions, and the core never imports it, so an
extraction can move it to Grove's side instead.

<a id="command-interface"></a>
## Command interface

`inspect` and `run` accept the same selection inputs. Only `run` records a handoff
and replaces its process. `inspect` is a proposal, not a launch reservation or a
side-effect-free evaluation of trusted TypeScript. A later `run` evaluates afresh.

| Input | Contract |
|---|---|
| `--kind TEXT` | Required nonempty UTF-8 caller token; no enumeration or Grove filename grammar |
| `--prompt TEXT` or `--prompt-file PATH` | Exactly one for `run`; optional for `inspect`, which otherwise renders the prompt argument as a marked placeholder; read once; valid UTF-8 with no NUL; preserve bytes, including trailing newlines; never read terminal stdin |
| `--task-file PATH` | Optional source, resolved against the original cwd; does not supply kind or identity |
| `--scope ID` with `--task-id ID` | Both or neither; identifies this task/artifact independently of paths |
| `--context PATH` | Optional version-1 JSON document, read as data; explicit generic context can replace a task file |
| `--choice ID` | One configured joint candidate; visible to both context assembly and selection |
| `--config PATH` | Explicit policy entry replacing the personal default, with no implicit merge |
| `--policy-env NAME` | Repeatable exact-name grant to the worker beyond its base environment; values are never printed in inspection |
| `--timeout-ms N`, `--context-bytes N` | Explicit finite bound changes within the hard ceilings below |
| `--state-dir PATH` | Explicit location for this owner's records; default under the user's local state directory |

No task file or Grove installation is required. Without a task file or context,
an owner can still route solely by kind if its policy declares no other required
facts. A policy requiring more context refuses rather than inferring it from the
prompt. Generic context can identify a reviewed artifact using its scope and
stable ID without adopting any Grove review label.

`inspect --json` emits one versioned object on stdout. It includes the resolved
entry path and its authority, policy/adapter versions, candidate/provider/model/
effort, explicit-choice input, reason, context sources and hashes, measured UTF-8
byte totals, effective bounds, timing, provenance evidence/revisions, executable
resolution and expanded argv. Human output contains the same facts without
requiring a parser. `run` reserves stdout and stdin for the final harness; its
short choice/run-ID diagnostics go to stderr. Inspection includes the unchanged
prompt in argv when one is supplied. The prompt is never delivered to the policy
worker, so omitting it does not change the selection.

Data commands support `--json`. Schema version 1 is explicit in context,
inspection, record exports and observations; unknown versions and unknown
contract fields are refused with their location. Help includes independent,
Grove, refusal-recovery and observation examples. No pager, interactive
confirmation or automatic retry is part of this interface.

<a id="policy-and-choice"></a>
## Policy and joint choice

The selected ESM TypeScript module exports one named `policy` value. It contains
`schemaVersion` (1), a nonempty owner-maintained `version`, a candidate catalog,
an optional `loadContext(request, host)` callback, and exactly one of `routes`
or `select(request, context, host)`. `routes` is the useful static form: an exact
kind-to-candidate-ID table. `select` may be asynchronous and perform computation
or HTTP requests. There is no catch-all route, implicit route inheritance,
fallback candidate, automatic retry, or partial model/effort override.
Keeping exactly one form keeps one evaluation path. A policy that wants exact
routes for most kinds and computation for a few exports `select` and consults
its own table. Inspection then reports computed selection, and the policy's
reason names the table entry it applied.

The request fields are `schemaVersion`, `kind`, `cwd`, optional `taskFile`,
optional `task` (the `scope`/`id` pair), optional `context`, optional
`explicitChoice`, and effective `limits`. Caller context contains
`schemaVersion`, optional `summary`, `acceptanceCriteria` (string array), `facts`
(JSON object), `assessments` (attributed JSON object), `sources` (source records)
and `reviewedArtifact` (a scope/ID pair). Empty optional collections remain
distinct from unknown facts. Loaded context uses the same shape and additionally
carries measured source metadata and any creator snapshot. No executable fields
are admitted in a caller context document.

Each candidate has a unique ID, a nonempty provider-origin label, model and effort
strings, a program and an argument array. Provider/model/effort are catalog
values, never inferred from the executable or its arguments. The program is a
literal absolute path or PATH name; relative path programs containing a separator
resolve against the caller's cwd and inspection reports that resolution.
Arguments are literals or explicit slot objects. Slots are `prompt`, `kind`,
`taskFile`, `taskId`, `scope`, `model`, `effort` and `runId`; `prompt` occurs
exactly once, and an absent optional input cannot satisfy a used slot. A slot
occupies one whole argument. No shell splitting, interpolation inside literals,
shell evaluation or second interpretation of prompt text occurs. Model and effort
need not appear as slots if an owner's wrapper encodes them; catalog identity is
an owner assertion, not verified backend identity.

The selection result has `status: selected`, `candidateId` and a nonblank
`reason`, or `status: refused`, `code`, `message` and `remedy`. The worker
returns the catalog snapshot with the result; Rust validates both before use.
A result cannot supply new executable words. Invalid exports, unknown candidate
IDs, malformed arguments, exceptions, an unresolved promise or abstention refuse.
All catalog shapes and static references are checked, but only the selected
executable is checked for availability. An unavailable alternative cannot cause
selection to silently switch to it or away from it.

With `--choice`, a static `routes` policy accepts any configured candidate the
choice names, including for a kind its table does not route, and cannot refuse
it. Inspection reports that the explicit choice, not a route, selected it. An
owner who wants to constrain explicit choices uses `select`, which must
explicitly accept or refuse the choice. Returning any other ID is
`explicit_choice_mismatch`, even if the policy describes it as a fallback.
Policy constraints apply to explicit choices as to normal selections. Each retry
is a separate invocation, reevaluates policy and receives a new run identity.

The supplied static examples give exact mappings and explain effort in terms of
abstraction, uncertainty, consequences, downstream repair, reversibility and
available checks. They are editable starting policy, not model rankings or
calibrated estimates. The dynamic example is a deterministic callback suitable
for a fake-harness test; no local inference service is required.

<a id="policy-authority"></a>
## Policy authority and runtime discovery

The personal default is `~/.config/harness-dispatch/policy.ts`. Missing, unreadable
or invalid selected policy stops the invocation and names the selected path.
There is no cwd search, repository override, environment-selected policy entry
or installation of policy into the user's configuration. A `--config` path is
resolved against the caller's original cwd; inspection identifies that explicit
authority. Personal policy can explicitly import a repository entry, in which
case that import is the owner's choice. Imported trusted code inherits that
authority; this is not a sandbox against its owner.

The Rust process locates its worker from the installation, never PATH or cwd,
and verifies the worker protocol/build identity before evaluating policy. It
starts the worker in a private empty directory, using null stdin, captured
diagnostic streams and a private framed protocol channel. The request passes
the caller's cwd as data; it does not make it the worker's runtime cwd.

The worker is compiled with dotenv, bunfig, tsconfig and package-json autoloading
all explicitly disabled. Its own entry imports embedded modules and prefixed
built-ins. Before importing the selected entry, it registers each documented
package specifier as a runtime virtual module backed by its embedded copy:
`harness-dispatch/sdk`, `harness-dispatch/grove`, and one
`harness-dispatch/examples/…` specifier per shipped example. Owner policy
imports those names. There is no installed path to name and no `node_modules`
for the owner to maintain. The SDK and adapter a policy receives are always the
running worker's own, so an upgrade cannot mix versions. A registered specifier
resolves to its embedded module even when a `node_modules/harness-dispatch`
package sits beside the importing entry. The prefix itself reserves nothing: an
unregistered name under it resolves like any other bare specifier. So the
registered list is part of the worker's versioned protocol, and every documented
specifier is on it.

Other external imports start at the selected absolute entry. Bare specifiers
resolve through `node_modules` from the importing module's directory, and
relative imports from the importing module, never from the invocation directory.
No automatic package installation runs. Missing imports fail. Because tsconfig
autoloading is off, `paths` aliases in a tsconfig beside owner policy do not
apply at run time. The shipped declarations serve editor and package type
checking only. Trusted policy may deliberately use normal module imports or
native Bun APIs; those actions are policy effects rather than implicit host
discovery.

Rust constructs a fresh worker environment containing only HOME, a PATH snapshot,
TMPDIR, LANG and LC_* values, plus exact `--policy-env` grants. It always excludes
BUN_*, NODE_OPTIONS, NODE_PATH, dynamic-loader injection variables and its private
protocol variables from grants. Installation control variables never come from
ambient input. The generic command cannot know a caller's completion variables,
so it cannot refuse to grant them. Naming one with `--policy-env` is the owner
explicitly giving the worker that authority. Grove's documented configurations
grant none, and the usage documentation warns against granting
`GROVE_SIGNAL_FILE`. Other credentials needed by policy require an explicit named
grant. The final harness receives the original environment, including Grove's
fresh completion channel, plus `HARNESS_DISPATCH_RUN_ID` and
`HARNESS_DISPATCH_STATE_DIR`; these two reserved values replace inherited stale
values. Worker scrubbing does not rewrite final harness environment policy.

<a id="bounded-context"></a>
## Bounded context

The worker sees a versioned request with kind, original cwd, optional task file,
stable identity, generic context and explicit candidate ID. It sees neither the
launch prompt nor reserved final environment. The context value separates caller
facts, loaded sources, selector assessments and artifact/creator references.
Unknown risk, scope or outcomes remain unknown. Policy states which facts it
requires; a failed required read or loader fails the whole selection.

The SDK supplies bounded text/JSON reads, source attribution, creator lookup,
diagnostic output and an abort signal. It measures every delivered source, hashes
the bytes actually read, and includes the adapter's explicit version. A context
loader returns the final serializable context; both worker and Rust validate its
size. Reads of arbitrary files or HTTP by trusted policy remain possible, but
unreported ambient reads are not advertised as reproducible context. Such a
policy must supply versions/digests for the evidence it puts into the context.
The receipt records the entry digest, declared policy version and worker build;
it does not claim to hash every dynamic dependency or every remote response.

SDK operations are `readText(path, maxBytes)`, `readJson(path, maxBytes)`,
`originalCreator(scope, artifactId)`, `diagnostic(text)` and `signal`.
Reads resolve explicit relative paths against request cwd and return content
plus canonical source name, byte count and SHA-256. Policy imports use their own
module-relative resolution. Creator lookup is a read-only protocol request to
the Rust store and returns either an attributed snapshot/revision or missing;
the worker never opens or writes the database. The loader returns context,
and the selection callback receives that measured value. The supplied adapter
uses these operations so its complete delivered context is inspectable.

| Resource | Default | Hard ceiling and behavior |
|---|---|---|
| Whole selection, from worker start to its result, including imports, context and callback | 30 seconds | Caller can choose 1–120 seconds; timeout refuses |
| Context delivered to selection, including caller JSON and source metadata | 256 KiB UTF-8 JSON | Caller can choose up to 8 MiB; overflow refuses, never silently truncates |
| One SDK source read | 64 KiB | At most the effective context budget; oversize source refuses |
| Number of context sources | 256 | Fixed; excess refuses |
| Policy result/catalog protocol message | 1 MiB | Fixed; excess refuses |
| Worker diagnostics, both streams together | 256 KiB | Drain within the bound; excess terminates evaluation with an output-limit error |
| Prompt | 1 MiB | Fixed; platform argv/environment limits can refuse smaller payloads at exec |
| Record-store lock wait | 2 seconds | Fixed; separate from the selection bound, which it neither extends nor consumes |

Time before handoff is therefore bounded by the selection bound, the worker's
cleanup grace and the lock wait, plus executable resolution and the record
commit. The raw prompt has its own budget and is never truncated to satisfy a
context limit. JSON size means encoded UTF-8 bytes, not characters or token
estimates.
Inspection reports actual source bytes and final encoded bytes separately, plus
limits and errors. Long-lived caches, token estimators and model calls are not
required. Policy cannot raise its own hard ceilings after evaluation begins.

<a id="execution-contract"></a>
## Execution and authority

The front process preserves its caller's cwd, descriptors, process group and
terminal ownership. Under Grove it is already the foreground job leader. The
worker joins that existing job; neither process creates a session or detaches.
The SDK's first release needs no subprocess API: file context and network
computation run in the worker. Trusted policies that create their own children
must keep them in the enclosing job, grant no interactive stdin, and reap them
before returning; detached/background policy services are outside the contract.
The supplied policy and adapter create no such children.

Rust handles INT, TERM and HUP while evaluating, unless a signal was already
ignored at entry. It stops the worker on cancellation and reaps it before
continuing or returning. A hard wall-clock deadline kills
a stuck worker even during module import or a synchronous infinite loop. Its
cleanup grace is at most one second, then KILL. Signal cancellation reaches
ordinary descendants through the enclosing process group. Abrupt KILL may leave
no terminal record; this is not a promise of cleanup against deliberately
detached owner code.

After a result arrives, Rust closes protocol descriptors, removes temporary
material, reaps the worker, validates the explicit choice, resolves the selected
executable, and commits the required handoff record. It checks cancellation at
each boundary. It then blocks the handled signals and checks for pending
cancellation once more. Cancellation observed after the commit launches nothing.
Rust appends a best-effort not-executed detail to the attempt and ends by
re-raising the signal. A failed append leaves an attempt of unknown execution,
never a success. Otherwise Rust restores the entry signal state and execs.

That final check is the linearization point. Cancellation observed before it
launches nothing, and a signal arriving after it is a signal to the admitted
foreground job, including across exec. A signal delivered between restoring the
entry mask and exec can end the process with the attempt recorded and its
execution unknown. No userspace design promises an atomic test-and-exec against
a later-arriving signal.

The handoff is transparent to signal state. The harness receives the signal mask,
and every disposition that survives exec, exactly as the front process inherited
them at entry, for every signal including SIGPIPE. A signal ignored at entry gets
no evaluation handler and stays ignored: a `nohup` caller's ignored HUP reaches
the harness ignored, and cannot cancel selection. Rust's standard runtime
ignores SIGPIPE before `main`, and its `exec` path resets SIGPIPE to default
before running pre-exec hooks while keeping the calling thread's mask. The
implementation must therefore record the entry SIGPIPE disposition before any
runtime initialization changes it, and reinstate it after that reset. The
command and controlling-PTY seams observe the result.

Exec preserves the process identity Grove supervises, terminal, cwd and native
exit/signal behavior. There is no Rust supervisor left to infer an exit code,
duration, usage or acceptance. If exec returns with an error, record an observable
launch failure when possible, report the errno with remediation and exit nonzero.
Failure to append that failure detail does not erase the existing attempted
handoff or convert it to success. A configured wrapper must in turn exec the
harness it fronts. Dispatch fixes one joint choice; later model changes inside a
harness are outside its observation and enforcement.

<a id="identity-and-creator"></a>
## Artifact identity and original creator

Generic identity is the pair **scope ID + artifact ID**. A run may be associated
with an output artifact, a task, a reviewed artifact, or no artifact. Paths are
descriptive source locations only. Scope and artifact IDs are nonempty opaque
strings with length limits (256 and 1024 UTF-8 bytes); IDs round-trip through
inspection and record commands. Independent callers allocate a fresh scope for
each unrelated workstream; `harness-dispatch scope new` returns a random UUID.

Grove supplies its selected handle as artifact ID and an opaque dispatch scope
through a new optional runtime slot. The dispatch scope is an evidence namespace,
not a session epoch or tree-mutation authority. Grove stores its UUID/binding in
the existing workspace control area, outside the task tree, lazily when a route
uses that slot. It reuses the binding across sessions and driver restarts in the
same live tree. Authoritative root creation invalidates any old binding; the next
use allocates a fresh scope. Finish retires the binding without deleting records.
The binding includes the observed workspace and task-root directory identities;
unexpected replacement refuses delegated launch with a scope-reset remedy.
Plain direct-harness routes do not allocate dispatch state. A manual/offline
replacement or restoration of a root requires explicit scope reset; device/inode
numbers alone are not proof against offline inode reuse. Cross-checkout discovery
and post-teardown artifact lookup are not promised. All retained records still
carry their IDs and can receive later observations by run ID.

`grove dispatch-scope show --json` reports the current binding without allocating
one. `grove dispatch-scope reset --expected UUID` explicitly replaces it with a
fresh UUID after a manual root replacement; it requires an existing readable
tree and no live driver, checks the expected binding and preserves old records.
Neither command imports or evaluates selector policy. This is bookkeeping for
caller identity, not an override of the selected harness or session kind.

An original creator is registered once for that exact artifact identity:

- `creator bind --scope S --artifact A --run R --evidence TEXT` registers a
  recorded run only if R belongs to S/A and an external observation confirms its
  execution and production of A. It snapshots that run's provider/model/effort;
  it never reads today's catalog to reconstruct yesterday's provider.
- `creator declare --scope S --artifact A --provider P --evidence TEXT` supplies
  the owner declaration needed for pre-adoption or direct-harness artifacts. It
  is labelled `declared`, with owner-supplied evidence, not `execution_recorded`.

**The first explicit creator registration wins.** Neither a new launch nor a new
observation overwrites it. Where a producer was invoked several times, the owner
registers the run that originally produced the reviewed artifact; the tool never
guesses from earliest attempt, latest success, current mapping or task retirement.
An attempted handoff alone cannot be bound as execution-recorded. Registration
with identical content is idempotent; a conflicting registration refuses and
names the existing association. Correction is explicit, using an expected
association revision and a reason; the prior assertion remains in record history.
This is one original creator, not contributor accounting.

Both registration commands accept `--expected-revision REV --reason TEXT` for
that explicit correction; without both, replacing an existing different
association is refused. `creator show --scope S --artifact A --json` reports the
exact snapshot and revision used by policy, including its evidence class.

The extra registration is deliberate: preserving direct exec means the tool
cannot honestly infer which retry produced an artifact. An automation with real
execution evidence may import the observation and register the creator; a human
may use the declared-provider remedy. The supplied review policy's refusal
explains both routes rather than converting an attempt into success.

<a id="review-policy"></a>
## Supplied review policy

The inactive example uses exact configured review-kind entries, all of which
apply the provider rule; other kinds use the owner's static table. It exports
its rule as a reusable selector so non-Grove callers can give a generic reviewed
artifact reference. Custom review labels require an explicit example-policy
entry. The core knows no review list or `Reviews` grammar.

The Grove adapter reads the supplied task and its ancestor briefs under the
supplied root. For a configured review entry it requires exactly one standalone
`**Reviews:** <stable-handle>` declaration, resolves that handle to exactly one
task header within the bounded tree read, and constructs a reviewed-artifact
reference in the supplied scope. It does not obtain kind/identity from the
filename, infer associations from a shared slug, select another leaf, or use
advisory running-session state. Missing, malformed or ambiguous declarations,
unreadable sources, and traversal limits refuse. Retired/renumbered task paths
continue to resolve through their stable headers. The adapter has independent
fixtures for the Grove conventions it interprets.

The policy obtains the exact original-creator record through the SDK. Missing
registration stops review with the bind/declaration remedy. It validates the
requested or mapped candidate against that provider, requiring a different
provider origin; a gateway label change cannot establish separation. It does not
select a replacement on failure. The same check runs on every invocation and on
explicit choices. Provenance kind, evidence and association revision appear in
inspection and the run record. Direct-harness routes do not execute this policy.

<a id="records-and-outcomes"></a>
## Records and later observations

The front process owns a versioned local SQLite store with bundled SQLite,
private user permissions and durable transactions. No database service or Bun
database client is required. Its default directory is
`~/.local/state/harness-dispatch`; an explicit state directory replaces it.
Concurrent invocations use short transactions, never a lock held across policy
evaluation. Disk-full, permission, schema or lock failures before handoff refuse.

Before exec, one committed transaction persists a collision-resistant run ID,
timestamp, scope/task/artifact associations, kind, selected catalog values,
explicit-choice input, resolved executable and argv, original cwd, policy entry
authority/digest/version, worker and adapter versions, context source digests and
sizes, effective limits, decision reason, selection timing and provenance used.
Raw environment values are not stored. Argv contains the prompt, so records are
private local execution data. The run ID is available to the final harness via
its environment and the optional argument slot. Inspection uses a visibly marked
proposed ID, creates no run or creator association, and does not promise that ID
will be reused.

Creator lookup returns a revision. The pre-handoff transaction checks every
creator revision used in selection; a concurrent correction causes an explicit
stale-provenance refusal, not a hidden selection retry. Readable but corrupt
records never become missing-provider defaults.

| Evidence | Meaning |
|---|---|
| Proposal | Result of inspection; no persisted launch claim |
| Handoff attempt | Durable intent immediately preceding exec; execution is unknown |
| Observable launch failure | Exec returned an error, or cancellation was observed after the attempt committed; appended to that attempt when possible |
| Execution confirmation | Later evidence from an external observer/final harness says execution occurred |
| Outcome observation | Separately sourced acceptance, findings, repair, human work, duration, usage or other measurements |

There is no implied state transition from exit zero to task accepted. Interrupted
or abruptly killed attempts may remain unknown forever. Evidence describes the
configured launched choice; it is never a claim to have authenticated the actual
backend model. A refusal before the handoff commit is a structured diagnostic
only: it creates no run and no record. Only a committed attempt can carry a
launch-failure detail.

`record show --run R --json` exports the run and its observations.
`record observe --run R --file observation.json` validates and atomically appends
one version-1 observation. It requires a caller-generated observation ID, source,
observed-at timestamp and evidence description. Repeating identical ID/content
is idempotent; conflicting content refuses. An import that corrects an earlier
observation names the observation it replaces, retaining both. No import can
change the immutable launch fields or silently change creator registration.

Supported observation fields include execution confirmation and artifact
production, exit/signal, duration, input/output/total usage with units, acceptance
(accepted/rejected/unknown), missed defects, false findings, downstream repair,
human-work measures and evidence links. Every field has an explicit
observed/unknown/unobserved distinction; absence does not mean zero or false.
Findings can have stable IDs and references to later repair observations.
Optional routing probabilities distinguish `choiceProbability` from
`successProbability`; a success estimate must name its calibration data/version
or be labelled uncalibrated. The first shipped policies emit neither.
Analytics, scoring aggregation and a learned-selector benchmark are follow-up.

Concretely, an observation envelope has `schemaVersion`, `observationId`, `runId`,
`source`, `observedAt`, `evidence`, optional `supersedes` and a `measurements`
object. Every measurement is an object with `state` equal to `observed`, `unknown`
or `unobserved`; `observed` requires a typed `value`, quantitative values require
`unit`, and other states carry no numeric value. Fields not supplied in an import
remain unobserved in the exported view. Creator binding requires an observed
`executionConfirmed` value of true and an observed `producedArtifact` value
matching the run's exact scope/ID. Observation source/evidence is an assertion by
the importer; the package validates shape and association, not external truth.

<a id="grove-integration"></a>
## Grove integration

Add optional whole-argument slots `kind`, `task_file`, `task_id` and `task_scope`
to lifecycle configuration. Populate them from the same authoritative selected
task used to compose the mandate: open kind token, absolute selected task path,
stable handle and evidence namespace. The existing prompt is passed unchanged.
Existing slots and direct-harness commands keep their current rules; prompt
remains exactly once. No task-body metadata, new Grove invocation override, or
prompt/filename scraping is introduced.

Grove configuration inspection shows these slots symbolically when no task is
selected. It does not fabricate a task, evaluate policy or allocate a scope.
Standalone `grove run` continues to offer its existing vocabulary. Lifecycle-only
slots are rejected in standalone templates that request them; the common
configuration machinery remains consumer-vocabulary-driven.

The owner points a personal command definition at `harness-dispatch run`, supplies
the new slots and prompt, and explicitly selects the shipped Grove example by
importing its package specifier from personal policy. A literal `--choice` may
be added to that command. No install overwrites personal configuration.
`grove config show` explains the configured wrapper; `harness-dispatch inspect`
explains its selection. Grove's pre-authoring
guarantee stops at the complete configured command. Static and computed delegated
policy are checked only at launch, so task authoring can succeed and delegated
launch subsequently refuse. Usage and configure-grove must explain both surfaces
and the exact remedy for incomplete mappings and missing creator registration.

<a id="diagnostics"></a>
## Diagnostics and exits

Before handoff, errors carry a stable code, stage, message, relevant input/source
and remedy. `--json` data-command failures use one JSON error on stderr and no
partial stdout object; policy diagnostics are captured and included separately,
never interleaved with protocol or structured output. Text mode prefixes policy
diagnostics on stderr. A failure never launches another candidate. A refused
`run` also reports the equivalent `inspect` invocation, as a command line in text
mode and an argv array in JSON: the same selection inputs, `--policy-env` names
but no values, and no prompt. An owner diagnosing an unattended refusal can then
reproduce the selection without reconstructing its inputs.

Exit codes before exec are 2 for malformed CLI input, 3 for policy/context/
selection refusal, 4 for required-record failure, 5 for worker/protocol/internal
failure, 124 for timeout, 126 for an unexecutable selected program, and 127 for
one not found. INT/TERM/HUP cleanup ends by restoring and re-raising that signal.
After exec, the harness's native exit or signal is unmodified; its code may
numerically coincide with a preflight code. Structured diagnostics and recorded
stage distinguish these cases, not a globally reserved harness exit range.

<a id="delivery"></a>
## Delivery and release

Use Bun 1.4.2 initially, pinned together with worker dependencies and build
identity; upgrades rerun the discovery probes. Compile workers for macOS arm64,
Linux arm64 and Linux x64. Bun 1.4.2 ships a single x64 build targeting the
Nehalem microarchitecture; its `baseline` name is an alias. Rust and bundled
SQLite build for Grove's corresponding targets with its existing glibc 2.17
floor. The worker is an
installed private companion, not a runtime downloaded on invocation. It is found
relative to the real installed front executable, including through a Homebrew
symlink; the supported portable archive preserves the same relative layout.
The Rust package builds independently; running selection additionally needs the
matching compiled worker, supplied by the package's build/install task.

Each existing archive gains the front executable, the private worker with its
embedded SDK, Grove adapter and inactive examples, their type declarations and
readable sources, and runtime/license notices. The Homebrew formula installs
that layout and checks matching versions. Source-development tasks build and
install the pair; a plain cargo install of the Rust package alone is not the
documented complete installation. No host runtime is silently used as a fallback.
The new Rust member inherits the workspace version and opts out of a second
cargo-release cut; the worker embeds that same package version.

Reusable package check/build/install/smoke tasks join the existing Taskfile, and
the repository check includes package checks. Archive-content assertions and
installed-layout smoke tests run the static and computed TypeScript cases with
fake harnesses and no separately installed runtime on **each** supported target,
natively or under emulation. A matching runtime archive is insufficient evidence
until that test executes. These are implementation/release acceptance
requirements, not results of this design.

The Linux floor has three dimensions, and each is claimed only as far as its
instrument observes it:

| Dimension | Floor | Instrument |
|---|---|---|
| C library | glibc 2.17, Grove's existing floor | Installed smoke tests run in a glibc-2.17 userland container on each Linux target |
| CPU | x64 Nehalem; arm64 at the Cortex-A53 level | The same tests under user-mode emulation with that CPU model |
| Kernel | Bun's documented support | None: a container or user-mode emulation runs on the host's kernel |

Bun 1.4.2's documents disagree about the kernel. Its README gives a 5.1 minimum,
while its installation page says Bun runs on 3.10 (RHEL 7) with degraded newer
syscalls. The release states that documented range, labelled documented rather
than executed, and each Bun upgrade rechecks it. This is an accepted trade-off:
RHEL 7-era kernels are claimed through Bun's documentation, not tested. A
cross-link or an ELF-symbol inspection alone satisfies none of the dimensions.
On macOS the worker additionally inherits Bun's documented macOS 13.0 minimum.

<a id="test-seams"></a>
## Agreed test seams and acceptance

The human agreed two process seams during requirements. Keep them as the public
acceptance instruments; internal tests may support them without replacing them.

| Seam | Required observable cases |
|---|---|
| New command, temporary policies and fake harnesses | Independent kind/context use with no Grove files or binary; optional task; static and computed selection; complete inspection including measured sources and authority; literal punctuation/newlines; a caller-ignored HUP or SIGPIPE and the entry signal mask reach the fake harness unchanged; policy errors, bad imports, missing context, limits, unavailable program and explicit-choice mismatch launch nothing |
| Same command, actual shipped examples | Different-origin reviewer on every invocation and explicit choice; same-origin/gateway disguise refuses; missing/ambiguous review relation refuses; declaration adoption; recorded creator requires execution/production evidence; retries preserve creator; changed current mapping cannot rewrite provider history |
| Same command, authority and lifecycle fixtures | Hostile cwd policy, dotenv, bunfig/preload, tsconfig, package shadow and BUN_OPTIONS stay inert through the public launcher, each beside its firing configuration below; explicit relative config and personal import are admitted; a documented package specifier resolves to the embedded module; worker and nested normal child environments lack caller completion values; structured diagnostics stay clean; import/loader/callback interruption and timeout launch nothing |
| Same command, records and observations | Required commit failure prevents exec; attempted handoff and exec failure stay distinct; cancellation after the commit launches nothing and marks the attempt not executed; pre-commit refusals create no run; unknown outcomes; round-trip run lookup and observation import, idempotency/conflicts/correction, creator registration and concurrent revision refusal; later observations after tree teardown |
| Existing Grove launch boundary | Original prompt and authoritative slots preserved as native data; task rename/retirement/reorder retains association; driver restart retains scope; root creation rotates scope; unexpected root replacement refuses; direct-harness compatibility; task authoring succeeds with a valid wrapper but bad delegated policy refuses at launch |
| Existing Grove launch boundary, controlling PTY | Final harness retains PID/group, cwd, terminal and native exits; the entry signal mask and dispositions, including SIGPIPE, reach it unchanged; helper receives null stdin and scrubbed control environment; final harness receives fresh channel; signal cancellation during selection and execution, plus descendant escalation |

Per-target release delivery adds the archive/install tests above. Documentation
review verifies activation, both inspection surfaces, scope setup/reset, creator
registration/declaration, later outcome entry and launch-time validation guidance.
Each hostile class has a named firing configuration, a positive control that
must be seen to fire, so a test cannot pass merely because its fixture never ran:

| Class | Firing configuration |
|---|---|
| cwd policy entry | The same file named by an explicit relative `--config`, which loads it |
| cwd `.env` and bunfig preload | A default-autoload probe build of the same worker, run in the hostile directory |
| `BUN_OPTIONS` preload | The shipped worker launched directly with the variable set, bypassing the front process's scrubbing |
| `node_modules/harness-dispatch` shadow | The fixture beside an admitted entry, under a probe build that does not register the virtual modules |
| tsconfig `paths` | The fixture beside an admitted entry, under a probe build with tsconfig and package.json autoloading enabled |

A class with no known firing configuration is reported as such, not counted as a
passing control. Missing-source fixtures carry the same obligation. The runtime
evidence records which of these have been seen to fire. Do not infer backend
identity, policy quality or task acceptance from these mechanics tests.

<a id="out-of-scope"></a>
## Out of scope

Local LLM selection and its evaluation pilot, model comparisons/calibration,
repository extraction, accumulated contributor exclusions, automatic fallbacks,
partial overrides, automatic retries, automatic creator guessing, cross-checkout
discovery, post-teardown artifact lookup and dedicated confined `grove run`
integration are not part of this increment. Windows is outside Grove's current
release targets. A hostile-code sandbox and supervision after final exec are
also outside this command's contract.
