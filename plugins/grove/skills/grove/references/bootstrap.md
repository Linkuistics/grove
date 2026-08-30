## Resolving the mandate, and when it does not resolve

Start from the mandate. `grove-llm resolve <handle>` turns the stable
`<slug>-k<key>` handle you were given into its current file path.

**A handle that resolves to nothing, or to a terminal (`DONE` / `ABANDONED`)
leaf, is a stale or hand-edited launch — not work to redo.** Say which of the two
you got, and stop: the tree has already moved past this leaf, and re-doing it
would either duplicate finished work or invent work no leaf asks for.

## Walking the brief chain

`grove-llm brief-chain <resolved-path>` enumerates the ancestor briefs. The verb
walks that leaf's **ancestor directories**, from the grove root down to the
leaf's own directory, and prints one absolute `BRIEF.md` path per line,
root→leaf. **A level with no brief is skipped silently**, so a node whose charter
has not been written yet still bootstraps — briefs are lazy artifacts and a
reader must not fail on a missing one.
