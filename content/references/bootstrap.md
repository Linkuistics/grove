## What Bootstrap reads, in order

Start from the mandate. Run `grove-llm resolve <handle>` to turn the stable
`<slug>-k<key>` handle you were given into its current file path, and stop if it
resolves to nothing or to a terminal (`DONE` / `ABANDONED`) leaf — that is a
stale or hand-edited launch, not work to redo. Then read, in order:

1. **The glossary** — `CONTEXT.md`, or the relevant bounded context via
   `CONTEXT-MAP.md`.
2. **The ADRs the briefs cite.**
3. **The `BRIEF.md` chain, root→leaf**, enumerated by `grove-llm brief-chain
   <resolved-path>` — the verb walks that leaf's **ancestor directories**, from
   the grove root down to the leaf's own directory, and prints each level's
   `BRIEF.md`, one absolute brief path per line, root→leaf. A level with no
   brief is skipped silently, so a node whose charter has not been written yet
   still bootstraps.
4. **The task file itself.**

That assembled context is the session's entire mandate; read nothing else by
reflex.
