# Seed capture — prior art survey

A survey of paradigms for capturing observations made during one workstream
that belong in a *different* workstream — future, parallel, or cross-repo.
The grove methodology calls such observations **seeds** (see `CONTEXT.md`).

This document characterises the paradigm space. It deliberately stops short
of prescribing a solution; the follow-up planning task grills on this
material to decide adopt / adapt / invent.

## How to read this

Each candidate is scored 0/1/2 against the four driving use cases from the
parent brief:

- **DEF** — deferred-future: target grove does not yet exist
- **PAR** — parallel-grove handoff: target is concurrently running
- **MUL** — multi-source: many groves contribute to one future seed
- **REP** — cross-repo: observation in repo A belongs in repo B

And against two grove-specific constraints:

- **ERG** — ergonomic fit: mid-flow capture, rename cost, invocation from
  any worktree
- **WAW** — walk-away-ability (SKILL.md constraint 6): what remains legible
  if all grove tooling disappears

## Shortlist

Two candidates worth serious consideration, and a third worth a closer look
as a fallback baseline:

1. **Dangling-link seeds in plain markdown.** A seed is a file (or set of
   files) named for its hypothesised future grove. References from elsewhere
   in the repo are plain `[[seed:name]]` strings — they resolve via `rg`,
   not via tool state. Strongest hits in DEF (the name *is* the dangling
   reference), MUL (many files can reference the same name), WAW (pure
   markdown). Synthesised from Obsidian/Foam dangling-link semantics, Rust
   RFC's `0000-` placeholder convention, and the `simonw/til` "no monotonic
   counter, no central index" shape.
2. **Maildir-shaped seed inbox.** Seeds live in `seeds/inbox/`,
   `seeds/claimed/`, `seeds/done/`; state transitions are `mv`, never
   edits. Lockless concurrency across worktrees, atomic on POSIX,
   filename-encoded status is greppable. Strongest hits in PAR
   (multiple worktrees can drop into one inbox) and MUL (no contention).
   The format inside each file is open — combines well with #1.
3. **GitHub Issues + cross-repo refs** (baseline). Familiar, has cross-repo
   `org/repo#NNN` syntax, but fails DEF (issue can't reference a project
   that doesn't exist), partially fails WAW (issues survive in a JSON
   export but not as plain markdown in the repo), and is repo-bound. Listed
   as the candidate to beat, not to adopt by default.

The detailed paradigm sections below show the work behind that shortlist.

---

## Paradigm 1 — Distributed bug trackers embedded in VCS

Items live in git refs or working-tree dot-directories; sync is push/pull.

### git-bug

- **Mechanism**: ops stored as JSON blobs in dedicated git objects under
  `refs/bugs/*`, `refs/identities/*`; each op = new commit on a per-bug
  chain. Capture via `git bug add`; sync via `git bug push/pull`.
- **Scores**: DEF 1 / PAR 1 / MUL 2 (DAG-merge is the point) / REP 0 /
  ERG mid (needs `git bug` invocation) / WAW poor (objects are in git but
  invisible to `grep` on a checkout).
- **Verdict**: **reject** — opaque to non-tool readers, violates WAW.

### Fossil tickets

- **Mechanism**: ticket-change "artifacts" stored as BLOBs in a per-project
  SQLite repo; relational tables are a derived cache.
- **Scores**: DEF 0 / PAR 0 / MUL 1 / REP 0 / ERG poor / WAW poor.
- **Verdict**: **reject** — wrong VCS, opaque at rest.

### Bugs Everywhere (BE)

- **Mechanism**: `.be/<dir-uuid>/bugs/<bug-uuid>/` with `values` (JSON) +
  `body` + comments subtree; lives in working tree, committed by host VCS.
- **Scores**: DEF 2 / PAR 1 / MUL 1 / REP 1 / ERG mid / WAW excellent.
- **Verdict**: **adapt** — file layout is closest to what seeds need;
  project is largely dead but the *format* survives the tool.

