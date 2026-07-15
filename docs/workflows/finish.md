# Finishing a grove — walkthrough

Close out a completed grove. There is no `grove finish` verb: `grove do` is the sole lifecycle entry verb, and finishing is an **in-session** step. When a grove has no live leaves left, the running session proposes the **complete finish cycle** and, on one confirmation, carries it out end to end. By the end of this walkthrough, `add-rate-limiting`'s durable output has been promoted out of `.grove/`, and the scaffolding has been deleted in a focused commit. That's where grove's cycle ends — integrating the branch (a merge, a PR, whatever your team's convention is) and tearing down the working tree are the user's own git/gh, or their worktree tooling, done whenever they choose (user-owned-worktrees).

> This page is about driving the **grove CLI** through its finish step. For *why a finished grove must not leave its scaffolding on the branch you integrate*, see [`../grove.md`](../grove.md); for the canonical step-by-step the session follows, the source of truth is the **Finish** step of the methodology in [`../../content/SKILL.md`](../../content/SKILL.md) (provisioned to the global `~/.claude/skills/grove/SKILL.md`), with the step-level design rationale in [`../adr/in-session-finish-cycle.md`](../adr/in-session-finish-cycle.md).

## Starting state

We pick up where [`multi-step.md`](multi-step.md) left off, several sessions further on: every leaf under `.grove/` has now been retired in place (each carries a `DONE` infix), and the design node's brief has been promoted upward. The tree still shows its full shape — done-ness and all — because nothing was moved into a separate folder.

```
$ cd ~/code/acme/add-rate-limiting
$ tree .grove
.grove
├── 01-DONE-plan-k1.md
├── 02-DONE-spike-token-bucket-k2.md
├── 03-design-token-bucket-k3
│   ├── 01-DONE-record-policy-adr-k5.md
│   └── BRIEF.md
├── 04-DONE-implement-k4.md
└── BRIEF.md

$ git log --oneline -5
a9b8c7d chore(grove): retire implement-k4
8e7d6c5 feat(rate-limit): wire token-bucket middleware
e1d0c9b chore(grove): retire design-token-bucket-k3 — promote design intent into root brief
b2a1d9c chore(grove): retire record-policy-adr-k5
3e2f1a0 docs(adr): 0002 token-bucket policy
```

The implementation is committed; the design intent has been recorded as an ADR; the root `BRIEF.md` has had design intent promoted into it from the retired node. The grove is done. Time to wrap up.

## The trigger is an empty pick, not a verb

There is nothing to *launch*. You finish a grove from inside an ordinary session — the same argument-less `grove do` you would run to do the next task:

```
$ grove do
```

`grove do` opens the continue session as always. Its first move is to pick the next live leaf — and this time the walk finds none:

```
$ grove-llm pick
# (no output on stdout)
# stderr: no live leaves; this grove is done
```

An empty `pick` (exit 0, nothing on stdout, *"no live leaves; this grove is done"* on stderr) is the finish trigger. The session recognises it and switches from "do the next task" to "propose the complete finish cycle." No CLI-side finished-detection is involved; the methodology in `SKILL.md` drives the whole thing.

## The complete finish cycle is three steps

The session proposes the cycle as three ordered steps:

1. **Promote** anything from `.grove/`'s briefs that should outlive the grove — to an ADR, a design doc, or `CONTEXT.md`.
2. **Delete** `.grove/` in one focused commit on the grove's branch.
3. **Signal** the loop to stop with `grove-llm complete --done`.

Step 1 is ordinary, reviewable session work — its edits land in the diff like any other commit. Step 2 is the one piece of mechanical teardown, gated by **one** confirmation (see below). Step 3 is bookkeeping for the loop, not the working tree. Nothing after step 3 is grove's concern: because grove created no git topology, it merges none and deletes none either (user-owned-worktrees) — integrating the branch and tearing down the working tree are entirely up to you, on your own schedule, using your own tooling.

## Step 1: promotion is where judgement lives

This is the highest-stakes step. Anything load-bearing that *only* lives in `.grove/`'s briefs must find a permanent home before deletion. Typical candidates:

- **Decisions** that were hard to reverse or that traded something off → a new ADR under `docs/adr/`.
- **Design intent or rationale** that future contributors will want to know → `docs/grove.md`, `docs/specs/<slug>.md`, or the README depending on audience.
- **Glossary terms** that emerged during the grove → `CONTEXT.md` (or the relevant per-bounded-context glossary). In a disciplined grove these landed inline at the moment they were resolved, so this is usually a no-op double-check, not new work.

What is *not* promoted: process scaffolding. Decomposition rationale, planning notes, the order in which things were tackled — those are exactly what `.grove/` exists to hold, and they exit when the directory exits. If the discipline of "what to promote vs. what to delete" feels unclear, the litmus is: would a reader who never opens `.grove/` ever need this? If yes, promote; if no, let it go.

The session commits the promotion(s) as one or more focused commits before moving on:

```
$ git log --oneline -2
4f3e2d1 docs: record rate-limiting design intent in docs/specs/rate-limiting.md
a9b8c7d chore(grove): retire implement-k4
```

