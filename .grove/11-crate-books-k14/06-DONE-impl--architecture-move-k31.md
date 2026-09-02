# architecture-move-k31

## Goal

Move `docs/ARCHITECTURE.md`'s descriptive account into the overview, leaving that
document carrying decisions, constraints and measurement records only, with every
citation of a moved anchor re-pointed and every link suite green.

## Context

- Decision 5 of `plan-k1`, as amended by `plan-k8`: the move is mechanical
  **after** the anchor resolver is widened, and `architecture-anchors-k19` is what
  widened it. If that leaf did not land, this one cannot be done safely — the
  Rust-source citations would move silently.
- The guards, and what each one catches:
  `every_repository_markdown_reference_resolves` sweeps every Markdown file in
  the repository; `architecture-anchors-k19`'s check covers the same anchors in
  Rust sources; and the `include_str!` content assertions in
  `crates/grove-llm/tests/composition_guidance.rs` read `docs/ARCHITECTURE.md` in
  eight places and **go red on this move by design**. Turning those red tests
  green is how the citations are re-pointed — not a hopeful sweep.
- What moves and what stays is settled, and it is narrower than this leaf was
  cut expecting. `docs/specs/overview-book-structure.md`, *What this book
  absorbs from `ARCHITECTURE.md`*, is the charter. **Three sections are in play**
  — *Runtime flow*, *Command surfaces*, *Main module seams* — and the six other
  descriptive-looking sections stay whole, because they describe other crates'
  internals and leave with those crates' books (`architecture-residue-k75`).
  Within the three, the rule is a positive move test with the default to stay: a
  clause moves iff it states what the system does, is shaped like, or reaches;
  everything else, including anything not confidently classifiable, stays. None
  of the three is "descriptive throughout" — the brief lists the residue each
  keeps, and this leaf should expect to leave text behind in all of them.
- Anchors are the contract, and the brief has decided them: **every anchor stays
  above its surviving argument, and no citation is re-pointed**, in Rust or in
  Markdown. The measurement behind that is in the brief — no real design citation
  points at description. Each stripped section opens with exactly one forward
  pointer into the overview page now carrying its description; that link is
  inbound to the book, so the book contract does not bind it and
  `every_repository_markdown_reference_resolves` checks it.

## Done when

- The description in *Runtime flow*, *Command surfaces* and *Main module seams*
  has moved into the overview under the brief's rule, and each of those sections
  opens with its forward pointer.
- Every surviving descriptive passage in the six sections not in play is marked
  with the book that will make it redundant, so `architecture-residue-k75`
  inherits a list rather than a judgement. **The decisions-only end state is not
  this leaf's; it falls due at k75.**
- The *Documentation ownership* table's own row is narrowed to the decisions,
  the constraints and the measurement records, in the wording the brief gives
  under *The book's row in the ownership table*. The overview's row is
  `overview-book-k30`'s and should already be there.
- No anchor moves and no citation is re-pointed;
  `every_architecture_anchor_citation_in_a_source_resolves` and
  `every_repository_markdown_reference_resolves` are green with no source edit.
- `bash scripts/check.sh` passes, with
  `crates/grove-llm/tests/composition_guidance.rs` green against re-pointed
  citations rather than against loosened assertions.
- The overview's own final validation still passes: moving prose into a book must
  not break its fragment graph.

## Notes

**Loosening an assertion is not re-pointing a citation.** The `include_str!`
checks go red because they assert content at a path; the fix is the path or the
content, never the assertion's strictness. A test weakened to accommodate the
move deletes the only guard this leaf has.

**A finding against a section does not reach the summary layer.** The ownership
table, this document's opening, and `CONTEXT-MAP.md`'s account of what
`docs/ARCHITECTURE.md` holds all summarise sections; correcting a section leaves
them stale and reading as descriptions.

**Verify the sweep, do not trust it.** A clean grep for a moved anchor proves
nothing on its own — run a positive control that finds an anchor you know is
present, and watch a deliberately broken citation come back dirty, before
crediting the clean read.

## Decisions (running log)

**The eight `include_str!` assertions did not go red, and that is the brief's
doing rather than a loosening.** Every assertion in
`crates/grove-llm/tests/composition_guidance.rs` that reads `docs/ARCHITECTURE.md`
targets *Task kinds and composition*, *Tree access lock* or *Authoritative
selection* — none of the three sections in play once
`docs/specs/overview-book-structure.md` narrowed the move. Nothing in that file
was edited. The guards that bound this leaf were the Markdown sweep and the
Rust-source anchor scan, and both are green with no source edit, as the brief
predicted.

**Clause by clause, the three sections keep exactly the residue the brief
listed, plus one clause it did not classify.** *Runtime flow* keeps the
*two advisory steps* paragraph; *Command surfaces* keeps *twelve, not
thirteen*, the discoverable-API ground, the flatness-is-pinned constraint
(rewritten from description into the constraint it grounds, so it stands
without the moved sentence it followed), *two entries are gone*, and the
compiler-enforced clause; *Main module seams* keeps the identity consequence,
the three absent-module records, the file-sized-modules ground, the visibility
constraint and the `loop-crate-verbs-k21` observation. The unclassified clause
— *every verb is admitted through the session-epoch guard* — was verified
against `crates/grove-llm/src/cli.rs` (`run` admits every command before
dispatch, `complete` included) and moved into chapter 2 as description.

