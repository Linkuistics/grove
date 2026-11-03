# Grove policy assets for Ravel

These readable caller assets belong to Grove. Grove ships this complete tree at
`libexec/ravel-policy` in its own installation. The same files can be used
directly from this checkout. Ravel embeds no Grove adapter or example.

The tree holds one asset set for each admitted major of Ravel:

| Set | Runtime | Files | Owner configuration |
| --- | --- | --- | --- |
| TypeScript | Ravel 1.x from 1.1.0, request and policy schema 2 | `*.ts` | `~/.config/ravel/policy.ts` |
| Rhai | Ravel 2.x from 2.0.0, `SCHEMA_VERSION` 4 | `*.rhai` | `~/.config/ravel/config.rhai` |

Each file names its runtime and schema. The layout is the same for both sets:

- `sample/policy.ts` and `sample/config.rhai` are one owner's complete
  configuration.
- `examples/grove-static.ts` and `examples/grove-static.rhai` are the starter
  route table for Grove's session kinds.
- `examples/static.rhai` is Grove's copy of Ravel's Rhai starter. Ravel 2.0.0
  embeds no module, so `grove-static.rhai` imports it by its path. Its header
  records its one change from Ravel's copy.
- `grove.ts` and `grove.rhai` are the review adapter, which reads the
  relationship and the creator from a Grove task file.
- `provenance.ts` and `provenance.rhai` are the provider-evidence wrapper and
  the receipt parser.
- `examples/review.ts` and `examples/review.rhai` are the generic
  creator-aware review selector. `examples/grove-review.ts` and
  `examples/grove-review.rhai` compose it with the adapter, the static routes
  and the wrapper.
- `typecheck/` is TypeScript only. It holds strict development fixtures for
  the 1.x set.

Grove's [delivery specification](../../docs/specs/codex-skill-provisioning.md#caller-assets)
owns the contract of both sets. Using these files does not install or overwrite
a personal configuration.

## The TypeScript set, for Ravel 1.x

Ravel 1.x supplies the generic `ravel/sdk` and `ravel/examples/static` modules.
Copy `sample/policy.ts` to `~/.config/ravel/policy.ts` and edit the copy, or
copy `examples/grove-static.ts` beside a personal policy and import it normally:

```typescript
export { policy } from "./grove-static.ts";
```

An absolute file import works too. Paths name readable caller files, not an
embedded `grove/*` module registry. Keep imports of generic modules spelled
`ravel/sdk` and `ravel/examples/static`. The installed worker resolves those.

## The Rhai set, for Ravel 2.x

Copy `sample/config.rhai` to `~/.config/ravel/config.rhai` and edit the copy.
The sample imports nothing, so it works alone.

To use the static example, keep `examples/grove-static.rhai` and
`examples/static.rhai` together, in this tree or as copies. Then import the
example by its path from your own `config.rhai`:

```rhai
import "/absolute/path/to/examples/grove-static" as grove;
export const SCHEMA_VERSION = 4;
export const VERSION = "my-config 1";
fn select(request, context) { grove::select(request, context) }
```

A relative path resolves from the directory of the importing file. To add kinds
of your own, extend `grove::ROUTES` and select through `grove::select_route`.
The header of `grove-static.rhai` gives the form and the reason.

Each Rhai asset passes `request.prompt` to its harness. With skill delivery
enabled, that is the prompt that Ravel prepared. The sample then grants Claude
a read of each `snapshotRoot` in `request.skills`. Codex runs with full access
and needs no grant. With delivery disabled, `request.skills` is `()` and the
sample grants nothing for skills.

## Choices, workspaces and launches

The static examples use illustrative `my-codex-wrapper` and
`my-claude-wrapper` commands. The samples have concrete harness command lines,
four arrangements and two modifiers. Their checkout choice is `.ravel-choice`.
With no file they use `claude-led codex-sol`. A file replaces the default whole
and must name exactly one arrangement plus any desired modifiers. For example:

```text
codex-design-claude-impl high-effort
```

The samples grant the main repository to both harnesses when they run in a
secondary jj workspace. Their `release-notes` route expects Grove's staged
`codex-headless.sh` in the caller's cwd. These static routes compare no creator
origins. For creator-aware routing, import `examples/grove-review.ts` on Ravel
1.x or `examples/grove-review.rhai` on Ravel 2.x instead.

