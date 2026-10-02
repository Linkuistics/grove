# harness-dispatch

`harness-dispatch` passes what its caller gave it to the `select` function of
an owner's TypeScript policy, and acts on what that function returns: it
reports the command, or records and runs it, or reports the refusal. It is
independent of Grove: a caller supplies a kind, a prompt and whatever else it
knows, and the owner's function returns the program, its arguments, and the
provider, model and reasoning-effort labels for what they run. The contract is
the [area specification](../../docs/specs/harness-selection-and-execution.md).
Its commands:

| Command | What it does |
|---|---|
| [`init`](#the-sample-policy) | Installs the sample policy as your personal default, when nothing is there. |
| [`inspect`](#inspect) | Reports the command `select` returns, its labels and the file its program resolves to. It launches nothing. |
| [`run`](#run) | Makes the same selection, commits a durable record of the handoff under a fresh run ID, and then replaces itself with the command. |
| [`record show`](#run-records) | Exports what a run recorded. |
| [`record observe`](#observations) | Attaches later evidence to a run: whether the harness ran, how it went, and how its work was judged. |

[A policy](#a-policy) is one function. The command holds no catalog of
harnesses and no table of kinds, and it has no input that names a choice: a
table from kind to command is something your `select` consults, and a caller
that wants to steer passes a [parameter](#parameters) your function reads. A
caller can hand the policy a JSON context document with `--context`, and a
policy can assemble its own context with `loadContext`, through reads that are
measured and hashed. A loader can look up an earlier run's recorded launch
fields, so a review can learn which provider its creator ran under from the
record, never from what the policy returns today. Four example policies ship
inside the worker: two starters that consult a table by kind, and two that hold
reviews to a provider rule, one of them for Grove's review leaves. None is
active until your own policy imports it. `init` installs a fifth file, the
[sample policy](#the-sample-policy), as your own: it is a working policy with
real `codex` and `claude` command lines, not an example to import.

Every invocation holds to the same limits. Selection is bounded in time, and
its context, reads, messages and output in size, so a policy that never
finishes, or delivers or prints too much, is stopped and nothing runs. Every
refusal says what to fix, and a refused `run` gives the `inspect` command that
reproduces it. Nothing is run in place of a command that was refused. An
interrupt before the harness starts cancels the run, and nothing runs. The
harness inherits the caller's signal mask and ignored signals, SIGPIPE's
included. The policy runs with a scrubbed environment, plus exactly the
variables you grant it, and nothing in the current directory or the
environment can run code in it or change which worker runs. What you set about
every invocation, the time bound and the grants among it, lives in one
[settings file](#owner-settings) beside your policy, so a caller passes none of
it.

## Install

Grove's Homebrew formula and release archives install harness-dispatch with
Grove:

```sh
brew install linkuistics/taps/grove
harness-dispatch --version
```

A release archive unpacks to an installation prefix holding `bin/` and
`libexec/`; keep the two together and put `bin/` on `PATH`. Under Homebrew the
prefix is `$(brew --prefix grove)`. `libexec/harness-dispatch/` there holds the
policy worker, and beside it the declarations and readable sources of the SDK
(`sdk/`), the Grove adapter (`grove/`) and the examples (`examples/`), with the
sample policy as `examples/sample.ts`. The
notices for the Bun runtime inside the worker and the SQLite inside the front
are in `libexec/harness-dispatch/notices/`.

## Supported platforms

Releases carry harness-dispatch for macOS on Apple silicon, Linux arm64 and
Linux x64. Each floor is claimed only as far as something observes it. Before
every release, the installed pair runs from each archive on its target:
natively on macOS, and at the Linux floors
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
It runs each of its cases through `PREFIX/bin/harness-dispatch` and through a
symlink to it. The sample policy is installed with `init` into an empty HOME
and inspected. A policy that consults a table and one that builds its command
from the caller's parameters are each inspected, run against a fake harness
and read back from the run's record. A policy that
imports one package declared by `main` and one by `exports` is inspected. And
a harness that signals itself shows that it inherited the caller's SIGPIPE and
HUP as they were. VERSION is the version the front and its worker must both
report. Run it with no Bun or Node on `PATH`, such as
`env -i PATH=/usr/bin:/bin bash scripts/installed-smoke.sh ~/.local "$VERSION"`.
Grove's `task release:smoke` runs it on every release target.

## Which policy runs

The personal default is `~/.config/harness-dispatch/policy.ts`. `--config PATH`
names another entry instead, and a relative path resolves against the current
directory. Nothing else selects a policy. There is no search of the current
directory or its parents, no environment variable and no repository override,
so entering a repository runs none of its code. Naming a file there with
`--config` is your explicit choice, and inspection reports it as explicit
authority. Your personal policy may itself import a repository entry. That
import is also your choice, and the imported code runs with the same trust.

A missing, unreadable or invalid entry refuses and names the path. With no
personal policy the refusal is `policy_missing`, and its remedy names
[`harness-dispatch init`](#the-sample-policy), the one command that writes a
policy. So does an
entry whose resolved path is not UTF-8 or contains `?`. The worker imports the
path as a string, which would then name another file: its runtime reads
`policy.ts?x` as `policy.ts` with a query. Nothing is
installed on the policy's behalf, even where a `package.json` lists a
dependency. Relative imports resolve from the importing file, bare imports
resolve through `node_modules` beside it or above it, and a missing import
refuses.

An ordinary npm package loads: run `npm install` beside your policy and import
it. The worker reads `package.json`, so a package's `main` and `exports`
choose its entry point. An `exports` map is read under the runtime's own
conditions, `bun` among them, and no variable changes that, a granted
`NODE_ENV` included.

The nearest `package.json` at or above a policy file is that file's own
package, as in Node. Two things follow for a policy directory that has one.
Its `imports` map resolves the file's `#` names. And a bare import of its own
`name` resolves through its `exports`, ahead of any `node_modules` package of
that name. A `package.json` in the directory you run from takes no part unless
the policy file sits in or under that directory.

One difference from Node matters when a `package.json` is broken. Node refuses
an import beside one it cannot parse. The worker passes over a `package.json`
that does not parse, or whose JSON is not an object, and says nothing. The
next one above it is then the file's package, and its `imports` map answers,
so a typo in the nearer file can change which code a `#` name loads. A
well-formed nearer file is always the one used, with or without a `name`, and
a `#` name it has no entry for refuses.

Do not alias a `harness-dispatch/…` name. An `imports` entry such as `"#sdk":
"harness-dispatch/sdk"` is looked up as a package in `node_modules`, and never
reaches the SDK built into the worker. With no such package the import
refuses, and beside a `node_modules/harness-dispatch` it loads that package.
Import `harness-dispatch/sdk` by its name.

## The sample policy

`harness-dispatch init` installs a sample policy as your personal default:

```sh
harness-dispatch init
```

```text
Installed the sample policy as /home/me/.config/harness-dispatch/policy.ts.
It launches codex with approvals off and full access (--ask-for-approval never, default_permissions=:danger-full-access). Read it, and edit it, before the first launch: it is yours.
See what it selects with
  harness-dispatch inspect --kind impl --param session_name=NAME --param repo=PATH
```

`init` takes no input. It writes `~/.config/harness-dispatch/policy.ts` and
nothing else. When anything is already at that path it refuses as
`policy_exists`, and it has no option to replace a policy: move yours away
first. Nothing but `init` ever writes a policy. The front carries the sample's
text, so `init` needs no worker, and the same file ships readable beside the
worker as `examples/sample.ts`. Once installed it is your own file, and an
upgrade never touches it.

**Read it before the first launch.** The sample is one owner's launch policy
for Grove, with the real `codex` and `claude` command lines. Its `codex`
command passes `--ask-for-approval never` and
`default_permissions=:danger-full-access`, so a session it launches can change
anything your account can. Remove those arguments if that is not what you want.

The file is a `select` over tables of its own, which you edit in place:

- **Routes.** Each of Grove's 23 session kinds, and the standalone
  `release-notes` kind that `grove run release-notes` launches, maps to a role
  and usually an effort. A kind it does not list refuses, with the policy's own
  code `incomplete_mapping`.
- **Arrangements.** Four of them say which harness takes which role.

  | Arrangement | Requirements, design and writing | Planning, prototypes and implementation |
  |---|---|---|
  | `codex-led` | `codex` leads, `claude` reviews | `codex` leads, `claude` reviews |
  | `claude-led` | `claude` leads, `codex` reviews | `claude` leads, `codex` reviews |
  | `codex-design-claude-impl` | `codex` leads, `claude` reviews | `claude` leads, `codex` reviews |
  | `claude-design-codex-impl` | `claude` leads, `codex` reviews | `codex` leads, `claude` reviews |

  The two mixed arrangements also route a few kinds their own way, `copy-edit`,
  `art` and `proof` among them, and name a model for some.
- **Modifiers.** `codex-sol` pins `codex` to its Sol model and runs the
  requirements, design and planning reviews at `high` rather than `xhigh`.
  `high-effort` raises to `high` every session kind whose route sets no
  effort; `release-notes` stays as it is.
- **The default.** With no [choice file](#the-choice-file) the sample selects
  `claude-led` with `codex-sol`.

It labels a `codex` command `openai` and a `claude` command `anthropic`. Each
review goes to the other provider by the arrangement, and the sample consults
no creator: [the review policy](#the-review-policy) is the one that does. Its
`reason` names the arrangement and modifiers it applied, and whether they were
the default or came from a choice file.

The commands place two [parameters](#parameters) Grove passes: `claude` is
named for `session_name`, and both harnesses receive `repo` as `--add-dir`. A
caller that did not pass one the selected command needs is refused, with the
policy's own code `parameter_missing` and the `--param` to add:

```sh
harness-dispatch inspect --kind impl --param session_name=parser --param repo=/work/parser
```

### The choice file

One checkout selects differently from another with a **choice file**:
`.harness-dispatch-choice` in the caller's directory, holding names separated
by whitespace. For the sample, in the directory Grove runs in:

```sh
echo 'codex-design-claude-impl high-effort' > .harness-dispatch-choice
```

A choice file there replaces the sample's default whole, so a modifier the
default applies is named again to keep it. It names exactly one arrangement
and any modifiers; none, or two, refuses as `choice_arrangement`. Without the
file the default applies.

The file can only name what your policy offers. It introduces no program,
argument or label, so a repository that ships one gains no authority: it picks
among your own options, and only where your policy reads it. harness-dispatch
itself never looks for the file. The SDK's `readChoice` does, when your
`select` calls it:

```ts
import { CHOICE_FILE, definePolicy, readChoice } from "harness-dispatch/sdk";

const efforts: Readonly<Record<string, string>> = { fast: "low", careful: "high" };

export const policy = definePolicy({
  schemaVersion: 2,
  version: "mine-1",
  select(request) {
    const choice = readChoice(request.cwd, Object.keys(efforts));
    if (choice !== undefined && "status" in choice) return choice;
    const name = choice?.[0] ?? "careful";
    const effort = efforts[name] ?? "high";
    return {
      status: "selected",
      program: "claude",
      args: ["--effort", effort, request.prompt],
      provider: "anthropic",
      model: "claude-opus-5-5",
      effort,
      reason: `${name}, ${choice === undefined ? "the default" : `chosen by ${CHOICE_FILE}`}`,
    };
  },
});
```

`readChoice(directory, offered)` reads `.harness-dispatch-choice` in
`directory` and returns one of three things:

- The names the file holds, in order, each one of `offered`. What a name means
  and how many a selection takes are your policy's to decide.
- `undefined` when there is no such file.
- A refusal to return from `select`, which launches nothing. A name outside
  `offered` refuses with the policy code `choice_unoffered`, naming the file,
  the name and the names offered. A file that is not a regular UTF-8 file of
  at most 4 KiB refuses as `choice_unreadable`.

Pass `request.cwd` as the directory. Policy code runs in `/`, so a relative
directory names a place there and not in the caller's checkout. The read is an
ordinary one, not a [measured source](#loading-context), so it needs no
`loadContext`, and inspection shows no digest for it. Name the choice you
applied in your `reason`, which inspection and the run record carry.

## Owner settings

What you set about every invocation, without a caller passing anything, lives
in one optional JSON file beside your policy,
`~/.config/harness-dispatch/settings.json`:

```json
{
  "timeoutMs": 240000,
  "contextBytes": 1048576,
  "stateDir": "/home/me/.local/state/dispatch-records",
  "policyEnv": ["ROUTER_TOKEN"]
}
```

| Key | Sets | Flag that replaces it |
|---|---|---|
| `timeoutMs` | [The selection bound](#the-selection-bound), in milliseconds, from 1000 to 600000 | `--timeout-ms` |
| `contextBytes` | The [context budget](#bounds), in bytes, from 1 to 8388608 | `--context-bytes` |
| `stateDir` | The directory holding [run records](#run-records), an absolute path | `--state-dir` |
| `policyEnv` | The names [granted to the policy](#the-policys-environment), an array | `--policy-env` adds to it |

Every key is optional, and a missing file sets nothing. Every command reads
the file before any policy runs, `record show` and `record observe` included,
so they all find the same records. A flag replaces its setting for one
invocation, and `--policy-env` adds names to the ones the file grants.
[Inspection](#inspect) reports each bound and the record directory with where
its value came from: `default`, `settings.json` or the flag.

A value is held to its flag's own rules. A bound outside its range refuses as
`malformed_input`, and a name that is never granted as `excluded_grant`, as the
flag would. A file that cannot be read, is not a JSON object, holds a key
other than these four, or holds a `stateDir` that is not an absolute path or a
`policyEnv` that is not an array of strings refuses as `settings_invalid`. Each
is exit 2, before any policy runs, and names the file as `source` and the key
as `location`.

The file has your personal policy's authority and its rules. It is found from
HOME alone: no variable, `XDG_CONFIG_HOME` included, and no file in the
current directory or a repository supplies or replaces it. Without an absolute
HOME it has no location and sets nothing. There is no setting for the policy
entry: if you keep your policy elsewhere, re-export it from the personal
entry, or name it with `--config`.

The settings are the same for every kind. Each bound is a ceiling, so one that
admits your slowest selection serves the rest. If you raise `timeoutMs` for a
[deciding agent](#handing-the-prompt-to-a-deciding-agent), you raise it for a
kind that only consults a table too, and that kind gives up its shorter
failure bound. A policy that wants a tighter limit on part of its own work
applies it in its own code. One record directory is what lets a
[run lookup](#looking-up-a-run) under one kind find a run recorded under
another.

A setting cannot live in the policy file. The grants shape the worker's
environment before any policy code loads, and the time bound runs from the
worker's start, imports included.

## A policy

```ts
import { definePolicy } from "harness-dispatch/sdk";

const efforts: Readonly<Record<string, string>> = { design: "high", impl: "medium", "review-impl": "high" };

export const policy = definePolicy({
  schemaVersion: 2,
  version: "2026-10-02",
  select(request) {
    const effort = Object.hasOwn(efforts, request.kind) ? efforts[request.kind] : undefined;
    if (effort === undefined) {
      return {
        status: "refused",
        code: "unrouted",
        message: `no command is configured for kind ${request.kind}`,
        remedy: "add the kind to the efforts table",
      };
    }
    return {
      status: "selected",
      program: "claude",
      args: ["--model", "claude-opus-5-5", "--effort", effort, request.prompt],
      provider: "anthropic",
      model: "claude-opus-5-5",
      effort,
      reason: `efforts[${JSON.stringify(request.kind)}]`,
    };
  },
});
```

The module exports one plain object named `policy`:

- `schemaVersion` is `2`, and `version` is your own nonblank label, which
  inspection reports.
- `select(request, context, host)` returns the command to run, or a refusal.
- `loadContext(request, host)` is optional, and assembles the
  [context](#context) `select` receives.

That is the only form. A field beyond these four refuses as `policy_invalid`,
naming it, and so does a `select` that is not a function. A policy that
declares `schemaVersion: 1`, the catalog contract of releases up to 21.13.0,
refuses as `unsupported_version` with the remedy to rewrite it: move each
catalog entry's program, arguments and labels into what `select` returns, and
build the arguments from the request. Nothing converts one.

`select` may be synchronous or `async`, and may do whatever trusted TypeScript
can, within [the selection bound](#the-selection-bound). It is called as a
method of the policy, and only once harness-dispatch has accepted the policy's
shape. Its first argument is the request:

| Field | Value |
|---|---|
| `schemaVersion` | `2` |
| `kind` | `--kind` |
| `prompt` | The caller's prompt, byte for byte. Under an `inspect` given no prompt, a [fixed marker](#inspect) |
| `cwd` | The caller's current directory, as data. The worker does not run there. |
| `params` | Every [`--param`](#parameters) by name, each value a string; an empty object when the caller passed none |
| `taskFile`, `taskId` | `--task-file`, as an absolute path, and `--task-id`. Each is absent when not given. |
| `context` | The `--context` document, as data, absent when not given. See [context](#context). |
| `limits` | The effective [bounds](#bounds): `selectionMs`, `contextBytes`, `sourceBytes`, `sources`, `messageBytes` and `diagnosticsBytes` |

The request is frozen, and it carries no run identity: a harness reads its own
from `HARNESS_DISPATCH_RUN_ID`. `select` also receives the measured context, or
`undefined` when there is none, and a host with `diagnostic` and `signal`, as
[context](#context) describes.

`select` returns, or resolves to, one of two results, told apart by `status`:

- `{ status: "selected", program, args, provider, model, effort, reason }` is
  [the command](#the-command) to run. `program` and `args` are what
  harness-dispatch executes. `provider`, `model` and `effort` are your labels
  for what that command runs, and `reason` says why: which table entry or rule
  applied. Inspection and the run record report the labels and the reason
  exactly as given.
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
| The result is neither shape: a missing or blank field, an `args` that is not an array of strings, a NUL in the program or an argument, an unknown `status`, or any field beyond its status's own | `selection_malformed`, with a `location` such as `result.args[2]` |

A promise that live work keeps pending, such as one waiting on a timer that
never resolves it, and a synchronous loop, are stopped instead by the selection
bound, with exit 124.

harness-dispatch validates the shape of what `select` returns and nothing about
its content. It does not check that the prompt is among the arguments, that the
program is one you listed anywhere, or that a kind has a route. Those are your
function's to hold, with the type checker and whatever tables it keeps. Nor
does anything it reports distinguish a `select` that consults a table from one
that reads the prompt or the task: both are a function and what it returned.

**A policy that reads the prompt holds a mandate.** The prompt tells its
reader what to do: under Grove it says to load a skill and carry out a task. A
policy can hand it to an agent of its own to evaluate, starting that agent in
`request.cwd` and reaping it before it returns. Nothing in harness-dispatch or
Grove keeps such an agent from doing what the prompt says instead of
evaluating it. Keeping it from executing the task, and from changing the
caller's tree, is your job as the policy's owner: what the agent is told, which
tools it has and whether it can write are all set by the policy that starts it.
And a policy that turns outside text into a command, such an agent's answer or
a file's content included, should choose among commands its own code wrote, and
never place that text in `program` or `args`.

### Handing the prompt to a deciding agent

harness-dispatch ships no agent policy and calls no model. The SDK has no
subprocess API either: a policy that wants an agent's judgment starts the agent
itself, with the runtime's own process API, and that is all dynamic dispatch
needs.

```ts
const COMMANDS = {
  deep: { model: "your-large-model", effort: "high" },
  quick: { model: "your-small-model", effort: "low" },
} as const;

async select(request, context, host) {
  // Bun.spawn: https://bun.com/docs/runtime/child-process
  const agent = Bun.spawn(["your-deciding-agent", request.prompt], {
    cwd: request.cwd,     // the worker itself runs in /
    stdin: "ignore",      // no interactive stdin
    stdout: "pipe",
    stderr: "ignore",
    signal: host.signal,  // stopped when the selection is
  });
  const answer = (await new Response(agent.stdout).text()).trim();
  await agent.exited;     // reaped before select returns
  const chosen = COMMANDS[answer as keyof typeof COMMANDS];
  if (chosen === undefined) {
    return { status: "refused", code: "answer_unknown", message: `the agent answered ${JSON.stringify(answer)}`, remedy: "have it answer deep or quick" };
  }
  return {
    status: "selected",
    program: "your-harness",
    args: ["--model", chosen.model, "--effort", chosen.effort, request.prompt],
    provider: "your-provider", model: chosen.model, effort: chosen.effort,
    reason: `the deciding agent answered ${answer}`,
  };
}
```

Four things in it are yours to keep:

- **Start it in `request.cwd`.** The worker runs in `/`, and a child inherits
  that unless you place it. An agent that resolves what the prompt refers to
  needs the caller's directory.
- **Pass `host.signal`.** The front stops only the worker when the selection
  bound runs out or the selection is interrupted. A child started without the
  signal outlives a timed-out selection, and keeps doing whatever it was doing.
- **Reap it before you return**, keep it in the foreground job and give it no
  interactive stdin. A detached or background service is outside the contract.
- **Choose among your own commands.** The answer picks an entry. It never
  becomes a program or an argument.

The agent inherits the worker's environment, so its credentials are names you
grant in [`policyEnv`](#owner-settings), and its time is the selection bound.
The 30-second default suits a policy that consults a table. For an agent, set
`timeoutMs`, up to 600000 (10 minutes).

**Keeping the agent from executing the task is your job**, as
[above](#a-policy): it holds the mandate, in the directory you started it in,
and nothing in harness-dispatch or Grove stops it from carrying the task out.
Tell it to evaluate and not to act, and give it no tools that write where its
harness lets you choose.

The types for all of this, `Policy`, `SelectionRequest`, `Selected`, `Refused`,
`DeliveredContext` and `SelectHost`, are in `harness-dispatch/sdk`, and
`definePolicy` checks your policy against them and types `select`'s arguments
for you.

`harness-dispatch/sdk` is built into the worker, so there is nothing to install.
For editor type checking, map the specifiers to the declarations beside the
worker:

```json
{
  "compilerOptions": {
    "paths": {
      "harness-dispatch/sdk": ["<prefix>/libexec/harness-dispatch/sdk/index.d.ts"],
      "harness-dispatch/grove": ["<prefix>/libexec/harness-dispatch/grove/index.d.ts"],
      "harness-dispatch/examples/*": ["<prefix>/libexec/harness-dispatch/examples/*.d.ts"]
    }
  }
}
```

The readable sources, `sdk/index.ts`, `grove/index.ts` and `examples/*.ts`,
sit beside the declarations.

## Context

A policy can be given more than the kind, the prompt and the parameters. A
caller supplies a version-1 JSON document with `--context`, and a policy can
assemble its own context in a `loadContext` callback. Either way, what `select` receives is measured,
hashed and bounded, and inspection shows exactly that.

### The context document

```json
{
  "schemaVersion": 1,
  "summary": "Rename the --verbose flag to --trace",
  "acceptanceCriteria": ["--verbose still parses, with a deprecation warning"],
  "facts": { "filesTouched": 3, "publicInterface": true },
  "assessments": { "risk": { "by": "triage", "value": "medium" } },
  "sources": [{ "name": "https://example.com/issues/12", "version": "2026-09-30" }],
  "reviewedArtifact": { "id": "parser-k3", "creator": { "declared": "anthropic" } }
}
```

Only `schemaVersion`, which is `1`, is required:

| Field | Value |
|---|---|
| `summary` | A string |
| `acceptanceCriteria` | An array of strings |
| `facts` | An object of any JSON: the caller's facts, as data |
| `assessments` | An object whose members are each `{ by, value }`: a judgment, attributed to whoever made it |
| `sources` | At most 256 source records, `{ name, sha256?, bytes?, version? }`, each with a nonblank `name` and a `sha256` (64 lowercase hexadecimal digits), a `version` or both, so that the evidence is pinned |
| `reviewedArtifact` | `{ id, creator? }`: the artifact a review is of, and at most one creator form, `{ run: "<run ID>" }` or `{ declared: "<provider>" }` |

A field you leave out stays out, and an empty one stays empty: `"facts": {}`
says there are no facts, while no `facts` says nothing about them. Nothing is
filled in for you. An unknown field refuses, and so does a later version. A
field named like an executable one, such as `program`, `args` or `select`,
refuses with a message saying that a context is data. harness-dispatch takes no
part of a command from a context: what your `select` builds from one is your
policy's. The document is read once, relative to the current directory, and
must be a regular file within the [context budget](#bounds). It is checked
before any policy runs, and a refusal names its `location`, such as
`context.reviewedArtifact.creator.run`.

A policy without a loader sees the document as `request.context` and, with
its measured source attached, as the `context` argument of `select`. A `select`
that reads no context is given one all the same, and it is still measured and
inspected.

### Loading context

```ts
import { definePolicy, type Context } from "harness-dispatch/sdk";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "2026-10-02",
  loadContext(request, host): Context {
    // A task file, when the caller names one, is read through the host, so it is measured.
    const task = request.taskFile === undefined ? undefined : host.readText(request.taskFile, 16384);
    return {
      ...request.context,
      schemaVersion: 1,
      ...(task === undefined ? {} : { summary: task.text.split("\n")[0], sources: [task.source] }),
    };
  },
  select(request, context) {
    const risky = context?.assessments?.["risk"]?.value === "high";
    const model = risky ? "claude-opus-5-5" : "claude-haiku-4-5";
    return {
      status: "selected",
      program: "claude",
      args: ["--model", model, request.prompt],
      provider: "anthropic",
      model,
      effort: risky ? "high" : "low",
      reason: `risk is ${risky ? "high" : "not high"}`,
    };
  },
});
```

A policy may export `loadContext(request, host)`. It runs once the policy's
shape has been accepted, and before `select`. It may be `async`, and it runs
within the selection bound. It returns a
version-1 context, shaped like the document above, and that is what `select`
receives. Nothing is selected without it. A loader that throws or rejects
refuses as `context_loader_failed`, one that is still pending when nothing is
left to settle it refuses as `context_loader_unsettled`, and one that returns
nothing or an invalid context refuses as `context_invalid` with the location.
Return plain JSON data: a function, a `NaN` and a cycle each refuse where they
sit. The request it receives is frozen, so build a new context rather than
changing `request.context`.

A loader that finds what it requires present but wrong can say so, rather than
throw: it returns a refusal in `select`'s shape.

```ts
loadContext(request, host) {
  const ticket = host.readText("ticket.md");
  if (ticket.text.includes("Status: closed")) {
    return { status: "refused", code: "ticket_closed", message: "ticket.md says the ticket is closed", remedy: "reopen the ticket, or run without it" };
  }
  return { schemaVersion: 1, sources: [ticket.source] };
}
```

That is the policy's own refusal: `policy_refused`, exit 3, at stage
`context`, with its code as `policyCode` and its remedy. `select` is not
called. A context has no `status`, so a loader's result that has one is
judged as a refusal, and anything but `{ status: "refused", code, message,
remedy }`, each nonblank, refuses as `context_invalid`. A bound the loader
exceeded is reported in its place.

The `host` a loader receives reads files for it:

| Operation | What it does |
|---|---|
| `host.readText(path, maxBytes?)` | Reads a UTF-8 text file once and returns `{ text, source }` |
| `host.readJson(path, maxBytes?)` | Reads a JSON file the same way and returns `{ value, source }` |
| `host.run(runId)` | Looks up a recorded run: see [looking up a run](#looking-up-a-run) |
| `host.diagnostic(text)` | Writes one line to the policy's stderr, which inspection shows and the run notice carries |
| `host.signal` | An `AbortSignal`, aborted when harness-dispatch stops the selection at its deadline or on an [interrupt](#interrupting-a-selection), so that a `fetch` in flight can stop too |

A relative `path` resolves against the caller's directory, `request.cwd`, and
never against the worker's own. Your policy's `import`s still resolve from its
own file. A read must name a regular file. It reads at most `maxBytes`, which
defaults to 64 KiB and may be raised to the whole context budget, but no
further. `source` is `{ name, bytes, sha256 }`: the canonical path, the number
of bytes read and their SHA-256, ready to attribute in `sources`. A missing
file, one that is not UTF-8 or, for `readJson`, not JSON throws. If the loader
fails because of that, even as the `cause` of its own error, the refusal is
`context_source_unreadable` and names the file. A source the policy can do
without is simply one it catches.

`select`'s host has `diagnostic` and `signal`, and no reads or lookups: it
receives the context the loader returned, as it was measured, and reads
nothing more through the host. A read or lookup the loader leaves for later
fails once its context is delivered. Trusted policy can still read anything
through Bun's own APIs, or fetch over HTTP. Those reads are not measured, so
put the versions or digests of anything you use that way into `sources`
yourself.

### Looking up a run

`host.run(runId)` looks a run up in the record store this invocation uses, the
one `--state-dir` names or the default. It returns the run's immutable launch
fields, as `run` committed them before it handed off. It is how a review
policy learns the provider its artifact's creator ran under: the creator's
session names its own `HARNESS_DISPATCH_RUN_ID`, and the record says what that
run was launched as, whatever the policy returns today.

```ts
loadContext(request, host) {
  const context = request.context ?? { schemaVersion: 1 };
  const run = context.reviewedArtifact?.creator?.run;
  if (run !== undefined) host.run(run);
  return context;
},
select(request, context) {
  const run = context?.reviewedArtifact?.creator?.run;
  const creator = context?.runs?.find((lookup) => lookup.runId === run);
  // ...
},
```

A run the store holds answers with its launch fields:

```json
{
  "runId": "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34",
  "status": "found",
  "recordedAt": "2026-09-30T08:15:42.117Z",
  "kind": "impl",
  "taskId": "T-12",
  "provider": "anthropic",
  "model": "model-large",
  "effort": "high",
  "launchFailure": null
}
```

`taskId` is `null` for a run given none. `launchFailure` is `null`, or
harness-dispatch's own record that the harness never started, as
[`record show`](#run-records) exports it, with its `cause`. `null` does not
mean the harness ran: whether it did is not known from the record alone. The
labels are the ones the run was launched under, and a run recorded under the
catalog contract answers with its labels the same way. The answer never
includes the program, the arguments or the argv, which holds the prompt. There
is no lookup by task identity, artifact or handle. A run is found only by its
ID, written exactly as harness-dispatch reported it. Anything else throws a
`TypeError`, and nothing is looked up.

A run the store does not hold answers `{ "runId": "…", "status": "missing" }`.
So does a store that does not exist yet, or holds nothing: your policy decides
what a missing run means. A store that exists but cannot be read is different.
So is one that is corrupt, another application's, another version, or a
record this release cannot read. Each refuses the whole selection with exit 4,
as the [refusals](#refusals) list them, naming the store. The policy never sees
it as a missing run and cannot catch it: harness-dispatch stops the worker at
once. A lookup waits for another process's lock as a commit does, at most 2
seconds. Its wait is part of the selection bound, though, and never passes it:
a wait that reaches the deadline is `selection_timeout`, exit 124. It reads
the run's launch fields and any launch failure, never its observations, so a
run's long history of observations does not slow a lookup of it.

Every answer is also delivered to `select`, in the context's `runs`, in the
order the loader asked. A loader cannot supply `runs` itself, so a provider
`select` finds there is always the record's. Each lookup is a measured source
too, counted against the 256-source bound: named by its run ID, with `via`
`run`, and sized and hashed over its answer's encoding. `inspect` answers
lookups from the same store as `run`, so the two make the same selection. It reads
the store to do so, and never creates or writes one.

### What select receives, and what inspection shows

harness-dispatch attaches every source it measured to the context, as
`measured`: the `--context` document first, then each read and run lookup in
order, each `{ name, via, bytes, sha256 }` with `via` naming `--context`,
`readText`, `readJson` or `run`. When the loader looked a run up, it attaches
the answers as `runs` as well. A loader can supply neither itself. The context
with them attached is the **delivered context**. `select` receives it frozen,
and its whole encoding, `measured` and `runs` included, must fit the context
budget.

Inspection reports it as `context`: whether a loader assembled it, every
measured source, their total bytes (`sourceBytes`), the delivered context's
encoded size (`encodedBytes`) and its SHA-256, and in `--json` the delivered
value itself. Sizes are bytes of UTF-8 JSON, measured as harness-dispatch
encodes the context: compactly, with object keys in sorted order. The digest is
of that same encoding. A context's `reviewedArtifact` is reported beside it,
and so is its `creator`, when the artifact names one. The creator is the
reference, its evidence class (`execution_recorded` for a run, `declared` for
an owner's declaration), the provider it gives, and for a run the first answer
the loader got for it:

```json
"creator": {
  "reference": { "run": "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34" },
  "evidence": "execution_recorded",
  "provider": "anthropic",
  "lookup": { "runId": "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34", "status": "found", "…": "…" }
}
```

The provider is `null` when the run is missing or was never looked up, and
`lookup` is `null` when it was never looked up. A declared creator gives its
declared provider. Text output shows the same as a `creator` row, with the
run's task identity and kind beside its recorded labels. harness-dispatch
cannot tell what a policy made of the creator: this is what its context
carried.

## Starter examples

Four policies ship inside the worker as editable starting points. None is
active until your own policy imports it.

| Specifier | Kinds it routes |
|---|---|
| `harness-dispatch/examples/static` | A caller's own kinds, without Grove: `question`, `bugfix`, `feature`, `migration` and `architecture`, over one harness at four efforts |
| `harness-dispatch/examples/grove-static` | All 23 of Grove's session kinds, exactly, over a lead harness and a reviewer from another provider |
| `harness-dispatch/examples/review` | The static example's kinds, and two review kinds under the provider rule: a review runs on another provider origin than its artifact's creator. See [the review policy](#the-review-policy) |
| `harness-dispatch/examples/grove-review` | Grove's session kinds as the Grove example routes them, with Grove's five review kinds under the provider rule, each review's artifact and creator read from its task file. See [the Grove review policy](#the-grove-review-policy) |

The two static examples are a `select` that consults an exact table by kind: a
kind one does not list refuses, with the policy's own code `incomplete_mapping`.
Each table entry is a function from the request to a command, so the prompt
reaches an argument because the entry puts it there. Each example explains,
kind by kind, why the work gets the effort it does, in terms of abstraction,
uncertainty, consequences, downstream repair, reversibility and available
checks. None ranks models. These are priors, not calibrated estimates. Their
programs, such as `my-codex-wrapper`, are illustrative wrappers you supply. Each
receives `--model`, `--effort` and the prompt, and should exec your harness
with them. Their models and providers are placeholders. The Grove example
routes reviews to the other provider, but a table keyed by kind cannot see
which provider actually created the artifact a review reads, so it enforces no
provider rule.

Use one whole from your personal policy:

```ts
export { policy } from "harness-dispatch/examples/grove-static";
```

Or select through a table of your own with the static example's `selectRoute`,
which returns the entry's command with a reason naming the entry, or the
refusal for a kind the table lacks:

```ts
import { definePolicy } from "harness-dispatch/sdk";
import { selectRoute, type Route } from "harness-dispatch/examples/static";

const claude = (effort: string): Route => (request) => ({
  program: "claude",
  args: ["--model", "claude-opus-5-5", "--effort", effort, request.prompt],
  provider: "anthropic",
  model: "claude-opus-5-5",
  effort,
});

export const policy = definePolicy({
  schemaVersion: 2,
  version: "mine-1",
  select: (request) => selectRoute({ design: claude("high"), impl: claude("medium") }, request),
});
```

Better still, copy `examples/grove-static.ts` or `examples/static.ts` from
beside the worker to `~/.config/harness-dispatch/policy.ts` and edit it. It
imports `harness-dispatch/sdk` as your own policy does, so the copy works where
it lands. A copy stays yours, whereas an example you import changes with the
installation.

### The review policy

`harness-dispatch/examples/review` applies one rule to the review kinds it
lists: every review it selects runs a command whose provider origin differs
from the original creator's, on every invocation and every retry. The rule is
this policy's, not harness-dispatch's: the command compares no providers and
knows no review kinds. Activate it from your personal policy:

```ts
export { policy } from "harness-dispatch/examples/review";
```

It builds on the static example: that example's routes, and a second harness
from another origin (`my-other-agent-wrapper`, `your-other-provider`) for
reviews. Every kind the static example routes is routed as it is there, and
the rule does not apply to it.

A review names its artifact and the artifact's creator in its
[context document](#the-context-document), and nothing else supplies them:

```json
{ "schemaVersion": 1, "reviewedArtifact": { "id": "parser-k3", "creator": { "run": "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34" } } }
```

```sh
harness-dispatch run --kind code-review --context review.json --prompt-file review.md
```

The creator is the run that made the artifact, `{ "run": "<run ID>" }`, whose
provider the record store holds, whatever the policy returns today. The session
that ran it can read its ID from `HARNESS_DISPATCH_RUN_ID`. For an artifact
made without such a run, before you adopted harness-dispatch or by a harness
launched directly, you declare its origin instead, `{ "declared":
"your-provider" }`, and inspection and the run record label it declared. A
reviewed artifact of any ID is admitted with a run of any task identity: a run
reference is its writer's word for which run made the artifact, and inspection
shows the run's task beside the reviewed ID so that a mismatch can be seen.

For each review kind, the policy's `reviews` table maps each origin a creator
can have to the command that reviews its work:

```ts
export const reviews = {
  "code-review": { "your-provider": otherAgent("high"), "your-other-provider": agent("high") },
  "architecture-review": { "your-provider": otherAgent("xhigh"), "your-other-provider": agent("xhigh") },
};
```

So whichever provider made the artifact, the other one reviews it, and a
change to which provider does your producing work needs no change here. In
order, a review kind's selection:

1. looks the creator's run up, and refuses a run the store does not hold or
   one whose harness never executed;
2. requires the creator's origin, recorded or declared, to be one of the
   origins the kind's entry lists, matched exactly, with no normalisation. A
   relabelled origin or a misspelt declaration therefore refuses, naming the
   origins listed, rather than comparing as a different provider;
3. takes the entry's command for that origin;
4. requires that command's `provider` label to differ from the creator's. A
   gateway changes the route to a model, not the model's origin, so a command
   that reaches the creator's model through one keeps that origin's label, and
   can never pass as another provider's review.

Every failure refuses as `policy_refused`, exit 3, and nothing is run in place
of the reviewer it refused. A kind `reviews` does not list takes the static
routes. So a context that names a reviewed artifact under such a kind refuses,
rather than taking a route without the rule: a review label of your own applies
the rule only once you list it.

| `policyCode` | Why | Remedy |
|---|---|---|
| `reviewed_artifact_missing` | A review kind's context names no reviewed artifact | Supply `reviewedArtifact` with the artifact's ID and creator |
| `creator_missing` | The reviewed artifact names no creator | Name the creator's run, or declare its origin |
| `creator_run_missing` | The record store does not hold the creator's run | Review with the store the creator ran with (`--state-dir`), or declare its origin |
| `creator_not_executed` | The creator's run carries a launch failure, so its harness never ran | Name the run that did make the artifact, or declare its origin |
| `creator_origin_unlisted` | The recorded or declared origin is not one the review kind's entry lists | List a relabelled origin's label in the entry, or correct the declaration |
| `same_origin` | The entry's command for the creator's origin carries that same origin | Map that origin to a command of another origin |
| `incomplete_mapping` | A kind that is not a review has no route | Add the route |
| `review_kind_unlisted` | A kind `reviews` does not list names a reviewed artifact | List the kind in `reviews`, or leave `reviewedArtifact` out |
| `creator_not_looked_up` | A policy of your own selects with the example's `select` without looking the run up | Look it up with `lookUpCreator` in your `loadContext` |

To apply the rule to routes and review kinds of your own, use its
`reviewSelector`, which returns the `loadContext` and `select` of such a
policy:

```ts
import { definePolicy } from "harness-dispatch/sdk";
import { reviewSelector } from "harness-dispatch/examples/review";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "mine-1",
  ...reviewSelector({
    routes: { feature: builder },
    reviews: { audit: { "your-provider": auditor, "your-other-provider": builder } },
  }),
});
```

Here `builder` and `auditor` are routes of your own, each a function from the
request to a command, as [the starter examples](#starter-examples) show. A
policy that assembles its own context, from a task file say, looks the
creator up by passing that context to `lookUpCreator(context, host)` in its
`loadContext`, and selects with the `select` that `reviewSelector` returns.

### The Grove review policy

`harness-dispatch/examples/grove-review` applies the review policy's rule to
Grove's reviews. Grove passes each session's kind, task file and handle
([called from Grove](#called-from-grove)). A Grove review leaf names what it
reviews in its own body:

```markdown
# parser-k13

**Reviews:** parser-k12
**Creator:** run 5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34
```

`**Reviews:**` is the reviewed producer's handle. `**Creator:**` is that
producer's original creator, in one of two forms. `run <run ID>` names the
dispatch run of the session that finished the producer, its
`HARNESS_DISPATCH_RUN_ID`, and the creator's origin is the one the record store
holds for that run. `declared <origin>` is your declaration for a producer
finished without harness-dispatch: before you adopted it, or by a harness Grove
launched directly. Inspection and the run record label it declared.

Grove's sessions write the run form themselves. The session that finishes a
producer, by retiring its leaf or closing its node, names its own run on each
live review of that producer, and one that finishes it without harness-dispatch
removes the line (the Grove plugin's `references/retire.md`). The declaration
is yours alone to write, as "When the creator line is missing" describes below.

Activate it from your personal policy, with Grove's dispatch command:

```ts
export { policy } from "harness-dispatch/examples/grove-review";
```

It keeps the Grove static example's two harnesses and its routes for every kind
but Grove's five reviews: `review-requirements`, `review-design`,
`review-planning`, `review-prototype` and `review-impl`. Each of those maps the
creator's origin to a reviewer from the other provider, at the effort the
static example gives that review. So every review it selects runs on another
origin than its producer's creator, on every invocation and every retry, by the
checks [the review policy](#the-review-policy) lists.

Those harnesses and routes are the installed static example's, whatever a
copy of it you edited says. A copy of `examples/grove-review.ts` imports them
by the same name, so it keeps them too. To apply the rule over your edited
copy, keep that copy beside your policy as `grove-static.ts`, make a copy of
this example your policy, and change its import of
`harness-dispatch/examples/grove-static` to `./grove-static.ts`
([activating it](../../docs/CONFIGURATION.md#harness-dispatch) has the steps).
Or build on `groveReviewSelector`, as shown below.

The task file is read by the Grove adapter, `harness-dispatch/grove`. It is
built into the worker, and nothing calls it unless your policy imports it,
directly or through this example. It reads the one task file
Grove supplies, through the host's measured read, on every invocation and for
every kind. Inspection therefore lists that file among the context's sources
with its digest, and the run record keeps the digest. It reads the whole file,
up to the context budget (`--context-bytes`), rather than the 64 KiB a read
takes by default. It takes the kind from `--kind`, never from the file's name.
It resolves no handle, and reads no brief, sibling or other file of the tree.

A line counts when it begins with `**Reviews:**` or `**Creator:**` at its first
character. A mention inside a sentence does not, and nor does an indented line.
A counted line is exactly the marker, one space and the value, with nothing
after it:

- for `**Reviews:**`, a handle, such as `parser-k12`;
- for `**Creator:**`, `run` and a run ID exactly as `HARNESS_DISPATCH_RUN_ID`
  gave it, or `declared` and an origin.

A declared origin is the rest of the line, verbatim, and must be one of the
origins the review kind's entry lists, exactly. The adapter has no markdown parser, so a line quoted
in a fenced block counts too, and refuses as a second line.

A review kind needs exactly one line of each. Any other kind whose task file has
a `**Reviews:**` line refuses, so a review of a kind you have not listed cannot
take a static route. The adapter's refusals come from `loadContext`, at stage
`context`:

| `policyCode` | Why | Remedy |
|---|---|---|
| `task_file_missing` | A review kind was given no `--task-file` | Pass the review's task file, as Grove's dispatch command does with `${task_file}` |
| `reviewed_artifact_conflict` | A `--context` document names a reviewed artifact too | Leave `reviewedArtifact` out of the document |
| `reviews_line_missing` | A review kind's task file has no `**Reviews:**` line | Add `**Reviews:** <handle>`, naming the producer it reviews |
| `reviews_line_duplicate` | It has more than one | Keep one; reword or indent the others, a fenced example included |
| `reviews_line_malformed` | The line is not `**Reviews:** <handle>` | Write the producer's handle, such as `parser-k12`, with one space before it and nothing after it |
| `creator_line_missing` | A review kind's task file has no `**Creator:**` line | Declare the origin of a producer finished without a run: see below |
| `creator_line_duplicate` | It has more than one | Keep the one for the session that finished the producer |
| `creator_line_malformed` | The line is neither `run <run ID>` nor `declared <origin>` | Write the run ID exactly as `HARNESS_DISPATCH_RUN_ID` gave it, or the declaration |
| `review_kind_unlisted` | A kind the policy does not list as a review has a `**Reviews:**` line | List the kind in `reviews` in your copy of the policy, or remove the line if the task is not a review |

A task file that cannot be read refuses as `context_source_unreadable`, naming
it. One larger than the context budget refuses as `source_too_large`: raise
`--context-bytes` in the command Grove runs. Once the lines are read, the
review policy's own refusals follow, such as `creator_run_missing` and
`creator_origin_unlisted` (see [the review policy](#the-review-policy)).

**When the creator line is missing.** The session that finishes a producer
settles the line on each live review naming it. Launched through
harness-dispatch, it writes `**Creator:** run <run ID>` from its own
`HARNESS_DISPATCH_RUN_ID`, replacing any line there. Launched without it, it has
no run to name, so it removes the line, including one that an earlier
dispatched attempt wrote. A review therefore reaches its launch with no line
when its producer was finished without harness-dispatch. harness-dispatch never
looks a run up by its task, so a run of the producer's task in the record store
does not stand in for the line. Declare the origin that finished the producer,
as your policy labels it, directly under the review's `**Reviews:**` line:

```markdown
**Reviews:** parser-k12
**Creator:** declared openai
```

Then run the refused launch's `inspect:` line until it reports a reviewer, and
run Grove again. Declare once the producer is finished, because a session that
finishes it later removes the line with any other. If a dispatched session did
finish the producer and left no line, write `**Creator:** run <run ID>` with
that session's own run ID, not an earlier attempt's.

**A run line is its writer's word.** harness-dispatch records which provider
each run launched. It cannot tell which run made an artifact. A line naming
some other run that exists lends that run's provider, and the launch does not
detect it. The launch does not compare the named run's task with the
`**Reviews:**` handle either, because a decomposed producer is finished by a
child task with a handle of its own. Inspection shows the two side by side:

```text
  reviewed   {"creator":{"run":"5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34"},"id":"parser-k12"}
  creator    run 5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34 (execution-recorded): provider openai, model your-codex-model, effort high; task tokens-k15, kind impl, recorded 2026-10-01T09:12:44.501Z
```

Here the run's task, `tokens-k15`, is not the reviewed `parser-k12`: a child
whose retirement closed the decomposed producer's node. Read these two lines
when a review's reviewer is not the one you expected. The creator line is in
the review's task file, so version control shows every change to it. A run is
found only in the record store that recorded it, so producer and review must
use the same one.

**Attaching a review's findings.** Because the review's task file names the
producer's run, the review can record what it found against that run with
`harness-dispatch record observe --run <run ID> --file observation.json`
([observations](#observations)). The record store is outside the tree, so the
run and its observations remain after Grove removes `.grove/`.

To apply the rule to routes and review kinds of your own, use the example's
`groveReviewSelector`, as you would the review policy's `reviewSelector`. The
Grove static example exports its two harnesses, `lead(effort)` and
`review(effort)`, as routes to build on:

```ts
import { definePolicy } from "harness-dispatch/sdk";
import { groveReviewSelector } from "harness-dispatch/examples/grove-review";
import { lead, review } from "harness-dispatch/examples/grove-static";

export const policy = definePolicy({
  schemaVersion: 2,
  version: "mine-1",
  ...groveReviewSelector({
    routes: { impl: lead("high") },
    reviews: { "review-impl": { openai: review("high"), anthropic: lead("high") } },
  }),
});
```

Or call the adapter from a loader of your own. `groveContext(request, host,
reviews)` returns the context the task file declares, or the refusal, and the
review policy's `lookUpCreator` looks the creator up before you return it:

```ts
import { groveContext } from "harness-dispatch/grove";
import { lookUpCreator } from "harness-dispatch/examples/review";

// Inside definePolicy({ … }):
loadContext(request, host) {
  const context = groveContext(request, host, reviews);
  return "status" in context ? context : lookUpCreator(context, host);
},
```

`groveContext` reads the `reviews` table's keys only, as the review kinds.

## Inputs

| Input | Meaning |
|---|---|
| `--kind TEXT` | Required. Any nonempty token, given to `select` as it is. Nothing else supplies the kind. |
| `--prompt TEXT` or `--prompt-file PATH` | The harness prompt, given to `select`. `run` needs exactly one; `inspect` selects with a [marker](#inspect) without either. |
| `--task-file PATH` | Optional. Resolved against the current directory and passed on as data. harness-dispatch itself does not read it, and it need not exist, but a policy's loader may read it, as [the Grove adapter](#the-grove-review-policy) does. It supplies no kind or identity. |
| `--task-id ID` | Optional. The task's stable identity, such as a Grove handle: opaque UTF-8 of at most 1024 bytes. |
| `--config PATH` | Optional. The policy entry to use instead of the personal default. |
| `--param NAME=VALUE` | Optional and repeatable. Caller data for `select`, by name. See [parameters](#parameters). |
| `--context PATH` | Optional. A version-1 JSON context document, read as data, relative to the current directory. See [context](#context). |
| `--timeout-ms MS` | Optional. The whole-selection bound in milliseconds, from 1000 to 600000, replacing the `timeoutMs` [owner setting](#owner-settings). The default is 30000. See [the selection bound](#the-selection-bound). |
| `--context-bytes BYTES` | Optional. The context budget in bytes, from 1 to 8388608 (8 MiB), replacing the `contextBytes` owner setting. The default is 262144 (256 KiB). See [bounds](#bounds). |
| `--state-dir PATH` | Optional. The directory holding run records, resolved against the current directory, replacing the `stateDir` owner setting. The default is `~/.local/state/harness-dispatch`. See [run records](#run-records). |
| `--policy-env NAME` | Optional and repeatable. Give the policy this environment variable, by exact name, from harness-dispatch's own environment, beside the names the `policyEnv` owner setting grants. See [the policy's environment](#the-policys-environment). |
| `--json` | Optional. Report as JSON. `inspect` prints one version-2 object on stdout, `run` writes its handoff notice as one JSON line on stderr, and each writes a refusal as JSON on stderr. See [inspect](#inspect), [run](#run) and [refusals](#refusals). |

The prompt is read once and kept byte for byte, trailing newlines included. It
must be valid UTF-8 with no NUL, and at most 1 MiB. A prompt file is resolved
against the current directory. A prompt that fails these checks, a file that
cannot be read and a file that is a terminal all refuse with exit 2 before any
policy runs. harness-dispatch never reads its own stdin, which stays the
harness's. The policy receives the prompt as `request.prompt` and places it in
the arguments it returns; harness-dispatch puts it nowhere itself.

## Parameters

`--param NAME=VALUE` passes one named string to the policy, and may be repeated.
`select` and `loadContext` read them as `request.params`, an object from name
to value, which is empty when the caller passed none. harness-dispatch gives a
parameter no meaning: its name and value are the caller's, and only your policy
reads them. A value reaches an argument because your function puts it there.

```sh
harness-dispatch inspect --kind impl --param repo=/work/parser --param session_name='parser: impl'
harness-dispatch run --kind impl --param repo=/work/parser --prompt 'Rename the flag'
```

```ts
select(request) {
  const repo = request.params["repo"];
  if (repo === undefined) {
    return { status: "refused", code: "repo_missing", message: "no repo parameter was passed", remedy: "pass --param repo=PATH" };
  }
  return {
    status: "selected",
    program: "codex",
    args: ["-C", repo, `--name=${request.params["session_name"] ?? request.kind}`, request.prompt],
    provider: "openai", model: "your-codex-model", effort: "high",
    reason: `kind ${request.kind} in ${repo}`,
  };
}
```

The name is everything before the first `=`, so it is nonempty and holds no
`=`. The value is everything after it, exactly as given: it may be empty, and
may hold `=`, spaces and newlines. A name given twice, a word with no `=` and
an empty name each refuse as `malformed_input`, exit 2, before any policy runs.
So do names and values of more than 64 KiB together. A parameter the caller did
not pass is absent, so a policy that needs one refuses for itself, as above.

A parameter is also how a caller steers one invocation: pass something your
`select` reads, such as `--param effort=high`. There is no input that names a
command, and nothing overrides a part of what `select` returns. Each
invocation, a retry included, evaluates the policy afresh.

## Bounds

Every selection runs within these bounds. None of them is met by cutting
something short: each overflow refuses, names its bound, and launches nothing.
Your policy sees their values in `request.limits`, but it cannot raise one,
and a policy that catches the error a bound throws is refused all the same.

| Bound | Default | Limit, and what exceeding it does |
|---|---|---|
| The whole selection, from the worker's start to its result, any child the policy starts included | 30 seconds | `timeoutMs` or `--timeout-ms` sets 1 to 600 seconds; `selection_timeout`, exit 124 |
| The delivered context's encoded JSON, `measured` included | 256 KiB | `contextBytes` or `--context-bytes` sets 1 byte to 8 MiB; `context_too_large` |
| One host read | 64 KiB, or the context budget if that is smaller | A read's `maxBytes` sets up to the context budget; `source_too_large` |
| Measured sources, the `--context` document included, and records in a context's `sources` | 256 | Fixed; `too_many_sources` |
| The exported policy object, or `select`'s result, as a protocol message | 1 MiB | Fixed; `message_too_large` |
| The parameters, every name and value together | 64 KiB | Fixed; `malformed_input`, exit 2 |
| What the policy prints, stdout and stderr together | 256 KiB | Fixed; `output_limit` |
| The prompt | 1 MiB | Fixed; `prompt_invalid`, exit 2 |

The prompt has its own budget and never counts against the context budget. A
result holds the arguments, the prompt among them, so a prompt near its bound
can put the result over the message bound: have the harness read such a prompt
from a file an argument names. Output past its bound is read and discarded, so that the policy never
blocks writing it, and the worker is stopped at once. The first 256 KiB are
kept in `diagnostics`. Inspection reports every bound, with `from` saying
whether it is the `default`, `fixed`, or set by `settings.json` (the
[owner settings](#owner-settings)), `--timeout-ms`, `--context-bytes` or a
read's `maxBytes`. A refusal a bound caused names it as `bound`.

### The selection bound

Your policy is trusted TypeScript, and it can hang: a loop at import, or an
`await` on work that never settles. So the whole selection has a wall-clock
bound. It runs from the worker's start to its result, and it covers the
policy's import, everything the policy does while it loads, its
`loadContext` and its `select`, and the time of any child the policy starts
and waits for. The default is 30 seconds, short so that a policy which only
consults a table fails fast. The `timeoutMs` [owner setting](#owner-settings)
sets it for every invocation and `--timeout-ms` for one, anywhere from 1000
(1 second) to 600000 (10 minutes), a ceiling that admits a policy which waits
minutes for a [deciding agent](#handing-the-prompt-to-a-deciding-agent). A
value outside that range, or anything but a whole number, refuses with exit 2
before any policy runs.

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

`bound.from` is `--timeout-ms`, `settings.json` or `default`, and whatever the
policy printed before it was stopped is kept in `diagnostics`. `host.signal` aborts when the
worker is sent TERM, and a policy with no TERM listener of its own then ends
once its abort listeners have run. A worker that returns its result
in time but then does not exit, because an exit handler holds it, also gets one
second before it is killed. Its selection stands. A policy that starts
processes of its own must end them before it returns; the front stops only the
worker. Starting a child under `host.signal` is what stops it with the
selection.

### Interrupting a selection

An INT, TERM or HUP that reaches harness-dispatch while it selects cancels the
selection: a Ctrl-C at the terminal, a `kill`, or a terminal that closes. The
front stops the worker as it does at the deadline, with TERM and at most one
second before KILL, and reaps it. Nothing is recorded or launched. It prints
one refusal, as JSON with `--json`, and then ends by that same signal, so a
shell reports 130, 143 or 129:

```json
{"schemaVersion":1,"error":{"code":"selection_cancelled","stage":"evaluation",
 "message":"selection with the policy /home/me/.config/harness-dispatch/policy.ts was cancelled by SIGINT: its worker was stopped and reaped, and nothing was recorded or launched",
 "source":"/home/me/.config/harness-dispatch/policy.ts","signal":"SIGINT",
 "remedy":"…","exit":130},"diagnostics":{"stdout":"","stderr":""}}
```

`exit` is the status a shell shows for that death, since a process that dies
of a signal has no exit code. A signal received while selecting decides the
outcome, whatever else happened meanwhile: a refusal the policy or the record
store would have caused is reported as the cancellation instead, and so is a
selection that ran out of time. `host.signal` aborts as it does at the
deadline, and what the policy printed is kept in `diagnostics`. A terminal
delivers its interrupt to the worker, and to any process your policy started,
as well as to the front, since they all stay in the caller's job.

A signal that was ignored when harness-dispatch started, as `nohup` ignores
HUP, stays ignored and cannot cancel a selection. A signal the caller blocked
stays blocked, and cannot cancel one either.

`run` goes on handling INT, TERM and HUP after selection, while it records the
run and announces it. Before it execs, it blocks them and looks once more. A
signal seen there launches nothing. The run is already recorded, so it is
marked as never executed, and the refusal names it:

```json
{"schemaVersion":1,"error":{"code":"handoff_cancelled","stage":"exec",
 "message":"the handoff of run 0f8e…, selected with the policy /home/me/.config/harness-dispatch/policy.ts, was cancelled by SIGTERM after its record was committed and before its harness was launched: nothing was launched",
 "source":"/home/me/.config/harness-dispatch/policy.ts",
 "run":{"id":"0f8e…","launchFailure":"recorded"},"signal":"SIGTERM",
 "remedy":"…","exit":143}}
```

With `--json` it is a second JSON line, after the handoff notice, which has
already carried what the policy printed. Then
harness-dispatch dies of the signal, as it does when a selection is
cancelled. [`record show`](#run-records) reports the run as a
`launch_failure` whose harness was `not_executed`, with the cause `cancelled`
and the signal. If that mark cannot be appended, the refusal says
`"launchFailure":"unrecorded"` and why, and the run stays a handoff attempt
whose execution is unknown. It never becomes a success.

One window remains. After that last look, harness-dispatch restores your
signal mask and execs the harness. A signal delivered between the two ends
harness-dispatch by its usual course, and the run stays recorded with its
execution unknown. No program can exec atomically with respect to a signal
that arrives later. A signal after the exec is the harness's.

## The command

A selected result is the command to run. harness-dispatch executes `program`
with `args`, each string one whole argument:

```ts
return {
  status: "selected",
  program: "claude",
  args: ["--model", "claude-opus-5-5", `--add-dir=${request.cwd}`, request.prompt],
  provider: "anthropic",
  model: "claude-opus-5-5",
  effort: "high",
  reason: "every kind runs the same command",
};
```

There is no shell. Nothing is split, globbed, interpolated or read twice, so
spaces, quotes, `$`, `;` and newlines arrive exactly as your function wrote
them. A caller value inside an argument, as in `--add-dir=` above, is ordinary
string building. An argument may be an empty string, and none may hold a NUL
character, which no argument can carry.

`provider`, `model` and `effort` are your labels for what the command runs.
harness-dispatch never infers them from the program or its arguments, and they
need not appear in `args`: a wrapper you run may encode the model itself. They
are your assertion, not verified backend identity. `provider` names the
model's origin; a gateway to a model does not change it. Inspection, the
handoff notice and the run record carry all three, and a later
[run lookup](#looking-up-a-run) answers with them.

## The program

The `program` a selected result returns is one of three things:

- An absolute path, used as it is.
- A relative path containing a `/`, such as `./tools/harness`, resolved against
  the caller's current directory.
- A plain name, looked up in the caller's `PATH` as the shell does: entries in
  order, an empty or relative entry meaning the current directory, and the
  first executable regular file wins. A `PATH` that is set but empty is one
  empty entry, the current directory. With `PATH` unset, no name is found.

A program that cannot be found refuses with exit 127, and one that exists but
cannot be executed with exit 126. A program whose resolved path is not valid
UTF-8 refuses with exit 126 too, under `run` and `inspect` alike: reports and
records name the file by a string, and no string names that one exactly.
harness-dispatch never runs another command instead. Resolution happens once: the resolved file is what `run` executes,
and the program as `select` returned it is the harness's `argv[0]`, as a shell
passes a command as typed.

## Inspect

```sh
harness-dispatch inspect --kind impl
harness-dispatch inspect --kind impl --task-id T-12 --prompt 'Implement the parser'
harness-dispatch inspect --kind review-impl --config ./policies/review.ts --json
```

Inspection is a proposal, not a launch reservation. It launches nothing and
reserves nothing, and a later `run` evaluates the policy afresh. It evaluates
trusted TypeScript, and is not promised to be free of that code's side
effects. It makes the same selection `run` would, and refuses where `run`
would, with the same exit. It reports the policy's path, its authority
(personal or explicit), its SHA-256 and version, the task file, task identity,
parameters and prompt it was given, the [context](#context) selection saw with
its measured sources, any reviewed artifact and its creator provenance, the
selected command with its provider, model and effort labels and the reason,
the file its program resolved to, every effective [bound](#bounds), the
selection time, the worker's identity, any adapter the policy imported, and
where `run` would record. It reports no run ID, because it creates no run.
Inspection never writes the record store, so it cannot tell you in advance
that `run` would find the store unusable for its commit. It reads the store
only to answer the policy's [run lookups](#looking-up-a-run), as `run` does.
Human text shows each argument quoted and escaped. `--json` prints the same
facts on stdout as one version-2 object:

```json
{
  "schemaVersion": 2,
  "evidence": "proposal",
  "stateDir": { "path": "/home/me/.local/state/harness-dispatch", "from": "default" },
  "kind": "impl",
  "taskFile": null,
  "taskId": "T-12",
  "params": { "repo": "/work/parser" },
  "prompt": { "supplied": true, "from": "--prompt", "bytes": 20 },
  "reviewedArtifact": null,
  "creator": null,
  "context": null,
  "policy": { "path": "/home/me/.config/harness-dispatch/policy.ts", "authority": "personal",
              "sha256": "9c1f…", "version": "2026-10-02" },
  "policyEnv": [],
  "selection": { "provider": "anthropic", "model": "claude-opus-5-5", "effort": "high",
                 "reason": "efforts[\"impl\"]" },
  "command": { "program": "claude",
               "args": ["--model", "claude-opus-5-5", "--effort", "high", "Implement the parser"],
               "executable": "/opt/homebrew/bin/claude" },
  "bounds": { "selection": { "ms": 30000, "from": "default" }, "context": { "bytes": 262144, "from": "default" },
              "source": { "bytes": 65536, "from": "default" }, "sources": { "sources": 256, "from": "fixed" },
              "message": { "bytes": 1048576, "from": "fixed" }, "diagnostics": { "bytes": 262144, "from": "fixed" } },
  "timing": { "selectionMs": 15 },
  "worker": { "path": "…/libexec/harness-dispatch/harness-dispatch-policy", "packageVersion": "…", "buildId": "…", "bunVersion": "1.4.2" },
  "adapter": null,
  "diagnostics": { "stdout": "", "stderr": "" }
}
```

`command` is the selected command: the `program` and `args` exactly as
`select` returned them, and `executable`, the absolute path of the file the
program resolved to. It is part of the contract. A caller that must launch the
command itself, such as one that confines it, takes it from there: it executes
`executable` with `args`, and does not resolve `program` again, because that
lookup depends on the directory and `PATH` inspection ran with. Such a launch
has no run record. `selection` holds the labels and the reason, as `select`
gave them. Nothing here says whether the policy consulted a table or computed.

**Without a prompt**, `select` receives a fixed marker in its place,
`<harness-dispatch inspect: no prompt was supplied>`, which
`harness-dispatch/sdk` exports as `PROMPT_NOT_SUPPLIED`. `"prompt"` is then
`{ "supplied": false, "marker": "…" }`. A policy that only places the prompt in
its arguments shows the marker where the prompt goes. A policy that reads the
prompt selects from the marker, so inspection reproduces such a selection only
when it is given the same prompt.

An explicit entry adds `"argument"` to `policy`, the `--config` value as
given. A prompt read from a file reports `"from": "--prompt-file"` and its
`"path"`. `params` is `{}` when no parameter was passed. `stateDir.from` is
`default`, `settings.json` or `--state-dir`. With a context, `context` is
`{ "loader", "sources", "sourceBytes", "encodedBytes", "sha256", "value" }`,
as [context](#what-select-receives-and-what-inspection-shows) describes, and
`reviewedArtifact` is the context's, when it names one, with its `creator`.
Text shows the context's size, digest and each measured source, and leaves the
value to `--json`. It also says how the program was found: as an absolute
path, relative to the current directory, or on `PATH` and in which entry.

`policyEnv` lists each name granted, by the owner settings and then by
`--policy-env`, and whether it was set, as `{ "name", "set" }`; text shows a
`policy env` row. Neither ever shows a value.

`adapter` is `{ "specifier": "harness-dispatch/grove", "version": "1" }` once
the policy has imported the [Grove adapter](#the-grove-review-policy), whether
directly or through an example that composes it, and whether while loading,
in `loadContext` or in `select`. Otherwise it is `null`. Text shows an
`adapter` row. The version is the adapter's own, which changes when what it
reads or how it reads it changes. The worker's version identifies its bytes.

The worker starts in a private empty directory, which only you can read or
write and which is removed afterwards, and moves to `/` before it loads your
policy. So your policy's current directory is `/`, not the directory you ran
harness-dispatch in: that one is `request.cwd`, and a relative path you open
yourself resolves against `/`. The worker has null stdin and the environment
[below](#the-policys-environment). It talks to the front over a
private channel on descriptor 3 and inherits no other descriptors, however high
a descriptor you started harness-dispatch with is numbered. Whatever the
policy prints is captured and kept apart from the report, so no output of its
own, not even text shaped like the report or like a protocol message, can
become part of either. `--json` carries it in
`diagnostics`, and text mode prints it on stderr, each line prefixed with
`policy stdout:` or `policy stderr:`.

### The policy's environment

The policy starts with only HOME, PATH, TMPDIR, LANG and `LC_*` from
harness-dispatch's own environment. Anything else it needs, such as a token
for a routing service it calls, you grant by name:

```sh
harness-dispatch inspect --kind impl --policy-env ROUTER_TOKEN
```

Each `--policy-env NAME` passes that one variable, exactly as named, with its
value, to the policy and to any process it starts. The `policyEnv`
[owner setting](#owner-settings) grants names the same way for every
invocation, and the flag adds to them. A name that is not set is simply
absent. Some names are never granted, and naming one refuses with
`excluded_grant`, exit 2, before any policy runs: Bun's `BUN_*` (`BUN_OPTIONS`
can preload code, and `BUN_BE_BUN` turns the worker into Bun itself);
`NODE_OPTIONS`, `NODE_PATH` and `NODE_PRESERVE_SYMLINKS`, which change what
loads and from where; `NODE_CHANNEL_*`, through which Bun would take the
worker's private channel to its front as its own IPC channel; the dynamic
loaders' `LD_*` and `DYLD_*`; and harness-dispatch's own `HARNESS_DISPATCH_*`,
which it sets for the harness. Other `NODE_*` names, such as
`NODE_EXTRA_CA_CERTS`, can be granted. No value is ever printed or recorded. A
refused `run` reproduces its grants in its `inspect` invocation by name.

harness-dispatch adds one variable of its own,
`BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, which turns Bun's runtime transpiler
cache off. Left on, Bun would run a cached transpilation of any imported file
of 4 KiB or more, kept under your HOME or a granted `XDG_CACHE_HOME`, in place
of the file itself. The worker neither reads nor writes that cache, so what
runs is always the file inspection names. No value of yours replaces the
setting, since `BUN_*` names are never granted.

**Do not grant `GROVE_SIGNAL_FILE`.** Grove ends a session when a file appears
at that path, so a policy holding it could end the session it is selecting for.
harness-dispatch cannot tell a caller's completion variables from any other, so
it does not refuse them. Without a grant, the policy and everything it starts
lack them, and the harness alone receives them, in the caller's unchanged
environment. The same holds for any credential: grant only what the policy
itself uses.

Nothing ambient takes part otherwise. A `.env` file, a `bunfig.toml` preload,
a `tsconfig.json` path alias, or a `node_modules/harness-dispatch` package
beside your entry changes nothing: the worker is compiled with Bun's dotenv,
bunfig and tsconfig autoloading off, it starts in its own empty directory, and
a documented `harness-dispatch/…` specifier that you import by name always
resolves to its embedded module. That holds however the package beside your
entry declares its modules, and where your entry's own package is itself named
`harness-dispatch`. The worker is found only beside the front's real path, so
neither PATH, the current directory, `argv[0]` nor a variable can substitute
another.

Nor does a `node_modules` or a `package.json` in your `TMPDIR`, or in any
directory between the one the worker started in and `/`. A module with no file
of its own, one your policy imports from a `data:` or `blob:` URL or registers
itself, resolves from the worker's current directory upward, and that is `/`.
So `/node_modules` and `/package.json` can still answer one, as they can
answer an import written in any file, and `/package.json` is read on every
evaluation. Imports written in a file resolve from the file. If your policy
changes directory itself, such a module resolves from wherever it moved to.

## Run

```sh
harness-dispatch run --kind impl --prompt 'Implement the parser'
harness-dispatch run --kind impl --task-file ./tasks/parser.md --task-id T-12 --prompt-file ./mandate.md
```

`run` makes the selection `inspect` reports. It then commits the run's handoff
record, described [below](#run-records), and replaces its own process with the
selected command. If the record cannot be committed, nothing is launched and `run`
exits 4. The harness keeps the caller's current directory, descriptors,
environment and process ID, so its exit code or signal is the command's own,
even where the code coincides with one of harness-dispatch's refusal exits.
It also keeps the caller's signal mask and every ignored signal, exactly as
harness-dispatch inherited them: a `nohup` caller's HUP stays ignored, and a
caller that ignored SIGPIPE, or left it at its default, hands the harness the
same. A signal the caller blocked reaches the harness blocked, and still
pending if it arrived meanwhile. Nothing supervises it afterwards. Its environment gains two variables, which
replace any values the caller had:

- `HARNESS_DISPATCH_RUN_ID`, the run's ID. The policy never sees it, so a
  harness that needs it reads it here.
- `HARNESS_DISPATCH_STATE_DIR`, the absolute directory the run was recorded
  in. `record show --state-dir "$HARNESS_DISPATCH_STATE_DIR"` finds the run
  from inside the harness.

Stdout and stdin are the harness's. On stderr, `run` prints whatever the policy
printed, prefixed as in inspection, and then one line naming the command's
labels, the run ID and the file it executes. With `--json`, that line is one
JSON object instead, `{"schemaVersion":1,"handoff":{…},"diagnostics":{…}}`. Its
`handoff` carries `runId`, `recordedAt`, `stateDir` and `kind` beside the
`provider`, `model`, `effort` and `reason` as inspection reports them, and
`executable`.

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

## Called from Grove

Grove launches a lifecycle session through harness-dispatch when a personal
command definition in `~/.config/grove/config.kdl` runs `run` with Grove's task
slots and the prompt:

```kdl
config {
    command "dispatch" "harness-dispatch run --kind ${kind} --task-file ${task_file} --task-id ${task_id} --prompt ${prompt}"
    bind "dispatched" "dispatch"
    route "impl" "dispatched"
}
```

Grove fills the slots from the leaf it selected: its kind, the absolute path of
its task file and its stable handle, each as one argument. The prompt arrives
unchanged, and your `select` receives it as `request.prompt`. To
harness-dispatch these are ordinary inputs, and it reads no Grove file or
filename. Grove's configuration admits the kind and runs this command, and your
policy returns the harness command. Grove checks its own command before it
writes a leaf, but not your policy, which is evaluated only at launch. A
definition can also pass a literal [parameter](#parameters), such as
`--param effort=high`, for your policy to read.
[`harness-dispatch/examples/grove-static`](#starter-examples) routes every
kind Grove ships.
[`harness-dispatch/examples/grove-review`](#the-grove-review-policy) routes them
the same way, and applies the provider rule to Grove's reviews, reading each
review's creator from its task file.

The harness receives Grove's completion channel, `GROVE_SIGNAL_FILE`, in the
environment it inherits. The policy does not, and must not be granted it with
`policyEnv` or `--policy-env`
([the policy's environment](#the-policys-environment)).
[Routing sessions through harness-dispatch](../../docs/CONFIGURATION.md#harness-dispatch),
in Grove's configuration reference, covers activation, both inspection
surfaces and the remedy when a launch refuses.

## Run records

Every `run` records one **handoff attempt** before it execs. The record is
durable intent, not evidence that the harness ran: a harness that exits 0 and
one that is killed at once leave the same record. harness-dispatch records
nothing after exec, because nothing of it is left to observe the harness.

The records live in one SQLite file, `records.sqlite3`, in the state
directory. That is `~/.local/state/harness-dispatch` unless the `stateDir`
[owner setting](#owner-settings) or `--state-dir` names another directory.
`XDG_STATE_HOME` is not consulted. Both the directory
and the file are created on first use, readable by you alone. SQLite is built
into harness-dispatch, so nothing else needs installing. The commit is one
short transaction, synced as SQLite's `synchronous = EXTRA` setting does (with
`F_FULLFSYNC` on macOS). On first use, each directory that gains a new record
directory is synced as well, before the commit. The record then survives a
power loss after the harness starts, and a sync that fails refuses like the
commit. It is taken only after the policy has finished, so no
policy ever runs with the store locked. A policy's run lookups only read it,
each in a short transaction of its own. If another process holds the store,
the commit waits at most 2 seconds, apart from the selection bound, and then
refuses. The store names itself with a version. A store that another
application wrote, that a newer harness-dispatch wrote, or that SQLite finds
corrupt refuses with exit 4. It is left exactly as it was: harness-dispatch
never resets or replaces a store. A launch record or launch-failure detail
this release cannot read, such as one a newer harness-dispatch wrote, refuses
the same way wherever it is read: by `record show`, `record observe` and a run
lookup. So does an observation `record show` cannot read. None of them reads
such a record as a run with less in it.

A run records its ID and the time it was committed. It also records the kind,
the task identity and task file, the parameters, the current directory, and
the policy's path, authority, SHA-256 and version. It keeps the selected
command exactly as `select` returned it, with its provider, model and effort
labels and the reason, the resolved program and the full argv. It keeps the
worker's identity, the effective bounds and the selection time. With a context, it keeps the reviewed artifact the
context names, and the context's measured sources, sizes and digest, but not
the context itself, and as `creator` the creator provenance its context
carried, as [inspection shows it](#what-select-receives-and-what-inspection-shows),
or `null` without one. It keeps the adapter the policy imported, as
[inspection](#inspect) reports it, or `null`. `record show` lists each measured
source with its digest under the context's row, so a Grove review's task file
shows there. No environment value is recorded. The argv includes the prompt, so the
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
  "launch": { "schemaVersion": 1, "kind": "impl", "taskId": "T-12", "params": { "repo": "/work/parser" },
              "candidate": { "id": null, "provider": "anthropic", "model": "claude-opus-5-5", "effort": "high",
                             "program": "claude", "args": ["--model", "claude-opus-5-5", "Implement the parser"] },
              "selection": { "form": null, "selectedBy": null, "explicitChoice": null, "reason": "…" }, "…": "…" },
  "launchFailure": null,
  "observations": [],
  "measurements": { "acceptance": { "state": "unobserved", "current": [] }, "duration": { "state": "unobserved", "current": [] }, "…": "…" }
}
```

`evidence` is `handoff_attempt`, with `execution` `unknown`, until something
more is known. An exec failure appended to the run makes it `launch_failure`,
with `execution` `not_executed`, and `launchFailure` holds the error, with the
cause `exec_error`. A signal seen just before exec does the same, with the
cause `cancelled` and the signal
([interrupting a selection](#interrupting-a-selection)). An
[observation](#observations) that confirms the harness ran makes it
`execution_confirmed`, with `execution` `confirmed`. Every measurement, from
exit and duration to acceptance and findings, is `unobserved` until an
observation supplies it, and an absent measurement is never zero. The run ID
must be exactly as reported, in lowercase. An unknown run refuses with exit 3,
saying whether a store exists at the directory it looked in.

The launch document is version 1, and every field is present in it, `null`
where the run has no value. The labels, program and arguments sit under
`candidate`. Releases up to 21.13.0 selected from a catalog, and a run one of
them recorded also carries what that contract had: a candidate ID
(`candidate.id`), a selection form (`selection.form` and `selectedBy`), an
explicit choice (`selection.explicitChoice`), and slot objects among the
candidate's arguments. A run recorded now has no value for those three and
writes `null`, and it adds `params`. Every command reads both: `record show`
exports either as it was stored, `record observe` appends to either, and a
[run lookup](#looking-up-a-run) answers with the labels of either, so a review
whose creator ran under the catalog contract still resolves that creator's
provider.

## Observations

harness-dispatch cannot see what happens after it execs, so it never records
an outcome. An observer can: a wrapper that saw the harness exit, a review that
judged its work, a person who repaired it. It writes what it saw as one
version-1 observation document, and imports it against the run:

```sh
harness-dispatch record observe --run "$HARNESS_DISPATCH_RUN_ID" --file observation.json
harness-dispatch record observe --run 5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34 --file correction.json --state-dir ./records --json
```

```json
{
  "schemaVersion": 1,
  "observationId": "review-k46-outcome",
  "runId": "5f0e2c41-9b7d-4a3e-8c15-2d6f7a9b0e34",
  "source": "review-impl session for static-dispatch-k12",
  "observedAt": "2026-10-01T09:30:00Z",
  "evidence": "the review's findings, integrated in change qrksvsyv",
  "measurements": {
    "executionConfirmation": { "state": "observed", "value": true },
    "duration": { "state": "observed", "value": 1260, "unit": "s" },
    "acceptance": { "state": "observed", "value": "accepted" },
    "missedDefects": { "state": "observed", "value": [{ "id": "F3", "summary": "unbounded read" }] },
    "totalUsage": { "state": "unknown" }
  }
}
```

The envelope needs every field but `supersedes`. `observationId` is your own
ID for the observation, a nonblank string of at most 1024 bytes, unique in the
store. `runId` must name the run `--run` names. `source` says who observed, and
`evidence` what the observation rests on. `observedAt` is an RFC 3339
date-time, with an uppercase `T` and a `Z` or numeric offset. harness-dispatch
checks the document's shape and that it names this run. It does not check
whether the observation is true: the source and evidence are yours to assert.

Each measurement has a `state`. `observed` carries a `value`, and a quantity
also its `unit`. `unknown` says the observer looked and could not tell.
`unobserved` says it did not look, which is also what a measurement left out
means. Neither carries a value or a unit, so nothing unknown is ever read as
zero, false or accepted.

| Measurement | Observed `value` | `unit` |
|---|---|---|
| `executionConfirmation` | `true`: the harness ran | none |
| `exit` | `{ "code": 0 }` from 0 to 255, or `{ "signal": "SIGTERM" }` | none |
| `duration`, `humanTime` | a number of at least 0 | `ms`, `s`, `min` or `h` |
| `inputUsage`, `outputUsage`, `totalUsage` | a number of at least 0 | any nonblank unit, such as `tokens` or `USD` |
| `acceptance` | `accepted` or `rejected` | none |
| `missedDefects`, `falseFindings` | a list of findings, `{ id, summary?, repairs? }`, each ID once | none |
| `downstreamRepair` | a list of repairs, `{ id, summary?, findings?, runId? }`, each ID once | none |
| `humanInterventions` | a whole number of at least 0 | none |
| `evidenceLinks` | a list of nonblank strings, such as URLs or commit IDs | none |
| `choiceProbability` | the probability the selector gave its choice, from 0 to 1 | none |
| `successProbability` | `{ "probability": 0.8, "calibration": "pilot-2026-10" }`, or `{ "probability": 0.8, "uncalibrated": true }` | none |

A finding's `repairs` name the observations that record its repair, which may
be imported later. A repair's `findings` name the findings it repaired, and its
`runId` the run that made it. A success probability must say which calibration
data or version it came from, or say that it is uncalibrated, so that it is
never mistaken for a choice probability. The shipped policies emit neither.

Importing the same `observationId` with the same content again changes nothing
and succeeds, reporting `"status": "already_recorded"`. Content is compared as
parsed JSON, so layout and key order do not matter. The same ID with other
content, or against another run, is refused as `observation_conflict`: a
recorded observation never changes. To correct one, import a new observation
whose `supersedes` names it. Both are kept, and the export marks the old one
`supersededBy`. The correction replaces it whole, so a measurement the
correction leaves out is no longer current. An observation is corrected at most
once; to correct a correction, supersede the correction. An observation cannot
confirm execution for a run whose launch failure harness-dispatch recorded.
Nothing an observation says changes the run's launch fields, and a document
that tries to carry them is refused.

`record show` lists each observation as it was imported, with every measurement,
those it left out as `unobserved`, and when it was recorded. Its
`measurements` then give, for every field, the `current` values from
observations nothing supersedes, each with its `observationId`, and a `state`:
`observed` if any of them observed the field, `unknown` if one reported that,
and `unobserved` otherwise. When two observers disagree, both values are listed:
harness-dispatch combines and scores nothing.

A store that an older harness-dispatch created, at record schema version 1, is
migrated to version 2 by its first observation, in the same transaction, which
only adds the observations table. Nothing else migrates it.

## Refusals

A refusal launches nothing, and nothing is run in its place. Nothing is
retried, paged or confirmed interactively: each invocation evaluates the policy
once. Every refusal names a stable code, the stage it arose in, a message, the
input or source involved, and a remedy. Text mode prints them on stderr, after
any output the policy printed, each line of it prefixed `policy stdout:` or
`policy stderr:`. `--json` prints one object on stderr,
`{"schemaVersion":1,"error":{…},"diagnostics":{…}}`, and nothing on stdout. The
error names `input`, such as `--kind design`, or `source`, usually the policy
entry, or both, and `location` where the problem is inside a policy, a context
or a `select` result. A policy's own refusal adds `policyCode`, and a
`policy code:` line in text. A refusal
caused by running out of a bound also names that bound, as `bound` in JSON and
a `bound:` line in text. A command line that cannot be parsed is refused the
same way, with clap's tip and usage line in the remedy.

A refused `run` also names the **equivalent `inspect` invocation**: the same
selection inputs, parameters included, with no prompt, from the same directory.
An owner can then reproduce an unattended refusal without reconstructing its
inputs. In text mode it is one command line for a POSIX shell, run in a
subshell so that pasting it leaves your directory alone, and a line saying
that the prompt is left out:

```text
harness-dispatch: refused (policy_refused, stage selection): the policy /home/me/.config/harness-dispatch/policy.ts refused the selection: no command is configured for kind design
  input: --kind design
  source: /home/me/.config/harness-dispatch/policy.ts
  policy code: unrouted
  remedy: add the kind to the efforts table
  inspect: (cd /work && harness-dispatch inspect --kind design --param repo=/work/parser --task-id T-12)
  prompt: omitted: a policy that reads the prompt selects as it did only when the same --prompt or --prompt-file is added
```

In JSON it is `error.inspect`: an argv array, the directory to run it in, and
the same statement about the prompt:

```json
"inspect": { "cwd": "/work", "argv": ["harness-dispatch", "inspect", "--kind", "design", "--param", "repo=/work/parser", "--task-id", "T-12", "--json"],
             "prompt": "omitted: a policy that reads the prompt selects as it did only when the same --prompt or --prompt-file is added" }
```

A policy that only places the prompt in its arguments selects the same command
without it. A policy that reads the prompt receives the
[marker](#inspect) instead, so add the run's own `--prompt` or `--prompt-file`
to reproduce what it selected.

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
what the remedy names, and inspect again until it reports a command. A refusal
at the record commit reproduces as a successful inspection, because inspection
never writes the store. A refusal at a run lookup reproduces, because
inspection reads the store as `run` does.

| Exit | Stage | Codes |
|---|---|---|
| 2 | `cli` | `malformed_input` (including a command line that cannot be parsed, a `--param` that is repeated, has no `=` or has no name, parameters over 64 KiB together, and a `--policy-env` name that is empty or holds `=`), `excluded_grant` (a `--policy-env` name that is never granted), `prompt_invalid`, `prompt_unreadable`; `settings_invalid` (the [owner settings](#owner-settings) file cannot be read or has the wrong shape), and `malformed_input` or `excluded_grant` for a setting its flag would refuse, each naming the file as `source` and the key as `location` |
| 3 | `authority` | `policy_missing`, `policy_unreadable`, `home_unset`, `cwd_unavailable`; for `init`, `policy_exists` (something is already at the personal default path) and `policy_unwritable` |
| 3 | `load` | `policy_import_failed` (a missing import, the entry threw while loading, or an await in it never settled) |
| 3 | `load` | `message_too_large` (the exported policy object is over 1 MiB) |
| 3 | `validation` | `policy_invalid`, and `unsupported_version` (a version-1 policy included), each with its `location` |
| 3 | `context` | `context_unreadable` (the `--context` file); `context_invalid` and `unsupported_version`, each with its `location`; `context_loader_failed`, `context_loader_unsettled` and `context_source_unreadable` (see [loading context](#loading-context)); `policy_refused` (a loader's own refusal, with `policyCode`); `context_too_large`, `source_too_large` and `too_many_sources` (see [bounds](#bounds)) |
| 3 | `selection` | `policy_refused` (the policy's own refusal, with `policyCode`); `selection_threw`, `selection_unsettled`, `selection_abstained` and `selection_malformed` (see [a policy](#a-policy)); `message_too_large` (a result over 1 MiB) |
| 3 | `evaluation` | `output_limit` (the policy printed more than 256 KiB) |
| 3 | `record` | `run_not_found` (`record show` or `record observe` of a run the store does not hold), `cwd_unavailable`; for `record observe`, `observation_conflict`, `supersedes_unknown`, `already_superseded` and `observation_contradicts_record` (see [observations](#observations)) |
| 3 | `observation` | `observation_unreadable` (the `--file`); `observation_too_large` (over the fixed 1 MiB bound); `observation_invalid` and `unsupported_version`, each with its `location` |
| 4 | `record` | `record_store_unwritable`, `record_store_locked` (held past the 2-second wait), `record_store_full`, `record_store_invalid` (another application's file, another version, or corrupt), `record_commit_failed`, `run_id_unavailable`, `home_unset` (HOME cannot place the default state directory); the same store codes when a [run lookup](#looking-up-a-run) cannot read the store, `record_store_invalid` also for a launch record this release cannot read, in `record show`, `record observe` or a run lookup, and for an observation `record show` cannot read |
| 5 | `worker` | `worker_missing`, `worker_identity_mismatch`, `worker_failed`, `protocol_error` |
| 5 | `evaluation` | `cancellation_unavailable` (the handlers that let a signal cancel selection cannot be installed) |
| 5 | `exec` | `signal_state_unavailable` (this build did not record its caller's signal state before the Rust runtime changed it, so `run` cannot hand that state on; nothing was evaluated) |
| 124 | `evaluation` | `selection_timeout` (the selection bound ran out, a run lookup's lock wait included) |
| 128 + N | `evaluation` | `selection_cancelled` (INT, TERM or HUP while selecting; the process then dies of that signal, which a shell reports as 130, 143 or 129: see [interrupting a selection](#interrupting-a-selection)) |
| 128 + N | `exec` | `handoff_cancelled` (INT, TERM or HUP after the run was recorded and before exec; the run is marked not executed, and the process then dies of that signal) |
| 126 | `resolution`, `exec` | `program_unexecutable`; `exec_failed` for any exec error but `ENOENT` |
| 127 | `resolution`, `exec` | `program_not_found`; `exec_failed` for `ENOENT` |

Once the harness runs, its own exit status or signal is the command's, even
where the code coincides with one of these.
