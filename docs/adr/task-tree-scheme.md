# The task tree is real directories with stable-keyed names

grove's task tree is a real directory tree under `.grove/`. The uniform rule:

> A node is a **directory** holding its numbered children, optionally headed by a
> `BRIEF.md` charter. The grove root `.grove/` is itself such a directory.

The hierarchy is carried by the filesystem — there is no flat dotted-decimal position
baked into filenames and no special root-brief sentinel.

```
.grove/
  BRIEF.md                       ← root brief (plain; heads .grove/)
  01-DONE-plan-k1.md             ← retired leaf
  05-dotted-decimal-k5/          ← decomposition node — carries a charter
    BRIEF.md
    01-DONE-id-model-k6.md
    04-DONE-lifecycle-k9.md
  07-distribution-k14/
    BRIEF.md
    04-DONE-install-k18.md
    05-remove-mirrors-k19.md     ← live leaf
  08-shed-tui-k20/               ← chain node — brief-less by rule
    01-DONE-shed-tui-k21.md
    02-shed-tui-review-k22.md    ← live leaf
    03-shed-tui-integrate-k23.md
  10-complete-signal-k24.md
```

**The charter is what distinguishes the two species of node.** A *decomposition*
node — written by `leaf-decompose` — always carries a `BRIEF.md`; it means *this
work proved bigger than one session*, and the charter is the context those extra
sessions need. A *chain* node — written by `leaf-add-chain` / `leaf-add-pair` for a
review chain or a vendor pair — never carries one; it means *these steps compose
one artifact*, a shape declared whole at construction. The discriminator is the
**presence of the file**, never a pattern in the name, which is what lets the
Retire cascade tell them apart without any reader parsing the step-suffix
convention (*task-kind-taxonomy*, which owns the chain decision). Nothing enforces
either species: a `BRIEF.md` written into a chain node simply makes it
brief-carrying.

## Naming grammar

Fields are ordered by human relevance — sort key first, user-facing state next,
machine handle last:

- **Leaf:** `NN-[DONE-|ABANDONED-]<slug>-k<key>.md`
- **Node:** directory `NN-<slug>-k<key>/` containing children, optionally headed by `BRIEF.md`
- **Root brief:** `.grove/BRIEF.md` (plain, unkeyed — the root dir's charter)

Three parts do the work:

1. **Position `NN`** — a 2-digit zero-padded decimal, *per level* (not a global
   dotted vector). It is the **mutable locator**: lexical == numeric == DFS order
   within a level, locale-robust because it is pure digits, and gives ~99
   siblings/level. A node's full position is its directory path; reorder/insert
   renames only the affected *level's* sibling directories (`07-…-k14/` →
   `08-…-k14/` — a `git mv`, or a plain rename in a jj-enabled tree), and the
   whole subtree — child names and keys — rides along untouched.
2. **Key `-k<key>`** — the **permanent stable id**. Assigned once (`max key in tree +
   1`), never rewritten by renumber or decompose, always the terminal token before the
   extension or trailing slash. The keys in the names *are* the counter — there is no
   counter file, and **every finished leaf stays in the tree, `DONE` or `ABANDONED`
   alike**, so the max is always visible. That is why abandonment is marked rather
   than deleted: removing a name lowers the max and the next `leaf-add` re-issues a
   live key (*pruning*).
   References resolve by key: `resolve [k]` / `resolve <slug>` finds the current path
   wherever the entity moved. The key is written `-k<key>` rather than the bracketed
   `[<key>]`, because brackets are shell-glob metacharacters that force escaping in
   `ls` / `cd` / copy-paste; `-k<key>` is glob-safe and stays unambiguous because the
   key is mandatory and terminal (parse peels the trailing `-k<digits>`, so
   `05-task-k9-k3.md` is slug `task-k9`, key `3`).
3. **Outcome** is marked in place as an infix right after the position, **leaves
   only**: `DONE` for work completed, `ABANDONED` for a path decided against
   (*pruning*). Both are skipped by `pick`; neither is ever removed from the tree. A
   node is never marked — node done-ness is implicit (no live leaf in its subtree),
   however its leaves finished. The infix sits at a fixed column per mark (position is
   fixed-width, the variable-width `-k<key>` is exiled to the end), so a directory's
   outcome column scans cleanly.

