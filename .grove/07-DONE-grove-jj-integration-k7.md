# grove-jj-integration-k7

**Kind:** planning

## Goal

Grill grove-side jj support and grow whatever leaves it needs. The skills
(`using-jujutsu`, `git-to-jj-mapping`) are the baseline: what must grove
itself change so grove-driven sessions work first-class in jj-enabled
working trees?

## Context

- **House rule stands:** all grove code/doc edits and their commits land in
  the Linkuistics/grove repo (`~/Development/grove`) — this leaf lives here
  only to *track* the work inside the workstream that motivated it
  (settled in `03-DONE-skill-design-k3.md`'s running log, Q9b).
- Known git-specific surface in grove, to grill through (verify against
  the current grove source, don't trust this list):
  - `SKILL.md`/prompt prose: "git is the history", "one focused commit",
    `git rev-parse` for repo/worktree names, "provide a working tree via
    git init / clone / worktree".
  - `grove-llm` verbs shell out to git: `leaf-decompose`/`leaf-insert` do
    `git mv`; renumber relies on it.
  - `grove do`'s launch flow and the user-owned-worktrees ADR assume git
    working trees; jj's analogue is `jj workspace`.
  - Session-naming probe: `git rev-parse --git-common-dir` for the main
    repo's basename.
- Open questions for the grilling (seed list): does grove need jj-native
  verbs, or does colocation + `using-jujutsu` cover it (jj colocated repos
  still answer `git rev-parse` and `git mv` works)? Is one-task-one-commit
  phrased VCS-neutrally? Do grove worktrees become jj workspaces or stay
  git worktrees in colocated repos?

## Done when

- Grilling run with the user; decisions logged inline here.
- Any resulting work externalized as leaves (here for tracking, with
  execution in ~/Development/grove) or as grove-repo issues — the grilling
  decides which.
- One focused commit naming `grove-jj-integration-k7`; leaf retired.

## Notes

HITL — needs the user present. Do not start editing the grove repo in this
session; the deliverable is decisions + tree growth.

## Decisions (running log)

**Verified surface (source sweep, not the seed list).** grove's git usage is
five call sites: `repo.rs` (`git_toplevel`, `git_common_dir` — used by
`launch.rs`, `llm_cli.rs`, `tree_migrate.rs`), `tree_rename.rs` (`git mv`
with an existing plain-rename fallback when untracked/no repo),
`tree_migrate.rs` (`git add`/`git commit` for the migration commit),
`launch.rs:132` (codex `--add-dir` gitdir grant), plus prose
(`content/SKILL.md`, `user-owned-worktrees` + `codex-gitdir-grant` ADRs).
Two facts sharpen the seed list: jj workspaces are never colocated (no
`--colocate` on jj 0.43's `workspace add`), so even colocated repos break
grove inside a secondary workspace; and `jj commit [FILESETS]` +
`jj workspace root` exist, covering the migrate-commit and root-resolution
needs natively.

**Q1 — support target: jj-aware plumbing.** Thin probe, no abstraction
layer. `repo.rs` learns a jj fallback (`jj workspace root` + main-repo
resolution); rename uses the plain-rename path in jj-enabled trees (jj has
no index to keep in step); `tree_migrate.rs` commits via
`jj commit .grove`; codex launch grant reviewed for `.jj`. Git remains the
default path. Rejected: colocated-only (dead on arrival for jj workspaces)
and a full VCS trait (~5 call sites don't earn it).

**Q2 — detection: jj-first.** In a colocated repo, `.jj/` presence picks jj
plumbing; git is used only when the tree is not jj-enabled. Symmetric with
the using-jujutsu skill's rule — the repo's state picks the interface — and
avoids `git mv` staging into an index jj ignores plus a git-made migration
commit jj must import.

**Q3 — docs voice: dual-VCS, git default.** Where a command is named, name
both (`git init` / `jj git init --colocate` / `jj git clone`; git worktree
or jj workspace); conceptual lines go VCS-neutral ("the VCS holds the
history"). `user-owned-worktrees` ADR extended in place to user-owned
*working trees* covering jj workspaces — grove reads no branch and no
bookmark. Rejected: fully-neutral prose (loses copy-pasteable commands)
and a jj appendix (drifts).

**Q4 — tracking vehicle: leaves here.** The user does not use GH issues,
and the grove repo has no in-repo inbox (the capture-issues prior art is
research only — no mechanism landed). Execution sessions work in
~/Development/grove and commit there; the leaf here is retired per slice,
so each slice makes two commits (grove repo: the work; this repo: the
bookkeeping). This grove stays open until the grove-side work ships —
accepted knowingly. Rejected: GH issues (not used), seeding a grove-repo
grove, a design-note doc (a TODO by another name).

**Q5 — decomposition: three sibling leaves, confirmed.** Grown with
`leaf-add`: `grove-jj-plumbing-k8` (jj-first probe in repo.rs, rename via
plain-rename in jj trees, migrate commit via `jj commit .grove`, tests for
native/workspace/colocated), `grove-codex-jj-sandbox-k9` (probe codex's
carve-out for `.jj`; adjust the `--add-dir` grant and rework the
codex-gitdir-grant ADR in place), `grove-jj-docs-k10` (dual-VCS prose
pass; user-owned-worktrees ADR reworked in place). Ordered plumbing →
sandbox → docs so prose describes shipped behaviour. Rejected: merging
k9 into k8, one combined leaf (exceeds one session).

No new glossary terms — the session reused jj-enabled and the symmetric
VCS rule as defined; no ADR raised here (the durable decisions bind the
grove repo and will land as reworked ADRs there, per k9/k10).
