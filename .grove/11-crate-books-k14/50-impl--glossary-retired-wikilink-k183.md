# glossary-retired-wikilink-k183

## Goal

Repoint `CONTEXT.md`'s one dangling `[[wikilink]]` — `[[Kind reference file]]`,
a term the glossary itself records as retired — at the term that replaced it.

## Context

- Found by `glossary-anchors-k62`, which anchored every term entry and then
  checked every wikilink target against the resulting heading set. Of 37 distinct
  targets across 115 occurrences, exactly one resolves to nothing. Two more read
  as misses under a naive comparison and are not: `[[Work-item handle]]` is
  **hard-wrapped** across a line break, and `[[procedure]]` names the second term
  of the joint heading *Condition / procedure (the `if` / `then` split)*, which a
  ` / ` split mangles because the parenthetical also contains ` / `.
- The dangling link sits in an `_Avoid_` line of the *Condition / procedure*
  entry: *"the driver resolved the kind before the session existed and named
  that kind's `[[Kind reference file]]`, so there is nothing to trigger the
  session into"* (`CONTEXT.md`, under `<a id="condition"></a>`).
- The glossary retired that name in the same document: *Family reference file*'s
  first `_Avoid_` reads *"the older name **kind reference file**, and the mapping
  it named"*, and records that `plugin-kind-skills-k17` moved the seven
  single-kind files inline into their own kind skills.
- **Which term replaces it is a judgement, not a rename.** The driver now names
  the kind's *skill*, so `[[Kind skill]]` is the likely successor — but the
  sentence is about what a session is *routed into*, and `[[Family reference
  file]]` is the other candidate. `glossary-anchors-k62` was scoped to
  presentation and explicitly forbidden from editing an `_Avoid_` line, so it
  left the call here rather than guessing it.

## Done when

- No `[[wikilink]]` in `CONTEXT.md` names a term the glossary does not define.
  The check is an enumeration, not a grep: parse every `[[…]]` target and every
  `###`/`####` heading (splitting joint headings on ` / ` and dropping
  parentheticals), then diff — hard-wrapped targets must be unwrapped before the
  comparison or they read as false misses.
- `bash scripts/check.sh` passes.

## Notes

**Two words of prose, and the reason it is a leaf at all** is that choosing
between `[[Kind skill]]` and `[[Family reference file]]` decides what the
`_Avoid_` line teaches — vocabulary, which `glossary-anchors-k62`'s task file
put out of its own scope.