The two samples refuse alike, with three exceptions that the specification
accepts in [Runtime parity](../../docs/specs/codex-skill-provisioning.md#runtime-parity).
Ravel 2.x measures each read. A choice file beyond 4,096 bytes, or a jj store
file beyond Ravel's source bound, therefore ends the evaluation as
`source_too_large`. Rhai and JavaScript split and trim U+FEFF and U+0085
differently. A jj store file that is not UTF-8 refuses on Ravel 2.x.

The sample's interactive Codex command uses `--no-alt-screen`, so session
endings remain in terminal scrollback after Ravel closes the session. Add that
flag to an existing owner configuration's interactive Codex arguments to use the
same display behaviour. Grove's terminal recovery still returns from an
alternate screen when a different harness leaves one active. Launch arguments
belong to the owner configuration.

`ravel start-session --config <file> --kind <kind> --prompt <text>` launches
the selected command. The caller supplies ending instructions and result
meaning. For Grove, that includes invoking the resolved Ravel path with
`end-session grove-continue`. Ravel has no inspection verb. Check choices
with temporary fake agents on a private PATH.

## Checks

Run `task ravel:policy RAVEL=/absolute/path/to/ravel RAVEL_RUNTIME=<1.1|2.0>`
against an explicit installed build. One case table in
`crates/grove/tests/ravel_asset_cases.rs` drives the checks of every asset in
both sets. These are the static example, the sample, the adapter, the review
examples and the wrapper. Each case states the expected outcome for each runtime, and an
accepted difference keeps its reason beside its case. The table compares the
receipt bytes too.

- With `RAVEL_RUNTIME=1.1`, or with it unset, the task typechecks the
  TypeScript set strictly against that installed pair's declarations. It reuses
  the repository's pinned Bun 1.4.3 and TypeScript 7.0.2. It then runs the
  table through the real worker with fake agents.
- With `RAVEL_RUNTIME=2.0`, the task runs the table through public
  `start-session` of the 2.0 build with fake agents. Ravel 2.0 has no
  typechecker.

Runtime tests copy the independent installed build, with no Bun or Node on the
child PATH and no model launches. The complete logs can be saved with
`set -o pipefail; task ravel:policy RAVEL=... 2>&1 | tee <log>`.

## Provider evidence and reviews

For selected-provider evidence on Ravel 1.x, opt in around the final owner
policy with `withProvenance` from `provenance.ts`. This wrapper supplies
`GROVE_PROVIDER_EVIDENCE` to the managed process while preserving the original
argv and labels. The `examples/grove-review.ts` policy already wraps its final selection.
When editing its composition, wrap the final policy only once. The [caller-owned record/context contract](../../docs/grove-provider-evidence.md)
explains retirement, attempts/node closes, missing/invalid evidence, the explicit
original-v22 exception and Bash/argv budgets.
`task ravel:provenance RAVEL=...` exercises production through installed Ravel and
actual Grove lifecycle sessions with fake agents. Personal policy files are never
automatically rewritten.


The review example imports `grove.ts` and `examples/review.ts` as ordinary files.
Copy their directory structure together, or import the example by absolute path:

```typescript
export { policy } from "/absolute/path/to/grove/assets/ravel-policy/examples/grove-review.ts";
```

For your own routes, compose `groveReviewSelector({ routes, reviews })` into a
`definePolicy` and opt into `withProvenance` around the final policy. Each review
entry maps the finishing creator's exact provider label to a reviewer command.
Unknown origins and commands with the same provider refuse, including gateways
that retain that model's origin. Other kinds keep the static routes and efforts.
The example commands/models are placeholders; it makes no owner-policy changes.

`grove.ts` reads only the supplied `--task-file` through Ravel's measured host.
Markers begin in column one, including in fenced examples; indented/inline
mentions do not count. Exactly one `**Reviews:** <handle>` and one `**Creator:**`
line immediately beneath it are required. CRLF is accepted; provider labels are
verbatim. The Creator is `recorded <canonical JSON>` from the finishing session
or `declared <provider>` from the owner. Missing, duplicate, malformed, displaced
and original-v22 `run` records refuse with repair/declaration remedies. A supplied
context may contain unrelated facts but cannot override those review facts.

For callers without Grove tasks, `examples/review.ts` accepts the generic
`facts.reviewedArtifact: { id }` and `facts.creator` shapes documented in the
record/context contract. It validates recorded receipts and matching provider
labels before selecting. Neither adapter reads a history/store or estimates the
creator from current producer routes. Ravel measures task/context bytes; the
owner and finishing session attest their meaning. Generic contexts are JSON data,
so canonical raw inline encoding is checked at the Grove task parser; generic
receipt objects are validated by fields and values independent of key order.
Raw JSON encoding and duplicate keys are no longer available after transport;
use a Grove task receipt when canonical input bytes are required.

### The Rhai review composition, on Ravel 2.x

The Rhai set has the same tables, refusals and receipts. Import
`examples/grove-review.rhai` by its path, and keep the tree together, because
it imports `grove-static.rhai`, `static.rhai`, `review.rhai`, `grove.rhai` and
`provenance.rhai` by their paths:

```rhai
import "/absolute/path/to/examples/grove-review" as grove_review;
export const SCHEMA_VERSION = 4;
export const VERSION = grove_review::VERSION;
fn load_context(request) { grove_review::load_context(request) }
fn select(request, context) { grove_review::select(request, context) }
```

Its `select` already wraps the final selection with `with_provenance`. For
your own tables, pass `#{ routes, reviews }` to `load_context_with` and
`select_with`. Then wrap the final selection once:

```rhai
provenance::with_provenance(selection, global::VERSION, request)
```

A Rhai route is a closure. Rhai finds the function of a closure only while a
function of the module that made it runs. `select_with` of `grove-review.rhai`
therefore runs the selector inside `within` of `grove-static.rhai`. Make your
own routes in a function of your configuration script, as `route(origin)`
below does. A closure that one top-level constant holds and another names does
not load.

```rhai
fn route(origin) { |request| #{ program: "my-agent", args: [request.prompt], provider: origin, model: "m", effort: "high" } }
const RULES = #{ routes: grove_review::ROUTES, reviews: #{ "review-impl": #{ openai: route("anthropic") } } };
```

The Rhai helpers class a blank label by the whitespace set of ECMAScript's
`trim`, so a label checks alike in both runtimes. Rhai lists the origins of a
refusal in code point order, and `review.ts` sorts them the same way. One
difference remains: a receipt that escapes an unpaired surrogate. The
[provider evidence contract](../../docs/grove-provider-evidence.md#runtimes)
gives its outcome on each runtime.

`task ravel:review RAVEL=/absolute/installed/ravel RAVEL_RUNTIME=<1.1|2.0>`
runs the typechecks of the 1.x set and the case table. It also runs actual
Grove retirement, retry and refusal sessions with fake agents. Grove owns these
caller checks; executable/runtime controls run in Ravel. No model is
launched.
