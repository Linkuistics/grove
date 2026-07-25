# plan-k1

**Kind:** planning

## Goal

Decide the shape of a **monorepo** holding both `Linkuistics/grove` (Rust CLI +
embedded `content/` methodology) and `Linkuistics/skills` (the `linkuistics` and
`testanyware` plugin marketplace), and grow the tree that gets us there.

## Context

Two repos today, already coupled at the documentation level:

- `~/Development/grove` — Rust CLI; embeds `content/` in the binary and sweeps it
  to every installed harness's **personal** skill dir on each `grove do`;
  distributed via `brew tap Linkuistics/taps`.
- `~/Development/skills` — plugin marketplace (`.claude-plugin/marketplace.json`)
  publishing `linkuistics` (15 skills) and `testanyware`; distributed via
  `claude plugin install`.

grove already names `linkuistics:decision-records` and `linkuistics:codebase-design`
as documentation-level prerequisites, and `content/driving.md` paraphrases
`doubt-driven-development` rather than bundling it. grove was originally **split
out** of the skills repo.

## Done when

Shared understanding is reached on the monorepo's shape, and the tree carries the
leaves that implement it.

## Decisions (running log)

### Q1 — What is this grove's goal? → **a monorepo for grove + skills**

Settled. The driver is **coupled change cadence**: "Most grove changes require
skill changes as well." Two repos means every such change is a cross-repo
lockstep edit with no atomic commit and no single review. A monorepo makes the
coupled change one commit.

Not chosen (and not merely deferred — superseded by the above): keeping two repos
and only deduplicating overlapping *content*, or keeping two repos and unifying
only the *distribution* mechanism. Both leave the cross-repo lockstep in place,
which is the actual pain.

### Q2 — Which repo hosts the merged history? → **grove absorbs skills**

Settled. `Linkuistics/grove` is the monorepo; the skills repo's markdown moves in.

The question was first framed as "which published URL survives", on the premise
that `/plugin marketplace add Linkuistics/skills` is identity-bearing. **It is
not** — looked up rather than assumed, from local state:

- `~/.claude/plugins/known_marketplaces.json` records the marketplace under the
  name **`linkuistics`**, with `source: {github, repo: "Linkuistics/skills"}` as
  merely the fetch address. The name already differs from the repo name.
- `~/.claude/plugins/installed_plugins.json` keys plugins `linkuistics@linkuistics`
  and `testanyware@linkuistics` — `<plugin>@<marketplace>`, both from JSON `name`
  fields.

So skill namespaces (`linkuistics:decision-records`, `testanyware:…`) are
**repo-independent**; a move changes only `source.repo`, re-pointed with one
`/plugin marketplace remove` + `add`. GitHub renames 301-redirect on the git
protocol, so clones and the brew tarball URL follow automatically.

With URL cost symmetric and cheap, the deciding asymmetry is **build tooling
risk**: moving 15 markdown dirs + `marketplace.json` + `install.sh` is a file
move; moving a Cargo workspace, a `build.rs` that embeds `content/` via
`include_dir!`, a test suite, cargo-release and a brew tap is a pipeline rebuild.
grove is also jj-native while skills is plain git, so this direction preserves jj
instead of forcing a conversion.

**Hazard to carry forward:** do not merely *archive* `Linkuistics/skills`.
`autoUpdate: true` would keep pulling successfully from a frozen repo — no error,
just silent staleness. Rename-with-redirect, or make the re-point explicit.

### Q3 — How far does "merge" go? → **repo merge only; distribution untouched**

Settled. Both mechanisms keep working side by side: brew + binary provisioning
for grove's methodology, plugin marketplace + `install.sh` for the skills.

Ruled out **by fact, not preference**: collapsing onto one mechanism. Claude Code
plugins are Claude-only, and `src/harness.rs` supports three harnesses
(`claude`/`.claude/skills`, `codex`/`.codex/skills`, `pi`/`.pi/agent/skills`).
Unifying on plugins would drop codex and pi — which is precisely why the skills
repo carries `install.sh` at all.

Deliberately **not** taken now, though the monorepo enables it: having grove's
`provision.rs` (stamped, idempotent, refuses to clobber a foreign dir, replaces a
stale symlink *as a link*) take over from `install.sh`'s plain symlinks for the
non-Claude harnesses. That is a real improvement and a separable grove.

### Q4 — How does skills' history enter grove? → **full graft via merge commit**

Settled. `jj git remote add skills` → `jj git fetch --remote skills` → a
two-parent merge commit joining two roots. All 68 commits preserved; `git blame`
on the 15 skill files stays continuous past the merge — which matters precisely
because the premise of the merge is that those files will now be edited *together
with* grove.

