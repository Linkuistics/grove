# Multi-step grove — walkthrough

The inner loop. By the end of this walkthrough, `acme/orders-api`'s `add-rate-limiting` grove has run through four sessions — an `impl` leaf, a `planning` leaf that grew the tree, an `impl` leaf inside the new subtree, and the implementation leaf — all driven automatically by a **single `grove do`**, plus one out-of-band `grove retire` to promote a completed subtree's brief upward.

> This page is about driving the **grove CLI** through the inner loop. For *what the loop is and why*, see [`../../content/SKILL.md`](../../content/SKILL.md); for *how a single session conducts itself*, follow the SKILL file. This walkthrough shows the **CLI cadence and the on-disk evolution** — not session-internal UX.

## The example sequence

The sequence below is one plausible shape, not a template. Real groves vary: a grove might have no planning leaves after the bootstrap, or it might have nested planning two deep. The point is *how each kind of session moves the tree*, not the specific order shown.

## Starting state

We pick up where [`start.md`](start.md) left off. The bootstrap session retired its `root-init` requirements leaf and committed a root `BRIEF.md` plus — for this walkthrough — three further leaves (imagine the grilling had decided on a quick spike ahead of the design):

```
$ cd ~/code/acme/add-rate-limiting
$ tree .grove
.grove
├── 01-DONE-plan-k1.md
├── 02-spike-token-bucket-k2.md
├── 03-design-token-bucket-k3.md
├── 04-implement-k4.md
└── BRIEF.md

$ git log --oneline
2c4d5e6 Bootstrap add-rate-limiting: root brief + initial leaves
1a2b3c4 Add idempotency keys to orders
```

`01-DONE-plan-k1.md` is the `root-init` requirements leaf, already retired by the bootstrap session. `02-spike-token-bucket-k2.md` is an **impl** leaf — a quick experimental spike. `03-design-token-bucket-k3.md` is a **planning** leaf: the bootstrap session declared the design open enough that the move is to decompose, not to code. `04-implement-k4.md` is an **impl** leaf placeholder for the actual implementation, which the design step will almost certainly decompose further. Each name is `NN-<slug>-k<key>`: the `NN` is the per-level position (the sort order), and the trailing `-k<key>` is the permanent identity that never changes — not under renumber, not under a slug edit.

## One `grove do` drives the whole loop

```
$ grove do
```

That single, argument-less command drives the **self-driving loop** (self-driving-loop), not one task. It launches a fresh, clean-context session on the first live leaf; when that session finishes its task and fires `grove-llm complete` as its last step, the loop relaunches a new session on the *next* live leaf — and so on until `grove-llm pick` comes up empty. You run `grove do` once, from inside this working tree, and watch the tree drain.

The CLI's role is small: it provisions the global skill, exec's a fresh harness session pre-named `orders-api: add-rate-limiting grove` with the continue prompt, and relaunches on each completion signal. The methodology is in the prompt, not the binary. A session that exits *without* signalling — your `/exit`, a Ctrl-C, a crash — **stops** the loop instead of relaunching; re-running `grove do` from inside this same working tree resumes from wherever the tree stands, because the loop keeps no state of its own and re-derives its position from `pick` every iteration (restart ≡ continuation).

The sections below trace the loop's iterations one at a time, showing how each kind of session moves the tree on disk.

## Iteration 1: an `impl` leaf

The loop's first session picks the first live leaf depth-first — `02-spike-token-bucket-k2.md` — reads `CONTEXT.md`, the root `BRIEF.md`, and the leaf itself, and gets to work. It produces a small experimental implementation under `src/`, commits it as one focused commit, retires the leaf **in place** by adding a `DONE` infix, and fires its completion signal:

```
$ tree .grove
.grove
├── 01-DONE-plan-k1.md
├── 02-DONE-spike-token-bucket-k2.md
├── 03-design-token-bucket-k3.md
├── 04-implement-k4.md
└── BRIEF.md

$ git log --oneline -3
9f8e7d6 chore(grove): retire spike-token-bucket-k2
6a5b4c3 feat(rate-limit): spike token-bucket counter
2c4d5e6 Bootstrap add-rate-limiting: root brief + initial leaves
```

Retirement is a rename **in place** — `02-spike-token-bucket-k2.md` → `02-DONE-spike-token-bucket-k2.md` — not a move into a separate folder. The leaf keeps its position (`02`) and key (`-k2`); the `DONE` infix right after the position is what `pick` skips. So `ls .grove` always shows the *complete* state, done leaves included, with zero file reads.

Two commits per session is grove's usual rhythm — the deliverable (`feat: spike …`) and the housekeeping (`chore(grove): retire …`) are separable concerns, so they get separable commits. Note the naming convention in the housekeeping subject: a work item is named by its stable `<slug>-k<key>` handle (`spike-token-bucket-k2`), never by its mutable position or path (task-tree-scheme §5).

## Iteration 2: a planning leaf grows the tree

The loop relaunches. The first live leaf is now `03-design-token-bucket-k3.md`, marked **Kind: planning**. The continue prompt is the same; the leaf's kind tells the session to cut the settled design into vertical slices rather than produce an artifact of its own — `planning` does not interrogate. The session sharpens any new terminology into `CONTEXT.md` inline, finds the design is too big for one focused session, and **grows the tree**: the leaf is *decomposed* into a node — its file becomes a **directory** holding its own `BRIEF.md` and one or more ordered child leaves.

