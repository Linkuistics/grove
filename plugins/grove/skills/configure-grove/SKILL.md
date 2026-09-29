---
name: configure-grove
description: Grove launch-policy configuration for global defaults and project overrides by task kind. Use when inspecting, recommending, updating or creating Grove model/effort mappings, profiles or .grove.kdl files.
harnesses: [any]
---

# Configure Grove

Configure the user's launch policy from the work each task must perform. This
is an operator skill, not a task kind: it needs no selected leaf and does not
start, retire or complete a Grove session.

## Establish the effective policy

Read `~/.config/grove/config.kdl`. In a jj workspace, run
`grove config show --json` and inspect its sources, selection, resolved commands
and parameter provenance. Read the selected local delta, if present. The
effective values matter more than comments or an inactive profile's contents.

Establish whether the target is a lifecycle task or `grove run KIND` first.
Standalone invocations read personal policy only; a workspace inspection with
a local delta does not describe their effective policy. For standalone work,
inspect from a delta-free workspace or explicitly qualify the validation.

Read the installed `grove-<kind>` skills for the routes being considered,
including their named family files. Discover custom routes from configuration;
their contract may be a standalone prompt or a staged skill instead. For a
project-specific recommendation, also read the task description, relevant brief
chain, acceptance criteria and project instructions. A task name alone is a
weak indication of difficulty.

Use [configuration.md](references/configuration.md) when writing either file.
Check installed Grove and harness versions before depending on new syntax,
model IDs or effort levels. Configuration inspection validates Grove policy;
it does not prove model access or launch a harness.
If a wrapper, harness profile/configuration or dispatcher supplies the model or
effort, inspect that source and its supported inspection interface too. Grove's
resolved argv proves the invocation, not settings hidden behind it. Identify
the actual owner of each value and any flag that would override a profile.

## Choose model and effort together

For model-selection work, use [model-selection.md](references/model-selection.md).
Infer the user's quality, time and usage priorities from the request and existing
policy; ask only where the unresolved trade-off would change the recommendation.
Higher abstraction and/or higher consequences of error justify more effort.
Account for downstream rework and verification strength; do not impose a ceiling
of high or assume that all tasks of one kind have equal reasoning demands.
**Every review must use a model from a different provider than the producer of
the artifact being reviewed.** This is a hard eligibility constraint, including
explicit model choices and fallbacks. Different models or harnesses from the
same provider do not satisfy it. Grove does not record the model provider of a
past launch: current defaults prove only planned pairings. For pending reviews
affected by a provider change, establish the artifact's actual producers from
the human or trustworthy session records. Do not activate a change that would
violate their separation; resolve unknown provenance first. An external dynamic
dispatcher must obtain and enforce this evidence at launch, including fallbacks.
Preserve deliberate provider diversity in research pairs.

Give a concrete comparison: task or kind, current model/effort, proposed
model/effort, reason, and any condition that would warrant a stronger choice.
Distinguish published measurements, reported experience and your inference.
Keep dated model rankings in the recommendation, not in this reusable skill.

## Make the requested change at the right scope

- **Recommendation:** inspect and present the proposed mapping and exact KDL
  changes. This request alone does not activate them.
- **Global defaults:** update the personal file's commands, shared values,
  bindings, routes or profiles. Prefer reuse of existing command templates;
  preserve executable, permission, approval and sandbox settings when changing
  only models and effort. For redirects, check execution-context compatibility,
  carried parameters and permission differences as configuration.md directs.
  Show every affected route, including inherited users.
- **Project or worktree:** create or patch the selected `.grove.kdl` with only
  the differences from personal policy. Establish ignore coverage before any
  write that jj could snapshot. Preserve unrelated existing overrides and
  account for a worktree delta shadowing the repository delta. These overrides
  persist for matching kinds in that scope; they are not limited to one leaf.
  Standalone invocations require personal-policy changes instead. If existing
  authorization covers only project policy, obtain authorization for that scope
  expansion before editing the personal file.

A request to apply or create configuration authorizes that scoped edit; do not
ask again for each step. If a project needs a command or kind not admitted by
personal policy, explain the required global addition and obtain authorization
if the existing request does not cover it. Apply the same scope test to harness
profiles or wrapper policy. A task-tailored change belongs in the local delta
unless the user wants it as the new default everywhere; it remains kind-wide
and persistent, so report the affected scope and how to undo your override.

## Verify and report

Retain the original bytes and compose the complete candidate before publishing
it. Use a same-directory temporary file and atomic rename, preserving file
permissions and refusing to overwrite concurrent edits. Keep provider-pair
changes together; if several files must change, establish a safe ordering or
coordinate a pause of affected launches with the human. Never toggle a live
selection merely to validate another profile. configuration.md describes scratch
inspection. Running loops use a new policy at their next launch.

After editing, run
`grove config show --json` in the intended workspace and compare the resolved
model, effort, command and sources for all affected routes. Check the complete
report for unintended changes and non-admitted keys. Validate an alternate
profile in its intended selection; an inactive profile is not evidence that its
references resolve. A local delta can mask a broken global reference, so verify
global changes in a workspace without a delta as well. Inspect known affected
workspaces for masking or broken deltas; report the verification scope and any
workspaces not checked rather than claiming machine-wide validation.
Check every producer/reviewer pair after resolving all overrides. Static checks
establish the planned pairing; a dynamic dispatcher must also enforce the rule
against the actual producer's recorded provider before launching the review.

If validation fails, repair the proposed change or restore only your edits,
preserving concurrent work. Report any remaining failure. Without a suitable jj
workspace, provide a clearly marked unvalidated candidate rather than claiming
`grove config show` passed. A paid model call is a separate smoke test, useful
when model access is uncertain and within the user's requested scope.

Return the changed paths, the effective task-to-model/effort mapping, validation
results, and remaining uncertainties. State when the changes take effect:
subsequent launches reload policy; an already running task keeps its model.
