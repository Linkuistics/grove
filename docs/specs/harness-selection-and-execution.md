# Harness selection and execution

**harness-dispatch** passes what its caller gave it to the owner's TypeScript
`select` function and acts on what that function returns: it records the
command and runs it as its own child to the end, or reports the refusal.
`inspect` reports a selection without launching anything. `run` launches it and
supervises it: it hands the harness the terminal, watches for the session's
exit signal, which `exit` sends, ends the harness, confines it when asked, and
records and reports how the run ended. `init` installs the sample policy.
`record observe` and `record show` carry later evidence against a recorded run.
The command needs no Grove, and Grove runs it for every session it launches.

This specification is the contract of that command and of Grove's use of it. The
[visual document](../design/harness-selection-and-execution/README.md) has
matching package, execution, supervision and provenance views, and the
[runtime evidence](../design/harness-selection-and-execution/runtime-evidence.md)
records what was observed of the runtime the worker is built on.

<a id="problem"></a>
## Problem

A caller knows the work and its session kind. Its owner knows which harness,
model and reasoning effort should perform it. Encoding both in Grove couples
selection experiments to the task-tree driver and prevents independent use, and
keeping a second launch configuration in Grove gives one decision two homes.

The [policy ownership decision](../adr/harness-selection-is-owned-by-policy.md)
defines that boundary. The [worker decision](../adr/policy-evaluation-precedes-the-launch.md)
chooses its runtime, and [dispatch supervises the harness](../adr/dispatch-supervises-the-harness.md)
makes the command the harness's parent for the whole run. This specification
owns the interface contracts below.
The [open-kind](../adr/a-kind-is-an-open-token.md) and
[foreground-job](../adr/the-launched-child-is-a-job.md) decisions govern
Grove's side of it.

<a id="package-boundary"></a>
## Package boundary

The public executable and independently buildable Rust package are named
`harness-dispatch`. They depend on no Grove domain, task-tree or jj package.
The front runs its harness through `keyed-launch`, a runner with no Grove,
task-tree or jj dependency of its own, which an extraction takes with it.
The installation also supplies a private `harness-dispatch-policy` executable:
TypeScript compiled with Bun, including its runtime and SDK. Its versioned
protocol is private to the matching package release. The worker's first message
states its protocol, its package version and a digest of the source it was
compiled from. The front refuses any other identity with exit 5, before the
worker learns which entry to evaluate. Neither a system Bun nor
a system Node installation participates in policy evaluation.

| Owner | Responsibility hidden behind its interface |
|---|---|
| Rust front process | Input validation, owner settings, selected-policy authority, worker lifecycle and bounds, result validation, program resolution, durable records, and the harness's supervision: its job, exit channel, escalation, confinement and end observation |
| Compiled policy worker | Load the selected TypeScript entry, assemble context, call `select`, return serializable evidence and its result |
| Owner policy | Every command it can return, with its model, effort and provider labels; context requirements, preferences and any review rule |
| Optional Grove adapter and example | Read the supplied task file's `Reviews` and `Creator` lines; translate them to a generic reviewed-artifact reference |
| Grove | Select one task, compose the prompt, supply authoritative caller data and a launch directory, run the command as a job, record a teardown, and decide the loop from the run ending |

