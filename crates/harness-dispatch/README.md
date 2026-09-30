# harness-dispatch

`harness-dispatch` evaluates an owner's TypeScript selection policy, reports
which harness, model and reasoning effort it chooses for a session kind, and
runs that harness. It is independent of Grove: a caller supplies a kind, a
prompt and optional task data, and the owner's policy supplies the choice. The
contract is the
[area specification](../../docs/specs/harness-selection-and-execution.md).

This release delivers **inspection and running of a static `routes` policy
or a computed `select`**. `inspect` reports the choice, the harness's expanded
arguments and the program that would run. `run` makes the same choice, commits a durable
record of the handoff with a fresh run ID, and then replaces itself with the
harness. `record show` exports what a run recorded. A caller can name one
configured candidate with `--choice`, which a routes policy takes instead of
the kind's route and a `select` policy accepts or refuses. Selection is bounded
in time, so a policy that never finishes is stopped and nothing runs. Every
refusal says what to fix, and a refused `run` gives the `inspect` command that
reproduces it. Three starter policies ship inside the worker, two static and
one computed. Signal handling at the handoff, task context and later
observations come in later releases. Until each arrives, its input is refused
by name. It is never accepted and ignored.

## Install

Grove's Homebrew formula and release archives install harness-dispatch with
Grove:

```sh
brew install linkuistics/taps/grove
harness-dispatch --version
```

A release archive unpacks to an installation prefix holding `bin/` and
`libexec/`; keep the two together and put `bin/` on `PATH`. The notices for
the Bun runtime inside the worker and the SQLite inside the front are in
`libexec/harness-dispatch/notices/`.

## Supported platforms

