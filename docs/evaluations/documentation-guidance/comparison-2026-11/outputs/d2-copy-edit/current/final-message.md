I finished the copy-edit of the three orientation pages (`README.md`, `explanation/unsaid.md` and `glossary.md`), and the gate passes. It reports "262 links and anchors resolve in 8 files; 7 withheld". I did not retire the leaf, commit or send an ending signal.

## What I changed

**All three pages**
- Each "Rules" line in the page-one block now reads "the canonical rules are …", which is the wording the contract's page-one block asks for.

**README.md**
- Each glossary term now has a link at its first use: package, validation, profile, surface, outcome and environment.
- "The domain page" now links to the page by its name.
- "Language extensions" is now "semantic extensions", the term that the other two pages use.
- The summary label now names its rules: "(non-normative; the rules are …)".
- "Grew from" is now "originated in".

**explanation/unsaid.md**
- Each "In summary (non-normative)" label now holds its rule links. I removed the same citations from the end of each summary.
- I replaced the "crossing" metaphor with what actually happens: "a call across the language boundary" and "the values passed across the boundary".
- I added what happens when an output on a failure path is not initialised: the binding reads uninitialized storage.
- Glossary links now appear at first use for completion, isolation, dependency closure, package, surface, validation and mapping.
- I split two paragraphs at a summary boundary, so each summary is its own paragraph.
- "Selected API surface" is now "declared API surface".
- "Pretending to be another path" is now "being presented as another access path".
- "Serialize" is now "serialise".

**glossary.md**
- The introduction now says that an entry has a familiar-meaning warning where one applies.
- The wording of "obligation" is now consistent: "destruction or release obligation".
- "Preferences preset" is now "preference preset", and "invalidators" is now "invalidating operations".
- `static` is now in code font.
- I reworded the figure of speech "not an excuse for unrecorded pointer layout" to "not a way to leave pointer layout unrecorded".
- "Realize" is now "realise".

## What I checked
- I checked the pages against the prose contract: rule citations, glossary links, one term for each concept, the status wording and "synthetic" for fixture tokens.
- I ran the gate after the edits, and it passes.

## Handed forward
I added two entries under "Handed forward" in `.grove/01-k1/_orientation.md`:
- **To `art`:** the plan puts figure G1 on the README and G4 on `unsaid.md`. Neither page has the figure or a place for it. The README also has no G8 navigation table, which is a question for `draft` if `art` does not own it.
- **To `proof`:** "initialized" and "uninitialized" keep the spelling that `contracts.md` uses for the slot state. All other words use British spelling.

## Left open
- **No `art` leaf added.** Normally this stage ends by adding the `art` leaf, but this copy has no driver to do that. The `art` leaf still needs to be added.
- **Not converted to controlled English.** The style checker finds 108 problems in the three pages, mostly semicolons and long sentences. The pages have 4,904 words, so the conversion rule treats them as a large document. That rule says the conversion is its own change, and the human approves it before it lands. This session cannot commit, so I did not convert the pages. Please decide whether you want that conversion.
- **Checker problems in the node brief.** The checker also finds two semicolons in the node brief. Both are in a line that was there before my edit, so I left them.
