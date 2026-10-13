# Policy mechanics

Source: [harness-dispatch usage](https://github.com/Linkuistics/grove/blob/main/crates/harness-dispatch/README.md).
Check `harness-dispatch --help` for the installed version before depending on
a flag.

## What Grove passes

`harness-dispatch` is a separate command installed beside Grove, and Grove
runs it for every session. For a lifecycle session it runs this, in the
working-tree root, with values from the leaf it selected:

```text
harness-dispatch run --kind=KIND --task-file=TASK_FILE --task-id=HANDLE --prompt=MANDATE --exit-dir=LAUNCH_DIR --ending-file=ENDING_FILE
```

The policy's `select` receives the kind, the task file, the handle and the
prompt, and reads the session's location as `request.cwd`. The exit directory
and ending file are run mechanics and reach no policy. Grove passes no
parameter, so `request.params` is empty, and no policy entry, bound, grant or
record directory. A policy that names a session or grants a secondary jj
workspace's harness its main repository derives either from `request.cwd`: the
sample reads the main repository from the `.jj/repo` file there, and names no
session. `grove run KIND`
selects through the same policy, with `run --confine`, outside its sandbox. It has no
task, so it passes no task file and no task identity, and its harness must be
a noninteractive command.

## Installing a policy

The personal policy is `~/.config/harness-dispatch/policy.ts`.
harness-dispatch never discovers one in a repository. Naming a repository file
with `--config`, or importing one from the personal policy, runs that
repository's code with the user's authority, so do either only with the user's
explicit authorization.

```sh
harness-dispatch init
```

`init` installs the sample policy when nothing is at that path, and has no
option to replace one. Installing or upgrading Grove writes no policy. The
sample launches `codex` with approvals off and full access. Say so before the
user's first launch, and remove those arguments if they do not want them.

The sample is a `select` over tables in the file:

- `ROUTES` maps each of Grove's session kinds, and the standalone
  `release-notes` kind, to a role and usually an effort. A kind it does not
  list refuses.
- `ARRANGEMENTS` says which harness takes which role. There are four:
  `codex-led`, `claude-led`, `codex-design-claude-impl` and
  `claude-design-codex-impl`.
- `MODIFIERS` change model and effort across an arrangement: `codex-sol` and
  `high-effort`.
- `DEFAULT` is what applies with no choice file.

Change a kind's model or effort in its route. Add a kind by adding its route.
A program that is a wrapper must `exec` its harness. The shipped examples,
`harness-dispatch/examples/grove-static` and `grove-review`, are alternatives
a policy imports. Prefer an edited copy to re-exporting a specifier, because an
imported example changes when the installation upgrades.

## The choice file

`.harness-dispatch-choice` in the working-tree root holds names separated by
whitespace. For the sample, one arrangement and any modifiers:

```sh
echo 'codex-led high-effort' > .harness-dispatch-choice
```

It replaces the sample's default whole. It can name only what the policy
offers: an unoffered name, no arrangement, or two refuse the launch, naming the
names on offer. The file introduces no program, argument or label, so it needs
no ignore rule and no trackedness check. Whether to commit it is the user's
choice, and a repository may ship one. It is read in the directory the launch
runs in: the working-tree root for a lifecycle session, with no search of
parent directories. `grove run` selects in its staged directory, so no
checkout's choice file applies to a standalone kind. It affects only an owner whose policy reads it, and harness-dispatch
itself never looks for it. A policy the user wrote reads one only if its
`select` calls the SDK's `readChoice`.

## Owner settings

`~/.config/harness-dispatch/settings.json` is optional, and holds at most four
keys: `timeoutMs` (1000 to 600000), `contextBytes`, `stateDir` (an absolute
path) and `policyEnv` (an array of names). They are the same for every kind. A
flag replaces its setting for one invocation, `--policy-env` adds to the
grants, and Grove passes no settings override. Raise
`timeoutMs` only when a policy legitimately takes longer than 30 seconds, since
the bound is also how long a stuck policy holds a launch.

**Never grant `GROVE_LAUNCH_DIR`**, in `policyEnv` or with `--policy-env`.
That directory carries the session's authority: a policy holding its path
could record a teardown or end the run it is selecting for. Dispatch keeps it
out of the worker unless the owner grants it, and does not refuse such a grant.
Grant only names the policy itself reads.

## Remedy a refused launch

A refused launch launches nothing, and nothing is substituted. Grove reports
that the session ended without an exit signal, with harness-dispatch's
exit status, and stops the loop. Grove's own exit status is 0, so read the
refusal from its report. The leaf stays live. The refusal's `inspect:` line
reproduces the selection without the prompt:

```text
inspect: (cd /home/you/app && /opt/grove/bin/harness-dispatch inspect --kind spike --task-file /home/you/app/.grove/01-spike--api-k1.md --task-id api-k1)
```

1. Run that line. It refuses the same way and launches nothing. It names the
   `harness-dispatch` Grove ran by its full path, so keep that path.
2. Do what the remedy names. `policy_missing` is remedied by the user
   installing or writing a policy. A policy's own refusal is `policy_refused`
   with a `policy code`: for the sample, `incomplete_mapping` means the kind
   has no route, and `choice_unoffered` or `choice_arrangement` that the choice
   file is wrong. A missing route is the user's decision. Do not fill it with a
   catch-all.
3. Run the line again until it reports a command.
4. Run `grove` again. The same leaf launches.

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
