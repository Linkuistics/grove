# 5-[5]-dotted-decimal-numbering — brief

**Kind:** planning

## Goal

Deliver the flat dotted-decimal task-id scheme settled in this node's grilling
(running log D1–D9 below) and recorded in **ADR-0033**: filenames
`<position>-[<key>]-<slug>[.BRIEF|.DONE].md`, a position-vector version-sort
comparator, permanent `[key]` references, mark-done-in-place (leaves only), and
the eight id-addressed verbs (incl. the new `resolve`). Built as **new,
separately-tested code that does not touch the live verb path** (D9) — this grove
stays an old-format tree until 060 migrates it.

## Status

**Design settled; planning done.** The grilling walked D1–D9 (below); the durable
record is **ADR-0033**. What remains is implementation, decomposed below. Build
order is dependency-driven: `010` (the id model) is the foundation the other three
consume.

## Decomposition

- `010-id-model` (work) — the parsed-filename type, the position-vector version-
  sort comparator (D2), next-key assignment (D6). Pure functions, exhaustively
  unit-tested; foundation for every verb.
- `020-read-verbs` (work) — `pick`, `brief-chain`, `resolve` (D4/D7) on the model.
- `030-grow-verbs-renumber` (work) — `leaf-add` + `leaf-insert` with the
  subtree-cascade renumber + position-only header rewrite + stray-positional
  cross-ref surfacing (D5). The renumber is the hardest piece — test it hard.
- `040-lifecycle-verbs` (work) — `root-init`, `leaf-decompose` (key-preserving,
  +first child), `leaf-retire` (`.DONE`) (D3/D4/D6).

All four build new-format logic against new-format fixtures; **none is wired in as
the live verb** (D9).

## Pointers

- **ADR-0033** — the settled scheme (durable record; cites adopted prior art and
  the rejected LexoRank / fractional-indexing alternative).
- **060-backwards-compat-migration** — owns the dual-format dispatch + `grove
  migrate` + old-format drop-plan; the methodology-prose / format-guide /
  `CONTEXT.md`-glossary updates describing the new scheme **go live there, not
  here** (D9) — putting not-yet-live terms in the live glossary would describe
  fiction while the running verbs still speak `NNN-slug`.
- `done/010-plan` D4/D5 (retired) — the direction this node settled.

## Notes

- The running log D1–D9 below is the full grilling record; **ADR-0033** is its
  promotion to the durable record (the log is deleted with the grove at finish).

## Decisions (running log)

**D1 — Filename grammar: dash-before-slug, slug kept (settled 2026-06-21).**
The flat namespace lives directly in `.grove/` (no node directories). A task id is
a vector of integers joined by `.`; the human slug is attached to the *last*
integer by a single `-`. So:
- **leaf** = `<id>-<slug>.md`, e.g. `2.2.1-add.md` (id `[2,2,1]`, slug `add`).
- **node brief** = `<id>-<slug>.BRIEF.md`, e.g. `2.2-verbs.BRIEF.md` (the node
  `2.2` carries its brief; children are `2.2.1-*`, `2.2.2-*`).
- **root brief** = `BRIEF.md` (the root has no id and no slug — unchanged).
Rationale: the `-` crisply delimits where the integer id ends and the slug begins,
so the parser never guesses — and it sidesteps the all-dot form's ambiguity
(`1.2.42.md` could be id `[1,2,42]` *or* id `[1,2]`+slug `42`) without banning
numeric slugs. Closest to today's `NNN-slug`, so reader + migration share idioms.
Keeping the slug on a brief means `ls` alone says what each node is *about*
(constraint 6). Parse rule: strip `.md`; if the remainder ends `.BRIEF` it's a
node brief (strip it); the leading `int('.'int)*` run is the id, the first `-`
after that run begins the slug (slugs may contain `-`, never `.`). Reserved slug
words: `BRIEF` (and the done marker, D3). `BRIEF`/`DONE` are excluded from the
numeric sort key (D2). **[Grammar amended by D6]** a permanent `[key]` is inserted
between position and slug — `2.3.1-[4]-add.md`; D6 is authoritative for the final
grammar.

