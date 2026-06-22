# `grove install` and `grove update` create git commits by default

`grove install` and `grove update` materialise the skill into the target repo by writing files. Historically the user had to `git add` and `git commit` afterward — empirically they did, e.g., `e674ea9 Install grove v2.0.0`. We now make commit-creation default behavior: both verbs produce a single **path-scoped commit** covering the **install scope** (every targeted harness path, combined). `--no-commit` opts out; `--message <text>` overrides the default message.

## Status
superseded by ADR-0031 — the `install`/`update` materialise machinery (and its path-scoped commit over the install scope) was deleted in leaf 090; distribution is now the binary-embedded global skill, provisioned on `grove do`.

## Why default-on
The manual step is forgettable and the existing pattern shows users already commit by hand. Default-on removes ceremony and keeps materialisation traceable.

## Why path-scoped, not index-sweeping
Explicit `git add <paths>` + `git commit -- <paths>` ensures grove never accidentally commits unrelated work-in-progress (or secrets staged for an in-flight change). grove refuses *only* when the install-scope paths themselves have pre-existing staged hunks unrelated to the install — that case is too ambiguous to resolve automatically. Unrelated dirty state elsewhere in the tree is fine.

## Why fail-loudly on commit failure
If the commit step fails (pre-commit hook rejects, detached HEAD, etc.), grove leaves the materialisation in place and exits non-zero with a follow-up command (`git commit -- <paths>`). Rolling the materialisation back would discard work the user can finish manually once the hook issue is resolved. Exiting zero would hide real failures from CI.

## Why one commit, not one per harness
A multi-harness invocation is one logical action; a single coherent diff is easier to review and to revert.

## Why grove does not touch the ADR on update
A scaffolded ADR with no content is worse than no ADR — it pretends to be a decision record. The existing eprintln nudge on update stays; the user authors the ADR.

## Default message
- Install: `Install grove v<version>`
- Update: `Update grove to v<version>`

Plain, descriptive, makes no assumption about whether the downstream repo uses Conventional Commits.
