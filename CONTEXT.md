# skills — jj adoption context

Terms for the workstream that makes agents prefer Jujutsu over git. The skill
names below are working names until `skill-design-k3` settles the canonical
ones.

## Language

**jj-enabled**:
A repository in which Jujutsu is initialized — a `.jj/` directory exists,
whether colocated with `.git/` or native. The trigger condition for jj being
the primary VCS interface.
_Avoid_: jj-aware, jj-capable, "has jj" (ambiguous with the binary being
installed)

**Colocation offer**:
The once-per-session suggestion to run `jj git init --colocate` in a repo
that is not jj-enabled while the jj binary is installed. An offer, never a
silent conversion.
_Avoid_: auto-colocation, conversion

**Workflow skill**:
The skill that auto-fires on VCS work, detects whether the repo is
jj-enabled, and teaches jj's native model (working-copy-as-commit,
`jj new`/`jj describe`, bookmarks, op-log undo). Working name.
_Avoid_: the jj skill (ambiguous with the Mapping skill)

**Mapping skill**:
The on-demand git→jj reference skill — command and concept translation,
loaded only when needed so the table costs no standing context. Working name.
_Avoid_: translation table (that's its content, not the skill)

## Example dialogue

> **Dev:** The repo has jj installed, so the agent should commit with `jj`.
> **Expert:** Installed isn't the trigger — is the repo *jj-enabled*? If
> there's no `.jj/`, the Workflow skill only makes a *colocation offer*; until
> that's accepted, git remains the interface.
> **Dev:** And if the agent forgets what `git stash` becomes?
> **Expert:** That's the Mapping skill's job — pulled in on demand, not
> resident.
