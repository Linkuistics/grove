# dive-openclaw-k9

**Kind:** work

## Goal

Deep-dive **openclaw/openclaw** (https://github.com/openclaw/openclaw) as a
survey source. Primary grove interest: its **tiered file-based memory** with
"no hidden state — memory is files on disk," which closely mirrors grove's
git-tracked-tree philosophy. End with a **takeaway-for-skills** and a
**takeaway-for-grove**.

## Context

- Shortlist rank #6 (`docs/research/skill-repo-prior-art.md` §1a). Verified
  380,319★ (GitHub API, 2026-06-25). A personal-AI-assistant control plane, not a
  skills repo — adopt the memory architecture, not files.
- Read the node `BRIEF.md` for downstream questions + discipline; `CONTEXT.md`
  for the target split.

## Done when

- A `## openclaw/openclaw` section is appended to
  `docs/research/skill-repo-prior-art.md` with cited findings, each tagged
  **target** + walk-away note, ending with takeaway-for-skills /
  takeaway-for-grove.

## Notes

- Focus — **grove Q4**: the memory tiering — `SOUL.md`/`IDENTITY.md` (immutable)
  → `MEMORY.md` (durable, loaded session-start) → `memory/YYYY-MM-DD.md` (running
  notes, today+yesterday auto-loaded) — and the on-demand `memory_search` /
  `memory_get` retrieval (retrieve rather than front-load). Map each tier onto
  grove's artifacts: which is the `CONTEXT.md` glossary, which is the `BRIEF.md`
  chain, and does grove need a "running notes" tier (it deliberately has none —
  constraint 1, "artifacts not state")?
- The sharp question: openclaw *auto-loads* recent notes; grove *reads on
  demand* via `brief-chain`. Which discipline wins for long-horizon work, and is
  there anything grove should borrow without reintroducing hidden state?
