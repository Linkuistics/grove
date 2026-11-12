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
alike. The major of that Ravel decides which configuration file it reads. Three
files decide the rest. [policy.md](references/policy.md) has the mechanics of each.

| File | Decides | Scope |
|---|---|---|
| `~/.config/ravel/policy.ts` on Ravel 1.x, or `~/.config/ravel/config.rhai` on Ravel 2.x | The command for each kind: program, arguments, and its provider, model and effort labels | Every checkout |
| `.ravel-choice` in a working-tree root | Which of the options the policy offers that checkout uses | One checkout's lifecycle sessions, and only under a policy that reads it. Never `grove run` |
| `~/.config/ravel/settings.json` | The selection time bound, the context budget, the environment granted to the configuration. The keys differ by major | Every launch of every kind, of both majors under one HOME |

A `config.kdl` or `.grove.kdl` from an earlier release is never read. Do not
edit one, and do not expect a converter.

## Establish the effective policy

Run `ravel --version` and `ravel start-session --help` with the PATH of the
Grove invocation, for the installed contract. Grove admits stable Ravel 1.x from
1.1.0 and stable 2.x from 2.0.0. The admission selects no route:

| Route | Ravel | Configuration | Settings |
|---|---|---|---|
| Native on 1.1 | 1.x from 1.1.0, Homebrew's `ravel` | `policy.ts`, schema 2, with Grove's TypeScript assets | Only `timeoutMs`, `contextBytes` and `policyEnv` |
| Native on 2.0 | 2.x from 2.0.0, the keg-only `ravel@2` until promotion | `config.rhai`, `SCHEMA_VERSION` 4, with Grove's Rhai assets | The keys of 2.0.0, with `{"skills": {"schemaVersion": 1, "sources": []}}` while keg sources are installed |
| Directory on 2.0 | As native on 2.0 | As native on 2.0 | The keys of 2.0.0 without `sources`. Ravel then reads its default skill directory |

Homebrew installs core and the five domain packs with Grove. It links each one
into Ravel's default skill directory, `$(brew --prefix)/share/ravel/skills/`.
`brew install` and `brew upgrade` are the whole skill setup. On the directory
route, Ravel adds the locations of those sources to each launch prompt, and no
native copy is necessary. Only the directory route delivers the skills that
Homebrew installs. On the native routes, each harness supplies its skills from
a native installation that already exists, and Grove's guidance sets up none.

Then read the configuration file of the selected major and its imports. A 2.x installation
runs only when `$(brew --prefix ravel@2)/bin` is first on the PATH of the
invocation. An incompatible first executable on PATH refuses with a remedy that
names both majors. Grove does not fall through to another copy. Never recommend
an upgrade of a shared installation that another session or configuration still
uses.

Grove 22.8 and earlier refuse Ravel 2.x and name `brew upgrade ravel`, which
cannot help once the default `ravel` is 2.x. Recommend `brew upgrade grove`
instead, because a lone Ravel upgrade does not upgrade Grove.

Ravel 1.1.0 refuses the settings keys of 2.0, and 2.0.0 refuses `policyEnv`.
The directory route through the default skill directory needs no settings key,
so a 1.1 launch and a 2.0 launch can share one HOME. While a 1.1 launch still
reads the settings under a HOME, put a 2.0 route that needs a 2.0-only key
under a separate HOME. Such keys include the `skills` object and its `sources`. Grove
converts no configuration from one major to the other.

The default skill directory belongs to the Ravel installation, not to a HOME.
A `skills` object with `sources` replaces the default sources with the named
local directories. An empty list there turns delivery off:
`{"skills": {"schemaVersion": 1, "sources": []}}`. Ravel 2.0.0 refuses a
top-level `sources` key as `settings_invalid`. Ravel runs no `git`
command and makes no network request for a source. A missing source refuses as
`skill_source_missing` before any configuration code runs, and the leaf stays
live.

Run `ravel skills resolve` to see the sources that a launch receives. The
configuration must pass `request.prompt`, which holds Ravel's preamble and the
mandate. The [usage guide](../../../../docs/USAGE.md#usage-directory-route)
gives the details.

Ravel has no verb that inspects routing, and none that installs anything.
Establish effective routing by
reading the owner's selection code, settings, wrappers and `.ravel-choice` from
the relevant checkout. For execution evidence without a paid model, use temporary
fake agents on an isolated PATH, following [policy.md](references/policy.md).
`start-session` launches the selected process. It is never a dry run.

A request to install a missing policy authorises two steps. Copy Grove's readable
sample for the selected major into the personal path, then edit the copy. The
sample is `assets/ravel-policy/sample/policy.ts` for 1.x and
`assets/ravel-policy/sample/config.rhai` for 2.x. Preserve existing policy bytes.
Do not overwrite an existing policy with the sample.

The commands of the sample disable approvals and enable full access.
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

After you edit a 1.x `policy.ts`, typecheck the complete candidate. Ravel 2.x has
no typechecker, so only launches verify a `config.rhai` candidate. Exercise
every affected kind and checkout through the real installed Ravel of that major
with temporary fake agents. Compare
the actual argv and the chosen route with the intended mapping, including the origins
of producers and reviewers. [policy.md](references/policy.md) describes that seam.

Use `task ravel:review RAVEL=/absolute/installed/ravel` for the shipped assets of
Grove, with `RAVEL_RUNTIME=2.0` for a 2.x build. An arbitrary owner policy needs fixtures for its own imports and wrappers.

Selection runs trusted code. A TypeScript selection may have side effects. A Rhai
selection can read files but writes none and launches no process. These fixtures
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
