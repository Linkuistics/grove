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
failure to its attempt. `record observe` validates and appends a version-1
observation against a recorded run, with idempotent repeats, refused conflicts
and retained corrections, and `record show` exports the run with its
observations, derived evidence and measurements. `loadContext` looks runs up
with `host.run`, which the front answers from the record store under `inspect`
and `run` alike. The delivered context carries each answer as a measured
source, and a run records the creator provenance its context carried. The
shipped `harness-dispatch/examples/review` exports the
[supplied review policy](#review-policy)'s selector over a generic reviewed
artifact: a looked-up creator run, refused when missing or never executed, or
a declaration; the creator's origin an exact member of the current catalog's;
and a reviewer of another origin, from its review kind's entry or the explicit
choice, never replaced when refused. The Grove adapter, `harness-dispatch/grove`,
reads the supplied task file's `**Reviews:**` and `**Creator:**` lines through
the measured read, and `harness-dispatch/examples/grove-review` composes it with
that selector over Grove's session kinds. A loader may return a refusal in
`select`'s shape, which refuses as the policy's own. Inspection and the run
record report the adapter's version whenever the policy imported it.
INT, TERM and HUP not ignored at entry cancel a selection until its program is
resolved: the worker is stopped and reaped, nothing is recorded or launched,
and the `selection_cancelled` refusal is followed by the re-raised signal.
Their handlers stay installed across the record commit, and the linearization
point follows it. A signal seen there launches nothing, appends the
not-executed detail, and is re-raised after the `handoff_cancelled` refusal.
Otherwise the harness receives the entry signal mask and every disposition
that survives exec, SIGPIPE's included. The worker's environment is its base
set plus exact `--policy-env` grants, which refuse the excluded names as
`excluded_grant` and which inspection lists by name, never by value, and the
front's own setting that turns Bun's runtime transpiler cache off. Each
hostile class in the [firing-configuration table](#test-seams) is proved inert
through the front beside its firing configuration, and the probe builds those
configurations need are test instruments that no front accepts.
Every release archive and the Homebrew formula carry the front and its worker
in the [delivered layout](#delivery), each target's worker compiled from a
digest-pinned Bun runtime. The installed smoke test runs the static and
computed TypeScript cases from the extracted archive on macOS arm64 natively,
and on each Linux target in a glibc-2.17 userland under a pinned user-mode QEMU
emulating its CPU floor, Nehalem or the Cortex-A53; Linux arm64 also runs as a
native container. Positive controls show that the userland enforces the glibc
floor and that each CPU model refuses an instruction beyond it. The release
task runs the archive-content assertions and this smoke test before it
publishes anything.
Every selection input in the [command interface](#command-interface) is
delivered. Of the [Grove integration](#grove-integration), the lifecycle
`kind`, `task_file` and `task_id` slots, their standalone refusal and their
symbolic inspection are delivered, and so is launching a session through a
personal command definition that runs `harness-dispatch run` with those slots
and the prompt, beside a direct-harness kind in the same configuration. Grove's
launch-boundary suite shows the harness receiving the unchanged prompt, the
task as native arguments, its run identity and Grove's completion channel,
which the policy worker does not receive. A literal `--choice` there reaches
policy. A leaf authored under that wrapper, whose policy lacks its kind,
refuses at launch and stays live. Under a controlling terminal, the same suite
shows a dispatched harness as the foreground job Grove launched, with its PID,
group, terminal and cwd, and with the signal state a direct harness receives.
Its exit code and signal death reach Grove as a direct harness's do, and its
worker has null stdin and no control variable. A typed interrupt during
selection launches nothing, records nothing and leaves no worker. Grove
answers it, an interrupt during execution and its escalation as it does for a
direct harness. Grove's configuration reference and usage guide, and the
configure-grove skill, explain activation. That means a personal command
definition, a personal policy that no install writes, and an optional literal
`--choice`. They explain that Grove's configuration owns the wrapper and the
policy owns selection, with `grove config show` and `harness-dispatch inspect`
as the two inspection surfaces. They also state that Grove's pre-authoring
check stops at the configured command, give the remedy for an incomplete
mapping, and warn never to grant `GROVE_SIGNAL_FILE` to the policy.
`harness-dispatch --help` carries the Grove command definition they quote,
which is the one the launch-boundary suite runs.
The methodology's [creator step](#identity-and-creator) is delivered. The Grove
plugin's task format, retirement procedure and composition guidance, and the
same files as Grove provisions them to Codex, direct the session that finishes
a producer to write its run on that producer's reviews, or to remove the line
when it has no run. Conformance rows and composition-guidance pins hold that
wording. The creator lifecycle cases of the
[Grove launch boundary](#test-seams) are delivered too. Fake sessions that
follow that step run under the real driver and the shipped Grove review
example. A review's creator survives its producer's retirement and a
reordering, under a mapping changed since. A review cut before its producer
decomposed selects from the run whose retirement closed the node, through two
levels, and inspection shows that run's task beside the reviewed handle. A
direct-harness finish removes a dispatched attempt's line, and the review
refuses until the owner declares. A review's findings attach to the producer's
run after `.grove/` is removed. Grove's configuration reference and usage
guide, the configure-grove skill and the dispatch README explain the two
`**Creator:**` forms, who writes or removes the line, the declaration remedy,
why a wrong but existing run is not detected at launch, and how a review
attaches its findings.
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
which harness-dispatch attaches as `measured`, and the answer to every run
lookup, attached as `runs`; a loader can supply neither. No
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
No automatic package installation runs. Missing imports fail. Because
package.json autoloading is off as well, the delivered worker reads no
`package.json` at run time, so a package resolves only by its file layout, such
as its `index.js`: one whose entry point `main` or `exports` declares does not
load. Whether that limitation is kept is open (`package-entry-resolution-k52`).
Because tsconfig
autoloading is off, `paths` aliases in a tsconfig beside owner policy do not
apply at run time. The shipped declarations serve editor and package type
checking only. Trusted policy may deliberately use normal module imports or
native Bun APIs; those actions are policy effects rather than implicit host
discovery.

Rust constructs a fresh worker environment containing only HOME, a PATH snapshot,
TMPDIR, LANG and LC_* values, plus exact `--policy-env` grants. It always excludes
BUN_*, NODE_OPTIONS, NODE_PATH, NODE_PRESERVE_SYMLINKS (a resolver option),
the inherited IPC channel variables (every `NODE_CHANNEL_*` name, through
which Bun would adopt descriptor 3 as its own channel), dynamic-loader injection
variables (every `LD_*` and `DYLD_*` name) and its private protocol variables
(every `HARNESS_DISPATCH_*` name: the worker is never told the run or where its
records are) from grants. Other `NODE_*` names are grantable; Bun 1.4.2 reads
none of them to load code in the worker.
Naming one refuses as `excluded_grant`, exit 2, before any policy runs; an
empty name or one holding `=` is `malformed_input`. A granted name the caller
has not set is absent from the worker, and duplicates are one grant.
Inspection lists each grant as `policyEnv`, `{ name, set }`, and no value is
printed or recorded anywhere. Beside them, Rust sets one value of its own,
`BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, which turns Bun's runtime transpiler cache
off. Left on, it would run the cached transpiled output of any imported file of
4 KiB or more, from under HOME or a granted `XDG_CACHE_HOME`, in place of that
file. So bytes nobody admitted as policy could change what the admitted file
does, while inspection and the run record report the file's own digest. Bun
consults the setting before either location, and no caller value replaces it,
because it is a `BUN_*` name. Installation control variables never come from
ambient input: the worker is located from the front's real path, never from
PATH, the cwd, `argv[0]` or a variable, and `BUN_BE_BUN`, which would make the
worker Bun itself, is a `BUN_*` name. The generic command cannot know a caller's completion variables,
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
requires; a failed required read or loader fails the whole selection. A loader
that finds a required fact present but invalid can say why instead, by
returning a refusal in `select`'s shape, `{ status: "refused", code, message,
remedy }`. That is the policy's own refusal, `policy_refused` at the context
stage, and `select` is not called. A version-1 context has no `status`, so a
loader's result with one is judged as a refusal, and a malformed refusal is an
invalid context. A bound the loader exceeded is still reported in its place.

The SDK supplies bounded text/JSON reads, source attribution, run lookup,
diagnostic output and an abort signal. It measures every delivered source, hashes
the bytes actually read, and reports the adapter's explicit version. The worker
reports `{ specifier, version }` for the Grove adapter once the policy has
imported it, directly or through an embedded example that composes it, and
`null` otherwise. It reports this after loading, after the context and after
the selection, and the front keeps the last report, so an import during
`loadContext` or `select` counts too. A context loader returns the final
serializable context, or a refusal; both worker and Rust validate its size. Reads of arbitrary files or HTTP by trusted policy remain possible, but
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
source. Reads and run lookups are open only while `loadContext` runs: `select`'s host
has `diagnostic` and `signal` alone, since the context it receives is the
measured one. `diagnostic` writes one line to the worker's captured stderr.
`signal` aborts when the front stops the worker at its deadline or on
cancellation; a worker whose
policy installs no TERM listener of its own then exits once the abort listeners
have run. The loader returns context, and the selection callback receives that
measured value. The supplied adapter reads its task file through these
operations, so the file is a measured source, and inspection and the run record
show its digest.

**Run lookup** is a read-only protocol request that the front answers from the
invocation's record store, under `inspect` as under `run`. The worker never
opens or writes the database, and is not told where it is. `run(runId)` takes a
run ID in its canonical form, and anything else throws a `TypeError` before
anything is asked. There is no lookup by task or artifact identity. The answer
is frozen: `{ runId, status: "found", recordedAt, kind, taskId, candidate: {
id, provider, model, effort }, launchFailure }`, with `taskId` `null` when the
run had none and `launchFailure` `null` or the appended detail as `record show`
exports it. Otherwise it is `{ runId, status: "missing" }`. It projects the
run's immutable launch fields and is never the whole launch document, whose
argv holds the prompt and whose program and arguments no context carries. It
reads those fields and any launch failure, never the run's observations, so
its work is bounded by one launch record however long the run's history grows.
A store file that does not exist, an empty one and a store without the run all
answer missing. A store that exists but cannot be read, one that is corrupt, of
another application or of another version, a launch document this release
cannot read, and a lock held past the wait each refuse the selection with the
store's exit-4 refusal. The front then ends the conversation and stops the
worker, so no policy code runs after it, and it never reads as missing. The
lookup waits for a writer's lock at most the fixed lock wait and never past the
selection's deadline. A wait that reaches the deadline is the selection timing
out. The bounded read after the wait is not interrupted, but the front checks
the deadline when it returns, so an answer that arrives late is a timeout too.

The **delivered context** is the loader's result, or the caller's document when
there is no loader, with `measured` attached: the `--context` document first,
then each SDK read and run lookup in call order, each `{ name, via, bytes,
sha256 }`. A read's name is its canonical path. A lookup's name is its run ID,
its `via` is `run`, and its bytes and digest are those of its answer's compact
encoding with sorted keys. So a lookup counts against the source bound like a
read. When `loadContext` looked a run up, `runs` is attached too: every answer,
in call order, a repeated lookup included. Without a lookup there is no
`runs`, and a context is delivered exactly as a policy without run lookup would
have it. The front keeps its own answers and refuses a worker whose context
reports others. A routes
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
| Record-store lock wait | 2 seconds | Fixed. The commit's wait follows selection, and neither extends nor consumes its bound. A run lookup's wait is inside selection, cut to the time left |

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
The one-line handoff notice is written between the commit and the block, so a
slow stderr delays the final check rather than widening the window after it.
A record-store refusal of the commit while a signal is noted is reported as
`selection_cancelled`, since nothing was recorded.

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
runtime initialization changes it, and reinstate it after that reset. It
records the mask and every signal's disposition from the executable's
initializer section, which the loader runs before `main`. Its pre-exec hook
sets each signal to its entry disposition and then restores the entry mask.
A build in which that initializer did not run refuses `run` with
`signal_state_unavailable` before evaluating anything. A signal the caller
blocked stays blocked throughout and reaches the harness pending. The
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
apply the provider rule; other kinds use the owner's static table. An entry
maps each creator provider origin to its reviewer, so the reviewer follows
whichever origin made the artifact; an origin the entry lacks refuses as an
incomplete mapping. It exports its rule as a reusable selector over a generic
reviewed artifact, so non-Grove callers can apply it without a task file.
Custom review labels require an explicit example-policy entry: a context that
names a reviewed artifact under a kind with no entry refuses, the generic form
of the adapter's rule below. The core knows no review list and no `Reviews` or
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

A standalone line begins with its marker at its first character, so a mention
inside a line or an indented one is prose. Every such line counts wherever it
is, a fenced example included: the adapter parses no markdown, so a quoted
example refuses as a duplicate rather than being skipped. The line is exactly
the marker, one space and the value, with nothing after it but a CRLF ending's
CR. `**Reviews:**` takes a Grove handle, `<slug>-k<key>` in the task-name
grammar. `**Creator:**` takes `run <run ID>` in the canonical form, or
`declared <label>`, whose label is the rest of the line, verbatim, so a stray
space makes it a non-member rather than being trimmed. The task file is read
whole, up to the context budget rather than the per-read default, because
leaves can be larger and only its digest enters the context. The adapter reads
the task file for every kind, and a review kind without one refuses. A caller
context that already names a reviewed artifact under a review kind refuses as
a conflict, since the task file names it. The adapter returns each refusal from
the loader, never from `select`. So a policy that composes the adapter's loader
with a `select` of its own still cannot route an unlisted review. Its refusal
codes are `task_file_missing`, `reviewed_artifact_conflict`,
`reviews_line_missing`, `reviews_line_duplicate`, `reviews_line_malformed`,
`creator_line_missing`, `creator_line_duplicate`, `creator_line_malformed`, and
`review_kind_unlisted`, which the selector's generic form shares.
`harness-dispatch/examples/grove-review` composes the adapter's loader, the run
lookup and the selector over the Grove static example's catalog and routes. Its
entries cover Grove's five `review-*` kinds, each at that example's effort for
the kind.

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
evaluation. During evaluation the store is only read, by run lookups, each a
short read transaction. Disk-full, permission, schema or lock failures before
handoff refuse.

Before exec, one committed transaction persists a collision-resistant run ID,
timestamp, task identity and reviewed-artifact association, kind, selected
catalog values, explicit-choice input, resolved executable and argv, original
cwd, policy entry authority/digest/version, worker and adapter versions, context
source digests and sizes, effective limits, decision reason, selection timing and
the creator provenance used. Raw environment values are not stored. The
creator provenance is what the delivered context carried, since dispatch cannot
tell which facts a policy used. It is `null` without a reviewed-artifact
creator, and otherwise `{ reference, evidence, provider, lookup }`. The
reference is the `run` or `declared` form as given, and the evidence class is
`execution_recorded` or `declared`. The provider is the declared label, or the
found run's recorded provider, else `null`. The lookup is the first answer in
`runs` for the referenced run, or `null` when none was looked up. Inspection
reports the same object, and both text forms show it with the run's task
identity. The adapter is the worker's last adapter report, `{ specifier,
version }` or `null`, and inspection reports it too. Argv
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
table arrives by a migration that only creates. Schema 2 adds the observations
table. A new store is created at version 2. Only `record observe` migrates a
version-1 store, by creating that table inside its own import transaction, so a
refused import leaves the store at version 1; every other command reads and
writes either version as it is. The schema version does not vouch for the
documents a store holds. So every read of a run checks two things before
anything is derived from it: that its launch document is a version-1 object,
and that any launch-failure detail names a cause. The reads are `record show`,
`record observe` and a run lookup. `record show` also checks each observation it
reads, by the import's own validation, against its row's run, ID and
`supersedes`. A document this release cannot read refuses with exit 4. That
happens before any evidence is exported, or any import or migration commits.
Such a document never reads as a missing run, an unobserved measurement or a
confirmation. Only a run lookup, which types the kind, task identity and
candidate into its answer, requires those fields too. The commit is one exclusive
transaction in a rollback journal at `synchronous = EXTRA`, the setting SQLite
documents as durable in that mode, with `fullfsync` on for macOS. That sync
reaches only the store's own directory. So on first use, the parent of every
record directory the invocation creates is synced before the commit, and a
failed sync refuses with exit 4 like the commit. The commit opens the store
only after the worker has been reaped.

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
names the run's evidence: `handoff_attempt`, whose execution is `unknown`;
`execution_confirmed`, whose execution a current observation confirms; or
`launch_failure`, whose harness was `not_executed`. Dispatch's own
launch-failure detail comes first. Its `cause` is `exec_error` for an exec that
returned, with the errno, or `cancelled` for a signal at the linearization
point, with the signal. Each observation is exported as imported,
with every supported measurement, those it did not supply as `unobserved`, and
with `recordedAt` and `supersededBy`. The run-level `measurements` give every
field a `state` and the `current` entries that supplied it: `observed` if any
current observation observed it, else `unknown` if one reported that, else
`unobserved`. Several current values are listed side by side, never combined.
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

| Measurement | Observed `value` | `unit` |
|---|---|---|
| `executionConfirmation` | `true`; an observer that cannot tell reports `unknown` | none |
| `exit` | `{ code }` from 0 to 255, or `{ signal }` named like `SIGTERM` | none |
| `duration`, `humanTime` | a number of at least 0 | `ms`, `s`, `min` or `h` |
| `inputUsage`, `outputUsage`, `totalUsage` | a number of at least 0 | any nonblank unit |
| `acceptance` | `accepted` or `rejected` | none |
| `missedDefects`, `falseFindings` | findings `{ id, summary?, repairs? }`, IDs unique within the list | none |
| `downstreamRepair` | repairs `{ id, summary?, findings?, runId? }` | none |
| `humanInterventions` | a whole number of at least 0 | none |
| `evidenceLinks` | nonblank strings | none |
| `choiceProbability` | a number from 0 to 1 | none |
| `successProbability` | `{ probability, calibration }` naming its data or version, or `{ probability, uncalibrated: true }` | none |

The envelope's `observationId` is a nonblank string of at most 1024 bytes,
unique in the store. `runId` must name the run `--run` names, and `observedAt`
is an RFC 3339 date-time. A finding's `repairs` name observation IDs that may
be recorded later, and a repair's `runId` names the run that made it. Repeats
are compared as parsed JSON, so layout and key order do not make a conflict.
`supersedes` names an observation of the same run that nothing supersedes yet,
so corrections form a chain. A correction replaces the whole observation: what
it omits is no longer current. An execution confirmation for a run with a
recorded launch failure refuses, because that failure is dispatch's own record
that exec never happened. The document is read from a regular file, within a
fixed 1 MiB bound.

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
methodology states the convention in the three rules the
[creator-reference decision](../adr/a-review-carries-its-creator-reference.md)
names. Its task format admits the line, its retirement procedure and node-close
steps carry the finishing session's step, and its composition guidance leaves
the comparing to the dispatcher's policy. They ship in the same release as
dispatch, so the methodology never asks a session to name a run from a tool
that is not installed.
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

Exit codes before exec are 2 for malformed CLI input, an excluded
`--policy-env` name included, 3 for policy/context/
selection refusal and for a refused record lookup or observation, 4 for
required-record or record-store failure, 5 for worker/protocol/internal
failure, 124 for timeout, 126 for an unexecutable selected program, and 127 for
one not found. An exec error after resolution exits 127 for `ENOENT`, including a
missing `#!` interpreter, and 126 otherwise. INT/TERM/HUP cleanup ends by
restoring and re-raising that signal. Its refusal, `selection_cancelled`, names
the signal, and its `exit` is the `128 + N` a shell reports for that death.
After the commit the refusal is `handoff_cancelled`, stage `exec`, which also
names the run and whether its not-executed detail was recorded. A
signal received during evaluation decides the outcome, whatever else the
selection came to; a timeout is exit 124 only when no signal was received.
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
declarations and readable source sit beside it in `sdk/`, the Grove adapter's
in `grove/`, and each example's in `examples/`. The supported portable
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
| Same command, authority and lifecycle fixtures | Hostile cwd policy, dotenv, bunfig/preload, tsconfig, package shadow, BUN_OPTIONS, an altered runtime transpiler cache and the caller's resolver and IPC channel variables stay inert through the public launcher, each beside its firing configuration below; explicit relative config and personal import are admitted; a documented package specifier resolves to the embedded module; worker and nested normal child environments lack caller completion values; structured diagnostics stay clean; import/loader/callback interruption and timeout launch nothing |
| Same command, records and observations | Required commit failure prevents exec; attempted handoff and exec failure stay distinct; cancellation after the commit launches nothing and marks the attempt not executed; pre-commit refusals create no run; unknown outcomes; round-trip run lookup and observation import, idempotency/conflicts/correction; policy run lookup returns immutable launch fields, reads no observation history, and an unreadable store refuses; a stored launch record or observation this release cannot read refuses every read of it; the review's run records the creator provenance used; later observations after tree teardown |
| Existing Grove launch boundary | Original prompt and authoritative `kind`, `task_file` and `task_id` slots preserved as native data; the final harness receives `HARNESS_DISPATCH_RUN_ID`; retiring and reordering the producer between its launch and its review's leaves the review's creator unchanged; a pre-cut review of a decomposed producer carries the run whose retirement closed it through a multi-level close, and selects although that run's task identity is the child's; a dispatched producer attempt followed by a direct-harness finish leaves the pre-existing review with no `Creator` line, and that review refuses with the declaration remedy, which then admits a different-origin reviewer; a review attaches an observation to the run its line names after `.grove/` is removed; direct-harness compatibility; task authoring succeeds with a valid wrapper but bad delegated policy refuses at launch |
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
| `BUN_OPTIONS` preload, and `BUN_BE_BUN` | The shipped worker launched directly with the variable set, bypassing the front process's scrubbing |
| `node_modules/harness-dispatch` shadow | The fixture beside an admitted entry, under a probe build that does not register the virtual modules |
| tsconfig `paths` | The fixture beside an admitted entry, under a probe build with tsconfig and package.json autoloading enabled |
| Runtime transpiler cache under HOME or `XDG_CACHE_HOME` | The shipped worker launched directly without the front's cache setting, after its own cache entry for the policy has had its output altered |
| `NODE_PRESERVE_SYMLINKS`, and `NODE_CHANNEL_FD` | The shipped worker launched directly with the variable set: a helper reached through a directory symlink resolves its bare import beside the link, and a module's `process.send` writes Bun's IPC message where the policy frame belongs |

`task dispatch:probes` builds the three probe builds from the shipped source,
each with one control removed. A probe reports the identity
`probe-<name>-<source digest>`, which no front accepts, so a probe can never
serve as an installation's worker, and an archive carrying one there fails the
installed smoke test before a release publishes. A firing configuration
therefore drives its probe directly, playing the front's side of the protocol.
Probe builds are never built by a release and never ship.

A class with no known firing configuration is reported as such, not counted as a
passing control. On Bun 1.4.2 a `~/.bunfig.toml` preload and tsconfig `paths`
in the caller's cwd are two: neither fired under any build. A test keeps
checking that each still fails to fire under its most permissive probe, and
fails if one gains a firing configuration. Missing-source fixtures carry the
same obligation. The runtime
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
