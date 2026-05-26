# Multi-step grove — walkthrough

The inner loop. By the end of this walkthrough, `acme/orders-api`'s `add-rate-limiting` grove has run through four sessions: a work leaf, a planning leaf that grew the tree, a work leaf inside the new subtree, and a `grove retire` invocation that collapsed the completed subtree into `.grove/done/`.

> This page is about driving the **grove CLI** through the inner loop. For *what the loop is and why*, see [`../../content/SKILL.md`](../../content/SKILL.md); for *how a single session conducts itself*, follow the SKILL file. This walkthrough shows the **CLI cadence and the on-disk evolution** — not session-internal UX.

## The example sequence

The sequence below is one plausible shape, not a template. Real groves vary: a grove might have no planning leaves after the bootstrap, or it might have nested planning two deep. The point is *how each kind of session moves the tree*, not the specific order shown.

## Starting state

We pick up where [`start.md`](start.md) left off. The bootstrap session has committed a root `BRIEF.md` and two initial leaves:

```
$ cd acme/orders-api/.grove-worktrees/add-rate-limiting
$ tree .grove
.grove
├── 010-design-token-bucket.md
├── 020-implement.md
└── BRIEF.md

$ git log --oneline
2c4d5e6 Bootstrap add-rate-limiting: root brief + initial leaves
1a2b3c4 Install grove v2.0.0
```

`010-design-token-bucket.md` is a **planning** leaf — the bootstrap session declared the design open enough that the next move is to grill, not to code. `020-implement.md` is a **work** leaf placeholder for the actual implementation, which the design step will almost certainly decompose further.

For this walkthrough we'll insert a deliberately small work leaf ahead of the planning task — a quick spike. Imagine the bootstrap session had emitted `010-spike-token-bucket.md` (work), `020-design-token-bucket.md` (planning), `030-implement.md` (work). The shape doesn't matter; the cadence does.

## Session 1: resume on a work leaf

```
$ grove continue add-rate-limiting
```

The harness opens in the worktree with grove's continue prompt. It picks the first live leaf depth-first — `010-spike-token-bucket.md` — reads `CONTEXT.md`, the root `BRIEF.md`, and the leaf itself, and gets to work. The session produces a small experimental implementation under `src/`, commits it as one focused commit, and retires the leaf by moving it into `.grove/done/`.

### `grove continue` is just a launcher — three equivalent ways to drive a task

The CLI does one thing: exec a fresh harness session in the worktree, pre-named `<repo>: <name> grove`, with `prompts/continue.md` (a one-line "Do the next task in .grove/…") as the first prompt. The methodology is in the prompt, not the binary. That means there are three equivalent ways to drive the next task once a grove is running:

- **`grove continue add-rate-limiting`** — what we just ran. Always a fresh session, always pre-named, always fed the continue prompt. The canonical move.
- **Keep going in an open session.** If a session is already running in the worktree (often the one `grove start` opened, or one from a prior `grove continue` you never closed), the next task is just the same continue prompt again — paste it, or say "do the next task in `.grove/`" in your own words. No new exec is needed; the harness history carries forward and the session keeps its name. The trade-off is context bleed: the prior task's working memory is still in the session, which is occasionally useful and occasionally noise.
- **`/clear` then re-prompt.** In an existing harness session, `/clear` (Claude Code) wipes the context window to give you the fresh-session benefit without spawning a new one. Paste the continue prompt afterward. *Gotcha:* `/clear` also clears the session name — the `<repo>: <name> grove` label is gone until you re-set it with `/rename`. Sessions still show up identifiably in `grove continue` invocations, just not in the one you `/clear`-ed.

The rest of this walkthrough writes `grove continue add-rate-limiting` for clarity, but read it as "run the continue prompt by whichever of the three routes suits you."

```
$ tree .grove
.grove
├── 020-design-token-bucket.md
├── 030-implement.md
├── BRIEF.md
└── done
    └── 010-spike-token-bucket.md

$ git log --oneline -3
9f8e7d6 chore(grove): retire 010-spike-token-bucket
6a5b4c3 feat(rate-limit): spike token-bucket counter
2c4d5e6 Bootstrap add-rate-limiting: root brief + initial leaves
```

Two commits per session is grove's usual rhythm when retirement is a clean rename — the deliverable (`feat: spike ...`) and the housekeeping (`chore(grove): retire ...`) are separable concerns, so they get separable commits.

## Session 2: resume on a planning leaf

```
$ grove continue add-rate-limiting
```

This time the first live leaf is `020-design-token-bucket.md`, marked **Kind: planning**. The continue prompt is the same; the leaf's kind tells the session to open with a grilling pass before doing anything else. Through the grilling, the session sharpens any new terminology into `CONTEXT.md` inline, decides the design is too big for one focused session, and **grows the tree**: the leaf is replaced by a node directory containing its own `BRIEF.md` and one or more ordered child leaves.

