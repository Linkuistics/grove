---
name: configure-grove
description: Launch-policy configuration for Grove — the owner's Ravel policy, its settings file and a checkout's choice file. Use when you install, inspect, recommend or edit which harness, model and effort Grove launches for a task kind. Use it also to choose an arrangement for one checkout, or to remedy a refused launch.
harnesses: [any]
---

# Configure Grove

Configure the user's launch policy from the work each task must perform. This
is an operator skill, not a task kind. It needs no selected leaf, and it does not
open, retire or complete a Grove session.

## What decides a launch

Grove has no launch configuration. It resolves the independently installed `ravel`
on PATH. It runs `start-session` for every session, lifecycle and `grove run KIND`
alike. Three files decide the rest. [policy.md](references/policy.md) has the
mechanics of each.

| File | Decides | Scope |
|---|---|---|
| `~/.config/ravel/policy.ts` | The command for each kind: program, arguments, and its provider, model and effort labels | Every checkout |
| `.ravel-choice` in a working-tree root | Which of the options the policy offers that checkout uses | One checkout's lifecycle sessions, and only under a policy that reads it. Never `grove run` |
| `~/.config/ravel/settings.json` | The selection time bound, the context budget, the environment granted to the policy | Every launch of every kind |

A `config.kdl` or `.grove.kdl` from an earlier release is never read. Do not
edit one, and do not expect a converter.

## Establish the effective policy

Run `ravel --version` and `ravel start-session --help` for the installed
contract. Then read `~/.config/ravel/policy.ts` and its imports. Grove accepts
stable Ravel 1.x from 1.1.0. An incompatible first executable on PATH
refuses with an upgrade remedy. Grove does not fall through to another copy.

Ravel has no inspection or installation verb. Establish effective routing by
reading the owner's selection code, settings, wrappers and `.ravel-choice` from
the relevant checkout. For execution evidence without a paid model, use temporary
fake agents on an isolated PATH, following [policy.md](references/policy.md).
`start-session` launches the selected process. It is never a dry run.

A request to install a missing policy authorises copying the readable
`assets/ravel-policy/sample/policy.ts` of Grove into the personal path and editing
the copy. Preserve existing policy bytes. Do not overwrite an existing policy with
the sample. The commands of the sample disable approvals and enable full access.
Explain those actual arguments when you configure a first launch. Preserve the
permission choices that the owner intends.

If the command of the policy is a wrapper, or a harness profile supplies the model
or effort, inspect that source too. The labels are the assertion of the owner about
what the command runs. The argv proves the invocation. It does not prove the
settings that hide behind the command.

Read the installed `grove-<kind>` skills for the kinds that you consider, including
their named family files. For a project-specific recommendation, also read the task
description, the relevant brief chain, the acceptance criteria and the project
instructions. A task name alone is a weak indication of difficulty.

## Choose model and effort together

For model-selection work, use [model-selection.md](references/model-selection.md).
Infer the quality, time and usage priorities of the user from the request and the
existing policy. Ask only where the unresolved trade-off would change the
recommendation.

Higher abstraction, higher consequences of error or both justify more effort.
Account for downstream rework and verification strength. Do not impose a ceiling of
high. Do not assume that all tasks of one kind have equal reasoning demands.

**Every review must use a model from a different provider than the producer of the
artifact being reviewed.** This is a hard eligibility constraint, including explicit
model choices. Different models or harnesses from the same provider do not satisfy
it. A table from kind to command proves only the planned pairing.

A provider change can affect pending reviews. For each one, establish the actual
creator of the artifact. Read the `**Creator:**` receipt or declaration of the
review, or ask the human. Do not activate a change that would violate that
separation. Resolve unknown provenance first. Preserve deliberate provider diversity
in research pairs.

Give a concrete comparison. Include these parts:

- The task or kind.
- The current model and effort.
- The proposed model and effort.
- The reason.
- Any condition that would warrant a stronger choice.

Distinguish published measurements, reported experience
and your inference. Keep dated model rankings in the recommendation. Do not put them
in this reusable skill.

## Make the requested change at the right scope

- **Recommendation:** Inspect and present the proposed mapping and the exact edit.
  This request alone does not activate it.
- **Every checkout:** Edit the policy file. When you change only models and effort,
  preserve the program, permission, approval and sandbox arguments. Do not introduce
  an unauthorised relaxation. Show every kind that the edit affects, including kinds
  that share the table entry or harness that you changed.
- **One checkout:** Write `.ravel-choice` in that working-tree root. Name options
  that the policy already offers. The file replaces the default selection of the
  policy whole. So name again any default modifier that the user wants to keep. The
  choice is kind-wide and persistent for that checkout. It is not limited to one
  leaf. A selection that the policy does not offer needs a policy edit first. That
  edit is a change for every checkout. If the request covered only the project,
  obtain authorisation for that wider scope.
- **Every launch:** Edit the settings file for a bound or a grant. **Never grant
  `GROVE_LAUNCH_DIR`.**

A request to apply or create a policy change authorises that scoped edit. Do not
ask again for each step. Report the affected scope and how to undo it.

## Verify and report

Retain the original bytes and compose the complete candidate before you publish it.
Use a same-directory temporary file and an atomic rename. Preserve file permissions.
Refuse to overwrite concurrent edits. If the path is a symlink, edit the file that
it names.

Keep provider-pair changes together. A running loop evaluates the policy afresh at
its next launch. A half-published change can therefore launch.

After you edit, typecheck the complete candidate. Exercise every affected kind and
checkout through the real installed Ravel worker with temporary fake agents. Compare
the actual argv and the chosen route with the intended mapping, including the origins
of producers and reviewers. [policy.md](references/policy.md) describes that seam.

Use `task ravel:review RAVEL=/absolute/installed/ravel` for the shipped assets of
Grove. An arbitrary owner policy needs fixtures for its own imports and wrappers.
Selection runs trusted TypeScript, and it may have side effects. These fixtures
observe the supplied inputs. The next real launch evaluates afresh. Tree verbs
consult no policy, so a clean tree does not prove that a kind has a route.

If verification fails, repair the change or restore only your edits. Preserve
concurrent work. Report any remaining failure. A paid model call is a separate smoke
test. It is useful when model access is uncertain and within the requested scope of
the user.

Return the changed paths, the effective mapping from kind to model and effort, the
verification results and the remaining uncertainties. State when the changes take
effect. The next launch evaluates the new policy. An already running session keeps
its model.
