# `grove do` (new grove) — walkthrough

Begin a new workstream. `grove do` is the sole lifecycle entry verb; this page shows its new-grove path. Running `grove do` against a working tree that already has a `.grove/` tree *continues* it instead — see [`multi-step.md`](multi-step.md). Opening a new grove is two steps, and only the second one is grove's: you create a git working tree yourself, then run argument-less `grove do` from inside it. By the end of this walkthrough, `acme/orders-api` has a fresh linked worktree at `~/code/acme/add-rate-limiting` on a new `add-rate-limiting` branch, and a harness session is running inside it on grove's start prompt.

> This page is about driving the **grove CLI**. For *what grove is and why*, see [`../grove.md`](../grove.md); for the methodology agents read at runtime, see [`../../content/SKILL.md`](../../content/SKILL.md). For the full flag surface, run `grove do --help`.

## Starting state

The main repo is on its default branch, and `grove` is installed via Homebrew (the binary provisions its embedded skill to the global `~/.claude/skills/grove/` on the first `grove do` — no per-repo install step).

```
$ cd ~/code/acme/orders-api
$ git status
On branch main
nothing to commit, working tree clean

$ git worktree list
~/code/acme/orders-api  1a2b3c4 [main]
```

We want to add request rate limiting to the API — a project we expect to span multiple sessions, with at least one planning step before any code is written. That's a grove.

## Create your own working tree

grove never creates, integrates, or tears down git topology (user-owned-worktrees) — that part is plain git, and it comes first. A linked worktree is the common choice, since it keeps the main checkout free for other work:

```
$ git worktree add ../add-rate-limiting -b add-rate-limiting
Preparing worktree (new branch 'add-rate-limiting')
HEAD is now at 1a2b3c4 Add idempotency keys to orders

$ git worktree list
~/code/acme/orders-api         1a2b3c4 [main]
~/code/acme/add-rate-limiting  1a2b3c4 [add-rate-limiting]
```

The new working tree can live anywhere — here, a sibling of `orders-api` under `~/code/acme/`. Its directory basename, `add-rate-limiting`, *is* the grove's name: grove derives it from `git rev-parse --show-toplevel`, never from the branch, so name the directory whatever you want the grove called. A plain `git init` or `git clone`, or a dedicated tool such as [worktrunk](https://github.com/max-sixty/worktrunk), works just as well — grove's only precondition is *a git working tree*.

## `grove do` opens the bootstrap session

```
$ cd ../add-rate-limiting
$ grove do
```

Two things happen, in order. First, the binary provisions (or refreshes) the global skill at `~/.claude/skills/grove/` from its embedded methodology — a cheap, idempotent no-op once it's current. Second, since this working tree has no `.grove/` yet, `grove do` exec's the harness with grove's start prompt and a pre-set session name:

```
claude -n "orders-api: add-rate-limiting grove" <prompt>
```

— where `<prompt>` is read from the binary-provisioned global skill at `~/.claude/skills/grove/prompts/start.md`, and `orders-api` (the `<repo-basename>` half of the session name) comes from the **main repo** — `git rev-parse --git-common-dir`'s parent — not from this working tree's own path. The session name follows the `<repo-basename>: <name> grove` convention so groves show up identifiably in harness UIs.

## The bootstrap session

The harness session that just opened is the **bootstrap session** — the first session in this working tree, and the first iteration of the **self-driving loop** that `grove do` started. Following the start prompt, the session will:

1. Run `grove-llm root-init` to scaffold `.grove/`: a root [`BRIEF.md`](../../content/BRIEF-FORMAT.md) stub and a first leaf, `01-plan-k1.md`, marked **Kind: planning** — a working-tree change, no commit yet.
2. Enter the loop at that planning leaf: run a grilling pass on your goal, sharpening any new terminology into `CONTEXT.md` inline, and grow the tree with `grove-llm leaf-add` — usually to a small initial decomposition, one or two leaves, no more.
3. Commit everything as one commit on the `add-rate-limiting` branch — the `root-init` scaffold, the grown leaves, and any `CONTEXT.md` edits — then retire `01-plan-k1.md` in place (it becomes `01-DONE-plan-k1.md` in the same commit, since nothing was committed before it to separate the retirement into its own commit). Finally it fires its completion signal so the loop relaunches into the first live leaf.

After the bootstrap session commits, `.grove/` looks something like:

```
$ tree -L 1 .grove
.grove
├── 01-DONE-plan-k1.md
├── 02-design-token-bucket-k2.md
├── 03-implement-k3.md
└── BRIEF.md
```

Each leaf is `NN-<slug>-k<key>.md`: a 2-digit per-level position (`01`, `02`, `03`), a slug, and a permanent key (`-k1`, `-k2`, `-k3`) that never changes once assigned. The planning leaf `01-plan-k1.md` came from `root-init`; `02-design-token-bucket-k2.md` and `03-implement-k3.md` are what the grilling decided to grow. Keep the bootstrap planning small; the decomposition will grow as later planning tasks discover what is actually there.

## Variation: `--no-launch`

When you want to confirm the CLI is ready to drive this working tree but skip the interactive session — for scripting, or because you intend to drive the harness by hand — pass `--no-launch`:

```
$ grove do --no-launch
grove: ready in /Users/you/code/acme/add-rate-limiting (no-launch)
```

`.grove/` is untouched: on a brand-new working tree there's nothing to scaffold yet, since `root-init` runs inside the bootstrap session, not the CLI itself. You can `cd` into the working tree and start the harness yourself with a free-form prompt (or run `grove-llm root-init` by hand first), or come back later with plain `grove do` — which opens the bootstrap session on a rootless tree, or continues an existing one.

## Multi-harness repos

If `acme/orders-api` has both `.claude/` and `.codex/` directories — i.e. both harnesses have been used in this repo — `grove do` cannot guess which harness this grove should be bound to. Pass `--harness` once at start time:

```
$ grove do --harness claude
```

The CLI records the binding by writing a one-line stamp in the **main repo**, keyed by the grove's name:

```
$ cat ~/code/acme/orders-api/.grove-stamps/add-rate-limiting
claude
```

Note the stamp lives under `orders-api` (the main repo), not under `add-rate-limiting` (this grove's own working tree) — later verbs resolve the main repo the same way (`git rev-parse --git-common-dir`'s parent) regardless of which working tree they run from, so the binding is found however the grove is addressed. Later verbs (`grove do`, `grove retire`) read that stamp and run the same harness, so a single grove never spans harnesses mid-flight. In single-harness repos relying on auto-detection, the stamp is not written — there's nothing to disambiguate. Passing `--harness` explicitly always writes the stamp, even in a single-harness repo, so a deliberate binding survives the next plain `grove do`.

## Codex harness

The flow is identical with `--harness codex`; only the exec'd binary changes. The working tree, its branch, and the session-name convention (`orders-api: add-rate-limiting grove`) are the same. The prompt is read from the same binary-provisioned global skill (`~/.claude/skills/grove/prompts/start.md`) whichever harness runs — the methodology is provisioned once, globally, not per harness.
