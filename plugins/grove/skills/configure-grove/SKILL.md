---
name: configure-grove
description: Launch-policy configuration for Grove — the owner's Ravel policy, its settings file and a checkout's choice file. Use when installing, inspecting, recommending or editing which harness, model and effort Grove launches for a task kind, choosing an arrangement for one checkout, or remedying a refused launch.
harnesses: [any]
---

# Configure Grove

Configure the user's launch policy from the work each task must perform. This
is an operator skill, not a task kind: it needs no selected leaf and does not
start, retire or complete a Grove session.

## What decides a launch

Grove has no launch configuration. It resolves independently installed `ravel` on PATH and runs `start-session` for every
session, lifecycle and `grove run KIND` alike, and three files decide
the rest. [policy.md](references/policy.md) has the mechanics of each.

| File | Decides | Scope |
|---|---|---|
| `~/.config/ravel/policy.ts` | The command for each kind: program, arguments, and its provider, model and effort labels | Every checkout |
| `.ravel-choice` in a working-tree root | Which of the options the policy offers that checkout uses | One checkout's lifecycle sessions, and only under a policy that reads it. Never `grove run` |
| `~/.config/ravel/settings.json` | The selection time bound, the context budget, the environment granted to the policy | Every launch of every kind |

A `config.kdl` or `.grove.kdl` from an earlier release is never read. Do not
edit one, and do not expect a converter.

## Establish the effective policy

Check `ravel --version` and `ravel start-session --help` for the installed
contract, then read `~/.config/ravel/policy.ts` and its imports. Grove accepts
stable Ravel 0.1.x starting at 0.1.0; an incompatible first executable on PATH
refuses with an upgrade remedy rather than falling through to another copy.

Ravel has no inspection or installation verb. Establish effective routing by
reading the owner's selection code, settings, wrappers and `.ravel-choice` from
the relevant checkout. For execution evidence without a paid model, use temporary
fake agents on an isolated PATH, following [policy.md](references/policy.md).
`start-session` launches the selected process; it is never a dry run.

A request to install a missing policy authorizes copying Grove's readable
`assets/ravel-policy/sample/policy.ts` into the personal path and editing the
copy. Preserve existing policy bytes. Do not overwrite an existing policy with
the sample. The sample's commands disable approvals and enable full access;
explain those actual arguments when configuring a first launch and preserve the
owner's intended permission choices.

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
actual creator from the review's `**Creator:**` receipt or declaration, or
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
- **One checkout:** write `.ravel-choice` in that working-tree
  root, naming options the policy already offers. It replaces the policy's
  default selection whole, so name again any default modifier the user wants
  kept. The choice is kind-wide and persistent for that checkout, not limited
  to one leaf. A selection the policy does not offer needs a policy edit
  first, which is a change for every checkout: obtain authorization for that
  wider scope if the request covered only the project.
- **Every launch:** edit the settings file for a bound or a grant. **Never grant `GROVE_LAUNCH_DIR`.**

A request to apply or create a policy change authorizes that scoped edit; do
not ask again for each step. Report the affected scope and how to undo it.

## Verify and report

Retain the original bytes and compose the complete candidate before publishing
it. Use a same-directory temporary file and atomic rename, preserving file
permissions and refusing to overwrite concurrent edits. If the path is a
symlink, edit the file it names. Keep provider-pair
changes together. A running loop evaluates the policy afresh at its next
launch, so a half-published change can launch.

After editing, typecheck the complete candidate and exercise every affected
kind/checkout through the real installed Ravel worker with temporary fake agents.
Compare the actual argv and chosen route with the intended mapping, including
producer/reviewer origins. [policy.md](references/policy.md) describes that seam.
Use `task ravel:review RAVEL=/absolute/installed/ravel` for Grove's shipped assets;
an arbitrary owner policy needs fixtures for its own imports and wrappers.
Selection runs trusted TypeScript and may have side effects. These fixtures are
observations of the supplied inputs; the next real launch evaluates afresh.
Tree verbs consult no policy, so a clean tree does not prove a kind is routed.

If verification fails, repair the change or restore only your edits,
preserving concurrent work. Report any remaining failure. A paid model call is
a separate smoke test, useful when model access is uncertain and within the
user's requested scope.

Return the changed paths, the effective kind-to-model/effort mapping,
verification results, and remaining uncertainties. State when the changes take
effect: the next launch evaluates the new policy; an already running session
keeps its model.
