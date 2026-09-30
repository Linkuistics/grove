# Harness selection and execution

This is the first-release design of **harness-dispatch**. Most of it specifies
behavior that is not implemented yet. Of the command itself, `inspect` and
`run` of a static `routes` policy and of a computed `select` are delivered:
policy authority, the compiled worker with its identity check, embedded SDK,
the two static starter examples and the dynamic one, policy validation, the
`select` result contract with its distinct refusals and the policy's own,
explicit-choice policing under `select`, the request, the prompt, task file,
task identity, explicit choice, `--context` document and state directory
inputs, `loadContext` with the SDK's measured reads, diagnostics and abort
signal, the measured delivered context, every [resource bound](#bounded-context)
with its refusal, argument-slot expansion including `runId`, program
resolution, the whole-selection deadline with its exit 124, the human and
version-1 JSON reports with their context sources, digests, sizes and bounds,
and structured refusals with their exit results. A refused `run` names its
equivalent `inspect` invocation. `run` commits its required handoff record,
with any reviewed artifact and the context's source digests and sizes, before a
plain exec, exports the run's identity to the harness, and appends an exec
failure to its attempt. `record show` exports a recorded run. Run lookup
(`host.run`) is refused by name. Handled signals and a signal-transparent
handoff are not yet delivered. Every release
archive and the Homebrew formula carry the front and its worker in the
[delivered layout](#delivery), each target's worker compiled from a
digest-pinned Bun runtime. The installed smoke test runs the static and
computed TypeScript cases from the extracted archive on macOS arm64 natively,
and on each Linux target in a glibc-2.17 userland under a pinned user-mode QEMU
emulating its CPU floor, Nehalem or the Cortex-A53; Linux arm64 also runs as a
native container. Positive controls show that the userland enforces the glibc
floor and that each CPU model refuses an instruction beyond it. The release
task runs the archive-content assertions and this smoke test before it
publishes anything.
Every other input and command is refused by name. Of the [Grove integration](#grove-integration), only the lifecycle
`kind`, `task_file` and `task_id` slots, their standalone refusal and their
symbolic inspection are delivered.
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
protocol is private to the matching package release. The worker's first message
states its protocol, its package version and a digest of the source it was
compiled from. The front refuses any other identity with exit 5, before the
worker learns which entry to evaluate. Neither a system Bun nor
a system Node installation participates in policy evaluation.

| Owner | Responsibility hidden behind its interface |
|---|---|
| Rust front process | Input validation, selected-policy authority, worker lifecycle and bounds, candidate/result validation, native argv expansion, durable records, final exec |
| Compiled policy worker | Load the selected TypeScript entry, assemble context, evaluate a static table or asynchronous selector, return serializable evidence and a candidate ID |
| Owner policy | Candidate catalog, model/effort values, context requirements, preferences and any review rule |
| Optional Grove adapter and example | Read the supplied task file's `Reviews` and `Creator` lines; translate them to a generic reviewed-artifact reference |
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
| `--prompt TEXT` or `--prompt-file PATH` | Exactly one for `run`; optional for `inspect`, which otherwise renders the prompt argument as a marked placeholder; read once; valid UTF-8 with no NUL; preserve bytes, including trailing newlines; never read terminal stdin, and refuse a prompt file that is a terminal |
| `--task-file PATH` | Optional source, resolved against the original cwd; does not supply kind or identity |
| `--task-id ID` | Optional task identity, independent of paths: an opaque nonempty UTF-8 string of at most 1024 bytes |
| `--context PATH` | Optional version-1 JSON document, read as data; explicit generic context can replace a task file |
| `--choice ID` | One configured joint candidate; visible to both context assembly and selection |
| `--config PATH` | Explicit policy entry replacing the personal default, with no implicit merge |
| `--policy-env NAME` | Repeatable exact-name grant to the worker beyond its base environment; values are never printed in inspection |
| `--timeout-ms N`, `--context-bytes N` | Explicit finite bound changes within the hard ceilings below |
| `--state-dir PATH` | Explicit location for this owner's records; default under the user's local state directory |

No task file or Grove installation is required. Without a task file or context,
an owner can still route solely by kind if its policy declares no other required
facts. A policy requiring more context refuses rather than inferring it from the
prompt. Generic context can identify a reviewed artifact and its creator's run
or declared provider without adopting any Grove review label.

`inspect --json` emits one versioned object on stdout. It includes the resolved
entry path and its authority, policy/adapter versions, candidate/provider/model/
effort, explicit-choice input, reason, context sources and hashes, measured UTF-8
byte totals, effective bounds, timing, the creator provenance used, executable
resolution and expanded argv. Human output contains the same facts without
requiring a parser. `run` reserves stdout and stdin for the final harness; its
short choice/run-ID diagnostics go to stderr, each as one JSON line under
`--json`. Inspection includes the unchanged
prompt in argv when one is supplied. The prompt is never delivered to the policy
worker, so omitting it does not change the selection.

Data commands support `--json`. Schema version 1 is explicit in context,
inspection, record exports and observations; unknown versions and unknown
contract fields are refused with their location. Help includes independent,
Grove, refusal-recovery and observation examples. No pager, interactive
confirmation or automatic retry is part of this interface.

<a id="policy-and-choice"></a>
## Policy and joint choice

The selected ESM TypeScript module exports one named `policy` value, a plain
object. It contains
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
optional `taskId`, optional `context`, optional `explicitChoice`, and effective
`limits`: `selectionMs`, `contextBytes`, `sourceBytes`, `sources`,
`messageBytes` and `diagnosticsBytes`. The policy receives the request frozen.
Caller context contains `schemaVersion`, optional `summary`,
`acceptanceCriteria` (string array), `facts` (JSON object), `assessments`
(attributed JSON object), `sources` (source records) and `reviewedArtifact`.
Each assessment is `{ by, value }`: a nonblank assessor and any JSON value. A
source record is `{ name, sha256?, bytes?, version? }`, with a nonblank name and
a digest (64 lowercase hexadecimal digits), a version or both, so that it pins
its evidence; a context holds at most 256.
A reviewed artifact has an `id` and an optional `creator` holding exactly one
of `run` (a canonical run ID) or `declared` (a provider-origin label). Empty
optional collections remain distinct from unknown facts, and nothing absent is
defaulted. Loaded context uses the same
shape. The delivered context additionally carries measured source metadata,
which harness-dispatch attaches as `measured` and a loader cannot supply, and,
once run lookup is delivered, the creator run snapshot it resolved. No
executable fields are admitted in a caller context document: an unknown field
refuses with its location, and one named like an executable field (`program`,
`args`, `catalog`, `select`, `loadContext` and the like) says why. `facts` and
assessment values are data, never checked for such names, because nothing in a
context can become an argument.

Each candidate has a unique ID, a nonempty provider-origin label, nonempty model
and effort strings, a program and an argument array. Provider/model/effort are catalog
values, never inferred from the executable or its arguments. The program is a
literal absolute path or PATH name; relative path programs containing a separator
resolve against the caller's cwd and inspection reports that resolution. A PATH
name is looked up as `execvp` does, in the caller's PATH: an empty or relative
entry is relative to the cwd, and the first executable regular file wins. A PATH
that is set but empty is one empty entry. An unset PATH leaves nothing to search,
so the name is not found. Catalog strings that can become argv words cannot
contain NUL, since exec cannot carry one: the program, `model`, `effort` and
every literal argument. A NUL there is invalid policy in any candidate,
refused at validation before anything is proposed or recorded.
Resolution happens once, and `run` executes the resolved file with the program
as configured for `argv[0]`. Inspection resolves the selected program as `run`
does and refuses with the same exit when it cannot.
Arguments are literal strings or slot objects of the form `{ slot: "<name>" }`.
Slots are `prompt`, `kind`,
`taskFile`, `taskId`, `model`, `effort` and `runId`; `prompt` occurs
exactly once, and an absent optional input cannot satisfy a used slot. A slot
occupies one whole argument. No shell splitting, interpolation inside literals,
shell evaluation or second interpretation of prompt text occurs. Model and effort
need not appear as slots if an owner's wrapper encodes them; catalog identity is
an owner assertion, not verified backend identity.

The selection result has `status: selected`, `candidateId` and a nonblank
`reason`, or `status: refused`, `code`, `message` and `remedy`. The worker
returns the catalog snapshot taken at import, and Rust validates it, and any
explicit choice against it, before any `loadContext` or `select` runs. The
context follows when there is a loader or a caller context, and Rust validates
and measures it. The result comes last, and Rust validates it against that
snapshot before use. A result cannot supply new executable words, and a field beyond
its status's own is refused. Invalid exports, unknown candidate IDs, malformed
arguments, exceptions, an unresolved promise or abstention refuse, each with
its own code. A policy's own refusal is reported under the stable
code `policy_refused`, with the policy's code beside it, its message and its
remedy, so an owner's codes never collide with the command's.
All catalog shapes and static references are checked, but only the selected
executable is checked for availability. An unavailable alternative cannot cause
selection to silently switch to it or away from it.

With `--choice`, a static `routes` policy accepts any configured candidate the
choice names, including for a kind its table does not route, and cannot refuse
it. Inspection reports that the explicit choice, not a route, selected it. Under
either form, an ID the catalog lacks refuses as `unknown_choice`, before any
`select` runs, and selects nothing else. An owner who wants to constrain
explicit choices uses `select`, which must
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
authority. The worker imports the entry's resolved path as a string, so that
path must name exactly the admitted file. A path that is not UTF-8 has only a
lossy string, and the runtime reads a `?` as the start of a query, so
`policy.ts?x` would load `policy.ts`. Either refuses at the authority stage
before any worker starts. Personal policy can explicitly import a repository entry, in which
case that import is the owner's choice. Imported trusted code inherits that
authority; this is not a sandbox against its owner.

The Rust process locates its worker from the installation, never PATH or cwd,
and verifies the worker protocol/build identity before evaluating policy. It
starts the worker in a private empty directory, using null stdin, captured
diagnostic streams and a private framed protocol channel. The channel is
descriptor 3, named by no variable, path or argument, and the worker inherits
no other descriptor beyond its standard streams. Every descriptor the front
holds is closed at the worker's exec, however high it is numbered, including
one above the soft descriptor limit. They are listed from the process's
descriptor directory, and if that listing is unavailable the invocation
refuses rather than bounding the sweep. The request passes the caller's
cwd as data; it does not make it the worker's runtime cwd.

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

The SDK supplies bounded text/JSON reads, source attribution, run lookup,
diagnostic output and an abort signal. It measures every delivered source, hashes
the bytes actually read, and includes the adapter's explicit version. A context
loader returns the final serializable context; both worker and Rust validate its
size. Reads of arbitrary files or HTTP by trusted policy remain possible, but
unreported ambient reads are not advertised as reproducible context. Such a
policy must supply versions/digests for the evidence it puts into the context.
The receipt records the entry digest, declared policy version and worker build;
it does not claim to hash every dynamic dependency or every remote response.

SDK operations are `readText(path, maxBytes)`, `readJson(path, maxBytes)`,
`run(runId)`, `diagnostic(text)` and `signal`.
Reads resolve explicit relative paths against request cwd and return content
plus canonical source name, byte count and SHA-256. Policy imports use their own
module-relative resolution. A read is synchronous, opens a regular file without
blocking, decodes strict UTF-8 and, for `readJson`, parses it. Its `maxBytes`
defaults to the per-read bound and may be set up to the context budget. A
failed read throws, and a loader that fails because of it refuses naming that
source. Reads are open only while `loadContext` runs: `select`'s host has
`diagnostic` and `signal` alone, since the context it receives is the measured
one. `diagnostic` writes one line to the worker's captured stderr. `signal`
aborts when the front stops the worker at its deadline; a worker whose policy
installs no TERM listener of its own then exits once the abort listeners have
run. Run lookup is a read-only protocol request to the
Rust store. It returns the run's immutable launch fields, including task
identity and catalog snapshot, plus any launch-failure detail, or missing. An
unreadable store refuses rather than reading as missing. The worker never opens
or writes the database. The loader returns context,
and the selection callback receives that measured value. The supplied adapter
uses these operations so its complete delivered context is inspectable.

The **delivered context** is the loader's result, or the caller's document when
there is no loader, with `measured` attached: the `--context` document first,
then each SDK read in call order, each `{ name, via, bytes, sha256 }`. A routes
policy with a loader or a caller context has its context assembled, measured
and inspected too. The worker checks the delivered size before it sends it, and
the front, which validates it by the caller-context rules, measures it again
and is the authority. **Final encoded bytes** and the context digest are those
of the front's compact encoding with object keys sorted. A bound exceeded
inside the worker is recorded the first time: the policy is thrown an error,
and catching it changes nothing, because the phase reports the recorded breach
instead of its value. The worker takes its limits from the front, never from
the `request.limits` the policy sees.

| Resource | Default | Hard ceiling and behavior |
|---|---|---|
| Whole selection, from worker start to its result, including imports, context and callback | 30 seconds | Caller can choose 1–120 seconds; timeout refuses |
| Context delivered to selection, including caller JSON and source metadata | 256 KiB UTF-8 JSON | Caller can choose 1 byte to 8 MiB; overflow refuses, never silently truncates |
| One SDK source read | 64 KiB, or the context budget if smaller | A read's `maxBytes` can choose up to the effective context budget; oversize source refuses |
| Number of context sources | 256 | Fixed; excess refuses |
| Policy result/catalog protocol message | 1 MiB | Fixed; excess refuses |
| Worker diagnostics, both streams together | 256 KiB | Drain within the bound; excess terminates evaluation with an output-limit error, keeping the first 256 KiB |
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
material, reaps the worker (which has the same cleanup grace to exit before
KILL), validates the explicit choice, resolves the selected
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
## Identity and original creator

A run may be associated with a **task identity**, a **reviewed artifact**, both,
or neither. The task identity is the caller's `--task-id`, which survives the
task's file being renamed, retired or reordered; Grove supplies its selected
handle. A reviewed artifact comes from caller or loaded context. Paths are
descriptive source locations only. Every run also has its own **run ID**: a
collision-resistant identity that dispatch allocates, records with the run and
exports to the final harness as `HARNESS_DISPATCH_RUN_ID`.

Dispatch looks nothing up by task or artifact identity. Handles restart in each
grove, and one workspace hosts successive groves, so a lookup by handle would
need a grove namespace that Grove deliberately does not keep. Creator provenance
travels instead as a reference to a run, and a run ID needs no namespace. The
[creator-reference decision](../adr/a-review-carries-its-creator-reference.md)
records that trade-off.

The **original creator** of a reviewed artifact is given by exactly one creator
reference, in one of two forms:

| Form | Evidence class | Where the provider comes from |
|---|---|---|
| `run <run-id>` | Execution-recorded | The named run's immutable catalog snapshot |
| `declared <provider>` | Declared | The owner's literal declaration |

A run reference is written by the session that ran under that run, which reads
its own `HARNESS_DISPATCH_RUN_ID`. Its provider comes from dispatch's record of
the configured launched choice, never from the session's transcription, today's
catalog or the current mapping. *Execution-recorded* describes that provider,
not the association. That the named run produced the artifact, and that it
executed, is the writing session's attestation, which dispatch cannot verify.
A session can reach other runs' IDs: other review bodies, version history,
`record show`, or an inherited variable when a direct harness runs inside a
dispatched session. A wrong but existing run lends its provider, and launch
does not detect it. A declaration remedies an artifact finished before adoption
or by a direct harness, which has no run to name. It is the owner's assertion and
is always reported as declared.

In Grove the reference is the review leaf's `**Creator:**` line, directly under
its `**Reviews:**` line. A session **finishes** a producer when it retires the
producer's leaf, and when its close cascade closes the producer's node, so one
session can finish several. For each producer it finishes, in its task's commit,
it owns the line on the review leaf it cuts and on every live review leaf that
already names that producer's handle. Under dispatch it writes
`**Creator:** run <run-id>`, replacing any line already there. Without
`HARNESS_DISPATCH_RUN_ID` it has no run to name, so it removes any `**Creator:**`
line, run or declared. Those reviews refuse until the owner writes
`**Creator:** declared <provider>` for the finished artifact. A declaration
written before the producer finished is removed with the rest, so no reference
outlives the invocation it described.

The finishing session is the original creator: its run, or the owner's
declaration of it. When a producer was
launched more than once, earlier attempts are not named: the finishing session
replaces or removes their line before the review runs. For a decomposed
producer it is the run whose retirement closed the node, whatever kind that
child was. This is the one-creator simplification, not contributor accounting.
A node that stays open writes nothing, and later work that repairs an already
finished producer changes no reference. A review's retries read the same line
and the same immutable record, so a retry cannot change the answer; only an
explicit edit, visible in version control, can.

Independent callers supply the same association as data: `reviewedArtifact`
carries the artifact ID and a creator of either form. The core validates that
shape and offers run lookup; it compares no providers and has no review
semantics. This is one original creator, not contributor accounting.

<a id="review-policy"></a>
## Supplied review policy

The inactive example uses exact configured review-kind entries, all of which
apply the provider rule; other kinds use the owner's static table. It exports
its rule as a reusable selector over a generic reviewed artifact, so non-Grove
callers can apply it without a task file. Custom review labels require an
explicit example-policy entry. The core knows no review list and no `Reviews` or
`Creator` grammar.

The Grove adapter reads only the supplied task file, on every invocation. For a
configured review entry it requires exactly one standalone `**Reviews:**
<handle>` line and exactly one standalone `**Creator:**` line in either form,
and builds the generic reviewed artifact from them. A task that declares
`**Reviews:**` under a kind that is not a configured review entry refuses, so
an unlisted review kind cannot take a static route. The adapter does not obtain
kind or identity from the filename, resolve the handle, enumerate the tree, read
briefs, select another leaf or use advisory running-session state. Missing,
duplicate or malformed lines and an unreadable task file refuse. The adapter has
independent fixtures for the Grove conventions it interprets.

For a run reference the policy looks the run up through the SDK. A run missing
from this record store, or carrying a launch-failure or not-executed detail,
refuses with the declaration remedy. Inspection and the review's run record
show the named run's task identity beside the `**Reviews:**` handle; the two
need not be equal, because a decomposed producer is finished by a child task
with its own handle. Launch therefore admits an existing run of any task
identity, and an attempt whose execution is unknown, on the reference's
attestation. Inspection is where such a mismatch shows.

The creator's provider, recorded or declared, must be an exact, case-sensitive
member of the current catalog's provider-origin set, with no normalisation. A
relabelled origin or a misspelt declaration therefore refuses with a correction
remedy instead of comparing as different. The requested or mapped candidate
must then have a different provider origin; a gateway label change cannot
establish separation. The policy never selects a replacement on failure. The
same checks run on every invocation, retry and explicit choice. The evidence
class, the creator reference, the resolved provider and the digest of the task
file the reference came from appear in inspection and in the review's run
record. Direct-harness routes do not execute this policy.

<a id="records-and-outcomes"></a>
## Records and later observations

The front process owns a versioned local SQLite store with bundled SQLite,
private user permissions and durable transactions. No database service or Bun
database client is required. Its default directory is
`~/.local/state/harness-dispatch`; an explicit state directory replaces it.
Concurrent invocations use short transactions, never a lock held across policy
evaluation. Disk-full, permission, schema or lock failures before handoff refuse.

Before exec, one committed transaction persists a collision-resistant run ID,
timestamp, task identity and reviewed-artifact association, kind, selected
catalog values, explicit-choice input, resolved executable and argv, original
cwd, policy entry authority/digest/version, worker and adapter versions, context
source digests and sizes, effective limits, decision reason, selection timing and
the creator provenance used. Raw environment values are not stored. Argv
contains the prompt, so records are private local execution data. The run ID is
available to the final harness via its environment and the optional argument
slot. Inspection uses a visibly marked proposed ID, creates no run, and does not
promise that ID will be reused.

A committed run's launch fields never change, so a creator snapshot a review
used cannot go stale before that review's own handoff. There is no creator
registration to revise. Readable but corrupt records never become
missing-provider defaults.

The store is `records.sqlite3` in the state directory, created on first use
with owner-only permissions. It names itself with a SQLite application ID and
a schema version. A store of another application, of another version, or that
SQLite reports as corrupt refuses with exit 4, and is never reset or replaced.
A run's launch fields are one document with its own version. Every field that
a later increment supplies is present in it, as `null` until then. A release
that records something new therefore writes it into new runs only, and a new
table arrives by a migration that only creates. The commit is one exclusive
transaction in a rollback journal at `synchronous = EXTRA`, the setting SQLite
documents as durable in that mode, with `fullfsync` on for macOS. That sync
reaches only the store's own directory. So on first use, the parent of every
record directory the invocation creates is synced before the commit, and a
failed sync refuses with exit 4 like the commit. The store is
opened only after the worker has been reaped.

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

`record show --run R --json` exports the run and its observations. The export
names the run's evidence: `handoff_attempt`, whose execution is `unknown`, or
`launch_failure`, whose harness was `not_executed`. Every outcome not observed
is `unobserved`.
`record observe --run R --file observation.json` validates and atomically appends
one version-1 observation. It requires a caller-generated observation ID, source,
observed-at timestamp and evidence description. Repeating identical ID/content
is idempotent; conflicting content refuses. An import that corrects an earlier
observation names the observation it replaces, retaining both. No import can
change the immutable launch fields.

Supported observation fields include execution confirmation, exit/signal,
duration, input/output/total usage with units, acceptance
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
remain unobserved in the exported view. Observation source/evidence is an
assertion by the importer; the package validates shape and association, not
external truth. A review session can attach its findings to the producer's run,
because its own task file names that run.

<a id="grove-integration"></a>
## Grove integration

Add optional whole-argument slots `kind`, `task_file` and `task_id` to lifecycle
configuration. Populate them from the same authoritative selected task used to
compose the mandate: open kind token, absolute selected task path and stable
handle. The existing prompt is passed unchanged. Existing slots and
direct-harness commands keep their current rules; prompt remains exactly once.
No task-body launch metadata, new Grove invocation override, or prompt/filename
scraping is introduced. Grove allocates and stores nothing for dispatch.

Grove configuration inspection shows these slots symbolically when no task is
selected. It does not fabricate a task or evaluate policy.

The `**Creator:**` line is a methodology convention, like `**Reviews:**`. The
finishing session writes or removes it; Grove's own code neither writes nor
reads either line and records nothing about how a producer ran. The
[creator-reference decision](../adr/a-review-carries-its-creator-reference.md)
names the methodology rules this amends. They ship with the dispatch
implementation, so the methodology never asks a session to name a run from a
tool that is not installed.
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
and the exact remedy for incomplete mappings and a missing creator reference.

<a id="diagnostics"></a>
## Diagnostics and exits

Before handoff, errors carry a stable code, stage, message, relevant input/source
and remedy. `--json` data-command failures use one JSON error on stderr and no
partial stdout object; policy diagnostics are captured and included separately,
never interleaved with protocol or structured output. Text mode prefixes policy
diagnostics on stderr. A failure never launches another candidate. A refused
`run` also reports the equivalent `inspect` invocation, as a command line in text
mode and an argv array in JSON, each with the directory it was run from: the
same selection inputs, `--policy-env` names but no values, and no prompt. An
owner diagnosing an unattended refusal can then reproduce the selection without
reconstructing its inputs. When an input, the program path or the directory is
not UTF-8, no JSON string or text command line holds it exactly. A lossy copy
would name other inputs, so the invocation is reported as unavailable, naming
what cannot be written. A command line that cannot be parsed has no
equivalent. Once a command line parses, its own `--json` flag chooses the
format. A `--json` word that is the value of another flag, such as the
prompt, is data.

Exit codes before exec are 2 for malformed CLI input, 3 for policy/context/
selection refusal, 4 for required-record failure, 5 for worker/protocol/internal
failure, 124 for timeout, 126 for an unexecutable selected program, and 127 for
one not found. An exec error after resolution exits 127 for `ENOENT`, including a
missing `#!` interpreter, and 126 otherwise. INT/TERM/HUP cleanup ends by
restoring and re-raising that signal.
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
symlink, at `../libexec/harness-dispatch/harness-dispatch-policy`. The SDK's
declarations and readable source sit beside it in `sdk/`, and each example's in
`examples/`. The supported portable
archive preserves the same relative layout: its top directory is an
installation prefix, with Grove's own executables beside the front in `bin/`.
A cross-compiled worker is its target's Bun runtime with the policy host
appended, so that runtime ships. Bun would fetch it unverified; the build
instead fetches each target's pinned runtime itself, refuses one whose digest
differs, and hands it to the compiler, so no release builds against an
unpinned runtime.
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
| C library | glibc 2.17, Grove's existing floor | Installed smoke tests run in a glibc-2.17 userland on each Linux target: under the CPU instrument's user-mode QEMU, and first as a container where Docker runs that architecture natively |
| CPU | x64 Nehalem; arm64 at the Cortex-A53 level | The same tests under user-mode QEMU with that CPU model. A probe executing one instruction beyond it (AVX2; an Armv8.1 atomic) must be killed there and run under `-cpu max`, and every ELF file in the archive must match the emulator's registration |
| Kernel | Bun's documented support | None: a container or user-mode emulation runs on the host's kernel |

The release task runs the archive-content assertions and these instruments over
the archives it is about to publish, and publishes nothing unless every one
passes.

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
| Same command, actual shipped examples | Different-origin reviewer on every invocation, retry and explicit choice; same-origin/gateway disguise refuses; a fake producer launched through dispatch writes its `Creator` line from `HARNESS_DISPATCH_RUN_ID`, and the dispatched review of that task file uses the named run's recorded provider, which a changed current mapping cannot rewrite; a store holding an earlier run of the same task identity does not satisfy a review task with no `Creator` line; an unknown run and a run marked not executed refuse; declaration adoption; missing, duplicate or malformed `Reviews`/`Creator` lines refuse; `Reviews` under a kind that is not a configured review entry refuses; a relabelled origin and a misspelt declaration refuse as non-members; the generic reviewed-artifact form selects without a task file |
| Same command, authority and lifecycle fixtures | Hostile cwd policy, dotenv, bunfig/preload, tsconfig, package shadow and BUN_OPTIONS stay inert through the public launcher, each beside its firing configuration below; explicit relative config and personal import are admitted; a documented package specifier resolves to the embedded module; worker and nested normal child environments lack caller completion values; structured diagnostics stay clean; import/loader/callback interruption and timeout launch nothing |
| Same command, records and observations | Required commit failure prevents exec; attempted handoff and exec failure stay distinct; cancellation after the commit launches nothing and marks the attempt not executed; pre-commit refusals create no run; unknown outcomes; round-trip run lookup and observation import, idempotency/conflicts/correction; policy run lookup returns immutable launch fields and an unreadable store refuses; the review's run records the creator provenance used; later observations after tree teardown |
| Existing Grove launch boundary | Original prompt and authoritative `kind`, `task_file` and `task_id` slots preserved as native data; the final harness receives `HARNESS_DISPATCH_RUN_ID`; retiring and reordering the producer between its launch and its review's leaves the review's creator unchanged; a pre-cut review of a decomposed producer carries the run whose retirement closed it through a multi-level close, and selects although that run's task identity is the child's; a dispatched producer attempt followed by a direct-harness finish leaves the pre-existing review with no `Creator` line, and that review refuses with the declaration remedy; direct-harness compatibility; task authoring succeeds with a valid wrapper but bad delegated policy refuses at launch |
| Existing Grove launch boundary, controlling PTY | Final harness retains PID/group, cwd, terminal and native exits; the entry signal mask and dispositions, including SIGPIPE, reach it unchanged; helper receives null stdin and scrubbed control environment; final harness receives fresh channel; signal cancellation during selection and execution, plus descendant escalation |

Per-target release delivery adds the archive/install tests above. Documentation
review verifies activation, both inspection surfaces, the `Creator` line
conventions and their remedies, later outcome entry and launch-time validation
guidance. Fake producers in the lifecycle cases follow the documented
convention, including removal without a run and the node-close step. A real
session's compliance is the methodology's to check, through the conformance
rows that ship with the amendment, not these seams'.
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
partial overrides, automatic retries, automatic creator guessing, lookup of runs
by task or artifact identity, cross-checkout discovery, post-teardown artifact
lookup and dedicated confined `grove run`
integration are not part of this increment. Windows is outside Grove's current
release targets. A hostile-code sandbox and supervision after final exec are
also outside this command's contract.