### ditz

- **Mechanism**: `bugs/` with one human-editable YAML file per issue +
  `project.yaml`.
- **Scores**: DEF 2 / PAR 1 / MUL 1 / REP 1 / ERG light / WAW excellent.
- **Verdict**: **adapt** — strongest plain-text precedent for "issue as
  greppable file".

### sit

- **Mechanism**: immutable hash-named record directories under `.sit/`;
  merge-by-union, no central authority.
- **Scores**: DEF 2 / PAR 2 / MUL 2 / REP 2 / ERG awkward (rename = append
  rename record) / WAW good (files on disk, but hashed names hurt humans).
- **Verdict**: **adapt with caution** — model is right; immutability adds
  friction; project dormant.

### git-appraise

- **Mechanism**: one-line JSON in git notes under `refs/notes/devtools/*`.
- **Scores**: tied to specific revisions, not free-floating intent.
  ERG mid / WAW poor.
- **Verdict**: **reject** — code-review tool, wrong shape.

### Radicle issues (COBs), ticgit-ng

- COBs: CRDT-as-git-objects, signed; opaque without radicle CLI. **Reject.**
- ticgit-ng: orphan `ticgit` branch in the repo; capture friction is high.
  **Reject.**

---

## Paradigm 2 — In-repo markdown conventions

No tool, or a thin tool over a markdown file convention.

### TODO.md / BACKLOG.md (single-file)

- **Mechanism**: H1 sections, `- [ ]` lines, `@user` / `#tag` markers.
  Aggregation = grep.
- **Scores**: DEF 1 / PAR 0 / MUL 1 (one file, no routing) / REP 0 / ERG
  high but conflict-prone across parallel worktrees / WAW perfect.
- **Verdict**: **reject as container** (single-file contention across
  worktrees), **adapt the `- [ ]` line grammar** inside per-seed files.

### ADR-style numbered files (Nygard / adr-tools)

- **Mechanism**: `doc/adr/NNNN-kebab-title.md`, monotonic prefix, per-file
  front-matter; `adr new` mints the next number.
- **Scores**: DEF 2 / PAR 1 (numbering collides across worktrees) / MUL 2 /
  REP 1 / WAW total.
- **Verdict**: **adapt** — keep the per-file model, **drop the monotonic
  number** (use date+slug to avoid worktree collisions). Borrow the
  status-in-front-matter convention.

### Rust RFC pattern (`text/0000-slug.md`)

- **Mechanism**: same as ADR but the number is *deferred* — `0000-` is the
  staging name until a PR is opened.
- **Scores**: DEF 2 (placeholder ID is literally the shape) / PAR 1 /
  MUL 2 / REP 0 / WAW total.
- **Verdict**: **adapt the placeholder-id-until-assigned mechanic.** A
  seed has no destination grove ID until that grove germinates — Rust
  RFCs solve precisely this with `0000-`.

### Backlog.md, TrackDown, Markdown Projects

- Front-matter-rich folder-per-issue with CLI tooling. ID-encoded
  filenames couple to tooling; WAW degrades to "files survive but IDs are
  meaningless without the tool".
- **Verdict**: **reject** — too tool-coupled for walk-away.

### Daylog / devlog / `simonw/til`

- **Mechanism**: dated files `YYYY-MM-DD.md` (daylog) or topic-folder +
  per-note file (TIL: `topic/slug.md`, dates from git). No ID minting, no
  central index.
- **Scores**: DEF 2 / PAR 2 (no central file = no collisions) / MUL 2 /
  REP 2 (just `cp`) / WAW total.
- **Verdict**: **adopt as the base shape.** Lowest-friction precedent.
  Combines naturally with the dangling-link primitive.

---

## Paradigm 3 — Local-first / PKM / task systems

### Obsidian (wikilinks + dangling-link semantics)

- **Mechanism**: vault of markdown; `[[Future-Seed-Name]]` creates an
  unresolved link, surfaced in graph + "unlinked mentions". Clicking
  auto-creates the target.