```
$ tree .grove
.grove
├── 01-DONE-plan-k1.md
├── 02-DONE-spike-token-bucket-k2.md
├── 03-design-token-bucket-k3
│   ├── 01-record-policy-adr-k5.md
│   └── BRIEF.md
├── 04-implement-k4.md
└── BRIEF.md

$ git log --oneline -1
4d3c2b1 plan(rate-limit): decompose design-token-bucket-k3 into one ADR leaf
```

Note what *didn't* happen: the planning session did not retire anything. The leaf `03-design-token-bucket-k3.md` became the directory `03-design-token-bucket-k3/` **in place** — same position, same key. The leaf-to-node promotion is a `git mv` (file → directory), but semantically it is *replacement*, not retirement: the completed planning work is the new node's `BRIEF.md`, and the leaf existed only to license the planning effort. The node keeps the planning leaf's key (`-k3`); its first child gets a fresh key (`-k5` — the next free key in the tree, since `-k1` through `-k4` are already taken).

## Iteration 3: a leaf inside the new subtree

The loop relaunches, and the depth-first pick descends into the new node, landing on `03-design-token-bucket-k3/01-record-policy-adr-k5.md` — an `impl` leaf. The session reads the ancestor briefs (root `BRIEF.md` → `03-design-token-bucket-k3/BRIEF.md`) plus the leaf, authors `docs/adr/token-bucket-policy.md`, commits it, and retires the leaf in place:

```
$ tree .grove
.grove
├── 01-DONE-plan-k1.md
├── 02-DONE-spike-token-bucket-k2.md
├── 03-design-token-bucket-k3
│   ├── 01-DONE-record-policy-adr-k5.md
│   └── BRIEF.md
├── 04-implement-k4.md
└── BRIEF.md

$ git log --oneline -3
b2a1d9c chore(grove): retire record-policy-adr-k5
3e2f1a0 docs(adr): 0002 token-bucket policy
4d3c2b1 plan(rate-limit): decompose design-token-bucket-k3 into one ADR leaf
```

Two things to notice. First, the retired leaf stays exactly where it lived — `03-design-token-bucket-k3/01-DONE-record-policy-adr-k5.md` — marked with the `DONE` infix, so the tree shows *where* each completed leaf belonged without any parallel shadow tree. Second, `03-design-token-bucket-k3/` now has no live leaf: its only child is done. The node is **implicitly** done — a brief is context, not a task, so a node is never marked `DONE`; its done-ness *is* the absence of any live leaf in its subtree.

## A brief-carrying node's retirement is asked, not assumed

When `03-design-token-bucket-k3/`'s last live leaf retired, the session's *judge retirement* step walked the parent chain, noticed the node had no live leaf left, and **asked the user** before treating it as done — a confirmation gives them a moment to add a follow-up leaf if the node is not actually finished. In this walkthrough the user said *not yet*, so the session fired its completion signal and the loop moved on to `04-implement-k4.md`. Node-level retirement is deliberate, not automatic — grove guides, it does not gate.

`03-design-token-bucket-k3/` is a **decomposition** node: `leaf-decompose` gave it a `BRIEF.md`, and that charter is exactly what the confirmation exists to promote. The *other* node species — a **chain node**, the `<stem>-chain/` or `<stem>-pair/` directory `leaf-add-chain` / `leaf-add-pair` writes — carries no charter by rule, so it closes **silently**: there is nothing to promote and nothing to decide. The discriminator is the file's presence, not the name.

Retiring a node moves nothing on disk (there is no `done/` to move into): its leaves are already marked done in place, and its `BRIEF.md` stays where it is. What retirement *does* is **promote** anything from the node's brief that future siblings should still see — up to the parent brief, an ADR, or the glossary — so it stays in the brief chain after the node goes quiet. To do that out of band — or whenever a prior session forgot to ask — the user runs `grove retire`, in-worktree:

```
$ grove retire 03-design-token-bucket-k3
```

`grove retire` launches a focused harness session whose prompt does exactly that: promote anything still relevant from the node's `BRIEF.md` upward (to the parent brief, an ADR, or the glossary), in one focused commit. The node directory and its `DONE` leaf stay put.

```
$ git log --oneline -1
e1d0c9b chore(grove): retire design-token-bucket-k3 — promote design intent into root brief
```

The promoted text from the node's brief is now part of an ancestor — `04-implement-k4`'s session can read it from the root `BRIEF.md` without descending into a quiet subtree. (Inside the regular loop, the same promotion runs implicitly when the user confirms the asked retirement, and the same cascade continues up the parent chain.)

If the node's `BRIEF.md` carried nothing worth promoting (the work was a tactical step whose conclusions live entirely in the code or ADR it produced), the retire session simply records that nothing needed promoting. The discipline is to *consider* promotion, not to always perform it.

## Iteration 4: the implementation leaf

The loop's next session picks `04-implement-k4.md`, wires the token-bucket middleware, commits, and retires the leaf in place (`04-DONE-implement-k4.md`). With that, every leaf is done — `grove-llm pick` comes up empty, and the loop switches from "do the next task" to proposing the **complete finish cycle**. That hand-off is the subject of [`finish.md`](finish.md).

## Codex harness

The CLI surface is identical: `grove do`, `grove retire <node-path>`, both run in-worktree. The harness exec'd is whichever was chosen at bootstrap time (recorded in `.grove-stamps/add-rate-limiting` in the main repo for multi-harness repos, auto-detected otherwise); the methodology and prompts come from the same binary-provisioned global skill (`~/.claude/skills/grove/`) whichever harness runs. `git log` and `tree` on `.grove/` look exactly the same — the on-disk evolution is the same shape for either harness.