A task file's in-file `# …` header is the **position-free handle** `# <slug>-k<key>`
(`# <slug>-k<key> — brief` for a node): the mutable `NN` lives only in the filename,
never in the body. That is what makes a renumber a pure file move with **zero content
rewrites**.

## Reference a work item by its stable handle

Commit messages and prose name a work item by `<slug>-k<key>` — never by its position
or directory path. The position/path is mutable (renumber, move); the `<slug>-k<key>`
handle is stable, so the historical record stays meaningful after restructures.

**A slug therefore has to carry meaning on its own, and stay unique tree-wide.**
Two facts make that a constraint rather than a preference. `.grove/` is deleted at
the finish cycle, so a commit message is the *only* surviving record of a work
item — a handle that names a role rather than a subject (`review-k4`) says nothing
once the tree is gone. And `resolve` matches a bare slug **exactly**, reporting
more than one match as ambiguous, so slugs that repeat per group degrade the
reference surface as the tree grows. This is what decides how a node's children
are named when the node itself supplies the context: they keep the qualifying stem
(`skill-review-k4`, not `review-k4`), and the node takes a distinguishing token so
its own slug does not collide with its first child's. The redundancy in a listing
is the price of the handle staying self-describing after the tree that explained it
no longer exists.

## Comparator and verbs

The comparator is numeric per-level order, so a node's brief heads its own subtree and
the DFS pre-order is the sort order. The verbs express this against the directory
structure: `pick` is a recursive DFS walk returning the first live leaf; `brief-chain`
walks parent directories collecting each `BRIEF.md`; `resolve` searches directories by
key; `leaf-decompose` turns a leaf file into a node *directory* (keeping its key);
`leaf-retire` adds the `DONE` infix in place; and `leaf-insert` moves sibling
directories (`git mv`, or a plain rename in a jj-enabled tree), subtrees riding
along.

### `pick` is a walk, not a scheduler

`pick` returns **the first live leaf in pre-order, and nothing modulates that** —
no priority, no grouping, no set of leaves that must be finished before another is
considered. Ordering in a grove is **contiguity**, at every level, and it is the
only ordering grove offers.

What that buys is that **a human computes `pick` by eye**: `find .grove` shows the
whole tree, and the next session is the first name in it carrying no outcome infix.
That legibility is what makes "the directory tree is the only state" (constraint 1)
worth something rather than merely true.

Any rule that groups leaves — *"once this review chain starts, finish it before
anything else"* — has to answer **is a group in flight?**, and both ways of
answering are blocked:

- **From memory** — which leaf retired last. That is loop state living outside the
  tree, which constraint 1 forbids; and `pick`'s statelessness is precisely what
  makes restart ≡ continuation (*self-driving-loop*).
- **From structure** — a run of related leaves holding both a `DONE` member and a
  live one. Computable and stateless, but it must then **skip a live leaf that
  sorts earlier**, and with two such runs in flight it must rank them. `pick` stops
  being a walk and becomes a priority scheduler, whose answer no reader of
  `find .grove` can predict.

The same principle already settles two other questions. An outcome taxonomy
carrying a `blocked` mark is rejected because a blocked leaf is *live work* and
`pick` must not skip it (*pruning*); and the review chain is not a first-class unit
(*task-kind-taxonomy*, whose *Considered options* holds the full costing).

The accepted cost: contiguity is **unprotected against a sibling-level
`leaf-insert`**, which can split a run of related leaves, where a node directory's
children are protected by containment. That asymmetry stands — `leaf-insert` names
its target explicitly and exists so a human can preempt, and a split is repaired by
one more `leaf-insert`.

It no longer bites the case it was written for. A review chain and a vendor pair
are **node directories** (*task-kind-taxonomy*), so their steps are children and a
sibling-level insert cannot land between them. Note *how* that gap closed: not by
teaching `pick` about groups — it is the same stateless local walk, and it leaves a
chain node for the next sibling as freely as it leaves any node — but as a
by-product of the grouping being structural. The residual exposure is a run of
related leaves a human left flat on purpose, which is exactly the case where
`leaf-insert`'s explicitness is the right answer.