- **Scores**: DEF 2 (dangling links are first-class — the highest-fit
  finding in the survey) / PAR 2 / MUL 2 (backlinks aggregate) / REP 1 /
  ERG zero-friction / WAW total (pure CommonMark + `[[wikilink]]`).
- **Verdict**: **adopt the dangling-`[[seed:name]]` primitive.** Even
  without Obsidian, the bracketed string is greppable.

### Foam

- Same file format as Obsidian, OSS, VSCode-native. **Adopt-compatible**
  with the dangling-link primitive.

### Logseq

- Markdown/org files; outliner mode; block IDs written into the source
  file as hidden properties (specifically so refs survive the app).
- **Verdict**: **adapt the "block ID written into the file" trick** if
  seeds ever need stable refs across renames. Outliner mode is otherwise
  too opinionated for seed capture.

### org-mode

- `org-capture` binds keys to (template, target-file, headline) tuples.
  `org-refile` moves entries between files, with completion.
- **Scores**: DEF 1 (no dangling-link concept) / PAR 2 / MUL 2 / REP 2 /
  ERG Emacs-bound / WAW good (plain `.org`).
- **Verdict**: **adapt the capture-template + refile shape** without
  requiring Emacs. The "*promotion* of an observation to a structured
  seed" workflow gap is something org-mode actually addresses; most PKM
  tools don't.

### Taskwarrior

- SQLite-ish local DB, `task add ... project:future.seedX`. Project
  hierarchies (`future.seedX.subY`).
- **Scores**: DEF 1 / PAR 1 / MUL 2 / REP 2 / ERG CLI-fast / WAW poor
  (opaque store).
- **Verdict**: **reject** — opaque store kills walk-away and provenance.

### todo.txt

- Single file, `+project @context` markers.
- **Scores**: DEF 2 / PAR 2 / MUL 2 / REP 2 / ERG trivial / WAW perfect.
- **Verdict**: **adopt the `+seed-name` line-level grammar** as a
  shorthand for in-context seed references inside non-seed files.

### Dendron

- Dotted-hierarchy markdown (`seeds.billing.tax.md`). VSCode-bound,
  archived 2023.
- **Verdict**: **adapt the dotted-hierarchy naming** for seed namespacing
  without tool dependency.

### Roam, Tana

- Proprietary / cloud-only, walk-away-hostile. **Reject.**
- Transferable idea: Roam's **"unlinked references"** (text-matching, not
  just explicit links) means typo'd or pre-naming-convention seed mentions
  surface retroactively. Replicable with `rg`.

### ripgrep-over-notes

- **Mechanism**: no tool. Convention: `SEED(billing-tax): observation`
  lines anywhere in repo. `rg 'SEED\(billing-tax\)'` aggregates.
- **Scores**: DEF 2 / PAR 2 / MUL 2 / REP 2 / ERG zero-ceremony / WAW
  perfect by construction.
- **Verdict**: **adopt as the fallback substrate.** Every other choice
  must degrade to this.

### Things 3 / OmniFocus

- Proprietary, opaque stores. **Reject — closed, ineligible.**

---

## Paradigm 4 — Coordinator daemons & exotic paradigms

### Maildir (`new/cur/tmp`)

- **Mechanism**: sender writes to `tmp/$unique`, then `rename(2)`s into
  `new/`. NFS-safe, atomic, lockless. Filename encodes flags (`:2,RS`) so
  status is greppable.
- **Scores**: DEF 2 / PAR 2 / MUL 2 / REP 1 / ERG single-file-write
  capture / WAW perfect (bare files; `ls`, `grep`, `mv` are the toolchain).
- **Verdict**: **adopt the rename-is-state pattern.** A seed inbox with
  `inbox/claimed/done/` subdirectories gives atomic capture, lockless
  concurrency, and `mv` as the only state transition.

### SourceHut / debbugs (email-as-tracker)

