# Grove policy assets for Ravel

These readable TypeScript files belong to Grove. Ravel 0.1.0 supplies the
generic `ravel/sdk` and `ravel/examples/static` modules; it embeds no Grove
adapter or example. Grove's delivery increment will ship these assets in its
own installation. They can already be used directly from this checkout.

Copy `sample/policy.ts` to `~/.config/ravel/policy.ts` and edit the copy, or
copy `examples/grove-static.ts` beside a personal policy and import it normally:

```typescript
export { policy } from "./grove-static.ts";
```

An absolute file import works too. Paths name readable caller files, not an
embedded `grove/*` module registry. Keep imports of generic modules spelled
`ravel/sdk` and `ravel/examples/static`; the installed worker resolves those.

The static example uses illustrative `my-codex-wrapper` and
`my-claude-wrapper` commands. The sample has concrete harness command lines,
four arrangements and two modifiers. Its checkout choice is `.ravel-choice`;
with no file it uses `claude-led codex-sol`. A file replaces the default whole
and must name exactly one arrangement plus any desired modifiers. For example:

```text
codex-design-claude-impl high-effort
```

The sample grants the main repository to both harnesses when running in a
secondary jj workspace. Its `release-notes` route expects Grove's staged
`codex-headless.sh` in the caller's cwd. These static routes compare no creator
origins. For creator-aware routing, import `examples/grove-review.ts` instead.
Using these files does not install or overwrite a personal policy.

`ravel start-session --config <file> --kind <kind> --prompt <text>` launches
the selected command. The caller supplies ending instructions and result
meaning; Ravel passes the prompt unchanged. For Grove, that includes invoking
the resolved Ravel path with `end-session grove-continue`. Ravel has no
inspection verb. Validate choices with temporary fake agents on a private PATH.

Run `task ravel:policy RAVEL=/absolute/path/to/ravel` for strict typechecks
against that installed pair's declarations and real-worker/fake-agent sessions.
Development typechecking reuses the repository's pinned Bun 1.4.2 and TypeScript
7.0.2. Runtime tests copy the independent installed front and worker, with no
Bun/Node on the child PATH and no model launches. The complete logs can be saved
with `set -o pipefail; task ravel:policy RAVEL=... 2>&1 | tee <log>`.

For selected-provider evidence, opt in around the final owner policy with
`withProvenance` from `provenance.ts`. This wrapper supplies
`GROVE_PROVIDER_EVIDENCE` to the managed process while preserving the original
argv and labels. The `examples/grove-review.ts` policy already wraps its final selection.
When editing its composition, wrap the final policy only once. The [caller-owned record/context contract](../../docs/grove-provider-evidence.md)
explains retirement, attempts/node closes, missing/invalid evidence, the explicit
original-v22 exception, Bash/argv budgets and confined executable grants.
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

Run `task ravel:review RAVEL=/absolute/installed/ravel` for public-SDK typechecks,
installed-worker adapter/selection cases, actual Grove retirement/retry/refusal
sessions and the retained pre-k6 compatibility suite. No model is launched.
