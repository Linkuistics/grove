# grove.dont-put-item-title-in-folder-names — brief

## Goal

Take the work item's title out of every node directory's name, so that a deep
task tree stays inside the operating system's path limit, and carry that title
on the node's own file instead — `NN-k<key>/_<slug>.md` where today it is
`NN-<slug>-k<key>/BRIEF.md` — then move every existing grove onto the new
grammar.

## Done when

- A node directory is `NN-k<key>/` and carries no slug. Its title is the name
  of its `_<slug>.md` file; the root's file is `_BRIEF.md`. The leaf grammar
  is unchanged.
- Every node holds exactly one `_` file. A level with none, or with two, is
  malformed and refused by name, with the canonical form in the refusal as
  every grammar refusal carries it.
- Every verb reads and writes the new grammar and nothing else: `resolve
  <slug>` finds a node by its `_` file, `brief-chain` prints it, `pick` and
  `kind` skip it, `leaf-decompose` writes `NN-k<key>/_<slug>.md`, `root-init`
  writes `_BRIEF.md`, and no title is ever read from a file's contents.
- The `ordinal-fs-tree` models, architecture and conformance kit state the
  obligation the reader now enforces; the ADR set, the specs, both glossaries,
  the skills and the user guide state the new grammar as the only one, with no
  naming history anywhere; `scripts/check.sh` is green, every touched book
  included.
- A release that reads the new grammar is installed on this machine, the
  plugin with it, and every grove under `~/Development` — this one included —
  opens under it.

## Decomposition

The design at `node-grammar-k2` was reviewed at `node-grammar-k4` and integrated
at `node-grammar-k5`. The planning artifact is `node-grammar-k3`; its review
`node-grammar-k12` precedes implementation. The interview is settled, not a new
session's input to re-interview.

The working increments stay in this grove as agreed in `plan-k1` decisions 7
and 10. Positions encode these dependencies:

- `distinguished-names-k6` — the library's supplied-name API, level validation
  and conformance, with every necessary caller adaptation. Grove's filename
  behavior stays consistent through this boundary.
- `node-files-k7` — Grove's complete grammar, handles, readers, writers,
  lifecycle and fixtures together; no dual-grammar reader.
- `node-methodology-k8` — the shipped skills and their conformance teaching
  the implemented grammar; installed skills wait for cutover.
- `node-documentation-k9` — the guide and remaining prose-only documentation
  and book examples, ready to release.
- `node-cutover-k10` — the last code/release step, with the human stop,
  publication, binary and plugin install, this-tree rename and restart handoff.
  It ends without a relaunch signal.
- `grove-migration-k11` — the remaining four known groves, plus newly
  discovered ones, verified after installation; this tree is checked again.

Each code increment owns every book whose source it changes in the same
commit; there is no book-repair leaf. Every repository-source increment runs
`bash scripts/check.sh`. Library behavior, Grove fixture behavior, plugin
conformance, checked guide examples and actual installed-tree reads are the
independent demonstrations. Each producer judges review lazily; any earned
review and integration must run before its dependent or the release handoff.

## Pointers

- The interview's decision log: `plan-k1`, ten decisions, each with the
  human's words and the evidence measured before it settled.
