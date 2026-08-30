---
name: grove
description: Grove's shared methodology spine — the seven constraints, the bootstrap, execution, decomposition, retirement and commit procedures, the three review and research family files, and the five format documents. Use when a `grove-<kind>` skill directs you here; when running any session inside a grove working tree; or when starting, picking up, or continuing a task tree under .grove/.
harnesses: [claude-code]
---

# grove — hierarchical, self-extending workstreams

A **grove** is one workstream driven as a VCS-tracked tree of task files under
`.grove/`. The tree's shape is the only state; the VCS holds the history.

**One task is one session.** A leaf is one session's whole work, and a leaf that
proves too big for that decomposes rather than running long.

This file is the **shared spine**: every rule that holds across families. What
one kind owns — its goal, its deliverable, its human-in-the-loop mark, its review
allowance, whether it passes the done flag when it signals — is inline in that
kind's own `grove-<kind>` skill and is stated **nowhere twice**.

The rest of this file is a register of **conditions** — that a situation exists
calling for something other than what you are doing — each naming the file whose
procedure answers it. Nothing here summarises those files; a procedure you need
is one read away.

## Your kind's skill was named for you

**The driver resolved your leaf and its kind before this session existed, so you
select nothing here.** Your prompt named one `grove-<kind>` skill; that skill is
your kind's procedure and you open it before you act. This spine is what it
directs you to for everything it does not own.

A kind exists **iff** a skill of that name exists. There is no list of kinds
here, and nothing in this corpus enumerates them.

## The spine — seven constraints

Non-negotiable, and cited by number throughout this corpus.

1. **Artifacts, not state.** No phase file, no session log, no status file.
2. **Read, don't run.** A session bootstraps by *reading markdown*.
3. **Suggested shape, not enforced schema.** Task files and briefs are freeform.
4. **Lazy and optional.** An artifact is created only when it earns its place —
   just-in-time, not few.
5. **grove guides, it does not gate.** grove never refuses to proceed.
6. **Walk-away-able.** Delete this skill and `.grove/` is still legible notes.
7. **One page of rules.** If the loop does not fit a page, cut until it does.

## What the driver settled before your session

**Your mandate is authoritative.** The driver picked one leaf before this
session existed, and nothing modulates that pick.

**Do not pick again.** `grove-llm pick` is a diagnostic, not this session's
dispatcher; where a second walk disagrees, the mandate wins.

**The stated VCS is definitive.** Do not re-derive which lane this working tree
is on, and disregard a harness banner that disagrees.

**The HITL mark predicts, it does not permit.** `requirements`, `prototype` and
`finish` normally resolve through a live human and every other kind is driven
alone — but any kind may stop and ask a human, and doing so is always legitimate.

## Bootstrap

Resolve the mandated handle, then read: the glossary, the ADRs the briefs cite,
the `BRIEF.md` chain root→leaf, and the task file. Nothing else by reflex.

## The loop

- When how this session was launched matters, read `references/driver.md`.
- When the mandated handle resolves to nothing or to a terminal leaf, stop as
  `references/bootstrap.md` directs.
- When considering an in-session reviewer, apply the budget in
  `references/execute.md`.
- When about to make a repo-wide claim, verify it as `references/execute.md`
  requires.
- When a decision settles, record it as `references/execute.md` directs.
- When handing a question back to a human, frame it as `references/execute.md`
  directs.
- When work surfaces that does not serve this leaf's goal, externalise it
  through `references/decompose.md`.
- When this leaf proves bigger than its brief, decompose it through
  `references/decompose.md`.
- When an artifact may need review, decide it as `references/decompose.md`
  directs.
- When a question may need two independent surveys, decide it as
  `references/decompose.md` directs.
- When cutting an integration step, place it by the rule in
  `references/decompose.md`.
- When you foresee work you cannot yet state precisely, follow
  `references/decompose.md`.
- When a design needs lessons this codebase cannot show, follow
  `references/decompose.md`.
- When the work is done, retire the leaf as `references/retire.md` directs,
  before committing.
- When this leaf's path looks decided against, stop and ask, as
  `references/retire.md` directs.
- When a leaf's place is in doubt, choose the verb `references/retire.md` names.
- When a node has no live leaf left, close it as `references/retire.md` directs.
- When the last live leaf retires, leave finishing to the driver, as
  `references/retire.md` directs.
- When the leaf is retired, commit it as `references/commit.md` directs.

## The family files

Three files in this spine belong to a **family** of kinds rather than to one,
and are loaded because a member's skill directed you here **by name** — never
because you chose between them.

- `references/review.md` — the five `review-*` kinds.
- `references/integrate-review.md` — the five `integrate-review-*` kinds.
- `references/research.md` — `research-a` and `research-b`.

## Artifacts

- When deciding where a durable artifact belongs, follow `references/grove.md`.
- When a rule cites the `linkuistics` plugin, read what binds without it in
  `references/grove.md`.
- When considering an ADR, apply the test in `ADR-FORMAT.md`.
- When changing a recorded decision, rework the set as `ADR-FORMAT.md` directs.
- When this increment may be an agreement point, consult `SPEC-FORMAT.md`.
- When changing a spec, keep its set current as `SPEC-FORMAT.md` directs.
- When a term is resolved, record it as `CONTEXT-FORMAT.md` directs.