**The runtime-flow diagram is deleted, not copied.** Chapter 3's *One
foreground iteration* list already draws the same relation step for step, and
the spec's figure rule treats a relation drawn twice on one page as an
editorial finding. Two sentences the list did not state — the driver stays in
the foreground and owns its child; the completion signal is a temporary
control message — were added to that section's closing paragraph.

**The module table lands in chapter 5 with one row corrected as it moves.**
`session_config`'s row said the loop re-reads `TemplateSource` *once per
iteration*; the book adjudicated *twice* (`template-source-read-count-k86`)
and `crates/grove-loop/src/loop_driver.rs` loads it at two points per
iteration. Carrying *once* into a proofed book would reintroduce a defect the
draft removed, so the row says *twice, before and after the tree transition*.
Every other row was checked against `crates/grove-loop/src/lib.rs`'s module
list and re-exports and stands as written.

**Residue marks are HTML comments, one per passage, grep-able as
`residue(<book>)`.** A visible mark would put process bookkeeping into a
rendered guide; a comment is invisible to a reader, visible to
`architecture-residue-k75`, and carries the qualifier a mixed passage needs
(which clauses it covers, which records stay, and which
`composition_guidance.rs` assertions pin that text). `residue(none)` marks a
passage no crate book will cover because it describes the methodology plugin;
k75's *do not delete what no book covers* rule then applies. One legend comment
at the first mark states the convention. *Session configuration* and *Process
ownership* are descriptive of `keyed-launch` and `driver_lease` too and are
deliberately unmarked, because the brief lists them as untouched.

**Two explicit anchors added, none moved.** `architecture-anchors-k19`
reported that `tree-access-lock` and `the-boundary-is-a-build-not-a-commit`
are cited from Rust sources but resolved only through generated heading
anchors, and handed the fix to this leaf. Both now carry `<a id="…"></a>`
lines in the document's existing pattern. Seen to matter: with the heading
retitled and the anchor absent the scan reported all eight citations red; with
the anchor present the same retitle was green.

**Controls seen red before the clean read was credited.** The Runtime-flow
forward pointer aimed at `#one-iteration-moved` failed
`every_repository_markdown_reference_resolves` with the file, line and reason;
the retitle above failed `every_architecture_anchor_citation_in_a_source_resolves`
the same way. Both edits were reverted and `jj diff --stat` shows only the
seven intended files.

**The summary layer was swept, not just the sections.** The document's
opening now says what it holds and what moved; its ownership row carries the
brief's wording; `README.md`'s architecture bullet no longer lists runtime flow
and module seams and a system-overview bullet sits beside it; `CONTEXT.md`'s
*how a seam works belongs to* sentence names the books; `CONTEXT-MAP.md`'s
grain sentence records the split and a past-tense record reads as past.
`docs/USAGE.md`'s two links point at a decision anchor and at *why*, and stand.

**The leaf's one in-session reviewer was spent on the move rule, since the
brief names an adversarial read as the rule's only control.** A fresh context
was given the rule, the diff and the destination pages with the conclusions
stripped. Twelve findings, classified:

- **Valid, fixed** — chapter 5's opening enumerated three maps and the table
  made four; the table's lead-in restated the boundary sentence the page
  already carries and miscounted the unwritten books (two, not three) — both
  dropped; the epoch-admission sentence in chapter 2 sat inside the paragraph
  that counts *three facts checked* and now stands as its own *described, not
  checked* paragraph; the *Bootstrap* clause was marked `grove-loop` and is the
  methodology's; the module-seams pointer named the modules heading for a claim
  half of which is the package map, and now points at the page from its
  package map onward; six residue marks covered a decision or a ground without
  saying so and now exempt it, and one (lazy chains against the eager pair) was
  design argument throughout and is unmarked; one methodology passage (what
  reviews and integrations are) was unmarked and is marked `none`.
- **Valid, reconciled** — chapter 3 said the driver *owns* the child it
  spawned while chapter 5's runner row says `keyed-launch` owns the spawn and
  the kill; chapter 3 now says the driver is the parent of the child the runner
  spawned for it.
- **Visible trade-off, kept** — the two explicit anchors duplicate the
  generated heading ids beneath them, as `command-surfaces` and
  `version-control-seam` already do; a comment directly before a table header
  now has a blank line after it, though CommonMark closed it on its own line
  either way; the `jj-workspace` marks name a book that has landed, and the
  legend now says such passages are deletable at once.
- **Noise** — three borderline passages (*moves are not commits*, the plugin
  install route, the reachability table) are decision records with an ADR or a
  measurement behind them and stay unmarked under the default.
- **No finding** — no moved clause was a decision; no deleted fact is absent
  from the book; every pointer resolves; the ownership row matches the brief;
  the corrected `session_config` row was verified against the loop source.

No second review is needed: every fix is a prose correction inside the same
pages, checked again by the link sweep, the anchor scan and the overview's
final validation.