- ADRs to read: `docs/adr/task-names-are-canonical.md` (the record that fixes
  the directory spelling; rework in place, drop its history),
  `docs/adr/iteration-reuses-the-existing-species.md` (its "sibling node
  directories sharing one slug" is now read off the `_` files),
  `docs/adr/entry-name-is-the-only-seam.md`,
  `docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md`.
- Specs: `docs/specs/module-decomposition.md` decisions 3 and 4 (the grammar
  and handle ownership); `docs/ordinal-fs-tree/ARCHITECTURE.md` with its
  `models/structure.als` and `models/operations.qnt`. `ReaderAccepts` is a
  definition, not an independently checked cardinality property;
  `inv_successHasValidLevels` checks executed-plan preservation, with limits
  and mutation-control evidence in `docs/formalism-findings.md` entry 049.
  `docs/specs/walkthrough-books.md` owns source/book synchronization.
- Glossary terms in play: Node directory, Work-item handle / title, Permanent
  key, No migration, Taskless root (`CONTEXT.md`); Distinguished child, Label
  (`docs/ordinal-fs-tree/CONTEXT.md`); the collision table in
  `CONTEXT-MAP.md`. The term to land: the node's own file, `_<slug>.md`.
- Code entry points: Grove's `TaskName` and `Handle` own name composition;
  its tree module supplies a node's actual file from the guarded snapshot.
  The library's `EntryName` owns distinguished-name validation; the reader and
  projected planner enforce it, and the conformance kit samples its laws.
  Follow all API and fixture consumers rather than treating pointers as a
  complete file list.
- Books: the manifests under `docs/walkthroughs/` name the roots each book
  reconstructs byte for byte — `grove-loop` (`task_name.rs`, `task_tree.rs`,
  `task_grow.rs`, `tree_lifecycle.rs` among them), `ordinal-fs-tree`
  (`name.rs`, `ops.rs`, `conformance.rs`), `grove-llm` (`cli.rs`) and
  `overview`. A source change lands with its fragments and prose in one
  commit, `scripts/check.sh` green.
- Test seams: existing seams only, agreed with the human in `plan-k1`
  decision 9 — the library's models then its conformance kit over grove's
  `TaskName`; `task_name.rs` unit tests; `grove-llm` black-box tests over
  fixture trees; `scripts/check.sh` for the books; the migration checked by
  opening each tree with the new binary.
- Install routes: `grove` and `grove-llm` are Homebrew
  (`/opt/homebrew/Cellar/grove/20.2.0`), the plugin is a marketplace cache keyed
  by commit. Neither is a symlink to this checkout, so nothing lands in the
  loop driving this grove until a release is cut and installed
  (`docs/RELEASING.md`).

## On the horizon

- Other node metadata in the `_` file — the human raised YAML front matter.
  What would go there is not yet statable. Whatever grove would *route* on
  stays in filenames (constraint 1); front matter is for what humans and
  sessions read.
- Entries that start with something other than a digit or `_`. The partition
  today is the human's "every other file will start with a digit (for now at
  least)".

## Notes

- **The requirement, in the human's words.** "The problem is that deep trees
  exceed the path length limit. I want to remove the title from the folder
  names, and instead put it in the BRIEF file name, with an underscore at the
  start of that filename to ensure it sorts before all the other contents."
  And: "We therefore *must* have a BRIEF in every folder."
- **The evidence.** In `~/Development/APIAnyware.add-ocaml-target/.grove` the
  deepest leaf path is 1003 characters, 26 levels down, against macOS's
  1024-byte `PATH_MAX`; the LLM driving it renamed nodes to single-letter
  titles (`05-c-k581`, `07-n-k765`) to proceed. Under the new grammar that
  path is 302 characters at the same depth.
- **The sort claim, measured.** `_` sorts first in Finder, VS Code and `ls`
  under `en_US.UTF-8`, and last in byte-order listings (jj, git, `find -s`,
  C-locale `ls`). The human chose `_` knowing that; grove itself never orders
  the `_` file.
- **No naming history.** Docs, ADRs, glossary entries, skill text and code
  comments state the grammar as it now is. Nothing records the old spelling or
  narrates the rename; the VCS holds the past.
- **Migration is workflow, not product.** *No migration* stands: no `migrate`
  verb, no automatic conversion, no second grammar in the reader. A script run
  once per tree — one rename per node directory, one per brief — over the five
  groves under `~/Development` (`APIAnyware.add-ocaml-target`, `InTheLoop`,
  `Writegood`, `grove.gh-issue-12`, this one), each verified by opening it
  with the new binary. `grove.gh-issue-12/.grove/FORMAT` is a foreign leftover;
  the migration decides whether to remove it.
- **The meta-grove's sequencing hazard.** The driver running this loop is the
  installed 20.2.0 binary, in memory for the loop's life, and the skills it
  names are the cached plugin. The session that renames this repo's own
  `.grove/` onto the new grammar must be the one after which the loop is
  stopped, the release installed, the plugin updated, and `grove` re-run
  (restart ≡ continuation) — `grammar-separator-k15` did exactly this in the
  same session as the release that could read it. That is a human moment.
- Until the plugin is updated, sessions in this loop read skill text that
  spells `BRIEF.md`; the verbs, not the sessions, write briefs, so the risk is
  confusion rather than corruption.
- The shared installed binaries meet every active grove. Cutover prepares
  converted scratch copies for new-reader preflight and coordinates stopping
  the other drivers before installation; those groves stay stopped until
  migration verifies them. A stopped cutover leaves its handle live until
  release, installation and this-tree verification are actually complete.
