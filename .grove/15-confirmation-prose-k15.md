# confirmation-prose-k15

**Kind:** impl

## Goal

Reconcile every normative surface outside the ADR / spec / glossary set to the
decision in `docs/adr/confirmation-boundary.md`: **the Retire cascade asks the
human nothing; it checks, promotes and reports instead.**

`retire-confirmation-k12` made the decision and reworked the ADR set, the spec and
`CONTEXT.md` in place. It deliberately did **not** touch `content/`, `docs/` or
`src/`, because the prose rewrite is `impl` work and the sweep is wide enough that
a fresh reviewer earns its place.

## Context

**What changed.** The cascade's per-node confirmation is gone for *all* node
species. In its place a brief-carrying node's close runs four steps: check the
node's brief `Done when` against what the subtree delivered; `leaf-add` the missing
work if the check fails and the gap can be named; escalate (stop and say so) if it
cannot; promote what survives upward and name the closed node by its
`<slug>-k<key>` handle in the commit message. A brief-less chain node still closes
as a no-op. Leaf retirement is **unchanged** — it was never confirmed. Pruning is
**unchanged** — still HITL. The finish cycle's single gate is **unchanged**, and is
now the loop's only routine human gate.

**Read the ADR first**, not this summary — it carries the two tests that generate
the rule, and the prose you write has to teach the tests, not just the outcome.
`CONTEXT.md` § *Confirmation boundary* is the compressed version.

**Do not restate the ADR.** The grain rule holds: `content/SKILL.md` states the
procedure and cites the record; the record holds the reasoning.

## Done when

Every surface that stated the old claim states the new one, and no surface states
both.

**Grep for the claim, do not walk a file list.** This is the lesson
`chain-node-integrate-k11` paid for twice and `docs/specs/task-kind-taxonomy.md`
records: a file list is written before the work and goes stale, while the claim
*is* the thing that went stale, so it does not know which file it is in. The
scoping grep was `ask the user|ask before|asks the human|ask again|confirmation|
before treating` over `content/ docs/ src/ README.md`; re-run it, widen it, and
trust it over the notes below.

Surfaces the scoping grep hit, as a **starting point and not a checklist**:

- `content/SKILL.md` § Retire — the substantial rewrite, including the recursion
  sentence (*"re-check, ask again, recurse"*).
- `content/BRIEF-FORMAT.md`, `content/TASK-FORMAT.md` — both justify the
  `BRIEF.md` discriminator by the confirmation. Its job changed rather than
  vanishing: it now selects whether a closing node has *work to do*.
- `content/driving.md` (~line 422), `content/prompts/retire.md`.
- `docs/grove.md` — *"a chain node closes silently while a decomposition node's
  close asks the human"*.
- `docs/workflows/multi-step.md` (~113–115) — a walkthrough whose scripted beat is
  *the user said not yet*. That interaction can no longer happen; rewrite the beat
  as check → `leaf-add`, which is a **better** illustration of the same point.
- `src/tree_grow.rs` (~989, ~1033), `src/llm_cli.rs` (~135), `src/tree_lifecycle.rs`
  (~241) — doc and test comments.
- `grove-llm --help` / `grove --help` text. `chain-node-integrate-k11` found two
  stale `--help` surfaces by grep after reading missed them, and `--help` is the
  only surface a human at a terminal reads.

**Keep the mutation tests.** `src/tree_grow.rs`'s brief-absence assertion is
load-bearing under the new rule too — a stray charter now silently promotes a chain
into the species that carries close-time work. Rewrite its *justification*; do not
weaken the assertion.

**Check `grove retire`'s framing** (`src/cli.rs:99`). The verb launches a session
to close a node by hand. Its purpose survives — promote the brief, close the node —
it simply no longer asks. Confirm the doc comment does not imply otherwise.

## Notes

**`content/` reaches sessions only at the next release.** It is provisioned to the
global skill dir **from the binary**, so this edit changes no running session until
a release is cut. That is the known lag `docs/specs/task-kind-taxonomy.md` records;
do not treat it as a defect of this leaf, and do not cut a release here.

**Do not re-litigate the decision.** If the rewrite surfaces a genuine hole in it,
say so and stop rather than patching prose around it — that is what
`confirmation-prose-review-k16` is for, one leaf later, with a fresh context.
