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
leaf's own directory, and prints one absolute node-file path per line,
root→leaf: the root `_BRIEF.md`, then each ancestor's `_<slug>.md`.
**Read the paths returned by `brief-chain`.** It refuses malformed levels
instead of skipping gaps; follow its named refusal before continuing bootstrap.
`BRIEF-FORMAT.md` owns the node-file grammar and the brief body's shape.
