I finished the draft stage for the EditorConfig guide. The guide has three pages in `docs/editorconfig/guide/`, and they follow the structure brief in `docs/editorconfig/contract.md`. The links gate passes, and the prose checker reports no hard findings. As the evaluation rules require, I did not retire the leaf, commit or send an ending signal.

## What I changed

- **`README.md`** introduces the guide to the reader and states the specification version, 0.17.2.
- **`explanation.md`** follows the order in the brief: how a core finds and ranks EditorConfig files, then the file format, then the glob forms. A "core" is the library that reads the files for an editor.
  - The page uses a worked directory tree to show which files the core reads and which pair wins. A "pair" is one setting, such as `indent_style = tab`.
  - The precedence rule has its own anchor, so a reader can link straight to it.
  - The page has the one complete example file that the brief asks for.
- **`reference.md`** has one entry for each of the 9 keys and each of the 7 glob forms. Each entry carries its own context and example, so a reader can use it without the pages before it.
  - All entries of one kind have the same fields.
  - Each entry has an anchor that does not depend on its heading text.
  - Each example is labelled "Fragment" and says what it leaves out.
  - Where the specification says nothing, the entry says so and does not guess. Examples are the case of some values, `/` inside `[seq]`, and whether the end numbers of `{num1..num2}` match.

## What I checked

- I checked each claim against the specification source, `docs/editorconfig/source/index.rst`, not against memory. Late in the session I also checked the length limits again, and the page now matches the source.
- `python3 tools/gate.py docs/editorconfig` passes. All 22 links and anchors in the 4 files resolve. The gate does not follow the 2 links to other sites.
- `ste-lint.py` first reported two sentences that were too long in `explanation.md`. I split the first one and turned the length limits into a list. The checker now reports no hard findings and 25 advisory ones. Most of the advisory findings are the passive "are omitted" in the example labels.

## What I handed forward

- I created the next leaf, `.grove/01-k1/02-copy-edit--editorconfig.md`, by hand, because the rules for this evaluation forbid `grove-llm`. It tells the copy-edit stage three things:
  - If the stage rewords the "are omitted" labels, it must change all of them the same way.
  - It must keep each easily confused term to one name throughout, for example the `root` key and the filesystem root directory.
  - It must not reword the gaps in the specification into answers.
- Nothing went into the node brief's "Handed forward" list, because I found no problem that belongs to a stage after copy-edit.

## What I left open

- The brief asks for two figures: which files the core reads, and the order in which pairs override each other. I covered both with numbered lists and a result table, not with drawings. The art stage can decide whether the guide needs real figures.
- The contract file is a stand-in that a model wrote, so a person has not confirmed the structure brief.
