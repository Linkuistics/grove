# chain-as-node-k7

**Kind:** design

## Goal

Decide whether a **review chain** (`X` → `review-X` → `integrate-review-X`) and a
**vendor pair** (`research` → `research` → `combine-research`) should be
represented as a **node directory** rather than as three sibling leaves sharing a
stem, and if so, specify the change.

Raised by the human, 2026-07-29, during `skill-integrate-k4`: *"the do-X
review-X integrate-review-X, and research-A research-B combine-research groups
should be represented as a directory in grove — it's better UX when viewing the
tree."*

## Context

This **reverses a decision the current design records twice**, so the first job
is to re-derive whether those arguments still hold, not to assume either side:

- `CONTEXT.md` § *Review chain / vendor pair*, `_Avoid_`: "giving a chain its own
  [[Node directory]] — a node already means 'this work proved bigger than one
  session', and one per chain overloads that signal, buys a `BRIEF.md` no step
  earned, and would apply `leaf-decompose` (a *reactive* verb) speculatively."
- `content/SKILL.md` § Decompose carries the same paragraph, as does
  `docs/specs/task-kind-taxonomy.md`.
- ADR `docs/adr/task-tree-scheme.md` — the naming scheme and `pick`'s walk.

The stem-plus-suffix naming (`<stem>` / `<stem>-review` / `<stem>-integrate`)
exists **because** of that decision: it was the way to make a chain legible from
`find .grove` without a directory. A directory would make that naming redundant,
so the two are one question, not two.

## Done when

The decision is recorded — as an edit to the existing ADR/spec/glossary set, in
place, never as a superseding append — and either

- the proposal is adopted, with a spec covering: what writes the directory
  (`leaf-add-chain` / `leaf-add-pair` gain it, or `leaf-decompose` is
  generalised), whether the node gets a `BRIEF.md` and who writes it, what
  happens to the stem-suffix naming, how `pick`'s pre-order walk behaves (it
  already descends node directories in place, so ordering may be unchanged), how
  the Retire cascade's per-node confirmation applies to a chain that closes, and
  the migration story for trees already carrying flat chains; **or**

- it is rejected, with the human's UX complaint answered by whatever *does* fix
  it, and the rejection's reasoning folded into the existing `_Avoid_` entries.

## Notes

**The UX claim is the load-bearing one and it is checkable.** "Better UX when
viewing the tree" has two readers with different needs: `find .grove` / a file
manager, and the **tree-viewer plugin** (`herdr-plugin/`), whose *only* contract
is the node-directory naming scheme (`CONTEXT.md` § Tree viewer plugin). Look at
both before arguing from first principles — the plugin may already render a
stem-grouped chain acceptably, or be one small change from it, which would
answer the complaint without touching the tree scheme at all.

**Known tensions to weigh, not to inherit:**

1. A node currently *means* "this proved bigger than one session". A chain is
   planned up front, so a chain-node makes node-ness mean two things.
2. `BRIEF.md` — constraint 4 says every artifact is created only when it earns
   its place. A chain node either carries a brief written because a step demanded
   it, or is the first node type allowed to be brief-less.
3. `leaf-insert` at sibling level can currently split a chain; containment would
   prevent that. The glossary calls this "the one real gap" and judges it not
   worth closing — that judgement is in scope to revisit.
4. Changing the directory scheme is a **plugin-compatibility question**
   (`CONTEXT.md` § Tree viewer plugin): the plugin and the binary version
   independently, and filename-only parsing is what the scheme was designed for.

**Scope check — this may not belong to this grove.** This grove's `BRIEF.md`
ships one skill, `using-codebase-memory`; its "Done when" says nothing about the
task-tree scheme. This leaf is grove-methodology work (the `grove` bounded
context, `CONTEXT.md`, not `plugins/CONTEXT.md`) that happens to have been raised
here. It is likely a **grove of its own** — it touches the ADR, the spec, the
glossary, two CLI verbs, and a migration. Raise that with the human at the start
of the session: if they agree, `leaf-prune` this leaf and start a new grove;
the file is written to be portable to that grove's root brief either way.