If `.grove/` carried nothing worth promoting (a small grove whose conclusions were entirely captured in its commits and ADRs as it went), this step produces no commits at all — just an explicit check that nothing was left behind. Because promotion is normal reviewable work, it happens *before* the confirmation gate: the diff is there to inspect when you decide whether to proceed.

## The confirmation gate

With promotion done, the session presents the concrete teardown plan and **waits** for explicit confirmation before running anything destructive:

```
Ready to finish grove `add-rate-limiting`. This will:
  2. delete .grove/ in one commit
  3. signal the loop to stop (grove-llm complete --done)
Proceed? (yes/no)
```

One gate, not several. Nothing in the cycle is irreversible in git: the `.grove/` deletion is a commit, and the signal step touches nothing but the loop's own state. The real risk the gate guards against is finishing the *wrong* grove — accidentally proposing this in a working tree that still has more to do. Naming the grove in the plan makes that risk visible before anything happens.

This single rule also makes the cycle safe **headless**, with no mode detection: the session proposes the teardown and waits. An interactive run gets a confirmation and proceeds; a headless run with no human present simply ends the turn with the plan as output and runs nothing destructive.

## Step 2: delete `.grove/` in one focused commit

On confirmation, the session removes the scaffolding:

```
$ git rm -r .grove
$ git commit -m "chore: remove grove scaffolding"
$ git log --oneline -1
b6a5f4e chore: remove grove scaffolding
```

One commit, scoped to `.grove/` only. The point is to keep whatever branch this commit will eventually land on free of any grove's local state. The history of completed groves lives in git's commit graph, not in retained directories.

```
$ tree -L 1
.
├── Cargo.toml
├── README.md
├── docs
├── src
└── tests
```

No `.grove/` left. The working tree still exists on disk (the session is standing in it), still on the `add-rate-limiting` branch, and it now looks like any other checkout of `acme/orders-api`.

## Step 3: signal the loop to stop

As its very last action, the session runs:

```
$ grove-llm complete --done
```

This is what distinguishes a clean finish from a crash or a Ctrl-C: the per-task signal (`grove-llm complete`, no flag) tells the self-driving loop to *relaunch* into the next task; `--done` tells it to *stop* instead. It touches nothing about the working tree — it writes only the loop's own signal file and reads `$GROVE_CLAUDE_PID` from the environment `grove do` set up — so it can run from anywhere, but it must come **last**: like the per-task signal it ends the session after a short grace period, so anything after it would be cut short.

## Resume is read from state, never a marker file

grove keeps no finish-progress file (constraint 1, *artifacts not state*). If a finish is interrupted — a closed terminal between promotion and the confirmation, say — you resume by running `grove do` again from inside the same working tree, and the session works out where it stopped from inspectable git and filesystem state. The two entry conditions are distinguishable by `grove-llm pick` itself: exit 0 + empty stdout + *"no live leaves; this grove is done"* means `.grove/` is still present (fresh finish, run from step 1); a non-zero exit + *"grove root not found"* means `.grove/` is already gone — the finish cycle already completed, so the session reports *"already finished"* and stops.

## Multi-harness stamp cleanup

If the repo runs multiple harnesses, a stamp may exist at `~/code/acme/orders-api/.grove-stamps/add-rate-limiting` — written at bootstrap time, in the **main repo**, to bind this grove to its harness (see [`start.md`](start.md)). Finishing neither reads nor removes it: the stamp has no bearing on the finish cycle. If you're also tearing down the working tree yourself once you're done integrating, you may want to remove the stamp alongside it — otherwise a future grove reusing the same name would inherit the old binding:

```
$ rm ~/code/acme/orders-api/.grove-stamps/add-rate-limiting
```

Single-harness repos never had a stamp and have nothing to clean up.

## After finishing

```
$ git log --oneline -3
b6a5f4e chore: remove grove scaffolding
4f3e2d1 docs: record rate-limiting design intent in docs/specs/rate-limiting.md
a9b8c7d chore(grove): retire implement-k4
```

`grove-llm complete --done` added no commit of its own — it only signalled the loop, so `chore: remove grove scaffolding` is still the tip. The grove is gone — no `.grove/`, no signal file, no state left behind anywhere grove touched. What's left is an ordinary git branch, in an ordinary working tree, ready for you to integrate however your team does that: a plain `git merge`, a PR, a squash — grove has no opinion. Once it's landed, tear down the working tree and branch with your own tooling (`git worktree remove` and `git branch -d`, or a dedicated manager's own cleanup command). Its durable output — code, ADRs, design docs, glossary entries — lives in the repo wherever you land the branch, where every contributor can find it without knowing this grove ever existed.

## Codex harness

Identical flow. Finishing is an in-session step driven by the same methodology, so there is no per-harness finish prompt to differ — every session reads its loop from the one binary-provisioned global skill (`~/.claude/skills/grove/SKILL.md`) whichever harness runs. The trigger (empty `grove-llm pick`), the three steps, the single confirmation gate, and the post-finish state of the working tree are all the same.