**D2 — Comparator: integer-vector version-sort, shorter-prefix-first, lenient on
foreign files (settled 2026-06-21).** The sort key is the integer vector parsed
from the id (`2.2.1-add.md` → `[2,2,1]`); the `-slug`, the `.BRIEF`/done markers,
and `.md` are *not* part of the key. Contract (matches `sort -V` / natural-sort):
- **element-wise integer compare** — `[2,9] < [2,10]` (numeric, not lexical: this
  is exactly why the current lexical-on-zero-padded sort can't carry over);
- **shorter-prefix-first** — `[2,2] < [2,2,1]`, so a node's brief heads its own
  subtree and the flat sorted list is DFS *pre-order*. Root brief is key `[]`,
  sorts first;
- **total order** — legitimate trees never tie (a node and its children differ in
  length; decompose replaces a leaf with its brief so they never coexist at one
  id), but the comparator is still total: a same-id collision (a *malformed* tree)
  breaks deterministically by full filename;
- **lenient on foreign/malformed files** — anything in `.grove/` not matching the
  leaf/brief grammar is simply not a leaf: `pick` ignores it and it sorts last; no
  error, so a stray `README.md`/half-written file never jams the loop. (Matches
  today's `pick` leniency.)

**D3 — Done-marker: filename suffix `.DONE`, leaves only (settled 2026-06-21;
amended by D4).** Mark-done-in-place (D5) keeps retired items in the one flat
list, so the marker carries what the old `done/` move used to. A done **leaf**
appends `.DONE` before `.md`: live `2.1-model.md` → done `2.1-model.DONE.md`.
**Only leaves carry `.DONE`** — nodes/briefs are never marked done (D4), so the
`.BRIEF.DONE` combination never occurs; the parser peels a single optional `.DONE`
from a leaf name, or `.BRIEF` from a brief name, never both on one file.
Chosen over frontmatter on **constraint 1**: with a filename marker `ls .grove/`
shows the *complete* state — tree shape *and* done-ness — with zero file reads;
`pick` skips by name; the numeric id and sort position are preserved; the marker
is excluded from the sort key (D2). Frontmatter would force `pick` to open and
YAML-parse every file (a new parse dependency + failure mode) and hide done-ness
from `ls` (a regression from the old done-by-absence). `DONE` is a reserved slug
word. Retire is a `git mv` rename (parity with today's done/ move).

**D4 — Done semantics: only leaves are DONE; node-done is implicit; decompose
enforces a first child (settled 2026-06-21).** A **brief is context, not a task**,
so it makes no done-claim — only leaves ever carry `.DONE`. This makes the bad
state the leaf flagged (a live leaf whose ancestor brief is `.DONE`) *unrepresent-
able* rather than something needing defined semantics. Consequences:
- **`pick`** is a pure per-file filter: skip every `.BRIEF`; skip every `.DONE`
  leaf; return the first live leaf in version-sort order. It never reads briefs
  and never reasons about node state.
- **node done-ness is implicit** — a node is done ⟺ `pick` finds no live leaf in
  its subtree. The cascade *judgment* (promote brief content upward; is this node
  finished?) still runs when the last live leaf under a node is retired, but it
  produces **no file flip** — the brief goes inert among its `.DONE` children and
  the whole tree is deleted wholesale at grove-finish. No node-retire `git mv`.
- **`brief-chain`** is unaffected: it ascends the picked leaf's *prefix* ids and
  collects each level's `BRIEF.md`; it never meets a DONE brief.
- **`leaf-retire`** operates on leaves only (it already refuses `BRIEF.md` today),
  so no code path can mark a brief done.
- **enforce ≥1 child:** `leaf-decompose <leaf> <first-child-slug>` creates the
  brief **and** `<id>.1-<first-child-slug>.md` atomically, mirroring `root-init`'s
  brief+first-leaf contract (ADR-0011 generalized to every node). A node is never
  childless, even transiently — so a brief-only subtree can never read as "done".

**D5 — Verb surface: id-addressed, two positionals `<id> <slug>` (settled
2026-06-21).** A node is now a dotted **id**, not a directory path, so
id-addressing replaces the current `--node <dir>`/path anchoring. The grow verbs:
- `leaf-add <parent-id> <slug> [--kind]` → append a child at `<parent-id>.<next>`
  where `<next>` = max existing child index + 1 (sequential, gapless). Root parent
  is `.` (`leaf-add . survey` → `1-survey.md`). The old `--prefix` escape hatch and
  `parse_prefix_slug` fiddliness are gone (the scheme renumbers instead of
  leaving +10 gaps).
- `leaf-insert <target-id> <slug> [--kind]` → create a new leaf at exactly
  `<target-id>`, shifting the current occupant and all later siblings up by 1.
  The id is pure-dotted; the slug is a separate positional (no embedded `-` to
  parse). Renumber **cascades through whole subtrees**: bumping `2.2`→`2.3` also
  rewrites `2.2.1`→`2.3.1`, `2.2.2`→`2.3.2` — the k-th segment of every id whose
  path starts `<parent>.<j>` for j ≥ position. Highest-first (intermediate FS
  state stays collision-free), `git mv` each, rewrite each moved file's
  `# <id>-<slug>` header, then surface cross-refs (D6). This is the
  "renumber-on-reorder accepted" cost (D5-of-010).
- `leaf-decompose <leaf-path> <first-child-slug>` → `git mv` the leaf to
  `<id>-<slug>.BRIEF.md`, retitle `# <id>-<slug>` → `# <id>-<slug> — brief`, and
  create `<id>.1-<first-child-slug>.md` atomically (D4 enforce-first-child).
- `leaf-retire <leaf-path>` → rename `<id>-<slug>.md` → `<id>-<slug>.DONE.md` (D3);
  refuses a `BRIEF.md` and an already-`.DONE` leaf.
- `root-init [<slug>]` → root `BRIEF.md` + first leaf `1-<slug>.md` (now `1-`, not
  `010-`).
All remain working-tree-only (no commit); the enclosing task's commit folds them
in. Headers across all verbs move from `# NNN-slug` to the full filename stem
(`# <position>-[<key>]-<slug>`, D6); renumber rewrites the *position* in the
header, never the key.

**D6 — Permanent key: every file carries `[n]` between position and slug;
references resolve by it (settled 2026-06-21).** Supersedes the leaf's cross-ref
auto-rewrite-vs-surface question by making durable references *stable by
construction*. Every leaf and node brief carries a **permanent integer key**
`[n]`, placed between the dotted position and the slug. Three orthogonal parts,
each one job:
- **position** — the dotted `2.3.1`: *where* it sits in the DFS order; rewritten on
  renumber. Sole input to the sort comparator (D2).
- **permanent key** — `[4]`: *stable identity*; assigned once at creation, never
  changes (not on renumber, not on decompose).
- **slug** — `add`: human label; ~stable, may be edited.

Canonical examples: leaf `2.3.1-[4]-add.md`; node brief `2.3-[2]-verbs.BRIEF.md`;
done leaf `2.3.1-[4]-add.DONE.md`; root brief `BRIEF.md` (the one unkeyed
singleton). Decisions:
- **Key assignment** = max existing `[n]` over the whole tree + 1 (a per-grove
  monotonic creation serial). **No counter file** — the keys in the filenames *are*
  the counter state (constraint 1); `.DONE` files stay in the tree (D3) so the max
  is always visible and keys are never reused.
- **Integer, not uuid**: a grove tree is single-worktree / single-writer, so uuid's
  decentralized-generation property buys nothing while costing legibility (`[a3]`
  opaque/unordered vs `[4]` readable/ordered). **Always-present, not on-collision**:
  a uniform stable handle on every task — robust to *both* renumber *and* slug
  edits — which also dissolves the slug-uniqueness question (two `model` tasks are
  `[3]`/`[8]`, distinct). Cost accepted: minor filename noise; two numbers per name
  (the bracket disambiguates position from key).
- **References resolve by `[n]`** (write `[n]` or `[n]-slug` for readability). Stable
  under renumber (position moves, key doesn't) and under slug edits. This is what
  *dissolves the cross-ref problem*: durable references never need rewriting.
  `leaf-insert`'s renumber rewrites the **position prefix only** (filenames +
  headers), never `[n]` or slug; a light **surface of stray *positional* (`2.2`)
  references** stays as a lint (durable refs should use `[n]`, not position).
- **Verb effects**: `root-init` → root `BRIEF.md` + `1-[1]-<slug>.md`. `leaf-add` /
  `leaf-insert` assign the next key to the new leaf. `leaf-decompose` **preserves the
  node's key** (leaf `2-[3]-build.md` → brief `2-[3]-build.BRIEF.md` — the entity
  that was `[3]` stays `[3]` as it grows children); the auto-created first child
  gets a fresh key. `leaf-retire` appends `.DONE`, key untouched.
- **Grammar (authoritative)**: `<position>-[<key>]-<slug>[.BRIEF|.DONE].md`; root
  brief `BRIEF.md`. Parse: strip `.md`; peel optional `.BRIEF`/`.DONE` (at most one,
  D4); leading `int('.'int)*` = position; `-[<digits>]` = key; trailing `-<slug>`
  (slug may contain `-`, never `.`/`[`/`]`). Reserved in slugs: `[`, `]`, and the
  words `BRIEF`/`DONE`. Sort key (D2) is the position vector only — key, slug, and
  markers are all excluded.

**D7 — `grove-llm resolve <ref>`: a reference→path verb (settled 2026-06-21).**
The LLM must not do string-based reasoning to turn a `[n]` reference into a
filename (no `ls | grep`). Add a **read verb** alongside `pick`/`brief-chain`:
`grove-llm resolve <ref>` prints the absolute path of the current file matching
`<ref>` (searching live *and* `.DONE` files), one per line. Resolution:
- `[4]` or `4` → the unique file whose permanent key is 4 (keys are unique
  tree-wide → exactly one match; this is the primary, durable-reference case).
- `[4]-add` → same (key resolves; the slug part is decorative/validated).
- bare slug `add` → resolve by slug: 0 → not found; 1 → print it; >1 → list every
  match with its `[n]` on stderr so the caller re-queries by the unambiguous key.
Not found → empty stdout + diagnostic on stderr (pick-style, non-error). A `.DONE`
target prints a stderr note (the referenced task is retired). This closes the loop
on D6 — D6 makes `[n]` the stable reference *producer*, `resolve` is the
*consumer*, so following a reference is one deterministic call. New work item in
the decomposition (a 5th verb beyond the four grow verbs).

**D8 — Prior art for the ADR (settled 2026-06-21).** *Adopted* mechanism cites:
**outline / multilevel-list / legal section numbering** (renumber-on-insert +
DFS pre-order), **ASN.1 / OID object identifiers** (dotted hierarchical id ordered
by per-component numeric compare), **materialized path** (flat storage of a tree,
lexical sort → pre-order), **natural / version sort** (`sort -V`, Debian
`dpkg --compare-versions`, SemVer precedence — the comparator). *Rejected*
alternative cites: **LexoRank** (Jira) and the **fractional-indexing** family
(Figma; CRDT list-position ids — Logoot/LSEQ/Treedoc) — insert-without-renumber,
but flat (not a tree) and illegible keys; already rejected by name in 010 D5.
The renumber cost grove accepts (D5) is the price of legible dotted integers that
*are* the tree.

**D9 — 050↔060 boundary: isolated build, integrate in 060 (settled 2026-06-21).**
This grove is itself an old `NNN-slug/` tree, and every commit on this branch is
made *by a session running these verbs against this tree*, so ADR-0031's "never
delete the old runtime before the new one works" applies at *commit* granularity.
Therefore **050 builds the new scheme as new, separately-tested code and does NOT
touch the live verb code path** — the running `grove-llm` verbs keep their current
old-format behaviour untouched, so no 050 commit can break this grove. **060 owns
all the format-detection dispatch + `grove migrate` + the old-format drop-plan**,
validated against this real tree (then migrates it). Consequence for 050's
decomposition: it produces the new id-model + comparator + the eight verbs'
new-format logic + tests against new-format fixtures — *not wired in as the live
verbs*. The methodology prose / format-guide updates (SKILL.md `NNN-slug`/`done/`
language, BRIEF/TASK-FORMAT) land when the scheme goes **live in 060**, not here.
