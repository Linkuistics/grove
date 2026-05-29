# Finishing a grove — walkthrough

Close out a completed grove. There is no `grove finish` verb: `grove do` is the sole lifecycle entry verb, and finishing is an **in-session** step. When a grove has no live leaves left, the running session proposes the **complete finish cycle** and, on one confirmation, carries it out end to end. By the end of this walkthrough, `add-rate-limiting`'s durable output has been promoted out of `.grove/`, the scaffolding has been deleted in a focused commit, the branch has been merged into `acme/orders-api`'s default branch, and the worktree and branch are gone. The default branch shows the grove's history as a contiguous run of commits — no `.grove/` left behind.

> This page is about driving the **grove CLI** through its finish step. For *why a finished grove must not leave its scaffolding on the default branch*, see [`../grove.md`](../grove.md); for the canonical step-by-step the session follows, the source of truth is the **Finish** step of the methodology in [`../../content/SKILL.md`](../../content/SKILL.md) (materialised per harness at `.claude/skills/grove/SKILL.md`), with the step-level design rationale in [`../adr/0010-in-session-finish-cycle.md`](../adr/0010-in-session-finish-cycle.md).

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

## The trigger is an empty pick, not a verb

There is nothing to *launch*. You finish a grove from inside an ordinary session — the same `grove do` you would run to do the next task:

```
$ grove do add-rate-limiting
```

`grove do` opens the continue session as always. Its first move is to pick the next live leaf — and this time the walk finds none:

```
$ grove-llm pick
# (no output on stdout)
# stderr: no live leaves; this grove is done
```

An empty `pick` (exit 0, nothing on stdout, *"no live leaves; this grove is done"* on stderr) is the finish trigger. The session recognises it and switches from "do the next task" to "propose the complete finish cycle." No CLI-side finished-detection is involved; the methodology in `SKILL.md` drives the whole thing.

## The complete finish cycle is five steps

The session proposes the cycle as five ordered steps:

1. **Promote** anything from `.grove/`'s briefs that should outlive the grove — to an ADR, a design doc, or `CONTEXT.md`.
2. **Delete** `.grove/` in one focused commit on the grove branch.
3. **Merge** the branch into the default branch (`git -C <repo> merge <name>`).
4. **Remove** the worktree.
5. **Delete** the branch.

Step 1 is ordinary, reviewable session work — its edits land in the diff like any other commit. Steps 2–5 are the mechanical teardown, and they are gated by **one** confirmation (see below). Two details that the seed sketch of this cycle got wrong and are worth internalising: **worktree-remove precedes branch-delete** (git refuses `git branch -d` on a branch checked out in a live worktree), and the teardown runs against the main repo with `git -C <repo>` rather than `cd`-ing — the session is standing *inside* the worktree it is about to remove, so it never changes its own directory out from under itself.

## Step 1: promotion is where judgement lives

This is the highest-stakes step. Anything load-bearing that *only* lives in `.grove/`'s briefs must find a permanent home before deletion. Typical candidates:

- **Decisions** that were hard to reverse or that traded something off → a new ADR under `docs/adr/`.
- **Design intent or rationale** that future contributors will want to know → `docs/grove.md`, `docs/specs/<area>-design.md`, or the README depending on audience.
- **Glossary terms** that emerged during the grove → `CONTEXT.md` (or the relevant per-bounded-context glossary). In a disciplined grove these landed inline at the moment they were resolved, so this is usually a no-op double-check, not new work.

What is *not* promoted: process scaffolding. Decomposition rationale, planning notes, the order in which things were tackled — those are exactly what `.grove/` exists to hold, and they exit when the directory exits. If the discipline of "what to promote vs. what to delete" feels unclear, the litmus is: would a reader who never opens `.grove/` ever need this? If yes, promote; if no, let it go.

The session commits the promotion(s) as one or more focused commits before moving on:

```
$ git log --oneline -2
4f3e2d1 docs: record rate-limiting design intent in docs/specs/rate-limiting.md
a9b8c7d chore(grove): retire 030-implement
```

If `.grove/` carried nothing worth promoting (a small grove whose conclusions were entirely captured in its commits and ADRs as it went), this step produces no commits at all — just an explicit check that nothing was left behind. Because promotion is normal reviewable work, it happens *before* the confirmation gate: the diff is there to inspect when you decide whether to proceed.

## The confirmation gate

With promotion done, the session presents the concrete teardown plan and **waits** for explicit confirmation before running anything destructive:

```
Ready to finish grove `add-rate-limiting`. This will:
  2. delete .grove/ in one commit
  3. merge add-rate-limiting → main (in ~/code/acme/orders-api)
  4. remove worktree ~/code/acme/orders-api/.grove-worktrees/add-rate-limiting
  5. delete branch add-rate-limiting
Proceed? (yes/no)
```

