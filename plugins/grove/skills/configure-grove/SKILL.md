---
name: configure-grove
description: Launch-policy configuration for Grove — the owner's harness-dispatch policy, its settings file and a checkout's choice file. Use when installing, inspecting, recommending or editing which harness, model and effort Grove launches for a task kind, choosing an arrangement for one checkout, or remedying a refused launch.
harnesses: [any]
---

# Configure Grove

Configure the user's launch policy from the work each task must perform. This
is an operator skill, not a task kind: it needs no selected leaf and does not
start, retire or complete a Grove session.

## What decides a launch

Grove has no launch configuration. It runs `harness-dispatch` for every
session, lifecycle and `grove run KIND` alike, and three files decide
the rest. [policy.md](references/policy.md) has the mechanics of each.

| File | Decides | Scope |
|---|---|---|
| `~/.config/harness-dispatch/policy.ts` | The command for each kind: program, arguments, and its provider, model and effort labels | Every checkout |
| `.harness-dispatch-choice` in a working-tree root | Which of the options the policy offers that checkout uses | One checkout's lifecycle sessions, and only under a policy that reads it. Never `grove run` |
| `~/.config/harness-dispatch/settings.json` | The selection time bound, the context budget, the record directory and the environment granted to the policy | Every launch of every kind |

A `config.kdl` or `.grove.kdl` from an earlier release is never read. Do not
edit one, and do not expect a converter.

## Establish the effective policy

Check `harness-dispatch --help` for the installed version before depending on
a flag. Then read the policy file itself. It is TypeScript the user owns, and
its tables are the source of what each kind runs.

`harness-dispatch inspect` reports what a kind would launch and launches
nothing. Inspect each kind under consideration from the root of the working
tree in question, as Grove runs it: there its choice file applies, and Grove
passes no parameter.

```sh
cd /work/parser && harness-dispatch inspect --kind impl
```

The report gives the provider, model and effort labels, the `reason`, the
resolved executable, the exact argv and where each bound came from. With no
policy it refuses as `policy_missing`. Installing one is the user's decision:
`harness-dispatch init` writes the sample, and nothing else may write a policy
where none exists. Never replace an existing policy with the sample or with a
file of your own: edit the one the user has.

If the policy's command is a wrapper, or a harness profile supplies the model
or effort, inspect that source too. The labels are the owner's assertion about
what the command runs, and the argv proves the invocation, not settings hidden
behind it.

Read the installed `grove-<kind>` skills for the kinds being considered,
including their named family files. For a project-specific recommendation,
also read the task description, relevant brief chain, acceptance criteria and
project instructions. A task name alone is a weak indication of difficulty.

## Choose model and effort together

For model-selection work, use [model-selection.md](references/model-selection.md).
Infer the user's quality, time and usage priorities from the request and existing
policy; ask only where the unresolved trade-off would change the recommendation.
Higher abstraction and/or higher consequences of error justify more effort.
Account for downstream rework and verification strength; do not impose a ceiling
of high or assume that all tasks of one kind have equal reasoning demands.
**Every review must use a model from a different provider than the producer of
the artifact being reviewed.** This is a hard eligibility constraint, including
explicit model choices. Different models or harnesses from the same provider do
not satisfy it. A table from kind to command proves only the planned pairing.
For pending reviews affected by a provider change, establish the artifact's
actual creator from the review's `**Creator:**` line and the run it names, or
from the human. Do not activate a change that would violate their separation;
resolve unknown provenance first. Preserve deliberate provider diversity in
research pairs.

Give a concrete comparison: task or kind, current model/effort, proposed
model/effort, reason, and any condition that would warrant a stronger choice.
Distinguish published measurements, reported experience and your inference.
Keep dated model rankings in the recommendation, not in this reusable skill.

## Make the requested change at the right scope

- **Recommendation:** inspect and present the proposed mapping and the exact
  edit. This request alone does not activate it.
- **Every checkout:** edit the policy file. Preserve the program, permission,
  approval and sandbox arguments when changing only models and effort, and do
  not introduce an unauthorized relaxation. Show every kind the edit affects,
  including kinds that share the table entry or harness you changed.
- **One checkout:** write `.harness-dispatch-choice` in that working-tree
  root, naming options the policy already offers. It replaces the policy's
  default selection whole, so name again any default modifier the user wants
  kept. The choice is kind-wide and persistent for that checkout, not limited
  to one leaf. A selection the policy does not offer needs a policy edit
  first, which is a change for every checkout: obtain authorization for that
  wider scope if the request covered only the project.
- **Every launch:** edit the settings file for a bound, the record directory
  or a grant. **Never grant `GROVE_LAUNCH_DIR`.**

A request to apply or create a policy change authorizes that scoped edit; do
not ask again for each step. Report the affected scope and how to undo it.

## Verify and report

Retain the original bytes and compose the complete candidate before publishing
it. Use a same-directory temporary file and atomic rename, preserving file
permissions and refusing to overwrite concurrent edits. If the path is a
symlink, edit the file it names. Keep provider-pair
changes together. A running loop evaluates the policy afresh at its next
launch, so a half-published change can launch.

After editing, run `harness-dispatch inspect` for every affected kind, in each
checkout whose choice file changes the answer, and compare the labels, command
and reason with what was intended. Check every producer and reviewer pair.
Nothing checks a policy before a launch: Grove's tree verbs consult none, so a
kind that inspection refuses is unrouted however cleanly its leaf was written.
Inspection evaluates trusted TypeScript, which may have side effects, and it is
a proposal, because every launch evaluates afresh. A policy that reads the
prompt selects as it will only when inspection is given the same prompt.

If verification fails, repair the change or restore only your edits,
preserving concurrent work. Report any remaining failure. A paid model call is
a separate smoke test, useful when model access is uncertain and within the
user's requested scope.

Return the changed paths, the effective kind-to-model/effort mapping,
verification results, and remaining uncertainties. State when the changes take
effect: the next launch evaluates the new policy; an already running session
keeps its model.
