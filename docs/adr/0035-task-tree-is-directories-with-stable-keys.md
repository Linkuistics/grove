# 35. The task tree is real directories with stable-keyed names — reversing ADR-0033's flat structure

- Status: **accepted** (decided in `refactor-to-archon`, dogfooding the 070/040 flip
  on a real tree, ADR-0031)
- Date: 2026-06-22
- Deciders: Antony Blakey (with the post-flip grilling session)
- Supersedes: the **structure** of **ADR-0033** — its "flat list of files in
  `.grove/`" with dotted-decimal positions encoded into every filename — and the
  in-flight 2-digit-flat / `00`-root / `tree/`-subdir refinements explored during
  this grilling. **Keeps** ADR-0033's durable ideas: the permanent key,
  reference-by-key (`resolve`), the numeric comparator, renumber-on-reorder, and
  done-ness marked in place (no `done/` directory).
- Amended (2026-06-22, while building the id model — leaf `id-grammar`, key 24): the
  permanent key is written **`-k<key>`**, not the `[<key>]` this ADR first drafted
  (see §1's *key delimiter* note for the rationale).
- Amended (2026-06-22, while building the grow/lifecycle verbs — leaf
  `grow-lifecycle-verbs`, key 26): a task file's in-file `# …` header is the
  **position-free handle** `# <slug>-k<key>` (`# <slug>-k<key> — brief` for a node) —
  the mutable per-level position `NN` lives only in the filename, never in the body.
  This is the body-level corollary of §5 (reference by stable handle, never by
  position) and is what makes the renumber a pure `git mv` with **zero content
  rewrites** (§2's "cheap restructure"). The v1→v2 migration rewrites old
  `# <dotted>-[<key>]-<slug>` headers down to this handle.
- Pairs with: **ADR-0031** (the self-extension core), **ADR-0034** (migrate-on-
  adoption — the mechanism this change rides to flip live trees).

## Context

070/040 migrated this grove to ADR-0033's flat scheme, and seen on disk the flat
encoding showed its seams:

- **Lexical ≠ DFS without padding.** `10-[22]-…` sorted *before* `2-[2]-…` in the
  real `ls` (lexical `"10" < "2"`), so human file managers (`ls`, yazi, `git
  status`) mis-ordered the tree even though grove's own comparator did not. Fixing
  it meant zero-padding positions purely for the dumb external sort.
- **The root brief has no honest flat home.** It is the one element with no
  position number, so any in-list representation (`BRIEF.md` sorting last, or a
  `00-BRIEF.md` sentinel) makes it *look* like a sibling of the top-level nodes
  when it is in fact their parent — the one file whose visual "siblings" are
  really its children.
- **Insert/reorder cascades.** Inserting at `2.2` renames `2.2`→`2.3` **and**
  every descendant `2.2.1`→`2.3.1`… — O(subtree) filename rewrites, because the
  full path is baked into each leaf's name.

These are all symptoms of encoding a *tree* in *flat filenames* — fighting the
filesystem instead of using it. Dogfooding (ADR-0031) surfaced this: the flat
scheme was sound in grove's own comparator but awkward for the humans who read the
tree in a file manager.

Filesystem facts established by probing the real environment (macOS, APFS,
`en_AU.UTF-8`): the filesystem is **case-insensitive** (`1A.md`/`1a.md` collide —
so any base-36/62 letter-as-digit scheme is unsafe), and only **low-ASCII digits**
collate predictably across the locale, C, and natural/version sort — letters and
high punctuation (`_`, `~`) do not. Pure zero-padded decimal positions are the
only locale-robust encoding.

## Decision

**A node is a directory; `.grove/` is the root node.** The uniform rule:

> A node is a directory holding a `BRIEF.md` (its charter) plus its numbered
> children. The grove root `.grove/` is itself such a directory.

There is no flat dotted position and no special root brief. The hierarchy is
carried by the filesystem.

```
.grove/
  BRIEF.md                       ← root brief (plain; heads .grove/)
  01-DONE-plan-k1.md             ← retired leaf
  05-dotted-decimal-k5/          ← node = directory; key rides in the dir name
    BRIEF.md
    01-DONE-id-model-k6.md
    04-DONE-lifecycle-k9.md
  07-distribution-k14/
    BRIEF.md
    04-DONE-install-k18.md
    05-remove-mirrors-k19.md     ← live leaf
  08-shed-tui-k20.md             ← live leaf
  10-complete-signal-k22.md
```

1. **Naming grammar**, fields ordered by *human relevance* (sort key first,
   user-facing state next, machine handle last):
   - **Leaf:** `NN-[DONE-]<slug>-k<key>.md`
   - **Node:** directory `NN-<slug>-k<key>/` containing `BRIEF.md` + children
   - **Root brief:** `.grove/BRIEF.md` (plain, unkeyed — the root dir's charter)
   - **Key delimiter** — the permanent key is written `-k<key>` (resolved
     2026-06-22 building leaf `id-grammar`, key 24; this ADR first drafted `[<key>]`).
     Brackets are shell-glob metacharacters, so `[<key>]` forced escaping in
     `ls`/`cd`/copy-paste; `-k<key>` is glob-safe. It stays unambiguous because
     the key is mandatory and always the **terminal** token — parse peels the
     trailing `-k<digits>`, so `05-task-k9-k3.md` is slug `task-k9`, key `3`. The
     `[DONE-]` above is metalanguage (the *optional* `DONE-` infix), not bracket
     syntax. All examples in this ADR use the `-k<key>` form.
2. **Position `NN` is a 2-digit zero-padded decimal, per level** (not a global
   dotted vector). It is the *mutable locator*: lexical == numeric == DFS within a
   level, locale-robust (pure digits), and gives ~99 siblings/level. A node's full
   position is its directory path; reorder/insert renames only the affected
   *level's* sibling dirs (`git mv 07-…-k14/ 08-…-k14/`), and the whole subtree
   — child names and keys — rides along untouched.
3. **The key (`-k<key>`) is the permanent stable id** (kept from ADR-0033):
   assigned once (`max key in tree + 1`), never rewritten by renumber or decompose,
   the **last** token before the extension / trailing slash. The keys in the names *are* the
   counter (no counter file); `.DONE` leaves stay in the tree so the max is always
   visible. **References resolve by key** — `resolve [k]` / `resolve <slug>` finds
   the current path wherever the entity moved.
4. **Done is marked in place as a `DONE` infix** right after the position
   (`NN-DONE-<slug>-k<key>.md`), leaves only. A brief/node is never marked done —
   node done-ness is implicit (no live leaf in its subtree). The `DONE` infix sits
   at a **fixed column** (the position is fixed-width, the variable-width `-k<key>`
   is exiled to the end), so a directory's done-prefix scans cleanly.
5. **Commit messages and prose name a work item by `<slug>-k<key>`** — never by
   its position or directory path. The position/path is mutable (renumber, move);
   the `<slug>-k<key>` handle is stable, so the historical record stays meaningful
   after restructures. This is a grove-skill instruction (see Consequences); it
   replaces the current inconsistent `070/030`-style path references (sometimes
   full path, sometimes elided).

**Comparator / verbs** keep ADR-0033's *semantics* (numeric per-level order, brief
heads its subtree, `pick` = first live leaf in DFS pre-order, reference-by-key) but
are re-expressed against the directory structure: `pick` is a recursive DFS walk,
`brief-chain` walks parent directories collecting each `BRIEF.md`, `resolve`
searches dirs by key, `leaf-decompose` turns a leaf file into a node *directory*,
`leaf-retire` adds the `DONE` infix in place, and `leaf-insert` `git mv`s sibling
directories (subtrees ride along).

## Rationale

- **Honesty:** the parent/child relationship is the filesystem's job; directories
  state it directly, dissolving the root-brief false-sibling problem and every
  numbering trick we circled (`00`, low-ASCII prefix, `tree/` subdir).
- **Cheap restructure:** moving/inserting a node is a single `git mv` of its
  directory; the subtree and all keys come along. Flat paid O(subtree) renames for
  the same edit. The user flagged insertions/reorderings as first-class, so this
  matters.
- **Native navigation:** `ls`/yazi/Finder render a real collapsible indented tree;
  `find .grove` still shows complete state (shape + `DONE`) with zero file reads —
  flat's only real edge (single non-recursive `ls`) is recovered by `find`/yazi.
- **Locale/FS-robust:** pure 2-digit decimal per level needs no letters
  (case-insensitive FS) and no punctuation-collation gamble; it sorts identically
  under byte, locale, and natural sort.
- **No sunk cost (ADR-0031, the no-sunk-cost principle):** the 070/040 flip was not
  wasted — it proved the whole pipeline (embed → provision → migrate-on-adoption →
  self-driving loop) on a real tree, and that pipeline is reused verbatim. Only the
  id grammar and the tree's on-disk *shape* change. Reversing ADR-0033's structure
  on merit is exactly the dogfeeding the refactor is built to do.

## Consequences

- **Reopens the scheme work** as a new node (`scheme-v2-directories`): the id/grammar
  model, the read verbs (`pick`/`brief-chain`/`resolve`), the grow+lifecycle verbs,
  a **v1-flat → v2-directories migration** (which must also still accept any
  un-opened old `NNN-slug/` tree → v2 directly), the prose rewrite, and a
  user-gated install-and-reflip (mirroring 070/040). The migration is fixture-
  tested before it touches a real tree and lands as one reviewable commit.
- **Methodology prose gains the commit-naming instruction** (content/SKILL.md):
  "reference a work item by `<slug>-k<key>`, not by its position/path." Because the
  global skill is binary-embedded and re-extracted on launch (ADR-0034), the prose
  ships with the v2 binary.
- ADR-0033 stays in history as the flat scheme it was; this ADR supersedes its
  *structure* only. The dotted-decimal *position* idea survives as the per-level
  number; the *flat-filename encoding* of it does not.
- Reversible in principle (scheme lives in names + git history), expensive once
  trees migrate — it clears the ADR bar as a durable trade-off, the same as 0033.
