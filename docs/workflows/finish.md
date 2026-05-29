# `grove finish` — walkthrough

> ⚠️ **Superseded — pending rewrite.** The `grove finish` *verb* has been removed: `grove do` is now the sole lifecycle entry verb, and finishing a grove is an **in-session** step (when the grove has no live leaves left, the running loop proposes the complete finish cycle). The step-by-step flow below still describes the *what* — promote durable output, delete `.grove/`, merge, drop the branch and worktree — but the *trigger* is no longer a `grove finish` invocation. This walkthrough (and the `prompts/finish.md` launcher prompt it references) will be rewritten to the in-session model once that flow is designed (grove leaf `020-do-proposes-finish-cycle`). Read the command invocations below as historical until then.

Close out a completed grove. By the end of this walkthrough, `add-rate-limiting`'s durable output has been promoted out of `.grove/`, the scaffolding has been deleted in a focused commit, the branch has been merged into `acme/orders-api`'s default branch, and the worktree is gone. The default branch shows the grove's history as a contiguous run of commits — no `.grove/` left behind.

> This page is about driving the **grove CLI**. For *why a finished grove must not leave its scaffolding on the default branch*, see [`../grove.md`](../grove.md); for what the harness session does step-by-step, the canonical instructions are in [`../../content/prompts/finish.md`](../../content/prompts/finish.md).

## Starting state

We pick up where [`multi-step.md`](multi-step.md) left off, several sessions further on: every leaf under `.grove/` has now been retired, the last node has been collapsed, and only `BRIEF.md` plus a fully populated `done/` remain.

```
$ cd acme/orders-api/.grove-worktrees/add-rate-limiting
$ tree .grove
.grove
├── BRIEF.md
└── done
    ├── 010-spike-token-bucket.md
    ├── 020-design-token-bucket
    │   ├── 010-record-policy-adr.md
    │   └── BRIEF.md
    └── 030-implement.md

$ git log --oneline -5
a9b8c7d chore(grove): retire 030-implement
8e7d6c5 feat(rate-limit): wire token-bucket middleware
e1d0c9b chore(grove): retire 020-design-token-bucket — promote design intent
b2a1d9c chore(grove): retire 020-design-token-bucket/010-record-policy-adr
3e2f1a0 docs(adr): 0002 token-bucket policy
```

The implementation is committed; the design intent has been recorded as an ADR; the root `BRIEF.md` has had design intent promoted into it from the retired node. The grove is done. Time to wrap up.

## `grove finish` is a harness launcher

```
$ grove finish add-rate-limiting
```

The CLI does not delete, promote, or merge anything itself. It exec's the harness in the worktree with the **finish prompt** (`prompts/finish.md` in the materialised content) and lets the session conduct the four-step wrap-up:

1. **Promote** anything from `.grove/`'s briefs that should outlive the grove — to an ADR, a design doc, or `CONTEXT.md`.
2. **Delete** `.grove/` in one focused commit.
3. **Merge** the branch into the default branch per this project's convention.
4. **Remove** the worktree and delete the branch.

Steps 1–3 happen *inside* the running harness session. Step 4 is awkward because the session lives inside the worktree it is about to remove — see the cleanup note below.

## Step 1: promotion is where judgement lives

This is the highest-stakes step. Anything load-bearing that *only* lives in `.grove/`'s briefs must find a permanent home before deletion. Typical candidates:

- **Decisions** that were hard to reverse or that traded something off → a new ADR under `docs/adr/`.
- **Design intent or rationale** that future contributors will want to know → `docs/grove.md`, `docs/specs/<area>-design.md`, or the README depending on audience.
- **Glossary terms** that emerged during the grove → `CONTEXT.md` (or the relevant per-bounded-context glossary). In a disciplined grove these landed inline at the moment they were resolved, so this is usually a no-op double-check, not new work.

What is *not* promoted: process scaffolding. Decomposition rationale, planning notes, the order in which things were tackled — those are exactly what `.grove/` exists to hold, and they exit when the directory exits. If the discipline of "what to promote vs. what to delete" feels unclear, the litmus is: would a reader who never opens `.grove/` ever need this? If yes, promote; if no, let it go.

The session commits the promotion(s) as one or more focused commits before moving to step 2:

```
$ git log --oneline -2
4f3e2d1 docs: record rate-limiting design intent in docs/specs/rate-limiting.md
a9b8c7d chore(grove): retire 030-implement
```

If `.grove/` carried nothing worth promoting (a small grove whose conclusions were entirely captured in its commits and ADRs as it went), this step produces no commits at all — just an explicit check that nothing was left behind.

## Step 2: delete `.grove/` in one focused commit