The SDK, the Grove adapter and the examples are embedded in the worker and
reached through fixed package specifiers (see
[runtime discovery](#policy-authority)). Their type declarations and readable
sources ship beside the worker. The Grove adapter is an explicit import, versioned
and built with the worker. The front has none of its code. The policy host
imports it only to embed it beside the SDK and the examples, register its
specifier and report its version, and never calls its reader. So it takes part
in a selection only when the policy imports it, directly or through the example
that composes it. It uses file data; it does not require a Grove binary for
selection. It never runs a second pick. The package owns no task-tree mutation.
The adapter ships with the package so that it always matches the embedded SDK.
It depends only on the public SDK and Grove's documented task conventions, so an
extraction can move it to Grove's side instead. That move takes it, and the
example that composes it, out of the host's embedded set as well.

<a id="command-interface"></a>
## Command interface

`inspect` and `run` accept the same selection inputs. Only `run` records a
handoff and launches. `inspect` is a proposal, not a launch reservation or a
side-effect-free evaluation of trusted TypeScript. A later `run` evaluates afresh.

| Input | Contract |
|---|---|
| `--kind TEXT` | Required nonempty UTF-8 caller token; no enumeration or Grove filename grammar |
| `--prompt TEXT` or `--prompt-file PATH` | Exactly one for `run`; optional for `inspect`, which otherwise evaluates with a marked placeholder as the prompt; read once; valid UTF-8 with no NUL; preserve bytes, including trailing newlines; never read terminal stdin, and refuse a prompt file that is a terminal |
| `--task-file PATH` | Optional source, resolved against the original cwd; does not supply kind or identity |
| `--task-id ID` | Optional task identity, independent of paths: an opaque nonempty UTF-8 string of at most 1024 bytes |
| `--param NAME=VALUE` | Repeatable caller data, delivered to the policy by name. A name is nonempty and holds no `=`; a value is any UTF-8 text. A repeated name refuses. All names and values together are at most 64 KiB |
| `--context PATH` | Optional version-1 JSON document, read as data; explicit generic context can replace a task file |
| `--config PATH` | Explicit policy entry replacing the personal default, with no implicit merge |
| `--policy-env NAME` | Repeatable exact-name grant to the worker, added to the [owner settings'](#owner-settings) grants; values are never printed in inspection |
| `--timeout-ms N`, `--context-bytes N` | Explicit finite bound, replacing the owner setting, within the hard ceilings below |
| `--state-dir PATH` | Explicit location for this owner's records, replacing the owner setting; default under the user's local state directory |

`run` alone takes four inputs about the run rather than the selection. They
never reach the policy, and `inspect` takes none of them:

| Input | Contract |
|---|---|
| `--exit-dir DIR` | An existing directory to allocate the run's [exit channel](#exit-signal) in, which dispatch neither creates nor removes; otherwise a private per-run directory |
| `--ending-file PATH` | A path that must not exist, in an existing directory, where dispatch writes the [run ending](#run-ending) once the harness is reaped |
| `--confine` | Run the harness under [confinement](#confinement) |
| `--runtime-read FILE` | Repeatable, with `--confine` only: a regular file the confined harness may read |

No task file or Grove installation is required. The command gives a parameter no
meaning: its name and value are the caller's, and only the policy reads them.
Generic context can identify a reviewed artifact and its creator's run or
declared provider without adopting any Grove review label.

`exit` is the [exit signal](#exit-signal): it takes no input, creates the file
`HARNESS_DISPATCH_EXIT_FILE` names, and with no such variable signals nothing,
says so and exits 0. It reads no owner setting, policy or record.

`inspect --json` emits one versioned object on stdout. It includes the resolved
entry path and its authority, policy/adapter versions, the parameters, the
selected command, its provider/model/effort labels and reason, context sources
and hashes, measured UTF-8 byte totals, effective bounds and where each came
from, timing and the creator provenance used. The selected command is reported
as `command`, an object holding the `program` and `args` the policy returned and
the `executable` that program resolved to, an absolute path. That object is part
of this contract: a caller that launches the command itself takes it from there.
Such a caller executes `executable` with `args` and does not resolve `program`
again: the lookup depends on the cwd and PATH inspection ran with, and another
resolver need not agree with it. Such a launch has no run record and no
supervision; a caller that wants either, a confined launch included, launches
through `run`. Human output contains the same facts without requiring a parser.
`run` reserves stdin and stdout for the harness; its handoff and end notices go
to stderr, each as one JSON line under `--json`.

`inspect` without a prompt gives the policy a fixed marker text as the prompt,
which the SDK names so that a policy can recognise it, and reports that the
prompt was not supplied. A policy that only places the prompt in its arguments
then shows the marker where the prompt goes. A policy that reads the prompt
selects from the marker, so inspection reproduces such a selection only when it
is given the same prompt.

`init` writes the [sample policy](#sample) to the personal default path and
reports that path. It takes no input, and reads no owner setting. It refuses
when anything already exists at that path, as `policy_exists`, and has no
option to replace it.

Data commands support `--json`. The policy, the request and inspection are
schema version 2; context documents, record exports and observations are
version 1. Unknown versions and unknown contract fields are refused with their
location. Help includes independent, Grove, refusal-recovery and observation
examples. No pager, interactive confirmation or automatic retry is part of this
interface.

<a id="policy-and-choice"></a>
## Policy and the selected command

The selected ESM TypeScript module exports one named `policy` value, a plain
object. It contains `schemaVersion` (2), a nonempty owner-maintained `version`,
`select(request, context, host)`, and an optional `loadContext(request, host)`
callback. `select` may be asynchronous and may compute anything. That is the only
form. The command holds no catalog of harnesses and no table of kinds, and it has
no input that names a choice. A table from kind to command is something an
owner's `select` consults, and its `reason` names the entry it applied. A policy
that declares version 1 refuses, with the remedy to rewrite it to this contract;
nothing converts one.

Whether a `select` is static or dynamic is not something the command knows,
reports or validates. Both words describe what the owner's function consults,
and nothing outside the policy file distinguishes them.

The request fields are `schemaVersion` (2), `kind`, `prompt`, `cwd`, `params`,
optional `taskFile`, optional `taskId`, optional `context`, and effective
`limits`: `selectionMs`, `contextBytes`, `sourceBytes`, `sources`,
`messageBytes` and `diagnosticsBytes`. `prompt` is the caller's prompt, byte for
byte. `params` holds every `--param` by name, each value a string, and is empty
when the caller passed none. The request carries no run identity: the harness
reads its own from its environment. The policy receives the request frozen.
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
`args`, `select`, `loadContext` and the like) says why. `facts` and
assessment values are data, never checked for such names: the command takes no
part of a command from a context, and what a policy builds from one is the
policy's.

`select` returns one of two variants, told apart by `status`:

| Variant | Fields |
|---|---|
| `selected` | `program`, `args` (an array of strings), `provider`, `model`, `effort` and `reason` |
| `refused` | `code`, `message` and `remedy` |

A selected result is the command to run. The front executes `program` with
`args`, each string one whole argument, with no shell splitting, shell
evaluation or second interpretation of any text. The function builds those
strings from its request, so a caller value inside an argument is ordinary
string building. `provider`, `model` and `effort` are the owner's labels for
what the command runs. They are never inferred from the program or its
arguments, they need not appear in `args`, and they are an owner assertion, not
verified backend identity. `reason` is nonblank, and inspection and the run
record report it verbatim. Every field is a nonblank string except `args`, whose
strings may be empty, and no string that reaches exec may hold NUL.

The program is a literal absolute path or PATH name; a relative path containing
a separator resolves against the caller's cwd and inspection reports that
resolution. A PATH name is looked up as `execvp` does, in the caller's PATH: an
empty or relative entry is relative to the cwd, and the first executable regular
file wins. A PATH that is set but empty is one empty entry. An unset PATH leaves
nothing to search, so the name is not found. Resolution happens once, and `run`
executes the resolved file with the program as returned for `argv[0]`.
Inspection resolves the program as `run` does and refuses with the same exit
when it cannot. A resolved path that is not UTF-8 refuses as unexecutable, for
`run` as for inspection. Inspection reports the path as a string that its
caller executes, and the lossy string of such a path names another file. The
file is not passed over for a later PATH entry, because the caller's shell
would have run it.

The worker reports the policy's version and whether it has a loader once the
entry has loaded, and Rust validates that before any `loadContext` or `select`
runs. The context follows when there is a loader or a caller context, and Rust
validates and measures it. The result comes last, and Rust validates its shape
before use: a field beyond its status's own is refused. Invalid exports,
malformed results, exceptions, an unresolved promise or abstention refuse, each
with its own code. A policy's own refusal is reported under the stable code
`policy_refused`, with the policy's code beside it, its message and its remedy,
so an owner's codes never collide with the command's. Nothing is substituted
for a refused or unavailable command. Each retry is a separate invocation,
reevaluates policy and receives a new run identity.

The command validates the shape of what `select` returns and nothing about its
content. It does not check that the prompt is among the arguments, that a
program is one the owner listed anywhere, or that a kind has a route. Those are
the owner's function's to hold, with the type checker and whatever tables it
keeps. A policy that turns outside text into a command, a
[deciding agent's](#dynamic-dispatch) answer included, should choose among
commands its own code wrote and never place that text in `program` or `args`.

Every shipped example is inactive until an owner's policy imports it, and each
is written to this contract. `harness-dispatch/examples/static` routes a
caller's own kinds without Grove, and `harness-dispatch/examples/grove-static`
routes Grove's session kinds. Both consult an exact table and explain effort in
terms of abstraction, uncertainty, consequences, downstream repair,
reversibility and available checks. They are editable starting policy, not model
rankings or calibrated estimates. `harness-dispatch/examples/review` and
`harness-dispatch/examples/grove-review` are the
[supplied review policy](#review-policy). The [sample policy](#sample) is not an
example to import: `init` installs it as the owner's own file.

<a id="sample"></a>
### The sample policy and the choice file

The sample is one policy file with the real `codex` and `claude` command lines.
It routes each of Grove's session kinds and the standalone `release-notes` kind,
and it offers four arrangements of which harness leads and which reviews, and
two modifiers that change model and effort across an arrangement. With no local
choice it selects its default arrangement and modifier. It labels `codex`
commands `openai` and `claude` commands `anthropic`. Run in a secondary jj
workspace, whose `.jj/repo` is a file naming the shared store, it grants the
harness the main repository holding that store, derived from the file in the
request's `cwd`: the store is `<main>/.jj/repo`, and a colocated repository's
git directory sits beside it rather than inside it. In a primary workspace the
store is inside the cwd, and it grants nothing more. That is the
whole of its jj knowledge, and none of it is the command's. It reads no
parameter and names no session.

`init` installs it and nothing else does: `run` and `inspect` with no policy
refuse as `policy_missing`, whose remedy names `init`. `init`'s report says that
the sample launches `codex` with approvals off and full access, so that an owner
reads the file before the first launch.

One checkout gets a different selection from another through a **choice file**,
which a supported SDK helper reads. The policy names a directory and the names
it offers. The helper reads `.harness-dispatch-choice` there: a regular file of
at most 4 KiB holding names separated by whitespace. It returns those names, or
nothing when the file is absent. A name the policy did not offer refuses, naming
the file, the name and the names offered, and so does a file of any other kind.
The refusal is a value the helper returns in `select`'s own shape, for the
policy to return. The file therefore chooses among what
the owner's policy already holds and can introduce no program, argument or
label. Because of that it needs no version-control check: a repository that
ships one picks among the owner's own options, and only for an owner whose
policy calls the helper. The read is an ordinary one, not a measured source; a
policy names the choice it applied in its `reason`. The sample calls the helper
with the caller's cwd and its arrangement and modifier names. A file there
replaces the sample's default selection whole, and must name exactly one
arrangement.

<a id="policy-authority"></a>
## Policy authority and runtime discovery

The personal default is `~/.config/harness-dispatch/policy.ts`. Missing, unreadable
or invalid selected policy stops the invocation and names the selected path.
There is no cwd search, repository override or environment-selected policy
entry, and nothing but an explicit `init` writes policy into the user's
configuration. A `--config` path is
resolved against the caller's original cwd; inspection identifies that explicit
authority. The worker imports the entry's resolved path as a string, so that
path must name exactly the admitted file. A path that is not UTF-8 has only a
lossy string, and the runtime reads a `?` as the start of a query, so
`policy.ts?x` would load `policy.ts`. Either refuses at the authority stage
before any worker starts. Personal policy can explicitly import a repository entry, in which
case that import is the owner's choice. Imported trusted code inherits that
authority; this is not a sandbox against its owner.

<a id="owner-settings"></a>
**Owner settings** are what an owner sets about every invocation without the
caller passing anything. They live in one optional JSON file beside the policy,
`~/.config/harness-dispatch/settings.json`, which the front reads before it
starts the worker:

| Key | Sets | Flag that replaces it |
|---|---|---|
| `timeoutMs` | The whole-selection bound | `--timeout-ms` |
| `contextBytes` | The context budget | `--context-bytes` |
| `stateDir` | The record directory, an absolute path | `--state-dir` |
| `policyEnv` | Names granted to the worker, an array | `--policy-env` adds to it |

Every key is optional and a missing file sets nothing. A value is held to its
flag's own rules, so a bound outside its range or an excluded grant refuses as
the flag would. A file that is not a JSON object, or that holds an unknown key,
refuses with exit 2, naming the file and the key. The file has the personal
policy's authority and its rules: it is found from HOME alone, and no variable,
cwd or repository file supplies or replaces it. Without an absolute HOME it has
no location and sets nothing. Every command but `init` and `exit` reads it, the
record commands included, so they find the same store. Inspection reports each
bound and the record directory with where its value came from. The policy entry
has no setting. An owner who keeps policy elsewhere re-exports it from the
personal entry, or names it with `--config`.

The settings are the same for every kind. Each bound is a ceiling, so one that
admits the owner's slowest selection serves the rest: an owner who raises the
time bound for a deciding agent raises it for a kind that only consults a
table, and gives up that kind's shorter failure bound. A policy that wants a
tighter limit on part of its own work, its deciding agent's time for one,
applies it in its own code. One record directory is what lets a run lookup
under one kind find a run recorded under another.

A setting cannot live in the policy module. The grants shape the worker's
environment before any policy code loads, and the time bound runs from the
worker's start, imports included.

The Rust process locates its worker from the installation, never PATH or cwd,
and verifies the worker protocol/build identity before evaluating policy. It
starts the worker in a private empty directory, which it creates for that one
evaluation with owner-only permissions whatever the caller's umask, using null
stdin, captured diagnostic streams and a private framed protocol channel. The
channel is
descriptor 3, named by no variable, path or argument, and the worker inherits
no other descriptor beyond its standard streams. Every descriptor the front
holds is closed at the worker's exec, however high it is numbered, including
one above the soft descriptor limit. They are listed from the process's
descriptor directory, and if that listing is unavailable the invocation
refuses rather than bounding the sweep. The request passes the caller's
cwd as data; it does not make it the worker's runtime cwd.

The worker does not stay in the directory it starts in. Before it registers a
module or announces itself, it moves to `/`, and a worker that cannot move
never says hello, so it is never given a policy. The two directories do
different work. As a process starts, the runtime reads bunfig, and the dotenv
files of its first VM, from the directory it starts in, which is why that one
is private and empty. It resolves some imports from the directory the process
is in, which is why the worker leaves: `/` has no ancestors. So policy code
runs with `/` as its current directory, and a relative path the policy opens
itself resolves there. The caller's directory is `request.cwd`.

The empty start directory is a second control, beside the switches below, only
for what the runtime reads as the process starts. The runtime loads dotenv
files again for each VM it starts later, from the directory the process is
then in. A native `Worker` a policy starts is such a VM, and it starts in `/`
unless the policy has moved. The dotenv switch alone keeps `/.env` out of it,
where a worker that stayed in its start directory would have both controls.
That is the price of the move, which closes a wider reach. Without it, every
directory between the start directory and `/` would have its `package.json`
read on each evaluation, and would answer a module with no file location, as
the paragraphs below state. The file the move leaves to the switch belongs to
whoever owns `/`. bunfig is not read again.

The worker is compiled with dotenv, bunfig and tsconfig autoloading explicitly
disabled, and with package.json autoloading explicitly enabled, so that an
ordinary package loads. Its own entry imports embedded modules and prefixed
built-ins. Before importing the selected entry, it registers each documented
package specifier as a runtime virtual module backed by its embedded copy:
`harness-dispatch/sdk`, `harness-dispatch/grove`, and one
`harness-dispatch/examples/…` specifier per shipped example. Owner policy
imports those names. There is no installed path to name and no `node_modules`
for the owner to maintain. The SDK and adapter a policy receives are always the
running worker's own, so an upgrade cannot mix versions. A registered specifier
that an import names resolves to its embedded module even when a
`node_modules/harness-dispatch` package sits beside the importing entry,
however that package declares its modules, and even when the entry's own
package is named `harness-dispatch`. The prefix itself reserves nothing: an
unregistered name under it resolves like any other bare specifier. So the
registered list is part of the worker's versioned protocol, and every documented
specifier is on it. The registration answers a specifier as an import writes
it. A name that an `imports` map produces is not one, which the
[alias limit](#imports-alias) below states.

Other external imports start at the selected absolute entry. Bare specifiers
resolve through `node_modules` from the importing module's directory upward, and
relative imports from the importing module, never from the invocation directory.
A package loads by the entry point its `package.json` declares, with `main` or
`exports`, and by its file layout when it declares none. An `exports` or
`imports` map is read under the runtime's own conditions, `bun` among them. No
variable chooses one, a granted `NODE_ENV` included. No automatic package
installation runs, whatever a `package.json` lists as a dependency. Missing
imports fail.

The worker reads `package.json` at run time, so an importing module's own
package takes part in its imports. That package is the nearest `package.json`
at or above the module that the runtime reads as one: a file, or a link to
one, holding a JSON object. It needs no `name`. Its `imports` map resolves the
module's `#` names, and a `#` name it has no entry for refuses, whatever a
`package.json` further up maps. A bare import of its own `name` resolves
through its `exports`, ahead of any `node_modules` package of that name, a
nearer one included. For a well-formed `package.json` this is Node's package
resolution, so a policy directory that is a package behaves as one. A
`package.json` the runtime does not read as one is passed over with no
diagnostic, and the search goes on upward: one that does not parse, one whose
JSON is not an object, or a directory of that name. So a broken `package.json`
beside a policy leaves the next one above it choosing the code, where Node
refuses the import
([runtime evidence](../design/harness-selection-and-execution/runtime-evidence.md#nearest-package-json)).
These files sit at or above the importing module, where `node_modules` is
already trusted, and an entry admitted with `--config` admits them with it. A
`package.json` in the caller's cwd takes no part unless an importing module
sits at or below it.

<a id="imports-alias"></a>
One consequence is a limit. An `imports` target that names a package is looked
up in `node_modules` by the runtime's resolver, which knows only files and
built-ins and never consults the worker's registration. So an alias to a registered
specifier, such as `"#sdk": "harness-dispatch/sdk"`, does not reach the
embedded module. With no `node_modules/harness-dispatch` it is a missing
package, and the import refuses. Beside one, it is that package. The same
`package.json` could point `#sdk` at any file, so the alias gives its author
no new reach, but an owner who writes one gets the installed package and not
the worker's SDK. Import a registered specifier by its name.

The runtime reads a `package.json` in one more place. It resolves a module
compiled into the worker, and any module with no file location, as though it
sat in the directory the process is in, and it records that directory and each
one above it. Importing the policy entry is such a resolution, so it happens
on every evaluation. The worker is in `/` by then, so that is `/package.json`
alone. No directory between where the worker started and `/` has its
`package.json` read on that account, the caller's TMPDIR included. One there
answers nothing. Nor is it opened, so one that never yields its content, a
link to a FIFO, delays no selection. A worker that did not move would open
each of them on every evaluation, and read a regular file whole: a link to a
FIFO would stall a plain policy to its deadline, as a file past 4 GiB would,
and one with an `imports` map would answer a `#` import from a `data:` module
([runtime evidence](../design/harness-selection-and-execution/runtime-evidence.md#package-json-autoloading)).
Those files belong to whoever can write there and not to the owner's policy,
which is why package.json autoloading depends on the move.

A module with no file location resolves from the worker's current directory
upward: its bare imports through `node_modules` there, and its `#` names and
its package's own name through the `package.json` there. Such a module is one
a policy imports from a `data:` or `blob:` URL, or registers as a virtual
module of its own. The worker is in `/` when any policy code runs, so that walk
is `/` alone. No directory between where the worker started and `/` takes part,
the caller's TMPDIR included, and a `node_modules` package or a `package.json`
in one is nothing to such a module. `/` itself does take part: `/node_modules`
and `/package.json` answer such a module, as they answer a module in a file,
which they are already above. `/package.json` is also read on every
evaluation, whatever the policy imports. So the guarantee stops short of `/`,
and a TMPDIR that is `/` has none. A policy that changes directory itself moves
that walk with it. That is a policy effect, like any other use of a native API.

Because tsconfig
autoloading is off, `paths` aliases in a tsconfig beside owner policy do not
apply at run time. The shipped declarations serve editor and package type
checking only. Trusted policy may deliberately use normal module imports or
native Bun APIs; those actions are policy effects rather than implicit host
discovery.

Rust constructs a fresh worker environment containing only HOME, a PATH snapshot,
TMPDIR, LANG and LC_* values, plus the exact names granted by the owner settings
and by `--policy-env`. It always excludes
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
worker Bun itself, is a `BUN_*` name. The generic command cannot know a
caller's control variables, so it cannot refuse to grant them. Naming one in
the owner settings or with `--policy-env` is the owner explicitly giving the
worker that authority. Grove passes no grant, and the usage documentation warns
against granting `GROVE_LAUNCH_DIR`. Other credentials needed by policy, such
as a deciding agent's, require an explicit named grant. The harness receives
the original environment, Grove's launch directory included, plus the three
reserved values `HARNESS_DISPATCH_RUN_ID`, `HARNESS_DISPATCH_STATE_DIR` and
`HARNESS_DISPATCH_EXIT_FILE`, which replace inherited ones; a
[confined](#confinement) harness receives a smaller environment. Worker
scrubbing does not rewrite the harness's environment.

<a id="bounded-context"></a>
## Bounded context

The worker sees a versioned request with kind, prompt, original cwd, parameters,
optional task file, stable identity and generic context. It sees no reserved
final environment, so the prompt it reads carries no authority to end the
caller's session. The context value separates caller
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
is frozen: `{ runId, status: "found", recordedAt, kind, taskId, provider, model,
effort, launchFailure }`, with `taskId` `null` when the
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
reports others. The worker checks the delivered size before it sends it, and
the front, which validates it by the caller-context rules, measures it again
and is the authority. **Final encoded bytes** and the context digest are those
of the front's compact encoding with object keys sorted. A bound exceeded
inside the worker is recorded the first time: the policy is thrown an error,
and catching it changes nothing, because the phase reports the recorded breach
instead of its value. The worker takes its limits from the front, never from
the `request.limits` the policy sees.

| Resource | Default | Hard ceiling and behavior |
|---|---|---|
| Whole selection, from worker start to its result, including imports, context, callback and any child the policy starts | 30 seconds | Owner or caller can choose 1–600 seconds; timeout refuses |
| Context delivered to selection, including caller JSON and source metadata | 256 KiB UTF-8 JSON | Owner or caller can choose 1 byte to 8 MiB; overflow refuses, never silently truncates |
| One SDK source read | 64 KiB, or the context budget if smaller | A read's `maxBytes` can choose up to the effective context budget; oversize source refuses |
| Number of context sources | 256 | Fixed; excess refuses |
| Policy result protocol message | 1 MiB | Fixed; excess refuses |
| Parameters, all names and values together | 64 KiB | Fixed; excess refuses |
| Worker diagnostics, both streams together | 256 KiB | Drain within the bound; excess terminates evaluation with an output-limit error, keeping the first 256 KiB |
| Prompt | 1 MiB | Fixed; platform argv/environment limits can refuse smaller payloads at exec |
| Record-store lock wait | 2 seconds | Fixed. The commit's wait follows selection, and neither extends nor consumes its bound. A run lookup's wait is inside selection, cut to the time left |

Time before handoff is therefore bounded by the selection bound, the worker's
cleanup grace and the lock wait, plus executable resolution and the record
commit. The default is short so that a policy which only consults a table fails
fast. The ceiling admits a policy that waits minutes for a deciding agent, and
only the owner or the caller raises the bound toward it. The raw prompt has its
own budget, outside the context budget, and is never truncated to satisfy a
context limit. JSON size means encoded UTF-8 bytes, not characters or token
estimates.
Inspection reports actual source bytes and final encoded bytes separately, plus
limits and errors. Dispatch itself keeps no long-lived cache, estimates no
tokens and calls no model. Policy cannot raise its own hard ceilings after
evaluation begins.

<a id="execution-contract"></a>
## Execution and authority

The front process preserves its caller's cwd, descriptors, process group and
terminal ownership while it selects. Under Grove it is the leader of the job
Grove's runner made it. The worker joins that job; neither process creates a
session or detaches. The SDK has no subprocess API: file context and network
computation run in the worker. Trusted policies that create their own children
must keep them in the enclosing job, grant no interactive stdin, and reap them
before returning; detached/background policy services are outside the contract.
The shipped examples, the sample and the adapter create no such children.

<a id="dynamic-dispatch"></a>
**Dynamic dispatch** needs nothing more than that. A policy can hand the prompt
to a deciding agent: it starts the agent as a child with the runtime's own
process API, in the caller's directory, which the request gives it as `cwd`,
waits for its answer, reaps it, and maps that answer to a command. The agent
inherits the worker's environment, so its credentials are names the owner
grants, and its time is the selection bound the owner sets. No agent policy and
no model-calling policy ships, and none is tested: a scripted program stands in
for the agent at the command seam.

The prompt a deciding agent receives is a mandate. Under Grove it tells its
reader to load a skill and carry out a task. Nothing in harness-dispatch or
Grove keeps a deciding agent from doing that instead of evaluating it. **Keeping
the agent from executing the task, and from changing the caller's tree, is the
policy owner's job**: what the agent is told, which tools it has and whether it
can write are all set by the policy that starts it. The dispatch README and the
SDK's description of `prompt` say so where an owner writing such a policy reads
them.

Rust handles INT, TERM and HUP from the start of an invocation to its end —
while the policy selects, across the handoff and while the harness runs —
unless a signal was already ignored at entry. It stops the worker on
cancellation and reaps it before continuing or returning. A hard wall-clock
deadline kills a stuck worker even during module import or a synchronous
infinite loop. Its cleanup grace is at most one second, then KILL. Signal
cancellation reaches ordinary descendants through the enclosing process group.
Abrupt KILL may leave no terminal record; this is not a promise of cleanup
against deliberately detached owner code.

After a result arrives, Rust closes protocol descriptors, removes temporary
material, reaps the worker (which has the same cleanup grace to exit before
KILL), validates the result, resolves the returned program, and commits the
required handoff record. It checks cancellation at each boundary. It then blocks
the handled signals and checks for pending cancellation once more. Cancellation
observed after the commit launches nothing. Rust appends a best-effort
not-executed detail to the attempt and ends by re-raising the signal. A failed
append leaves an attempt of unknown execution, never a success. Otherwise Rust
spawns the harness, with its own handlers still installed, and unblocks the
handled signals once the spawn has returned. The one-line handoff notice is
written between the commit and the block, so a slow stderr delays the final
check rather than widening the window after it. A record-store refusal of the
commit while a signal is noted is reported as `selection_cancelled`, since
nothing was recorded.

That final check is the linearization point. Cancellation observed before it
launches nothing, and a handled signal arriving after it but before the reap
sample cancels the run, which dispatch ends and records as [supervision](#supervision) states. Only a signal dispatch
cannot catch ends it with the attempt's execution unknown.

The harness's start is transparent to the signal mask and dispositions. The
harness receives the signal mask, and every disposition that survives exec,
exactly as the front process inherited them at entry, for every signal including
SIGPIPE. A signal
ignored at entry gets no handler at any stage and stays ignored: a `nohup`
caller's ignored HUP reaches the harness ignored, and cancels neither the
selection nor the run. One ignored disposition is the front's to change for
itself: with SIGCHLD ignored the system would reap the harness unwatched, its
status lost and its group's ID no longer reserved, so the front restores
SIGCHLD's default, which installs no handler, and the harness still receives the
ignored entry disposition. Rust's standard runtime ignores SIGPIPE before `main`,
and its spawn path resets SIGPIPE to default before running pre-exec hooks
while keeping the calling thread's mask. The front therefore records the entry
SIGPIPE disposition before any runtime initialization changes it, and
reinstates it in the harness after that reset. It records the mask and every
signal's disposition from the executable's initializer section, which the
loader runs before `main`. The pre-exec hook of the spawned harness sets each
signal to its entry disposition and then restores the entry mask, so the
handlers the front keeps for itself never reach the harness. A build in which
that initializer did not run refuses `run` with `signal_state_unavailable`
before evaluating anything. A signal the caller blocked stays blocked throughout,
in dispatch and in the harness. Pending signals do not pass on: the harness,
like any spawned child, starts with none pending, so a blocked signal sent to
dispatch stays pending in dispatch, undelivered, while one sent to the harness
is pending there. The command and controlling-PTY seams observe the result.

A harness that cannot be started is a launch failure: the front appends an
observable launch failure when it can, reports the errno with a remedy and exits
nonzero, and failing to append it does not erase the attempt or convert it to
success. A returned wrapper must in turn exec the harness it fronts. Dispatch
runs one command; later model changes inside a harness are outside its
observation and enforcement.

<a id="supervision"></a>
### Supervision

`run` spawns the selected command as its own child and stays its parent until it
has reaped it, for every caller
([dispatch supervises the harness](../adr/dispatch-supervises-the-harness.md)).
The harness is a **job**: it is spawned into a process group of its own, with
the entry signal state above, and when the front owns a controlling terminal and
is its foreground group, the front hands the terminal to the harness's group.
Once the harness is reaped the front takes the terminal back from the harness's
group, or, when the harness died of a signal, from whichever group then holds
it, unless that is its own group or the session leader's. It does so with
SIGTTOU ignored, as it handed the terminal over, and restores the terminal
attributes it saved at the handover. A front that never held the foreground
during the run takes nothing
([the launched child is a job](../adr/the-launched-child-is-a-job.md)). A typed
Ctrl-C therefore reaches the harness's group and not the front. The harness's
cwd and standard streams are the front's, and the front reads none of them. Its
`argv[0]` is the program as `select` returned it. Its environment is the
front's own with three reserved values set, each replacing an inherited one:
`HARNESS_DISPATCH_RUN_ID`, `HARNESS_DISPATCH_STATE_DIR` and
`HARNESS_DISPATCH_EXIT_FILE`. A [confined](#confinement) harness's is smaller.

<a id="exit-signal"></a>
**The exit signal.** Every run has a fresh **exit channel**: a path with a
128-bit random name, never deliberately reused, allocated before the harness
starts and published to it as `HARNESS_DISPATCH_EXIT_FILE`. The channel's
appearance is the run's **exit signal**, a session's statement that its run is
over. It carries nothing, and dispatch reads no content. `harness-dispatch exit`
sends it: it creates that file, and one that already exists is success. Where
the variable is unset or empty it signals nothing, says that it is not running
under a supervised run, and exits 0. It reads no owner setting, policy or
record, so it runs inside [confinement](#confinement). A nested run publishes its
own channel to its own harness, so an exit signal ends only the run whose
channel it names, and the worker is never told a channel
([policy authority](#policy-authority)).

The channel is allocated in the directory `--exit-dir` names when the caller
passes one, which must exist and which dispatch neither creates nor removes, and
otherwise in a private per-run directory dispatch creates, owner-only, under its
TMPDIR and removes after the run. A harness inside a sandbox of its own can
create the channel only where that sandbox lets it write, which dispatch cannot
know and a caller can: such a caller names the directory. Dispatch removes the
channel after the run.

**The escalation.** An interactive harness that has finished returns to its
prompt rather than exiting, so the exit signal is the evidence that a run is
over, and ending the harness is dispatch's job. Dispatch is outside whatever
sandbox the harness runs under, where a harness asked to end itself may be
silently denied. Once the channel appears dispatch waits the **grace**, 2
seconds, then sends SIGTERM to the harness's process group and to the harness,
waits the **kill-grace**, 5 seconds, then sends SIGKILL. The grace lets the exit
verb's own call return and the session's turn end; the kill-grace is time for an
orderly shutdown. Both are dispatch's fixed constants, which no setting, flag or
policy field changes. Signalling the group ends the harness's descendants with
it: a tool subprocess, a language server, an agent's own in-flight command. A
harness that exits within the grace is never signalled.

**Cancellation while the harness runs.** A handled signal dispatch receives
while the harness runs cancels the run. For an interactive run dispatch sends
the same signal to the harness's group and to the harness, waits the
kill-grace, then sends SIGKILL. A confined run's group is killed at once, so
that a cancellation nested inside another supervisor's grace finishes inside it.

<a id="group-cleanup"></a>
**The group ends with the run.** The harness exiting does not end its group: a
tool that ignored the TERM, or one still running when the harness exited within
the grace or on its own, would outlive it. So in every run, interactive or
confined and whatever its ending, dispatch observes the harness's exit, which a
stop is not, without reaping it. While the unreaped harness still reserves the
group's ID, dispatch sends SIGKILL to what remains of the group, and sends it
again after a short pause, because macOS can let a member forked as the first
kill lands survive it. It then reaps the harness, whose exit or signal stays the
run's, and, signalling nothing more, confirms within 1 second that the group is
gone; only the system's answer that no such process group exists confirms it. A
descendant gets the time its harness gives it: a harness that wants a tool to
finish waits for it before exiting. A group still present after that second is
a **supervision failure**: dispatch appends its end observation as observed,
writes no ending file, says on stderr that members of the harness's group may
survive, and exits 5. A caller that acts on the ending file therefore never acts
beside a survivor in the group. Processes outside the group are outside this
contract, as they are outside cancellation's: one that deliberately left it, and
a harness's own command that the harness starts in a session of its own, as
Claude Code does its shell commands.

<a id="run-ending"></a>
**The run ending.** Once the harness is reaped, the run has one **run ending**:

| Ending | When |
|---|---|
| `cancelled` | Dispatch observed a handled signal before or at the reap sample |
| `exit_signal` | Otherwise, when the exit channel exists |
| `harness_exit` | Otherwise: the harness ended without the exit signal |

The **reap sample** is the latch read immediately after the wait returns,
before the duration reading, the reap observer, terminal recovery or group
confirmation. It consumes the latch even if an earlier cancellation is already
known, retaining that earlier signal. A signal delivered between the kernel's
reap and this adjacent read counts as cancellation; a later one belongs to the
invocation. This observable boundary replaces an unobservable physical ordering
between signal delivery and the kernel's reap.

Cancellation takes precedence, since the run was taken away whatever the session
said. The channel is looked for after the reap, so a session that signals and
exits at once still ends through the exit signal.

Dispatch then takes the terminal back and restores it and, once the harness's
group is confirmed gone, writes its **end observation** to the ending file when
the caller named one. It then appends the same observation to the run
([records](#records-and-outcomes)) and writes a one-line end notice to stderr
naming the run, its ending, the harness's exit or signal, its duration and whether
the observation was recorded, as one JSON line under `--json`. The ending file is published
before the append, so a store lock cannot delay the caller reading it. A handled
signal after the reap sample does not change the run ending: dispatch keeps
catching signals through recording and the notice, then re-raises a late signal
before returning the run's exit status. An earlier cancellation retains its own
signal. A group still present remains a supervision failure (exit 5), which
takes precedence over a late signal.
It ends:

| Ending | Dispatch's own exit |
|---|---|
| `harness_exit` | The harness's exit code, or death by the harness's signal |
| `exit_signal` | 0 when the escalation ended the harness or it exited 0; otherwise as for `harness_exit` |
| `cancelled` | Death by the cancelling signal |

A [supervision failure](#group-cleanup) exits 5 whatever its ending. A death by
a signal is reproduced by dying of that signal, without a core dump of
dispatch's own. A failed append or ending file is reported on stderr and changes
neither the ending nor the exit. A run that started no harness has no ending: a
refusal, a cancellation at the linearization point and a harness that could not
be started write no ending file and exit as [diagnostics](#diagnostics) state.

**The ending file.** `--ending-file PATH` names a path that must not exist, in a
directory that does, both checked before selection. Dispatch creates the file
exclusively and owner-only once the harness is reaped and its group is gone,
holding the end observation. It is how a caller tells a harness that quit with 0
from one ended through the exit signal, which dispatch's exit status cannot.

**When dispatch dies.** Dispatch catches every handled signal it did not inherit
ignored, so only an end it cannot survive — SIGKILL, an abort — leaves the
harness running. That harness is orphaned in its own process group, nothing
escalates it, and its run stays as it stood: an attempt whose execution is
unknown, with no end observation and no ending file. Its caller sees dispatch
dead of a signal, with no ending. A caller that handed dispatch the terminal
takes it back from the orphan's group, by the same rule dispatch takes it back
from the harness's when the harness died of a signal, and restores it. In the
background, that group can no longer read the terminal: a read fails with EIO
while the group is orphaned, and stops the reader where a subreaper in the same
session keeps it from being orphaned.

<a id="confinement"></a>
### Confinement

`run --confine` runs the harness under mandatory filesystem confinement, with no
unconfined fallback: Seatbelt on macOS, bubblewrap on Linux. Selection runs
before it and outside it, with the owner's settings, grants and bounds, and only
the harness and its descendants are confined. Writes are confined to the cwd,
which must be a directory other than `/`; to dispatch's private run directory,
which holds the harness's temporary directory; and to the exit channel's
directory. Reads are confined to installed system runtime resources, the
selected executable, dispatch's own executable, so that the harness can run
`harness-dispatch exit` by its path, and each `--runtime-read FILE`, a regular
file granted read-only, such as a harness's credentials. The policy, the owner
settings and the record store stay outside, and since a backend grants a whole
directory, dispatch refuses a grant that would reach them rather than carve
them out of it. Before selection it compares canonical paths: a granted path —
the cwd, the exit directory, the private run directory or a runtime read — that
is a protected path, contains one or lies inside one refuses with exit 2, naming
both. The protected paths are the directory holding the selected policy entry,
the owner settings file, the state directory and the canonical `--ending-file`
path when supplied. The ending file must remain outside writable grants so
the harness cannot supply the report its caller trusts. The two executables are exempt,
since the harness must run them. Dispatch also checks the backend's implicit
system-read grants against protected paths: owner data placed under a system
runtime tree refuses rather than becoming readable through that tree. Runtime
file names must be UTF-8 so the record preserves each grant exactly, and their
read-only permission holds even beneath a writable directory. A module the policy imports from elsewhere is
the owner's choice, as its authority is. The harness's environment is HOME,
USER, LOGNAME, PATH, LANG and the LC_* values, TMPDIR, TMP and TEMP set to its
private temporary directory, and the three reserved values; nothing else of the
front's reaches it. A confined harness runs noninteractively: it is handed no
terminal, and it starts in a POSIX session of its own with null stdin, writing
to the front's stdout and stderr. Its `argv[0]` is the resolved path, because
the sandbox launcher runs the path it is given.

A missing or unusable confinement backend, an unusable grant, a cwd of `/` or a grant that
reaches a protected path refuses before selection, and failing to establish
confinement launches nothing. Dispatch probes the backend by establishing an
empty sandbox around `/usr/bin/true`, capturing diagnostics; a failed probe
refuses as `confinement_unusable` without a run record. Preflight does not
guarantee subsequent grant setup or exclude intervening resource changes. The selected
harness's own permission flags cannot disable the outer boundary. The run
record notes the confinement and its grants.
[Standalone invocations](standalone-invocations.md) state what Grove builds on
it.

<a id="identity-and-creator"></a>
## Identity and original creator

A run may be associated with a **task identity**, a **reviewed artifact**, both,
or neither. The task identity is the caller's `--task-id`, which survives the
task's file being renamed, retired or reordered; Grove supplies its selected
handle. A reviewed artifact comes from caller or loaded context. Paths are
descriptive source locations only. Every run also has its own **run ID**: a
collision-resistant identity that dispatch allocates, records with the run and
exports to the harness as `HARNESS_DISPATCH_RUN_ID`.

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
| `run <run-id>` | Execution-recorded | The provider label the named run recorded at launch |
| `declared <provider>` | Declared | The owner's literal declaration |

A run reference is written by the session that ran under that run, which reads
its own `HARNESS_DISPATCH_RUN_ID`. Its provider comes from dispatch's record of
the command that run launched, never from the session's transcription or from
what today's policy would return. *Execution-recorded* describes that provider,
not the association. That the named run produced the artifact, and that it
executed, is the writing session's attestation, which dispatch cannot verify.
A session can reach other runs' IDs: other review bodies, version history,
`record show`, or an inherited variable when a harness started by hand runs
inside a dispatched session. A wrong but existing run lends its provider, and
launch does not detect it. A declaration remedies an artifact finished by a
session that ran without dispatch, which has no run to name. It is the owner's
assertion and is always reported as declared.

In Grove the reference is the review leaf's `**Creator:**` line, directly under
its `**Reviews:**` line. A session **finishes** a producer when it retires the
producer's leaf, and when its close cascade closes the producer's node, so one
session can finish several. For each producer it finishes, in its task's commit,
it owns the line on the review leaf it cuts and on every live review leaf that
already names that producer's handle. A review it cuts, it cuts before it
retires its leaf, with the `**Reviews:**` line written: the creator line goes
under that one, and a leaf cut inside a node afterwards would reopen a node the
cascade had closed. Under dispatch it writes
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

`harness-dispatch/examples/review` uses exact configured review-kind entries,
all of which apply the provider rule; other kinds use the owner's own table.
An entry maps each creator provider origin to the command that reviews it, so
the reviewer follows whichever origin made the artifact; an origin the entry
lacks refuses, naming the origins it lists. It exports its rule as a reusable selector over a
generic reviewed artifact, so non-Grove callers can apply it without a task
file. Custom review labels require an explicit example-policy entry: a context
that names a reviewed artifact under a kind with no entry refuses, the generic
form of the adapter's rule below. The core knows no review list and no
`Reviews` or `Creator` grammar.

The Grove adapter reads only the supplied task file, on every invocation. For a
configured review entry it requires exactly one standalone `**Reviews:**
<handle>` line and exactly one standalone `**Creator:**` line in either form,
and builds the generic reviewed artifact from them. A task that declares
`**Reviews:**` under a kind that is not a configured review entry refuses, so
an unlisted review kind cannot take an ordinary route. The adapter does not obtain
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
lookup and the selector over the Grove static example's commands and table. Its
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
match for one of the origins the review entry lists, with no normalisation. A
relabelled origin or a misspelt declaration therefore refuses with a correction
remedy instead of comparing as different. The command the entry gives for that
origin must then carry a different provider label, and one that carries the
creator's own refuses; a gateway label change cannot establish separation. The
policy never selects a replacement on failure. The same checks run on every
invocation and retry. The evidence class, the creator reference, the resolved
provider and the digest of the task file the reference came from appear in
inspection and in the review's run record. The rule binds only a policy that
imports the example: the sample pairs each review with the other provider by
its chosen arrangement, and consults no creator.

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

Before the harness starts, one committed transaction persists a collision-resistant run ID,
timestamp, task identity and reviewed-artifact association, kind, the
parameters, the selected command's provider, model and effort labels, its
resolved executable and argv, original
cwd, policy entry authority/digest/version, worker and adapter versions, context
source digests and sizes, effective limits, decision reason, selection timing,
the creator provenance used, and its confinement: `null`, or the runtime grants
of a confined run. Raw environment values are not stored. The
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
available to the harness via its environment. Inspection creates no run
and reports no run ID.

A committed run's launch fields never change, so a creator snapshot a review
used cannot go stale before that review's own handoff. Nothing registers a
creator, so there is no registration to revise. Readable but corrupt records
never become missing-provider defaults.

The store is `records.sqlite3` in the state directory, created on first use
with owner-only permissions. It names itself with a SQLite application ID and
a schema version. A store of another application, of another version, or that
SQLite reports as corrupt refuses with exit 4, and is never reset or replaced.
A run's launch fields are one document with its own version. Every field is
present in it, `null` where the run has no value for it. A release that
records something new writes it into new runs only, and a new table arrives by
a migration that only creates. The launch document is still version 1. A run
recorded under the catalog contract carries a candidate ID, a selection form
and an explicit choice; a run recorded under this one has no value for those
three and writes `null`, and it adds its parameters. A run recorded before
supervision has no confinement field and reads as unconfined. So every command
reads every form, and a review whose creator ran under an earlier contract still
resolves that creator's provider. Schema 2 adds the observations
table. A new store is created at version 2. A version-1 store is migrated by the
first observation appended to it, a `record observe` import or `run`'s end
observation, which creates that table inside the append's own transaction, so a
refused import or a failed end observation leaves the store at version 1.
Nothing else migrates: every other write leaves either version as it is. The
schema version does not vouch for the documents a store holds. So every read of
a run checks two things before anything is derived from it: that its launch
document is a version-1 object, and that any launch-failure detail names a
cause. The reads are `record show`,
`record observe` and a run lookup. `record show` also checks each observation it
reads, by the import's own validation, against its row's run, ID and
`supersedes`. A document this release cannot read refuses with exit 4. That
happens before any evidence is exported, or any import or migration commits.
Such a document never reads as a missing run, an unobserved measurement or a
confirmation. Only a run lookup, which types the kind, task identity and labels
into its answer, requires those fields too. The commit is one exclusive
transaction in a rollback journal at `synchronous = EXTRA`, the setting SQLite
documents as durable in that mode, with `fullfsync` on for macOS. That sync
reaches only the store's own directory. So on first use, the parent of every
record directory the invocation creates is synced before the commit, and a
failed sync refuses with exit 4 like the commit. The commit opens the store
only after the worker has been reaped.

| Evidence | Meaning |
|---|---|
| Proposal | Result of inspection; no persisted launch claim |
| Handoff attempt | Durable intent immediately preceding the harness's start; execution is unknown |
| Observable launch failure | The harness could not be started, or cancellation was observed after the attempt committed; appended to that attempt when possible |
| Execution confirmation | Dispatch's own end observation, or later evidence from an outside observer or the harness, says execution occurred |
| Outcome observation | Separately sourced acceptance, findings, repair, human work, duration, usage or other measurements |

There is no implied state transition from exit zero to task accepted. An
attempt whose dispatch was killed while its harness ran remains unknown unless
an outside observation says otherwise. Evidence describes the
command launched and the labels its owner gave it; it is never a claim to have
authenticated the actual backend model. A refusal before the handoff commit is a structured diagnostic
only: it creates no run and no record. Only a committed attempt can carry a
launch-failure detail.

`record show --run R --json` exports the run and its observations. The export
names the run's evidence: `handoff_attempt`, whose execution is `unknown`;
`execution_confirmed`, whose execution a current observation confirms; or
`launch_failure`, whose harness was `not_executed`. Dispatch's own
launch-failure detail comes first. Its `cause` is `exec_error` for a harness
that could not be started, with the errno, or `cancelled` for a signal at the
linearization point, with the signal. Each observation is exported as imported,
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

Dispatch's **end observation** is an ordinary observation of this kind, appended
in its own short transaction once the harness is reaped, under the same lock
wait as the commit, migrating a version-1 store as an import does. Its `source`
is `harness-dispatch`, its ID names the run, its evidence says that dispatch
supervised the harness to its end, and it observes `executionConfirmation`,
`ending`, `exit` and `duration`, measured from the harness's start to its reap.
An outside import can correct it like any other. A dispatch that dies before the
reap appends none.

Supported observation fields include execution confirmation, the run ending,
exit/signal, duration, input/output/total usage with units, acceptance
(accepted/rejected/unknown), missed defects, false findings, downstream repair,
human-work measures and evidence links. Every field has an explicit
observed/unknown/unobserved distinction; absence does not mean zero or false.
Findings can have stable IDs and references to later repair observations.
Optional routing probabilities distinguish `choiceProbability` from
`successProbability`; a success estimate must name its calibration data/version
or be labelled uncalibrated. The shipped policies emit neither, and dispatch
aggregates and scores nothing.

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
| `ending` | `exit_signal`, `harness_exit` or `cancelled` | none |
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
that the harness never started. The document is read from a regular file, within a
fixed 1 MiB bound.

<a id="grove-integration"></a>
## Grove integration

Grove runs harness-dispatch itself for every session it launches, and has no
launch configuration of its own. `run` launches a lifecycle session and a
standalone invocation alike. Grove reads no personal file, and a `config.kdl`
or `.grove.kdl` left on disk is never read and refuses nothing. There is no
`grove config` command, no task-body launch metadata, no Grove invocation
override, and no scraping of the prompt or a filename. Grove stores nothing for
dispatch and reads back from it nothing but a run's ending.

**Finding it.** Grove runs the `harness-dispatch` executable installed beside
its own, found from Grove's own real path, never from PATH or the cwd. The two
ship in one `bin/` directory, so the sibling is the matching release. A missing
or unexecutable one is a launch failure that names that path and how to install
the pair.

<a id="lifecycle-launch"></a>
**A lifecycle session.** For the leaf it selected, the driver runs
`harness-dispatch run` in the working-tree root. That is the location the prompt
assumes, so a policy reads the session's location from the request's `cwd`. The
selection inputs come from the same authoritative selection the driver composes
the mandate from:

| Passed as | Value |
|---|---|
| `--kind` | The leaf's open kind token |
| `--task-file` | The absolute selected task path |
| `--task-id` | The stable handle |
| `--prompt` | The mandate, unchanged |
| `--exit-dir` | This launch's launch directory |
| `--ending-file` | A file in that directory |

The last two are run mechanics and reach no policy. Grove passes no parameter,
and nothing else: no policy entry, bound, grant or record directory, which are
the owner's [settings](#owner-settings). Naming the session is the
methodology's, and a policy that must grant a secondary jj workspace's harness
its store derives it from the cwd's `.jj/repo`, as the [sample](#sample) does.

**The launch directory.** For each launch the driver creates a fresh
owner-only directory, named with 128 random bits, in its workspace control area
under `.jj/grove/`, and publishes its path to dispatch as `GROVE_LAUNCH_DIR`,
after scrubbing the loop-control variables it inherited. The harness inherits
it through dispatch; the worker never does. The
[session epoch](../adr/one-live-driver-per-working-tree.md) binds the path, so
the `grove-llm` tree verbs admit a session only while its launch is the current
one. The directory holds the exit channel dispatch allocates there, the ending
file and the session's teardown record. A session's own sandbox already writes
there, which is why Grove names it as the exit directory. The driver removes it
once it has read the launch; a replacement driver removes abandoned ones,
unread, after taking the lease and invalidating the old epoch.

**The process and terminal chain.** Grove's runner makes dispatch a foreground
job — a process group of its own, the terminal handed to it, default
dispositions for the terminal-generated signals
([the launched child is a job](../adr/the-launched-child-is-a-job.md)) — and
reports its start and reap to the [session witness](item-status.md). The worker
joins dispatch's job while the policy selects, and dispatch makes the harness a
job of its own, so a typed Ctrl-C cancels a selection in progress and, once the
harness runs, reaches the harness alone. The driver forwards its own TERM or HUP
to dispatch, which cancels the run, and waits up to 10 seconds, longer than
dispatch's kill-grace, record-store lock wait and group confirmation together,
before killing dispatch's group. Once dispatch is reaped, Grove's runner takes
the terminal back from whichever group holds it, dispatch's or, when dispatch
died, the orphaned harness's, and restores it
([supervision](#supervision)); the driver then resets it as it always has.

**Ending a session.** The prompt tells every session that it ends with
`harness-dispatch exit`, the [exit signal](#exit-signal), once its task is
retired and committed. A finish session that has torn the grove down first runs
`grove-llm record-teardown`, Grove's teardown verb. It is admitted under the
session epoch like any tree verb, refuses while `.grove/` still exists, naming
`finish-commit`, and otherwise creates the teardown record in the launch
directory; a record already there is success. Run without a launch directory it
records nothing and says so. `grove-llm complete` is gone. Once dispatch is
reaped and the epoch invalidated, the driver reads the launch:

| Seen, first match wins | The loop |
|---|---|
| The driver itself received TERM or HUP during the launch | Interrupted, ending by that signal |
| A teardown record | Finished |
| An `exit_signal` ending in the ending file | Relaunched on the next leaf |
| Anything else: the harness's own exit, a cancellation of dispatch, a refusal, dispatch's death, a supervision failure, a missing or unreadable ending file | Stopped, the leaf live |

A teardown record finishes whatever the run ending, so a finish session whose
harness exits on its own after recording still finishes. A run that ended
through the exit signal relaunches whatever the harness's status. Nothing else
relaunches, so the loop never relaunches onto a tree that a harness orphaned by
dispatch's death may still be changing; the stop says that such a harness may
have survived.

**A standalone invocation.** `grove run KIND` stages the invocation's private
working directory, then runs `harness-dispatch run --confine` there with the
kind, the invocation's whole prompt, each `--runtime-read` grant, and an
`--ending-file` in its own private directory outside the sandbox. A standalone
invocation has no task, so Grove passes no task file and no task identity, and
it passes no parameter. Grove runs dispatch noninteractively — in a session of
its own, with null stdin and its output going to the invocation's transcript —
in Grove's own environment with its loop-control variables removed, so the
owner's grants and bounds apply to selection. The prompt tells the harness to
run `harness-dispatch exit`, by the canonical path of the dispatch beside Grove,
once every required output is written and checked. Grove forwards its own
cancellation to dispatch as the driver does. It publishes the declared outputs
only on an `exit_signal` ending with dispatch's exit 0 and no cancellation of
its own; any other end publishes nothing. The invocation has a run record and
its harness a run ID. [Standalone invocations](standalone-invocations.md) own
staging, outputs and publication.

**A refusal.** A kind the policy does not route is caught when its leaf
launches, and nowhere earlier. Tree verbs and root scaffolding consult no
policy, so a session can cut a leaf of a kind that will refuse. A refused
launch prints dispatch's own diagnostic, with its remedy and the equivalent
`inspect` invocation, on the terminal the session would have had. Grove reports
it as it reports any session that ends without the exit signal: the kind, the
handle and the exit status, a pointer to the diagnostic above, and that the
loop stopped. The leaf stays live, and rerunning `grove` continues from it. A
refused `grove run` leaves dispatch's diagnostic in its transcript and
publishes nothing.

**The runner.** `keyed-launch` stays a unit with no Grove or dispatch knowledge,
and both use it. Dispatch runs the harness through it: the job and terminal,
the exit channel, the escalation and the confinement. Grove runs dispatch
through it: the job and terminal, launch events, forwarded cancellation, and the
noninteractive mode. Grove allocates no channel, applies no escalation and
confines nothing. Decision 7 of the [module decomposition](module-decomposition.md)
states its surface.

**A meta-grove.** This repository's suite runs inside a live session, so the
cargo guard that cleared the retired channel variable clears `GROVE_LAUNCH_DIR`
and `HARNESS_DISPATCH_EXIT_FILE` for everything cargo runs: a test that
inherited either could record a teardown for, or end, the session it was typed
into.

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

An owner starts with `harness-dispatch init`, or writes a policy and imports a
shipped example by its package specifier. `harness-dispatch inspect` is the one
inspection surface. Grove's usage guide and the configure-grove skill explain
installing and editing the policy, the owner settings, the choice file,
inspection, the launch-time boundary and a refused launch with its remedy, and
warn against granting `GROVE_LAUNCH_DIR` to policy. With the dispatch README
these documents also explain the two `**Creator:**` forms, who writes or
removes the line, the declaration remedy, why a wrong but existing run is not
detected at launch, how a review attaches its findings, and how a session ends
its run. Each invocation they quote is one `harness-dispatch --help` carries or
Grove's launch-boundary suite makes.

<a id="diagnostics"></a>
## Diagnostics and exits

Before handoff, errors carry a stable code, stage, message, relevant input/source
and remedy. `--json` data-command failures use one JSON error on stderr and no
partial stdout object; policy diagnostics are captured and included separately,
never interleaved with protocol or structured output. Text mode prefixes policy
diagnostics on stderr. A failure never launches another command. A refused
`run` also reports the equivalent `inspect` invocation, as a command line in text
mode and an argv array in JSON, each with the directory it was run from: the
same selection inputs, parameters included, `--policy-env` names but no values,
and no prompt. An owner diagnosing an unattended refusal can then reproduce the
selection without reconstructing its inputs. The report says that the prompt is
left out, and that a policy which reads the prompt selects as it did only when
the same prompt is added. A missing policy refuses as `policy_missing`, and its
remedy names `harness-dispatch init`. When an input, the program path or the
directory is not UTF-8, no JSON string or text command line holds it exactly. A lossy copy
would name other inputs, so the invocation is reported as unavailable, naming
what cannot be written. A command line that cannot be parsed has no
equivalent. Once a command line parses, its own `--json` flag chooses the
format. A `--json` word that is the value of another flag, such as the
prompt, is data.

Exit codes before the harness starts are 2 for malformed CLI input, an excluded
`--policy-env` name and a confinement grant that reaches a protected path
included, 3 for policy/context/
selection refusal and for a refused record lookup or observation, 4 for
required-record or record-store failure, 5 for worker/protocol/internal
failure, 124 for timeout, 126 for an unexecutable selected program, and 127 for
one not found. A harness that cannot be started after resolution exits 127 for
`ENOENT`, including a missing `#!` interpreter, and 126 otherwise. INT/TERM/HUP cleanup ends by
restoring and re-raising that signal. Its refusal, `selection_cancelled`, names
the signal, and its `exit` is the `128 + N` a shell reports for that death.
After the commit the refusal is `handoff_cancelled`, stage `exec`, which also
names the run and whether its not-executed detail was recorded. A
signal received during evaluation decides the outcome, whatever else the
selection came to; a timeout is exit 124 only when no signal was received.
Once the harness has started, dispatch's exit reports the run as
[the run ending](#run-ending) states, and a harness's own code may numerically
coincide with a preflight code. Structured diagnostics, the recorded stage and
the ending file distinguish these cases, not a globally reserved harness exit
range. `exit` exits 0 when it signalled or had nothing to signal, and nonzero,
naming the path and the error, when it cannot create the channel.

<a id="delivery"></a>
## Delivery and release

The worker is compiled with Bun 1.4.2, pinned together with the worker's
dependencies and build identity; an upgrade reruns the cases the
[runtime evidence](../design/harness-selection-and-execution/runtime-evidence.md)
records. Workers are compiled for macOS arm64, Linux arm64 and Linux x64. Bun
1.4.2 ships a single x64 build targeting the Nehalem microarchitecture; its
`baseline` name is an alias. Rust and bundled SQLite build for Grove's
corresponding targets at its glibc 2.17 floor. The worker is an
installed private companion, not a runtime downloaded on invocation. It is found
relative to the real installed front executable, including through a Homebrew
symlink, at `../libexec/harness-dispatch/harness-dispatch-policy`. The SDK's
declarations and readable source sit beside it in `sdk/`, the Grove adapter's
in `grove/`, and each example's and the sample's in `examples/`. The front
carries the sample's text itself, so `init` writes it without the worker. The
supported portable
archive preserves the same relative layout: its top directory is an
installation prefix, with Grove's own executables beside the front in `bin/`.
A cross-compiled worker is its target's Bun runtime with the policy host
appended, so that runtime ships. Bun would fetch it unverified; the build
instead fetches each target's pinned runtime itself, refuses one whose digest
differs, and hands it to the compiler, so no release builds against an
unpinned runtime.
The Rust package builds independently; running selection additionally needs the
matching compiled worker, supplied by the package's build/install task.

Each release archive carries the front executable, the private worker with its
embedded SDK, Grove adapter and inactive examples, their type declarations and
readable sources, the sample policy, and runtime/license notices. The Homebrew formula installs
that layout and checks matching versions. Source-development tasks build and
install the pair; a plain cargo install of the Rust package alone is not a
complete installation. No host runtime is ever used as a fallback. The Rust
package inherits the workspace version and takes no cargo-release cut of its
own; the worker embeds that same package version.

The Taskfile has the package's check, build and install tasks and the release's
smoke task, and the repository check includes the package checks.
Archive-content assertions and an installed-layout smoke test run on **each**
supported target: macOS arm64 natively, and each Linux target at the floors
below. The smoke test installs the sample with `init` and inspects it, and runs
a policy that consults a table, a policy that starts a child and uses its
answer, and a policy importing one package declared by `main` and one by
`exports`, with fake harnesses and no separately installed runtime. A matching runtime archive is
not evidence for a target; only that test executing there is.

The Linux floor has three dimensions, and each is claimed only as far as its
instrument observes it:

| Dimension | Floor | Instrument |
|---|---|---|
| C library | glibc 2.17, Grove's own floor | Installed smoke tests run in a glibc-2.17 userland on each Linux target: under the CPU instrument's user-mode QEMU, and first as a container where Docker runs that architecture natively |
| CPU | x64 Nehalem; arm64 at the Cortex-A53 level | The same tests under user-mode QEMU with that CPU model. A probe executing one instruction beyond it (AVX2; an Armv8.1 atomic) must be killed there and run under `-cpu max`, and every ELF file in the archive must match the emulator's registration |
| Kernel | Bun's documented support | None: a container or user-mode emulation runs on the host's kernel |

The release task runs the archive-content assertions and these instruments over
the archives it is about to publish, and publishes nothing unless every one
passes.

Bun 1.4.2's documents disagree about the kernel. Its README gives a 5.1 minimum,
while its installation page says Bun runs on 3.10 (RHEL 7) with degraded newer
syscalls. The dispatch README and the release procedure state that documented
range, labelled documented rather than executed, and each Bun upgrade rechecks
it. This is an accepted trade-off:
RHEL 7-era kernels are claimed through Bun's documentation, not tested. A
cross-link or an ELF-symbol inspection alone satisfies none of the dimensions.
On macOS the worker additionally inherits Bun's documented macOS 13.0 minimum.

<a id="test-seams"></a>
## Agreed test seams and acceptance

The human agreed four seams during requirements, and each existed before this
design: dispatch's command suite, its record suite, Grove's driver suite and
Grove's standalone suite. They are the public acceptance instruments; internal
tests support them and never replace them, `keyed-launch`'s own suite among
them, whose interface cases the first seam absorbs. Every Grove test that
launches a session goes through the real front and its compiled worker, so the
built worker is a precondition for those tests. There is no stand-in for
`harness-dispatch`, and no test calls a model.

| Seam | Required observable cases |
|---|---|
| 1. Dispatch's command, temporary policies, fake harnesses and a controlling PTY | **Selection.** Independent kind/context use with no Grove files or binary; optional task; the prompt, every parameter and the caller's cwd reach `select` unchanged, and a command it builds from them is the argv launched, whole and directly, literal punctuation and newlines included; a repeated or malformed parameter refuses; a scripted stand-in for a deciding agent, started by the policy, receives the prompt in the caller's directory, and its answer decides which command launches; a stand-in still running at the selection bound launches nothing; complete inspection including the `command` object, measured sources, authority and where each bound came from; inspection without a prompt shows the marker; owner settings set the time bound, the context budget, the record directory and a grant with no flag passed, a flag replaces each, and a malformed settings file refuses; a version-1 policy, policy errors, bad imports, malformed results, missing context, limits and an unavailable program launch nothing. **The sample and examples.** `init` writes the sample into an empty HOME and refuses beside an existing policy, and `run` and `inspect` with no policy refuse naming it; the installed sample selects, for every kind under every selection it offers, the program and arguments recorded from the configuration it converts, with no parameter passed; run from a secondary jj workspace it grants the store its `.jj/repo` names, and from a primary one nothing more, and it names no session; a choice file selects among the sample's offered names, an unoffered name refuses and an absent file leaves the default; different-origin reviewer on every invocation and retry; same-origin/gateway disguise refuses; a fake producer launched through dispatch writes its `Creator` line from `HARNESS_DISPATCH_RUN_ID`, and the dispatched review of that task file uses the named run's recorded provider, which a changed policy cannot rewrite; a store holding an earlier run of the same task identity does not satisfy a review task with no `Creator` line; an unknown run and a run marked not executed refuse; declaration adoption; missing, duplicate or malformed `Reviews`/`Creator` lines refuse; `Reviews` under a kind that is not a configured review entry refuses; a relabelled origin and a misspelt declaration refuse as unlisted; the generic reviewed-artifact form selects without a task file. **Authority and lifecycle fixtures.** Hostile cwd policy, dotenv, bunfig/preload, a dotenv file where a policy starts a `Worker`, tsconfig, a cwd `package.json`, package shadow, a package and a `package.json` between the worker's start directory and `/`, one that never yields its content included, BUN_OPTIONS, an altered runtime transpiler cache and the caller's resolver and IPC channel variables stay inert through the public launcher, each beside its firing configuration below; the front starts its worker in an owner-only empty directory and removes it, and policy code runs in `/`; explicit relative config and personal import are admitted; a package beside an entry loads by the entry point its `package.json` declares, with `main`, `exports`, a subpath, a condition or its own `imports` map, and a granted `NODE_ENV` chooses no condition; an entry's own `package.json` supplies its `imports` map and answers its own name ahead of a nearer `node_modules`; the nearest `package.json` that reads as one is a module's own, named or not, and one that does not is passed over for the next above; a missing package is not installed beside a `package.json` that lists it; a documented package specifier resolves to the embedded module, and an `imports` alias to one is a missing package or the package beside it; the worker and a child it starts lack every `HARNESS_DISPATCH_*` value and the caller's control values; structured diagnostics stay clean; import/loader/callback interruption and timeout launch nothing. **Supervision**, absorbing the runner's interface cases. The harness runs in its own process group holding the terminal, and the terminal comes back to dispatch with its modes restored, even after a raw-mode fake is killed by the escalation; dispatch takes the terminal back from the harness's group alone when the harness exits rather than dying of a signal, leaving the foreground with any other group then holding it, and from whichever group holds it, its own and the session leader's excepted, when the harness died of a signal (these two other-group cases are runner-level PTY cases, `keyed-launch/tests/job.rs`); the entry signal mask and dispositions, SIGPIPE's and a caller-ignored HUP's included, reach the harness, and none of dispatch's own handlers does; a signal the caller blocked, sent to dispatch before the spawn, stays pending in dispatch, and the harness starts with it blocked and not pending; a typed Ctrl-C reaches the harness alone, and during selection dispatch and its worker alone; the exit signal drives grace → TERM → kill-grace → KILL on the harness's group, reaping a TERM-ignoring fake and its descendants, and a fake that exits within the grace is not signalled; the harness's group ends with the run, so a TERM-ignoring descendant is gone before the ending file is written whether its fake exits on the TERM, within the grace or on its own, and the fake's own exit or signal stays the run's; a fake stopped by SIGSTOP is neither reaped nor killed as though it had exited (runner-level case, `keyed-launch/tests/launch.rs`); a caller that ignores SIGCHLD still has its fake supervised to an ending, and the fake receives SIGCHLD ignored; the harness's own exit code or signal is dispatch's when it exits on its own; each of the three endings is reported in the ending file with the harness's exit and duration, and gives the exit status its row states; every run gets a fresh exit channel, so a channel left by an earlier run ends nothing, and the channel is allocated in `--exit-dir` when given; `exit` with no channel is a no-op that exits 0, and a nested run's harness ends only the nested run; a handled signal after the linearization point and before the reap sample ends the run as `cancelled`, the harness's group reaped; an existing ending file and a missing exit directory refuse before selection. **Confinement refusals.** `--confine` refuses before selection with exit 2 when the cwd holds the policy entry, when the exit directory holds the state directory, when a runtime read names the settings file, and when the cwd reaches the policy's directory through a symlinked alias; a cwd beside those paths is admitted |
| 2. Dispatch's records | Required commit failure prevents launch; attempted handoff and a harness that could not be started stay distinct; cancellation after the commit launches nothing and marks the attempt not executed; pre-commit refusals create no run; after a supervised run `record show` carries dispatch's own end observation, confirming execution with the run ending, the exit and the duration, and the run reads `execution_confirmed`, also against an existing version-1 store, which the end observation migrates with no import between; a dispatch killed while its harness runs leaves the attempt as it stood, with no end observation; outside imports still append to and correct any run, a supervised one's end observation included, with idempotency and conflicts as before; round-trip run lookup; policy run lookup returns immutable launch fields, reads no observation history, and an unreadable store refuses; a run recorded under the catalog contract, and one recorded before supervision, is shown, observed and looked up, and a review resolves its provider; a stored launch record or observation this release cannot read refuses every read of it; the review's run records the creator provenance used; a confined run's record notes its confinement and grants; later observations after tree teardown |
| 3. Grove's launch boundary: the real driver, front and worker, a fake harness and the policy in a temporary HOME, under a PTY | The mandate, kind, task file and handle reach `select` as native data, no parameter reaches it, and its `cwd` is the working-tree root, with no Grove configuration file anywhere; a `config.kdl` and a `.grove.kdl` left on disk, valid, invalid or tracked, change nothing about a launch; tree verbs and root scaffolding succeed with no policy installed; a kind the policy refuses leaves its leaf live and the loop stopped, its diagnostic on the terminal, and once the policy is corrected `grove` launches that leaf; a missing `harness-dispatch` beside Grove is reported with its path; **the endings**: the exit signal relaunches; a teardown record then the exit signal finishes; a teardown record then the harness exiting on its own finishes; the harness exiting on its own stops with the leaf live; dispatch killed while a raw-mode harness holds the terminal stops with the leaf live, and while that orphaned harness still lives the driver's group holds the foreground again with the modes it handed over restored; a driver started in the background takes no foreground; `record-teardown` refuses while `.grove/` exists; the driver's own TERM interrupts the loop through dispatch with the harness's group reaped and the terminal restored; **staleness**: a stale session's tree verbs and `record-teardown` refuse after the epoch rotates, and its exit signal cannot end a later launch; each launch directory is removed after its launch, and an abandoned one by a replacement driver; **creator**: the harness receives `HARNESS_DISPATCH_RUN_ID`; retiring and reordering the producer between its launch and its review's leaves the review's creator unchanged; a pre-cut review of a decomposed producer carries the run whose retirement closed it through a multi-level close, and selects although that run's task identity is the child's; a close cascade names its run on every live review of each node it closes, one nested in another node and one the closing session cut included, and on no terminal review and no review of another producer; a finish by a session with no run removes a stale `Creator` line from the pre-existing review, planted by a dispatched attempt that named its run without finishing, and that review refuses with the declaration remedy, which then admits a different-origin reviewer; a review attaches an observation to the run its line names after `.grove/` is removed |
| 4. `grove run` under real confinement, a deterministic harness | The policy selects the standalone kind outside the sandbox, and a variable the owner granted is visible to it; the harness runs inside the sandbox with the invocation's prompt and a recorded run ID, cannot read the personal policy, the owner settings or the record store, and cannot write outside its working directory and dispatch's run directory; the file dispatch resolved is the file confined and run, whatever else of that name PATH or Grove's own directory holds; outputs publish on the exit signal with a clean end and never on the harness's exit alone, nor when it fails after signalling; a harness that signals and exits cleanly leaving a writer in its group has that writer stopped before any output publishes; a refused selection publishes nothing, and a signal during selection or during the run launches or publishes nothing and leaves no confined process running |

Per-target release delivery adds the archive/install tests above. Documentation
review verifies installing the sample, the owner settings, the choice file,
inspection, the deciding-agent warning, the `Creator` line conventions and
their remedies, how a session ends its run, later outcome entry and launch-time
validation guidance. Fake producers in the lifecycle cases follow the documented
convention, including removal without a run, the node-close step and cutting a
review before retiring. One does not, by design: the attempt that plants the
stale line names its run while its leaf is live, which the convention forbids.
A real session's compliance is the methodology's to check, through the
conformance rows that ship with the convention, not these seams'.
Each hostile class has a named firing configuration, a positive control that
must be seen to fire, so a test cannot pass merely because its fixture never ran:

| Class | Firing configuration |
|---|---|
| cwd policy entry | The same file named by an explicit relative `--config`, which loads it |
| cwd `.env` and bunfig preload | A default-autoload probe build of the same worker, run in the hostile directory |
| `BUN_OPTIONS` preload, and `BUN_BE_BUN` | The shipped worker launched directly with the variable set, bypassing the front process's scrubbing |
| `node_modules/harness-dispatch` shadow, in three layouts: files in `node_modules`, a `node_modules` package whose `exports` declares them, and the entry's own package named `harness-dispatch` | Each fixture beside an admitted entry, under a probe build that does not register the virtual modules |
| cwd `package.json`: its `imports` map, its own name, and a package in a `node_modules` beside it | The same import from an entry in that directory, named by an explicit relative `--config` |
| tsconfig `paths` | The fixture beside an admitted entry, under a probe build with tsconfig autoloading enabled |
| `.env` in the directory the process is in when a policy starts a native `Worker` | A default-autoload probe build of the same worker, started in an empty directory, whose policy moves into the hostile directory before it starts the `Worker` |
| `node_modules` package between the worker's start directory and `/`, for a module with no file location | The fixture in the caller's TMPDIR, under a probe build that does not move to `/`, started in a directory under that TMPDIR |
| `package.json` between the worker's start directory and `/`: its `imports` map, its own name and a package its `main` declares, for a module with no file location; and one that never yields its content, a link to a FIFO, for any policy | The fixture in the caller's TMPDIR, under the same probe build started in a directory under that TMPDIR. It answers such a module. It opens the link, which the test sees as a reader on the FIFO, and it is still running and has reported nothing when its time is up. Beside a `package.json` it can read, it loads the same entry |
| Runtime transpiler cache under HOME or `XDG_CACHE_HOME` | The shipped worker launched directly without the front's cache setting, after its own cache entry for the policy has had its output altered |
| `NODE_PRESERVE_SYMLINKS`, and `NODE_CHANNEL_FD` | The shipped worker launched directly with the variable set: a helper reached through a directory symlink resolves its bare import beside the link, and a module's `process.send` writes Bun's IPC message where the policy frame belongs |

`task dispatch:probes` builds the probe builds from the shipped source, each
with one control removed. A probe reports the identity
`probe-<name>-<source digest>`, which no front accepts, so a probe can never
serve as an installation's worker, and an archive carrying one there fails the
installed smoke test before a release publishes. A firing configuration
therefore drives its probe directly, playing the front's side of the protocol.
Probe builds are never built by a release and never ship.

A class with no known firing configuration is reported as such, not counted as a
passing control. On Bun 1.4.2 these are a `~/.bunfig.toml` preload, tsconfig
`paths` in the caller's cwd, and two in a directory the worker moves into after
it has started, which is what `/` would need to hold: a `.env` for the VM that
made the move, and a bunfig preload for that VM or a `Worker` started there.
None fires under any build. A test keeps checking that each still fails to fire
under its most permissive probe, and fails if one gains a firing configuration.
For the moved-into and tsconfig classes the test's policy makes the move: every
worker but one probe leaves the caller's cwd, and `/` can hold no fixture.
A package or `package.json` above the worker's start directory answers a
module in a file under no build, since a file's imports resolve from the file.
Its case asserts that under the probe that does not move, beside the same
fixture firing for a module with no file location.
Missing-source fixtures carry the same obligation. The runtime
evidence records which of these have been seen to fire. Do not infer backend
identity, policy quality or task acceptance from these mechanics tests.

<a id="out-of-scope"></a>
## Out of scope

Any agent or model-calling policy and its evaluation pilot, model
comparisons/calibration, analytics and a learned-selector benchmark, repository
extraction, accumulated contributor exclusions, automatic fallbacks, partial
overrides, automatic retries, automatic creator guessing, lookup of runs by task
or artifact identity, cross-checkout discovery and post-teardown artifact lookup
are not part of harness-dispatch. Windows is outside Grove's release targets. A
hostile-code sandbox is outside this command's contract.

Deliberately absent, each considered and declined:

- **A catalog, and a caller input that names a choice.** The owner's function
  returns the command, and a caller that wants to steer passes a parameter the
  policy chooses to read.
- **A converter from Grove's former configuration, a refusal while one of its
  files is present, and a check of a kind's route before its leaf is written.**
  The sample is the conversion, old files are ignored, and no early check could
  cover a deciding agent.
- **Installing a policy implicitly.** Only `init` writes one.
- **The command discovering a checkout's file itself.** The choice file is read
  by a helper the owner's policy calls.
- **Owner settings that differ by kind.** The
  [policy ownership decision](../adr/harness-selection-is-owned-by-policy.md)
  records why it was declined and what would reopen it.
- **A verb that keeps the `exec` handoff, graces an owner or caller sets, and a
  parent-death signal for the harness.**
  [Dispatch supervises the harness](../adr/dispatch-supervises-the-harness.md)
  records why each was declined and what would reopen it.
- **Relaying job-control stops.** A harness that stops — on a typed Ctrl-Z
  while its terminal generates signals, by suspending itself, or on a read from
  the background — keeps the terminal, because no supervisor in the chain waits
  for a stop, as Grove's runner never has. Likewise, a launch that hands the
  terminal down late, after `grove &` then `fg`, continues no child that stopped
  while in the background. The handover gate also tests the launcher's
  terminal, not that its stdin is that terminal, so a launcher started with
  piped streams inside a foreground job still takes the foreground. Each
  predates supervision. Reopen with a harness that suspends itself, or a
  launcher started inside another's foreground job: the answer is a shell's,
  where each supervisor takes the terminal back and stops itself when its child
  stops, and on continuing hands the terminal back and continues its child.
- **Detecting a harness that returned to its prompt without the exit signal.**
  Such a run stalls until a human ends it, as before: nothing dispatch can
  observe tells a forgotten signal from work in progress.
- **An after-run policy hook that reports a harness's usage.** Usage stays with
  outside observers through `record observe`.
