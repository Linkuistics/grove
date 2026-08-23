# flip-k27

## Goal

Cut increment 2 — **the flip** — into leaves. `crate-k7` closed with
`ordinal-fs-tree` standing alone and driven end to end by its own CLI; this leaf
decides how grove moves onto it, and produces the decomposition rather than any
of the work.

The root brief already fixes the shape: grove's tree modules are deleted, grove
supplies a domain implementation of `EntryName`, and trees in flight are
unaffected because no on-disk name changes. What is not fixed is the ordering,
the leaf boundaries, and the answers to the four questions increment 1 handed
forward.

## Context

Beyond the brief chain — which is the root `BRIEF.md` alone, since `crate-k7`
is closed and its brief binds nothing outside its own subtree:

- `docs/ordinal-fs-tree/ARCHITECTURE.md`, `CONTEXT.md` and `CLI.md`. The third
  matters more than it looks: its *What `cli-k16` found* section is the only
  place the library's error surface is described from a consumer's side.
- `docs/adr/entry-name-is-the-only-seam.md` and
  `docs/adr/entries-are-never-removed.md` — the seam record now carries the
  vocabulary cost this leaf has to price.
- `src/tree_id.rs`, `src/tree_rename.rs` and the rest of grove's tree modules,
  as **prior art and never as authority**: the root brief records that
  `tree_id.rs`'s grammar is deliberately lenient and therefore breaks the
  library's canonicity obligation.
- grove's own regression cover, which is this increment's safety net rather than
  its deliverable: roughly 130 CLI-contract tests — `leaf`, `session_kind_tree`,
  `composition_verbs`, `leaf_ops`, `kind`, `jj_tree_verbs`, `resolve`, `pick`,
  `root_init`, `brief_chain`, `tree_access`.

## Done when

- Increment 2 is a node with a `BRIEF.md` and dependency-ordered leaves, each
  leaving grove building and its tests green.
- Each of the four inherited questions is either answered in that brief or
  assigned to a leaf that will answer it. The root brief states all four; they
  are the version-control-aware move, the lenient name grammar, the refusal
  vocabulary, and the dependency line's `default-features = false`.
- The brief says what regression evidence a flip leaf owes — grove's existing
  suites are the net, and a pure refactor that changes a test is a finding.
- Whether the skill-distillation leaf can be cut yet is decided, not deferred by
  omission: it must run after every modelling episode, and increment 2 will
  produce more.

## Notes

**The four inherited questions, so that cutting them is not re-derivation.** The
root brief carries each with its evidence; this list is the index.

1. **The version-control-aware move is gone.** The library does `rename(2)`
   unconditionally, so a tracked entry renamed through it leaves git's index
   holding the old path. jj is unaffected. Decide: re-stage after a rename,
   accept the changed `git status`, or something else.
2. **grove's name grammar is not canonical.** `parse_position` accepts `5` and
   `Entry::name` renders `05`, so one entry can occupy two files. Either the
   grammar tightens and a lenient spelling becomes a refusal naming the
   canonical form — what the reference domain does — or the obligation is
   knowingly waived and the flip records why.
3. **The library's refusals speak the library's vocabulary, and grove's
   collides.** Not merely differs: grove's `Leaf` is a task file and the
   library's is any regular file, so `TargetNotNode`'s message is actively
   misleading in grove's own words. The seam ADR names this as the condition
   that would reopen it.
4. **`default-features = false` on grove's dependency line**, which keeps the
   library's imposed dependency set at `libc` and is the only thing that checks
   that claim.

**Do not decide these here.** This is a `planning` leaf: its deliverable is the
decomposition, and an answer settled in passing is an answer nobody reviewed.
Assign each to the leaf that owns it, or record it in the node brief with the
evidence, and cut.

**A pure refactor is the premise, and it is falsifiable.** The root brief says
no on-disk name changes, so trees in flight need no migration by construction.
Question 2 is the one that can break that promise — tightening the grammar makes
a tree grove itself wrote unreadable if grove ever wrote a lenient name. Whether
it did is a question about the corpus, and it is worth cutting a leaf for it if
the answer is not obvious.

## Decisions (running log)

**Increment 2 is a sibling node, not this leaf decomposed.** `library-k6` cut
`crate-k7` the same way — the planner retires as its own `DONE` leaf beside the
node it created. Decomposing this leaf would have been the wrong trigger:
`references/decompose.md`'s node verb is for a leaf that proved *bigger than its
brief*, and this one completed exactly as chartered.

**No review chain was cut for this decomposition.** `references/decompose.md`
names *a decomposition others will build on for months* as earning one, so this
was a judgement rather than an omission. Against: the cut is heavily constrained
— an already-checked architecture, a fixed operation set, four explicitly
enumerated questions and an existing 130-test net — so the failure mode is a
misplaced boundary, which the first leaf to read the brief hits immediately and
cheaply. The precedent in this grove agrees: `library-k6` cut none for
`crate-k7`, and increment 1 landed clean. A later leaf that finds the ordering
wrong should `leaf-insert` rather than treat it as settled.

**The skill-distillation leaf was cut now, at the root, rather than deferred.**
The reasoning is in `formalism-skill-k38`'s own body, where the session that
executes it will read it. Recorded here only because the task asked for the
decision to be visible rather than made by omission.

**Question 2 needed no corpus-survey leaf.** The task file said one was worth
cutting *if the answer is not obvious*. It is: every on-disk name grove writes is
rendered by `tree_id::Entry::name`, which pads the position `{:02}` — the grow
verbs, the lifecycle verbs and `tree_migrate`'s output alike — so grove has never
written a lenient position and only a hand edit can produce one. That fact is in
the node brief so `domain-k29` starts from it.