Rejected: flat import (dead-ends blame at the import commit, losing the thing the
merge exists to enable) and rewrite-then-graft (conflict-free, but rewrites all 68
hashes so the archived repo becomes a different lineage and commit citations stop
resolving).

**Collision set, measured — 5 root files, everything else lands clean:**

| Path | Status |
|---|---|
| `plugins/`, `.claude-plugin/`, `install.sh` | net-new |
| `docs/adr/symmetric-vcs-rule.md` | net-new (not among grove's 11 slugs) |
| `docs/research/*` (3) | net-new (no overlap with grove's 7) |
| `.gitignore`, `CHANGELOG.md`, `CONTEXT.md`, `LICENSE`, `README.md` | collide |

`LICENSE` is 201L vs 191L, both Apache-2.0 with identical heads — diff it properly
rather than picking one, since grove additionally carries `content/LICENSES/` with
third-party attributions (mattpocock, addyosmani).

### Q5 — How does the glossary resolve? → **`CONTEXT-MAP.md` + two contexts**

Settled. The monorepo is honestly two bounded contexts, and this dogfoods
multi-context support that `content/CONTEXT-FORMAT.md` documents but grove has
never exercised.

- `CONTEXT-MAP.md` (root, new) — lists both contexts and their relationship
  (grove → skills: the methodology cites `linkuistics:decision-records` and
  `linkuistics:codebase-design` as prerequisites).
- `CONTEXT.md` (root) — grove's existing 147-line glossary, unchanged.
- `plugins/CONTEXT.md` — **rewritten**. Skills' current file is not a domain
  glossary at all; it is titled *"skills — jj adoption context"* and holds four
  terms from one finished workstream. Turning it into an actual skills-domain
  glossary is the work that should have happened when that workstream finished.

**Finding to carry forward:** `plugins/linkuistics/` has no `plugin.json`, so
Claude Code versions the plugin by **repo HEAD SHA**
(`installed_plugins.json` records `"version": "e0ba6f40f6e8"`). After the merge
*every* grove commit re-versions the plugin and triggers an `autoUpdate`
re-install even when no skill changed. Harmless but noisy; fixable with an
explicit `plugin.json` version. The same fact defuses the versioning worry in the
other direction — grove's `v15.0.0` cargo-release tags and the plugins' SHA
versions are separate axes and will not fight.

### Q6 — What happens to `Linkuistics/skills`? → **archive + announce**

Settled, on the basis that the consumer set is effectively this machine.

Rejected, though it works: keeping the repo alive as a **pointer marketplace** —
gutted to a `marketplace.json` whose entries `git-subdir` into the monorepo, so
existing installs follow transparently with zero user action. The mechanism is
real (220 of the 273 plugins in `anthropics/claude-plugins-official` use remote
`git-subdir`/`url` sources), but a permanently live second repo is not worth
sparing one person two commands. Recorded in the ADR with its reopening
condition: external consumers appearing.

**Hazard this leaves:** archiving does not break `autoUpdate` — it keeps
succeeding against a frozen repo, silently. Announcement is therefore a
completion condition of `cutover-k6`, not a courtesy.

### Tree confirmed and grown

Five work leaves under the root, ordered by dependency (`k2` the only hard
prerequisite; `k3`–`k5` mutually independent behind it; `k6` last, needing the
merge pushed):

```
02-graft-history-k2        03-glossary-partition-k3
04-docs-reconciliation-k4  05-plugin-versioning-k5
06-cutover-k6
```

Each passes `driving.md`'s vertical-slice test — demoable alone, not a horizontal
layer waiting on siblings. `04` is the likeliest to `leaf-decompose`, at the seam
between README/CHANGELOG and the ADR-set pass; left whole rather than pre-split,
per constraint 4.

**Not leafed, deliberately.** Unifying provisioning (grove's `provision.rs`
replacing `install.sh` for codex/pi) goes to the root brief's *On the horizon*
instead: the fog-or-ticket test asks whether the question can be *stated*
precisely today, and the load-bearing part — which harnesses the sweep covers,
and how to avoid double-provisioning a skill Claude already gets via plugin — is
exactly the part still fogged.

**ADR raised:** `docs/adr/skills-monorepo.md`. All three when-to-write conditions
hold — hard to reverse (a pushed history graft), surprising without context (the
skills README currently says grove *moved out*), and a real trade-off. Each
rejected alternative carries what would reopen it.

**No glossary edits this session.** Nothing durable to grove's domain was
resolved — the merge is an event, not a domain concept. The two-context split is
`glossary-partition-k3`'s work.

## Notes
