# harness-dispatch

`harness-dispatch` evaluates an owner's TypeScript selection policy and reports
which harness, model and reasoning effort it chooses for a session kind. It is
independent of Grove: a caller supplies a kind, and the owner's policy supplies
the choice. The contract is the
[area specification](../../docs/specs/harness-selection-and-execution.md).

This release delivers **inspection of a static `routes` policy**. Launching the
chosen harness (`run`), prompts, task context, explicit choices, computed
selection, records and the selection deadline come in later releases. Until
each arrives, its input is refused by name. It is never accepted and ignored.

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

## Which policy runs

The personal default is `~/.config/harness-dispatch/policy.ts`. `--config PATH`
names another entry instead, and a relative path resolves against the current
directory. Nothing else selects a policy. There is no search of the current
directory or its parents, no environment variable and no repository override,
so entering a repository runs none of its code. Naming a file there with
`--config` is your explicit choice, and inspection reports it as explicit
authority. Your personal policy may itself import a repository entry. That
import is also your choice, and the imported code runs with the same trust.

A missing, unreadable or invalid entry refuses and names the path. Nothing is
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
  `{ slot: "prompt" }`.
- `routes` maps each kind to a candidate ID, exactly. There is no catch-all, no
  inheritance and no fallback. A kind the table does not name refuses as an
  incomplete mapping.

`harness-dispatch/sdk` is built into the worker, so there is nothing to install.
For editor type checking, map the specifier to the declarations beside the
worker:

```json
{
  "compilerOptions": {
    "paths": { "harness-dispatch/sdk": ["<prefix>/libexec/harness-dispatch/sdk/index.d.ts"] }
  }
}
```

The readable source, `sdk/index.ts`, sits beside the declarations.

## Inspect

```sh
harness-dispatch inspect --kind impl
harness-dispatch inspect --kind review-impl --config ./policies/review.ts --json
```

Inspection is a proposal. It launches nothing and reserves nothing. It evaluates
trusted TypeScript, which may have side effects of its own. It reports the
policy's path, its authority (personal or explicit), the policy version, the
chosen candidate with its provider, model and effort, the reason, the selection
time, and the worker's identity. `--json` prints the same facts on stdout as
one version-1 object:

```json
{
  "schemaVersion": 1,
  "evidence": "proposal",
  "kind": "impl",
  "policy": { "path": "/home/me/.config/harness-dispatch/policy.ts", "authority": "personal", "version": "2026-09-30" },
  "selection": { "form": "routes", "selectedBy": "route", "candidateId": "deep", "provider": "anthropic",
                 "model": "claude-opus-5-5", "effort": "high", "reason": "routes[\"impl\"] names candidate \"deep\"" },
  "timing": { "selectionMs": 15 },
  "worker": { "path": "…/libexec/harness-dispatch/harness-dispatch-policy", "packageVersion": "…", "buildId": "…", "bunVersion": "1.4.2" },
  "diagnostics": { "stdout": "", "stderr": "" }
}
```

An explicit entry adds `"argument"`, the `--config` value as given.

The worker runs in a private empty directory with null stdin. Its environment
contains only HOME, PATH, TMPDIR, LANG and `LC_*`. It talks to the front over a
private channel on descriptor 3 and inherits no other descriptors. Whatever the
policy prints is captured and kept apart from the report. `--json` carries it in
`diagnostics`, and text mode prints it on stderr, each line prefixed with
`policy stdout:` or `policy stderr:`.

## Refusals

A refusal launches nothing and never substitutes another candidate. Text mode
prints the code, stage, message, relevant input, source and location, and a
remedy on stderr. `--json` prints one object on stderr,
`{"schemaVersion":1,"error":{…},"diagnostics":{…}}`, and nothing on stdout.

| Exit | Stage | Codes |
|---|---|---|
| 2 | `cli` | `malformed_input`, `unsupported_input` (an input or command a later release delivers) |
| 3 | `authority` | `policy_missing`, `policy_unreadable`, `home_unset`, `cwd_unavailable` |
| 3 | `load` | `policy_import_failed` (a missing import, or the entry threw while loading) |
| 3 | `validation` | `policy_invalid` and `unsupported_version`, each with its `location`; `unsupported_form` (`select`, `loadContext`) |
| 3 | `selection` | `incomplete_mapping` |
| 5 | `worker` | `worker_missing`, `worker_identity_mismatch`, `worker_failed`, `protocol_error` |
