# skills — jj adoption context

Terms for the workstream that makes agents prefer Jujutsu over git.

## Language

**jj-enabled**:
A repository in which Jujutsu is initialized — a `.jj/` directory exists,
whether colocated with `.git/` or native. The trigger condition for jj being
the primary VCS interface.
_Avoid_: jj-aware, jj-capable, "has jj" (ambiguous with the binary being
installed)

**Symmetric VCS rule**:
The repo's state alone picks the interface: jj-enabled → jj is the
interface; not jj-enabled → git, silently. The skills never convert a repo
or offer to — no `jj git init`, regardless of whether jj is installed.
_Avoid_: colocation offer (a dropped earlier design — the skills no longer
propose conversion)

**using-jujutsu**:
The workflow skill: auto-fires on VCS work, detects whether the repo is
jj-enabled, and teaches jj's native model (working-copy-as-commit,
`jj new`/`jj describe`, bookmarks, op-log undo). Harness-neutral core with
per-harness enforcement recipes.
_Avoid_: the jj skill (ambiguous with git-to-jj-mapping), Workflow skill
(pre-settlement working name)

**git-to-jj-mapping**:
The on-demand git→jj reference skill — command and concept translation,
loaded only when needed so the table costs no standing context.
_Avoid_: translation table (that's its content, not the skill), Mapping
skill (pre-settlement working name)

## Example dialogue

> **Dev:** The repo has jj installed, so the agent should commit with `jj`.
> **Expert:** Installed isn't the trigger — is the repo *jj-enabled*? If
> there's no `.jj/`, the *symmetric VCS rule* says git remains the
> interface, silently; `using-jujutsu` never offers to convert.
> **Dev:** And if the agent forgets what `git stash` becomes?
> **Expert:** That's `git-to-jj-mapping`'s job — pulled in on demand, not
> resident.
