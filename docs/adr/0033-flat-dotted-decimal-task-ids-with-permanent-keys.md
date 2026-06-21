# 33. Task ids are flat dotted-decimal positions with permanent keys

- Status: **accepted** (decided in `refactor-to-archon` 050-dotted-decimal-numbering;
  built by its child leaves 010–040, integrated + migrated by leaf 060)
- Date: 2026-06-21
- Deciders: Antony Blakey (with grove `refactor-to-archon` 010-plan D4/D5 +
  050-dotted-decimal-numbering grilling D1–D9)
- Pairs with: **ADR-0031** (the self-extension core this scheme is part of). The
  full grilling record is the 050 BRIEF's running log (D1–D9).

## Context

grove's task tree is today a directory hierarchy: a node is a directory
`NNN-slug/` holding a `BRIEF.md` plus child leaves `NNN-slug.md`; retired leaves
move into a `done/` directory; `pick` walks depth-first relying on the
*coincidence* that zero-padded `NNN` prefixes sort identically lexically and
numerically. 010-plan D4/D5 set the direction to replace this with a **flat**
namespace of dotted-decimal ids (legible sequential integers, not fractional /
LexoRank keys), ordered by a numeric per-segment comparator, with renumber-on-
reorder accepted and done-ness marked in place (no `done/` directory). This ADR
settles the exact scheme; the open mechanics were grilled in leaf 050.

## Decision

A task tree is a **flat list of files in `.grove/`** (no node directories). Each
filename has three orthogonal parts plus optional markers:

```
<position>-[<key>]-<slug>[.BRIEF|.DONE].md
```

- **position** — a dotted integer vector (`2.3.1`): *where* the task sits in the
  DFS pre-order. The node `2.3` carries `2.3-[k]-slug.BRIEF.md`; its children are
  `2.3.1-…`, `2.3.2-…`. Position is rewritten on renumber.
- **permanent key** — a bracketed integer (`[4]`): *stable identity*. Assigned
  once at creation (next key = max `[n]` in the tree + 1), **never changed** — not
  by renumber, not by decompose. The keys in the filenames *are* the counter
  (constraint 1); `.DONE` files stay in the tree, so the max is always visible and
  keys are never reused. There is no counter file.
- **slug** — a human label (`add`); `[a-z0-9-]`, may contain `-`, never `.`/`[`/`]`.
  `BRIEF` and `DONE` are reserved.
- root brief is the one unkeyed singleton: `BRIEF.md`.

**Comparator** (the sort key is the **position vector only** — key, slug, markers
excluded): element-wise integer compare (`[2,9] < [2,10]`), with a shorter prefix
sorting before any vector that extends it (`[2,2] < [2,2,1]`), so the flat sorted
list *is* the DFS pre-order and a node's brief heads its own subtree. Foreign /
malformed files are not leaves: `pick` ignores them and they sort last (no error).

**Done is marked in place, leaves only.** A retired leaf appends `.DONE`
(`2.1-[3]-model.DONE.md`); **a brief is context, not a task, so it is never marked
done** — node done-ness is *implicit* (no live leaf in its subtree). This makes
the inconsistent state "a live leaf under a `.DONE` brief" unrepresentable.
`pick` is a pure per-file filter (skip every `.BRIEF`, skip every `.DONE`, return
the first live leaf in sort order); it never reads or reasons about briefs.
`ls .grove/` therefore shows the *complete* state — tree shape and done-ness —
with zero file reads.