## Rationale

- **Honesty.** The parent/child relationship is the filesystem's job; directories
  state it directly, dissolving the false-sibling problem of a positionless root brief
  and the numbering tricks a flat encoding forces (`00` sentinels, low-ASCII prefixes,
  a `tree/` subdir).
- **Cheap restructure.** Moving or inserting a node is a single move of its
  directory; the subtree and all keys come along. A flat filename encoding paid
  O(subtree) renames for the same edit, because each leaf baked its full path into its
  name. Insertions and reorderings are first-class operations, so this matters.
- **Native navigation.** `ls` / yazi / Finder render a real collapsible indented tree;
  `find .grove` shows complete state — shape plus `DONE` — with zero file reads.
- **Locale- and filesystem-robust.** The environment is case-insensitive (`1A`/`1a`
  collide, so any base-36/62 letter-as-digit scheme is unsafe) and only low-ASCII
  digits collate predictably across locale, C, and natural/version sort. Pure 2-digit
  decimal per level needs no letters and no punctuation-collation gamble; it sorts
  identically under byte, locale, and natural sort.

### Prior art

- **Adopted:** outline / legal section numbering (renumber-on-insert with DFS
  pre-order); ASN.1 / OID object identifiers (a dotted hierarchical id ordered by
  per-component numeric compare); the materialized-path tree encoding (flat storage of
  a tree whose lexical/version sort yields pre-order); and natural / version sort
  (`sort -V`, SemVer precedence) for the comparator.
- **Rejected — and why renumber-on-reorder is accepted:** LexoRank (Jira) and the
  fractional-indexing family (Figma; the CRDT list-position schemes Logoot / LSEQ /
  Treedoc) allow insertion *without* renumbering, but produce a **flat total order,
  not a tree**, with **illegible keys**. grove pays renumber-on-reorder to keep
  legible integers that *are* the tree and that a file manager renders. Reorder is
  rare; append and decompose — the common ops — stay cheap.

Two supporting choices:

- **Integer key, not a uuid.** A grove tree is single-worktree, single-writer, so a
  uuid's decentralized-generation property buys nothing while costing legibility (`k4`
  readable and ordered vs an opaque hash).
- **Filename `DONE` marker, not frontmatter.** Keeping the entire state in the filename
  means `pick` reads no file contents and `ls` shows done-ness. Frontmatter would force
  `pick` to open and parse every file and hide done-ness from `ls`.

## Migration: `grove do` migrates on adoption

The live verbs are **current-format-only** — there is no transitional dual-format
reader threaded through every verb. Instead, `grove do` **migrates an old-format
`.grove/` on adoption**, before driving:

- The migration is a **reviewable single commit** (git-authored, or jj-authored
  via `jj commit .grove` in a jj-enabled tree), **idempotent** on a
  current-format tree, and **fixture-tested hard before it ever touches a real tree**
  (a migration bug could corrupt a live tree; the single reviewable commit is
  revertable).
- `grove migrate` also exists as an explicit human verb (see
  *do-is-sole-lifecycle-verb* for the lifecycle surface).
- The migration reads **two** legacy shapes: the v1-flat
  `<dotted>-[<key>]-<slug>` filenames, and the original `NNN-slug/` directories with a
  `done/` subdirectory. Old-format code shrinks to the parser the migration consumes
  once.

Migrate-on-adoption is strictly less code than a dual reader plus a migration, and it
avoids a bimodal, flagless CLI spanning two argument grammars. Its whole value — keep
in-flight groves working — is delivered by a one-time, reviewable rename, with no
permanent two-format surface. Because `grove do` re-derives state every run
(restart ≡ continuation; see *self-driving-loop*), a re-run after a partial migration
is safe.

## Consequences

- The scheme is part of the self-extension core (see
  *self-extension-core-and-methodology*); the grove methodology prose carries the
  reference-by-handle instruction, and because the global skill is binary-embedded and
  re-extracted on launch, that prose ships with the binary.
- Reversible in principle (the scheme lives in names plus VCS history), expensive once
  trees migrate — it clears the ADR bar as a durable trade-off.