- **Mechanism**: tickets *are* emails; mutations via subject prefixes
  (`!close`, `!tag`). debbugs writes per-bug status + mbox of
  correspondence.
- **Verdict**: **adapt** the addressing scheme: a seed-name in a filename
  routing-key (analogous to a list address) means one stream can address
  another without a daemon. Also worth borrowing: **commands as
  filename/header prefix** so a plain text drop can mutate state.

### Radicle COBs / git-bug (CRDT-on-git)

- Already covered in Paradigm 1; **reject as substrate**, but the
  **op-log shape** (seed = append, never mutate) is transferable.

### Pijul / Sapling (commutative patches)

- **Reject** for storage. Transferable: **patches commute** — design
  seeds so order-of-arrival is irrelevant.

### Automerge / Patchwork (Ink & Switch)

- Binary CRDT blob; opaque without library. **Reject** for WAW.
- Transferable: **branches as social artifacts** — a seed is a tiny
  "branch of thought", not a row in a table.

### Fossil timeline (append-only artifacts)

- Tickets are sequences of control artifacts replayed in timestamp order.
  **Reject** at storage; **adapt the event-replay model.** Transferable:
  **append-only fields** with `+name` (append) vs `name` (replace)
  semantics inside a single file.

### Hypercore / Dat

- Binary append-only log; opaque bare. **Reject.**
- Transferable: **single-writer-per-stream** — each grove owns its own
  seed log within a seed; no contention.

### systemd `.path` units / direnv

- Filesystem activation: drop a file, a daemon picks it up.
- **Verdict**: **adopt as optional activation layer.** Transferable:
  **`DirectoryNotEmpty=` semantics** — a non-empty `seeds/inbox/` *is*
  the notification. No daemon needed for correctness; daemon is pure UX.

### notmuchfs (directory-name-as-query)

- **Mechanism**: FUSE layer projecting tag queries as virtual maildirs;
  directory name = query.
- **Verdict**: **adapt as projection layer.** Seeds live in one canonical
  location; `seeds/by-target/<grove-name>/` is computed view, not source
  of truth. Avoids index drift.

### LSP-style local services

- No prior art for seed servers specifically. **Reject for v1** (tooling
  before protocols). Transferable: **stdio per-worktree** if grove later
  exposes a `grove seed-serve` capability.

---

## Paradigm 5 — Centralised-tracker integrations (the baseline to beat)

### GitHub Issues (incl. cross-repo refs and Projects v2)

- **Mechanism**: hosted issues; `org/repo#NNN` cross-repo refs;
  task-list checkboxes; Projects v2 as a query layer.
- **Scores**: DEF 0 (target repo + issue must exist) / PAR 1 / MUL 1 / REP
  2 (the one thing it does well) / ERG requires leaving the worktree
  context to `gh issue create` / WAW partial (JSON export survives, but
  issues are not in-repo markdown).
- **Verdict**: **reject as primary substrate**, **acceptable as optional
  promotion target** when a seed graduates to public tracking.

### Linear

- Hosted, slick UX, API-first. Same fundamental DEF/WAW problems as
  GitHub Issues, plus closed-source storage. **Reject.**

### Jira, Pivotal Tracker, Asana

- Heavier-weight variants of the same model. **Reject.**

---

## Cross-cutting findings

These showed up in multiple paradigm surveys and are decisive for the
seed convention design.

1. **Walk-away divides the field cleanly.** Tools that store items as
   files in the working tree (BE, ditz, sit, daylogs, ADR-tools)
   survive tool loss with `grep`. Tools that store items inside git
   objects, SQLite, or binary CRDTs (git-bug, Fossil, Radicle, Automerge,
   Taskwarrior) do not — even when the data is technically in git. SKILL
   constraint 6 makes this decisive.

2. **Dangling-link semantics solve the deferred-future case.** Obsidian
   and Foam treat `[[Future-Thing]]` as a first-class reference to
   something that doesn't exist yet. Rust RFCs use `0000-` as a placeholder
   filename for the same reason. Both work *without the tool* because the
   text is greppable. This is the single most leveraged primitive across
   the survey.

