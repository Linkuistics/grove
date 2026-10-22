# Policy mechanics

Sources: [Ravel 1.0.0 session CLI](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md)
and [readable policy assets of Grove](../../../../../assets/ravel-policy/README.md).
Run `ravel --version` and `ravel start-session --help` before you rely on flags.

## What Grove passes

Grove resolves the first executable regular `ravel` in an absolute PATH entry. It
canonicalises the path and verifies the version once. Then it uses that exact path for
launch, confinement and the ending instructions of the prompt. Ravel need not share the
prefix or the version of Grove.

A missing Ravel names `brew install linkuistics/taps/ravel`. An unsupported version
refuses with an upgrade remedy. Grove currently accepts stable Ravel 1.x from 1.1.0.

For lifecycle work, in the working-tree root, the public request is:

```text
ravel start-session --kind=KIND --task-file=TASK_FILE --task-id=HANDLE --prompt=MANDATE --exit-dir=LAUNCH_DIR
```

`select` receives the kind, the task file, the task identity, the unchanged prompt and
`request.cwd`. Grove passes no parameter, no settings override and no record path. The
exit directory is supervision machinery, and it reaches no policy.

`grove run KIND` uses the same public verb with `--confine` in its staged cwd. It has no
task file and no task identity. Its selected command must be noninteractive. Grove
grants documented runtime reads. It grants no private worker protocol.

The caller prompt supplies canonical `ravel end-session grove-continue` and
`ravel end-session grove-force-stop` commands. Neither takes a session ID. Grove
interprets continue as completion. Grove interprets force-stop as stopping without
completion. A standalone force-stop publishes no outputs.

Ravel returns the opaque result on stdout only after cleanup. Session output and
diagnostics remain separate. Grove interprets that result after its own authority
verification. A finish first records the teardown of Grove. Original installed-v22 sessions
keep their original ending command and channel. Installing new assets never changes a
running mandate.

## Installing and selecting a policy

The personal entry is `~/.config/ravel/policy.ts`. Ravel discovers no repository
policy. Explicit `--config` or personal imports execute that code with the authority of
the owner. Authorise the imported code before you activate it.

Ravel has no `init` or `inspect`. When asked to install, copy the readable
`assets/ravel-policy/sample/policy.ts` of Grove to an absent personal entry. Then edit
the copy. Preserve an existing policy. Installing or upgrading Grove writes none.

The sample disables approvals and enables full access for its concrete harness
commands. Configure those arguments according to the request of the owner.

The `ROUTES` map of the sample assigns roles and efforts to kinds. `ARRANGEMENTS` offer
`codex-led`, `claude-led`, `codex-design-claude-impl` and `claude-design-codex-impl`.
`MODIFIERS` offer `codex-sol` and `high-effort`. `DEFAULT` applies without a choice file.
A missing route refuses. A wrapper must exec its harness.

The readable `examples/grove-static.ts` and `examples/grove-review.ts` import caller
files normally. They also import the generic `ravel/sdk` and `ravel/examples/static`.
Ravel embeds no Grove module. Copy the required directory structure, or import an
explicitly authorised absolute file.

`.ravel-choice` in the launch cwd names offered options. Whitespace separates the names:

```sh
printf '%s\n' 'codex-led high-effort' > .ravel-choice
```

It replaces the sample default whole and must contain exactly one arrangement. It
introduces no program or argument. The generic `readChoice` helper reads it only when
the policy opts in. There is no parent search. Standalone work selects in its staged
cwd, so it sees no checkout choice file.

## Owner settings

`~/.config/ravel/settings.json` is optional. Its keys are `timeoutMs` (1000–600000),
`contextBytes` and `policyEnv` (environment names). Flags override bounds, and
`--policy-env` adds grants. Grove supplies none.

`stateDir` is an obsolete unknown key, not a Ravel store location. Raise the 30-second
default only when selection needs it.

Never grant `GROVE_LAUNCH_DIR`, because it carries session authority. Policy evaluation
runs with a minimal environment. It does not inherit the Grove, Ravel or legacy
controls. Grant only names that the selection code actually reads. Trusted policy code
is not a sandbox against a hostile same-user script.