One gate, not five. Per-step confirmation was rejected as a flow-breaking "wizard" — and it would buy nothing, because nothing in the cycle is irreversible in git: the `.grove/` deletion is a commit, the merge is revertible, deleting a *merged* branch loses nothing, and a removed worktree is re-attachable. The real risk the gate guards against is finishing the *wrong* grove, which one clear plan-and-confirm addresses. The plan names the merge target, the worktree path, and the branch so that risk is visible.

This single rule also makes the cycle safe **headless**, with no mode detection: the session proposes the teardown and waits. An interactive run gets a confirmation and proceeds; a headless run with no human present simply ends the turn with the plan as output and runs nothing destructive.

## Step 2: delete `.grove/` in one focused commit

On confirmation, the session removes the scaffolding:

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

No `.grove/` left. The worktree still exists on disk (the session is standing in it), but it now looks like any other branch of `acme/orders-api`.

## Step 3: merge into the default branch

The canonical cycle uses a **plain `git merge`**, run against the main repo so the session never has to leave the worktree:

```
$ git -C ~/code/acme/orders-api merge add-rate-limiting
```

Plain `git merge` fast-forwards when the default branch has not advanced since the grove branched, and makes a merge commit when it has. It never blocks on policy and never manufactures a merge bubble on a clean fast-forward (the reason `--no-ff`-always was rejected). On a genuine conflict the cycle stops and the operator — or the in-session LLM — resolves it before continuing.

After a fast-forward merge, the grove's history appears on the default branch as a contiguous run:

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

grove guides, it does not gate (constraint 5): a team whose convention is squash-or-PR can of course land the branch that way instead — but that is a deliberate deviation the operator drives, not something the in-session cycle chooses. The canonical cycle runs the plain merge; the invariant is one promotion + one deletion *before* whatever merge shape lands.

## Step 4: remove the worktree

```
$ git -C ~/code/acme/orders-api worktree remove .grove-worktrees/add-rate-limiting
```

`git worktree remove` deletes the worktree directory and untracks it from `git worktree list`. It runs before the branch delete because git refuses to delete a branch that is checked out in a live worktree.

## Step 5: delete the branch

```
$ git -C ~/code/acme/orders-api branch -d add-rate-limiting
```

`git branch -d` is the *safe* delete — it succeeds only because step 3 merged the branch into the default. (If you deviated to a squash merge in step 3, `git branch -d` will refuse because git can't see the squash relationship; `-D` is appropriate there, since the squash commit on the default branch is the canonical record.)

## Resume is read from state, never a marker file

grove keeps no finish-progress file (constraint 1, *artifacts not state*). If a finish is interrupted — a conflicted merge, a closed terminal — you resume by running `grove do add-rate-limiting` again, and the session works out where it stopped from inspectable git and filesystem state:

- The two entry conditions are distinguishable by `grove-llm pick` itself: exit 0 + empty stdout + *"no live leaves; this grove is done"* means `.grove/` is still present (fresh finish, run from step 1); a non-zero exit + *"grove root not found"* means `.grove/` is already gone (steps 1–2 done, resume at the merge).
- From there each step is guarded: skip 1–2 if `.grove/` is gone; skip 3 if `git -C <repo> merge-base --is-ancestor add-rate-limiting main` passes (already merged); skip 4 if the worktree is gone; skip 5 if the branch is gone; if all are done, report *"already finished"* and stop.

One benign quirk: in the window where the worktree is gone but the branch remains, `grove do` re-attaches the worktree before the session starts, so step 4 simply removes it again — wasteful but convergent.

## Multi-harness stamp cleanup

If the repo runs multiple harnesses and `.grove-stamps/add-rate-limiting` was written at `grove do` time (to bind the grove to its harness — see [`start.md`](start.md)), remove it alongside the worktree:

```
$ rm ~/code/acme/orders-api/.grove-stamps/add-rate-limiting
```

The stamp's sole purpose was to bind the grove to its harness across verbs; with the grove gone, the stamp is dead state. Single-harness repos never had a stamp and have nothing to clean up.

## After finishing

```
$ git -C ~/code/acme/orders-api worktree list
~/code/acme/orders-api  b6a5f4e [main]

$ ls ~/code/acme/orders-api/.grove-worktrees 2>/dev/null || echo "(none)"
(none)
```

The grove is gone. Its durable output — code, ADRs, design docs, glossary entries — lives in the repo on the default branch, where every contributor can find it without knowing this grove ever existed.

## Codex harness

Identical flow. Finishing is an in-session step driven by the same methodology, so there is no per-harness finish prompt to differ — the Codex session reads its loop from `.codex/skills/grove/SKILL.md` rather than `.claude/skills/grove/SKILL.md`, and the body is identical. The trigger (empty `grove-llm pick`), the five steps, the single confirmation gate, the plain merge, and the post-finish state of the default branch are all the same.