3. **Monotonic numbering is the silent killer of parallel-worktree
   workflows.** ADR-tools, Backlog.md, and Rust RFCs all collide when two
   worktrees mint IDs concurrently. Date+slug naming avoids this entirely.

4. **Maildir-style `rename(2)`-is-state is the cleanest known solution to
   lockless concurrent state transitions on a filesystem.** It is
   directly applicable to a seed inbox shared by parallel groves.

5. **Cross-repo handoff is almost never solved by existing tools.** Every
   surveyed system except centralised trackers (GitHub Issues) assumes one
   repo. The seed convention will need an explicit `target-repo:`
   front-matter field plus a `git mv` / copy-paste protocol — no
   inheritable convention exists, only ones to avoid (anything with
   embedded IDs).

6. **Promotion is the workflow gap.** Capture is trivial in every system;
   *promoting* an observation from inbox-noise to a structured seed
   (org-mode's `refile`, debbugs' `!retitle`) is the rare and valuable
   operation. The seed convention should make promotion as cheap as `mv`.

7. **Every distributed-bug-tracker project is effectively unmaintained.**
   ditz 2011, BE 2013, ticgit 2017, sit 2019, git-appraise 2020. The
   failure mode is social, not technical: nobody wants to read tickets in
   a CLI. Seeds dodge this because they are write-mostly handoff
   artifacts, not a sustained tracker — so the dead projects' *formats*
   are reusable even though their UX wasn't.

---

## ADR candidates flagged

The following decisions will surface in the next planning task. Listing
them here so they are not lost; they are NOT raised as ADRs in this
task (per the task spec).

1. **Seed storage location**: `.grove/seeds/` (per-worktree, retired with
   the grove) vs. `seeds/` at repo root (outlives groves) vs. an
   out-of-tree location synced across repos. Trade-off: walk-away
   visibility vs. cross-repo handoff vs. retirement semantics.

2. **Seed identifier shape**: dangling-`[[seed:name]]` wikilink, vs.
   `+seed-name` todo.txt-style tag, vs. `SEED(name)` ripgrep marker, vs.
   path-encoded (`seeds/<name>/...`). The choice determines what `rg`
   pattern aggregates a seed.

3. **State transition mechanism**: maildir-style `mv` across
   `inbox/claimed/done/` subdirs, vs. front-matter `status:` field, vs.
   append-only event-log within the seed. Trade-off: atomicity vs.
   git-friendliness vs. history.

4. **Multi-source aggregation strategy**: one file per seed grown by
   append (Fossil-style), vs. one file per observation grouped into a seed
   directory (ditz-style), vs. dangling-link backlinks (Obsidian-style).
   Trade-off: merge conflicts vs. provenance vs. discoverability.

5. **Cross-repo handoff protocol**: `target-repo:` front-matter + manual
   copy, vs. dedicated `seeds.git` worktree synced across repos, vs.
   promotion-to-GitHub-Issues. Trade-off: walk-away vs. coordination cost
   vs. social legibility.

6. **Promotion / germination semantics**: what happens when `grove start
   <seed-name>` is run? Does the seed get moved into the new grove's
   `.grove/`? Linked from it? Copied? Trade-off: history preservation vs.
   simplicity.

---

## Out-of-scope follow-ups noted during research

- A deep dive into **CRDT-backed local-first stores** (Automerge,
  Hypermerge, recent Ink & Switch work post-Patchwork) was not done. Worth
  a focused leaf only if the seed convention turns out to need true
  multi-writer merge semantics rather than the simpler maildir model.
- **LSP-as-seed-server** as an editor-integration story was deferred
  intentionally — protocols before tooling is the wrong order.
- Real-world usage patterns of **org-refile across repos** (people who use
  one `~/org/` directory as a cross-project inbox) might be worth a
  shorter, more practical follow-up: it's the closest existing thing to
  what seeds want.
