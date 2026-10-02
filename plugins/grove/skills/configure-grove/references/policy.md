# Policy mechanics

Source: [harness-dispatch usage](https://github.com/Linkuistics/grove/blob/main/crates/harness-dispatch/README.md).
Check `harness-dispatch --help` for the installed version before depending on
a flag.

## What Grove passes

`harness-dispatch` is a separate command installed beside Grove, and Grove
runs it for every session. For a lifecycle session it runs this, in the
working-tree root, with values from the leaf it selected:

```text
harness-dispatch run --kind=KIND --task-file=TASK_FILE --task-id=HANDLE --prompt=MANDATE --param=session_name=NAME --param=worktree=WORKTREE --param=repo=REPO
```

The policy's `select` receives the kind, the task file, the handle and the
prompt, and the session name and the two roots as `request.params`. Grove
passes no policy entry, bound, grant or record directory. `grove run KIND`
selects through the same policy, with `inspect`, outside its sandbox. It has no
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
grants, and Grove passes no flag. Raise
`timeoutMs` only when a policy legitimately takes longer than 30 seconds, since
the bound is also how long a stuck policy holds a launch.

**Never grant `GROVE_SIGNAL_FILE`**, in `policyEnv` or with `--policy-env`.
That variable is the authority to end the Grove session, and a policy holding
it could end the session it is selecting for. harness-dispatch cannot tell
completion variables from other names, so it does not refuse the grant. Grant
only names the policy itself reads.

## Remedy a refused launch

A refused launch launches nothing, and nothing is substituted. Grove reports
that the session ended without a completion signal, with harness-dispatch's
exit status, and stops the loop. Grove's own exit status is 0, so read the
refusal from its report. The leaf stays live. The refusal's `inspect:` line
reproduces the selection without the prompt:

```text
inspect: (cd /home/you/app && /opt/grove/bin/harness-dispatch inspect --kind spike --param 'session_name=app: app grove' --param worktree=/home/you/app --param repo=/home/you/app --task-file /home/you/app/.grove/01-spike--api-k1.md --task-id api-k1)
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

The sample sends each review to the other provider by its arrangement and
consults no creator. A policy that imports
`harness-dispatch/examples/grove-review` enforces the pairing at launch
instead: for the five `review-*` kinds it reads the review leaf's
`**Reviews:**` and `**Creator:**` lines and selects a reviewer from another
provider than the creator's. It checks one provider, the original creator's.
A producer several providers worked on is still yours to check.

The `**Creator:**` line sits directly under `**Reviews:**` and has two forms,
with two writers:

- `**Creator:** run <run-id>` comes from the session that finished the
  producer, by retiring its leaf or closing its node. The `grove` skill's
  `references/retire.md` owns that step. The session writes its own
  `HARNESS_DISPATCH_RUN_ID`, replacing any line there. With no run it removes
  the line, in either form.
- `**Creator:** declared <provider>` is the user's alone, for a producer
  finished with no run. Do not write one on the user's behalf: the provider is
  their assertion about who made the artifact.

Under the review example, a review that cannot name its creator refuses as
`policy_refused` at stage `context`, and its policy code says which line is
wrong: `creator_line_missing`, `reviews_line_missing`, a `_duplicate` or
`_malformed` form of either, or `review_kind_unlisted`. For a missing creator,
ask the user which provider finished the producer and have them write the
declaration. Never replace a refused reviewer with one of the creator's
provider.

**A run line is its writer's word, and launch does not check it.**
harness-dispatch records the provider each run launched. It cannot tell which
run made an artifact, so a line naming some other existing run lends that run's
provider. Nor is the run's task compared with the `**Reviews:**` handle,
because a decomposed producer is finished by a child task with its own handle.
So never copy a run ID into a review from another review, from version history
or from `record show`. Inspecting the review with its task file and handle, as
a refusal's `inspect:` line does, shows the reviewed handle beside the run, its
recorded provider and the task it was launched for. Report a run whose task is
neither the reviewed producer nor work under it.

A review can attach its findings to the run its line names:

```sh
harness-dispatch record observe --run "$RUN_ID" --file observation.json
```

The record store is outside the tree, so the observation remains after
`.grove/` is removed. The observation document's shape is in
[harness-dispatch usage](https://github.com/Linkuistics/grove/blob/main/crates/harness-dispatch/README.md#observations).
