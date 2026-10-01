# Routing Grove sessions through harness-dispatch

Sources: [Grove configuration reference](https://github.com/Linkuistics/grove/blob/main/docs/CONFIGURATION.md#harness-dispatch)
and [harness-dispatch usage](https://github.com/Linkuistics/grove/blob/main/crates/harness-dispatch/README.md).
Check `harness-dispatch --help` for the installed version before depending on
a flag.

## Who owns what

`harness-dispatch` is a separate command installed with Grove. A lifecycle
route delegates to it when its command definition runs `harness-dispatch run`
with Grove's task slots and the prompt. Two personal sources then own the two
halves of a launch, and neither reads the other:

- **Grove configuration owns the wrapper.** `~/.config/grove/config.kdl`, with
  any admitted `.grove.kdl`, decides which kinds launch and which
  `harness-dispatch run` command each one runs. `grove config show` inspects
  it.
- **Dispatch policy owns selection.** `~/.config/harness-dispatch/policy.ts`
  is TypeScript. Its catalog holds joint harness, model and effort candidates,
  each labelled with a provider, and its routes or `select` give each kind one
  of them. `harness-dispatch inspect` inspects it.

For a dispatched kind, Grove's resolved argv carries no model or effort.
Establish them with `harness-dispatch inspect`, never from Grove's report.

The policy has no project scope. harness-dispatch evaluates the personal
policy, or the entry that a `--config` argument in the command names, and never
discovers one in a repository. A `.grove.kdl` can re-point a route at a
personal dispatch binding, but it cannot declare the command. Naming a
repository file with `--config` runs that repository's code with the user's
authority, so add it only with the user's explicit authorization.

## Activate

Compose and publish each file as SKILL.md's *Verify and report* directs.

1. In personal Grove configuration, define the command exactly as below, bind
   it, and route each dispatched kind to the binding:

   ```kdl
   config {
       command "dispatch" "harness-dispatch run --kind ${kind} --task-file ${task_file} --task-id ${task_id} --prompt ${prompt}"
       bind "dispatched" "dispatch"
       route "impl" "dispatched"
   }
   ```

   Grove fills the slots from the leaf it selected. Each dispatched kind still
   needs its Grove route. Direct-harness routes for other kinds stay valid in
   the same file. Do not route a standalone `grove run` kind to this command,
   because standalone invocations refuse the lifecycle-only slots.

2. Create the personal policy if it is absent, and never overwrite an existing
   one. The Grove starter, `harness-dispatch/examples/grove-static`, routes
   every kind Grove ships over placeholder wrappers and models. Prefer a copy
   of `libexec/harness-dispatch/examples/grove-static.ts` from the installation
   prefix, edited to the user's programs, models, providers and efforts, over
   re-exporting the specifier. An imported example changes when the
   installation upgrades, and a copy does not. Installing or upgrading writes
   neither personal file. A candidate's program that is a wrapper must `exec`
   its harness.

   When the user wants every review on another provider than the one that
   created the reviewed artifact, use `harness-dispatch/examples/grove-review`
   instead, or a copy of it. It routes other kinds as the Grove starter does.
   For the five `review-*` kinds it reads the review leaf's own `**Reviews:**`
   and `**Creator:**` lines through its Grove adapter, `harness-dispatch/grove`,
   and chooses a reviewer from the other provider. Route those review kinds to
   the dispatch binding too. A review needs `**Creator:** run <run-id>`, naming
   the dispatch run that finished its producer, or `**Creator:** declared
   <provider>`, the user's declaration for a producer finished without
   dispatch. Sessions settle the run form themselves, as *The creator line*
   below says. Do not write a declaration on the user's behalf. The provider is
   their assertion about who made the artifact.

3. For a kind that must run one particular candidate, add a separate command
   definition with a literal `--choice ID` before `--prompt ${prompt}`, and
   route that kind to it. The ID names a whole catalog candidate. A routes
   policy takes it, a `select` policy accepts or refuses it, and an ID the
   catalog lacks refuses. There is no Grove flag, environment override or
   task-file field for a choice.

**Never add `--policy-env GROVE_SIGNAL_FILE` to the command.** That variable is
the authority to end the Grove session, and a policy holding it could end the
session it is selecting for. harness-dispatch cannot tell completion variables
from other names, so it does not refuse the grant. Grant only names the policy
itself reads.

## Verify

Grove checks a kind before a leaf of that kind is written, and the check stops
at the configured command. A passing `grove config show` or a successful
`grove-llm leaf-add` proves nothing about the policy. Static routes and a
computed `select` alike are evaluated only when a leaf launches. So verify each
half on its own surface:

- Run `grove config show --json` in the intended workspace. Each dispatched
  kind should resolve to `harness-dispatch` with the `kind`, `task_file`,
  `task_id` and `prompt` slots.
- Run `harness-dispatch inspect --kind KIND --json` for every dispatched kind.
  It should report a candidate, its provider, model and effort, the resolved
  executable and the argv. Add `--task-file` and `--task-id` when the policy
  reads the task. Inspection evaluates trusted TypeScript, which may have side
  effects, and it is a proposal, because every launch evaluates afresh.

Check producer and reviewer providers across the policy's choices as for
static Grove policy. A routes table proves only the planned pairing. Report
both surfaces. A kind whose inspection refuses is unconfigured, whatever
Grove's check said.

## Remedy an incomplete mapping

When the policy has no candidate for a launched kind, harness-dispatch refuses
with `incomplete_mapping`. A `select` policy refuses as `policy_refused`, with
its own code. Nothing launches, and no default candidate is substituted. Grove
reports that the session ended without a completion signal, with exit status 3,
and stops the loop. The leaf stays live. The refusal's last line, `inspect:`,
reproduces the selection. The remedy:

1. Run that `inspect:` line. It refuses the same way and launches nothing.
2. Add the kind to the policy's `routes`, mapped to a catalog ID the user
   chooses, or route the Grove kind to a command definition with a literal
   `--choice ID`. A missing route is the user's decision. Do not fill it with
   a catch-all or a fallback candidate.
3. Run the `inspect:` line again until it reports a candidate.
4. Run `grove` again. The same leaf launches.

## Remedy a review that cannot name its creator

Under the Grove review example, a review task file that cannot name its
producer's creator refuses as `policy_refused` at stage `context`, before any
reviewer is chosen. Its `policyCode` says which line is wrong:

- `creator_line_missing`: the review has `**Reviews:**` and no
  `**Creator:**`. The producer was finished with no run to name: before
  dispatch was adopted, or by a harness Grove launched directly. A recorded run
  of the producer's task does not stand in, because harness-dispatch never
  looks a run up by task. Ask the user which provider finished the producer,
  and have them write `**Creator:** declared <provider>` directly under
  `**Reviews:**`. Only when a dispatched session finished it and left no line
  is the remedy `**Creator:** run <run-id>`, with that session's own
  `HARNESS_DISPATCH_RUN_ID`, never an earlier attempt's.
- `reviews_line_missing`, `reviews_line_duplicate`, `reviews_line_malformed`,
  `creator_line_duplicate`, `creator_line_malformed`: the review needs exactly
  one line of each, each beginning its own line, with one space after the
  marker and nothing after the value. A line quoted in a fenced block counts.
- `review_kind_unlisted`: a kind the policy does not list as a review has a
  `**Reviews:**` line. List the kind among the policy's reviews, or remove the
  line if the task is not a review.
- `creator_run_missing`, `creator_origin_unknown`, `same_origin`: the named run
  is not in the record store, the creator's provider is not one of the
  catalog's, or the reviewer shares it. Follow the refusal's remedy.

Run the refusal's `inspect:` line after each correction until it reports a
reviewer, then run `grove` again. Never replace a refused reviewer with one of
the creator's provider.

## The creator line

A review's `**Creator:**` line sits directly under `**Reviews:**` and has two
forms, with two writers:

- `**Creator:** run <run-id>` comes from the session that finished the
  producer, by retiring its leaf or closing its node. The `grove` skill's
  `references/retire.md` owns that step. Under dispatch the session writes its
  own `HARNESS_DISPATCH_RUN_ID`, replacing any line there. With no run it
  removes the line, in either form, so an earlier attempt's run does not stand
  for the artifact.
- `**Creator:** declared <provider>` is the user's alone, for a producer
  finished with no run. It is written once the producer is finished, because
  a session that finishes it later removes it.

**A run line is its writer's word, and launch does not check it.**
harness-dispatch records the provider each run launched. It cannot tell which
run made an artifact, so a line naming some other existing run lends that
run's provider. Launch does not compare the run's task with the `**Reviews:**`
handle either, because a decomposed producer is finished by a child task with
its own handle. So never copy a run ID into a review from another review,
from version history or from `record show`. When verifying a review route,
inspect the review itself:

```sh
harness-dispatch inspect --kind review-impl --task-file FILE --task-id HANDLE
```

Its `reviewed` line gives the reviewed handle. Its `creator` line gives the
run, the provider recorded for it, and the task and kind it was launched for.
Report a run whose task is neither the reviewed producer nor work under it.
The producer and its review must use the same record store.

**A review can attach its findings to that run.** Its own task file names it:

```sh
harness-dispatch record observe --run <run-id> --file observation.json
```

The record store is outside the tree, so the observation remains after
`.grove/` is removed. The observation document's shape is in
[harness-dispatch usage](https://github.com/Linkuistics/grove/blob/main/crates/harness-dispatch/README.md#observations).
