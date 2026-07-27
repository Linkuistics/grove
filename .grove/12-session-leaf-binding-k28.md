# session-leaf-binding-k28

**Kind:** design

## Goal

Decide whether the loop driver should **bind** the launched session to the leaf
it routed on, rather than having the session re-derive that leaf independently —
and, either way, make the skill's Pick step tell the truth about where the pick
actually happens.

## Context

Raised by the human mid-`observe-live-surface-k26`, on the observation that *a
grove session starts with pick, but the pick has to occur before the session
starts because that determines what model/harness is used.*

Verified against the source. The driver resolves the leaf **before** the session
exists, and never tells the session which leaf it resolved:

- `resolve_kind` (`src/loop_driver.rs:1066`) runs `grove-llm kind
  --with-harness` as a subprocess; with no leaf path that verb picks internally
  (`src/llm_cli.rs:344` → `tree_read::pick`). This is the **routing** pick: its
  result feeds *Kind routing*, binding harness (leaf ▸ kind ▸ family ▸ stamp) and
  model (harness-major, four keys).
- `picked_leaf` (`src/loop_driver.rs:958`) picks again in-process, but its own
  doc comment says it is "*reporting*, not routing" — it only supplies a path to
  the dry-run readiness line, and degrades to `None` rather than failing.
- `content/prompts/continue.md` hands the session **no leaf identity at all**. It
  says "use the grove skill", whose loop step 1 is Pick — so the session runs a
  third, independent `grove-llm pick`.

The session's pick is *unbound*: it reads a directory and returns a path, with no
reference to the routing decision already made and already acted on. The two
agree only because nothing mutates `.grove/` between the driver's peek and the
session's first tool call. That is an **unenforced coincidence, not an
invariant**.

The failure it admits is the one *Kind routing* is otherwise careful to exclude.
That policy makes a kind resolving no model var **fail loudly**, explicitly
rejecting fall-through on the grounds that it is "still grove deciding, only less
visibly". But if the driver's pick and the session's pick ever disagreed, the
session would run a leaf under a harness and model bound for a *different* leaf's
kind — silently, with no diagnostic anywhere. Same class of error, opposite
handling.

Divergence is currently hard to trigger, which is why this is a design question
and not a bug report. The candidate windows are all narrow: a human editing
`.grove/` between peek and launch, a harness that mutates the tree during its own
startup, or any future driver that peeks further ahead than it launches. None is
known to occur today.

## Done when

- A decision is recorded on whether the driver binds the session to the routed
  leaf. The obvious shape — driver exports the resolved leaf path, the skill's
  Pick step reads it and treats a mismatch against a live `pick` as a **loud**
  failure — is a candidate, not the answer; "accept the coincidence, fix only the
  documentation" is a legitimate outcome and is cheaper.
- Whatever is decided, the **cost of binding** is weighed against constraint 2
  (*read, don't run*) and constraint 5 (*grove guides, it does not gate*): a
  session that refuses to start because an env var disagrees with the tree is a
  gate, and grove does not gate. A warning is not.
- The skill's Pick step and the *Kind routing* glossary entry are reconciled with
  where the pick actually happens. SKILL.md presents Pick as the session's first
  act; the routing pick is the driver's, and it has already bound the model
  answering the prompt by the time the session reads that step.
- If the decision changes behaviour, an ADR carries it (this touches
  *self-driving-loop* and *model-per-task-kind*); if it is documentation-only,
  no ADR is written.

## Notes

Scope is the **binding**, not the pick count. Three picks per iteration is cheap
(a directory walk) and de-duplicating them for its own sake buys nothing — the
in-process `picked_leaf` is already justified in its doc comment as degrading
harmlessly. The question is whether the *identity* the driver routed on should
travel to the session, not whether the walk should run fewer times.

Independent of the herdr work; sequenced last on arrival, with no claim that it
belongs there permanently.
