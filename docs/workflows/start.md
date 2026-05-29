# `grove do` (new grove) — walkthrough

Begin a new workstream. `grove do` is the sole lifecycle entry verb; this page shows its new-grove path (the former `grove start`). Running `grove do` against a grove that already exists *continues* it instead — see [`multi-step.md`](multi-step.md). By the end of this walkthrough, `acme/orders-api` has a fresh worktree at `.grove-worktrees/add-rate-limiting/` on a new `add-rate-limiting` branch, and a harness session is running inside it on grove's start prompt.

> This page is about driving the **grove CLI**. For *what grove is and why*, see [`../grove.md`](../grove.md); for the methodology agents read at runtime, see [`../../content/SKILL.md`](../../content/SKILL.md). For the full flag surface, run `grove do --help`.

## Starting state

We pick up where [`install.md`](install.md) left off: the repo is on its default branch with grove materialised under `.claude/skills/grove/`, and no groves exist yet.

```
$ cd acme/orders-api
$ git status
On branch main
nothing to commit, working tree clean

$ ls .grove-worktrees 2>/dev/null || echo "(none)"
(none)
```

We want to add request rate limiting to the API — a project we expect to span multiple sessions, with at least one planning step before any code is written. That's a grove.

## The default start

```
$ grove do add-rate-limiting
Preparing worktree (new branch 'add-rate-limiting')
HEAD is now at 1a2b3c4 Install grove v2.0.0
```

Two things happened on disk, then a third in your terminal.

First, grove created a git worktree pinned to a new branch:

```
$ tree -L 2 .grove-worktrees
.grove-worktrees
└── add-rate-limiting
    ├── ... (full working copy of the repo)
    └── (no .grove/ yet — the bootstrap session writes that)

$ git branch -a | grep add-rate-limiting
  add-rate-limiting
+ add-rate-limiting          (worktree: .grove-worktrees/add-rate-limiting)
```

The branch was cut from `origin/HEAD` (or `main` as a fallback when no origin is configured — see `--start-point` below to override). The worktree sits at the canonical grove location, `<repo>/.grove-worktrees/<name>/`; every session of this grove will run in the same worktree continuously.

Second, grove exec'd the harness inside that worktree with grove's start prompt and a pre-set session name. For Claude Code, that becomes:

```
claude -n "orders-api: add-rate-limiting grove" <prompt>
```

— where `<prompt>` is rendered from `.claude/skills/grove/prompts/start.md` with `{{NAME}}` substituted. The session name follows the `<repo>: <name> grove` convention so groves show up identifiably in harness UIs.

## The bootstrap session

The harness session that just opened is the **bootstrap session** — the first session on the new branch. It is part of the grove, but it is *not* what `grove do` itself did; the CLI's job ended at `exec_harness`. Following the start prompt, the session will:

1. Run a grilling pass on your goal, sharpening any new terminology into `CONTEXT.md` inline.
2. Propose a root [`BRIEF.md`](../../content/BRIEF-FORMAT.md) for the grove and a small initial decomposition — usually one or two leaves, no more.
3. Commit that brief (and any `CONTEXT.md` edits) as the first commit on the `add-rate-limiting` branch.

After the bootstrap session commits, `.grove/` looks something like:

```
$ tree -L 1 .grove-worktrees/add-rate-limiting/.grove
.grove-worktrees/add-rate-limiting/.grove
├── 010-design-token-bucket.md
├── 020-implement.md
└── BRIEF.md
```

That brief and its first leaves are produced by the bootstrap session — `grove do` itself does not write a `BRIEF.md`. Keep the bootstrap planning small; the decomposition will grow as later planning tasks discover what is actually there.

## Variation: branching from somewhere other than origin's HEAD

Pass `--start-point <ref>` when the grove should branch from a tag, a release branch, or a colleague's WIP — anywhere other than `origin/HEAD`:

```
$ grove do add-rate-limiting --start-point release/2026.04
Preparing worktree (new branch 'add-rate-limiting')
HEAD is now at 9c8d7e6 cut 2026.04
```

The flag is forwarded straight to `git worktree add ... -b add-rate-limiting <ref>`; any ref that `git` accepts works (branch, tag, sha, `origin/<name>`).

## Variation: `--no-launch`

When you want the worktree and branch but not the interactive session — for inspection, scripting, or because you intend to drive the harness by hand — pass `--no-launch`:

```
$ grove do add-rate-limiting --no-launch
Preparing worktree (new branch 'add-rate-limiting')
HEAD is now at 1a2b3c4 Install grove v2.0.0
grove: worktree ready at /Users/you/code/acme/orders-api/.grove-worktrees/add-rate-limiting (no-launch)
```

The worktree and branch exist exactly as in the default flow; `.grove/` is still empty until a bootstrap session runs. You can `cd` into the worktree and start the harness yourself with a free-form prompt, or come back later with `grove do add-rate-limiting` (which continues the now-existing grove).

## Multi-harness repos

If `acme/orders-api` has both `.claude/` and `.codex/` directories — i.e. grove is materialised under each — `grove do` cannot guess which harness this grove should be bound to. Pass `--harness` once at start time:

```
$ grove do add-rate-limiting --harness claude
```

The CLI records the binding by writing a one-line stamp at `.grove-stamps/add-rate-limiting`:

```
$ cat .grove-stamps/add-rate-limiting
claude
```

Later verbs (`grove do`, `grove takeover`, `grove retire`) read that stamp and run the same harness, so a single grove never spans harnesses mid-flight. In single-harness repos the stamp is not written — there's nothing to disambiguate.

## Codex harness

The flow is identical with `--harness codex`; only the exec'd binary changes. The worktree path (`.grove-worktrees/add-rate-limiting/`), branch name (`add-rate-limiting`), and session-name convention (`orders-api: add-rate-limiting grove`) are the same. The bootstrap session reads its prompt from `.codex/skills/grove/prompts/start.md` rather than `.claude/skills/grove/prompts/start.md`; the prompt body is identical.
