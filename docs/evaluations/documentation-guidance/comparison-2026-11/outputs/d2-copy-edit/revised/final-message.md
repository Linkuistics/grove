I finished the `copy-edit` stage on the three orientation pages. The gate passes. No sentence change touched structure, normative wording or the teaching the draft owns.

## What I changed

**Page-one blocks.** Section 5 of the plan fixes the wording of the "Rules" line in the block at the top of each page. Two pages did not use it, so I changed them:
- In `README.md`, the line now cites the rule identifiers (L1–L6 with the grammar, C1–C8, A1–A5), not just the file names.
- In `explanation/unsaid.md`, the line now opens with "the canonical rules are".
- In `README.md`, the "Reader" line also says what the primary reader already knows, which the contract requires.

**One term, one name.**
- The pages described a profile as a "language extension" in some places and a "semantic extension" in others. The glossary and the A1 summary say "semantic extension", so I used that everywhere.
- I made "preference preset" the same in all places.
- `README.md` called the producers "Humans, analysers and third-party producers" in one section and "Human authors, analysers and third-party tools" in another. I used the second name in both.
- In `unsaid.md`, the package now describes the "declared" API surface, not the "selected" one. "Declared" matches the glossary and `README.md`.
- In the glossary, "one owned obligation" is now "one ownership obligation", the term the glossary defines.

**Glossary links at first use.** The contract says to link a glossary term the first time each page uses it. These terms had glossary entries but no links:
- In `README.md`: package, validation, surface, outcome, concurrency, pattern, environment and digest tokens.
- In `unsaid.md`: completion, isolation, package, surface, validation and access mapping.

**Precise wording.**
- One sentence said `watch_view` "has storage for an output length". It now names the second pointer, `length`, so the reader can find it in the C header.
- The glossary's ABI entry said a package does not "implement" an ABI. It now says "define", which matches the source.
- I removed bold from "within this draft", because the prose contract does not allow bold for emphasis.

**Glossary links into the explanation page.** Some links used loose labels such as "callback teardown" or "ownership explanation". Each now uses the exact heading of the section it points to. I also dropped "relevant" from two "relevant rule(s)" labels so that every entry uses the same label. The glossary's opening now says that an entry warns against a misleading familiar meaning where one applies.

I kept the US "-ize" spellings in the document, such as "initialized" and "serialize". The normative files use them, and the contract does not set a spelling.

## What I checked

- I ran the gate after the edits and again at the end. The final run passed: 262 links and anchors resolve, and 7 links point to withheld files, which the gate does not follow.
- I compared each page with the plan's prose contract, the page-one block and its section 9.1.
- I ran the controlled-English checker on the new task file. It found no defects.

## Handed forward and left open

- **Next stage.** Per the stage's last-act rule, I created the `art` leaf as `.grove/01-k1/02-art--orientation-k2.md`.
- **Two missing figures.** The contract places two figures on these pages: G1, the system context, in `README.md`, and G4, the ownership-transfer sequence, in `unsaid.md`. The draft placed neither figure. It also did not record these empty slots in the handed-forward list, which the plan asks the draft to do. I described both slots in the `art` leaf instead of sending the work back to `draft`, because figures belong to `art`. The leaf also notes that `figures/style/` does not exist yet, so the first `art` session must create the shared style there.
- **Figure text.** There was no figure text to correct, so the "Handed forward" list in the node brief still says nothing was handed forward.
- **What I did not do.** As the launch rules say, I did not retire the leaf, commit, record a teardown or send an ending signal.
