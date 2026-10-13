# Policy mechanics

Sources: [Ravel 0.1.0 session CLI](https://github.com/Linkuistics/Ravel/blob/v0.1.0/docs/session-cli.md)
and [Grove's readable policy assets](../../../../../assets/ravel-policy/README.md).
Check `ravel --version` and `ravel start-session --help` before relying on flags.

## What Grove passes

Grove resolves the first executable regular `ravel` in an absolute PATH entry,
canonicalizes and version-checks it once, then uses that exact path for launch,
confinement and prompt ending instructions. It need not share Grove's prefix or
version. Missing Ravel names `brew install linkuistics/taps/ravel`; unsupported
versions refuse with an upgrade remedy. Grove currently accepts stable 0.1.x.

For lifecycle work, in the working-tree root, the public request is:

```text
ravel start-session --kind=KIND --task-file=TASK_FILE --task-id=HANDLE --prompt=MANDATE --exit-dir=LAUNCH_DIR
```

`select` receives kind, task file, task identity, unchanged prompt and
`request.cwd`. Grove passes no parameter, settings override or record path.
The exit directory is supervision machinery and reaches no policy.
`grove run KIND` uses the same public verb with `--confine` in its staged cwd;
it has no task file/identity. Its selected command must be noninteractive.
Grove grants documented runtime reads, not a private worker protocol.

The caller prompt supplies canonical `ravel end-session grove-continue` and
`ravel end-session grove-force-stop` commands. Neither takes a session ID.
Grove interprets continue as completion and force-stop as stopping without
completion; standalone force-stop publishes no outputs. Ravel returns the opaque result on stdout
only after cleanup; session output and diagnostics remain separate. Grove
interprets that result after its own authority checks. A finish first records
Grove's teardown. Original installed-v22 sessions keep their original ending
command/channel; installing new assets never changes a running mandate.

## Installing and selecting a policy

The personal entry is `~/.config/ravel/policy.ts`. Ravel discovers no repository
policy. Explicit `--config` or personal imports execute that code with the
owner's authority. Authorize the imported code before activating it.

Ravel has no `init` or `inspect`. When asked to install, copy Grove's readable
`assets/ravel-policy/sample/policy.ts` to an absent personal entry and edit the
copy. Preserve an existing policy. Installation/upgrading Grove writes none.
The sample disables approvals and enables full access for its concrete harness
commands; configure those arguments according to the owner's request.

The sample's `ROUTES` map kinds to roles/efforts. `ARRANGEMENTS` offer
`codex-led`, `claude-led`, `codex-design-claude-impl` and
`claude-design-codex-impl`; `MODIFIERS` offer `codex-sol` and `high-effort`.
`DEFAULT` applies without a choice file. A missing route refuses. A wrapper
must exec its harness. The readable `examples/grove-static.ts` and
`examples/grove-review.ts` import caller files normally, together with generic
`ravel/sdk` and `ravel/examples/static`; Ravel embeds no Grove module. Copy the
required directory structure or import an explicitly authorized absolute file.

`.ravel-choice` in the launch cwd names offered options separated by whitespace:

```sh
printf '%s\n' 'codex-led high-effort' > .ravel-choice
```

It replaces the sample default whole and must contain exactly one arrangement.
It introduces no program or argument. The generic `readChoice` helper reads it
only when the policy opts in. There is no parent search; standalone work selects
in its staged cwd, so it sees no checkout choice file.

## Owner settings

`~/.config/ravel/settings.json` is optional. Its keys are `timeoutMs`
(1000–600000), `contextBytes` and `policyEnv` (environment names). Flags override
bounds and `--policy-env` adds grants; Grove supplies none. `stateDir` is an
obsolete unknown key, not a Ravel store location. Raise the 30-second default
only when selection needs it.

Never grant `GROVE_LAUNCH_DIR`: it carries session authority. Policy evaluation
starts with a minimal environment and does not inherit Grove/Ravel/legacy
controls. Grant only names the selection code actually reads. Trusted policy
code is not a sandbox against a hostile same-user script.

## Verify choices without a model launch

Read selection code and its actual wrapper/profile settings first. There is no
selection-only command. `start-session` executes its selected command.

For shipped assets, run `task ravel:review RAVEL=/absolute/installed/ravel`.
It typechecks public SDK imports and runs temporary policies/fake agents through
a copied independent front/worker on a child PATH without Bun/Node or Grove.
No personal policy is overwritten and no model runs.

For an owner policy, build an isolated temporary HOME, copy the complete policy
and authorized imports/settings/choice, and put executable fake agents matching
its harness names on a private PATH. Make them record argv and end through the
absolute installed Ravel with a fixture-only result. Cover every changed kind,
choice and provider pair. A policy using absolute commands or wrappers requires
an explicit fixture adaptation; PATH alone cannot replace those programs.
Do not pass production prompts or advertise this as evaluating the original
policy unchanged. Verify the requested prompt/context too if selection reads it.
Policy evaluation itself may execute code or start tools, so inspect it before
running these fixtures. Typechecking alone establishes no routing outcome.

## Remedy a refused launch

Ravel reports routing refusals on stderr, exits 3 and launches no selected
process. Grove reports no supported result and stops, leaving the leaf live.
Read the actual policy code/remedy; `policy_missing` requires an owner policy,
`incomplete_mapping` an explicit kind route, and invalid choice codes a choice
among offered options. Do not add a catch-all to hide a missing owner decision.
Correct the source, verify with safe fixtures, then rerun `grove` to retry the
same leaf. No `inspect:` reproduction is available. A public start-session
reproduction can launch after repair and needs its own ending instructions.

## Reviews and the creator line

The static sample sends reviews to the other provider by arrangement and consults
no creator. Grove's readable `assets/ravel-policy/examples/grove-review.ts`
enforces creator-aware pairing. Import that file normally or copy its directory
structure beside the owner policy; there is no embedded Grove module in Ravel.
For the five `review-*` kinds, it measures the supplied task file, parses exactly
one `**Reviews:**` line and its adjacent `**Creator:**`, and selects a command
whose provider differs from the finishing creator's exact origin label. Unknown
origins and same-provider gateways refuse; no substitute reviewer launches.

The Creator line has two current forms and two writers:

- `**Creator:** recorded <canonical JSON>` is copied byte for byte from the
  finishing session's `GROVE_PROVIDER_EVIDENCE`. The opt-in Grove
  `withProvenance` wrapper records the actual selected provider and a fresh
  attempt identity. A retry that finishes replaces previous evidence; a node
  close attributes the closing child. No receipt means the session removes an
  earlier Creator; invalid evidence requires repair before commit.
- `**Creator:** declared <provider>` is the owner's assertion for an artifact
  without recorded evidence. Do not write one on their behalf.

[The record/context contract](../../../../../docs/grove-provider-evidence.md)
owns exact grammar, budgets and lifetime. The line must be directly beneath
Reviews. Column-one markers count even in fenced examples; indented/inline
mentions do not. Missing, malformed, duplicate or displaced markers refuse
with a specific policy code and repair remedy. A missing creator needs the
owner's declaration; never infer it from today's producer route or an earlier
same-task attempt. A caller context cannot override task review facts.

Receipts and declarations are same-user attestations, not authenticated backend
identity. Measured reads pin task bytes; commits preserve those bytes for review
retries. Receipt task/kind need not equal the reviewed handle: a child can close
a whole node. Confirm the association against the actual finishing work when
changing a policy. Ravel supplies no run lookup, record verb or persistent store.
Review findings belong to Grove task/document artifacts and their VCS history.

Only sessions still launched by the original installed-v22 driver use
`**Creator:** run <HARNESS_DISPATCH_RUN_ID>` under that driver's legacy
policy/store and ending contract. New consumers refuse that form with a remedy
to declare the provider or finish again under a receipt-producing policy. Never
copy legacy IDs or receipts from other reviews, version history or a store.