## Verify choices without a model launch

Read the selection code and its actual wrapper and profile settings first. There is no
selection-only command. `start-session` executes its selected command.

For shipped assets, run `task ravel:review RAVEL=/absolute/installed/ravel`. It
typechecks the public SDK imports. It runs temporary policies and fake agents through a
copied independent front and worker on a child PATH without Bun, Node or Grove. It
overwrites no personal policy, and it runs no model.

For an owner policy, build an isolated temporary HOME. Copy the complete policy and the
authorised imports, settings and choice file. Put executable fake agents on a private
PATH. Name them as the harnesses of the policy. Make them record argv and end through
the absolute installed Ravel with a fixture-only result. Cover every changed kind,
choice and provider pair.

A policy that uses absolute commands or wrappers requires an explicit fixture
adaptation. PATH alone cannot replace those programs. Do not pass production prompts.
Do not advertise this as an evaluation of the original policy unchanged.

If selection reads the prompt or context, verify the requested prompt and context too.
Policy evaluation itself may execute code or run tools, so inspect it before you run
these fixtures. Typechecking alone establishes no routing outcome.

## Remedy a refused launch

Ravel reports routing refusals on stderr, exits 3 and launches no selected process.
Grove reports no supported result and stops. The leaf stays live.

Read the actual policy code and remedy. `policy_missing` requires an owner policy.
`incomplete_mapping` requires an explicit kind route. Invalid choice codes require a
choice among offered options. Do not add a catch-all to hide a missing owner decision.

Correct the source. Verify with safe fixtures. Then rerun `grove` to retry the same
leaf. No `inspect:` reproduction is available. A public `start-session` reproduction can
launch after the correction, and it needs its own ending instructions.

## Reviews and the creator line

The static sample sends reviews to the other provider by arrangement and consults no
creator. The readable `assets/ravel-policy/examples/grove-review.ts` of Grove enforces
creator-aware pairing. Import that file normally, or copy its directory structure beside
the owner policy. Ravel has no embedded Grove module.

For the five `review-*` kinds, it measures the supplied task file. It parses exactly one
`**Reviews:**` line and its adjacent `**Creator:**`. It selects a command whose provider
differs from the exact origin label of the finishing creator. Unknown origins and
same-provider gateways refuse. No substitute reviewer launches.

The Creator line has two current forms and two writers:

- `**Creator:** recorded <canonical JSON>` is the `GROVE_PROVIDER_EVIDENCE` of the
  finishing session, copied byte for byte. The opt-in `withProvenance` wrapper of Grove
  records the actual selected provider and a fresh attempt identity. A retry that
  finishes replaces the previous evidence. A node close attributes the closing child. No
  receipt means that the session removes an earlier Creator. Invalid evidence requires
  a correction before commit.
- `**Creator:** declared <provider>` is the assertion of the owner for an artifact
  without recorded evidence. Do not write one on behalf of the owner.

[The record and context contract](../../../../../docs/grove-provider-evidence.md) owns
the exact grammar, the budgets and the lifetime. The line must be directly beneath
Reviews. Column-one markers count even in fenced examples. Indented and inline mentions
do not count.

Missing, malformed, duplicate or displaced markers refuse with a specific policy code
and a remedy for the correction. A missing creator needs the declaration of the owner. Never infer it
from the producer route of today or from an earlier same-task attempt. A caller context
cannot override task review facts.

Receipts and declarations are same-user attestations, not authenticated backend
identity. Measured reads pin task bytes. Commits preserve those bytes for review
retries. Receipt task and kind need not equal the reviewed handle, because a child can
close a whole node.

Verify the association against the actual finishing work when you change a policy.
Ravel supplies no run lookup, no record verb and no persistent store. Review findings
belong to Grove task and document artifacts and their VCS history.

Only sessions that the original installed-v22 driver still launches use
`**Creator:** run <HARNESS_DISPATCH_RUN_ID>` under the legacy policy, store and ending
contract of that driver. New consumers refuse that form with a remedy to declare the
provider or finish again under a receipt-producing policy. Never copy legacy IDs or
receipts from other reviews, version history or a store.