**References resolve by the permanent key**, not the position. Written `[n]` (or
`[n]-slug` for readability), a reference is stable under both renumber (position
moves, key doesn't) and slug edits. `grove-llm resolve <ref>` turns a reference
into the current file path (`[4]`/`4` → the unique keyed file, live or `.DONE`; a
bare slug → 0/1/many, listing matches with their keys so the caller re-queries by
key). This dissolves the cross-reference-rewrite problem rather than solving it:
durable references never need rewriting.

**Verbs** (id-addressed, two clean positionals; all working-tree-only, no commit):
- `leaf-add <parent-id> <slug>` → child at `<parent-id>.<next>` with a fresh key
  (root parent `.`).
- `leaf-insert <target-id> <slug>` → new leaf at `<target-id>`, shifting the
  occupant and later siblings up by one; the renumber **cascades through whole
  subtrees** (bumping `2.2`→`2.3` also rewrites `2.2.1`→`2.3.1`…), highest-first,
  rewriting only the **position** in filenames and `# …` headers — never the key —
  and surfacing stray *positional* cross-references as a lint.
- `leaf-decompose <leaf-path> <first-child-slug>` → `git mv` leaf to
  `…-slug.BRIEF.md` (**preserving its key** — the entity that was `[3]` becomes
  *node* `[3]`), retitle, and create the first child `…1-[new]-slug.md` atomically
  (every node always has ≥1 child — ADR-0011 generalised).
- `leaf-retire <leaf-path>` → append `.DONE` (refuses a brief / an already-done
  leaf).
- `root-init [<slug>]` → root `BRIEF.md` + first leaf `1-[1]-<slug>.md`.
- `pick` / `brief-chain` / `resolve` as above.

## Rationale

- **Prior art — adopted:** outline / multilevel-list / legal section numbering
  (renumber-on-insert + DFS pre-order), ASN.1 / OID object identifiers (dotted
  hierarchical id ordered by per-component numeric compare), the materialized-path
  tree encoding (flat storage of a tree; lexical/version sort → pre-order), and
  natural / version sort (`sort -V`, Debian `dpkg --compare-versions`, SemVer
  precedence) for the comparator.
- **Prior art — rejected (and why renumber is accepted):** **LexoRank** (Jira)
  and the **fractional-indexing** family (Figma; the CRDT list-position ids
  Logoot / LSEQ / Treedoc) allow insertion *without* renumbering, but produce a
  **flat total order, not a tree**, with **illegible keys**. grove pays renumber-
  on-reorder to keep legible dotted integers that *are* the tree and that `ls`
  renders (constraint 6). 010-plan D5 already rejected these by name. Reorder is
  rare; append + decompose (the common ops) stay cheap.
- **Integer key, not uuid:** a grove tree is single-worktree / single-writer, so
  a uuid's decentralized-generation property buys nothing while costing legibility
  (`[a3]` opaque/unordered vs `[4]` readable/ordered).
- **Always-present key, not on-collision:** a uniform stable handle on every task,
  robust to renumber *and* slug edits, which also removes any slug-uniqueness
  requirement. Cost accepted: minor filename noise; the bracket disambiguates the
  position number from the key.
- **Filename `.DONE` marker, not frontmatter:** keeps the *entire* state in the
  filename (constraint 1) — `pick` reads no file contents, `ls` shows done-ness.
  Frontmatter would force `pick` to open and parse every file and hide done-ness
  from `ls` (a regression from the old done-by-absence).

## Consequences

- **Transition (ADR-0031's dogfood):** this grove is itself an old `NNN-slug/`
  tree, so 050 builds the new scheme as **new, separately-tested code without
  touching the live verb path** (every commit stays safe); leaf 060 adds the
  transitional dual-format reader + a one-time `grove migrate` + the old-format
  drop-plan, and the methodology prose / format guides move to the new scheme when
  it goes live in 060.
- `resolve` is a new (eighth) verb; `leaf-add`/`leaf-insert` change from
  path/`--node` addressing to id positionals; `leaf-decompose` gains a first-child
  argument; the `--prefix` escape hatch disappears (the scheme renumbers instead
  of leaving +10 gaps).
- Reversible in principle (the scheme lives in filenames + git history) but
  expensive once trees are migrated; it clears the ADR bar as a durable trade-off.