```
$ tree .grove
.grove
├── 020-design-token-bucket
│   ├── 010-record-policy-adr.md
│   └── BRIEF.md
├── 030-implement.md
├── BRIEF.md
└── done
    └── 010-spike-token-bucket.md

$ git log --oneline -1
4d3c2b1 plan(rate-limit): decompose token-bucket design into one ADR leaf
```

Note what *didn't* happen: the planning session did not retire anything. The original `020-design-token-bucket.md` file did not move to `done/`; it became a directory in place. The leaf-to-node promotion is `git mv` (path-wise) but semantically it is *replacement*, not retirement. The completed planning work is the new node's `BRIEF.md`; the leaf existed only to license the planning effort.

## Session 3: resume on a leaf inside the new subtree

```
$ grove continue add-rate-limiting
```

Depth-first pick descends into the new node and lands on `020-design-token-bucket/010-record-policy-adr.md` — a work leaf. The session reads the ancestor briefs (root → `020-design-token-bucket/BRIEF.md`) plus the leaf, authors `docs/adr/0002-token-bucket-policy.md`, commits it, and retires the leaf:

```
$ tree .grove
.grove
├── 020-design-token-bucket
│   └── BRIEF.md
├── 030-implement.md
├── BRIEF.md
└── done
    ├── 010-spike-token-bucket.md
    └── 020-design-token-bucket
        └── 010-record-policy-adr.md

$ git log --oneline -3
b2a1d9c chore(grove): retire 020-design-token-bucket/010-record-policy-adr
3e2f1a0 docs(adr): 0002 token-bucket policy
4d3c2b1 plan(rate-limit): decompose token-bucket design into one ADR leaf
```

Two things to notice. First, `.grove/done/` mirrors the live tree's shape — retired leaves keep their `<node>/<leaf>` relative paths, so a future reader can see *where* each completed leaf belonged. Second, `020-design-token-bucket/` now contains only its `BRIEF.md` — the node's last live leaf is gone. The grove is at the cusp of node-level retirement.

## A node-level retirement is a manual move

When `020-design-token-bucket/`'s last live leaf retired, the *judge retirement* step at the end of session 3 noticed but did not act. Node-level retirement is a separate, deliberate move that the user issues — grove guides, it does not gate:

```
$ grove retire add-rate-limiting/020-design-token-bucket
```

`grove retire` launches a focused harness session in the worktree with a prompt that does exactly two things: promote anything still relevant from the node's `BRIEF.md` upward (to the parent brief, an ADR, or the glossary), then `mv` the subtree into `done/` preserving its relative path. One focused commit.

```
$ tree .grove
.grove
├── 030-implement.md
├── BRIEF.md
└── done
    ├── 010-spike-token-bucket.md
    └── 020-design-token-bucket
        ├── BRIEF.md
        └── 010-record-policy-adr.md

$ git log --oneline -1
e1d0c9b chore(grove): retire 020-design-token-bucket — promote design intent into root BRIEF
```

The promoted text from the node's brief is now part of an ancestor — `030-implement.md`'s next session can read it from the root `BRIEF.md` without descending into a dead subtree.

If the node's `BRIEF.md` carried nothing worth promoting (the work was a tactical step whose conclusions live entirely in the code or ADR it produced), the retire session simply moves the subtree across. The discipline is to *consider* promotion, not to always perform it.

## Orienting on a grove without picking a task

Late in a grove's life — or when handing it off — it's often useful to take stock without committing to the next step. `grove takeover <name>` opens a session whose prompt explicitly says *don't pick a task*: read `CONTEXT.md`, the root `BRIEF.md`, and a recent slice of `git log -- .grove/`, then report what's done, what's open, what the next task would be, and any open questions in the briefs. Use it when picking up a grove you didn't start, or when you want a status read before deciding whether to continue, retire, or finish.

```
$ grove takeover add-rate-limiting
```

It produces no commits. If the report convinces you to keep going, run `grove continue add-rate-limiting` afterward.

## Codex harness

The CLI surface is identical: `grove continue add-rate-limiting`, `grove retire add-rate-limiting/<node-path>`, `grove takeover add-rate-limiting`. The harness exec'd is whichever was chosen at `grove start` time (recorded in `.grove-stamps/<name>` for multi-harness repos, auto-detected otherwise); the prompts are read from `.codex/skills/grove/prompts/` instead of `.claude/skills/grove/prompts/` but their content is identical. `git log` and `tree` on `.grove/` look exactly the same — the on-disk evolution is the same shape for either harness.
