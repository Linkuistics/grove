I drafted the three-page guide. It covers all nine keys and all seven glob forms, and the gate passes.

**Brief check.** The node brief points to `docs/editorconfig/contract.md`, and that file has a full structure brief. It names the reader and what they should be able to do afterwards. It gives the order of the three pages, and it says what to emphasise and what to leave out. So I had what I needed to draft. The contract says it was written by a model as a stand-in, not decided by a person.

**What I wrote**, all under `docs/editorconfig/guide/`:
- **`README.md`** explains what EditorConfig decides for a file. It also has a table of the four parts (file, core, plugin, editor) and says which part does what. It ends with a reading order and direct links into the other two pages.
- **`explanation.md`** has three sections, in the contract's order:
  - **Lookup and precedence** says which files a core searches, how the `root` key stops the search, which sections apply to a path, and which pair wins. It ends with an example that works out the settings for one path in a small directory tree. That example includes the one complete EditorConfig file the contract asks for.
  - **The file format** covers line types, section headers, pairs, why `;` and `#` start a comment only at the start of a line, and the length limits.
  - **The glob forms** covers the seven patterns, where a glob matches, and escapes.
- **`reference.md`** has one entry per key and one per glob form, each with a short example. The glob entries use explicit HTML anchors. Without them, headings made only of symbols, such as `*`, would get empty or clashing link targets.

**How I checked the facts.** I checked each statement against the specification source, not from memory. Where the source is unclear, the guide says so:
- The source does not say whether `{num1..num2}` includes its two end numbers, and the guide says that.
- The source says only some keys' values ignore letter case, so the guide claims this only for those keys.

The gate passes: 33 links and anchors resolve. The writing-style checker reports no hard problems and nine passive-voice warnings. Those warnings are advisory only, and I left them.

**Handed forward:**
- **Diagrams.** The contract asks for two figures: which files are searched for a path, and the order in which pairs override each other. Figures belong to the art stage, so I described both in words and did not draw them. I added both to the node brief's handed-forward list, with where each one goes.
- **Copy-edit leaf.** The tool that would normally create the next stage's task file is not available here, so I wrote `.grove/01-k1/02-copy-edit--editorconfig-k2.md` by hand. It lists the word risks the contract names: "pair" against "rule", the `root` key against the filesystem's root directory, and "section" used for both the file construct and a part of a page.

**Left open:** As you instructed, I did not retire the leaf, commit, or record a teardown.
