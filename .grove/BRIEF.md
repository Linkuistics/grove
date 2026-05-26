# retire-cascade — brief

## Goal
Tighten grove's retirement doctrine so that completing a leaf reliably triggers a parent-chain check, asking the user before each node-level retirement and cascading upward when a retire empties the next ancestor. Today the doctrine *says* "when a node's last live leaf completes, retire the node," but in practice sessions end after the leaf commit without the parent-chain check, and the node sits un-retired until someone runs `grove retire` after the fact.

## Done when
- `content/SKILL.md`'s `**Retire.**` paragraph explicitly describes the parent-chain walk, the user-confirmation step, and the recursion.
- The leaf-level `mv` of a finished leaf into its parent's `done/` is clearly distinguished from node-level retirement (the former is mechanical, the latter is judgement and warrants asking).
- The change is consistent with Principle 5 ("grove guides, it does not gate") — confirmation, not gating; the user can say "not yet."
- `content/prompts/continue.md` stays a thin pointer to SKILL.md — methodology does not migrate into the prompt.

## Pointers
- `content/SKILL.md` — the doctrine; the `**Retire.**` paragraph and the inner-loop mermaid graph's `retire{"node's last live leaf?"}` diamond
- `content/prompts/continue.md` — the launcher prompt that defers to SKILL.md (one line today; keep it that way)
- `content/prompts/retire.md` — the prompt used by `grove retire` for the after-the-fact path; not on this grove's edit path, but useful to read for tone
- `docs/workflows/multi-step.md` — user-facing walkthrough that describes the inner loop; may need a wording tweak so it matches the revised doctrine

## Notes
- Motivating observation (May 2026): a long-running grove in this very repo ended with several un-retired subtrees because the agent never re-checked the parent chain after each leaf retirement. The user drives continue from up-arrow recall, so there's no out-of-session moment to notice the missing step — the doctrine has to make the check happen *inside* the session.
- "Ask" rather than "auto-retire" preserves the lazy/optional principle — the user may want to add a follow-up leaf to a "completed" node, and a confirmation gives them a moment to say so before the subtree gets archived.
- The cascade can run multiple levels in one session: a node retirement may empty its parent, which may empty its grandparent, etc. The walk terminates at the grove root.
