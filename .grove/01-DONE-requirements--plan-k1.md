# plan-k1

## Goal

Establish, in the human's words, what `dont-put-item-title-in-folder-names`
asks for: a change to the task-tree grammar so that a node directory's name no
longer carries the work item's title (its slug), because deep trees exceed the
path length limit. Decide the shape of the replacement and grow the tree that
delivers it.

## Context

- `docs/adr/task-names-are-canonical.md` — fixes `NN-<slug>-k<key>` as the node
  directory spelling; any change reworks this record in place.
- `crates/grove-loop/src/task_name.rs` — the one renderer/parser of the grammar
  (grove's `EntryName` implementation); `docs/walkthroughs/grove-loop/04-the-name.md`
  reconstructs its bytes exactly, so a grammar change re-opens that book.
- `docs/ordinal-fs-tree/CONTEXT.md` *Distinguished child* — the library's word
  for `BRIEF.md`: a regular file carrying neither ordinal nor key, never
  ordered, never descended into.
- Two trees are in flight on the current grammar: this one and
  `../grove.gh-issue-12/.grove/`. Grove converts no tree (`No migration`), so
  the change is a cutover as `grammar-separator-k15` was.

## Done when

- The requirement is stated in the human's words in this file's running log and
  promoted to the root `BRIEF.md`.
- The open questions are either settled here or cut as leaves a fresh session
  can pick up.

## Notes

## Decisions (running log)

**1. The subject, and why.** "Item title" is the work item's slug; "folder
names" are node directories `NN-<slug>-k<key>/`. The motivation is concrete:
**deep trees exceed the path length limit**. The change: remove the title from
the folder name and carry it in the brief's *filename* instead, with a leading
underscore so the brief sorts before every other entry in its folder. (Human,
verbatim: "The problem is that deep trees exceed the path length limit. I want
to remove the title from the folder names, and instead put it in the BRIEF file
name, with an underscore at the start of that filename to ensure it sorts
before all the other contents.")

**2. Every folder must therefore hold a brief.** With the title on the brief's
filename, a node without a brief has no title, so the brief becomes mandatory
in every node directory. The alternative the human named — a dummy file that
carries the name and nothing else — is the rejected option: the two writers of
nodes (`leaf-decompose`, `root-init`) already write a brief every time, so
mandatory costs nothing on grove's own paths and only a hand-made node can
violate it. (Human: "We therefore *must* have a BRIEF in every folder. The
alternative is a dummy file with the name.")

**3. The brief's filename leads with `_`, and the folder keeps position and
key.** The node directory becomes `NN-k<key>/`; the title moves to the brief's
filename, which starts with `_` so that metadata is distinguished from
children by its first character — every child starts with a digit. Measured
before deciding: `_` sorts first in Finder (`localizedStandardCompare`), VS
Code (`Intl.Collator`) and `ls` under `en_US.UTF-8`, and last in byte-order
listings (`LC_ALL=C ls`, `find -s`, jj and git file lists). `+` and `,` are the
only characters that sort first in both regimes without a shell, CLI or
hidden-file hazard; `00-` sorts first everywhere but reads as a peer of the
children. The human rejected `00-` for that reason — "we want to distinguish
metadata in general from children" — and chose `_`, accepting the byte-order
cost; grove itself never orders the brief. The human also raised that the `_`
file may later carry other node metadata (YAML front matter). Recorded as a
horizon note: anything grove would *route* on must stay in filenames
(constraint 1), so front matter is for what humans and sessions read.

**4. The `_` file is `_<slug>.md` — the file *is* the node.** Its name is the
node's title, its body the brief (and, later, any front matter). So a node is
`NN-k<key>/_<slug>.md` where a leaf is `NN-<kind>--<slug>-k<key>.md`:
`.grove/09-k12/_pilot.md`. Rejected: `_BRIEF-<slug>.md` (longer, and "BRIEF"
under-describes a file that also carries metadata) and `_<slug>-k<key>.md` (a
second copy of the key, free to disagree with the folder's — the property this
repo removed at `drop-step-suffix-k18` and again at `name-ownership-k14`). The
handle `<slug>-k<key>` is now composed from two filenames — the folder's key
and the `_` file's slug — and is still read from names, never from contents.
The human also stated the partition this rests on: "Every other file will start
with a digit (for now at least)" — so `_…` is the node's own file, `NN-…` a
child, anything else foreign. The leaf grammar is unchanged by this grove.

**5. The root's file is `_BRIEF.md`, fixed.** The root is not an entry and has
no slug; its title is the grove name, which comes from the working tree, is
not a slug (`grove.dont-put-item-title-in-folder-names`), and stays in the
header `# <grove name> — brief` as today. `BRIEF` is already a reserved token,
so `_BRIEF.md` can never collide with a node's `_<slug>.md`. Rejected:
`_<grove name>.md` verbatim (a second, looser rule for one file, stranded by a
workspace rename) and `_root.md` / `_grove.md` (a new word for no gain).

**6. No naming history in the artifacts.** Docs, ADRs, glossary entries, skill
text and code comments state the grammar as it now is; nothing records the old
spelling or narrates the rename. The reworked `task-names-are-canonical` record
drops history rather than adding to it. (Human: "We don't need to record the
old structure or show the history of naming in this project docs or
comments.")

**7. Migration is this grove's workflow, not the product.** Every grove under
`~/Development` is renamed onto the new grammar (each node directory to
`NN-k<key>/`, each `BRIEF.md` to `_<slug>.md`, the root's to `_BRIEF.md`) by
a leaf of this tree, run once, when the binary that reads the new grammar is
installed. Nothing ships: no `migrate` verb, no automatic conversion in bare
`grove`, no second grammar in the reader. *No migration* stands, and an
un-migrated tree halts loudly at its first node name with the refusal naming
the canonical form. (Human: "We need to migrate all the groves in
~/Development, but that is not part of the published grove tooling or
codebase - it's part of this grove's workflow.")

**8. The limit is macOS `PATH_MAX` (1024 bytes), and it was reached.** In
`~/Development/APIAnyware.add-ocaml-target/.grove` the deepest leaf path is
1003 characters, 26 directory levels down, and the tree carries the workaround
the human described: the LLM had to rename node directories to single-letter
titles (`05-c-k581`, `07-n-k765`, `04-j-k777`, `07-t-k788`) to proceed. Windows
(260) would have failed far earlier. So `Done when` is structural — no folder
name carries a slug — and the check is that every existing tree, migrated,
opens under the new grammar with room to grow. (Human: "MacOS - in
APIAnyware.add-ocaml-target. The LLM had to rename directories by changing
titles to a single character in order to proceed. It would be even worse
under Windows.")
Measured: that 1003-character path becomes 302 characters under the new
grammar, at the same 25 node levels.

**9. The test seams, agreed.** Existing seams only; no new one. (1) The
`ordinal-fs-tree` models lead — `structure.als` and `operations.qnt` are
re-drawn first, because *at most one distinguished child per node* stops being
a theorem the filesystem supplies (one fixed name per domain) and becomes an
obligation the reader enforces (a level with two `_` files, or none, is
malformed); then the library's conformance kit is run over grove's `TaskName`.
(2) `task_name.rs` unit tests for the grammar. (3) `grove-llm` black-box tests
over fixture trees for the verbs: `leaf-decompose` writes
`NN-k<key>/_<slug>.md`, `resolve <slug>` finds a node by its `_` file,
`brief-chain` prints it, `pick` and `kind` skip it, `root-init` writes
`_BRIEF.md`. (4) `scripts/check.sh` — every book whose root changed is green
in the same commit as the source change (`docs/specs/walkthrough-books.md`,
*An accepted source change*). (5) The migration is checked by opening each
migrated tree with the new binary, not by a unit test.

**10. The tree: `design`, then `planning`.** Not a one-sitting job: the
library's obligation and models change, the grove grammar and every verb over
it change, four walkthrough books reconstruct the touched roots byte for byte,
the skills and docs spell `BRIEF.md` in fifty-four files outside the books,
this repo's own tree must be cut over in the same session as the binary that
reads it, and four other groves are migrated after that. The `design` leaf
reworks the design records for both contexts (the library's architecture and
models; `task-names-are-canonical`; the glossary) with no history; `planning`
consumes it and cuts the `impl` leaves. The design session decides its own
review chain lazily, and inserts it ahead of `planning` if it cuts one.
