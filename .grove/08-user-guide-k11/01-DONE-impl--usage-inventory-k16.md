# usage-inventory-k16

## Goal

Write down, and commit on its own, the set `docs/USAGE.md` must cover: every
command and flag the guide is obliged to document, and every user journey from
scaffolding a grove to teardown. Do not edit the guide in this session.

## Context

- The root brief's *Done when* requires the inventory to precede the guide's
  edit, so that "complete" names a checkable set and an adversarial read has
  something to read against.
- **The inventory's own boundary is yours to settle and to write down.** The
  brief says "every subcommand and flag the binary exposes", and `grove` itself
  exposes only `-h` and `-V` — the loop is what a bare `grove` runs. The commands
  a human actually meets are split across two binaries: `grove`, and the
  `grove-llm` verbs that `docs/USAGE.md` already documents in its *Review
  composition and escalation* section. State which surface the guide owns and
  why, in the inventory itself, before enumerating it. Getting that boundary
  wrong in either direction is the failure mode: too narrow and the guide is
  complete against a set nobody wanted; too wide and it takes
  `docs/CONFIGURATION.md`'s subject.
- Derive the command rows from `--help` output, not from the current guide. The
  guide is the artifact under test; using it as the source makes the inventory
  agree with it by construction.
- The journeys run from `root-init` through pick, session, retire, commit, node
  close, review composition, pruning, and the complete finish cycle. `CONTEXT.md`
  names each of those; the guide's job is the human's path through them.

## Done when

- A committed inventory exists — location yours to choose within the ownership
  table's rules; a section of `docs/USAGE.md` is legitimate if it is genuinely
  the guide's own subject, and a separate file directly under `docs/` is not,
  because the ownership table bounds what may sit there.
- Every command row is traceable to `--help` output, and every flag of every
  covered command appears.
- Every journey row names its start state and its end state.
- The inventory says what it deliberately excludes, and why.
- The inventory names the guide's **stable entry points** — the rows a
  walkthrough book may cite by anchor — as an explicit set. Every walkthrough
  book must cite one guide anchor from its `README.md` reader contract
  (`docs/specs/walkthrough-books.md`, *Outbound links*), and the anchors it may
  reserve come from this set. Naming it here rather than in the guide is the same
  argument as the rest of this leaf: an anchor set chosen while writing the guide
  describes what was written.
- `bash scripts/check.sh` passes and no link is left dangling.

## Notes

**Do not improve the guide while you are in it.** A single commit that both sets
the standard and meets it cannot show that the standard was met; the two-leaf
split exists for exactly that reason and folding them back defeats it. Anything
you notice about the guide's content goes in the inventory as a gap row, which is
`usage-guide-k23`'s work list.

## Decisions (running log)

1. **The inventory lives at `docs/specs/user-guide-coverage.md`.** The ownership
   table bounds what may sit *directly* under `docs/` and explicitly permits
   focused files under `docs/adr/`, `docs/specs/` and `docs/research/`.
   `SPEC-FORMAT.md`'s membership test passes: a session on a later grove editing
   the guide, and every book reserving a guide anchor, must read it. A section of
   `docs/USAGE.md` was rejected because writing it there is editing the guide,
   which this leaf may not do.
2. **The guide owns the whole installed command surface — both binaries, every
   subcommand and flag — in the human's register.** Delivery is the
   justification: `docs/ARCHITECTURE.md`'s *Repository products* table has
   Homebrew install `grove` and `grove-llm` together, and no other ownership row
   documents either. `grove-llm --help`'s "not meant for direct human use" is
   about who drives the verbs, not who must understand them. The register limit —
   what a verb does to the tree and when a human runs it, not the session's
   calling contract — is what keeps the guide off `CONFIGURATION.md`'s subject.
3. **Every command row is derived from `--help` at grove 20.1.0**, run against a
   fresh `cargo build --bins`, not from the guide's current contents. The guide is
   the artifact under test.
4. **Eight stable entry points, `usage-`-prefixed.** The prefix exists because
   books cite them as `USAGE.md#…`, where a bare `finish` or `pick` would read as
   a repository term rather than a location. The set is closed and additive: a
   book needing a ninth changes the inventory first.
5. **`book-check` and `syllabus` are excluded** as repository-internal authoring
   tools — the *Repository products* table ships `grove` and `grove-llm` only.
6. **No in-session reviewer spent.** The adversarial read the node's *Done when*
   asks for is already scheduled downstream: `usage-guide-k23`'s notes cut a
   `review-impl` that reads the guide against this inventory, which is the root
   brief's stated closer, and a reviewer here would inspect the same boundary
   argument one leaf earlier without the guide to test it against. This session's
   harness also forbids subagent dispatch, so the pass could not have been run in
   the prescribed four-step form.
