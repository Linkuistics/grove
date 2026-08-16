# skill-delivered-methodology-k2

## Goal

Turn the settled *what* from `plan-k1` into a spec: how Grove's methodology
reaches a session once it is a provisioned, progressive-disclosure skill again,
and what rides the short guaranteed core in `${prompt}`.

This is a genuine agreement point — the artifact is load-bearing and the rewrite
that follows builds on it for months — so `docs/specs/` is the right home
(`SPEC-FORMAT.md`), and the existing `docs/specs/mandate-delivered-methodology.md`
is what it reworks rather than sits beside.

## Context

Read `plan-k1` and the root `BRIEF.md` first; both are short and carry the
decisions and the tension. In one line: mandate delivery is retired because a
~49 KiB `${prompt}` was measured to degrade sessions, and the skill it reverts to
was itself measured to go unread — so the design must answer **both**, and
answering one is a swap.

The prior design is not wrong so much as superseded, and much of its reasoning
survives its mechanism. `docs/specs/mandate-delivered-methodology.md` is 1,640
lines; read for *why* a device exists before deleting it.

## Done when

A spec exists that settles, at minimum:

1. **The rule for the guaranteed core.** What earns a place in `${prompt}`, stated
   as a rule rather than a list — a list goes stale silently, which is the
   superseded ADR's own objection to manifests. Two delivery paths that can
   disagree is precisely the state it rejected, so the rule is what makes the
   size honest and the drift bounded.
2. **Trigger strength.** The frontmatter `description:`, the launcher's wording,
   how `SKILL.md` opens. **This is the half that answers the observed prior
   failure**, and nothing else in the design does — so it is the spec's
   load-bearing section, not a finishing touch. Worth surveying how other skills
   that must not be skipped are worded.
3. **The skill's layout.** What is in `SKILL.md` versus `references/`, and where
   per-kind discipline lives given that `${prompt}` names the kind's file
   directly.
4. **What the rework of the two records says**, including the argument owed for
   overturning *"never as a replacement for triggering conditions"*.

## Notes

- The 140 unit markers are **scaffolding**: they already record which prose is
  `if` and which is `then`. Use the classification to drive the split, then let
  the deletion increment remove the markers.
- `grove-llm methodology <id>` still works while you are in this leaf, and is the
  fastest way to read a procedural body without opening `content/`.
- Not in scope: cutting the increments. That is a `planning` leaf, and whether it
  needs one or the spec can be sliced directly is this session's call to make at
  the end.
- Review chain: cut `review-design` with the same bare stem if the spec lands
  load-bearing, which it is expected to.