```
$ git rm -r .grove
$ git commit -m "chore: remove grove scaffolding"
$ git log --oneline -1
b6a5f4e chore: remove grove scaffolding
```

One commit, scoped to `.grove/` only. The point is to keep the default branch — which this commit will soon land on — free of any grove's local state. The history of completed groves lives in git's commit graph, not in retained directories.

```
$ tree -L 1
.
├── Cargo.toml
├── README.md
├── docs
├── src
└── tests
```

No `.grove/` left. The worktree still exists on disk (you are standing in it), but it now looks like any other branch of `acme/orders-api`.

## Step 3: merge into the default branch per project convention

grove guides, it does not gate (constraint 5). The CLI takes no opinion on *how* the grove branch lands on `main`; the project's convention does. Three common shapes, pick whichever matches yours:

```
# Fast-forward merge (small, single-author grove, no PR review required):
$ git -C ~/code/acme/orders-api checkout main
$ git -C ~/code/acme/orders-api merge --ff-only add-rate-limiting

# Squash merge (collapse the grove's history into one main commit):
$ git -C ~/code/acme/orders-api checkout main
$ git -C ~/code/acme/orders-api merge --squash add-rate-limiting
$ git -C ~/code/acme/orders-api commit -m "feat: add rate limiting (grove add-rate-limiting)"

# PR-and-rebase (standard team convention):
$ git push -u origin add-rate-limiting
$ gh pr create --base main --head add-rate-limiting --title "feat: add rate limiting"
# ... reviews, then merge via the PR UI
```

The session running the finish prompt picks whichever shape fits the project and runs the corresponding commands. The convention is the variable; the discipline of one promotion + one deletion *before* the merge is invariant.

After a fast-forward merge, the grove's history appears on `main` as a contiguous run:

```
$ git -C ~/code/acme/orders-api log --oneline main -8
b6a5f4e chore: remove grove scaffolding
4f3e2d1 docs: record rate-limiting design intent in docs/specs/rate-limiting.md
a9b8c7d chore(grove): retire 030-implement
8e7d6c5 feat(rate-limit): wire token-bucket middleware
e1d0c9b chore(grove): retire 020-design-token-bucket — promote design intent
b2a1d9c chore(grove): retire 020-design-token-bucket/010-record-policy-adr
3e2f1a0 docs(adr): 0002 token-bucket policy
2c4d5e6 Bootstrap add-rate-limiting: root brief + initial leaves
```

Under a squash merge, the same range collapses into one commit on `main` — and the grove's per-task history is preserved on the merged-out branch in the reflog and remote, not on `main`. Either is fine; the constraint is that *whatever lands on `main` carries no `.grove/`*.

## Step 4: worktree and branch cleanup

The session is still standing inside the soon-to-be-removed worktree, so the cleanup commands target the main repo directly with `git -C`:

```
$ git -C ~/code/acme/orders-api worktree remove .grove-worktrees/add-rate-limiting
$ git -C ~/code/acme/orders-api branch -d add-rate-limiting
```

`git worktree remove` deletes the worktree directory and untracks it from `git worktree list`. `git branch -d` deletes the branch *only if it has been merged* into its upstream — which it has, after step 3. (If you squashed, `git branch -d` will refuse because git can't see the squash relationship; `-D` is appropriate there since the squash commit on `main` is the canonical record.)

In practice the harness session running `grove finish` does steps 1–3, then exits, and you run step 4 from the main-repo shell once the session is gone. There is no harm in this split — the work is done; the cleanup is a tidy-up.

## Multi-harness stamp cleanup

If the repo runs multiple harnesses and `.grove-stamps/add-rate-limiting` was written at `grove start` time, remove it alongside the worktree:

```
$ rm ~/code/acme/orders-api/.grove-stamps/add-rate-limiting
```

The stamp's sole purpose was to bind the grove to its harness across launcher verbs; with the grove gone, the stamp is dead state. Single-harness repos never had a stamp and have nothing to clean up.

## After finishing

```
$ git -C ~/code/acme/orders-api worktree list
~/code/acme/orders-api  b6a5f4e [main]

$ ls ~/code/acme/orders-api/.grove-worktrees 2>/dev/null || echo "(none)"
(none)
```

The grove is gone. Its durable output — code, ADRs, design docs, glossary entries — lives in the repo on `main`, where every contributor can find it without knowing this grove ever existed.

## Codex harness

Identical flow. The harness binary differs, the finish prompt is read from `.codex/skills/grove/prompts/finish.md` rather than `.claude/skills/grove/prompts/finish.md`, and the prompt body is identical. The four steps, the merge-convention freedom, the stamp-cleanup detail, and the post-finish state of `main` are all the same.
