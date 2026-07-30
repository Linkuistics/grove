# Multi-step grove — walkthrough

The inner loop. By the end of this walkthrough, `acme/orders-api`'s `add-rate-limiting` grove has run through five sessions — an `impl` leaf, a `planning` leaf that grew the tree, an `impl` leaf inside the new subtree, a follow-up leaf the node's own close-time check demanded, and the implementation leaf — all driven automatically by a **single `grove do`**, with the manual `grove retire` shown as the out-of-band way to promote a completed subtree's brief upward.

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

Two things to notice. First, the retired leaf stays exactly where it lived — `03-design-token-bucket-k3/01-DONE-record-policy-adr-k5.md` — marked with the `DONE` infix, so the tree shows *where* each completed leaf belonged without any parallel shadow tree. Second, `03-design-token-bucket-k3/` now has no live leaf: its only child is done. That makes the node **implicitly** done — a brief is context, not a task, so a node is never marked `DONE`; its done-ness *is* the absence of any live leaf in its subtree. Being an inference rather than a stored state, it is also cheap to get wrong and cheap to reverse, which is what the next section turns on.

## A brief-carrying node's close is checked, not asked

When `03-design-token-bucket-k3/`'s last live leaf retired, the same session walked the parent chain, noticed the node had no live leaf left, and **asked the user nothing**. A node is never marked, so a close writes nothing at all: whatever a human answered, the tree would be byte-identical afterwards, and a node closed in error is reopened by one `leaf-add` with nothing to undo ([`adr/confirmation-boundary.md`](../adr/confirmation-boundary.md)). What the session does instead is **check** the node's brief `Done when` against what the subtree actually delivered.

Here the check fails, and it fails *nameably*: the brief promised the policy **and** the knobs that expose it, and the ADR records only the policy. A failed check names missing **work**, not a decision to take — so the session cuts the leaf rather than raising a question:

```
$ grove-llm leaf-add 03-design-token-bucket-k3 document-policy-knobs
/…/.grove/03-design-token-bucket-k3/02-document-policy-knobs-k6.md

$ tree .grove
.grove
├── 01-DONE-plan-k1.md
├── 02-DONE-spike-token-bucket-k2.md
├── 03-design-token-bucket-k3
│   ├── 01-DONE-record-policy-adr-k5.md
│   ├── 02-document-policy-knobs-k6.md
│   └── BRIEF.md
├── 04-implement-k4.md
└── BRIEF.md

$ git log --oneline -1
f5e4d3c chore(grove): design-token-bucket-k3 stays open — add document-policy-knobs-k6
```

The node is live again, the follow-up leaf lands at the next free position with a fresh key (`-k6`), and nothing had to be un-marked to get there — that is the whole reason the close needs no gate. It gets its own housekeeping commit, separate from the retirement that preceded it: *this leaf is done* and *this node is not* are two different facts. The human reviews the decision in that diff, after the fact, instead of being interrupted before it. The only case that stops is a check that fails *un*-nameably — a residue that is a scope judgement rather than work. That is an escalation, discretionary and triggered by evidence the session actually met, not a routine gate.

`03-design-token-bucket-k3/` is a **decomposition** node: `leaf-decompose` gave it a `BRIEF.md`, and that charter is what supplies both the `Done when` to check and the text to promote. The *other* node species — a **chain node**, the `<stem>-chain/` or `<stem>-pair/` directory `leaf-add-chain` / `leaf-add-pair` writes — carries no charter by rule, so its close has **nothing to do**: no rollup to check and nothing to promote. The discriminator is the file's presence, not the name.

## Iteration 4: the follow-up leaf, and the node's close

The loop relaunches and the depth-first pick stays inside the node, landing on the leaf the last session cut. The session documents the knobs, commits, and retires the leaf in place. Walking the parent chain again, the node has no live leaf — and this time the `Done when` holds.

Closing a node moves nothing on disk (there is no `done/` to move into): its leaves are already marked done in place, and its `BRIEF.md` stays where it is. What the close *does* is **promote** anything from the node's brief that future siblings should still see — up to the parent brief, an ADR, or the glossary — so it stays in the brief chain after the node goes quiet, and then **report** the close by naming the node's handle in the commit message:

```
$ git log --oneline -2
e1d0c9b chore(grove): retire document-policy-knobs-k6, close design-token-bucket-k3 — promote design intent into root brief
7c6b5a4 docs(rate-limit): document token-bucket policy knobs
```

The promoted text is now part of an ancestor — `04-implement-k4`'s session reads it from the root `BRIEF.md` without descending into a quiet subtree. Had that left the *root* with no live leaf either, the walk would repeat one level up, silently, so an unattended run crosses a whole chain of closes without stopping.

If the node's `BRIEF.md` carried nothing worth promoting (the work was a tactical step whose conclusions live entirely in the code or ADR it produced), the session records that nothing needed promoting. The discipline is to *consider* promotion, not to always perform it.

To run that promotion out of band — because a prior session skipped it, or because you want it in its own commit — the user runs `grove retire`, in-worktree:

```
$ grove retire 03-design-token-bucket-k3
```

`grove retire` launches a focused harness session whose prompt does exactly that: promote anything still relevant from the node's `BRIEF.md` upward, in one focused commit. The node directory and its `DONE` leaves stay put.

## Iteration 5: the implementation leaf

The loop's next session picks `04-implement-k4.md`, wires the token-bucket middleware, commits, and retires the leaf in place (`04-DONE-implement-k4.md`). With that, every leaf is done — `grove-llm pick` comes up empty, and the loop switches from "do the next task" to proposing the **complete finish cycle**. That hand-off is the subject of [`finish.md`](finish.md).

## Codex harness

The CLI surface is identical: `grove do`, `grove retire <node-path>`, both run in-worktree. The methodology and prompts come from the same binary-provisioned global skill (`~/.claude/skills/grove/`) whichever harness runs, so `git log` and `tree` on `.grove/` look exactly the same — the on-disk evolution is the same shape for either harness.

The harness is resolved **per leaf, on every iteration** — leaf beats kind beats family beats stamp ([`adr/model-per-task-kind.md`](../adr/model-per-task-kind.md)). The stamp in `.grove-stamps/add-rate-limiting` is the **fallback**: an explicit on-disk binding that runs any leaf nothing more specific claims, not a binding that necessarily runs every task. So one `grove do` can span harnesses mid-flight. Had `03-design-token-bucket-k3` been cut as a review chain rather than decomposed, `GROVE_REVIEW_HARNESS=codex` would send its `-review` step to codex while every other iteration above stayed on the stamped harness; a vendor pair's two `research` leaves each name their own harness on the task file and differ by construction.

None of that changes anything this walkthrough demonstrates — not the tree, not the commits, not the CLI cadence. `pick` is the same stateless walk, retirement is the same in-place rename, and the loop relaunches on the same completion signal. What varies is which binary the driver exec's, reported in the one line it prints per launch (`grove: launching codex (model: …) — record-policy-adr-k5 (impl)`).