Releases carry harness-dispatch for macOS on Apple silicon, Linux arm64 and
Linux x64. Each floor is claimed only as far as something observes it. Before
every release, the installed pair runs from each archive at the Linux floors
([Releasing](../../docs/RELEASING.md#installed-smoke-test)).

| Floor | Minimum | Basis |
|---|---|---|
| Linux C library | glibc 2.17 | Executed, in a CentOS 7 userland |
| Linux CPU | x64: Nehalem (SSE4.2); arm64: Cortex-A53 (Armv8.0) | Executed, under QEMU emulating that CPU |
| Linux kernel | Bun 1.4.2's documented range: 5.1 in its README, 3.10 (RHEL 7) on its installation page | Documented, not executed |
| macOS | 13.0 | Documented by Bun 1.4.2, not executed |

The kernel is Bun's claim, not this project's: containers and user-mode
emulation both run on the host's kernel, so neither can observe an older one.
Bun's two documents disagree. Its README gives 5.1 as the minimum, while its
installation page says Bun runs on kernels as old as 3.10, degrading newer
system calls gracefully. Each Bun upgrade rechecks both.

## Install from a checkout

```sh
task dispatch:install                   # into ~/.local/bin and ~/.local/libexec
task dispatch:install PREFIX=/opt/dispatch
```

The installation is a pair. `bin/harness-dispatch` is the front executable.
`libexec/harness-dispatch/harness-dispatch-policy` is a private worker compiled
with Bun 1.4.2, which carries its own runtime, so no system Bun or Node takes
part. The front finds the worker only at that path relative to its own real
location, following symlinks. PATH, the current directory and environment
variables play no part. The front checks the worker's protocol, package version
and source digest before it sends any policy. A missing worker or one from
another build refuses with exit 5.

Building needs Bun 1.4.2 and Task. `task dispatch:worker` compiles the worker
into `target/libexec/harness-dispatch/` for the checkout's own `target/*/`
binaries. `task dispatch:check` runs the package checks. Cargo never builds the
worker, so after editing its TypeScript, run `task dispatch:worker` again. The
front refuses a worker built from other source rather than using it.

`scripts/installed-smoke.sh PREFIX VERSION` checks an installation from itself.
It inspects and runs a TypeScript policy with a fake harness through
`PREFIX/bin/harness-dispatch` and through a symlink to it, then reads the run
back. VERSION is the version both must report. Run it with no Bun or Node on
`PATH`, such as `env -i PATH=/usr/bin:/bin bash scripts/installed-smoke.sh
~/.local "$VERSION"`. Grove's `task release:smoke` runs it on every release
target.

## Which policy runs

The personal default is `~/.config/harness-dispatch/policy.ts`. `--config PATH`
names another entry instead, and a relative path resolves against the current
directory. Nothing else selects a policy. There is no search of the current
directory or its parents, no environment variable and no repository override,
so entering a repository runs none of its code. Naming a file there with
`--config` is your explicit choice, and inspection reports it as explicit
authority. Your personal policy may itself import a repository entry. That
import is also your choice, and the imported code runs with the same trust.

A missing, unreadable or invalid entry refuses and names the path. So does an
entry whose resolved path is not UTF-8 or contains `?`. The worker imports the
path as a string, which would then name another file: its runtime reads
`policy.ts?x` as `policy.ts` with a query. Nothing is
installed on the policy's behalf. Relative imports resolve from the importing
file, bare imports resolve through `node_modules` beside it, and a missing
import refuses.

## A routes policy

```ts
import { definePolicy } from "harness-dispatch/sdk";

export const policy = definePolicy({
  schemaVersion: 1,
  version: "2026-09-30",
  catalog: [
    {
      id: "deep",
      provider: "anthropic",
      model: "claude-opus-5-5",
      effort: "high",
      program: "claude",
      args: ["--model", { slot: "model" }, { slot: "prompt" }],
    },
  ],
  routes: { design: "deep", impl: "deep", "review-impl": "deep" },
});
```

The module exports one plain object named `policy`:

- `schemaVersion` is `1`, and `version` is your own nonempty label, which
  inspection reports.
- `catalog` lists joint candidates. Each has a unique `id`, a `provider` naming
  the model's origin, and nonempty `model`, `effort` and `program` strings.
  Each `args` entry is a literal string or a slot object such as
  `{ slot: "prompt" }`. None of the strings that can become an argument
  (`program`, `model`, `effort` and the literals) may contain a NUL character,
  which no argument can carry.
- `routes` maps each kind to a candidate ID, exactly. There is no catch-all, no
  inheritance and no fallback. A kind the table does not name refuses as an
  incomplete mapping.

Every candidate in the catalog is checked, including those no route names, so a
mistake anywhere refuses rather than waiting for the kind that would reach it.

## A select policy

```ts
import { definePolicy } from "harness-dispatch/sdk";

const claude = { provider: "anthropic", program: "claude", args: ["--model", { slot: "model" }, { slot: "prompt" }] } as const;

export const policy = definePolicy({
  schemaVersion: 1,
  version: "2026-09-30",
  catalog: [
    { id: "deep", model: "claude-opus-5-5", effort: "high", ...claude },
    { id: "quick", model: "claude-haiku-4-5", effort: "low", ...claude },
  ],
  async select(request) {
    if (request.explicitChoice !== undefined) {
      return { status: "selected", candidateId: request.explicitChoice, reason: "the caller chose it" };
    }
    if (request.kind.startsWith("review")) {
      return { status: "selected", candidateId: "deep", reason: `kind ${request.kind} is a review` };
    }
    return {
      status: "refused",
      code: "unrouted",
      message: `no candidate is configured for kind ${request.kind}`,
      remedy: "add the kind to select, or name a candidate with --choice",
    };
  },
});
```

Instead of `routes`, a policy may export `select`, a function that computes the
choice on each invocation. A policy has exactly one of the two, and both or
neither refuses as `policy_invalid`. `select` may be synchronous or `async`,
and may do whatever trusted TypeScript can, within
[the selection bound](#the-selection-bound). It is called as a method of the
policy, and only once harness-dispatch has accepted the policy, catalog
included, so an invalid catalog refuses without running it. Its one argument is
the request:

| Field | Value |
|---|---|
| `schemaVersion` | `1` |
| `kind` | `--kind` |
| `cwd` | The caller's current directory, as data. The worker does not run there. |
| `taskFile`, `taskId` | `--task-file`, as an absolute path, and `--task-id`. Each is absent when not given. |
| `explicitChoice` | The `--choice` ID, absent when not given. It is always an ID the catalog has. |
| `limits` | `{ selectionMs }`, the effective whole-selection bound in milliseconds |

The prompt is never part of it. Loaded context, `loadContext` and `--context`
come in a later release, and are refused until then.

`select` returns, or resolves to, one of two results:

- `{ status: "selected", candidateId, reason }` names a candidate in the
  catalog, with a nonblank reason. Inspection and the run record report the
  reason exactly as given.
- `{ status: "refused", code, message, remedy }`, each nonblank, refuses for
  the policy's own reason. It is reported as `policy_refused`, exit 3, with
  your code as `policyCode` and your message and remedy.

Nothing else is a result. Each other outcome refuses with exit 3 and a code of
its own, and launches nothing:

| Outcome | Code |
|---|---|
| `select` throws, or its promise rejects | `selection_threw` |
| Its promise is still pending when nothing is left running that could settle it | `selection_unsettled` |
| It returns `undefined` or `null` | `selection_abstained` |
| The result is neither shape: a missing or blank field, an unknown `status`, or any field beyond its status's own | `selection_malformed`, with a `location` such as `result.reason` |
| It names an ID the catalog does not have | `unknown_candidate` |

A result names a candidate and nothing more. It cannot supply a program or
arguments, and it cannot add a candidate: the catalog it is checked against was
taken when the policy loaded, before `select` ran. A promise that live work
keeps pending, such as one waiting on a timer that never resolves it, and a
synchronous loop, are stopped instead by the selection bound, with exit 124.

With `--choice`, `select` must decide on the caller's choice: select that same
ID to accept it, or return a refusal. Any other ID refuses as
`explicit_choice_mismatch`, even one the policy's reason calls a fallback. An
ID the catalog does not have refuses as `unknown_choice` before `select` is
called.

A policy that wants exact routes for most kinds and computation for a few
exports `select` and consults its own table. Its reason should name the entry
it applied, as the dynamic example's does. The types for all of this,
`SelectPolicy`, `SelectionRequest` and `SelectionResult`, are in
`harness-dispatch/sdk`, and `definePolicy` types `request` for you.

`harness-dispatch/sdk` is built into the worker, so there is nothing to install.
For editor type checking, map the specifiers to the declarations beside the
worker:

```json
{
  "compilerOptions": {
    "paths": {
      "harness-dispatch/sdk": ["<prefix>/libexec/harness-dispatch/sdk/index.d.ts"],
      "harness-dispatch/examples/*": ["<prefix>/libexec/harness-dispatch/examples/*.d.ts"]
    }
  }
}
```

The readable sources, `sdk/index.ts` and `examples/*.ts`, sit beside the
declarations.

## Starter examples

Three policies ship inside the worker as editable starting points. None is
active until your own policy imports it.

| Specifier | Kinds it routes |
|---|---|
| `harness-dispatch/examples/static` | A caller's own kinds, without Grove: `question`, `bugfix`, `feature`, `migration` and `architecture`, over one harness at four efforts |
| `harness-dispatch/examples/grove-static` | All 23 of Grove's session kinds, exactly, over a lead harness and a reviewer from another provider |
| `harness-dispatch/examples/dynamic` | The static example's kinds, through a `select` that applies their routes and polices explicit choices |

The two static examples map their kinds exactly: a kind one does not list
refuses. Each explains, kind
by kind, why the work gets the effort it does, in terms of abstraction,
uncertainty, consequences, downstream repair, reversibility and available
checks. None ranks models. These are priors, not calibrated estimates. Their
programs, such as `my-codex-wrapper`, are illustrative wrappers you supply. Each
receives `--model`, `--effort` and the prompt, and should exec your harness
with them. Their models and providers are placeholders. The Grove example
routes reviews to the other provider, but a static table cannot see which
provider actually created the artifact a review reads, so it enforces no
provider rule.

Use one whole from your personal policy:

```ts
export { policy } from "harness-dispatch/examples/grove-static";
```

Or keep its routes and supply your own catalog, with the same candidate IDs.
Each example also exports its `catalog` and a `CandidateId` type, so a typo in
an ID is a type error:

```ts
import { definePolicy } from "harness-dispatch/sdk";
import { routes } from "harness-dispatch/examples/grove-static";

export const policy = definePolicy({ schemaVersion: 1, version: "mine-1", catalog: [/* … */], routes });
```

Better still, copy `examples/grove-static.ts` or `examples/static.ts` from
beside the worker into your configuration directory and edit it. It imports
`harness-dispatch/sdk` as your own policy does.

The dynamic example builds on the static one, importing its catalog and routes
by their specifier. Its `select` shows what a table cannot say: without a
choice, it applies the kind's route and names that entry in its reason; with
one, it accepts a candidate whose effort is at least the route's and refuses
one below it as `effort_below_route`. A kind with no route sets no floor, so
any configured choice is accepted for it, and without a choice it refuses as
`incomplete_mapping`, in the policy's own words. It is deterministic, with no
clock, file, network or model involved. Use it whole, or call its exported
`select` from a policy of your own over the same catalog:

```ts
export { policy } from "harness-dispatch/examples/dynamic";
```

## Inputs

| Input | Meaning |
|---|---|
| `--kind TEXT` | Required. Any nonempty token, matched exactly against the routes, or given to `select`. Nothing else supplies the kind. |
| `--prompt TEXT` or `--prompt-file PATH` | The harness prompt. `run` needs exactly one; `inspect` shows a placeholder without either. |
| `--task-file PATH` | Optional. Resolved against the current directory and passed on as data. It is not read, need not exist, and supplies no kind or identity. |
| `--task-id ID` | Optional. The task's stable identity, such as a Grove handle: opaque UTF-8 of at most 1024 bytes. |
| `--config PATH` | Optional. The policy entry to use instead of the personal default. |
| `--choice ID` | Optional. Select this configured candidate: a routes policy takes it instead of the kind's route, and `select` accepts or refuses it. See [explicit choice](#explicit-choice). |
| `--timeout-ms MS` | Optional. The whole-selection bound in milliseconds, from 1000 to 120000. The default is 30000. See [the selection bound](#the-selection-bound). |
| `--state-dir PATH` | Optional. The directory holding run records, instead of `~/.local/state/harness-dispatch`, resolved against the current directory. See [run records](#run-records). |

The prompt is read once and kept byte for byte, trailing newlines included. It
must be valid UTF-8 with no NUL, and at most 1 MiB. A prompt file is resolved
against the current directory. A prompt that fails these checks, a file that
cannot be read and a file that is a terminal all refuse with exit 2 before any
policy runs. harness-dispatch never reads its own stdin, which stays the
harness's. The prompt never reaches the policy worker, so supplying it or not
cannot change the selection.

## Explicit choice

`--choice ID` names one candidate in the policy's catalog. A routes policy
selects it for any kind, including a kind its table does not route, and does
not consult the table. Inspection reports `selectedBy` as `explicit_choice`
rather than `route`, and the choice itself as `explicitChoice`. The run record
keeps both. A `select` policy sees the choice and must accept or refuse it, as
[a select policy](#a-select-policy) describes; it reports `selectedBy` as
`select`, beside the `explicitChoice` it accepted. An ID the catalog does not
have refuses with `unknown_choice`, exit 3, under either form and before any
`select` runs, and its remedy lists the configured IDs. Nothing else is
selected in its place. The ID is matched exactly, and an empty one refuses with
exit 2 before any policy runs.

```sh
harness-dispatch inspect --kind design --choice deep
harness-dispatch run --kind impl --choice quick --prompt 'Rename the flag'
```

The choice names the whole joint candidate. There is no way to override its
model or effort alone. Each invocation, a retry included, evaluates the policy
afresh.

## The selection bound

Your policy is trusted TypeScript, and it can hang: a loop at import, or an
`await` on work that never settles. So the whole selection has a wall-clock
bound. It runs from the worker's start to its result, and it covers the
policy's import, everything the policy does while it loads, and its `select`.
The default is
30 seconds. `--timeout-ms` sets it for one invocation, anywhere from 1000
(1 second) to 120000 (2 minutes). A value outside that range, or anything but
plain digits, refuses with exit 2 before any policy runs.

The front process keeps the time itself, so the bound holds whatever the policy
does, including a synchronous loop that no timer inside the worker could
interrupt. When it runs out, the front sends the worker TERM and gives it at
most one second to exit, then kills it, and reaps it before returning. Nothing
is launched, and the command exits **124**:

```json
{"schemaVersion":1,"error":{"code":"selection_timeout","stage":"evaluation",
 "message":"the policy entry /home/me/.config/harness-dispatch/policy.ts did not return a result within the selection bound of 1000 ms (--timeout-ms), so its worker was stopped and nothing was launched",
 "input":"--timeout-ms","source":"/home/me/.config/harness-dispatch/policy.ts",
 "bound":{"name":"selection","ms":1000,"from":"--timeout-ms"},
 "remedy":"…","exit":124},"diagnostics":{"stdout":"","stderr":""}}
```

`bound.from` is `--timeout-ms` or `default`, and whatever the policy printed
before it was stopped is kept in `diagnostics`. A worker that returns its result
in time but then does not exit, because an exit handler holds it, also gets one
second before it is killed. Its selection stands. A policy that starts
processes of its own must end them before it returns; the front stops only the
worker.

## Arguments and slots

A candidate's `args` are its harness's arguments after the program. Each is a
literal string or a slot object, `{ slot: "<name>" }`, which fills that one
whole argument with one value:

| Slot | Value |
|---|---|
| `prompt` | The prompt, unchanged. Every candidate uses it exactly once. |
| `kind` | `--kind` |
| `taskFile` | `--task-file`, as an absolute path |
| `taskId` | `--task-id` |
| `model`, `effort` | The candidate's own catalog values |
| `runId` | The run's ID, the same one the harness receives as `HARNESS_DISPATCH_RUN_ID` |

There is no shell. Nothing is split, globbed, interpolated into a literal or
read twice, so spaces, quotes, `$`, `;` and newlines arrive exactly as given.
A slot whose input the caller did not supply refuses with `missing_input`,
naming the flag that would fill it. It is never left empty or dropped. Under
`inspect`, a `runId` argument is a proposed ID, marked as one, because
inspection records no run.
Model and effort need not appear as slots if a wrapper you configure encodes
them itself; the catalog values are your assertion, not something
harness-dispatch verifies.

## The program

`program` is one of three things:

- An absolute path, used as it is.
- A relative path containing a `/`, such as `./tools/harness`, resolved against
  the caller's current directory.
- A plain name, looked up in the caller's `PATH` as the shell does: entries in
  order, an empty or relative entry meaning the current directory, and the
  first executable regular file wins. A `PATH` that is set but empty is one
  empty entry, the current directory. With `PATH` unset, no name is found.

Only the selected candidate's program is checked. A program that cannot be
found refuses with exit 127, and one that exists but cannot be executed with
exit 126. harness-dispatch never runs another candidate instead. The resolved
file is what `run` executes, and the program as configured is the harness's
`argv[0]`, as a shell passes a command as typed.

## Inspect

```sh
harness-dispatch inspect --kind impl
harness-dispatch inspect --kind impl --task-id T-12 --prompt 'Implement the parser'
harness-dispatch inspect --kind review-impl --config ./policies/review.ts --json
```

Inspection is a proposal, not a launch reservation. It launches nothing and
reserves nothing, and a later `run` evaluates the policy afresh. It evaluates
trusted TypeScript, and is not promised to be free of that code's side
effects. It makes the same choice `run` would, and refuses where `run` would,
with the same exit. It
reports the policy's path, its authority (personal or explicit), its SHA-256
and version, the task file, task identity and prompt it was given, any explicit
choice, the chosen candidate with its provider, model and effort, what selected
it and why, the resolved program,
the expanded argv, the effective selection bound, the selection time, the
worker's identity, and where `run` would record. It also shows a proposed run
ID. That ID is marked as proposed, no run holds it, and a later `run`
allocates its own. Inspection never opens the record store, so it cannot tell
you in advance that `run` would find the store unusable. Without a
prompt, the prompt's argument is a marked placeholder. Human text shows each
argument quoted and escaped. `--json` prints the same facts on stdout as one
version-1 object:

```json
{
  "schemaVersion": 1,
  "evidence": "proposal",
  "proposedRunId": "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34",
  "stateDir": { "path": "/home/me/.local/state/harness-dispatch", "from": "default" },
  "kind": "impl",
  "taskFile": null,
  "taskId": "T-12",
  "prompt": { "supplied": true, "from": "--prompt", "bytes": 20 },
  "policy": { "path": "/home/me/.config/harness-dispatch/policy.ts", "authority": "personal",
              "sha256": "9c1f…", "version": "2026-09-30" },
  "selection": { "form": "routes", "selectedBy": "route", "explicitChoice": null, "candidateId": "deep",
                 "provider": "anthropic", "model": "claude-opus-5-5", "effort": "high",
                 "reason": "routes[\"impl\"] names candidate \"deep\"" },
  "executable": { "program": "claude", "resolvedBy": "PATH", "pathEntry": "/opt/homebrew/bin", "path": "/opt/homebrew/bin/claude" },
  "argv": ["claude", "--model", "claude-opus-5-5", "Implement the parser"],
  "bounds": { "selection": { "ms": 30000, "from": "default" } },
  "timing": { "selectionMs": 15 },
  "worker": { "path": "…/libexec/harness-dispatch/harness-dispatch-policy", "packageVersion": "…", "buildId": "…", "bunVersion": "1.4.2" },
  "diagnostics": { "stdout": "", "stderr": "" }
}
```

`selection.form` is `routes` or `select`. `selectedBy` is `route`,
`explicit_choice` (a routes policy took the caller's choice without its table)
or `select` (the policy's `select` chose it; beside an `explicitChoice`, it
accepted that choice). `reason` is the policy's own under `select`. Human text
shows the same as a `selected` row. An explicit entry adds `"argument"`, the
`--config` value as given. A prompt
read from a file reports `"from": "--prompt-file"` and its `"path"`. Without a
prompt, `"prompt"` is `{ "supplied": false }` and its argument in `argv` is
`{ "placeholder": "prompt" }`, and a `runId` argument is
`{ "proposedRunId": "…" }`. `stateDir.from` is `default` or `--state-dir`.
`resolvedBy` is `absolute`, `cwd` or `PATH`, and only `PATH` adds `pathEntry`,
the entry as it is spelled in PATH.

The worker runs in a private empty directory with null stdin. Its environment
contains only HOME, PATH, TMPDIR, LANG and `LC_*`. It talks to the front over a
private channel on descriptor 3 and inherits no other descriptors, however high
a descriptor you started harness-dispatch with is numbered. Whatever the
policy prints is captured and kept apart from the report. `--json` carries it in
`diagnostics`, and text mode prints it on stderr, each line prefixed with
`policy stdout:` or `policy stderr:`.

## Run

```sh
harness-dispatch run --kind impl --prompt 'Implement the parser'
harness-dispatch run --kind impl --task-file ./tasks/parser.md --task-id T-12 --prompt-file ./mandate.md
```

`run` makes the choice `inspect` reports. It then commits the run's handoff
record, described [below](#run-records), and replaces its own process with the
harness. If the record cannot be committed, nothing is launched and `run`
exits 4. The harness keeps the caller's current directory, descriptors,
environment and process ID, so its exit code or signal is the command's own,
even where the code coincides with one of harness-dispatch's refusal exits.
Nothing supervises it afterwards. Its environment gains two variables, which
replace any values the caller had:

- `HARNESS_DISPATCH_RUN_ID`, the run's ID. It is the same ID a `runId` slot
  passes as an argument.
- `HARNESS_DISPATCH_STATE_DIR`, the absolute directory the run was recorded
  in. `record show --state-dir "$HARNESS_DISPATCH_STATE_DIR"` finds the run
  from inside the harness.

Stdout and stdin are the harness's. On stderr, `run` prints whatever the policy
printed, prefixed as in inspection, and then one line naming the candidate, the
run ID and the file it executes. With `--json`, that line is one JSON object
instead, `{"schemaVersion":1,"handoff":{…},"diagnostics":{…}}`. Its `handoff`
carries `runId`, `recordedAt` and `stateDir` beside the choice, with
`selectedBy` and `explicitChoice` as inspection reports them.

If the operating system refuses to execute the resolved file, `run` reports
`exec_failed` with the error and a remedy. That exits 127 when the file, or the
interpreter on its `#!` line, does not exist, and 126 otherwise. A prompt too
large for the platform's own argument limit fails this way (`E2BIG`), although
it is within the 1 MiB bound. The failure is appended to the run's record as a
launch failure. The error reports the outcome as `run`: in JSON,
`{"id":"…","launchFailure":"recorded"}`, or `"unrecorded"` with the reason if
the append itself failed. An unrecorded failure leaves the run a handoff
attempt whose execution is unknown, never a success. With `--json`, the error
is a second JSON line after the handoff notice.

## Run records

Every `run` records one **handoff attempt** before it execs. The record is
durable intent, not evidence that the harness ran: a harness that exits 0 and
one that is killed at once leave the same record. harness-dispatch records
nothing after exec, because nothing of it is left to observe the harness.

The records live in one SQLite file, `records.sqlite3`, in the state
directory. That is `~/.local/state/harness-dispatch` unless `--state-dir`
names another directory. `XDG_STATE_HOME` is not consulted. Both the directory
and the file are created on first use, readable by you alone. SQLite is built
into harness-dispatch, so nothing else needs installing. The commit is one
short transaction, synced as SQLite's `synchronous = EXTRA` setting does (with
`F_FULLFSYNC` on macOS). On first use, each directory that gains a new record
directory is synced as well, before the commit. The record then survives a
power loss after the harness starts, and a sync that fails refuses like the
commit. It is taken only after the policy has finished, so no
policy ever runs with the store locked. If another process holds the store,
`run` waits at most 2 seconds, apart from the selection bound, and then
refuses. The store names itself with a version. A store that another
application wrote, that a newer harness-dispatch wrote, or that SQLite finds
corrupt refuses with exit 4. It is left exactly as it was: harness-dispatch
never resets or replaces a store.

A run records its ID and the time it was committed. It also records the kind,
the task identity and task file, the current directory, and the policy's path,
authority, SHA-256 and version. It keeps the selected candidate exactly as the
catalog configured it, how it was selected and why, including any explicit
choice, the resolved program and the full argv. It keeps the worker's identity,
the effective bounds
and the selection time. Fields that later releases supply, such as a reviewed
artifact, loaded context, an adapter version and creator provenance, are
`null`. No environment value is recorded. The argv includes the prompt, so the
records are private execution data. A committed run's launch fields never
change.

```sh
harness-dispatch record show --run "$HARNESS_DISPATCH_RUN_ID" --json
harness-dispatch record show --run 5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34 --state-dir ./records
```

`record show` exports one run, in text or as one version-1 JSON object:

```json
{
  "schemaVersion": 1,
  "runId": "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34",
  "recordedAt": "2026-09-30T08:15:42.117Z",
  "evidence": "handoff_attempt",
  "execution": "unknown",
  "launch": { "schemaVersion": 1, "kind": "impl", "taskId": "T-12", "candidate": { "id": "deep", "provider": "anthropic", "…": "…" }, "…": "…" },
  "launchFailure": null,
  "observations": [],
  "outcomes": { "executionConfirmation": { "state": "unobserved" }, "exit": { "state": "unobserved" }, "…": "…" }
}
```

`evidence` is `handoff_attempt`, with `execution` `unknown`, until an exec
failure is appended. It is then `launch_failure`, with `execution`
`not_executed`, and `launchFailure` holds the error. Every outcome, from exit
and duration to acceptance and findings, is `unobserved`: this release has no
way to add observations, and an absent measurement is never zero. The run ID
must be exactly as reported, in lowercase. An unknown run refuses with exit 3,
saying whether a store exists at the directory it looked in.

## Refusals

A refusal launches nothing and never substitutes another candidate. Nothing is
retried, paged or confirmed interactively: each invocation evaluates the policy
once. Every refusal names a stable code, the stage it arose in, a message, the
input or source involved, and a remedy. Text mode prints them on stderr, after
any output the policy printed, each line of it prefixed `policy stdout:` or
`policy stderr:`. `--json` prints one object on stderr,
`{"schemaVersion":1,"error":{…},"diagnostics":{…}}`, and nothing on stdout. The
error names `input`, such as `--kind design`, or `source`, usually the policy
entry, or both, and `location` where the problem is inside a policy or a
`select` result. A policy's own refusal adds `policyCode`, and a
`policy code:` line in text. A refusal
caused by running out of a bound also names that bound, as `bound` in JSON and
a `bound:` line in text. A command line that cannot be parsed is refused the
same way, with clap's tip and usage line in the remedy.

A refused `run` also names the **equivalent `inspect` invocation**: the same
selection inputs, with no prompt, from the same directory. An owner can then
reproduce an unattended refusal without reconstructing its inputs. In text mode
it is one command line for a POSIX shell, run in a subshell so that pasting it
leaves your directory alone:

```text
harness-dispatch: refused (incomplete_mapping, stage selection): the routes in /home/me/.config/harness-dispatch/policy.ts name no candidate for kind "design"
  input: --kind design
  source: /home/me/.config/harness-dispatch/policy.ts
  location: policy.routes
  remedy: add a route "design" to a candidate ID in /home/me/.config/harness-dispatch/policy.ts, or name one configured candidate with --choice ID; harness-dispatch never substitutes a default candidate
  inspect: (cd /work && harness-dispatch inspect --kind design --task-id T-12)
```

In JSON it is `error.inspect`, an argv array and the directory to run it in:

```json
"inspect": { "cwd": "/work", "argv": ["harness-dispatch", "inspect", "--kind", "design", "--task-id", "T-12", "--json"] }
```

The program is the one the caller ran, as the caller spelled it. `--json`
carries over when the refusal was JSON. A value that begins with a hyphen is
written `--flag=value`. When a value, the program path or the directory is
not valid UTF-8, no text or JSON string holds it exactly. A lossy copy would
name other inputs, so the invocation is reported as unavailable instead:
`inspect: unavailable: --task-id is not valid UTF-8, so no command line
reproduces it exactly`, or `{"unavailable": "…"}` in JSON. A command line that
cannot be parsed has no equivalent, and `inspect`'s own refusals name none.
Once a command line parses, its own `--json` flag chooses the format. A
`--json` that is the value of another flag, such as `--prompt --json`, is data.
Run the invocation, correct
what the remedy names, and inspect again until it reports a choice. A refusal
at the record commit reproduces as a successful inspection, because inspection
never opens the store.

| Exit | Stage | Codes |
|---|---|---|
| 2 | `cli` | `malformed_input` (including a command line that cannot be parsed, and an empty `--choice`), `unsupported_input` (an input or command a later release delivers), `prompt_invalid`, `prompt_unreadable` |
| 3 | `authority` | `policy_missing`, `policy_unreadable`, `home_unset`, `cwd_unavailable` |
| 3 | `load` | `policy_import_failed` (a missing import, the entry threw while loading, or an await in it never settled) |
| 3 | `validation` | `policy_invalid` and `unsupported_version`, each with its `location`; `unsupported_form` (`loadContext`) |
| 3 | `selection` | `incomplete_mapping`; `unknown_choice` (a `--choice` the catalog does not have); `policy_refused` (the policy's own refusal, with `policyCode`); `selection_threw`, `selection_unsettled`, `selection_abstained`, `selection_malformed`, `unknown_candidate` and `explicit_choice_mismatch` (see [a select policy](#a-select-policy)) |
| 3 | `expansion` | `missing_input` (a slot whose input was not supplied) |
| 3 | `record` | `run_not_found` (`record show` of a run the store does not hold), `cwd_unavailable` |
| 4 | `record` | `record_store_unwritable`, `record_store_locked` (held past the 2-second wait), `record_store_full`, `record_store_invalid` (another application's file, another version, or corrupt), `record_commit_failed`, `run_id_unavailable`, `home_unset` (HOME cannot place the default state directory) |
| 5 | `worker` | `worker_missing`, `worker_identity_mismatch`, `worker_failed`, `protocol_error` |
| 124 | `evaluation` | `selection_timeout` (the selection bound ran out) |
| 126 | `resolution`, `exec` | `program_unexecutable`; `exec_failed` for any exec error but `ENOENT` |
| 127 | `resolution`, `exec` | `program_not_found`; `exec_failed` for `ENOENT` |

Once the harness runs, its own exit status or signal is the command's, even
where the code coincides with one of these.
